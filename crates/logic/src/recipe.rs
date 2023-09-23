#![allow(clippy::clone_on_copy)]
#![allow(clippy::let_unit_value)]
#![allow(clippy::just_underscores_and_digits)]
#![allow(clippy::unused_unit)]

use annapurna_data::types;
use ascent::ascent;
use std::collections::HashMap;

use crate::aggregators::vec_missing;
use crate::program::AscentProgram;

type Ingredient = String;
type Recipe = String;

ascent! {
    pub (crate) struct RecipeProgram;
    relation recipe_ingredients(Recipe, Vec<Ingredient>);
    relation has(Ingredient);

    relation is_recipe(Recipe);
    is_recipe(x) <-- recipe_ingredients(x, _);

    relation missing(Recipe, Vec<Ingredient>);
    missing(recipe, contents) <-- recipe_ingredients(recipe, contents), for ingredient in contents.iter(), !has(ingredient);
    // the above implementation of `missing` does not filter out the ingredients that are present
    // the below implementation of `missing` does filter out the ingredients that are present
    relation missing_ingredients(Recipe, Vec<Ingredient>);
    missing_ingredients(recipe, missing) <--
        recipe_ingredients(recipe, contents),
        for ingredient in contents.iter(),
        !has(ingredient),
        agg missing = (vec_missing(contents.to_vec()))(x) in has(x);

    relation can_make(Recipe);
    can_make(recipe) <-- is_recipe(recipe), !missing(recipe, _);
}

pub struct RecipeManager {
    available_ingredients: Vec<types::Ingredient>,
    recipes: Vec<types::Recipe>,
}

impl RecipeManager {
    pub fn new(available_ingredients: Vec<types::Ingredient>, recipes: Vec<types::Recipe>) -> Self {
        Self {
            available_ingredients,
            recipes,
        }
    }

    /// Prepare the ascent program for running
    pub(crate) fn get_program(&self) -> RecipeProgram {
        let has = self
            .available_ingredients
            .iter()
            .map(|i| (i.to_string(),))
            .collect();

        let recipe_ingredients = self
            .recipes
            .iter()
            .map(|recipe| {
                (
                    recipe.name.clone(),
                    recipe
                        .ingredients
                        .iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>(),
                )
            })
            .collect();

        RecipeProgram {
            has,
            recipe_ingredients,
            ..Default::default()
        }
    }

    pub(crate) fn run(&self) -> RecipeProgram {
        let mut program = self.get_program();
        program.run();
        program
    }
}

impl AscentProgram for RecipeManager {
    type Output = RecipeResult;

    fn process(&self) -> Self::Output {
        let program = self.run();
        RecipeResult {
            can_make: program.can_make.into_iter().map(|(r,)| r).collect(),
            missing: program.missing_ingredients.into_iter().fold(
                HashMap::new(),
                |mut acc, (recipe, missing)| {
                    acc.insert(recipe, missing);
                    acc
                },
            ),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RecipeResult {
    pub can_make: Vec<Recipe>,
    // pub missing: Vec<(Recipe, Vec<Ingredient>)>,
    pub missing: HashMap<Recipe, Vec<Ingredient>>,
}
