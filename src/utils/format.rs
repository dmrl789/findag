use std::fmt;
use hex;

pub struct ByteVec(pub Vec<u8>);

impl fmt::Display for ByteVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}

// Helper function to convert [u8; 8] to IVec
pub fn bytes_to_ivec(bytes: [u8; 8]) -> sled::IVec {
    sled::IVec::from(&bytes[..])
}

// Helper function to convert String to Vec<u8>
pub fn string_to_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

// Helper function to convert Vec<u8> to String
pub fn bytes_to_string(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).to_string()
}

// Helper function to convert i64 to u64
pub fn i64_to_u64(n: i64) -> Option<u64> {
    if n >= 0 {
        Some(n as u64)
    } else {
        None
    }
}

// Helper function to convert u64 to i64
pub fn u64_to_i64(n: u64) -> i64 {
    n as i64
} 