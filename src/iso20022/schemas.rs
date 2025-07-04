/// Example ISO 20022 Settlement Transaction (semt.017)
pub const SETTLEMENT_TRANSACTION: &str = r#"
<ISO20022Transaction>
    <message_id>msg-456</message_id>
    <creation_date_time>2025-07-03T12:30:00Z</creation_date_time>
    <transaction_type>Settlement</transaction_type>
    <debtor>EuroclearBank</debtor>
    <creditor>CounterpartyCustodian</creditor>
    <amount>50000000</amount>
    <currency>EUR</currency>
    <findag_tx_id>fdg1qsettlementhash</findag_tx_id>
</ISO20022Transaction>
"#;

/// Example ISO 20022 Funds Transfer (pacs.008)
pub const CREDIT_TRANSFER: &str = r#"
<ISO20022Transaction>
    <message_id>msg-789</message_id>
    <creation_date_time>2025-07-03T14:00:00Z</creation_date_time>
    <transaction_type>CreditTransfer</transaction_type>
    <debtor>ClientBank</debtor>
    <creditor>BeneficiaryBank</creditor>
    <amount>250000</amount>
    <currency>USD</currency>
    <findag_tx_id>fdg1qcredittransferhash</findag_tx_id>
</ISO20022Transaction>
"#;

/// Example invalid message (for negative test)
pub const INVALID_MESSAGE: &str = r#"
<ISO20022Transaction>
    <message_id>msg-bad</message_id>
    <!-- Missing required fields -->
</ISO20022Transaction>
"#; 