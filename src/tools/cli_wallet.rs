use clap::{Parser, Subcommand};
use ed25519_dalek::{SigningKey, Signer};
use std::fs;
use std::path::PathBuf;
use std::io::{self, Write};
use hex;
use crate::core::address::{Address, generate_address};
use serde::{Serialize, Deserialize};
use reqwest;
use chrono;

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

#[allow(dead_code)]
async fn fetch_asset_whitelist(node_url: &str) -> Vec<String> {
    let url = format!("{node_url}/assets");
    match reqwest::get(&url).await {
        Ok(resp) => match resp.json::<serde_json::Value>().await {
            Ok(json) => json["assets"].as_array().map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()).unwrap_or_default(),
            Err(_) => vec![],
        },
        Err(_) => vec![],
    }
}

#[tokio::main]
#[allow(dead_code)]
async fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Generate {} => {
            let (signing_key, address) = generate_address();
            println!("Address: {}", address.as_str());
            println!("Public Key: {}", hex::encode(signing_key.verifying_key().to_bytes()));
            println!("Secret Key: {}", hex::encode(signing_key.to_bytes()));
        }
        Commands::Import { secret } => {
            let secret_bytes = hex::decode(secret).expect("Invalid hex");
            assert_eq!(secret_bytes.len(), 32);
            let signing_key = SigningKey::from_bytes(&secret_bytes.try_into().unwrap());
            let address = Address::from_signing_key(&signing_key);
            println!("Imported Address: {}", address.as_str());
        }
        Commands::Export { file } => {
            let signing_key = load_signing_key(file);
            println!("Secret Key: {}", hex::encode(signing_key.to_bytes()));
        }
        Commands::Address { file } => {
            let signing_key = load_signing_key(file);
            let address = Address::from_signing_key(&signing_key);
            println!("Address: {}", address.as_str());
        }
        Commands::Balance { file } => {
            let signing_key = load_signing_key(file);
            let address = Address::from_signing_key(&signing_key);
            // Prompt for asset code (default to USD)
            print!("Enter asset code (default: USD): ");
            io::stdout().flush().unwrap();
            let mut asset = String::new();
            io::stdin().read_line(&mut asset).unwrap();
            let asset = asset.trim();
            let asset = if asset.is_empty() { "USD" } else { asset };
            let whitelist = fetch_asset_whitelist(&cli.node_url).await;
            if !whitelist.contains(&asset.to_string()) {
                println!("Error: '{asset}' is not a supported asset.");
                return;
            }
            let url = format!("{}/balance/{}/{}", cli.node_url, address.as_str(), asset);
            let resp = reqwest::get(&url).await.expect("Failed to query node");
            let json: serde_json::Value = resp.json().await.expect("Invalid response");
            println!("Balance for {}: {} {}", address.as_str(), json["balance"], asset);
        }
        Commands::Send { file, to, amount, currency } => {
            let whitelist = fetch_asset_whitelist(&cli.node_url).await;
            if !whitelist.contains(currency) {
                println!("Error: '{currency}' is not a supported asset.");
                return;
            }
            let signing_key = load_signing_key(file);
            let from_address = Address::from_signing_key(&signing_key);
            let to_address = Address(to.clone());
            let signature = signing_key.sign(b"mock-tx");
            let tx = ApiTransaction {
                from: from_address.as_str().to_string(),
                to: to_address.as_str().to_string(),
                amount: *amount,
                currency: currency.clone(),
                signature: hex::encode(signature.to_bytes()),
                public_key: hex::encode(signing_key.verifying_key().to_bytes()),
            };
            let url = format!("{}/tx", cli.node_url);
            let client = reqwest::Client::new();
            let resp = client.post(&url).json(&tx).send().await.expect("Failed to send tx");
            let json: serde_json::Value = resp.json().await.expect("Invalid response");
            println!("Node response: {json}");
        }
    }
}

#[allow(dead_code)]
fn load_signing_key(file: &PathBuf) -> SigningKey {
    let sk_hex = fs::read_to_string(file).expect("Failed to read key file");
    let sk_bytes = hex::decode(sk_hex.trim()).expect("Invalid hex in key file");
    assert_eq!(sk_bytes.len(), 32);
    SigningKey::from_bytes(&sk_bytes.try_into().unwrap())
}

fn load_signing_key_from_file(path: &str) -> Result<SigningKey, Box<dyn std::error::Error>> {
    let keypair_data = fs::read_to_string(path)?;
    let keypair_json: serde_json::Value = serde_json::from_str(&keypair_data)?;
    
    if let Some(secret_key_hex) = keypair_json["secret_key"].as_str() {
        let secret_bytes = hex::decode(secret_key_hex)?;
        let signing_key = SigningKey::from_bytes(&secret_bytes.try_into().unwrap());
        Ok(signing_key)
    } else {
        Err("Invalid keypair format".into())
    }
}

fn save_signing_key_to_file(signing_key: &SigningKey, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let keypair_data = serde_json::json!({
        "secret_key": hex::encode(signing_key.to_bytes()),
        "public_key": hex::encode(signing_key.verifying_key().to_bytes()),
        "created_at": chrono::Utc::now().to_rfc3339()
    });
    
    fs::write(path, serde_json::to_string_pretty(&keypair_data)?)?;
    Ok(())
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