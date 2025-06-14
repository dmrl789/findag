use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, SECRET_KEY_LENGTH};
use rand::rngs::OsRng;
use ring::aead::{Aad, LessSafeKey, UnboundKey, Nonce, AES_256_GCM, BoundKey, NonceSequence, SealingKey, OpeningKey};
use std::fs;
use std::path::Path;
use std::io::{Write, Read};

pub struct Wallet {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl Wallet {
    pub fn new() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        Wallet { signing_key, verifying_key }
    }

    // Encrypts the signing key with a user-supplied key (you need to implement key derivation)
    pub fn save_encrypted<P: AsRef<Path>>(&self, path: P, encryption_key: &[u8; 32]) -> Result<(), String> {
        let mut key_bytes = self.signing_key.to_bytes().to_vec();

        // Encrypt the key bytes with AES-256-GCM (ring)
        let unbound_key = UnboundKey::new(&AES_256_GCM, encryption_key).map_err(|e| format!("{:?}", e))?;
        let mut sealing_key = LessSafeKey::new(unbound_key);
        let nonce = Nonce::assume_unique_for_key([0u8; 12]); // Replace with a random nonce in production!
        sealing_key.seal_in_place_append_tag(nonce, Aad::empty(), &mut key_bytes).map_err(|e| format!("{:?}", e))?;

        fs::write(path, key_bytes).map_err(|e| e.to_string())
    }

    // Decrypt and load a signing key
    pub fn load_encrypted<P: AsRef<Path>>(path: P, encryption_key: &[u8; 32]) -> Result<Self, String> {
        let mut key_bytes = fs::read(path).map_err(|e| e.to_string())?;

        let unbound_key = UnboundKey::new(&AES_256_GCM, encryption_key).map_err(|e| format!("{:?}", e))?;
        let mut opening_key = LessSafeKey::new(unbound_key);
        let nonce = Nonce::assume_unique_for_key([0u8; 12]);
        let decrypted = opening_key.open_in_place(nonce, Aad::empty(), &mut key_bytes).map_err(|e| format!("{:?}", e))?;
        let signing_key = SigningKey::from_bytes(decrypted.try_into().map_err(|_| "Failed to decode key")?);
        let verifying_key = signing_key.verifying_key();
        Ok(Wallet { signing_key, verifying_key })
    }
}
