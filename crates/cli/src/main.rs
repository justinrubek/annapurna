use clap::Parser;
use std::collections::HashMap;

mod aggregators;
mod example;

use example::run as example_run;
mod named;
use named::run as named_run;

mod recipe;
use recipe::RecipeManager;

pub mod program;
use program::AscentProgram;
use types::{Ingredient, Recipe};

pub mod commands;
use commands::{BasicCommands, Commands};
pub mod types;

#[cfg(test)]
mod tests;

fn recipe(recipes: Vec<Recipe>, has_ingredients: Vec<Ingredient>) {
    // convert to String
    let manager = RecipeManager::new(has_ingredients, recipes);

    let res = manager.process();

    let can_make = res.can_make;
    let missing = res.missing;
    println!("Can make: {can_make:?}");
    println!("Missing: {missing:?}");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args = commands::Args::parse();
    match args.command {
        Commands::Command(command) => match command.command {
            BasicCommands::Run => {
                let recipe_contents = std::fs::read_to_string("facts/recipes.ron").unwrap();
                let recipes_data: HashMap<String, Vec<String>> =
                    ron::from_str(&recipe_contents).unwrap();
                let recipes = Recipe::from_hashmap(recipes_data).into_keys().collect();

                let inventory_contents = std::fs::read_to_string("facts/inventory.ron").unwrap();
                let inventory_data: Vec<String> = ron::from_str(&inventory_contents).unwrap();
                let inventory: Vec<Ingredient> =
                    inventory_data.into_iter().map(Ingredient::new).collect();

                println!("Recipes:");
                // recipes.iter().for_each(|r| println!("{r}"));

                recipe(recipes, inventory);
            }
        },
    }

    Ok(())
}

#[allow(dead_code)]
fn example() {
    let edges = vec![(1, 2), (2, 3)];
    let connected = example_run(edges.clone());
    println!("Connected: {connected:?}");

    let connected = named_run(edges);
    println!("Connected: {connected:?}");
}
