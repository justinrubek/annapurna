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
    /// it may be preferable to use the `sever dev` command instead as it will also run the backend
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
pub async fn async_debounce_watch<P: AsRef<Path>>(
    paths: Vec<P>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    let mut debouncer =
        new_async_debouncer(Duration::from_secs(1), Some(Duration::from_secs(1)), tx).await?;

    paths.iter().for_each(|p| {
        debouncer
            .watcher()
            .watch(p.as_ref(), RecursiveMode::Recursive)
            .unwrap();
    });

    // when the files change, we want to trigger a `yarn run build` in `./web`
    let mut build_command: Option<tokio::process::Child> = None;

    while let Some(event) = rx.recv().await {
        match event {
            Ok(_events) => {
                // the assumption is that we will trigger on any event, so we are not checking the info

                // if there is a running process, we must kill it first
                if let Some(ref mut command) = build_command {
                    command.kill().await?;
                }

                let process = tokio::process::Command::new("yarn")
                    .args(["build"])
                    .current_dir("./web")
                    .spawn()?;

                build_command = Some(process);
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
