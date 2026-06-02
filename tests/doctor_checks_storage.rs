use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn doctor_fails_before_init_and_passes_after_init() {
    let dir = TempProject::new();

    let before = run_in(dir.path(), &["doctor"]);
    assert_eq!(before.status.code(), Some(1));
    let before_stdout = stdout(&before);
    assert!(before_stdout.contains(".r03ust exists: false"));
    assert!(before_stdout.contains("r03ust storage is missing or unusable"));

    assert_success(&run_in(dir.path(), &["init"]));

    let after = run_in(dir.path(), &["doctor"]);
    assert_success(&after);
    let after_stdout = stdout(&after);
    assert!(after_stdout.contains("ledger readable: true"));
    assert!(after_stdout.contains("logs writable: true"));
    assert!(after_stdout.contains("r03ust storage is usable"));
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
        String::from_utf8_lossy(&output.stderr)
    );
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}
