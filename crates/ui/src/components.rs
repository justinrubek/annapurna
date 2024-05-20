use annapurna_data::types::{Ingredient, Recipe as RecipeData, Task as TaskData};
use dioxus::prelude::*;
use std::{collections::HashSet, time::Duration};
use wasm_bindgen::JsValue;

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

#[allow(non_snake_case)]
pub(crate) fn TaskCreate(props: CreateFormProps<TaskData>) -> Element {
    let mut description = use_signal(|| "".to_string());
    let mut duration = use_signal(|| "".to_string());
    let mut start_time = use_signal(|| "".to_string());

    rsx! {
        div {
            style: r#"display: flex; flex-direction: column; border: 1px solid black; padding: 1rem;"#,

            h3 { "Create Task" }

            label { r#for: "description", "Description" }
            input {
                id: "description",
                value: "{description}",
                oninput: move |event| description.set(event.value().clone()),
            }

            label { r#for: "duration", "Duration" }
            input {
                id: "duration",
                value: "{duration}",
                r#type: "number",
                oninput: move |event| duration.set(event.value().clone()),
            }

            label { r#for: "start_time", "Start Time" }
            input {
                id: "start_time",
                value: "{start_time}",
                r#type: "time",
                oninput: move |event| start_time.set(event.value().clone()),
            }

            button {
                onclick: move |_| {
                    let duration = if duration().is_empty() {
                        None
                    } else {
                        let duration_quantity: u64 = duration().parse().ok().unwrap();
                        let duration_seconds = duration_quantity * 60;
                        Some(Duration::new(duration_seconds, 0))
                    };
                    let start_time = if start_time().is_empty() {
                        None
                    } else {
                        web_sys::console::log_1(&JsValue::from_str(&start_time()));
                        Some(start_time().parse().ok().unwrap())
                    };

                    let task = TaskData {
                        description: description(),
                        completed: false,
                        duration,
                        start_time,
                        completion_time: None,
                    };
                    props.on_create.call(task);
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

#[derive(Clone, PartialEq, Props)]
pub struct TodoTaskProps {
    pub task: TaskData,
    pub on_complete: EventHandler<()>,
}

#[allow(non_snake_case)]
pub fn TodoTask(props: TodoTaskProps) -> Element {
    rsx! {
        div {
            style: r#"display: flex; flex-direction: column; border: 1px solid black; padding: 1rem;"#,

            h3 { "{props.task.description}" }
            {if props.task.completed {
                rsx!{
                    label { "Completed" }
                }
            } else {
                rsx!{
                    label { "Not Completed" }
                }
            }}

            {if let Some(duration) = props.task.duration {
                rsx!{
                    label { r#for: "duration", "Duration" }
                    input {
                        id: "duration",
                        value: format!("{duration:?}"),
                        disabled: true,
                    }
                }
            } else {
                rsx!{}
            }}

            {if let Some(start_time) = props.task.start_time {
                rsx!{
                    label { r#for: "start_time", "Start Time" }
                    input {
                        id: "start_time",
                        value: start_time.to_string(),
                        disabled: true,
                    }
                }
            } else {
                rsx!{}
            }}

            button {
                onclick: move |_| {
                    props.on_complete.call(());
                },
                "complete"
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
