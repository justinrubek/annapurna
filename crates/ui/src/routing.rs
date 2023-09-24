use crate::{
    components::{IngredientCreate, InventoryCreate, Recipe, RecipeCreate},
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
    #[route("/debug")]
    DebugPage {},
}

#[allow(non_snake_case)]
pub(crate) fn Nav(cx: Scope) -> Element {
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
    render! {
        style {
            style_contents
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
                to: Route::DebugPage {},
                "debug"
            }
        }

        Outlet::<Route> { }
    }
}

#[allow(non_snake_case)]
pub(crate) fn Index(cx: Scope) -> Element {
    cx.render(rsx! {
        p {
            r#"Annapurna is a cooking and lifestyle utility.
            Using it will allow you to improve your diet by making nutritional choices easier.
            You can use it to plan and track your diet, and it will save you time and money by reducing the amount of
            food you waste.
            "#,
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

    let creating_recipe = use_state(cx, || false);

    cx.render(rsx! {
        div {
            h1 { "Recipes" }

            button {
                onclick: |_| creating_recipe.set(true),
                "add recipe"
            }
            button {
                onclick: |_| {
                    let filename = "recipes.ron";
                    let text = ron::ser::to_string_pretty(&app_state.read().recipes, Default::default()).unwrap();
                    util::download_string(filename, &text).expect("failed to download");
                },
                "export recipes"
            }

            if *creating_recipe.get() {
                render! {
                    RecipeCreate {
                        on_create: |recipe| {
                            app_state.write().add_recipe(recipe);
                            creating_recipe.set(false);
                        },
                        on_cancel: |_| creating_recipe.set(false),
                    }
                }
            }

            app_state.read().recipes.iter().cloned().map(|recipe| {
                 rsx! {
                    div {
                        Recipe {
                            name: recipe.name.clone(),
                            ingredients: recipe.ingredients.iter().map(|ingredient| ingredient.name.clone()).collect(),
                        }
                        button {
                            onclick: move |_| {
                                app_state.write().remove_recipe(&recipe.name);
                            },
                            "remove"
                        }
                    }
                }
            })
        }
    })
}

#[allow(non_snake_case)]
pub(crate) fn DebugPage(cx: Scope) -> Element {
    cx.render(rsx! {
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
    })
}

#[allow(non_snake_case)]
pub(crate) fn AppIngredients(cx: Scope) -> Element {
    let app_state = use_shared_state::<AppState>(cx).unwrap();

    let creating_ingredient = use_state(cx, || false);

    cx.render(rsx! {
        div {
            h1 { "Ingredients" }

            button {
                onclick: |_| creating_ingredient.set(true),
                "add ingredient"
            }
            button {
                onclick: |_| {
                    let filename = "ingredients.ron";
                    let text = ron::ser::to_string_pretty(&app_state.read().ingredients, Default::default()).unwrap();
                    util::download_string(filename, &text).expect("failed to download");
                },
                "export ingredients"
            }

            if *creating_ingredient.get() {
                render! {
                    IngredientCreate {
                        on_create: |ingredient| {
                            app_state.write().add_ingredient(ingredient);
                            creating_ingredient.set(false);
                        },
                        on_cancel: |_| creating_ingredient.set(false),
                    }
                }
            }

            app_state.read().ingredients.iter().cloned().map(|ingredient| rsx! {
                div {
                    p { format!("name: {}", &ingredient.name) }
                    button {
                        onclick: move |_| {
                            app_state.write().remove_ingredient(&ingredient.name);
                        },
                        "remove"
                    }
                }
            })
        }
    })
}

#[allow(non_snake_case)]
pub(crate) fn AppInventory(cx: Scope) -> Element {
    let app_state = use_shared_state::<AppState>(cx).unwrap();

    let creating_inventory = use_state(cx, || false);

    cx.render(rsx! {
        div {
            h1 { "Inventory" }

            div {
                input {
                    r#type: "file",
                    multiple: false,
                    onchange: move |event| {
                        let app_state = app_state.clone();
                        let files = event.files.clone();

                        cx.spawn({
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
                onclick: |_| creating_inventory.set(true),
                "add ingredient"
            }
            button {
                onclick: |_| {
                    let filename = "inventory.ron";
                    let text = ron::ser::to_string_pretty(&app_state.read().inventory, Default::default()).unwrap();
                    util::download_string(filename, &text).expect("failed to download");
                },
                "export inventory"
            }

            if *creating_inventory.get() {
                render! {
                    InventoryCreate {
                        on_create: |ingredient| {
                            app_state.write().add_inventory(ingredient);
                            creating_inventory.set(false);
                        },
                        on_cancel: |_| creating_inventory.set(false),
                    }
                }
            }

            app_state.read().inventory.iter().cloned().map(|ingredient| rsx! {
                div {
                    p { format!("name: {}", &ingredient.name) }
                    button {
                        onclick: move |_| {
                            app_state.write().remove_inventory(&ingredient.name);
                        },
                        "remove"
                    }
                }
            })
        }
    })
}

#[allow(non_snake_case)]
pub(crate) fn AppLogicViewer(cx: Scope) -> Element {
    let app_state = use_shared_state::<AppState>(cx).unwrap();

    let recipe_result = use_state::<Option<RecipeResult>>(cx, || None);

    cx.render(rsx! {
        button {
            onclick: |_| {
                let app_state = app_state.clone();
                let recipe_result = recipe_result.clone();
                cx.spawn({
                    async move {
                        let app_state = app_state.read();
                        let result = annapurna_logic::recipe(app_state.recipes.clone(), app_state.inventory.clone());
                        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!("{:?}", result)));
                        recipe_result.set(Some(result));

                    }
                });
            },
            "perform logic"
        }

        if let Some(result) = recipe_result.get() {
            rsx! {
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
            }
        }
    })
}
