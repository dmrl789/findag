use std::collections::HashMap;
use serde_json::Value;
use findag::core::types::{Transaction, ShardId};
use findag::core::address::Address;
use ed25519_dalek::{PublicKey, Signature, Verifier};

#[derive(Debug)]
struct TransactionAnalysis {
    raw_json: String,
    parsed_fields: HashMap<String, String>,
    validation_errors: Vec<String>,
    signature_analysis: SignatureAnalysis,
}

#[derive(Debug)]
struct SignatureAnalysis {
    signature_length: usize,
    public_key_length: usize,
    signature_valid: bool,
    signature_bytes: Vec<u8>,
    public_key_bytes: Vec<u8>,
}

fn main() {
    println!("🔍 FinDAG Transaction Analyzer");
    println!("================================");
    
    // Test with a sample transaction that matches the bot's format
    let sample_tx = r#"{
        "from": "fdg1qbotd60e1cdd",
        "to": "fdg1qalice1234567890", 
        "amount": 100,
        "signature": [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        "payload": [],
        "findag_time": 0,
        "hashtimer": [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        "public_key": [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        "shard_id": 0
    }"#;
    
    let analysis = analyze_transaction(sample_tx);
    print_analysis(&analysis);
}

fn analyze_transaction(raw_json: &str) -> TransactionAnalysis {
    let mut analysis = TransactionAnalysis {
        raw_json: raw_json.to_string(),
        parsed_fields: HashMap::new(),
        validation_errors: Vec::new(),
        signature_analysis: SignatureAnalysis {
            signature_length: 0,
            public_key_length: 0,
            signature_valid: false,
            signature_bytes: Vec::new(),
            public_key_bytes: Vec::new(),
        },
    };
    
    // Parse JSON
    let json_value: Value = match serde_json::from_str(raw_json) {
        Ok(v) => v,
        Err(e) => {
            analysis.validation_errors.push(format!("JSON parse error: {}", e));
            return analysis;
        }
    };
    
    // Extract fields
    if let Some(from) = json_value["from"].as_str() {
        analysis.parsed_fields.insert("from".to_string(), from.to_string());
    }
    
    if let Some(to) = json_value["to"].as_str() {
        analysis.parsed_fields.insert("to".to_string(), to.to_string());
    }
    
    if let Some(amount) = json_value["amount"].as_u64() {
        analysis.parsed_fields.insert("amount".to_string(), amount.to_string());
    }
    
    if let Some(shard_id) = json_value["shard_id"].as_u64() {
        analysis.parsed_fields.insert("shard_id".to_string(), shard_id.to_string());
    }
    
    // Analyze signature
    if let Some(signature_array) = json_value["signature"].as_array() {
        analysis.signature_analysis.signature_length = signature_array.len();
        analysis.signature_analysis.signature_bytes = signature_array
            .iter()
            .filter_map(|v| v.as_u64().map(|n| n as u8))
            .collect();
            
        if analysis.signature_analysis.signature_length != 64 {
            analysis.validation_errors.push(format!(
                "Invalid signature length: {} (expected 64)", 
                analysis.signature_analysis.signature_length
            ));
        }
    }
    
    // Analyze public key
    if let Some(pubkey_array) = json_value["public_key"].as_array() {
        analysis.signature_analysis.public_key_length = pubkey_array.len();
        analysis.signature_analysis.public_key_bytes = pubkey_array
            .iter()
            .filter_map(|v| v.as_u64().map(|n| n as u8))
            .collect();
            
        if analysis.signature_analysis.public_key_length != 32 {
            analysis.validation_errors.push(format!(
                "Invalid public key length: {} (expected 32)", 
                analysis.signature_analysis.public_key_length
            ));
        }
    }
    
    // Validate addresses
    if let Some(from) = analysis.parsed_fields.get("from") {
        if !from.starts_with("fdg1q") {
            analysis.validation_errors.push(format!("Invalid from address format: {}", from));
        }
    }
    
    if let Some(to) = analysis.parsed_fields.get("to") {
        if !to.starts_with("fdg1q") {
            analysis.validation_errors.push(format!("Invalid to address format: {}", to));
        }
    }
    
    analysis
}

fn print_analysis(analysis: &TransactionAnalysis) {
    println!("📋 Transaction Analysis Results");
    println!("================================");
    
    println!("📄 Parsed Fields:");
    for (key, value) in &analysis.parsed_fields {
        println!("  {}: {}", key, value);
    }
    
    println!("\n🔐 Signature Analysis:");
    println!("  Signature length: {}", analysis.signature_analysis.signature_length);
    println!("  Public key length: {}", analysis.signature_analysis.public_key_length);
    println!("  Signature bytes: {:02x?}", &analysis.signature_analysis.signature_bytes);
    println!("  Public key bytes: {:02x?}", &analysis.signature_analysis.public_key_bytes);
    
    if !analysis.validation_errors.is_empty() {
        println!("\n❌ Validation Errors:");
        for error in &analysis.validation_errors {
            println!("  - {}", error);
        }
    } else {
        println!("\n✅ No validation errors found");
    }
    
    println!("\n📊 Expected vs Actual:");
    println!("  Expected signature length: 64");
    println!("  Actual signature length: {}", analysis.signature_analysis.signature_length);
    println!("  Expected public key length: 32");
    println!("  Actual public key length: {}", analysis.signature_analysis.public_key_length);
} 