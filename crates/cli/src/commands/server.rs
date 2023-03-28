use annapurna::config::Config;
use lockpad_auth::PublicKey;

#[derive(clap::Args, Debug)]
pub(crate) struct ServerCommand {
    #[clap(subcommand)]
    pub command: ServerCommands,

    #[arg(default_value = "0.0.0.0:5000", long, short)]
    pub addr: std::net::SocketAddr,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum ServerCommands {
    /// start the http server
    Http,
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
        }

        Ok(())
    }
}
