use std::time::{SystemTime, UNIX_EPOCH};
use chrono::Utc;

pub fn now_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros() as u64
}

pub fn get_findag_time_micro() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros() as u64
}

pub fn validate_hashtimer(timestamp: u64) -> Result<(), String> {
    let now = get_findag_time_micro();
    if timestamp > now {
        return Err("Timestamp is in the future".into());
    }
    if now - timestamp > 5_000_000 { // 5 seconds in microseconds
        return Err("Timestamp is too old".into());
    }
    Ok(())
}

pub fn current_timestamp_micros() -> u64 {
    get_findag_time_micro()
}
