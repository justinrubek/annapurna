use annapurna::config::Config;
use annapurna_data::Facts;
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
}

impl ServerCommand {
    pub(crate) async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = Config::load()?;

        let pg_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.postgres_url)
            .await?;

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
            .pg_pool(pg_pool)
            .public_keys(key_set.clone())
            .static_path(config.static_path.clone())
            .auth_url(auth_url.clone())
            .auth_app_id(config.auth_app_id.clone())
            .facts(facts)
            .build()?;

        match self.command {
            ServerCommands::Http => server.run().await?,
        };

        Ok(())
    }
}
