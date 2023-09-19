use std::collections::HashSet;

use annapurna_data::types::{Ingredient, Recipe as RecipeData};
use dioxus::prelude::*;
use dioxus_html::input_data::keyboard_types::Key;

use crate::state::AppState;

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

#[derive(Props)]
pub(crate) struct CreateFormProps<'a, T> {
    on_create: EventHandler<'a, T>,
    on_cancel: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub(crate) fn RecipeCreate<'a>(cx: Scope<'a, CreateFormProps<'a, RecipeData>>) -> Element<'a> {
    let app_state = use_shared_state::<AppState>(cx).unwrap();
    let name = use_state(cx, || "".to_string());
    let ingredients = use_ref::<HashSet<String>>(cx, HashSet::new);

    let ingredient_input = use_state(cx, || "".to_string());

    cx.render(rsx! {
        div {
            style: r#"display: flex; flex-direction: column; border: 1px solid black; padding: 1rem;"#,

            h3 { "Create Recipe" }

            label { r#for: "name", "Name" }
            input {
                id: "name",
                value: "{name}",
                oninput: |event| name.set(event.value.clone()),
            }

            label { r#for: "ingredients", "Add ingredient" }
            input {
                id: "ingredients",
                list: "annapurna-ingredients",
                // when enter is pressed, add the ingredient to the list
                onkeyup: |event| {
                    if event.key() == Key::Enter {
                        if ingredient_input.get().is_empty() {
                            return;
                        }

                        ingredients.write().insert(ingredient_input.get().clone());
                        ingredient_input.set("".to_string());
                    }
                },

                value: "{ingredient_input}",
                oninput: |event| ingredient_input.set(event.value.clone()),
            }

            h4 { "Ingredients" }
            ul {
                ingredients.read().iter().cloned().map(|ingredient| rsx! {
                    div {
                        key: "{ingredient}",
                        li { "{ingredient}" }
                        button {
                            value: "{ingredient}",
                            onclick: move |_| {
                                ingredients.write().remove(&ingredient);
                            },
                            "remove"
                        }
                    }
                })
            }
            button {
                onclick: |_| {
                    let recipe = annapurna_data::types::Recipe {
                        name: name.get().clone(),
                        ingredients: ingredients.read().iter().cloned().map(Ingredient::new).collect(),
                    };
                    cx.props.on_create.call(recipe);
                },
                "create"
            }
            button {
                onclick: |_| cx.props.on_cancel.call(()),
                "cancel"
            }

            Datalist {
                id: "annapurna-ingredients",
                items: app_state.read().ingredients.iter().map(|i| i.to_string()).collect(),
            }
        }
    })
}

#[allow(non_snake_case)]
pub(crate) fn IngredientCreate<'a>(cx: Scope<'a, CreateFormProps<'a, Ingredient>>) -> Element<'a> {
    let name = use_state(cx, || "".to_string());

    cx.render(rsx! {
        div {
            style: r#"display: flex; flex-direction: column; border: 1px solid black; padding: 1rem;"#,

            h3 { "Create Ingredient" }

            label { r#for: "name", "Name" }
            input {
                id: "name",
                value: "{name}",
                oninput: |event| name.set(event.value.clone()),
            }

            button {
                onclick: |_| {
                    let ingredient = annapurna_data::types::Ingredient {
                        name: name.get().clone(),
                    };
                    cx.props.on_create.call(ingredient);
                },
                "create"
            }
            button {
                onclick: |_| cx.props.on_cancel.call(()),
                "cancel"
            }
        }
    })
}

#[derive(Props, PartialEq)]
pub(crate) struct DatalistProps<'a> {
    id: &'a str,
    items: Vec<String>,
}

/// Displays a datalist component with the given items.
#[allow(non_snake_case)]
pub(crate) fn Datalist<'a>(cx: Scope<'a, DatalistProps<'a>>) -> Element<'a> {
    render! {
        datalist {
            id: "{cx.props.id}",

            cx.props.items.iter().map(|item| rsx! {
                option {
                    key: "{item}",
                    value: "{item}",
                    "{item}"
                }
            })
        }
    }
}
