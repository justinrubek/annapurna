use std::collections::HashMap;

use crate::program::AscentProgram;
use crate::recipe::RecipeManager;
use crate::types::{Ingredient, Recipe};

fn build_ingredients(data: Vec<&str>) -> Vec<Ingredient> {
    data.into_iter().map(|i| Ingredient::new(i.to_string())).collect()
}

fn build_recipe(name: &str, ingredients: Vec<&str>) -> (String, Vec<Ingredient>) {
    (name.to_string(), build_ingredients(ingredients))
}

#[test]
fn finds_makeable() {
    let recipes = vec![
        build_recipe("cake", vec!["flour", "sugar", "eggs"]),
        build_recipe("pie", vec!["flour", "sugar", "eggs", "milk"]),
        build_recipe("bread", vec!["flour", "water", "salt", "yeast"]),
    ].into_iter().fold(HashMap::new(), |mut acc, (name, ingredients)| {
        let recipe = Recipe::new(name, ingredients.clone());
        acc.insert(recipe, ingredients);
        acc
    });

    let has_ingredients = build_ingredients(vec!["flour", "water", "salt", "yeast"]);

    let manager = RecipeManager::new(has_ingredients, recipes);
    let res = manager.process();

    let can_make = res.can_make;
    
    assert_eq!(can_make, vec!["bread"]);
}
