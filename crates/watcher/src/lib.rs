use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

pub use notify;
use notify::{Error, ErrorKind, Event, RecommendedWatcher, Watcher};

pub mod error;

/// Deduplicate event data entry
struct EventData {
    /// Insertion Time
    insert: Instant,
    /// Last Update
    update: Instant,
}

impl EventData {
    fn new_any() -> Self {
        let time = Instant::now();
        Self {
            insert: time,
            update: time,
        }
    }
}

/// A debounced event kind.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DebouncedEventKind {
    /// No precise events
    Any,
    /// Event but debounce timed out (for example continuous writes)
    AnyContinuous,
}

/// A debounced event.
///
/// Does not emit any specific event type on purpose, only distinguishes between an any event and a continuous any event.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DebouncedEvent {
    /// Event path
    pub path: PathBuf,
    /// Event kind
    pub kind: DebouncedEventKind,
}

impl DebouncedEvent {
    fn new(path: PathBuf, kind: DebouncedEventKind) -> Self {
        Self { path, kind }
    }
}

type DebounceData = Arc<Mutex<DebounceDataInner>>;

#[derive(Default)]
struct DebounceDataInner {
    d: HashMap<PathBuf, EventData>,
    timeout: Duration,
    e: Vec<crate::Error>,
}

impl DebounceDataInner {
    /// Retrieve a vec of debounced events, removing them if they are not continuous
    pub fn debounced_events(&mut self) -> Vec<DebouncedEvent> {
        let mut events_expired = Vec::with_capacity(self.d.len());
        let mut data_back = HashMap::with_capacity(self.d.len());
        // TODO: drain_filter https://github.com/rust-lang/rust/issues/59618
        for (k, v) in self.d.drain() {
            if v.update.elapsed() >= self.timeout {
                events_expired.push(DebouncedEvent::new(k, DebouncedEventKind::Any));
            } else if v.insert.elapsed() >= self.timeout {
                data_back.insert(k.clone(), v);
                events_expired.push(DebouncedEvent::new(k, DebouncedEventKind::AnyContinuous));
            } else {
                data_back.insert(k, v);
            }
        }
        self.d = data_back;
        events_expired
    }

    /// Returns all currently stored errors
    pub fn errors(&mut self) -> Vec<Error> {
        let mut v = Vec::new();
        std::mem::swap(&mut v, &mut self.e);
        v
    }

    /// Add an error entry to re-send later on
    pub fn add_error(&mut self, e: crate::Error) {
        self.e.push(e);
    }

    /// Add new event to debouncer cache
    pub fn add_event(&mut self, e: Event) {
        for path in e.paths.into_iter() {
            if let Some(v) = self.d.get_mut(&path) {
                v.update = Instant::now();
            } else {
                self.d.insert(path, EventData::new_any());
            }
        }
    }
}

#[async_trait::async_trait]
pub trait AsyncDebounceEventHandler {
    async fn handle_event(&mut self, event: Result<Vec<DebouncedEvent>, Vec<Error>>);
}

#[async_trait::async_trait]
impl<F> AsyncDebounceEventHandler for F
where
    F: FnMut(Result<Vec<DebouncedEvent>, Vec<Error>>) + Send + 'static,
{
    async fn handle_event(&mut self, event: Result<Vec<DebouncedEvent>, Vec<Error>>) {
        self(event)
    }
}

#[async_trait::async_trait]
impl AsyncDebounceEventHandler
    for tokio::sync::mpsc::Sender<Result<Vec<DebouncedEvent>, Vec<Error>>>
{
    async fn handle_event(&mut self, event: Result<Vec<DebouncedEvent>, Vec<Error>>) {
        let _ = self.send(event).await;
    }
}

pub struct AsyncDebouncer<T: Watcher> {
    stop: Arc<AtomicBool>,
    watcher: T,
    debouncer_task: Option<tokio::task::JoinHandle<()>>,
}

impl<T: Watcher> AsyncDebouncer<T> {
    pub async fn stop(mut self) {
        self.set_stop();
        if let Some(t) = self.debouncer_task.take() {
            let _ = t.await;
        }
    }

    fn set_stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }

    pub fn watcher(&mut self) -> &mut dyn Watcher {
        &mut self.watcher
    }
}

impl<T: Watcher> Drop for AsyncDebouncer<T> {
    fn drop(&mut self) {
        // don't imitate c++ async futures and block on drop
        self.set_stop();
    }
}

/// Creates a new debounced watcher with custom configuration.
/// Timeout is the amount of time after which a debounced event is emitted or a continuous event is send, if there still are events incoming for the specific path.
/// If tick_rate is None, notify will select a tick rate that is less than the provided timeout.
pub async fn new_async_debouncer_opt<F: AsyncDebounceEventHandler + Send + 'static, T: Watcher>(
    timeout: Duration,
    tick_rate: Option<Duration>,
    mut event_handler: F,
    config: notify::Config,
) -> Result<AsyncDebouncer<T>, Error> {
    let data = DebounceData::default();

    let stop = Arc::new(AtomicBool::new(false));

    let tick_div = 4;
    let tick = match tick_rate {
        Some(v) => {
            if v > timeout {
                return Err(Error::new(ErrorKind::Generic(format!(
                    "Invalid tick_rate, tick rate {:?} > {:?} timeout!",
                    v, timeout
                ))));
            }
            v
        }
        None => timeout.checked_div(tick_div).ok_or_else(|| {
            Error::new(ErrorKind::Generic(format!(
                "Failed to calculate tick as {:?}/{}!",
                timeout, tick_div
            )))
        })?,
    };

    {
        let mut data_w = data.lock().unwrap();
        data_w.timeout = timeout;
    }

    let data_c = data.clone();
    let stop_c = stop.clone();
    let debouncer_task = tokio::spawn(async move {
        loop {
            if stop_c.load(Ordering::Acquire) {
                break;
            }
            tokio::time::sleep(tick).await;
            let send_data;
            let errors: Vec<crate::Error>;
            {
                let mut lock = data_c.lock().expect("Can't lock debouncer data!");
                send_data = lock.debounced_events();
                errors = lock.errors();
            }
            if !send_data.is_empty() {
                event_handler.handle_event(Ok(send_data)).await;
            }
            if !errors.is_empty() {
                event_handler.handle_event(Err(errors)).await;
            }
        }
    });

    let watcher = T::new(
        move |e: Result<Event, Error>| {
            let mut lock = data.lock().expect("Can't lock debouncer data!");

            match e {
                Ok(e) => lock.add_event(e),
                // errors are stored and send later on
                Err(e) => lock.add_error(e),
            }
        },
        config,
    )?;

    let guard = AsyncDebouncer {
        watcher,
        debouncer_task: Some(debouncer_task),
        stop,
    };

    Ok(guard)
}

/// Creates a new debounced watcher with the recommended watcher implementation.
/// Timeout specifies the time after which events are emitted.
/// If tick_rate is None, notify will select a tick rate that is less than the provided timeout.
pub async fn new_async_debouncer<F: AsyncDebounceEventHandler + Send + 'static>(
    timeout: Duration,
    tick_rate: Option<Duration>,
    event_handler: F,
) -> Result<AsyncDebouncer<RecommendedWatcher>, Error> {
    new_async_debouncer_opt::<F, RecommendedWatcher>(
        timeout,
        tick_rate,
        event_handler,
        notify::Config::default(),
    )
    .await
}
