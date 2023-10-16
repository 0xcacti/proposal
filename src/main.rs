use std::process;

use clap::{crate_version, Parser, Subcommand};
use proposal::proposal::Config;

/// proposal is a command line tool for getting a epoch proposal information for Ethereum
/// validators
#[derive(Debug, Parser)]
#[command(name="proposal", version=crate_version!(), about="proposal info", long_about = "Get validator proposal information for an epoch", arg_required_else_help(true))]
struct AppParser {
    // The RPC URL to use
    #[arg(
        long = "The consensus layer rpc url to use",
        required = false,
        global = true
    )]
    rpc_url: Option<String>,
    /// The subcommand to run
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Get a list of validators that will be proposers for the given epoch
    Get,
    /// Check if the given validators will be proposers for the given epoch
    Check {
        /// The list of validators in question
        #[arg(required = true)]
        validators: Vec<String>,
    },
}
fn main() {
    let config = Config::from(Config::figment()).unwrap();
    let app = AppParser::parse();
    match app.command {
        Some(Commands::Get) => {
            println!("Get");
        }
        Some(Commands::Check { validators }) => {
            println!("Check: {:?}", validators);
        }
        None => {
            eprintln!("No command provided");
            process::exit(1);
        }
    }
}
