mod aggregators;
pub mod error;
pub mod program;
mod recipe;
pub mod types;

#[cfg(test)]
mod tests;

use program::AscentProgram;
use recipe::RecipeManager;
use types::{Ingredient, Recipe};

pub fn recipe(recipes: Vec<Recipe>, has_ingredients: Vec<Ingredient>) {
    // convert to String
    let manager = RecipeManager::new(has_ingredients, recipes);

    let res = manager.process();

    let can_make = res.can_make;
    let missing = res.missing;
    println!("Can make: {can_make:?}");
    println!("Missing: {missing:?}");
}
