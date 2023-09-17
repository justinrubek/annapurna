use crate::{components::Recipe, state::AppState, util};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Debug, Clone, Routable)]
pub(crate) enum Route {
    #[route("/")]
    Index {},
    #[route("/app")]
    AppIndex {},
    #[route("/app/recipes")]
    AppRecipes {},
}

#[allow(non_snake_case)]
pub(crate) fn Index(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "hello, wasm!" }
        Link {
            to: Route::AppRecipes {},
            "recipes"
        }
        button {
            onclick: |_| async move {
                let filename = "test.txt";
                let text = "hello, wasm!";
                util::download_string(filename, text).expect("failed to download");
            },
            "download file"
        }
        Recipe {
            name: "pizza dough".to_string(),
            ingredients: vec!["flour".to_string(), "water".to_string(), "salt".to_string(), "yeast".to_string()],
        }
    })
}

#[allow(non_snake_case)]
pub(crate) fn AppIndex(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "app index" }
    })
}

#[allow(non_snake_case)]
pub(crate) fn AppRecipes(cx: Scope) -> Element {
    let app_state = use_shared_state::<AppState>(cx).unwrap();

    cx.render(rsx! {
        app_state.read().recipes.iter().map(|recipe| rsx! {
            Recipe {
                name: recipe.name.clone(),
                ingredients: recipe.ingredients.iter().map(|ingredient| ingredient.name.clone()).collect(),
            }
        })
    })
}
