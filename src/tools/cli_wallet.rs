use clap::{Parser, Subcommand};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signer, SECRET_KEY_LENGTH, PUBLIC_KEY_LENGTH};
use rand::rngs::OsRng;
use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};
use hex;
use crate::core::address::{Address, generate_address};
use serde::{Serialize, Deserialize};
use reqwest;

#[derive(Parser)]
#[command(name = "FinDAG CLI Wallet")]
#[command(about = "A simple CLI wallet for FinDAG. Only whitelisted assets are supported for sending and balance queries. See below for supported assets.", long_about = None)]
struct Cli {
    #[arg(long, default_value = "http://127.0.0.1:8080")]
    node_url: String,
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
    /// Show the balance (via node API)
    Balance {
        #[arg(long)]
        file: PathBuf,
    },
    /// Send a payment (sign and submit tx)
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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ApiTransaction {
    from: String,
    to: String,
    amount: u64,
    currency: String,
    signature: String,
    public_key: String,
}

async fn fetch_asset_whitelist(node_url: &str) -> Vec<String> {
    let url = format!("{}/assets", node_url);
    match reqwest::get(&url).await {
        Ok(resp) => match resp.json::<serde_json::Value>().await {
            Ok(json) => json["assets"].as_array().map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()).unwrap_or_default(),
            Err(_) => vec![],
        },
        Err(_) => vec![],
    }
}

#[tokio::main]
async fn main() {
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
            // Prompt for asset code (default to USD)
            print!("Enter asset code (default: USD): ");
            io::stdout().flush().unwrap();
            let mut asset = String::new();
            io::stdin().read_line(&mut asset).unwrap();
            let asset = asset.trim();
            let asset = if asset.is_empty() { "USD" } else { asset };
            let whitelist = fetch_asset_whitelist(&cli.node_url).await;
            if !whitelist.contains(&asset.to_string()) {
                println!("Error: '{}' is not a supported asset.", asset);
                return;
            }
            let url = format!("{}/balance/{}/{}", cli.node_url, address.as_str(), asset);
            let resp = reqwest::get(&url).await.expect("Failed to query node");
            let json: serde_json::Value = resp.json().await.expect("Invalid response");
            println!("Balance for {}: {} {}", address.as_str(), json["balance"], asset);
        }
        Commands::Send { file, to, amount, currency } => {
            let whitelist = fetch_asset_whitelist(&cli.node_url).await;
            if !whitelist.contains(&currency) {
                println!("Error: '{}' is not a supported asset.", currency);
                return;
            }
            let keypair = load_keypair(file);
            let from_address = Address::from_public_key(&keypair.public);
            let to_address = Address(to.clone());
            let signature = keypair.sign(b"mock-tx");
            let tx = ApiTransaction {
                from: from_address.as_str().to_string(),
                to: to_address.as_str().to_string(),
                amount: *amount,
                currency: currency.clone(),
                signature: hex::encode(signature.to_bytes()),
                public_key: hex::encode(keypair.public.as_bytes()),
            };
            let url = format!("{}/tx", cli.node_url);
            let client = reqwest::Client::new();
            let resp = client.post(&url).json(&tx).send().await.expect("Failed to send tx");
            let json: serde_json::Value = resp.json().await.expect("Invalid response");
            println!("Node response: {}", json);
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
// Balance: cargo run --bin cli_wallet -- balance --file mykey.txt --node-url http://127.0.0.1:8080
// Send: cargo run --bin cli_wallet -- send --file mykey.txt --to <address> --amount 1000 --currency USD --node-url http://127.0.0.1:8080
//
// Supported assets:
// EUR, USD, GBP, JPY, CHF, SGD, AED, CNY, BUND, OAT, BTP, GILT, UST, JGB, T-BILL, CP, CD, XAU, XAG, XPT, XPD, XS1234567890, FR0000120271, BE0003796134, DE0001135275, ETF1, UCITS1, BTC, ETH, USDT, USDC
// Attempting to send or query unsupported assets will result in an error. 