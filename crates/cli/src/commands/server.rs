use annapurna::config::Config;
use annapurna_data::Facts;
use async_watcher::{
    notify::{RecommendedWatcher, RecursiveMode},
    AsyncDebouncer, DebouncedEvent,
};
use lockpad_auth::PublicKey;
use std::path::Path;
use tokio::time::Duration;
use tracing::info;

#[derive(clap::Args, Debug)]
pub(crate) struct ServerCommand {
    #[clap(subcommand)]
    pub command: ServerCommands,

    #[arg(default_value = "0.0.0.0:3000", long, short)]
    pub addr: std::net::SocketAddr,
}

/// A command for running the API server
#[derive(clap::Subcommand, Debug)]
pub(crate) enum ServerCommands {
    /// start the http server
    Http,
    /// passthrough to `yarn run dev`, while also starting the http server
    Dev,
}

impl ServerCommand {
    pub(crate) async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = Config::load()?;

        // Load auth keys
        let auth_url = config.auth_url;
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{auth_url}/.well-known/jwks.json"))
            .send()
            .await
            .expect("failed to fetch jwks from auth server");

        let jwks_str = res.text().await.unwrap();
        let key_set = PublicKey::parse_from_jwks(&jwks_str)?;

        let facts = Facts::read_from_directory(config.facts_path)?;

        let server = annapurna_http::Server::builder()
            .addr(self.addr)
            .public_keys(key_set.clone())
            .static_path(config.static_path.clone())
            .auth_url(auth_url.clone())
            .auth_app_id(config.auth_app_id.clone())
            .facts(facts)
            .build()?;

        match self.command {
            ServerCommands::Http => server.run().await?,
            ServerCommands::Dev => {
                info!("Launching dev server");
                // Determine the root of the repo so the command can be run from any directory
                let (path, _trust) = gix_discover::upwards(Path::new("."))?;
                let (_repo, worktree) = path.into_repository_and_work_tree_directories();
                let worktree = worktree.expect("no worktree directory found");

                let files_path = worktree.join("public");
                let crates_path = worktree.join("crates");

                let (mut file_events, _debouncer) =
                    async_debounce_watch(vec![files_path, crates_path]).await?;

                // build the static files
                let static_path = build_static(&worktree).await?;

                // spawn another task that is responsible for running the server
                // it will receive a message when it is time to restart the server
                let mut server = server.clone();
                server.change_dir(static_path);

                let mut task = tokio::spawn(server.clone().run());

                loop {
                    tokio::select! {
                        events = file_events.recv() => {
                            match events {
                                Some(events) => {
                                    info!(?events, "file changed, restarting server");

                                    // Run nix build
                                    let static_path = build_static(&worktree).await?;

                                    let mut server = server.clone();
                                    server.change_dir(static_path);

                                    // Restart server
                                    task.abort();
                                    task = tokio::spawn(server.clone().run());
                                }
                                None => {
                                    tracing::error!("file watcher channel closed, exiting");
                                    break;
                                }
                            }
                        }
                        result = &mut task => {
                            info!(?result, "http server exited");
                            result??;
                        }
                    }
                }
            }
        };

        Ok(())
    }
}

/// Watches all given paths for changes.
/// When a change is detected, run triggers the server to rebuild.
pub async fn async_debounce_watch<P: AsRef<Path>>(
    paths: Vec<P>,
) -> Result<
    (
        tokio::sync::mpsc::Receiver<Result<Vec<DebouncedEvent>, Vec<notify::Error>>>,
        AsyncDebouncer<RecommendedWatcher>,
    ),
    Box<dyn std::error::Error>,
> {
    let (tx, rx) = tokio::sync::mpsc::channel(1);

    let mut debouncer =
        AsyncDebouncer::new(Duration::from_secs(1), Some(Duration::from_secs(1)), tx).await?;

    // add the paths to the watcher
    paths.iter().for_each(|p| {
        debouncer
            .watcher()
            .watch(p.as_ref(), RecursiveMode::Recursive)
            .unwrap();
    });

    Ok((rx, debouncer))
}

async fn build_static(
    worktree: &std::path::Path,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let out_path = worktree.join(".static");
    info!(?out_path, "building static files with nix");

    tokio::process::Command::new("nix")
        .args([
            "build",
            ".#static-files",
            "--out-link",
            out_path.to_str().unwrap(),
        ])
        .current_dir(worktree)
        .spawn()?
        .wait_with_output()
        .await?;

    Ok(out_path.join("public"))
}
