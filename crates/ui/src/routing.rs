use crate::{components::Recipe, util};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Debug, Clone, Routable)]
pub(crate) enum Route {
    #[route("/")]
    Index {},
    /*
    #[route("/app")]
    AppIndex {},
    #[route("/app/recipes")]
    AppRecipes {},
    */
}

#[allow(non_snake_case)]
pub(crate) fn Index(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "hello, wasm!" }
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
