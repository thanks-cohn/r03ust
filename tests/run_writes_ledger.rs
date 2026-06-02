use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn run_records_success_and_writes_stdout_log() {
    let dir = TempProject::new();
    init(dir.path());

    let output = run_in(dir.path(), &["run", "--", "echo", "hello"]);
    assert_success(&output);

    let lines = ledger_lines(dir.path());
    assert_eq!(lines.len(), 1);
    let receipt = &lines[0];
    assert!(receipt.starts_with('{') && receipt.ends_with('}'));
    assert!(receipt.contains("\"schema_version\":1"));
    assert!(receipt.contains("\"command\":\"echo hello\""));
    assert!(receipt.contains("\"argv\":[\"echo\",\"hello\"]"));
    assert!(receipt.contains("\"exit_code\":0"));
    assert!(receipt.contains("\"success\":true"));
    assert!(receipt.contains("\"duration_ms\":"));
    assert!(receipt.contains(&format!("\"cwd\":\"{}\"", dir.path().display())));
    assert!(receipt.contains("\"timestamp\":\"unix-ms:"));
    assert!(receipt.contains("\"os\":"));
    assert!(receipt.contains("\"arch\":"));

    let stdout_log = dir.path().join(json_string_field(receipt, "stdout_log"));
    let stderr_log = dir.path().join(json_string_field(receipt, "stderr_log"));
    assert_eq!(fs::read_to_string(stdout_log).unwrap(), "hello\n");
    assert_eq!(fs::read_to_string(stderr_log).unwrap(), "");
}

#[test]
fn run_records_failure_exit_code_and_stderr_log() {
    let dir = TempProject::new();
    init(dir.path());

    let output = run_in(
        dir.path(),
        &["run", "--", "sh", "-c", "echo bad >&2; exit 7"],
    );
    assert_eq!(output.status.code(), Some(7));

    let lines = ledger_lines(dir.path());
    assert_eq!(lines.len(), 1);
    let receipt = &lines[0];
    assert!(receipt.contains("\"command\":\"sh -c echo bad >&2; exit 7\""));
    assert!(receipt.contains("\"exit_code\":7"));
    assert!(receipt.contains("\"success\":false"));

    let stdout_log = dir.path().join(json_string_field(receipt, "stdout_log"));
    let stderr_log = dir.path().join(json_string_field(receipt, "stderr_log"));
    assert_eq!(fs::read_to_string(stdout_log).unwrap(), "");
    assert_eq!(fs::read_to_string(stderr_log).unwrap(), "bad\n");
}

struct TempProject {
    path: PathBuf,
}

impl TempProject {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "r03bust-test-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&path).unwrap();
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempProject {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn init(path: &Path) {
    assert_success(&run_in(path, &["init"]));
}

fn run_in(path: &Path, args: &[&str]) -> Output {
    Command::new(bin())
        .current_dir(path)
        .args(args)
        .output()
        .unwrap()
}

fn bin() -> PathBuf {
    std::env::var_os("CARGO_BIN_EXE_r03bust")
        .map(PathBuf::from)
        .or_else(|| option_env!("CARGO_BIN_EXE_r03bust").map(PathBuf::from))
        .unwrap()
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "expected success, got {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn ledger_lines(path: &Path) -> Vec<String> {
    fs::read_to_string(path.join(".r03bust/ledger.jsonl"))
        .unwrap()
        .lines()
        .map(ToOwned::to_owned)
        .collect()
}

fn json_string_field(line: &str, field: &str) -> String {
    let marker = format!("\"{field}\":\"");
    let start = line.find(&marker).unwrap() + marker.len();
    let rest = &line[start..];
    let end = rest.find('"').unwrap();
    rest[..end].to_string()
}
