pub(crate) mod server;

use server::ServerCommand;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Commands {
    Command(Command),
    /// commands for running the server
    Server(ServerCommand),
}

#[derive(clap::Args, Debug)]
pub(crate) struct Command {
    #[clap(subcommand)]
    pub command: BasicCommands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum BasicCommands {
    Run,
}
