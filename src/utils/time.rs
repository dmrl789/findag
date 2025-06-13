use chrono::Utc;

pub fn get_findag_time_micro() -> u128 {
    let now = Utc::now();
    now.timestamp_micros() as u128
}
