use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy)]
pub struct Timestamp {
    millis_since_epoch: u128,
}

impl Timestamp {
    pub fn display(self) -> String {
        format!("unix-ms:{}", self.millis_since_epoch)
    }

    pub fn file_stamp(self) -> String {
        self.millis_since_epoch.to_string()
    }
}

pub fn now_utc() -> Timestamp {
    let millis_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    Timestamp { millis_since_epoch }
}
