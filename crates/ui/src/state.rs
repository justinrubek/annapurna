use annapurna_data::types::{Ingredient, Recipe};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub(crate) struct AppState {
    pub recipes: Vec<Recipe>,
    pub ingredients: Vec<Ingredient>,
    pub inventory: Vec<Ingredient>,
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

    pub fn add_inventory(&mut self, ingredient: Ingredient) {
        self.inventory.push(ingredient);
    }

    pub fn remove_inventory(&mut self, name: &str) {
        self.inventory.retain(|i| i.name != name);
    }

    pub fn set_inventory(&mut self, ingredients: Vec<Ingredient>) {
        self.inventory = ingredients;
    }
}
