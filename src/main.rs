mod capture;
mod cli;
mod commands;
mod core;
mod error;
mod util;

use crate::cli::{parse, Commands};
use crate::error::Result;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<u8> {
    match parse()? {
        Commands::Init => commands::init::run(),
        Commands::Run { command } => commands::run::run(command),
        Commands::Last => commands::last::run(),
        Commands::Ledger => commands::ledger::run(),
        Commands::Doctor => commands::doctor::run(),
    }
}
