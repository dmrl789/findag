use clap::Parser;
use libp2p_identity::Keypair;
use reqwest::Client;
use serde_json::json;

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

    // Faucet keypair (devnet - using a well-known seed)
    let mut faucet_seed = [0u8; 32]; // Faucet seed for devnet
    let faucet_keypair = Keypair::ed25519_from_bytes(&mut faucet_seed).expect("32-byte seed");
    
    let faucet_address = compute_address(&faucet_keypair.public().encode_protobuf());
    println!("🔑 Faucet address: {faucet_address}");

    // Test accounts to fund
    let test_accounts = vec![
        "fdg1qalice1234567890",
        "fdg1qbob1234567890", 
        "fdg1qcharlie1234567890",
        "fdg1qdiana1234567890",
        "fdg1qedward1234567890",
    ];

    // Bot account (same as transaction_bot uses)
    let mut bot_seed = [42u8; 32]; // Same seed as transaction_bot
    let bot_keypair = Keypair::ed25519_from_bytes(&mut bot_seed).expect("32-byte seed");
    let bot_address = compute_address(&bot_keypair.public().encode_protobuf());
    
    // Verification output
    println!("VERIFICATION:");
    println!("Bot Seed: {:?}", &bot_seed);
    println!("Bot Public Key: {}", hex::encode(bot_keypair.public().encode_protobuf()));
    println!("Bot Address: {bot_address}");

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
            "signature": signature.unwrap(),
            "payload": payload,
            "findag_time": findag_time,
            "hashtimer": hashtimer.to_vec(),
            "public_key": faucet_keypair.public().encode_protobuf(),
            "shard_id": 0
        });

        // Send funding transaction
        match client.post(format!("{}/tx", cli.node_url))
            .json(&tx_request)
            .send()
            .await {
            Ok(response) => {
                if response.status().is_success() {
                    println!("  ✅ Funded {} with {} USD", account, cli.amount);
                } else {
                    let status = response.status();
                    let error_text = response.text().await.unwrap_or_default();
                    println!("  ❌ Failed to fund {account}: {status} - {error_text}");
                }
            }
            Err(e) => {
                println!("  ❌ Network error funding {account}: {e}");
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
    let addr_hex = hex::encode(&pubkey[..8]); // Use first 8 bytes for shorter address
    format!("fdg1q{addr_hex}")
} 