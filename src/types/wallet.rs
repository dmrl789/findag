use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EncryptedKey {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 12],
    pub salt: [u8; 16],
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wallet {
    pub address: String,
    pub encrypted_key: EncryptedKey,
    pub portfolio: Portfolio,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Portfolio {
    pub assets: Vec<AssetHolding>,
    pub currencies: Vec<CurrencyHolding>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetHolding {
    pub asset_id: String,
    pub amount: u64,
    pub metadata: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyHolding {
    pub symbol: String,
    pub amount: u64,
}
