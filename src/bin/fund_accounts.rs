use clap::Parser;
use ed25519_dalek::{Keypair, Signer};
use reqwest::Client;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::env;
use rand::SeedableRng;
use hex;

#[derive(Parser)]
#[command(name = "fund_accounts")]
#[command(about = "Fund accounts using real transactions via HTTP API")]
struct Cli {
    #[arg(long, default_value = "http://127.0.0.1:3001")]
    node_url: String,
    
    #[arg(long, default_value = "10000")]
    amount: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let client = Client::new();

    println!("💰 Funding accounts via HTTP API...");
    println!("📡 Target node: {}", cli.node_url);
    println!("💵 Amount per account: {} USD", cli.amount);

    // Faucet keypair (devnet - using a well-known seed) - RFC 8032 compliant
    let faucet_seed = [0u8; 32]; // Faucet seed for devnet
    let faucet_secret_key = ed25519_dalek::SecretKey::from_bytes(&faucet_seed).expect("32-byte seed");
    let faucet_public_key = (&faucet_secret_key).into();
    let faucet_keypair = Keypair { secret: faucet_secret_key, public: faucet_public_key };
    
    let faucet_address = compute_address(faucet_keypair.public.as_bytes());
    println!("🔑 Faucet address: {}", faucet_address);

    // Test accounts to fund
    let test_accounts = vec![
        "fdg1qalice1234567890",
        "fdg1qbob1234567890", 
        "fdg1qcharlie1234567890",
        "fdg1qdiana1234567890",
        "fdg1qedward1234567890",
    ];

    // Bot account (same as transaction_bot uses) - RFC 8032 compliant
    let bot_seed = [42u8; 32]; // Same seed as transaction_bot
    let bot_secret_key = ed25519_dalek::SecretKey::from_bytes(&bot_seed).expect("32-byte seed");
    let bot_public_key = (&bot_secret_key).into();
    let bot_keypair = Keypair { secret: bot_secret_key, public: bot_public_key };
    let bot_address = compute_address(bot_keypair.public.as_bytes());
    
    // Verification output
    println!("VERIFICATION:");
    println!("Bot Seed: {:?}", &bot_seed);
    println!("Bot Public Key: {}", hex::encode(bot_keypair.public.as_bytes()));
    println!("Bot Address: {}", bot_address);

    let mut all_accounts = test_accounts.clone();
    all_accounts.push(&bot_address);

    // Fund each account
    for (i, account) in all_accounts.iter().enumerate() {
        println!("💰 Funding account {}: {}", i + 1, account);
        
        // Create funding transaction
        let payload = format!("Funding {} USD to {}", cli.amount, account).into_bytes();
        
        // Get current FinDAG time
        let findag_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        // Create hashtimer
        let mut hashtimer = [0u8; 32];
        hashtimer[0..8].copy_from_slice(&findag_time.to_le_bytes());
        
        // Create message to sign
        let message = format!("{}{}{}", faucet_address, account, cli.amount);
        let signature = faucet_keypair.sign(message.as_bytes());
        
        // Create transaction request
        let tx_request = json!({
            "from": faucet_address,
            "to": account,
            "amount": cli.amount,
            "signature": signature.to_bytes().to_vec(),
            "payload": payload,
            "findag_time": findag_time,
            "hashtimer": hashtimer.to_vec(),
            "public_key": faucet_keypair.public.to_bytes().to_vec(),
            "shard_id": 0
        });

        // Send funding transaction
        match client.post(&format!("{}/tx", cli.node_url))
            .json(&tx_request)
            .send()
            .await {
            Ok(response) => {
                if response.status().is_success() {
                    println!("  ✅ Funded {} with {} USD", account, cli.amount);
                } else {
                    let status = response.status();
                    let error_text = response.text().await.unwrap_or_default();
                    println!("  ❌ Failed to fund {}: {} - {}", account, status, error_text);
                }
            }
            Err(e) => {
                println!("  ❌ Network error funding {}: {}", account, e);
            }
        }

        // Small delay between transactions
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    println!("🎉 Funding complete!");
    println!("📊 Check balances at: {}/balance/<address>", cli.node_url);
    println!("📦 Check mempool at: {}/mempool-transactions", cli.node_url);

    Ok(())
}

fn compute_address(pubkey: &[u8]) -> String {
    let addr_hex = hex::encode(&pubkey[..8]); // Use first 8 bytes for shorter address (same as Address::from_public_key)
    format!("fdg1q{}", addr_hex)
} 