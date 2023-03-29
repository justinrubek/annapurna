#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Axum(#[from] axum::Error),
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
    #[error(transparent)]
    LockpadAuth(#[from] lockpad_auth::error::Error),
    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[error("Failed to build server struct")]
    ServerBuilder,
    #[error("Invalid html file")]
    InvalidHtml(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::info!(?self, "error response");
        #[allow(clippy::match_single_binding)]
        let status = match self {
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}
