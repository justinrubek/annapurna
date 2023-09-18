use annapurna_data::types::{Ingredient, Recipe};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub(crate) struct AppState {
    pub recipes: Vec<Recipe>,
    pub ingredients: Vec<Ingredient>,
}
