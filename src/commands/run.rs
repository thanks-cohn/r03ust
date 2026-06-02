use crate::capture::{git, machine};
use crate::core::ledger_writer;
use crate::core::run_receipt::RunReceipt;
use crate::error::{R03bustError, Result};
use crate::util::{fs, time};
use std::process::Command;
use std::time::Instant;

pub fn run(command: Vec<String>) -> Result<u8> {
    if command.is_empty() {
        return Err(R03bustError::EmptyCommand);
    }

    let cwd = std::env::current_dir()?;
    fs::ensure_storage(&cwd)?;

    let started_at = time::now_utc();
    let safe_stamp = started_at.file_stamp();
    let stdout_log = fs::logs_dir(&cwd).join(format!("{safe_stamp}-stdout.log"));
    let stderr_log = fs::logs_dir(&cwd).join(format!("{safe_stamp}-stderr.log"));

    let instant = Instant::now();
    let output = Command::new(&command[0])
        .args(&command[1..])
        .current_dir(&cwd)
        .output()?;
    let duration_ms = instant.elapsed().as_millis();

    std::fs::write(&stdout_log, &output.stdout)?;
    std::fs::write(&stderr_log, &output.stderr)?;

    let exit_code = output.status.code().unwrap_or(1);
    let machine = machine::capture();
    let git = git::capture(&cwd);
    let receipt = RunReceipt {
        schema_version: 1,
        timestamp: started_at.display(),
        cwd: cwd.display().to_string(),
        command: command.join(" "),
        argv: command,
        exit_code,
        success: output.status.success(),
        duration_ms,
        stdout_log: path_for_receipt(&cwd, &stdout_log),
        stderr_log: path_for_receipt(&cwd, &stderr_log),
        os: machine.os,
        arch: machine.arch,
        hostname: machine.hostname,
        git_branch: git.git_branch,
        git_commit: git.git_commit,
        git_dirty: git.git_dirty,
    };

    ledger_writer::append(&cwd, &receipt)?;
    println!(
        "recorded r03ust receipt: success={} exit_code={}",
        receipt.success, receipt.exit_code
    );

    Ok(exit_code_to_u8(exit_code))
}

fn path_for_receipt(cwd: &std::path::Path, path: &std::path::Path) -> String {
    path.strip_prefix(cwd).unwrap_or(path).display().to_string()
}

fn exit_code_to_u8(exit_code: i32) -> u8 {
    u8::try_from(exit_code).unwrap_or(1)
}
