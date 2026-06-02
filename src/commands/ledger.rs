use crate::core::ledger_writer;
use crate::error::Result;
use crate::util::fs;

pub fn run() -> Result<u8> {
    let cwd = std::env::current_dir()?;
    if let Err(error) = fs::ensure_storage(&cwd) {
        println!("no usable r03ust storage: {error}");
        return Ok(0);
    }

    let (receipts, malformed) = ledger_writer::read_valid(&cwd)?;
    for (line, error) in malformed {
        eprintln!("malformed ledger line {line}: {error}");
    }

    if receipts.is_empty() {
        println!("ledger is empty: no run receipts found");
        return Ok(0);
    }

    for (index, receipt) in receipts.iter().enumerate() {
        println!("{} | {}", index + 1, receipt.compact_line());
    }

    Ok(0)
}
