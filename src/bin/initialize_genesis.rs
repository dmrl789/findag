use clap::{Command, Arg};
use ed25519_dalek::Signer;
use findag::core::address::{Address, generate_deterministic_signing_key};
use serde_json::json;
use base64::Engine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Initialize Genesis")
        .version("1.0")
        .about("Initialize genesis state with faucet account")
        .arg(
            Arg::new("node-url")
                .long("node-url")
                .value_name("URL")
                .help("Node HTTP API URL")
                .default_value("http://127.0.0.1:3000")
                .num_args(1),
        )
        .arg(
            Arg::new("faucet-amount")
                .long("faucet-amount")
                .value_name("AMOUNT")
                .help("Initial faucet balance in USD")
                .default_value("1000000")
                .num_args(1),
        )
        .get_matches();

    let node_url = matches.get_one::<String>("node-url").unwrap();
    let faucet_amount: u64 = matches.get_one::<String>("faucet-amount").unwrap().parse()?;

    println!("🔧 Initializing genesis state...");
    println!("📡 Target node: {node_url}");
    println!("💰 Faucet amount: {faucet_amount} USD");

    // Generate deterministic faucet keypair
    let faucet_seed = [1u8; 32]; // Fixed seed for faucet
    let faucet_signing_key = generate_deterministic_signing_key(&faucet_seed);
    let faucet_address = Address::from_signing_key(&faucet_signing_key);
    
    println!("🔑 Faucet address: {}", faucet_address.as_str());
    println!("🔑 Faucet public key: {}", hex::encode(faucet_signing_key.verifying_key().to_bytes()));

    // Create a dummy transaction to fund the faucet
    // This is a special case where we create a transaction from a "genesis" address
    let genesis_address = "fdg1qgenesis000000000000000000000000000000000000000000000000000000";
    let message = format!("{}{}{}", genesis_address, faucet_address.as_str(), faucet_amount);
    
    // Create a dummy signature (in a real system, this would be properly signed)
    let dummy_signature = faucet_signing_key.sign(message.as_bytes());
    
    let transaction_data = json!({
        "from": genesis_address,
        "to": faucet_address.as_str(),
        "amount": faucet_amount,
        "signature": base64::engine::general_purpose::STANDARD.encode(dummy_signature.to_bytes()),
        "payload": base64::engine::general_purpose::STANDARD.encode(Vec::<u8>::new()),
        "findag_time": 0,
        "hashtimer": base64::engine::general_purpose::STANDARD.encode(vec![0u8; 32]),
        "public_key": base64::engine::general_purpose::STANDARD.encode(faucet_signing_key.verifying_key().to_bytes()),
        "shard_id": 0
    });

    // Send the transaction to the node
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{node_url}/tx"))
        .json(&transaction_data)
        .send()
        .await?;

    if response.status().is_success() {
        println!("✅ Genesis initialization successful!");
        println!("💰 Faucet account funded with {faucet_amount} USD");
    } else {
        let error_text = response.text().await?;
        println!("❌ Failed to initialize genesis: {error_text}");
    }

    Ok(())
} 