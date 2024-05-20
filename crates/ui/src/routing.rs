use crate::{
    components::{IngredientCreate, InventoryCreate, Recipe, RecipeCreate, TaskCreate, TodoTask},
    state::AppState,
    util,
};
use annapurna_data::types::Ingredient;
use annapurna_logic::recipe::RecipeResult;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Debug, Clone, Routable)]
pub(crate) enum Route {
    #[layout(Nav)]
    #[route("/")]
    Index {},
    #[route("/app")]
    AppIndex {},
    #[route("/app/recipes")]
    AppRecipes {},
    #[route("/app/ingredients")]
    AppIngredients {},
    #[route("/app/inventory")]
    AppInventory {},
    #[route("/app/logic/viewer")]
    AppLogicViewer {},
    #[route("/app/todo")]
    AppTodo {},
    #[route("/debug")]
    DebugPage {},
}

#[allow(non_snake_case)]
pub(crate) fn Nav() -> Element {
    let style_contents = r#"
        .navlink {
            margin: 0 1rem;

            &.active {
                color: red;
                font-weight: bold;
            }
        }

        .navlink:hover {
            cursor: pointer;
        }

        .nav {
            display: flex;
            flex-direction: row;
            border: 1px solid black;
            padding: 1rem;
        }
    "#;
    rsx! {
        style {
            {style_contents}
        }
        nav {
            class: "nav",
            Link {
                active_class: "active",
                class: "navlink",
                to: Route::Index {},
                "home"
            }
            Link {
                active_class: "active",
                class: "navlink",
                to: Route::AppRecipes {},
                "recipes"
            }
            Link {
                active_class: "active",
                class: "navlink",
                to: Route::AppIngredients {},
                "ingredients"
            }
            Link {
                active_class: "active",
                class: "navlink",
                to: Route::AppInventory {},
                "inventory"
            }
            Link {
                active_class: "active",
                class: "navlink",
                to: Route::AppLogicViewer {},
                "logic viewer"
            }
            Link {
                active_class: "active",
                class: "navlink",
                to: Route::AppTodo {},
                "todo"
            }
            Link {
                active_class: "active",
                class: "navlink",
                to: Route::DebugPage {},
                "debug"
            }
        }

        Outlet::<Route> { }
    }
}

#[allow(non_snake_case)]
pub(crate) fn Index() -> Element {
    rsx! {
        p {
            r#"Annapurna is a cooking and lifestyle utility.
            Using it will allow you to improve your diet by making nutritional choices easier.
            You can use it to plan and track your diet, and it will save you time and money by reducing the amount of
            food you waste.
            "#,
        }
    }
}

#[allow(non_snake_case)]
pub(crate) fn AppIndex() -> Element {
    rsx! {
        div { "app index" }
    }
}

#[allow(non_snake_case)]
pub(crate) fn AppRecipes() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let mut creating_recipe = use_signal(|| false);

    rsx! {
        div {
            h1 { "Recipes" }

            button {
                onclick: move |_| creating_recipe.set(true),
                "add recipe"
            }
            button {
                onclick: move |_| {
                    let filename = "recipes.ron";
                    let text = ron::ser::to_string_pretty(&app_state().recipes, Default::default()).unwrap();
                    util::download_string(filename, &text).expect("failed to download");
                },
                "export recipes"
            }

            if creating_recipe() {
                {rsx! {
                    RecipeCreate {
                        on_create: move |recipe| {
                            app_state.write().add_recipe(recipe);
                            creating_recipe.set(false);
                        },
                        on_cancel: move |_| creating_recipe.set(false),
                    }
                }}
            }

            {app_state().recipes.iter().cloned().map(move |recipe| {
                 rsx! {
                    div {
                        Recipe {
                            name: recipe.name.clone(),
                            ingredients: recipe.ingredients.iter().map(|ingredient| ingredient.name.clone()).collect(),
                        }
                        button {
                            onclick: move |_| {
                                app_state().remove_recipe(&recipe.name.clone());
                            },
                            "remove"
                        }
                    }
                }
            })}
        }
    }
}

#[allow(non_snake_case)]
pub(crate) fn DebugPage() -> Element {
    rsx! {
        div {
            h1 { "Debug" }
            button {
                onclick: |_| async move {
                    let filename = "test.txt";
                    let text = "hello, wasm!";
                    util::download_string(filename, text).expect("failed to download");
                },
                "download file"
            }
        }
    }
}

#[allow(non_snake_case)]
pub(crate) fn AppIngredients() -> Element {
    let mut app_state = consume_context::<Signal<AppState>>();
    let mut creating_ingredient = use_signal(|| false);

    rsx! {
        div {
            h1 { "Ingredients" }

            button {
                onclick: move |_| creating_ingredient.set(true),
                "add ingredient"
            }
            button {
                onclick: move |_| {
                    let filename = "ingredients.ron";
                    let text = ron::ser::to_string_pretty(&app_state().ingredients, Default::default()).unwrap();
                    util::download_string(filename, &text).expect("failed to download");
                },
                "export ingredients"
            }

            if creating_ingredient() {
                {rsx! {
                    IngredientCreate {
                        on_create: move |ingredient| {
                            app_state.write().add_ingredient(ingredient);
                            creating_ingredient.set(false);
                        },
                        on_cancel: move |_| creating_ingredient.set(false),
                    }
                }}
            }

            {app_state().ingredients.iter().cloned().map(|ingredient| rsx! {
                div {
                    p { {format!("name: {}", &ingredient.name)} }
                    button {
                        onclick: move |_| {
                            app_state.write().remove_ingredient(&ingredient.name);
                        },
                        "remove"
                    }
                }
            })}
        }
    }
}

#[allow(non_snake_case)]
pub(crate) fn AppInventory() -> Element {
    let mut app_state = consume_context::<Signal<AppState>>();
    let mut creating_inventory = use_signal(|| false);

    rsx! {
        div {
            h1 { "Inventory" }

            div {
                input {
                    r#type: "file",
                    multiple: false,
                    onchange: move |event| {
                        let files = event.files().clone();

                        spawn({
                            async move {
                                if let Some(file_engine) = files {
                                    let files = file_engine.files();
                                    let empty_filename = String::new();
                                    let filename = files.first().unwrap_or(&empty_filename);
                                    match file_engine.read_file(filename).await {
                                        Some(contents) => {
                                            let items: Vec<Ingredient> = ron::de::from_bytes(&contents).unwrap();
                                            app_state.write().set_inventory(items);
                                        }
                                        None => {
                                            tracing::error!("no content");
                                        }
                                    }
                                }
                            }
                        });
                    },
                }
            }
            button {
                onclick: move |_| creating_inventory.set(true),
                "add ingredient"
            }
            button {
                onclick: move |_| {
                    let filename = "inventory.ron";
                    let text = ron::ser::to_string_pretty(&app_state().inventory, Default::default()).unwrap();
                    util::download_string(filename, &text).expect("failed to download");
                },
                "export inventory"
            }

            if creating_inventory() {
                {rsx! {
                    InventoryCreate {
                        on_create: move |ingredient| {
                            app_state.write().add_inventory(ingredient);
                            creating_inventory.set(false);
                        },
                        on_cancel: move |_| creating_inventory.set(false),
                    }
                }}
            }

            {app_state().inventory.iter().cloned().map(|ingredient| rsx! {
                div {
                    p { {format!("name: {}", &ingredient.name)} }
                    button {
                        onclick: move |_| {
                            app_state.write().remove_inventory(&ingredient.name);
                        },
                        "remove"
                    }
                }
            })}
        }
    }
}

#[allow(non_snake_case)]
pub(crate) fn AppLogicViewer() -> Element {
    let app_state = consume_context::<Signal<AppState>>();
    let mut recipe_result = use_signal::<Option<RecipeResult>>(|| None);

    rsx! {
        button {
            onclick: move |_| {
                spawn({
                    async move {
                        let result = annapurna_logic::recipe(app_state().recipes.clone(), app_state().inventory.clone());
                        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("{:?}", result)));
                        recipe_result.set(Some(result));

                    }
                });
            },
            "perform logic"
        }

        if let Some(result) = recipe_result() {
            {rsx! {
                div {
                    h3 { "can make these recipes" }
                    ul {
                        for recipe in result.can_make.iter().cloned() {
                            li {
                                "{recipe}"
                            }
                        }
                    }

                    h3 { "missing items for these recipes" }
                    ul {
                        for (recipe, ingredients) in result.missing.iter() {
                            li {
                                h4 { "{recipe}" }
                                ul {
                                    for ingredient in ingredients.iter().cloned() {
                                        li { "{ingredient}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }}
        }
    }
}

#[allow(non_snake_case)]
pub(crate) fn AppTodo() -> Element {
    let mut app_state = use_context::<Signal<AppState>>();
    let mut creating_todo = use_signal(|| false);

    rsx! {
        div {
            h1 { "Todo" }

            button {
                onclick: move |_| creating_todo.set(true),
                "add todo"
            }
            button {
                onclick: move |_| {
                    let filename = "todo.ron";
                    let text = ron::ser::to_string_pretty(&app_state().todo, Default::default()).unwrap();
                    util::download_string(filename, &text).expect("failed to download");
                },
                "export todos"
            }

            if creating_todo() {
                {rsx! {
                    TaskCreate {
                        on_create: move |todo| {
                            app_state.write().add_todo(todo);
                            creating_todo.set(false);
                        },
                        on_cancel: move |_| creating_todo.set(false),
                    }
                }}
            }

            {app_state().todo.iter().cloned().map(move |todo| {
                let cloned_todo = todo.clone();
                rsx! {
                   div {
                       key: "{todo.description}",
                       TodoTask {
                           task: todo.clone(),
                           on_complete: move |_| {
                               app_state.write().complete_todo(cloned_todo.clone());
                           },
                       }
                       button {
                           onclick: move |_| {
                               app_state.write().remove_todo(todo.clone());
                           },
                           "remove"
                       }
                   }
                }
            })}
        }
    }
}
