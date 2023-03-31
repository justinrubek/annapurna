use std::path::Path;

use annapurna::config::Config;
use lockpad_auth::PublicKey;

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
        tracing::info!("config: {:?}", config);

        // Load auth keys
        let auth_url = config.auth_url;
        let client = reqwest::Client::new();
        let res = client
            .get(format!("{auth_url}/.well-known/jwks.json"))
            .send()
            .await
            .unwrap();
        let jwks_str = res.text().await.unwrap();
        let key_set = PublicKey::parse_from_jwks(&jwks_str)?;

        let server = annapurna_http::Server::builder()
            .addr(self.addr)
            .public_keys(key_set)
            .static_path(config.static_path)
            .build()?;

        match self.command {
            ServerCommands::Http => server.run().await?,
            ServerCommands::Dev => {
                // Determine the root of the repo so the command can be run from any directory
                let (path, _trust) = gix_discover::upwards(Path::new("."))?;
                let (_repo, worktree) = path.into_repository_and_work_tree_directories();
                let worktree = worktree.expect("no worktree directory found");
                let frontend_path = worktree.join("web");

                // launch yarn
                let _child = tokio::process::Command::new("yarn")
                    .args(["dev", "--port", "4000"])
                    .current_dir(frontend_path)
                    // .kill_on_drop(true)
                    .spawn()?;

                // launch server
                server.run().await?
            }
        };

        Ok(())
    }
}
