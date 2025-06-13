use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signer};
use rand::rngs::OsRng;

pub struct FinDagKeypair {
    pub keypair: Keypair,
}

impl FinDagKeypair {
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let keypair = Keypair::generate(&mut csprng);
        FinDagKeypair { keypair }
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.keypair.public
    }
}
