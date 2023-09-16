use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

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

#[derive(PartialEq, Props)]
struct RecipeProps {
    name: String,
    ingredients: Vec<String>,
}

fn Recipe(cx: Scope<RecipeProps>) -> Element {
    cx.render(rsx! {
        div {
            h3 { "{cx.props.name}" }
            div {
                h4 { "Ingredients" }
                ul {
                    cx.props.ingredients.iter().map(|ingredient| rsx! {
                        li { "{ingredient}" }
                    })
                }
            }
        }
    })
}
