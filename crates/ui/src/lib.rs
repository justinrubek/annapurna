use crate::routing::Route;
use dioxus::prelude::*;
use dioxus_router::components::Router;
use wasm_bindgen::prelude::*;

mod components;
mod routing;
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
    render! {
        Router::<Route> { }
    }
}
