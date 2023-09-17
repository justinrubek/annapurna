use crate::state::AppState;
use annapurna_data::types::Recipe;
use dioxus::hooks::UseSharedState;

pub const BASE_API_URL: &str = "/api";
pub const RECIPE_API_URL: &str = "/recipes";

/// Format a relative path to an absolute URL for the API.
fn format_url(path: &str) -> String {
    // determine the absolute path to base of the current page from the browser
    let current_url = web_sys::window().unwrap().location().origin().unwrap();

    let url = format!("{}{}{}", current_url, BASE_API_URL, path);
    url
}

/// Retrieves recipes from the API.
pub async fn get_recipes() -> Result<Vec<Recipe>, reqwest::Error> {
    let url = format_url(RECIPE_API_URL);
    reqwest::get(&url).await?.json().await
}

/// Retrieves recipes from the API and updates the app state.
pub async fn resolve_recipes(app_state: UseSharedState<AppState>) {
    let recipes = get_recipes().await.unwrap();
    app_state.write().recipes = recipes;
}
