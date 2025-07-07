use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use serde::{Serialize, Deserialize};
use crate::storage::PersistentStorage;
use crate::core::types::{Block, Transaction};
use crate::core::address::Address;
use ed25519_dalek::{Signature, VerifyingKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestConfig {
    pub concurrent_users: u32,
    pub requests_per_user: u32,
    pub ramp_up_duration: Duration,
    pub test_duration: Duration,
    pub target_tps: u32,
    pub load_pattern: LoadPattern,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadPattern {
    Constant,
    RampUp,
    Spike,
    Burst,
    Random,
    Realistic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestResult {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub total_duration: Duration,
    pub avg_response_time: f64,
    pub min_response_time: f64,
    pub max_response_time: f64,
    pub p50_response_time: f64,
    pub p95_response_time: f64,
    pub p99_response_time: f64,
    pub throughput_tps: f64,
    pub error_rate: f64,
    pub response_times: Vec<f64>,
    pub errors: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct LoadTester {
    storage: Arc<PersistentStorage>,
    config: LoadTestConfig,
}

impl LoadTester {
    pub fn new(storage: Arc<PersistentStorage>, config: LoadTestConfig) -> Self {
        Self { storage, config }
    }

    pub async fn run_load_test(&self) -> LoadTestResult {
        println!("🚀 Starting load test with {} concurrent users", self.config.concurrent_users);
        
        let start_time = Instant::now();
        let semaphore = Arc::new(Semaphore::new(self.config.concurrent_users as usize));
        let mut handles = Vec::new();
        let mut response_times = Vec::new();
        let mut errors = Vec::new();
        
        // Create load pattern
        let load_pattern = self.generate_load_pattern();
        
        for (user_id, delay) in load_pattern.into_iter().enumerate() {
            let semaphore = semaphore.clone();
            let storage = self.storage.clone();
            let config = self.config.clone();
            
            let handle = tokio::spawn(async move {
                // Wait for ramp-up delay
                tokio::time::sleep(delay).await;
                
                let mut user_response_times = Vec::new();
                let mut user_errors = Vec::new();
                
                for request_id in 0..config.requests_per_user {
                    let _permit = semaphore.acquire().await.unwrap();
                    
                    let request_start = Instant::now();
                    let result = Self::execute_request(&storage, user_id, request_id).await;
                    let response_time = request_start.elapsed().as_millis() as f64;
                    
                    user_response_times.push(response_time);
                    
                    if let Err(e) = result {
                        user_errors.push(format!("User {} Request {}: {}", user_id, request_id, e));
                    }
                    
                    // Respect target TPS
                    if config.target_tps > 0 {
                        let target_interval = 1000.0 / config.target_tps as f64;
                        if response_time < target_interval {
                            tokio::time::sleep(Duration::from_millis((target_interval - response_time) as u64)).await;
                        }
                    }
                }
                
                (user_response_times, user_errors)
            });
            
            handles.push(handle);
        }
        
        // Collect results
        for handle in handles {
            if let Ok((user_times, user_errors)) = handle.await {
                response_times.extend(user_times);
                errors.extend(user_errors);
            }
        }
        
        let total_duration = start_time.elapsed();
        let total_requests = response_times.len() as u64;
        let failed_requests = errors.len() as u64;
        let successful_requests = total_requests - failed_requests;
        
        // Calculate percentiles
        response_times.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let p50 = Self::percentile(&response_times, 50.0);
        let p95 = Self::percentile(&response_times, 95.0);
        let p99 = Self::percentile(&response_times, 99.0);
        
        LoadTestResult {
            total_requests,
            successful_requests,
            failed_requests,
            total_duration,
            avg_response_time: response_times.iter().sum::<f64>() / response_times.len() as f64,
            min_response_time: *response_times.first().unwrap_or(&0.0),
            max_response_time: *response_times.last().unwrap_or(&0.0),
            p50_response_time: p50,
            p95_response_time: p95,
            p99_response_time: p99,
            throughput_tps: total_requests as f64 / total_duration.as_secs_f64(),
            error_rate: (failed_requests as f64 / total_requests as f64) * 100.0,
            response_times,
            errors,
            timestamp: chrono::Utc::now(),
        }
    }
    
    fn generate_load_pattern(&self) -> Vec<Duration> {
        match self.config.load_pattern {
            LoadPattern::Constant => {
                vec![Duration::from_millis(0); self.config.concurrent_users as usize]
            },
            LoadPattern::RampUp => {
                let interval = self.config.ramp_up_duration / self.config.concurrent_users;
                (0..self.config.concurrent_users).map(|i| interval * i).collect()
            },
            LoadPattern::Spike => {
                let mut delays = vec![Duration::from_millis(0); self.config.concurrent_users as usize];
                let spike_users = self.config.concurrent_users / 4;
                for i in 0..spike_users {
                    delays[i as usize] = Duration::from_millis(100 * i as u64);
                }
                delays
            },
            LoadPattern::Burst => {
                let mut delays = Vec::new();
                let burst_size = self.config.concurrent_users / 5;
                for i in 0..self.config.concurrent_users {
                    let burst_id = i / burst_size;
                    delays.push(Duration::from_millis(burst_id as u64 * 500));
                }
                delays
            },
            LoadPattern::Random => {
                use rand::Rng;
                let mut rng = rand::thread_rng();
                (0..self.config.concurrent_users).map(|_| {
                    Duration::from_millis(rng.gen_range(0..1000))
                }).collect()
            },
            LoadPattern::Realistic => {
                // Simulate realistic user behavior with think time
                let mut delays = Vec::new();
                for i in 0..self.config.concurrent_users {
                    let base_delay = i * 100; // Staggered start
                    let think_time = (i % 3) * 200; // Variable think time
                    delays.push(Duration::from_millis((base_delay + think_time) as u64));
                }
                delays
            }
        }
    }
    
    async fn execute_request(storage: &Arc<PersistentStorage>, user_id: usize, request_id: u32) -> Result<(), String> {
        // Simulate different types of requests
        match request_id % 4 {
            0 => {
                // Write operation
                let key = format!("load_test_user_{}_req_{}", user_id, request_id);
                let value = format!("value_{}_{}", user_id, request_id);
                storage.store_parameter(&key, &value)
                    .map_err(|e| format!("Write failed: {}", e))?;
            },
            1 => {
                // Read operation
                let key = format!("load_test_user_{}_req_{}", user_id, request_id % 100);
                storage.load_parameter(&key)
                    .map_err(|e| format!("Read failed: {}", e))?;
            },
            2 => {
                // Block creation simulation
                let block = Self::create_test_block(user_id, request_id);
                let _serialized = bincode::serialize(&block)
                    .map_err(|e| format!("Block serialization failed: {}", e))?;
            },
            3 => {
                // Transaction processing simulation
                let tx = Self::create_test_transaction(user_id, request_id);
                let _serialized = bincode::serialize(&tx)
                    .map_err(|e| format!("Transaction serialization failed: {}", e))?;
            },
            _ => unreachable!()
        }
        
        Ok(())
    }
    
    fn create_test_block(user_id: usize, request_id: u32) -> Block {
        Block {
            block_id: [user_id as u8; 32],
            parent_blocks: vec![],
            transactions: vec![],
            findag_time: request_id as u64,
            hashtimer: [request_id as u8; 32],
            proposer: Address::random(),
            signature: Signature::from_bytes(&[0u8; 64]),
            public_key: VerifyingKey::from_bytes(&[0u8; 32]).unwrap(),
            shard_id: crate::core::types::ShardId(0),
            merkle_root: None,
        }
    }
    
    fn create_test_transaction(user_id: usize, request_id: u32) -> Transaction {
        Transaction {
            from: Address::random(),
            to: Address::random(),
            amount: request_id as u64,
            payload: format!("Test transaction {} from user {}", request_id, user_id).into_bytes(),
            findag_time: request_id as u64,
            hashtimer: [request_id as u8; 32],
            signature: Signature::from_bytes(&[0u8; 64]),
            public_key: VerifyingKey::from_bytes(&[0u8; 32]).unwrap(),
            shard_id: crate::core::types::ShardId(0),
            source_shard: None,
            dest_shard: None,
            target_chain: None,
            bridge_protocol: None,
        }
    }
    
    fn percentile(data: &[f64], percentile: f64) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let index = (percentile / 100.0 * (data.len() - 1) as f64).round() as usize;
        data[index.min(data.len() - 1)]
    }
}

impl LoadTestResult {
    pub fn print_summary(&self) {
        println!("📊 Load Test Results Summary");
        println!("============================");
        println!("Total Requests: {}", self.total_requests);
        println!("Successful: {}", self.successful_requests);
        println!("Failed: {}", self.failed_requests);
        println!("Error Rate: {:.2}%", self.error_rate);
        println!("Throughput: {:.2} TPS", self.throughput_tps);
        println!("Average Response Time: {:.2}ms", self.avg_response_time);
        println!("Min Response Time: {:.2}ms", self.min_response_time);
        println!("Max Response Time: {:.2}ms", self.max_response_time);
        println!("P50 Response Time: {:.2}ms", self.p50_response_time);
        println!("P95 Response Time: {:.2}ms", self.p95_response_time);
        println!("P99 Response Time: {:.2}ms", self.p99_response_time);
        println!("Total Duration: {:.2}s", self.total_duration.as_secs_f64());
        println!();
    }
    
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
} 