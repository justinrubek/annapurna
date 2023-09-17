use crate::{routing::Route, state::AppState};
use dioxus::prelude::*;
use dioxus_router::components::Router;
use wasm_bindgen::prelude::*;

mod api;
mod components;
mod routing;
mod state;
pub mod util;

#[wasm_bindgen(start)]
fn init_wasm() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn launch_app() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, AppState::default);

    render! {
        Router::<Route> { }
    }
}
