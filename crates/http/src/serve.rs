use crate::error::{Error, Result};
use axum::{
    body::{self, BoxBody, HttpBody},
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use html_editor::{
    operation::{Editable, Htmlifiable, Selector},
    Node,
};
use std::collections::HashMap;

/// The state of the injector middleware.
/// It accepts a hashmap of variables to inject into the html response body.
#[derive(Clone, Debug)]
pub(crate) struct InjectorState {
    pub variables: HashMap<String, String>,
}

/// Middleware that injects variables into the html response body.
/// This is used to inject server-side variables into the dom which can then be read by the client-side javascript.
/// The function will look for a `text/html` content type header and if it is found, it will parse the body as html, inject the variables into the dom, and then return the new body.
/// The variables are injected into a div with the id "injected-config".
/// The client-side javascript can then read the variables from this div using the `data-` prefixed attributes.
pub(crate) async fn inject_variables_into_html(
    State(state): State<InjectorState>,
    request: Request<BoxBody>,
    next: Next<BoxBody>,
) -> Result<impl IntoResponse> {
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
            let bytes = hyper::body::to_bytes(body).await?;
            let html = String::from_utf8(bytes.to_vec())?;
            let mut dom = html_editor::parse(&html).map_err(Error::InvalidHtml)?;

            // Add the variables to the dom
            let values = state
                .variables
                .iter()
                .map(|(key, value)| (format!("data-{}", key), value))
                .collect::<Vec<_>>();
            // Get the values as references
            let mut values = values
                .iter()
                .map(|(key, value)| (key.as_str(), value.as_str()))
                .collect::<Vec<_>>();
            values.push(("id", "injected-config"));
            let node = Node::new_element("div", values, vec![]);
            dom.insert_to(&Selector::from("body"), node);

            // Create a new response body
            let new_html = dom.html();
            let bytes = new_html.as_bytes().to_owned();
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

pub(crate) async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}
