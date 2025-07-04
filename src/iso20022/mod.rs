pub mod schemas;
pub mod handler;

use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ISO20022Transaction {
    pub message_id: String,
    pub creation_date_time: String,
    pub transaction_type: String,
    pub debtor: String,
    pub creditor: String,
    pub amount: u64,
    pub currency: String,
    pub findag_tx_id: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ISO20022Error {
    #[error("XML parsing error: {0}")]
    XmlParseError(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Invalid amount format: {0}")]
    InvalidAmount(String),
    #[error("Invalid date format: {0}")]
    InvalidDate(String),
}

/// Parse an ISO20022 XML message into a structured transaction
pub fn parse_iso20022(xml_content: &str) -> Result<ISO20022Transaction, ISO20022Error> {
    // Simple XML parsing using string operations for now
    // In a production environment, you'd use a proper XML parser
    
    let extract_field = |content: &str, field_name: &str| -> Result<String, ISO20022Error> {
        let start_tag = format!("<{}>", field_name);
        let end_tag = format!("</{}>", field_name);
        
        if let Some(start) = content.find(&start_tag) {
            if let Some(end) = content.find(&end_tag) {
                let start_pos = start + start_tag.len();
                if start_pos < end {
                    return Ok(content[start_pos..end].to_string());
                }
            }
        }
        Err(ISO20022Error::MissingField(field_name.to_string()))
    };

    let message_id = extract_field(xml_content, "message_id")?;
    let creation_date_time = extract_field(xml_content, "creation_date_time")?;
    let transaction_type = extract_field(xml_content, "transaction_type")?;
    let debtor = extract_field(xml_content, "debtor")?;
    let creditor = extract_field(xml_content, "creditor")?;
    let amount_str = extract_field(xml_content, "amount")?;
    let currency = extract_field(xml_content, "currency")?;
    let findag_tx_id = extract_field(xml_content, "findag_tx_id")?;

    let amount = amount_str.parse::<u64>()
        .map_err(|_| ISO20022Error::InvalidAmount(amount_str))?;

    Ok(ISO20022Transaction {
        message_id,
        creation_date_time,
        transaction_type,
        debtor,
        creditor,
        amount,
        currency,
        findag_tx_id,
    })
}

/// Export an ISO20022 transaction to XML format
pub fn export_iso20022(transaction: &ISO20022Transaction) -> Result<String, ISO20022Error> {
    let xml = format!(
        r#"<ISO20022Transaction>
    <message_id>{}</message_id>
    <creation_date_time>{}</creation_date_time>
    <transaction_type>{}</transaction_type>
    <debtor>{}</debtor>
    <creditor>{}</creditor>
    <amount>{}</amount>
    <currency>{}</currency>
    <findag_tx_id>{}</findag_tx_id>
</ISO20022Transaction>"#,
        transaction.message_id,
        transaction.creation_date_time,
        transaction.transaction_type,
        transaction.debtor,
        transaction.creditor,
        transaction.amount,
        transaction.currency,
        transaction.findag_tx_id
    );
    
    Ok(xml)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iso20022::schemas;

    #[test]
    fn test_parse_valid_settlement() {
        let tx = parse_iso20022(schemas::SETTLEMENT_TRANSACTION)
            .expect("Should parse valid settlement transaction");
        assert_eq!(tx.transaction_type, "Settlement");
        assert_eq!(tx.amount, 50000000);
        assert_eq!(tx.currency, "EUR");
    }

    #[test]
    fn test_parse_valid_credit_transfer() {
        let tx = parse_iso20022(schemas::CREDIT_TRANSFER)
            .expect("Should parse valid credit transfer transaction");
        assert_eq!(tx.transaction_type, "CreditTransfer");
        assert_eq!(tx.amount, 250000);
        assert_eq!(tx.currency, "USD");
    }

    #[test]
    fn test_serialize_roundtrip() {
        let tx = parse_iso20022(schemas::SETTLEMENT_TRANSACTION).unwrap();
        let xml_back = export_iso20022(&tx).unwrap();
        assert!(xml_back.contains("<ISO20022Transaction>"));
        assert!(xml_back.contains("EuroclearBank"));
    }

    #[test]
    fn test_parse_invalid_message() {
        let result = parse_iso20022(schemas::INVALID_MESSAGE);
        assert!(result.is_err(), "Invalid message should fail parsing");
    }

    #[test]
    fn test_parse_missing_fields() {
        let invalid_xml = r#"
<ISO20022Transaction>
    <message_id>msg-123</message_id>
    <!-- Missing required fields -->
</ISO20022Transaction>
"#;
        let result = parse_iso20022(invalid_xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_amount() {
        let invalid_xml = r#"
<ISO20022Transaction>
    <message_id>msg-123</message_id>
    <creation_date_time>2025-07-03T12:30:00Z</creation_date_time>
    <transaction_type>Settlement</transaction_type>
    <debtor>BankA</debtor>
    <creditor>BankB</creditor>
    <amount>invalid</amount>
    <currency>EUR</currency>
    <findag_tx_id>fdg1qhash</findag_tx_id>
</ISO20022Transaction>
"#;
        let result = parse_iso20022(invalid_xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_roundtrip_credit_transfer() {
        let original_tx = parse_iso20022(schemas::CREDIT_TRANSFER).unwrap();
        let xml = export_iso20022(&original_tx).unwrap();
        let parsed_tx = parse_iso20022(&xml).unwrap();
        
        assert_eq!(original_tx, parsed_tx);
    }

    #[test]
    fn test_iso20022_schemas_accessible() {
        // Test that all schema constants are accessible
        assert!(!schemas::SETTLEMENT_TRANSACTION.is_empty());
        assert!(!schemas::CREDIT_TRANSFER.is_empty());
        assert!(!schemas::INVALID_MESSAGE.is_empty());
        
        // Test that they contain expected content
        assert!(schemas::SETTLEMENT_TRANSACTION.contains("Settlement"));
        assert!(schemas::CREDIT_TRANSFER.contains("CreditTransfer"));
        assert!(schemas::INVALID_MESSAGE.contains("msg-bad"));
        
        println!("ISO20022 schemas integration test passed!");
    }
}
