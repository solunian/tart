use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod handle;

#[derive(Parser)]
#[command(version, author, about, long_about=None, arg_required_else_help(true))]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command()]
    /// initialize a tart repository
    Init {
        /// directory to initialize `.tart`
        directory: Option<PathBuf>,
    },
    #[command()]
    /// destroy a tart repository
    Destroy {
        /// directory to destroy `.tart`
        directory: Option<PathBuf>,
    },
    #[command()]
    Board { name: Option<String> },
}

fn main() {
    let cli = CLI::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Init { directory }) => {
            println!("{}", handle::init(directory.as_ref()));
        }
        Some(Commands::Destroy { directory }) => {
            println!("{}", handle::destroy(directory.as_ref()));
        }
        Some(Commands::Board { name }) => {
            println!("{}", handle::board(name.as_ref()));
        }
        None => {}
    };
}
