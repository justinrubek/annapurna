use crate::{constants, state};
use base64::Engine as _;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

/// Creates a new request that has the token attached to it on the `Authorization` header.
/// If a valid token is not found then the request is returned as-is.
pub(crate) async fn fetch_with_token(
    event: &web_sys::FetchEvent,
) -> Result<web_sys::Request, JsValue> {
    let url = event.request().url();
    info!("fetch_with_token: {:?}", url);

    let token = retrieve_token(event).await?;
    if token.is_none() {
        return Ok(event.request());
    }

    // TODO: consider handling token authorization using a type instead of string equality
    match token.as_deref() {
        None | Some(constants::UNAUTHORIZED_TOKEN) => Ok(event.request()),
        Some(token) => {
            debug!("adding token: {:?}", token);
            let new_request = add_token_to_request(&event.request(), token)?;
            Ok(new_request)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenPayload {
    exp: u64,
}

/// Attempts to retrieve the token from the database.
/// If the token is not found then an attempt is made to refresh the token.
/// If the token is successfully refreshed then the new token is returned.
async fn retrieve_token(event: &web_sys::FetchEvent) -> Result<Option<String>, JsValue> {
    let rexie = state::build_database().await?;
    let token = state::get_key(&rexie, constants::TOKEN_KEY).await?;

    if token.is_none() {
        // Attempt to refresh the token
        let token = refresh_token(event).await?;

        if token.is_none() {
            return Ok(None);
        }
    }

    // Ensure that the token isn't expired.
    if let Some(token) = &token {
        // Take the second parts of the token, decode it, and look at the `exp` field.
        let token_part: &str = token.split('.').nth(1).unwrap();
        let engine = base64::engine::general_purpose::URL_SAFE_NO_PAD;
        let token_part = engine.decode(token_part).unwrap();
        let token_part: TokenPayload = serde_json::from_slice(&token_part).unwrap();
        info!("token_part: {:?}", token_part);

        // Get the current time in seconds
        let now = js_sys::Math::floor(js_sys::Date::now() / 1000.0) as u64;

        if token_part.exp < now {
            // Attempt to refresh the token
            let token = refresh_token(event).await?;

            if token.is_none() {
                return Ok(None);
            }
        }
    }

    // Ensure that the token is not the unauthorized token
    if let Some(token) = &token {
        if token == constants::UNAUTHORIZED_TOKEN {
            return Ok(None);
        }
    }

    Ok(token)
}

/// Forms a new request that attaches the given token to the `Authorization` header as a bearer token.
/// This is used to authenticate requests to the API.
/// As a result this will form a new request with the mode set to same-origin.
fn add_token_to_request(
    request: &web_sys::Request,
    token: &str,
) -> Result<web_sys::Request, JsValue> {
    let headers = web_sys::Headers::new_with_headers(&request.headers())?;
    headers.append("Authorization", &format!("Bearer {}", token))?;

    let mut init = web_sys::RequestInit::new();
    init.headers(&headers);
    init.mode(web_sys::RequestMode::SameOrigin);

    let new_request = web_sys::Request::new_with_request_and_init(request, &init)?;
    Ok(new_request)
}

/// Attempts to refresh the token by sending a request to the API.
/// If the token is successfully refreshed then the new token is returned.
async fn refresh_token(_event: &web_sys::FetchEvent) -> Result<Option<String>, JsValue> {
    // TODO: implement this
    // TODO: lock the refresh so that we don't have multiple requests trying to refresh the token at the same time
    Ok(None)
}
