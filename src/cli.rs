use crate::error::{R03bustError, Result};

#[derive(Debug)]
pub enum Commands {
    Init,
    Run { command: Vec<String> },
    Last,
    Ledger,
    Doctor,
}

pub fn parse() -> Result<Commands> {
    let mut args = std::env::args().skip(1);
    let Some(command) = args.next() else {
        return Err(R03bustError::Usage(usage()));
    };

    match command.as_str() {
        "init" => Ok(Commands::Init),
        "run" => {
            let rest: Vec<String> = args.collect();
            let command = if rest.first().is_some_and(|arg| arg == "--") {
                rest.into_iter().skip(1).collect()
            } else {
                rest
            };
            if command.is_empty() {
                Err(R03bustError::EmptyCommand)
            } else {
                Ok(Commands::Run { command })
            }
        }
        "last" => Ok(Commands::Last),
        "ledger" => Ok(Commands::Ledger),
        "doctor" => Ok(Commands::Doctor),
        "-h" | "--help" | "help" => Err(R03bustError::Usage(usage())),
        other => Err(R03bustError::Usage(format!(
            "unknown command `{other}`\n{}",
            usage()
        ))),
    }
}

fn usage() -> String {
    "usage: r03ust <init|run|last|ledger|doctor>".to_string()
}
