use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub(crate) struct RecipeProps {
    name: String,
    ingredients: Vec<String>,
}

#[allow(non_snake_case)]
pub(crate) fn Recipe(cx: Scope<RecipeProps>) -> Element {
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
