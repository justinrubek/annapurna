#![allow(clippy::clone_on_copy)]
#![allow(clippy::let_unit_value)]
#![allow(clippy::unused_unit)]
use std::collections::HashMap;

use ascent::ascent;

use crate::program::AscentProgram;

type Ingredient = String;
type Recipe = String;

ascent! {
    pub (crate) struct RecipeProgram;
    relation needs_ingredient(Recipe, Ingredient);
    relation has(Ingredient);

    relation is_recipe(Recipe);
    is_recipe(x) <-- needs_ingredient(x, _);

    relation missing(Recipe, Ingredient);
    missing(recipe, ingredient) <-- needs_ingredient(recipe, ingredient), !has(ingredient);

    relation can_make(Recipe);

    can_make(recipe) <-- is_recipe(recipe), !missing(recipe, _);
}

pub struct RecipeManager {
    available_ingredients: Vec<Ingredient>,
    recipes: HashMap<Recipe, Vec<Ingredient>>,
}

impl RecipeManager {
    pub fn new(
        available_ingredients: Vec<Ingredient>,
        recipes: HashMap<Recipe, Vec<Ingredient>>,
    ) -> Self {
        Self {
            available_ingredients,
            recipes,
        }
    }

    /// Prepare the ascent program for running
    pub(crate) fn get_program(&self) -> RecipeProgram {
        // Create a needs entry for each ingredient in each recipe
        let needs = self
            .recipes
            .iter()
            .flat_map(|(recipe, ingredients)| {
                ingredients
                    .iter()
                    .map(move |ingredient| (recipe.clone(), ingredient.clone()))
            })
            .collect::<Vec<(Recipe, Ingredient)>>();

        let has = self
            .available_ingredients
            .clone()
            .into_iter()
            .map(|i| (i,))
            .collect();

        RecipeProgram {
            has,
            needs_ingredient: needs,
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
            missing: program.missing.into_iter().map(|(r, i)| (r, i)).collect(),
            needs_ingredient: program
                .needs_ingredient
                .into_iter()
                .map(|(r, i)| (r, i))
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct RecipeResult {
    pub can_make: Vec<Recipe>,
    pub missing: Vec<(Recipe, Ingredient)>,
    pub needs_ingredient: Vec<(Recipe, Ingredient)>,
}
