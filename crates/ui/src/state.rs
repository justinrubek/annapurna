use annapurna_data::types::Recipe;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub(crate) struct AppState {
    pub recipes: Vec<Recipe>,
}
