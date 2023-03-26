#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Commands {
    Command(Command),
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
