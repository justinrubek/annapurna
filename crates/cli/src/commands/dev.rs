use annapurna_watcher::new_async_debouncer;
use notify::RecursiveMode;
use std::path::Path;
use tokio::time::Duration;

#[derive(clap::Args, Debug)]
pub(crate) struct DevCommand {
    #[clap(subcommand)]
    pub command: DevCommands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum DevCommands {
    /// watch for changes in the web directory and rebuild the frontend
    Watch,
}

impl DevCommand {
    pub(crate) async fn run(&self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        match self.command {
            DevCommands::Watch => {
                println!("Watching for changes...");

                let paths = vec![
                    "web/src",
                    "web/package.json",
                    "web/yarn.lock",
                    "web/public",
                    "web/tsconfig.json",
                    "web/astro.config.mjs",
                    "web/postcss.config.cjs",
                    "web/tailwind.config.cjs",
                    "web/tailwind.theme.config.js",
                ];

                async_debounce_watch(paths).await?;

                Ok(())
            }
        }
    }
}

/// Watches all given paths for changes.
/// Changes are debounced, and gathered into a list of events
pub async fn async_debounce_watch<P: AsRef<Path>>(paths: Vec<P>) -> notify::Result<()> {
    // Create an AsyncDebouncer with a timeout of 1 second and a tick rate of 1 second.
    // Process it until the debouncer is dropped.
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    let mut debouncer =
        new_async_debouncer(Duration::from_secs(1), Some(Duration::from_secs(1)), tx).await?;

    paths.iter().for_each(|p| {
        debouncer
            .watcher()
            .watch(p.as_ref(), RecursiveMode::Recursive)
            .unwrap();
    });

    while let Some(event) = rx.recv().await {
        match event {
            Ok(events) => {
                for event in events {
                    println!("event: {:?}", event);
                }
            }
            Err(errors) => {
                for error in errors {
                    println!("error: {:?}", error);
                }
            }
        }
    }

    Ok(())
}
