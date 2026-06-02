use crate::capture::{git, machine};
use crate::error::Result;
use crate::util::fs;
use std::fs::OpenOptions;

pub fn run() -> Result<u8> {
    let cwd = std::env::current_dir()?;
    let machine = machine::capture();
    let git_available = git::git_available();
    let inside_git_repo = git::inside_git_repo(&cwd);

    println!("current working directory: {}", cwd.display());
    println!("OS: {}", machine.os);
    println!("architecture: {}", machine.arch);
    println!("git available: {}", git_available);
    println!(
        "inside git repo: {}",
        inside_git_repo
            .map(|value| value.to_string())
            .unwrap_or_else(|| "unknown".to_string())
    );

    let storage = fs::storage_dir(&cwd);
    let ledger = fs::ledger_path(&cwd);
    let logs = fs::logs_dir(&cwd);

    let storage_ok = storage.is_dir();
    let ledger_exists = ledger.is_file();
    let logs_exists = logs.is_dir();
    let ledger_readable = OpenOptions::new().read(true).open(&ledger).is_ok();
    let logs_writable = logs_exists && tempfile_in_logs_is_writable(&logs);

    println!(".r03bust exists: {}", storage_ok);
    println!("ledger.jsonl exists: {}", ledger_exists);
    println!("logs directory exists: {}", logs_exists);
    println!("ledger readable: {}", ledger_readable);
    println!("logs writable: {}", logs_writable);

    if storage_ok && ledger_exists && logs_exists && ledger_readable && logs_writable {
        println!("r03bust storage is usable");
        Ok(0)
    } else {
        println!("r03bust storage is missing or unusable; run `r03bust init`");
        Ok(1)
    }
}

fn tempfile_in_logs_is_writable(logs: &std::path::Path) -> bool {
    let probe = logs.join(".doctor-write-test");
    match OpenOptions::new().create_new(true).write(true).open(&probe) {
        Ok(_) => std::fs::remove_file(probe).is_ok(),
        Err(_) => false,
    }
}
