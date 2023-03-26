#[allow(unused_imports)]
use axum::{
    routing::{get, post},
    Router,
};
use lockpad_auth::PublicKey;
use std::net::SocketAddr;

pub mod error;

use error::Result;

pub struct Server {
    addr: SocketAddr,

    public_keys: Vec<PublicKey>,
}

#[derive(Clone)]
pub struct ServerState {
    pub public_key: PublicKey,
}

impl AsRef<PublicKey> for ServerState {
    fn as_ref(&self) -> &PublicKey {
        &self.public_key
    }
}

impl Server {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub async fn run(self) -> Result<()> {
        let cors = tower_http::cors::CorsLayer::permissive();

        let public_key = self.public_keys[0].clone();
        let state = ServerState { public_key };

        let app = Router::new()
            .route("/", get(root))
            .with_state(state)
            .layer(cors);

        tracing::info!("Listening on {0}", self.addr);
        axum::Server::bind(&self.addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}

pub struct Builder {
    addr: Option<SocketAddr>,
    public_keys: Option<Vec<PublicKey>>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            addr: None,
            public_keys: None,
        }
    }

    pub fn addr(mut self, addr: SocketAddr) -> Self {
        self.addr = Some(addr);
        self
    }

    pub fn public_keys(mut self, public_keys: Vec<PublicKey>) -> Self {
        self.public_keys = Some(public_keys);
        self
    }

    pub fn build(self) -> Result<Server> {
        let addr = self.addr.ok_or(error::Error::ServerBuilder)?;
        let public_keys = self.public_keys.ok_or(error::Error::ServerBuilder)?;

        Ok(Server { addr, public_keys })
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            addr: Some(SocketAddr::from(([0, 0, 0, 0], 3000))),
            public_keys: None,
        }
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}
