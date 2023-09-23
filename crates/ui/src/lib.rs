use crate::{
    api::{resolve_ingredients, resolve_recipes},
    routing::Route,
    state::AppState,
};
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
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, AppState::default);
    let app_state = use_shared_state::<AppState>(cx).unwrap();
    use_future(cx, (), |_| resolve_recipes(app_state.clone()));
    use_future(cx, (), |_| resolve_ingredients(app_state.clone()));

    render! {
        Router::<Route> { }
    }
}
