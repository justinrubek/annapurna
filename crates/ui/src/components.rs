use std::collections::HashSet;

use annapurna_data::types::{Ingredient, Recipe as RecipeData};
use dioxus::prelude::*;

#[derive(Clone, PartialEq, Props)]
pub(crate) struct RecipeProps {
    name: String,
    ingredients: Vec<String>,
}

#[allow(non_snake_case)]
pub(crate) fn Recipe(props: RecipeProps) -> Element {
    rsx! {
        div {
            h3 { "{props.name}" }
            div {
                h4 { "Ingredients" }
                ul {
                    {props.ingredients.iter().map(|ingredient| rsx! {
                        li { "{ingredient}" }
                    })}
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub(crate) struct CreateFormProps<T: 'static + Clone + PartialEq> {
    on_create: EventHandler<T>,
    on_cancel: EventHandler<()>,
}

#[allow(non_snake_case)]
pub(crate) fn RecipeCreate(props: CreateFormProps<RecipeData>) -> Element {
    let mut name = use_signal(|| "".to_string());
    let mut ingredients = use_signal::<HashSet<String>>(HashSet::new);
    let mut ingredient_input = use_signal(|| "".to_string());

    rsx! {
        div {
            style: r#"display: flex; flex-direction: column; border: 1px solid black; padding: 1rem;"#,

            h3 { "Create Recipe" }

            label { r#for: "name", "Name" }
            input {
                id: "name",
                value: "{name}",
                oninput: move |event| name.set(event.value().clone()),
            }

            label { r#for: "ingredients", "Add ingredient" }
            input {
                id: "ingredients",
                list: "annapurna-ingredients",
                // when enter is pressed, add the ingredient to the list
                onkeyup: move |event| {
                    if event.key() == Key::Enter {
                        if ingredient_input().is_empty() {
                            return;
                        }

                        ingredients().insert(ingredient_input().clone());
                        ingredient_input.set("".to_string());
                    }
                },

                value: "{ingredient_input}",
                oninput: move |event| ingredient_input.set(event.value().clone()),
            }

            h4 { "Ingredients" }
            ul {
                {ingredients.read().iter().cloned().map(|ingredient| rsx! {
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
                })}
            }
            button {
                onclick: move |_| {
                    let recipe = annapurna_data::types::Recipe {
                        name: name().clone(),
                        ingredients: ingredients.read().iter().cloned().map(Ingredient::new).collect(),
                    };
                    props.on_create.call(recipe);
                },
                "create"
            }
            button {
                onclick: move |_| props.on_cancel.call(()),
                "cancel"
            }
        }
    }
}

#[allow(non_snake_case)]
pub(crate) fn IngredientCreate(props: CreateFormProps<Ingredient>) -> Element {
    let mut name = use_signal(|| "".to_string());

    rsx! {
        div {
            style: r#"display: flex; flex-direction: column; border: 1px solid black; padding: 1rem;"#,

            h3 { "Create Ingredient" }

            label { r#for: "name", "Name" }
            input {
                id: "name",
                value: "{name}",
                oninput: move |event| name.set(event.value().clone()),
            }

            button {
                onclick: move |_| {
                    let ingredient = annapurna_data::types::Ingredient {
                        name: name(),
                    };
                    props.on_create.call(ingredient);
                },
                "create"
            }
            button {
                onclick: move |_| props.on_cancel.call(()),
                "cancel"
            }
        }
    }
}

#[allow(non_snake_case)]
pub(crate) fn InventoryCreate(props: CreateFormProps<Ingredient>) -> Element {
    let mut name = use_signal(|| "".to_string());

    rsx! {
        div {
            style: r#"display: flex; flex-direction: column; border: 1px solid black; padding: 1rem;"#,

            h3 { "Create Inventory Item" }

            label { r#for: "name", "Name" }
            input {
                id: "name",
                value: "{name}",
                list: "annapurna-ingredients",
                oninput: move |event| name.set(event.value().clone()),
            }

            button {
                onclick: move |_| {
                    let ingredient = annapurna_data::types::Ingredient {
                        name: name(),
                    };
                    props.on_create.call(ingredient);
                },
                "create"
            }
            button {
                onclick: move |_| props.on_cancel.call(()),
                "cancel"
            }
        }
    }
}

#[derive(Clone, Props, PartialEq)]
pub(crate) struct DatalistProps {
    id: String,
    items: Vec<String>,
}

/// Displays a datalist component with the given items.
#[allow(non_snake_case)]
pub(crate) fn Datalist(props: DatalistProps) -> Element {
    rsx! {
        datalist {
            id: "{props.id}",
            {props.items.iter().map(|item| rsx! {
                option {
                    key: "{item}",
                    value: "{item}",
                    "{item}"
                }
            })}
        }
    }
}
