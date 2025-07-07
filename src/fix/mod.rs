// src/fix/mod.rs

pub mod schemas;

use anyhow::{Result, anyhow};
use crate::core::types::Transaction;
use crate::core::address::Address;
use sha2::{Sha256, Digest};
use ed25519_dalek::{Signature, VerifyingKey};

/// Minimal representation of a FIX Order Single (MsgType = D)
#[derive(Debug, Clone)]
pub struct FixOrderSingle {
    pub cl_ord_id: String,   // Client order ID (11)
    pub symbol: String,      // Symbol (55) e.g., EUR/USD
    pub side: String,        // Side (54) 1=Buy, 2=Sell
    pub order_qty: u64,      // OrderQty (38)
    pub price: Option<f64>,  // Price (44) for limit orders
    pub currency: String,    // Currency (15)
    pub account: String,     // Account (1)
    pub msg_type: String,    // MsgType (35) - should be "D" for Order Single
}

/// Parse a simple raw FIX string to a FixOrderSingle.
pub fn parse_order_single(raw: &str) -> Result<FixOrderSingle> {
    let mut cl_ord_id = String::new();
    let mut symbol = String::new();
    let mut side = String::new();
    let mut order_qty = 0u64;
    let mut price = None;
    let mut currency = String::new();
    let mut account = String::new();
    let mut msg_type = String::new();

    for pair in raw.split('\x01') {
        let parts: Vec<&str> = pair.split('=').collect();
        if parts.len() != 2 { continue; }

        match parts[0] {
            "35" => msg_type = parts[1].to_string(),
            "11" => cl_ord_id = parts[1].to_string(),
            "55" => symbol = parts[1].to_string(),
            "54" => side = parts[1].to_string(),
            "38" => order_qty = parts[1].parse().unwrap_or(0),
            "44" => price = Some(parts[1].parse().unwrap_or(0.0)),
            "15" => currency = parts[1].to_string(),
            "1"  => account = parts[1].to_string(),
            _ => {}
        }
    }

    // Validate this is an Order Single message
    if msg_type != "D" {
        return Err(anyhow!("Invalid MsgType: expected 'D' (Order Single), got '{}'", msg_type));
    }

    if cl_ord_id.is_empty() || symbol.is_empty() || side.is_empty() || order_qty == 0 {
        return Err(anyhow!("Missing required FIX fields: cl_ord_id, symbol, side, or order_qty"));
    }

    Ok(FixOrderSingle {
        cl_ord_id,
        symbol,
        side,
        order_qty,
        price,
        currency,
        account,
        msg_type,
    })
}

/// Convert FIX Order Single → FinDAG Transaction
pub fn fix_order_to_findag_tx(fix: &FixOrderSingle) -> Transaction {
    let from = Address::new(fix.account.clone());
    let to = Address::new(format!("FX::{}", fix.symbol)); // symbolic — adjust for your settlement logic
    let amount = fix.order_qty;
    
    // Create payload with FIX order details
    let payload = format!(
        "fix_order:{}:{}:{}:{}:{}",
        fix.cl_ord_id,
        fix.symbol,
        fix.side,
        fix.order_qty,
        fix.price.map(|p| p.to_string()).unwrap_or_else(|| "MARKET".to_string())
    ).into_bytes();

    // Compute HashTimer as SHA-256 of client order ID
    let mut hasher = Sha256::new();
    hasher.update(fix.cl_ord_id.as_bytes());
    let hashtimer = hasher.finalize();
    let mut hashtimer_array = [0u8; 32];
    hashtimer_array.copy_from_slice(&hashtimer);

    // Create a dummy signature and public key for now
    // In a real implementation, these would be properly signed by the FIX client
    let dummy_signature = Signature::from_bytes(&[0u8; 64]).unwrap();
    let dummy_public_key = VerifyingKey::from_bytes(&[0u8; 32]).unwrap();

    Transaction {
        from,
        to,
        amount,
        payload,
        findag_time: 0, // Will be set by the system
        hashtimer: hashtimer_array,
        signature: dummy_signature,
        public_key: dummy_public_key,
        shard_id: crate::core::types::ShardId(0),
        source_shard: None,
        dest_shard: None,
        target_chain: None,
        bridge_protocol: Some("FIX".to_string()),
    }
}

/// Validate FIX message checksum (simple implementation)
pub fn validate_fix_checksum(raw: &str) -> Result<bool> {
    let parts: Vec<&str> = raw.split('\x01').collect();
    if parts.is_empty() {
        return Err(anyhow!("Empty FIX message"));
    }

    // Find the checksum field (10=xxx)
    let mut calculated_checksum = 0u8;
    let mut found_checksum = false;
    let mut checksum_value = 0u8;

    for part in &parts {
        if part.starts_with("10=") {
            found_checksum = true;
            let checksum_str = &part[3..];
            checksum_value = checksum_str.parse().unwrap_or(0);
            break;
        }
    }

    // Calculate checksum (sum of all ASCII values mod 256)
    for part in &parts {
        if !part.starts_with("10=") { // Exclude the checksum field itself
            for byte in part.as_bytes() {
                calculated_checksum = calculated_checksum.wrapping_add(*byte);
            }
        }
    }

    Ok(found_checksum && calculated_checksum == checksum_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fix::schemas::EXAMPLE_FIX_ORDER;

    #[test]
    fn test_parse_fix_order() {
        let order = parse_order_single(EXAMPLE_FIX_ORDER).expect("Should parse");
        assert_eq!(order.cl_ord_id, "ORD12345");
        assert_eq!(order.symbol, "EUR/USD");
        assert_eq!(order.side, "1");
        assert_eq!(order.order_qty, 1_000_000);
        assert_eq!(order.currency, "EUR");
        assert_eq!(order.account, "ALICE_ACCOUNT");
        assert_eq!(order.msg_type, "D");
    }

    #[test]
    fn test_fix_to_findag_tx() {
        let order = parse_order_single(EXAMPLE_FIX_ORDER).unwrap();
        let tx = fix_order_to_findag_tx(&order);

        assert_eq!(tx.from.as_str(), "ALICE_ACCOUNT");
        assert_eq!(tx.to.as_str(), "FX::EUR/USD");
        assert_eq!(tx.amount, 1_000_000);
        assert_eq!(tx.bridge_protocol, Some("FIX".to_string()));
        
        // Check payload contains order details
        let payload_str = String::from_utf8_lossy(&tx.payload);
        assert!(payload_str.contains("ORD12345"));
        assert!(payload_str.contains("EUR/USD"));
        assert!(payload_str.contains("1"));
    }

    #[test]
    fn test_invalid_msg_type() {
        let invalid_fix = "8=FIX.4.2\x0135=8\x0111=ORD12345\x0155=EUR/USD\x0154=1\x0138=1000000\x0110=123\x01";
        let result = parse_order_single(invalid_fix);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid MsgType"));
    }

    #[test]
    fn test_missing_required_fields() {
        let incomplete_fix = "8=FIX.4.2\x0135=D\x0111=ORD12345\x0155=EUR/USD\x0110=123\x01";
        let result = parse_order_single(incomplete_fix);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Missing required FIX fields"));
    }
} 