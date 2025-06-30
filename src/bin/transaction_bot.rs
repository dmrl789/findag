use clap::{Parser, Subcommand};
use reqwest;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;
use ed25519_dalek::{Keypair, Signer};
use rand::rngs::OsRng;
use base64::engine::general_purpose;
use base64::Engine;
use hex;
use rand::RngCore;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(long, default_value = "http://127.0.0.1:3000")]
    node_url: String,
    
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Start {
        #[arg(long, default_value = "30")]
        duration_seconds: u64,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionRequest {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub signature: Vec<u8>,
    pub payload: Vec<u8>,
    pub findag_time: u64,
    pub hashtimer: Vec<u8>,
    pub public_key: Vec<u8>,
    pub shard_id: u16,
}

struct Bot {
    id: String,
    keypair: Keypair,
    nonce: u64,
    client: reqwest::Client,
    node_url: String,
}

impl Bot {
    fn new(id: String, node_url: String) -> Self {
        let mut rng = OsRng;
        let keypair = Keypair::generate(&mut rng);
        
        Self {
            id,
            keypair,
            nonce: 1,
            client: reqwest::Client::new(),
            node_url,
        }
    }

    fn get_public_key_hex(&self) -> String {
        hex::encode(self.keypair.public.to_bytes())
    }

    async fn send_transaction(&mut self, to: &str, amount: u64) -> Result<(), Box<dyn std::error::Error>> {
        // Create canonical message to sign
        let message = format!("{}{}{}", self.get_public_key_hex(), to, amount);
        
        // Sign the message
        let signature = self.keypair.sign(message.as_bytes());
        let signature_bytes = signature.to_bytes();

        // Create transaction request
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let findag_time = (now.as_nanos() / 100) as u64;
        let mut hashtimer = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut hashtimer);
        let public_key = self.keypair.public.to_bytes().to_vec();
        let tx_request = TransactionRequest {
            from: self.get_public_key_hex(),
            to: to.to_string(),
            amount,
            signature: signature_bytes.to_vec(),
            payload: message.as_bytes().to_vec(),
            findag_time,
            hashtimer,
            public_key,
            shard_id: 0,
        };

        println!("[Bot-{}] Sending transaction: from={}, to={}, amount={}", 
                self.id, self.get_public_key_hex(), to, amount);

        // Send to node
        let res = self.client
            .post(&format!("{}/tx", self.node_url))
            .json(&tx_request)
            .send()
            .await?;

        if res.status().is_success() {
            println!("[Bot-{}] Transaction successful", self.id);
            self.nonce += 1;
        } else {
            let status = res.status();
            let err_text = res.text().await.unwrap_or_else(|_| "<no body>".to_string());
            println!(
                "[Bot-{}] Transaction failed with status: {}. Error body: {}",
                self.id, status, err_text
            );
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match &cli.command {
        Some(Commands::Start { duration_seconds }) => {
            println!("🤖 Starting transaction bot 01");
            println!("📡 Target: {}", cli.node_url);
            println!("⏱️  Interval: 3ms");
            
            let mut bot = Bot::new("01".to_string(), cli.node_url.clone());
            let start_time = std::time::Instant::now();
            let duration = Duration::from_secs(*duration_seconds);
            
            let mut sent = 0;
            let mut successful = 0;
            let mut failed = 0;
            
            while start_time.elapsed() < duration {
                let recipients = vec![
                    "fdg1qalice1234567890",
                    "fdg1qbob1234567890", 
                    "fdg1qcharlie1234567890",
                    "fdg1qdiana1234567890",
                    "fdg1qedward1234567890",
                ];
                
                let recipient = recipients[rand::random::<usize>() % recipients.len()];
                let amount = rand::random::<u64>() % 1000 + 1;
                
                sent += 1;
                match bot.send_transaction(recipient, amount).await {
                    Ok(_) => successful += 1,
                    Err(_) => failed += 1,
                }
                
                sleep(Duration::from_millis(3)).await;
            }
            
            let elapsed = start_time.elapsed();
            let tps = if elapsed.as_secs() > 0 { 
                successful as f64 / elapsed.as_secs() as f64 
            } else { 
                0.0 
            };
            let success_rate = if sent > 0 { 
                (successful as f64 / sent as f64) * 100.0 
            } else { 
                0.0 
            };
            
            println!("[Bot-{}] Stats: {} sent, {} successful, {} failed, {:.2} TPS, {:.1}% success rate", 
                    bot.id, sent, successful, failed, tps, success_rate);
        }
        None => {
            println!("No command specified. Use 'start' to begin sending transactions.");
        }
    }
    
    Ok(())
} 