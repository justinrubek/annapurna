use axum::{
    body::{self},
    extract::FromRef,
    middleware,
    response::Html,
    routing::{get, get_service},
    Router,
};
use hyper::{client::HttpConnector, Body};
use lockpad_auth::PublicKey;
use std::{collections::HashMap, net::SocketAddr, path::PathBuf};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, ServiceBuilderExt};

pub mod error;
mod serve;

use error::Result;
use serve::{handle_error, inject_variables_into_html, InjectorState};

type Client = hyper::client::Client<HttpConnector, Body>;

pub struct Server {
    addr: SocketAddr,

    public_keys: Vec<PublicKey>,
    static_path: PathBuf,
}

#[derive(Clone)]
pub struct ServerState {
    pub public_key: PublicKey,
}

impl FromRef<ServerState> for PublicKey {
    fn from_ref(state: &ServerState) -> Self {
        state.public_key.clone()
    }
}

// TODO: Proxy State.
// Potentially try to impl axum's FromRef for ServerState and see if that works.
#[derive(Clone)]
pub struct ProxyState {
    pub server_state: ServerState,
    pub client: Client,
}

impl FromRef<ProxyState> for ServerState {
    fn from_ref(state: &ProxyState) -> Self {
        state.server_state.clone()
    }
}

impl FromRef<ProxyState> for Client {
    fn from_ref(state: &ProxyState) -> Self {
        state.client.clone()
    }
}

/// Get the default router for the API routes.
fn api_routes<S: std::clone::Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new().route("/", get(root))
}

impl Server {
    pub fn builder() -> Builder {
        Builder::default()
    }

    /// Run the server.
    /// This will start the api server and serve it from /api and serve static files when no route is matched.
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
            .nest("/api", api_routes())
            .with_state(state)
            .fallback_service(serve_service)
            .layer(cors);

        tracing::debug!("listening on {}", &self.addr);
        axum::Server::bind(&self.addr)
            .serve(app.into_make_service())
            .await
            .unwrap();

        Ok(())
    }

    /// Run the server in development mode.
    /// This will not serve static files, but instead will be proxied by the frontend server.
    /// This is useful for development, as it allows the frontend server to handle hot-reloading.
    pub async fn run_dev(self) -> Result<()> {
        let cors = tower_http::cors::CorsLayer::permissive();

        let public_key = self.public_keys[0].clone();
        let server_state = ServerState { public_key };

        let app = Router::new()
            // reverse-proxy to the frontend server
            .nest("/api", api_routes())
            .layer(cors)
            .with_state(server_state);

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
