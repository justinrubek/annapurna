use annapurna_data::{
    types::{Ingredient, Recipe},
    Facts,
};
use axum::{
    extract::{FromRef, State},
    middleware,
    response::{Html, IntoResponse, Redirect},
    routing::{get, get_service, post},
    Form, Router,
};
use axum_extra::TypedHeader;
use lockpad_auth::PublicKey;
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    path::PathBuf,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tracing::info;

pub mod error;
mod handlers;
mod serve;

use error::Result;
use serve::{inject_variables_into_html, InjectorState};

#[derive(Clone)]
pub struct Server {
    pg_pool: sqlx::pool::Pool<sqlx::Postgres>,

    addr: SocketAddr,
    auth_url: String,
    auth_app_id: String,

    public_keys: Vec<PublicKey>,
    static_path: PathBuf,

    facts: Facts,
}

#[derive(Clone)]
pub struct ServerState {
    pub pg_pool: sqlx::pool::Pool<sqlx::Postgres>,

    pub public_key: PublicKey,
    pub auth_url: String,
    pub auth_app_id: String,

    pub facts: Facts,
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
}

impl FromRef<ProxyState> for ServerState {
    fn from_ref(state: &ProxyState) -> Self {
        state.server_state.clone()
    }
}

/// Get the default router for the API routes.
fn api_routes<S: std::clone::Clone + Send + Sync + 'static>() -> Router<S>
where
    ServerState: axum::extract::FromRef<S>,
    PublicKey: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/", get(root))
        .route(
            "/inventory",
            get(handlers::inventory::list_inventory).post(handlers::inventory::create_inventory),
        )
        .route("/login", get(login_redirect))
        .route("/submit", post(dummy_form))
        .route("/recipes", get(get_recipes))
        .route("/ingredients", get(get_ingredients))
        .route("/health", get(health))
}

impl Server {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn change_dir(&mut self, dir: PathBuf) {
        self.static_path = dir;
    }

    /// Run the server.
    /// This will start the api server and serve it from /api and serve static files when no route is matched.
    pub async fn run(self) -> Result<()> {
        info!("Starting server on {}", self.addr);

        let cors = tower_http::cors::CorsLayer::permissive();

        let public_key = self.public_keys[0].clone();
        let state = ServerState {
            pg_pool: self.pg_pool,
            public_key,
            auth_url: self.auth_url.clone(),
            auth_app_id: self.auth_app_id.clone(),
            facts: self.facts.clone(),
        };

        // TODO: populate frontend_state with necessary variables
        let mut frontend_state = InjectorState {
            variables: HashMap::new(),
        };
        frontend_state
            .variables
            .insert("auth_url".to_string(), self.auth_url);

        // serve all files from the static directory
        // if the file contains html, it will be edited to include runtime environment variables
        // these may be loaded by the client-side javascript
        let serve_dir = ServeDir::new(self.static_path);
        let serve_service = get_service(serve_dir).layer(ServiceBuilder::new().layer(
            middleware::from_fn_with_state(frontend_state, inject_variables_into_html),
        ));

        let app = Router::new()
            .nest("/api", api_routes())
            .with_state(state)
            .fallback_service(serve_service)
            .layer(cors);

        tracing::debug!("listening on {}", &self.addr);
        let listener = TcpListener::bind(&self.addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }

    /// Run the server in development mode.
    /// This will not serve static files, but instead will be proxied by the frontend server.
    /// This is useful for development, as it allows the frontend server to handle hot-reloading.
    pub async fn run_dev(self) -> Result<()> {
        let cors = tower_http::cors::CorsLayer::permissive();

        let auth_url = self.auth_url.clone();
        let auth_app_id = self.auth_app_id.clone();
        let public_key = self.public_keys[0].clone();
        let facts = self.facts.clone();
        let server_state = ServerState {
            pg_pool: self.pg_pool,
            public_key,
            auth_url,
            auth_app_id,
            facts,
        };

        let app = Router::new()
            // reverse-proxy to the frontend server
            .nest("/api", api_routes())
            .layer(cors)
            .with_state(server_state);

        tracing::debug!("listening on {}", &self.addr);
        let listener = TcpListener::bind(&self.addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

pub struct Builder {
    addr: Option<SocketAddr>,
    auth_url: Option<String>,
    auth_app_id: Option<String>,
    pg_pool: Option<sqlx::pool::Pool<sqlx::Postgres>>,
    public_keys: Option<Vec<PublicKey>>,
    static_path: Option<PathBuf>,
    facts: Option<Facts>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            addr: None,
            auth_url: None,
            auth_app_id: None,
            pg_pool: None,
            public_keys: None,
            static_path: None,
            facts: None,
        }
    }

    pub fn addr(mut self, addr: SocketAddr) -> Self {
        self.addr = Some(addr);
        self
    }

    pub fn auth_url(mut self, auth_url: String) -> Self {
        self.auth_url = Some(auth_url);
        self
    }

    pub fn auth_app_id(mut self, auth_app_id: String) -> Self {
        self.auth_app_id = Some(auth_app_id);
        self
    }

    pub fn pg_pool(mut self, pg_pool: sqlx::pool::Pool<sqlx::Postgres>) -> Self {
        self.pg_pool = Some(pg_pool);
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

    pub fn facts(mut self, facts: Facts) -> Self {
        self.facts = Some(facts);
        self
    }

    pub fn build(self) -> Result<Server> {
        let addr = self.addr.ok_or(error::Error::ServerBuilder)?;
        let auth_url = self.auth_url.ok_or(error::Error::ServerBuilder)?;
        let auth_app_id = self.auth_app_id.ok_or(error::Error::ServerBuilder)?;
        let pg_pool = self.pg_pool.ok_or(error::Error::ServerBuilder)?;
        let public_keys = self.public_keys.ok_or(error::Error::ServerBuilder)?;
        let static_path = self.static_path.ok_or(error::Error::ServerBuilder)?;
        let facts = self.facts.ok_or(error::Error::ServerBuilder)?;

        Ok(Server {
            addr,
            auth_url,
            auth_app_id,
            pg_pool,
            public_keys,
            static_path,
            facts,
        })
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            addr: Some(SocketAddr::from(([0, 0, 0, 0], 3000))),
            auth_url: None,
            auth_app_id: None,
            pg_pool: None,
            public_keys: None,
            static_path: None,
            facts: None,
        }
    }
}

async fn root() -> Html<String> {
    Html("<h1>Hello, world!</h1>".to_string())
}

async fn login_redirect(
    TypedHeader(host): TypedHeader<axum_extra::headers::Host>,
    State(ServerState {
        auth_url,
        auth_app_id,
        ..
    }): State<ServerState>,
) -> impl IntoResponse {
    let protocol = if host.to_string().starts_with("localhost") {
        "http"
    } else {
        "https"
    };
    let callback_url = format!("{protocol}://{host}/login-callback",);

    let mut query_params = HashMap::new();
    query_params.insert("redirect_uri", &callback_url);
    query_params.insert("client_id", &auth_app_id);
    let query_string = serde_urlencoded::to_string(&query_params).unwrap();

    let url = format!("{auth_url}/login?{query_string}");
    Redirect::temporary(&url)
}

#[derive(Debug, serde::Deserialize)]
struct DummyForm {
    name: String,
}

async fn dummy_form(
    claims: lockpad_auth::Claims,
    Form(payload): Form<DummyForm>,
) -> impl IntoResponse {
    tracing::info!("claims: {:?}", claims);
    Redirect::to(&format!("/?name={}", payload.name))
}

async fn get_recipes(
    State(ServerState { facts, .. }): State<ServerState>,
) -> axum::Json<Vec<Recipe>> {
    axum::Json(facts.recipes)
}

async fn get_ingredients(
    State(ServerState { facts, .. }): State<ServerState>,
) -> axum::Json<HashSet<Ingredient>> {
    // iterate over all recipes and collect all ingredients
    let ingredients = facts
        .recipes
        .iter()
        .flat_map(|recipe| recipe.ingredients.iter())
        .cloned()
        .collect();

    axum::Json(ingredients)
}

pub async fn health() -> &'static str {
    "OK"
}
