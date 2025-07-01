use clap::{App, Arg};
use ed25519_dalek::{Keypair, Signer};
use findag::core::address::generate_deterministic_keypair;
use findag::storage::state::StateDB;
use reqwest;
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Initialize Genesis")
        .version("1.0")
        .about("Initialize genesis state with faucet account")
        .arg(
            Arg::with_name("node-url")
                .long("node-url")
                .value_name("URL")
                .help("Node HTTP API URL")
                .default_value("http://127.0.0.1:3000")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("faucet-amount")
                .long("faucet-amount")
                .value_name("AMOUNT")
                .help("Initial faucet balance in USD")
                .default_value("1000000")
                .takes_value(true),
        )
        .get_matches();

    let node_url = matches.value_of("node-url").unwrap();
    let faucet_amount: u64 = matches.value_of("faucet-amount").unwrap().parse()?;

    println!("🔧 Initializing genesis state...");
    println!("📡 Target node: {}", node_url);
    println!("💰 Faucet amount: {} USD", faucet_amount);

    // Generate deterministic faucet keypair
    let faucet_seed = [1u8; 32]; // Fixed seed for faucet
    let faucet_keypair = generate_deterministic_keypair(&faucet_seed);
    let faucet_address = findag::core::address::generate_address_from_keypair(&faucet_keypair);
    
    println!("🔑 Faucet address: {}", faucet_address.0);
    println!("🔑 Faucet public key: {}", hex::encode(faucet_keypair.public.to_bytes()));

    // Create a dummy transaction to fund the faucet
    // This is a special case where we create a transaction from a "genesis" address
    let genesis_address = "fdg1qgenesis000000000000000000000000000000000000000000000000000000";
    let message = format!("{}{}{}", genesis_address, faucet_address.0, faucet_amount);
    
    // Create a dummy signature (in a real system, this would be properly signed)
    let dummy_signature = faucet_keypair.sign(message.as_bytes());
    
    let transaction_data = json!({
        "from": genesis_address,
        "to": faucet_address.0,
        "amount": faucet_amount,
        "signature": dummy_signature.to_bytes().to_vec(),
        "payload": vec![],
        "findag_time": 0,
        "hashtimer": vec![0u8; 32],
        "public_key": faucet_keypair.public.to_bytes().to_vec(),
        "shard_id": 0
    });

    // Send the transaction to the node
    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/tx", node_url))
        .json(&transaction_data)
        .send()
        .await?;

    if response.status().is_success() {
        println!("✅ Genesis initialization successful!");
        println!("💰 Faucet account funded with {} USD", faucet_amount);
    } else {
        let error_text = response.text().await?;
        println!("❌ Failed to initialize genesis: {}", error_text);
    }

    Ok(())
} 