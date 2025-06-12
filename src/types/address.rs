use base58::{ToBase58};
use blake3::Hasher;

pub enum AddressType {
    Single,
    MultiSig,
}

pub fn generate_address(public_key: &[u8], addr_type: AddressType) -> String {
    let mut hasher = Hasher::new();
    hasher.update(public_key);
    let hash = hasher.finalize();

    let prefix = match addr_type {
        AddressType::Single => "1",
        AddressType::MultiSig => "3",
    };

    let base = hash.as_bytes()[..20].to_base58();
    format!("{}{}", prefix, base)
}

pub fn readable_handle(base: &str) -> String {
    format!("@{}.fd", base)
}
