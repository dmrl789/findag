#![no_main]
use libfuzzer_sys::fuzz_target;
use findag::core::types::Block; // Adjust the path if needed

fuzz_target!(|data: &[u8]| {
    let _ = bincode::deserialize::<Block>(data);
}); 