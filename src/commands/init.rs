use crate::error::Result;
use crate::util::fs;

pub fn run() -> Result<u8> {
    let cwd = std::env::current_dir()?;
    fs::create_storage(&cwd)?;
    println!(
        "initialized r03ust storage at {}",
        fs::storage_dir(&cwd).display()
    );
    Ok(0)
}
