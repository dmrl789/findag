use ed25519_dalek::Keypair;
use rand::SeedableRng;
use rand::rngs::StdRng;
use findag::core::address::Address;

fn main() {
    // Use the same deterministic seed as the bot
    let seed = [42u8; 32];
    let mut rng = StdRng::from_seed(seed);
    let keypair = Keypair::generate(&mut rng);
    
    let address = Address::from_public_key(&keypair.public);
    println!("Bot address: {}", address);
    println!("Public key: {:?}", keypair.public.to_bytes());
} 