use findag::storage::{initialize_high_frequency_storage, get_storage_metrics};
use findag::core::types::{Block, Round, SerializableBlock, SerializableRound};
use findag::consensus::validator_set::ValidatorSet;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use serde_json::json;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use findag::core::address::Address;
use ed25519_dalek::{Signature, VerifyingKey};
use findag::core::types::ShardId;

#[derive(Debug)]
struct LoadTestConfig {
    duration_seconds: u64,
    target_tps: u64, // Transactions per second
    concurrent_users: u64,
    ramp_up_seconds: u64,
    ramp_down_seconds: u64,
    test_type: LoadTestType,
}

#[derive(Debug, Clone)]
enum LoadTestType {
    ConstantLoad,
    RampUpLoad,
    BurstLoad,
    SpikeLoad,
    EnduranceTest,
}

#[derive(Debug)]
struct LoadTestMetrics {
    total_requests: AtomicU64,
    successful_requests: AtomicU64,
    failed_requests: AtomicU64,
    total_response_time: AtomicU64, // in milliseconds
    min_response_time: Mutex<u64>,
    max_response_time: Mutex<u64>,
    start_time: Instant,
    end_time: Mutex<Option<Instant>>,
}

impl LoadTestMetrics {
    fn new() -> Self {
        Self {
            total_requests: AtomicU64::new(0),
            successful_requests: AtomicU64::new(0),
            failed_requests: AtomicU64::new(0),
            total_response_time: AtomicU64::new(0),
            min_response_time: Mutex::new(u64::MAX),
            max_response_time: Mutex::new(0),
            start_time: Instant::now(),
            end_time: Mutex::new(None),
        }
    }

    fn record_request(&self, response_time_ms: u64, success: bool) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        
        if success {
            self.successful_requests.fetch_add(1, Ordering::Relaxed);
        } else {
            self.failed_requests.fetch_add(1, Ordering::Relaxed);
        }
        
        self.total_response_time.fetch_add(response_time_ms, Ordering::Relaxed);
        
        // Update min/max response times
        {
            let mut min_time = self.min_response_time.lock().unwrap();
            *min_time = (*min_time).min(response_time_ms);
        }
        
        {
            let mut max_time = self.max_response_time.lock().unwrap();
            *max_time = (*max_time).max(response_time_ms);
        }
    }

    fn finish(&self) {
        let mut end_time = self.end_time.lock().unwrap();
        *end_time = Some(Instant::now());
    }

    fn get_summary(&self) -> LoadTestSummary {
        let total = self.total_requests.load(Ordering::Relaxed);
        let successful = self.successful_requests.load(Ordering::Relaxed);
        let failed = self.failed_requests.load(Ordering::Relaxed);
        let total_time = self.total_response_time.load(Ordering::Relaxed);
        
        let duration = if let Some(end_time) = *self.end_time.lock().unwrap() {
            end_time.duration_since(self.start_time).as_secs_f64()
        } else {
            self.start_time.elapsed().as_secs_f64()
        };
        
        let avg_response_time = if total > 0 {
            total_time as f64 / total as f64
        } else {
            0.0
        };
        
        let tps = if duration > 0.0 {
            total as f64 / duration
        } else {
            0.0
        };
        
        let success_rate = if total > 0 {
            (successful as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        
        LoadTestSummary {
            total_requests: total,
            successful_requests: successful,
            failed_requests: failed,
            avg_response_time_ms: avg_response_time,
            min_response_time_ms: *self.min_response_time.lock().unwrap(),
            max_response_time_ms: *self.max_response_time.lock().unwrap(),
            throughput_tps: tps,
            success_rate_percent: success_rate,
            duration_seconds: duration,
        }
    }
}

#[derive(Debug)]
struct LoadTestSummary {
    total_requests: u64,
    successful_requests: u64,
    failed_requests: u64,
    avg_response_time_ms: f64,
    min_response_time_ms: u64,
    max_response_time_ms: u64,
    throughput_tps: f64,
    success_rate_percent: f64,
    duration_seconds: f64,
}

impl LoadTestSummary {
    fn print(&self) {
        println!("=== Load Test Results ===");
        println!("Duration: {:.2} seconds", self.duration_seconds);
        println!("Total requests: {}", self.total_requests);
        println!("Successful requests: {}", self.successful_requests);
        println!("Failed requests: {}", self.failed_requests);
        println!("Success rate: {:.2}%", self.success_rate_percent);
        println!("Throughput: {:.2} TPS", self.throughput_tps);
        println!("Average response time: {:.2}ms", self.avg_response_time_ms);
        println!("Min response time: {}ms", self.min_response_time_ms);
        println!("Max response time: {}ms", self.max_response_time_ms);
        println!();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🔥 Starting FinDAG Load Test Suite");
    println!("===================================");

    // Initialize storage with high-frequency configuration
    println!("📊 Initializing high-frequency storage...");
    let (storage, monitor) = initialize_high_frequency_storage("load_test_db").await?;
    println!("✅ Storage initialized successfully");

    // Run different load tests
    let mut all_results = Vec::new();

    // Test 1: Constant Load Test
    println!("\n📈 Running Constant Load Test...");
    let constant_config = LoadTestConfig {
        duration_seconds: 60,
        target_tps: 1000,
        concurrent_users: 10,
        ramp_up_seconds: 10,
        ramp_down_seconds: 10,
        test_type: LoadTestType::ConstantLoad,
    };
    let constant_result = run_load_test(&storage, &constant_config).await;
    constant_result.print();
    all_results.push(("Constant Load".to_string(), constant_result));

    // Test 2: Ramp Up Load Test
    println!("\n📈 Running Ramp Up Load Test...");
    let ramp_config = LoadTestConfig {
        duration_seconds: 120,
        target_tps: 2000,
        concurrent_users: 20,
        ramp_up_seconds: 30,
        ramp_down_seconds: 30,
        test_type: LoadTestType::RampUpLoad,
    };
    let ramp_result = run_load_test(&storage, &ramp_config).await;
    ramp_result.print();
    all_results.push(("Ramp Up Load".to_string(), ramp_result));

    // Test 3: Burst Load Test
    println!("\n💥 Running Burst Load Test...");
    let burst_config = LoadTestConfig {
        duration_seconds: 30,
        target_tps: 5000,
        concurrent_users: 50,
        ramp_up_seconds: 5,
        ramp_down_seconds: 5,
        test_type: LoadTestType::BurstLoad,
    };
    let burst_result = run_load_test(&storage, &burst_config).await;
    burst_result.print();
    all_results.push(("Burst Load".to_string(), burst_result));

    // Test 4: Spike Load Test
    println!("\n⚡ Running Spike Load Test...");
    let spike_config = LoadTestConfig {
        duration_seconds: 90,
        target_tps: 3000,
        concurrent_users: 30,
        ramp_up_seconds: 10,
        ramp_down_seconds: 10,
        test_type: LoadTestType::SpikeLoad,
    };
    let spike_result = run_load_test(&storage, &spike_config).await;
    spike_result.print();
    all_results.push(("Spike Load".to_string(), spike_result));

    // Test 5: Endurance Test
    println!("\n⏰ Running Endurance Test...");
    let endurance_config = LoadTestConfig {
        duration_seconds: 300, // 5 minutes
        target_tps: 500,
        concurrent_users: 5,
        ramp_up_seconds: 30,
        ramp_down_seconds: 30,
        test_type: LoadTestType::EnduranceTest,
    };
    let endurance_result = run_load_test(&storage, &endurance_config).await;
    endurance_result.print();
    all_results.push(("Endurance Test".to_string(), endurance_result));

    // Get final database metrics
    println!("\n📊 Final Database Metrics:");
    if let Ok(metrics) = get_storage_metrics(&monitor) {
        println!("Size on disk: {} bytes", metrics.size_on_disk);
        println!("Write operations: {}", metrics.write_operations);
        println!("Read operations: {}", metrics.read_operations);
        println!("Error count: {}", metrics.error_count);
        println!("Uptime: {} seconds", metrics.uptime_seconds);
    }

    // Print overall summary
    print_load_test_summary(&all_results);

    // Generate load test report
    generate_load_test_report(&all_results).await?;

    println!("\n✅ Load test suite completed successfully!");
    Ok(())
}

async fn run_load_test(
    storage: &Arc<findag::storage::PersistentStorage>,
    config: &LoadTestConfig,
) -> LoadTestSummary {
    let metrics = Arc::new(LoadTestMetrics::new());
    let storage_clone = storage.clone();
    let metrics_clone = metrics.clone();

    println!("  Target TPS: {}", config.target_tps);
    println!("  Concurrent users: {}", config.concurrent_users);
    println!("  Duration: {} seconds", config.duration_seconds);

    // Calculate request intervals
    let request_interval_ms = 1000 / config.target_tps;
    let requests_per_user = config.target_tps / config.concurrent_users;

    // Spawn user threads
    let mut handles = vec![];
    
    for user_id in 0..config.concurrent_users {
        let storage_user = storage_clone.clone();
        let metrics_user = metrics_clone.clone();
        let user_requests = requests_per_user;
        let interval = request_interval_ms;
        
        let handle = tokio::spawn(async move {
            simulate_user_workload(storage_user, metrics_user, user_id, user_requests, interval).await;
        });
        handles.push(handle);
    }

    // Wait for all users to complete
    for handle in handles {
        handle.await.unwrap();
    }

    metrics.finish();
    metrics.get_summary()
}

async fn simulate_user_workload(
    storage: Arc<findag::storage::PersistentStorage>,
    metrics: Arc<LoadTestMetrics>,
    user_id: u64,
    requests_per_second: u64,
    interval_ms: u64,
) {
    let mut request_count = 0;
    let start_time = Instant::now();
    
    loop {
        let request_start = Instant::now();
        
        // Simulate different types of operations
        let operation_type = request_count % 4;
        let success = match operation_type {
            0 => {
                // Write operation
                let key = format!("load_test_user_{}_write_{}", user_id, request_count);
                let value = format!("load_test_value_{}_{}", user_id, request_count);
                storage.store_parameter(&key, &value).is_ok()
            },
            1 => {
                // Read operation
                let key = format!("load_test_user_{}_read_{}", user_id, request_count % 100);
                storage.load_parameter(&key).is_ok()
            },
            2 => {
                // Block serialization
                let block = create_test_block(request_count);
                let serializable = SerializableBlock::from(block);
                bincode::serialize(&serializable).is_ok()
            },
            _ => {
                // Round processing
                let round = create_test_round(request_count);
                let serializable = SerializableRound::from(round);
                bincode::serialize(&serializable).is_ok()
            }
        };
        
        let response_time = request_start.elapsed().as_millis() as u64;
        metrics.record_request(response_time, success);
        
        request_count += 1;
        
        // Check if we should stop
        if start_time.elapsed().as_secs() >= 60 { // Run for 1 minute per user
            break;
        }
        
        // Wait for next request interval
        if interval_ms > 0 {
            sleep(Duration::from_millis(interval_ms)).await;
        }
    }
}

fn create_test_block(id: u64) -> Block {
    Block {
        block_id: [id as u8; 32],
        parent_blocks: vec![],
        transactions: vec![],
        findag_time: id, // or use a timestamp if needed
        hashtimer: [id as u8; 32],
        proposer: Address::random(),
        signature: Signature::from_bytes(&[0u8; 64]),
        public_key: VerifyingKey::from_bytes(&[0u8; 32]).unwrap(),
        shard_id: ShardId(0),
        merkle_root: None,
    }
}

fn create_test_round(id: u64) -> Round {
    Round {
        round_number: id,
        parent_round_hash: [id as u8; 32],
        finalized_block_hashes: vec![],
        block_hashtimers: vec![],
        quorum_signature: vec![],
        findag_time: id, // or use a timestamp if needed
        proposer: Address::random(),
        proposer_signature: Signature::from_bytes(&[0u8; 64]),
        proposer_public_key: VerifyingKey::from_bytes(&[0u8; 32]).unwrap(),
    }
}

fn print_load_test_summary(results: &[(String, LoadTestSummary)]) {
    println!("\n📊 LOAD TEST SUMMARY");
    println!("====================");
    
    let mut total_requests = 0;
    let mut total_successful = 0;
    let mut total_failed = 0;
    let mut avg_tps = 0.0;
    let mut avg_success_rate = 0.0;
    let mut count = 0;

    for (test_name, result) in results {
        println!("{}: {} TPS, {:.2}% success rate", 
            test_name, result.throughput_tps, result.success_rate_percent);
        
        total_requests += result.total_requests;
        total_successful += result.successful_requests;
        total_failed += result.failed_requests;
        avg_tps += result.throughput_tps;
        avg_success_rate += result.success_rate_percent;
        count += 1;
    }

    if count > 0 {
        avg_tps /= count as f64;
        avg_success_rate /= count as f64;
    }

    let overall_success_rate = if total_requests > 0 {
        (total_successful as f64 / total_requests as f64) * 100.0
    } else {
        0.0
    };

    println!("\nOverall Results:");
    println!("Total requests: {}", total_requests);
    println!("Total successful: {}", total_successful);
    println!("Total failed: {}", total_failed);
    println!("Overall success rate: {:.2}%", overall_success_rate);
    println!("Average TPS: {:.2}", avg_tps);
    println!("Average success rate: {:.2}%", avg_success_rate);
}

async fn generate_load_test_report(results: &[(String, LoadTestSummary)]) -> anyhow::Result<()> {
    let report = json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "test_suite": "FinDAG Load Test",
        "version": env!("CARGO_PKG_VERSION"),
        "results": results.iter().map(|(test_name, result)| json!({
            "test_name": test_name,
            "total_requests": result.total_requests,
            "successful_requests": result.successful_requests,
            "failed_requests": result.failed_requests,
            "avg_response_time_ms": result.avg_response_time_ms,
            "min_response_time_ms": result.min_response_time_ms,
            "max_response_time_ms": result.max_response_time_ms,
            "throughput_tps": result.throughput_tps,
            "success_rate_percent": result.success_rate_percent,
            "duration_seconds": result.duration_seconds
        })).collect::<Vec<_>>()
    });

    let report_json = serde_json::to_string_pretty(&report)?;
    std::fs::write("load_test_report.json", report_json)?;
    println!("📄 Load test report saved to: load_test_report.json");

    Ok(())
} 