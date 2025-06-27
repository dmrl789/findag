#![no_main]
use libfuzzer_sys::fuzz_target;
use findag::api::http_server::Transaction; // Adjust the path if needed

fuzz_target!(|data: &[u8]| {
    let _ = serde_json::from_slice::<Transaction>(data);
}); 