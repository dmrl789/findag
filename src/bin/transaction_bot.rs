use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use tokio::time::{sleep, Duration};
use reqwest::Client;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use findag::core::types::Transaction;
use findag::core::address::Address;
use rand07::Rng;
use ed25519_dalek::{Keypair, Signature, PublicKey, Signer};

#[derive(Parser)]
#[command(name = "findag-transaction-bot")]
#[command(about = "FinDAG Transaction Bot: Stress testing and load generation", long_about = None)]
struct Cli {
    #[arg(long, default_value = "http://127.0.0.1:3000")]
    node_url: String,
    
    #[arg(long, default_value = "3")]
    interval_ms: u64,
    
    #[arg(long)]
    duration_seconds: Option<u64>,
    
    #[arg(long, default_value = "1")]
    concurrent_bots: u32,
    
    #[arg(long, default_value = "1000")]
    max_transactions: Option<u64>,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a simple transaction bot
    Start {},
    /// Run stress test with multiple concurrent bots
    Stress {
        #[arg(long, default_value = "10")]
        num_bots: u32,
        
        #[arg(long, default_value = "60")]
        duration_seconds: u64,
    },
    /// Run a burst test (high frequency for short duration)
    Burst {
        #[arg(long, default_value = "100")]
        num_transactions: u64,
        
        #[arg(long, default_value = "10")]
        interval_ms: u64,
    },
    /// Run a realistic load test (variable intervals)
    Realistic {
        #[arg(long, default_value = "300")]
        duration_seconds: u64,
        
        #[arg(long, default_value = "5")]
        min_interval_ms: u64,
        
        #[arg(long, default_value = "50")]
        max_interval_ms: u64,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct TransactionResponse {
    success: bool,
    message: Option<String>,
    tx_id: Option<String>,
}

#[derive(Debug)]
struct BotStats {
    total_sent: AtomicU64,
    total_successful: AtomicU64,
    total_failed: AtomicU64,
    start_time: Instant,
}

impl BotStats {
    fn new() -> Self {
        Self {
            total_sent: AtomicU64::new(0),
            total_successful: AtomicU64::new(0),
            total_failed: AtomicU64::new(0),
            start_time: Instant::now(),
        }
    }
    
    fn increment_sent(&self) {
        self.total_sent.fetch_add(1, Ordering::Relaxed);
    }
    
    fn increment_successful(&self) {
        self.total_successful.fetch_add(1, Ordering::Relaxed);
    }
    
    fn increment_failed(&self) {
        self.total_failed.fetch_add(1, Ordering::Relaxed);
    }
    
    fn get_stats(&self) -> (u64, u64, u64, f64) {
        let sent = self.total_sent.load(Ordering::Relaxed);
        let successful = self.total_successful.load(Ordering::Relaxed);
        let failed = self.total_failed.load(Ordering::Relaxed);
        let duration = self.start_time.elapsed().as_secs_f64();
        (sent, successful, failed, duration)
    }
    
    fn print_stats(&self, bot_id: &str) {
        let (sent, successful, failed, duration) = self.get_stats();
        let tps = if duration > 0.0 { successful as f64 / duration } else { 0.0 };
        let success_rate = if sent > 0 { (successful as f64 / sent as f64) * 100.0 } else { 0.0 };
        
        println!("[Bot-{}] Stats: {} sent, {} successful, {} failed, {:.2} TPS, {:.1}% success rate", 
                 bot_id, sent, successful, failed, tps, success_rate);
    }
}

struct TransactionBot {
    client: Client,
    node_url: String,
    bot_id: String,
    stats: Arc<BotStats>,
}

impl TransactionBot {
    fn new(node_url: String, bot_id: String, stats: Arc<BotStats>) -> Self {
        Self {
            client: Client::new(),
            node_url,
            bot_id,
            stats,
        }
    }
    
    fn create_test_transaction(&self) -> Transaction {
        let mut rng = rand07::thread_rng();
        
        // Generate random addresses
        let from_addr = format!("fdg1qbot{}", rng.gen_range(100000, 999999));
        let to_addr = format!("fdg1qbot{}", rng.gen_range(100000, 999999));
        
        // Generate random amount
        let amount = rng.gen_range(1, 1000);
        
        // Generate a dummy keypair for signing
        let keypair = Keypair::generate(&mut rng);
        
        // Create a dummy signature (in real usage, this would be properly signed)
        let dummy_message = format!("Test transaction from bot-{}", self.bot_id);
        let signature = keypair.sign(dummy_message.as_bytes());
        
        // Create transaction
        Transaction {
            from: Address(from_addr),
            to: Address(to_addr),
            amount,
            payload: format!("Test transaction from bot-{}", self.bot_id).into_bytes(),
            findag_time: chrono::Utc::now().timestamp_micros() as u64,
            hashtimer: rng.gen(),
            signature,
            public_key: keypair.public,
            shard_id: findag::core::types::ShardId(0),
            source_shard: None,
            dest_shard: None,
            target_chain: None,
            bridge_protocol: None,
        }
    }
    
    async fn send_transaction(&self) -> bool {
        let tx = self.create_test_transaction();
        self.stats.increment_sent();
        
        match self.client.post(&format!("{}/tx", self.node_url))
            .json(&tx)
            .timeout(Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    self.stats.increment_successful();
                    println!("[Bot-{}] ✅ Transaction sent: {} tokens", self.bot_id, tx.amount);
                    true
                } else {
                    self.stats.increment_failed();
                    println!("[Bot-{}] ❌ Failed to send transaction: {}", self.bot_id, response.status());
                    false
                }
            }
            Err(e) => {
                self.stats.increment_failed();
                println!("[Bot-{}] ❌ Error sending transaction: {}", self.bot_id, e);
                false
            }
        }
    }
    
    async fn run_simple(&self, interval_ms: u64, max_transactions: Option<u64>) {
        println!("🤖 Starting transaction bot {}", self.bot_id);
        println!("📡 Target: {}", self.node_url);
        println!("⏱️  Interval: {}ms", interval_ms);
        
        let mut tx_count = 0;
        let interval = Duration::from_millis(interval_ms);
        
        loop {
            if let Some(max) = max_transactions {
                if tx_count >= max {
                    break;
                }
            }
            
            self.send_transaction().await;
            tx_count += 1;
            
            sleep(interval).await;
        }
        
        self.stats.print_stats(&self.bot_id);
    }
    
    async fn run_burst(&self, num_transactions: u64, interval_ms: u64) {
        println!("🚀 Starting burst test bot {}", self.bot_id);
        println!("📡 Target: {}", self.node_url);
        println!("💥 Sending {} transactions with {}ms intervals", num_transactions, interval_ms);
        
        let interval = Duration::from_millis(interval_ms);
        
        for i in 0..num_transactions {
            self.send_transaction().await;
            
            if i < num_transactions - 1 {
                sleep(interval).await;
            }
        }
        
        self.stats.print_stats(&self.bot_id);
    }
    
    async fn run_realistic(&self, duration_seconds: u64, min_interval_ms: u64, max_interval_ms: u64) {
        println!("🎯 Starting realistic load bot {}", self.bot_id);
        println!("📡 Target: {}", self.node_url);
        println!("⏱️  Duration: {}s, Interval: {}-{}ms", duration_seconds, min_interval_ms, max_interval_ms);
        
        let start_time = Instant::now();
        let duration = Duration::from_secs(duration_seconds);
        
        while start_time.elapsed() < duration {
            self.send_transaction().await;
            
            // Random interval between min and max
            let interval = rand07::thread_rng().gen_range(min_interval_ms, max_interval_ms + 1);
            sleep(Duration::from_millis(interval)).await;
        }
        
        self.stats.print_stats(&self.bot_id);
    }
}

async fn run_stress_test(node_url: String, num_bots: u32, duration_seconds: u64) {
    println!("🔥 Starting stress test with {} bots for {} seconds", num_bots, duration_seconds);
    println!("📡 Target: {}", node_url);
    
    let stats = Arc::new(BotStats::new());
    let mut handles = vec![];
    
    for i in 0..num_bots {
        let bot_url = node_url.clone();
        let bot_id = format!("{:02}", i + 1);
        let bot_stats = stats.clone();
        
        let handle = tokio::spawn(async move {
            let bot = TransactionBot::new(bot_url, bot_id, bot_stats);
            bot.run_simple(100, None).await; // 100ms interval
        });
        
        handles.push(handle);
    }
    
    // Wait for duration
    sleep(Duration::from_secs(duration_seconds)).await;
    
    // Cancel all bots
    for handle in handles {
        handle.abort();
    }
    
    // Print final stats
    stats.print_stats("TOTAL");
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let stats = Arc::new(BotStats::new());
    
    match &cli.command {
        Commands::Start {} => {
            let bot = TransactionBot::new(
                cli.node_url.clone(),
                "01".to_string(),
                stats.clone(),
            );
            bot.run_simple(cli.interval_ms, cli.max_transactions).await;
        }
        
        Commands::Stress { num_bots, duration_seconds } => {
            run_stress_test(cli.node_url, *num_bots, *duration_seconds).await;
        }
        
        Commands::Burst { num_transactions, interval_ms } => {
            let bot = TransactionBot::new(
                cli.node_url.clone(),
                "BURST".to_string(),
                stats.clone(),
            );
            bot.run_burst(*num_transactions, *interval_ms).await;
        }
        
        Commands::Realistic { duration_seconds, min_interval_ms, max_interval_ms } => {
            let bot = TransactionBot::new(
                cli.node_url.clone(),
                "REAL".to_string(),
                stats.clone(),
            );
            bot.run_realistic(*duration_seconds, *min_interval_ms, *max_interval_ms).await;
        }
    }
} 