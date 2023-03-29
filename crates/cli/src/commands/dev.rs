use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use tokio::sync::mpsc::Receiver;

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

                async_watch(paths).await?;

                Ok(())
            }
        }
    }
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (tx, rx) = tokio::sync::mpsc::channel(100);

    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(paths: Vec<P>) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    for path in paths {
        watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;
    }

    while let Some(res) = rx.recv().await {
        let event = res?;
        println!("event: {:?}", event);
    }

    Ok(())
}
