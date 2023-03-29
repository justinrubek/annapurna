use axum::{
    body::{self},
    middleware,
    response::Html,
    routing::{get, get_service},
    Router,
};
use lockpad_auth::PublicKey;
use std::{collections::HashMap, net::SocketAddr, path::PathBuf};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, ServiceBuilderExt};

pub mod error;
mod serve;

use error::Result;
use serve::{handle_error, inject_variables_into_html, InjectorState};

pub struct Server {
    addr: SocketAddr,

    public_keys: Vec<PublicKey>,
    static_path: PathBuf,
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

        // TODO: populate frontend_state with necessary variables
        let mut frontend_state = InjectorState {
            variables: HashMap::new(),
        };
        frontend_state
            .variables
            .insert("auth_url".to_string(), "http://localhost:3000".to_string());

        // serve all files from the static directory
        // if the file contains html, it will be edited to include runtime environment variables
        // these may be loaded by the client-side javascript
        let serve_dir = ServeDir::new(self.static_path);
        let serve_service = get_service(serve_dir).handle_error(handle_error).layer(
            ServiceBuilder::new().map_request_body(body::boxed).layer(
                middleware::from_fn_with_state(frontend_state, inject_variables_into_html),
            ),
        );

        let app = Router::new()
            .with_state(state)
            .route("/", get(root))
            .layer(cors)
            .fallback_service(serve_service);

        tracing::debug!("listening on {}", &self.addr);
        axum::Server::bind(&self.addr)
            .serve(app.into_make_service())
            .await
            .unwrap();

        Ok(())
    }
}

pub struct Builder {
    addr: Option<SocketAddr>,
    public_keys: Option<Vec<PublicKey>>,
    static_path: Option<PathBuf>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            addr: None,
            public_keys: None,
            static_path: None,
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

    pub fn static_path(mut self, static_path: PathBuf) -> Self {
        self.static_path = Some(static_path);
        self
    }

    pub fn build(self) -> Result<Server> {
        let addr = self.addr.ok_or(error::Error::ServerBuilder)?;
        let public_keys = self.public_keys.ok_or(error::Error::ServerBuilder)?;
        let static_path = self.static_path.ok_or(error::Error::ServerBuilder)?;

        Ok(Server {
            addr,
            public_keys,
            static_path,
        })
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            addr: Some(SocketAddr::from(([0, 0, 0, 0], 3000))),
            public_keys: None,
            static_path: None,
        }
    }
}

async fn root() -> Html<String> {
    Html("<h1>Hello, world!</h1>".to_string())
}
