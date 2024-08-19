use annapurna_data::Facts;
use annapurna_logic::recipe;
use clap::Parser;
use std::collections::HashSet;

pub mod commands;
use commands::{BasicCommands, Commands};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args = commands::Args::parse();
    match args.command {
        Commands::Command(command) => match command.command {
            BasicCommands::Run => {
                let facts = Facts::read_from_directory("facts")?;

                let res = recipe(facts.recipes, facts.inventory);
                let can_make = res.can_make;
                let missing = res.missing;
                println!("Can make: {can_make:?}");
                println!("Missing: {missing:?}");
            }
            BasicCommands::FoodData { file, .. } => {
                let data: fooddata::FoundationFoods = {
                    let contents = tokio::fs::read_to_string(file).await?;
                    serde_json::from_str(&contents)?
                };
                let mut nutrients = HashSet::new();
                data.foundation_foods.iter().for_each(|food| {
                    let macros = food.macros();
                    println!("{}: {macros:?}\n", food.description);
                    println!("{:?}\n", food.food_portions);
                    food.food_nutrients.iter().for_each(|nutrient| {
                        nutrients.insert(nutrient.nutrient.name.clone());
                    });
                });

                // nutrients.iter().for_each(|nutrient| println!("{nutrient:?}"));
            }
        },
        Commands::Server(server) => server.run().await?,
    }

    Ok(())
}
