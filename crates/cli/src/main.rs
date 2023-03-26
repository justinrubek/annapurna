use annapurna_logic::{
    recipe,
    types::{Ingredient, Recipe},
};
use clap::Parser;
use std::collections::HashMap;

pub mod commands;
use commands::{BasicCommands, Commands};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        Commands::Server(server) => server.run().await?,
    }

    Ok(())
}
