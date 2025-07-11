use clap::{Parser, Subcommand};
use std::time::Duration;
use tokio::time::sleep;
use libp2p_identity::Keypair;
use findag::core::types::Transaction;
use findag::core::address::Address;
use reqwest::Client;
use serde_json::json;
use ed25519_dalek::{Signature, VerifyingKey};

#[derive(Parser)]
#[command(name = "transaction_bot")]
#[command(about = "A bot that sends transactions to the FinDAG network")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start {
        #[arg(long, default_value = "http://127.0.0.1:3000")]
        node_url: String,
        
        #[arg(long, default_value = "3")]
        interval_ms: u64,
    },
}

struct TransactionBot {
    client: Client,
    node_url: String,
    interval_ms: u64,
    keypair: Keypair,
}

impl TransactionBot {
    fn new(node_url: String, interval_ms: u64) -> Self {
        // Use deterministic keypair generation
        let mut bot_seed = [42u8; 32]; // Fixed seed for deterministic behavior
        
        // Generate keypair using libp2p_identity
        let keypair = Keypair::ed25519_from_bytes(&mut bot_seed).expect("32-byte seed");
        
        // Convert to ed25519_dalek types for address generation
        let public_key_bytes = keypair.public().encode_protobuf();
        let verifying_key = VerifyingKey::from_bytes(&public_key_bytes[..32].try_into().unwrap()).unwrap();
        
        // Verification output
        println!("VERIFICATION:");
        println!("Seed: {:?}", &bot_seed);
        println!("Public Key: {}", hex::encode(keypair.public().encode_protobuf()));
        println!("Address: {}", Address::from_verifying_key(&verifying_key));
        
        Self {
            client: Client::new(),
            node_url,
            interval_ms,
            keypair,
        }
    }
    
    fn get_from_address(&self) -> Address {
        let public_key_bytes = self.keypair.public().encode_protobuf();
        let verifying_key = VerifyingKey::from_bytes(&public_key_bytes[..32].try_into().unwrap()).unwrap();
        Address::from_verifying_key(&verifying_key)
    }
    
    async fn send_transaction(&self, from: &Address, to: &Address, amount: u64) -> Result<(), Box<dyn std::error::Error>> {
        // Create a simple transaction payload
        let payload = format!("Transfer {amount} USD from {from} to {to}").into_bytes();
        
        // Get current FinDAG time (use system time for now)
        let findag_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        // Create hashtimer (simplified - just use first 8 bytes of findag_time)
        let mut hashtimer = [0u8; 32];
        hashtimer[0..8].copy_from_slice(&findag_time.to_le_bytes());
        
        // Create the message to sign (must match what the API expects)
        // The API expects: format!("{}{}{}", signed_tx.from, signed_tx.to, signed_tx.amount)
        // where from, to are strings, not Address objects
        let message = format!("{}{}{}", from.as_str(), to.as_str(), amount);
        let signature_bytes = self.keypair.sign(message.as_bytes()).unwrap();
        
        // Convert to ed25519::Signature
        let signature = Signature::from_bytes(&signature_bytes.clone().try_into().unwrap());
        
        // Convert public key
        let public_key_bytes = self.keypair.public().encode_protobuf();
        let public_key = VerifyingKey::from_bytes(&public_key_bytes[..32].try_into().unwrap()).unwrap();
        
        println!("[Bot-01] Sending transaction: from={from}, to={to}, amount={amount}");
        
        let _tx = Transaction {
            from: from.clone(),
            to: to.clone(),
            amount,
            payload: payload.clone(),
            findag_time,
            hashtimer,
            signature,
            public_key,
            shard_id: findag::core::types::ShardId(0),
            source_shard: None,
            dest_shard: None,
            target_chain: None,
            bridge_protocol: None,
        };
        
        // Create the request payload that matches TransactionRequest struct
        let request_payload = json!({
            "from": from.as_str(),
            "to": to.as_str(),
            "amount": amount,
            "signature": signature_bytes,
            "payload": payload,
            "findag_time": findag_time,
            "hashtimer": hashtimer.to_vec(),
            "public_key": public_key_bytes[..32].to_vec(),
            "shard_id": 0
        });
        
        let response = self.client
            .post(format!("{}/tx", self.node_url))
            .json(&request_payload)
            .send()
            .await?;
        
        if response.status().is_success() {
            println!("[Bot-01] Transaction sent successfully");
        } else {
            let status = response.status();
            let error_body = response.text().await?;
            println!("[Bot-01] Transaction failed with status: {} {}. Error body: {}", 
                status.as_u16(), 
                status.canonical_reason().unwrap_or("Unknown"), 
                error_body);
        }
        
        Ok(())
    }
    
    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let from_address = self.get_from_address();
        
        // Test recipient addresses
        let test_addresses = vec![
            Address::new("fdg1qalice1234567890".to_string()),
            Address::new("fdg1qbob1234567890".to_string()),
            Address::new("fdg1qcharlie1234567890".to_string()),
            Address::new("fdg1qdiana1234567890".to_string()),
            Address::new("fdg1qedward1234567890".to_string()),
        ];
        
        let mut transaction_count = 0;
        let mut successful_count = 0;
        let mut failed_count = 0;
        let start_time = std::time::Instant::now();
        
        loop {
            for to_address in &test_addresses {
                let amount = (transaction_count % 1000) + 1; // 1-1000 USD
                
                match self.send_transaction(&from_address, to_address, amount).await {
                    Ok(_) => successful_count += 1,
                    Err(_) => failed_count += 1,
                }
                
                transaction_count += 1;
                
                // Print stats every 100 transactions
                if transaction_count % 100 == 0 {
                    let elapsed = start_time.elapsed().as_secs_f64();
                    let tps = transaction_count as f64 / elapsed;
                    let success_rate = (successful_count as f64 / transaction_count as f64) * 100.0;
                    println!("[Bot-01] Stats: {transaction_count} sent, {successful_count} successful, {failed_count} failed, {tps:.2} TPS, {success_rate:.1}% success rate");
                }
                
                sleep(Duration::from_millis(self.interval_ms)).await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Start { node_url, interval_ms } => {
            println!("🤖 Starting transaction bot 01");
            println!("📡 Target: {node_url}");
            println!("⏱️  Interval: {interval_ms}ms");
            
            let bot = TransactionBot::new(node_url, interval_ms);
            bot.run().await?;
        }
    }
    
    Ok(())
} 