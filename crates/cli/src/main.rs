mod example;
use std::collections::HashMap;

use example::run as example_run;
mod named;
use named::run as named_run;

mod recipe;
use recipe::RecipeManager;

pub mod program;
use program::AscentProgram;

fn recipe() {
    let mut recipes = HashMap::new();
    recipes.insert("bread", vec!["flour", "water", "salt", "yeast"]);
    recipes.insert(
        "cake",
        vec!["flour", "sugar", "butter", "eggs", "milk", "salt"],
    );
    // convert recipe contents to strings
    let recipes = recipes
        .into_iter()
        .map(|(recipe, ingredients)| {
            (
                recipe.to_string(),
                ingredients.into_iter().map(|i| i.to_string()).collect(),
            )
        })
        .collect();

    let has_ingredients = vec!["flour", "water", "salt", "yeast", "butter"];

    // convert to String
    let has_ingredients = has_ingredients.into_iter().map(|i| i.to_string()).collect();

    let manager = RecipeManager::new(has_ingredients, recipes);

    let res = manager.process();

    let can_make = res.can_make;
    let missing = res.missing;
    let needs_ingredient = res.needs_ingredient;
    println!("Can make: {can_make:?}");
    println!("Missing: {missing:?}");
    println!("Needs: {needs_ingredient:?}");
}

fn main() {
    recipe();
}

#[allow(dead_code)]
fn example() {
    let edges = vec![(1, 2), (2, 3)];
    let connected = example_run(edges.clone());
    println!("Connected: {connected:?}");

    let connected = named_run(edges);
    println!("Connected: {connected:?}");
}
