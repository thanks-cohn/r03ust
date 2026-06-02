use crate::error::{R03bustError, Result};
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};

pub const STORAGE_DIR: &str = ".r03ust";
pub const LEDGER_FILE: &str = "ledger.jsonl";
pub const LOGS_DIR: &str = "logs";

pub fn storage_dir(cwd: &Path) -> PathBuf {
    cwd.join(STORAGE_DIR)
}

pub fn ledger_path(cwd: &Path) -> PathBuf {
    storage_dir(cwd).join(LEDGER_FILE)
}

pub fn logs_dir(cwd: &Path) -> PathBuf {
    storage_dir(cwd).join(LOGS_DIR)
}

pub fn ensure_storage(cwd: &Path) -> Result<()> {
    let storage = storage_dir(cwd);
    let ledger = ledger_path(cwd);
    let logs = logs_dir(cwd);

    if !storage.is_dir() {
        return Err(R03bustError::StorageMissing);
    }
    if !ledger.is_file() {
        return Err(R03bustError::LedgerMissing(ledger));
    }
    if !logs.is_dir() {
        return Err(R03bustError::LogsMissing(logs));
    }

    Ok(())
}

pub fn create_storage(cwd: &Path) -> Result<()> {
    fs::create_dir_all(logs_dir(cwd))?;
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(ledger_path(cwd))?;
    Ok(())
}
