use annapurna_data::Facts;
use annapurna_logic::recipe;
use clap::Parser;

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
        },
        Commands::Server(server) => server.run().await?,
    }

    Ok(())
}
