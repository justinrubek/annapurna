use axum::{
    body::{self, BoxBody, HttpBody},
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use html_editor::{
    operation::{Editable, Htmlifiable, Selector},
    Node,
};
use lockpad_auth::PublicKey;
use std::{collections::HashMap, net::SocketAddr, path::PathBuf};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, ServiceBuilderExt};

pub mod error;

use error::Result;

pub struct Server {
    addr: SocketAddr,

    public_keys: Vec<PublicKey>,
    static_path: PathBuf,
}

#[derive(Clone)]
pub struct ServerState {
    pub public_key: PublicKey,
}

#[derive(Clone, Debug)]
pub struct FrontendState {
    pub variables: HashMap<String, String>,
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
        let mut frontend_state = FrontendState {
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

async fn inject_variables_into_html(
    State(state): State<FrontendState>,
    request: Request<BoxBody>,
    next: Next<BoxBody>,
) -> std::result::Result<impl IntoResponse, Response> {
    let response = next.run(request).await;

    let (mut parts, body) = response.into_parts();

    let is_html = parts
        .headers
        .get(hyper::header::CONTENT_TYPE)
        .map(|value| value == "text/html")
        .unwrap_or(false);

    let body = match is_html {
        true => {
            // Extract the body and parse it into a dom
            let bytes = hyper::body::to_bytes(body).await.unwrap();
            let html = String::from_utf8(bytes.to_vec()).unwrap();
            let mut dom = html_editor::parse(&html).unwrap();

            // Add the variables to the dom
            let nodes = state
                .variables
                .iter()
                .map(|(key, value)| {
                    Node::new_element(
                        "div",
                        vec![(&format!("data-config-{}", key), value)],
                        vec![],
                    )
                })
                .collect::<Vec<_>>();
            let parent = Node::new_element("div", vec![("id", "injected-config")], nodes);
            dom.insert_to(&Selector::from("body"), parent);

            // Create a new response body
            let new_html = dom.html();
            let bytes = new_html.as_bytes().to_vec();
            let body = body::boxed(axum::body::Full::from(bytes));

            // update the content length header to match the new body
            let content_length = body.size_hint().exact().unwrap();
            parts.headers.insert(
                hyper::header::CONTENT_LENGTH,
                hyper::header::HeaderValue::from(content_length),
            );

            body
        }
        false => body,
    };

    let response = Response::from_parts(parts, body);

    Ok(response)
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

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}
