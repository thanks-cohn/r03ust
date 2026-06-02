use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn last_reads_newest_valid_receipt() {
    let dir = TempProject::new();
    init(dir.path());

    assert_success(&run_in(dir.path(), &["run", "--", "echo", "old"]));
    assert_success(&run_in(dir.path(), &["run", "--", "echo", "new"]));

    let output = run_in(dir.path(), &["last"]);
    assert_success(&output);
    let stdout = stdout(&output);
    assert!(stdout.contains("command: echo new"));
    assert!(stdout.contains("success: true"));
    assert!(stdout.contains("exit code: 0"));
}

#[test]
fn ledger_handles_empty_file() {
    let dir = TempProject::new();
    init(dir.path());

    let output = run_in(dir.path(), &["ledger"]);
    assert_success(&output);
    assert!(stdout(&output).contains("ledger is empty"));
}

#[test]
fn ledger_and_last_report_malformed_lines_without_panic() {
    let dir = TempProject::new();
    init(dir.path());
    assert_success(&run_in(dir.path(), &["run", "--", "echo", "good"]));

    let ledger = dir.path().join(".r03ust/ledger.jsonl");
    let first_line = fs::read_to_string(&ledger).unwrap();
    let mut file = OpenOptions::new().append(true).open(&ledger).unwrap();
    writeln!(file, "not-json").unwrap();
    writeln!(file, "{}", first_line.trim()).unwrap();

    let ledger_output = run_in(dir.path(), &["ledger"]);
    assert_success(&ledger_output);
    assert!(stdout(&ledger_output).contains("success=true"));
    assert!(stderr(&ledger_output).contains("malformed ledger line 2"));

    let last_output = run_in(dir.path(), &["last"]);
    assert_success(&last_output);
    assert!(stdout(&last_output).contains("command: echo good"));
    assert!(stderr(&last_output).contains("malformed ledger line 2"));
}

struct TempProject {
    path: PathBuf,
}

impl TempProject {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "r03ust-test-{}-{}",
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
    std::env::var_os("CARGO_BIN_EXE_r03ust")
        .map(PathBuf::from)
        .or_else(|| option_env!("CARGO_BIN_EXE_r03ust").map(PathBuf::from))
        .unwrap()
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "expected success, got {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        stdout(output),
        stderr(output)
    );
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}
