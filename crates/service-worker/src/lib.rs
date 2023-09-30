use crate::token::fetch_with_token;
use gloo_utils::format::JsValueSerdeExt;
use js_sys::Promise;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, ServiceWorkerGlobalScope, Url};

mod constants;
pub mod error;
mod state;
mod token;

/// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn init_worker() -> std::result::Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());

    // Ensure that we're running in a service worker
    let global = js_sys::global();
    if let Ok(true) = js_sys::Reflect::has(&global, &JsValue::from_str("ServiceWorkerGlobalScope"))
    {
        console::log_1(&JsValue::from_str("in service worker"));
        // we're in a service worker, so we can cast the global to a ServiceWorkerGlobalScope
        let _global = global.unchecked_into::<ServiceWorkerGlobalScope>();
    } else {
        console::log_1(&JsValue::from_str("not in service worker"));
        return Err(error::Error::NotInServiceWorker.into());
    }

    Ok(())
}

/// A message sent from the client to the service worker.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
enum WorkerMessage {
    LoginCallback { token: String, redirect_to: String },
    Logout { redirect_to: String },
    PostRegister,
}

/// Called by the service worker's `message` event.
#[wasm_bindgen]
pub async fn on_message(event: web_sys::ExtendableMessageEvent) {
    info!("on_message");

    match event.data().into_serde() {
        Ok(WorkerMessage::LoginCallback { token, redirect_to }) => {
            info!(
                "LoginCallback {{ token: {:?}, redirect_to: {:?} }}",
                token, redirect_to
            );

            let rexie = state::build_database().await.unwrap();
            state::set_key(&rexie, constants::TOKEN_KEY, &token)
                .await
                .unwrap();

            // redirect to the home page
            let client = event
                .source()
                .unwrap()
                .unchecked_into::<web_sys::WindowClient>();

            let redirect = client.navigate(&redirect_to).unwrap();
            JsFuture::from(redirect).await.unwrap();
        }
        Ok(WorkerMessage::Logout { redirect_to }) => {
            info!("Logout {{ redirect_to: {:?} }}", redirect_to);
            let rexie = state::build_database().await.unwrap();
            state::remove_key(&rexie, constants::TOKEN_KEY)
                .await
                .unwrap();

            let client = event
                .source()
                .unwrap()
                .unchecked_into::<web_sys::WindowClient>();

            let redirect = client.navigate(&redirect_to).unwrap();
            JsFuture::from(redirect).await.unwrap();
        }
        Ok(WorkerMessage::PostRegister) => {
            debug!("PostRegister");
        }
        Err(e) => {
            info!("error: {:?}", e);
        }
    }
}

/// Called by the service worker's `fetch` event.
/// Intercepts and potentially modifies fetch requests that are made by the client
#[wasm_bindgen]
pub async fn on_fetch(event: web_sys::FetchEvent) -> std::result::Result<Promise, JsValue> {
    let global = js_sys::global();
    let global = global.unchecked_into::<ServiceWorkerGlobalScope>();

    let request = modify_request(&global, &event).await?;

    let response = fetch_with_request(&request);
    Ok(response)
}

/// Bindings for the `fetch` function which is available globally in the service worker context
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = fetch)]
    fn fetch_with_request(request: &web_sys::Request) -> Promise;
}

/// Performs the logic for determining whether to intercept a request and what to do with it.
pub async fn modify_request(
    scope: &ServiceWorkerGlobalScope,
    event: &web_sys::FetchEvent,
) -> Result<web_sys::Request, JsValue> {
    let request = event.request();
    let url = request.url();
    let url = Url::new(&url).unwrap();
    let hostname = url.hostname();

    let location = scope.location();

    let url_is_this_host = location
        .hostname()
        .eq_ignore_ascii_case(&hostname.to_string());

    if url_is_this_host {
        debug!("url is this host");
    }

    // TODO: better logic for determining whether to intercept the request
    let is_api_request = url.pathname().starts_with("/api/");
    let is_navigation = request.mode() == web_sys::RequestMode::Navigate;

    if is_api_request && !is_navigation {
        info!("adding token to request");
        fetch_with_token(event).await
    } else {
        info!("not adding token to request");
        Ok(request)
    }
}
