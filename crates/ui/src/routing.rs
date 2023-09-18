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
