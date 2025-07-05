use clap::{Parser, Subcommand};
use reqwest;
use std::time::Duration;
use tokio::time::sleep;
use ed25519_dalek::{Keypair, Signer};
use findag::core::types::Transaction;
use findag::core::address::Address;
use reqwest::Client;
use serde_json::json;
use hex;

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
        // Use deterministic keypair generation (RFC 8032 compliant)
        let bot_seed = [42u8; 32]; // Fixed seed for deterministic behavior
        
        // Method A: RFC-compliant deterministic keypair from seed
        let secret_key = ed25519_dalek::SecretKey::from_bytes(&bot_seed).expect("32-byte seed");
        let public_key = (&secret_key).into();
        let keypair = Keypair { secret: secret_key, public: public_key };
        
        // Verification output
        println!("VERIFICATION:");
        println!("Seed: {:?}", &bot_seed);
        println!("Public Key: {}", hex::encode(keypair.public.as_bytes()));
        println!("Address: {}", Address::from_public_key(&keypair.public));
        
        Self {
            client: Client::new(),
            node_url,
            interval_ms,
            keypair,
        }
    }
    
    fn get_from_address(&self) -> Address {
        Address::from_public_key(&self.keypair.public)
    }
    
    async fn send_transaction(&self, from: &Address, to: &Address, amount: u64) -> Result<(), Box<dyn std::error::Error>> {
        // Create a simple transaction payload
        let payload = format!("Transfer {} USD from {} to {}", amount, from, to).into_bytes();
        
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
        let signature = self.keypair.sign(message.as_bytes());
        
        println!("[Bot-01] Sending transaction: from={}, to={}, amount={}", from, to, amount);
        
        let _tx = Transaction {
            from: from.clone(),
            to: to.clone(),
            amount,
            payload: payload.clone(),
            findag_time,
            hashtimer,
            signature,
            public_key: self.keypair.public.clone(),
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
            "signature": signature.to_bytes().to_vec(),
            "payload": payload,
            "findag_time": findag_time,
            "hashtimer": hashtimer.to_vec(),
            "public_key": self.keypair.public.to_bytes().to_vec(),
            "shard_id": 0
        });
        
        let response = self.client
            .post(&format!("{}/tx", self.node_url))
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
                    println!("[Bot-01] Stats: {} sent, {} successful, {} failed, {:.2} TPS, {:.1}% success rate", 
                        transaction_count, successful_count, failed_count, tps, success_rate);
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
            println!("📡 Target: {}", node_url);
            println!("⏱️  Interval: {}ms", interval_ms);
            
            let bot = TransactionBot::new(node_url, interval_ms);
            bot.run().await?;
        }
    }
    
    Ok(())
} 