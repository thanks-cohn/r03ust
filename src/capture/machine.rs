#[derive(Debug, Clone)]
pub struct MachineInfo {
    pub os: String,
    pub arch: String,
    pub hostname: Option<String>,
}

pub fn capture() -> MachineInfo {
    MachineInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        hostname: hostname(),
    }
}

fn hostname() -> Option<String> {
    if let Ok(value) = std::env::var("HOSTNAME") {
        if !value.trim().is_empty() {
            return Some(value);
        }
    }

    let output = std::process::Command::new("hostname").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
    (!value.is_empty()).then_some(value)
}
