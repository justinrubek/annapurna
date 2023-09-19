use annapurna_data::types::{Ingredient, Recipe};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub(crate) struct AppState {
    pub recipes: Vec<Recipe>,
    pub ingredients: Vec<Ingredient>,
}

impl AppState {
    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.push(recipe);
    }

    pub fn add_ingredient(&mut self, ingredient: Ingredient) {
        self.ingredients.push(ingredient);
    }

    pub fn remove_recipe(&mut self, name: &str) {
        self.recipes.retain(|r| r.name != name);
    }

    pub fn remove_ingredient(&mut self, name: &str) {
        self.ingredients.retain(|i| i.name != name);
    }
}
