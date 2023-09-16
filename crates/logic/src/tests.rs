use std::collections::{HashMap, HashSet};

use crate::program::AscentProgram;
use crate::recipe::RecipeManager;
use annapurna_data::types::{Ingredient, Recipe};

fn build_ingredients(data: Vec<&str>) -> Vec<Ingredient> {
    data.into_iter()
        .map(|i| Ingredient::new(i.to_string()))
        .collect()
}

fn build_recipe(name: &str, ingredients: Vec<&str>) -> Recipe {
    Recipe::new(name.to_string(), build_ingredients(ingredients))
}

#[test]
fn finds_makeable() {
    let recipes = vec![
        build_recipe("cake", vec!["flour", "sugar", "eggs"]),
        build_recipe("pie", vec!["flour", "sugar", "eggs", "milk"]),
        build_recipe("bread", vec!["flour", "water", "salt", "yeast"]),
    ];

    let has_ingredients = build_ingredients(vec!["flour", "water", "salt", "yeast"]);

    let manager = RecipeManager::new(has_ingredients, recipes);
    let res = manager.process();

    let can_make = res.can_make;

    assert_eq!(can_make, vec!["bread"]);
}

#[test]
fn missing_ingredients() {
    let recipes = vec![
        build_recipe("mayo", vec!["eggs", "oil", "vinegar", "salt"]),
        build_recipe("cake", vec!["flour", "sugar", "eggs", "salt", "milk"]),
        build_recipe("pie", vec!["flour", "sugar", "eggs", "milk"]),
        build_recipe(
            "enriched-bread",
            vec!["flour", "water", "salt", "yeast", "milk", "oil"],
        ),
    ];

    let has_ingredients = build_ingredients(vec!["flour", "water", "salt", "yeast", "milk", "oil"]);

    let manager = RecipeManager::new(has_ingredients, recipes);
    let res = manager.process();

    let missing = res.missing;

    let mut expected = HashMap::new();
    expected.insert(
        "cake".to_string(),
        vec!["eggs".to_string(), "sugar".to_string()],
    );
    expected.insert(
        "pie".to_string(),
        vec!["eggs".to_string(), "sugar".to_string()],
    );
    expected.insert(
        "mayo".to_string(),
        vec!["eggs".to_string(), "vinegar".to_string()],
    );

    // convert hashmap values to hashsets
    let missing = missing.into_iter().fold(HashMap::new(), |mut acc, (k, v)| {
        acc.insert(k, v.into_iter().collect::<HashSet<_>>());
        acc
    });

    let expected = expected
        .into_iter()
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.insert(k, v.into_iter().collect::<HashSet<_>>());
            acc
        });

    assert_eq!(missing, expected);
}
