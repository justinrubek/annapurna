use annapurna_data::types::{Ingredient, Recipe};

mod aggregators;
pub mod error;
pub mod program;
mod recipe;

#[cfg(test)]
mod tests;

use program::AscentProgram;
use recipe::{RecipeManager, RecipeResult};

pub fn recipe(recipes: Vec<Recipe>, has_ingredients: Vec<Ingredient>) -> RecipeResult {
    // convert to String
    let manager = RecipeManager::new(has_ingredients, recipes);

    manager.process()
}
