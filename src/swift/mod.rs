use anyhow::{Result, anyhow};
use crate::core::types::Transaction;
use crate::core::address::Address;
use sha2::{Sha256, Digest};
use ed25519_dalek::{Signature, VerifyingKey};

#[derive(Debug, Clone, PartialEq)]
pub struct MT103Message {
    pub reference: String,
    pub value_date: String,
    pub currency: String,
    pub amount: u64,
    pub debtor: String,
    pub creditor: String,
}

pub fn parse_mt103(raw: &str) -> Result<MT103Message> {
    let mut reference = String::new();
    let mut value_date = String::new();
    let mut amount = 0u64;
    let mut currency = String::new();
    let mut debtor = String::new();
    let mut creditor = String::new();

    for line in raw.lines() {
        if line.starts_with(":20:") {
            reference = line[4..].trim().to_string();
        } else if line.starts_with(":32A:") {
            // Example: :32A:250703USD12345,
            let date_part = &line[5..11];
            let currency_part = &line[11..14];
            let amount_part = line[14..].replace(",", "").trim().to_string();
            value_date = date_part.to_string();
            currency = currency_part.to_string();
            amount = amount_part.parse::<u64>()
                .map_err(|_| anyhow!("Invalid amount format"))?;
        } else if line.starts_with(":50K:") {
            debtor = line[5..].trim().to_string();
        } else if line.starts_with(":59:") {
            creditor = line[4..].trim().to_string();
        }
    }

    if reference.is_empty() || amount == 0 || debtor.is_empty() || creditor.is_empty() {
        return Err(anyhow!("Invalid or incomplete MT103 message"));
    }

    Ok(MT103Message {
        reference,
        value_date,
        currency,
        amount,
        debtor,
        creditor,
    })
}

pub fn mt103_to_findag_tx(mt: &MT103Message) -> Transaction {
    // Create addresses from the debtor and creditor strings
    let from = Address::new(mt.debtor.clone());
    let to = Address::new(mt.creditor.clone());
    
    let amount = mt.amount;
    
    // Create payload with currency information
    let payload = format!("currency:{}", mt.currency).into_bytes();

    let mut hasher = Sha256::new();
    hasher.update(mt.reference.as_bytes());
    let hashtimer = hasher.finalize();
    let mut hashtimer_array = [0u8; 32];
    hashtimer_array.copy_from_slice(&hashtimer);

    // Create a dummy signature and public key for now
    // In a real implementation, these would be properly signed
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
        bridge_protocol: None,
    }
}

pub const EXAMPLE_MT103: &str = r#"
:20:REFERENCE12345
:32A:250703USD123456,
:50K:ALICE_BANK
:59:BOB_BANK
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_mt103() {
        let mt = parse_mt103(EXAMPLE_MT103).expect("Should parse valid MT103");
        assert_eq!(mt.reference, "REFERENCE12345");
        assert_eq!(mt.currency, "USD");
        assert_eq!(mt.amount, 123456);
        assert_eq!(mt.debtor, "ALICE_BANK");
        assert_eq!(mt.creditor, "BOB_BANK");
    }

    #[test]
    fn test_convert_mt103_to_findag_tx() {
        let mt = parse_mt103(EXAMPLE_MT103).unwrap();
        let findag_tx = mt103_to_findag_tx(&mt);

        assert_eq!(findag_tx.amount, 123456);
        assert_eq!(findag_tx.payload, b"currency:USD");
        assert_ne!(findag_tx.hashtimer, [0u8; 32]);
    }

    #[test]
    fn test_parse_invalid_mt103() {
        let bad = ":20:ONLYREFERENCE";
        let result = parse_mt103(bad);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_mt103_with_different_currency() {
        let eur_message = r#"
:20:REFERENCE67890
:32A:250703EUR50000,
:50K:EURO_BANK
:59:AMERICAN_BANK
"#;
        let mt = parse_mt103(eur_message).expect("Should parse valid MT103");
        assert_eq!(mt.reference, "REFERENCE67890");
        assert_eq!(mt.currency, "EUR");
        assert_eq!(mt.amount, 50000);
        assert_eq!(mt.debtor, "EURO_BANK");
        assert_eq!(mt.creditor, "AMERICAN_BANK");
    }

    #[test]
    fn test_hashtimer_generation() {
        let mt = parse_mt103(EXAMPLE_MT103).unwrap();
        let findag_tx = mt103_to_findag_tx(&mt);
        
        // Verify hashtimer is not all zeros
        assert_ne!(findag_tx.hashtimer, [0u8; 32]);
        
        // Verify hashtimer is consistent for same reference
        let findag_tx2 = mt103_to_findag_tx(&mt);
        assert_eq!(findag_tx.hashtimer, findag_tx2.hashtimer);
    }
} 