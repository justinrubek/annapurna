use crate::{
    api::resolve_recipes,
    components::{Recipe, RecipeCreate},
    state::AppState,
    util,
};
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
    use_future(cx, (), |_| resolve_recipes(app_state.clone()));

    let creating_recipe = use_state(cx, || false);

    cx.render(rsx! {
        div {
            h1 { "Recipes" }

            button {
                onclick: |_| creating_recipe.set(true),
                "add recipe"
            }

            if *creating_recipe.get() {
                render! {
                    RecipeCreate {
                        on_create: |recipe| {
                            app_state.write().recipes.push(recipe);
                            creating_recipe.set(false);
                        },
                        on_cancel: |_| creating_recipe.set(false),
                    }
                }
            }

            app_state.read().recipes.iter().map(|recipe| rsx! {
                Recipe {
                    name: recipe.name.clone(),
                    ingredients: recipe.ingredients.iter().map(|ingredient| ingredient.name.clone()).collect(),
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
    cx.render(rsx! {
        div { "app ingredients" }
    })
}

#[allow(non_snake_case)]
pub(crate) fn AppInventory(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "app inventory" }
    })
}
