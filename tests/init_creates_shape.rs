use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn init_creates_storage_shape_and_preserves_ledger() {
    let dir = TempProject::new();

    assert_success(&run_in(dir.path(), &["init"]));

    let ledger = dir.path().join(".r03bust/ledger.jsonl");
    let logs = dir.path().join(".r03bust/logs");
    assert!(ledger.is_file());
    assert!(logs.is_dir());

    fs::write(&ledger, "keep me\n").unwrap();

    assert_success(&run_in(dir.path(), &["init"]));

    assert_eq!(fs::read_to_string(&ledger).unwrap(), "keep me\n");
    assert!(logs.is_dir());
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
