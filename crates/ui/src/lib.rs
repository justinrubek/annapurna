use crate::{
    api::{resolve_ingredients, resolve_recipes},
    components::Datalist,
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
    dioxus::launch(app);
}

fn app() -> Element {
    let app_state = use_context_provider(|| Signal::new(AppState::default()));
    use_future(move || resolve_recipes(app_state));
    use_future(move || resolve_ingredients(app_state));

    rsx! {
        Router::<Route> { }

        button {
            onclick: move |_| resolve_recipes(app_state),
            "Refresh"
        }

        Datalist {
            id: "annapurna-ingredients",
            items: app_state().ingredients.iter().map(|i| i.to_string()).collect(),
        }
    }
}
