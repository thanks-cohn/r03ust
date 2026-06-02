use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct GitInfo {
    pub git_branch: Option<String>,
    pub git_commit: Option<String>,
    pub git_dirty: Option<bool>,
}

pub fn capture(cwd: &Path) -> GitInfo {
    GitInfo {
        git_branch: git_output(cwd, &["rev-parse", "--abbrev-ref", "HEAD"]),
        git_commit: git_output(cwd, &["rev-parse", "HEAD"]),
        git_dirty: git_dirty(cwd),
    }
}

pub fn git_available() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn inside_git_repo(cwd: &Path) -> Option<bool> {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(cwd)
        .output()
        .ok()
        .map(|output| {
            output.status.success() && String::from_utf8_lossy(&output.stdout).trim() == "true"
        })
}

fn git_output(cwd: &Path, args: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
    (!value.is_empty()).then_some(value)
}

fn git_dirty(cwd: &Path) -> Option<bool> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(cwd)
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    Some(!output.stdout.is_empty())
}
