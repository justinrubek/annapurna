use crate::{routing::Route, state::AppState};
use annapurna_data::types::{Ingredient, Recipe};
use dioxus::prelude::*;
use dioxus_router::components::Router;
use wasm_bindgen::prelude::*;

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
    let recipes = vec![Recipe::new(
        "pizza dough".to_string(),
        vec![
            Ingredient::new("flour".to_string()),
            Ingredient::new("water".to_string()),
            Ingredient::new("salt".to_string()),
            Ingredient::new("yeast".to_string()),
        ],
    )];
    use_shared_state_provider(cx, || AppState { recipes });

    render! {
        Router::<Route> { }
    }
}
