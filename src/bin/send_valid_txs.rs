use findag::core::types::Transaction;
use findag::core::address::Address;
use ed25519_dalek::{Keypair, Signer};
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::Rng;
use rand::rngs::OsRng;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("🚀 Sending valid transactions to see DAG building up...");
    
    let client = Client::new();
    let node_url = "http://localhost:3000";
    
    // Funded accounts (these should have been funded by fund_accounts)
    let funded_accounts = vec![
        "fdg1qalice1234567890",
        "fdg1qbob1234567890", 
        "fdg1qcharlie1234567890",
        "fdg1qdiana1234567890",
        "fdg1qedward1234567890",
    ];
    
    let mut rng = StdRng::from_rng(OsRng).unwrap();
    let mut tx_count = 0;
    
    // Send transactions for 30 seconds
    let start_time = std::time::Instant::now();
    let duration = Duration::from_secs(30);
    
    while start_time.elapsed() < duration {
        // Pick random sender and receiver
        let from_idx = rng.gen_range(0, funded_accounts.len());
        let mut to_idx = rng.gen_range(0, funded_accounts.len());
        while to_idx == from_idx {
            to_idx = rng.gen_range(0, funded_accounts.len());
        }
        
        let from = funded_accounts[from_idx];
        let to = funded_accounts[to_idx];
        let amount = rng.gen_range(1, 100); // Small amounts to avoid draining accounts
        
        // Create a dummy keypair for signing (in real usage, you'd use the actual private key)
        let keypair = Keypair::generate(&mut rng);
        
        // Create transaction payload (asset instruction)
        let payload = format!("transfer:{}:{}:USD", from, to).into_bytes();
        
        // Create transaction
        let tx = Transaction {
            from: Address(from.to_string()),
            to: Address(to.to_string()),
            amount,
            payload,
            findag_time: chrono::Utc::now().timestamp() as u64,
            hashtimer: [0u8; 32], // Will be set by the node
            signature: keypair.sign(format!("{}{}{}", from, to, amount).as_bytes()),
            public_key: keypair.public,
            shard_id: findag::core::types::ShardId(0),
            source_shard: None,
            dest_shard: None,
            target_chain: None,
            bridge_protocol: None,
        };
        
        // Send transaction to node
        let tx_json = json!({
            "from": from,
            "to": to,
            "amount": amount,
            "currency": "USD",
            "shard_id": 0
        });
        
        match client.post(&format!("{}/tx", node_url))
            .json(&tx_json)
            .send()
            .await {
                Ok(response) => {
                    if response.status().is_success() {
                        println!("✅ Tx #{}: {} -> {} ({} USD) - ACCEPTED", tx_count, from, to, amount);
                    } else {
                        println!("❌ Tx #{}: {} -> {} ({} USD) - REJECTED: {}", 
                                tx_count, from, to, amount, response.status());
                    }
                }
                Err(e) => {
                    println!("❌ Tx #{}: Failed to send: {}", tx_count, e);
                }
            }
        
        tx_count += 1;
        
        // Wait a bit between transactions
        sleep(Duration::from_millis(500)).await;
    }
    
    println!("🎉 Sent {} transactions! Check your FinDAG logs to see the DAG building up!", tx_count);
    println!("You should now see blocks and rounds being created instead of just transaction rejections.");
} 