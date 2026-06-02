use crate::core::run_receipt::RunReceipt;
use crate::error::Result;
use crate::util::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub type MalformedLedgerLine = (usize, String);
pub type LedgerRead = (Vec<RunReceipt>, Vec<MalformedLedgerLine>);

pub fn append(cwd: &Path, receipt: &RunReceipt) -> Result<()> {
    let mut ledger = OpenOptions::new()
        .create(true)
        .append(true)
        .open(fs::ledger_path(cwd))?;
    ledger.write_all(receipt.to_json_line().as_bytes())?;
    ledger.write_all(b"\n")?;
    ledger.flush()?;
    Ok(())
}

pub fn read_valid(cwd: &Path) -> Result<LedgerRead> {
    let ledger = fs::ledger_path(cwd);
    let file = OpenOptions::new().read(true).open(ledger)?;
    let reader = BufReader::new(file);
    let mut receipts = Vec::new();
    let mut malformed = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line_number = index + 1;
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        match RunReceipt::from_json_line(&line) {
            Ok(receipt) => receipts.push(receipt),
            Err(error) => malformed.push((line_number, error)),
        }
    }

    Ok((receipts, malformed))
}
