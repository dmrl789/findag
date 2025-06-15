use crate::crypto::nonce::Nonce;

impl Signature {
    pub fn new(nonce: Nonce) -> Self {
        Signature { nonce }
    }
} 