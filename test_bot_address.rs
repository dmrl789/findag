use ed25519_dalek::SigningKey;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::RngCore;
use findag::core::address::Address;

fn main() {
    // Use the same deterministic seed as the bot
    let seed = [42u8; 32];
    let mut rng = StdRng::from_seed(seed);
    let mut secret_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_bytes);
    let keypair = SigningKey::from_bytes(&secret_bytes);
    
    let address = Address::from_verifying_key(&keypair.verifying_key());
    println!("Bot address: {address}");
    println!("Public key: {:?}", keypair.verifying_key().to_bytes());
} 