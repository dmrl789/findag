use clap::{Parser, Subcommand};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signer, SECRET_KEY_LENGTH, PUBLIC_KEY_LENGTH};
use rand::rngs::OsRng;
use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};
use hex;
use core::address::{Address, generate_address};

#[derive(Parser)]
#[command(name = "FinDAG CLI Wallet")]
#[command(about = "A simple CLI wallet for FinDAG", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new keypair and address
    Generate {},
    /// Import a keypair from a hex-encoded secret key
    Import {
        #[arg(long)]
        secret: String,
    },
    /// Export the secret key to hex
    Export {
        #[arg(long)]
        file: PathBuf,
    },
    /// Show the wallet address
    Address {
        #[arg(long)]
        file: PathBuf,
    },
    /// Show the (mocked) balance
    Balance {
        #[arg(long)]
        file: PathBuf,
    },
    /// Send a payment (sign and print tx)
    Send {
        #[arg(long)]
        file: PathBuf,
        #[arg(long)]
        to: String,
        #[arg(long)]
        amount: u64,
        #[arg(long, default_value = "USD")]
        currency: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Generate {} => {
            let (keypair, address) = generate_address();
            println!("Address: {}", address.as_str());
            println!("Public Key: {}", hex::encode(keypair.public.as_bytes()));
            println!("Secret Key: {}", hex::encode(keypair.secret.as_bytes()));
        }
        Commands::Import { secret } => {
            let secret_bytes = hex::decode(secret).expect("Invalid hex");
            assert_eq!(secret_bytes.len(), SECRET_KEY_LENGTH);
            let secret = SecretKey::from_bytes(&secret_bytes).unwrap();
            let public = PublicKey::from(&secret);
            let keypair = Keypair { secret, public };
            let address = Address::from_public_key(&keypair.public);
            println!("Imported Address: {}", address.as_str());
        }
        Commands::Export { file } => {
            let keypair = load_keypair(file);
            println!("Secret Key: {}", hex::encode(keypair.secret.as_bytes()));
        }
        Commands::Address { file } => {
            let keypair = load_keypair(file);
            let address = Address::from_public_key(&keypair.public);
            println!("Address: {}", address.as_str());
        }
        Commands::Balance { file } => {
            let keypair = load_keypair(file);
            let address = Address::from_public_key(&keypair.public);
            // Mocked balance (replace with real API call)
            println!("Balance for {}: 1000 USD (mocked)", address.as_str());
        }
        Commands::Send { file, to, amount, currency } => {
            let keypair = load_keypair(file);
            let from_address = Address::from_public_key(&keypair.public);
            let to_address = Address(to.clone());
            // Mocked transaction (replace with real tx creation and network send)
            println!("Sending {} {} from {} to {}", amount, currency, from_address.as_str(), to_address.as_str());
            // Sign and print tx
            let signature = keypair.sign(b"mock-tx");
            println!("Signed tx with signature: {}", hex::encode(signature.to_bytes()));
        }
    }
}

fn load_keypair(file: &PathBuf) -> Keypair {
    let sk_hex = fs::read_to_string(file).expect("Failed to read key file");
    let sk_bytes = hex::decode(sk_hex.trim()).expect("Invalid hex in key file");
    assert_eq!(sk_bytes.len(), SECRET_KEY_LENGTH);
    let secret = SecretKey::from_bytes(&sk_bytes).unwrap();
    let public = PublicKey::from(&secret);
    Keypair { secret, public }
}

// Usage examples:
// Generate: cargo run --bin cli_wallet -- generate
// Import: cargo run --bin cli_wallet -- import --secret <hex>
// Export: cargo run --bin cli_wallet -- export --file mykey.txt
// Address: cargo run --bin cli_wallet -- address --file mykey.txt
// Balance: cargo run --bin cli_wallet -- balance --file mykey.txt
// Send: cargo run --bin cli_wallet -- send --file mykey.txt --to <address> --amount 1000 --currency USD 