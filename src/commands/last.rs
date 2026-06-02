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

    let Some(receipt) = receipts.last() else {
        println!("no run receipts found in .r03ust/ledger.jsonl");
        return Ok(0);
    };

    println!("timestamp: {}", receipt.timestamp);
    println!("command: {}", receipt.command);
    println!("success: {}", receipt.success);
    println!("exit code: {}", receipt.exit_code);
    println!("duration: {} ms", receipt.duration_ms);
    println!("stdout log: {}", receipt.stdout_log);
    println!("stderr log: {}", receipt.stderr_log);
    println!("git branch: {}", display_option(&receipt.git_branch));
    println!("git commit: {}", display_option(&receipt.git_commit));
    println!("dirty state: {}", display_bool(receipt.git_dirty));

    Ok(0)
}

fn display_option(value: &Option<String>) -> &str {
    value.as_deref().unwrap_or("null")
}

fn display_bool(value: Option<bool>) -> String {
    value
        .map(|v| v.to_string())
        .unwrap_or_else(|| "null".to_string())
}
