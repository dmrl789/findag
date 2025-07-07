use sha2::{Sha256, Digest};

/// Decodes a HashTimer and extracts its components
/// 
/// HashTimer structure: [FinDAG Time prefix (13-14 hex chars)][SHA256 hash suffix]
/// 
/// # Arguments
/// * `hashtimer_hex` - The HashTimer as a hex string (with or without 0x prefix)
/// 
/// # Returns
/// * `Option<(u64, String, String)>` - (FinDAG Time, Time prefix hex, Hash suffix hex)
pub fn decode_hashtimer(hashtimer_hex: &str) -> Option<(u64, String, String)> {
    // Remove 0x prefix if present
    let clean_hex = hashtimer_hex.trim_start_matches("0x");
    
    // HashTimer should be 64 hex characters (32 bytes)
    if clean_hex.len() != 64 {
        return None;
    }
    
    // Extract the first 13-14 hex characters as FinDAG Time prefix
    // FinDAG Time is typically 13-14 hex digits (52-56 bits)
    let time_prefix_hex = &clean_hex[..14]; // First 14 hex chars = 56 bits
    let hash_suffix_hex = &clean_hex[14..]; // Remaining 50 hex chars = 200 bits
    
    // Convert time prefix to u64
    let time_value = u64::from_str_radix(time_prefix_hex, 16).ok()?;
    
    Some((
        time_value,
        time_prefix_hex.to_string(),
        hash_suffix_hex.to_string()
    ))
}

/// Validates that a HashTimer follows the expected structure
/// 
/// # Arguments
/// * `hashtimer_hex` - The HashTimer as a hex string
/// * `expected_content` - The content that should produce the hash suffix
/// 
/// # Returns
/// * `bool` - True if the HashTimer is valid
pub fn validate_hashtimer(hashtimer_hex: &str, expected_content: &[u8]) -> bool {
    let decoded = decode_hashtimer(hashtimer_hex);
    if let Some((_time, _prefix, hash_suffix)) = decoded {
        // Compute SHA256 of expected content
        let mut hasher = Sha256::new();
        hasher.update(expected_content);
        let computed_hash = hasher.finalize();
        
        // Convert computed hash to hex and compare only the first 50 hex chars
        let computed_hex = format!("{computed_hash:x}");
        let computed_prefix = &computed_hex[..50];
        
        // The hash suffix should match the computed hash prefix
        computed_prefix == hash_suffix
    } else {
        false
    }
}

/// Formats FinDAG Time for human readability
/// 
/// # Arguments
/// * `time_value` - The FinDAG Time value
/// 
/// # Returns
/// * `String` - Human readable time format
pub fn format_findag_time(time_value: u64) -> String {
    // FinDAG Time is in tenths of microseconds
    let microseconds = time_value / 10;
    let seconds = microseconds / 1_000_000;
    let remaining_micros = microseconds % 1_000_000;
    
    format!("{seconds}s {remaining_micros}μs (FinDAG: {time_value})")
}

/// Audit a HashTimer and print detailed information
/// 
/// # Arguments
/// * `hashtimer_hex` - The HashTimer to audit
/// * `content` - Optional content for hash validation
pub fn audit_hashtimer(hashtimer_hex: &str, content: Option<&[u8]>) {
    println!("🔍 HashTimer Audit: {hashtimer_hex}");
    println!("{}", "=".repeat(60));
    
    match decode_hashtimer(hashtimer_hex) {
        Some((time_value, time_prefix, hash_suffix)) => {
            println!("✅ Valid HashTimer structure");
            println!("📅 FinDAG Time: {}", format_findag_time(time_value));
            println!("⏰ Time Prefix: 0x{time_prefix}");
            println!("🔐 Hash Suffix: 0x{hash_suffix}");
            println!("📏 Time bits: {} bits", time_prefix.len() * 4);
            println!("📏 Hash bits: {} bits", hash_suffix.len() * 4);
            
            if let Some(content) = content {
                let is_valid = validate_hashtimer(hashtimer_hex, content);
                println!("🔍 Hash Validation: {}", if is_valid { "✅ Valid" } else { "❌ Invalid" });
            }
        }
        None => {
            println!("❌ Invalid HashTimer format");
            println!("   Expected: 64 hex characters");
            println!("   Got: {} characters", hashtimer_hex.trim_start_matches("0x").len());
        }
    }
    println!("{}", "=".repeat(60));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_decode_hashtimer() {
        // Example HashTimer from your logs
        let hashtimer = "0x8fb592a9c7841fb826796c1726f579feb1b54d8a2edc462824063286010df802";
        
        let result = decode_hashtimer(hashtimer);
        assert!(result.is_some());
        
        let (time_value, time_prefix, hash_suffix) = result.unwrap();
        
        // Should extract 14 hex chars for time prefix
        assert_eq!(time_prefix.len(), 14);
        assert_eq!(hash_suffix.len(), 50);
        
        println!("Decoded HashTimer:");
        println!("Time: {}", format_findag_time(time_value));
        println!("Prefix: 0x{time_prefix}");
        println!("Suffix: 0x{hash_suffix}");
    }
    
    #[test]
    fn test_validate_hashtimer() {
        // Create a test HashTimer
        let content = b"test block content";
        let mut hasher = Sha256::new();
        hasher.update(content);
        let hash = hasher.finalize();
        
        // Create a HashTimer with time prefix + hash
        let time_prefix = "1234567890abcd"; // 14 hex chars
        let hash_suffix = format!("{hash:x}");
        // Truncate hash to fit exactly 64 characters total
        let hash_suffix = &hash_suffix[..50]; // 50 hex chars to make total 64
        let hashtimer = format!("{time_prefix}{hash_suffix}");
        
        // Ensure the hashtimer is exactly 64 hex characters
        assert_eq!(hashtimer.len(), 64);
        
        assert!(validate_hashtimer(&hashtimer, content));
        assert!(!validate_hashtimer(&hashtimer, b"wrong content"));
    }
} 