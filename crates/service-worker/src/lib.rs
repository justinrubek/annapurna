use gloo_utils::format::JsValueSerdeExt;
use js_sys::Promise;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{console, ServiceWorkerGlobalScope};

pub mod error;

/// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn init_worker() -> std::result::Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());

    // Ensure that we're running in a service worker
    let global = js_sys::global();
    if let Ok(true) = js_sys::Reflect::has(&global, &JsValue::from_str("ServiceWorkerGlobalScope"))
    {
        console::log_1(&JsValue::from_str("in service worker"));
        // we're in a service worker, so we can cast the global to a ServiceWorkerGlobalScope
        let _global = global.unchecked_into::<ServiceWorkerGlobalScope>();
    } else {
        console::log_1(&JsValue::from_str("not in service worker"));
        return Err(error::Error::NotInServiceWorker.into());
    }

    Ok(())
}

/// A message sent from the client to the service worker.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
enum WorkerMessage {
    LoginCallback { token: String, redirect_to: String },
    Logout { redirect_to: String },
    PostRegister,
}

/// Called by the service worker's `message` event.
#[wasm_bindgen]
pub fn on_message(event: web_sys::ExtendableMessageEvent) {
    info!("on_message");

    match event.data().into_serde() {
        Ok(WorkerMessage::LoginCallback { token, redirect_to }) => {
            info!(
                "LoginCallback {{ token: {:?}, redirect_to: {:?} }}",
                token, redirect_to
            );
            // TODO: store token in locally
        }
        Ok(WorkerMessage::Logout { redirect_to }) => {
            info!("Logout {{ redirect_to: {:?} }}", redirect_to);
            // TODO: remove token from storage
        }
        Ok(WorkerMessage::PostRegister) => {
            debug!("PostRegister");
        }
        Err(e) => {
            info!("error: {:?}", e);
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = fetch)]
    fn fetch_with_request(request: &web_sys::Request) -> Promise;
}

/// Called by the service worker's `fetch` event.
/// Intercepts and potentially modifies fetch requests that are made by the client
#[wasm_bindgen]
pub async fn on_fetch(event: web_sys::FetchEvent) -> std::result::Result<Promise, JsValue> {
    let request = event.request();

    let url = request.url();
    info!("on_fetch: {:?}", url);

    // let window = web_sys::window().expect("no global `window` exists");

    let response = fetch_with_request(&request);
    Ok(response)
}
