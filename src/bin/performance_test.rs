use findag::storage::{initialize_production_storage, get_storage_metrics, perform_storage_maintenance};
use findag::core::types::{Block, Round, SerializableBlock, SerializableRound};
use std::sync::Arc;
use std::time::Instant;
use serde_json::json;
use findag::core::address::Address;
use ed25519_dalek::{Signature, VerifyingKey};
use findag::core::types::ShardId;

#[derive(Debug)]
struct PerformanceMetrics {
    operation: String,
    count: u64,
    total_time_ms: u64,
    avg_time_ms: f64,
    min_time_ms: u64,
    max_time_ms: u64,
    throughput_ops_per_sec: f64,
    errors: u64,
}

impl PerformanceMetrics {
    fn new(operation: String) -> Self {
        Self {
            operation,
            count: 0,
            total_time_ms: 0,
            avg_time_ms: 0.0,
            min_time_ms: u64::MAX,
            max_time_ms: 0,
            throughput_ops_per_sec: 0.0,
            errors: 0,
        }
    }

    fn add_measurement(&mut self, duration_ms: u64, success: bool) {
        self.count += 1;
        self.total_time_ms += duration_ms;
        self.avg_time_ms = self.total_time_ms as f64 / self.count as f64;
        self.min_time_ms = self.min_time_ms.min(duration_ms);
        self.max_time_ms = self.max_time_ms.max(duration_ms);
        
        if !success {
            self.errors += 1;
        }
    }

    fn calculate_throughput(&mut self, total_duration_sec: f64) {
        self.throughput_ops_per_sec = self.count as f64 / total_duration_sec;
    }

    fn print_summary(&self) {
        let error_rate = if self.count > 0 {
            (self.errors as f64 / self.count as f64) * 100.0
        } else {
            0.0
        };

        println!("=== {} Performance Summary ===", self.operation);
        println!("Total operations: {}", self.count);
        println!("Total time: {}ms", self.total_time_ms);
        println!("Average time: {:.2}ms", self.avg_time_ms);
        println!("Min time: {}ms", self.min_time_ms);
        println!("Max time: {}ms", self.max_time_ms);
        println!("Throughput: {:.2} ops/sec", self.throughput_ops_per_sec);
        println!("Error rate: {:.2}%", error_rate);
        println!("Success rate: {:.2}%", 100.0 - error_rate);
        println!();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("🚀 Starting FinDAG Performance Test Suite");
    println!("==========================================");

    // Initialize storage with production configuration
    println!("📊 Initializing storage...");
    let (storage, monitor) = initialize_production_storage("performance_test_db").await?;
    println!("✅ Storage initialized successfully");

    // Run different performance tests
    let mut all_metrics = Vec::new();

    // Test 1: Database Write Performance
    println!("\n📝 Testing Database Write Performance...");
    let write_metrics = test_database_writes(&storage, 10000).await;
    write_metrics.print_summary();
    all_metrics.push(write_metrics);

    // Test 2: Database Read Performance
    println!("\n📖 Testing Database Read Performance...");
    let read_metrics = test_database_reads(&storage, 10000).await;
    read_metrics.print_summary();
    all_metrics.push(read_metrics);

    // Test 3: Block Serialization Performance
    println!("\n🔧 Testing Block Serialization Performance...");
    let serialization_metrics = test_block_serialization(10000).await;
    serialization_metrics.print_summary();
    all_metrics.push(serialization_metrics);

    // Test 4: Round Processing Performance
    println!("\n⏰ Testing Round Processing Performance...");
    let round_metrics = test_round_processing(1000).await;
    round_metrics.print_summary();
    all_metrics.push(round_metrics);

    // Test 5: Concurrent Operations Performance
    println!("\n🔄 Testing Concurrent Operations Performance...");
    let concurrent_metrics = test_concurrent_operations(&storage, 1000, 10).await;
    concurrent_metrics.print_summary();
    all_metrics.push(concurrent_metrics);

    // Test 6: Memory Usage Performance
    println!("\n💾 Testing Memory Usage Performance...");
    let memory_metrics = test_memory_usage(&storage, 10000).await;
    memory_metrics.print_summary();
    all_metrics.push(memory_metrics);

    // Test 7: Stress Test
    println!("\n🔥 Running Stress Test...");
    let stress_metrics = stress_test(&storage, 50000).await;
    stress_metrics.print_summary();
    all_metrics.push(stress_metrics);

    // Get database metrics
    println!("\n📊 Database Metrics:");
    if let Ok(metrics) = get_storage_metrics(&monitor) {
        println!("Size on disk: {} bytes", metrics.size_on_disk);
        println!("Tree count: {}", metrics.tree_count);
        println!("Write operations: {}", metrics.write_operations);
        println!("Read operations: {}", metrics.read_operations);
        println!("Flush count: {}", metrics.flush_count);
        println!("Error count: {}", metrics.error_count);
        println!("Uptime: {} seconds", metrics.uptime_seconds);
    }

    // Perform maintenance
    println!("\n🔧 Performing database maintenance...");
    perform_storage_maintenance(&storage).await?;
    println!("✅ Maintenance completed");

    // Print overall summary
    print_overall_summary(&all_metrics);

    // Generate performance report
    generate_performance_report(&all_metrics).await?;

    println!("\n✅ Performance test suite completed successfully!");
    Ok(())
}

async fn test_database_writes(storage: &Arc<findag::storage::PersistentStorage>, count: u64) -> PerformanceMetrics {
    let mut metrics = PerformanceMetrics::new("Database Writes".to_string());
    let start_time = Instant::now();

    for i in 0..count {
        let key = format!("test_key_{}", i);
        let value = format!("test_value_{}_with_some_data_to_make_it_longer_{}", i, i * 2);
        
        let write_start = Instant::now();
        let result = storage.store_parameter(&key, &value);
        let duration = write_start.elapsed().as_millis() as u64;
        
        metrics.add_measurement(duration, result.is_ok());
        
        if i % 1000 == 0 {
            println!("  Processed {} writes...", i);
        }
    }

    let total_duration = start_time.elapsed().as_secs_f64();
    metrics.calculate_throughput(total_duration);
    metrics
}

async fn test_database_reads(storage: &Arc<findag::storage::PersistentStorage>, count: u64) -> PerformanceMetrics {
    let mut metrics = PerformanceMetrics::new("Database Reads".to_string());
    let start_time = Instant::now();

    for i in 0..count {
        let key = format!("test_key_{}", i % 1000); // Read from existing keys
        
        let read_start = Instant::now();
        let result = storage.load_parameter(&key);
        let duration = read_start.elapsed().as_millis() as u64;
        
        metrics.add_measurement(duration, result.is_ok());
        
        if i % 1000 == 0 {
            println!("  Processed {} reads...", i);
        }
    }

    let total_duration = start_time.elapsed().as_secs_f64();
    metrics.calculate_throughput(total_duration);
    metrics
}

async fn test_block_serialization(count: u64) -> PerformanceMetrics {
    let mut metrics = PerformanceMetrics::new("Block Serialization".to_string());
    let start_time = Instant::now();

    for i in 0..count {
        // Create a test block
        let block = Block {
            block_id: [i as u8; 32],
            parent_blocks: vec![],
            transactions: vec![],
            findag_time: i,
            hashtimer: [i as u8; 32],
            proposer: Address::random(),
            signature: Signature::from_bytes(&[0u8; 64]),
            public_key: VerifyingKey::from_bytes(&[0u8; 32]).unwrap(),
            shard_id: ShardId(0),
            merkle_root: None,
        };

        let serializable = SerializableBlock::from(block);
        
        let serialize_start = Instant::now();
        let result = bincode::serialize(&serializable);
        let duration = serialize_start.elapsed().as_millis() as u64;
        
        metrics.add_measurement(duration, result.is_ok());
        
        if i % 1000 == 0 {
            println!("  Processed {} serializations...", i);
        }
    }

    let total_duration = start_time.elapsed().as_secs_f64();
    metrics.calculate_throughput(total_duration);
    metrics
}

async fn test_round_processing(count: u64) -> PerformanceMetrics {
    let mut metrics = PerformanceMetrics::new("Round Processing".to_string());
    let start_time = Instant::now();

    for i in 0..count {
        let round = Round {
            round_number: i,
            parent_round_hash: [i as u8; 32],
            finalized_block_hashes: vec![],
            block_hashtimers: vec![],
            quorum_signature: vec![],
            findag_time: i,
            proposer: Address::random(),
            proposer_signature: Signature::from_bytes(&[0u8; 64]),
            proposer_public_key: VerifyingKey::from_bytes(&[0u8; 32]).unwrap(),
        };

        let serializable = SerializableRound::from(round);
        
        let process_start = Instant::now();
        let result = bincode::serialize(&serializable);
        let duration = process_start.elapsed().as_millis() as u64;
        
        metrics.add_measurement(duration, result.is_ok());
        
        if i % 100 == 0 {
            println!("  Processed {} rounds...", i);
        }
    }

    let total_duration = start_time.elapsed().as_secs_f64();
    metrics.calculate_throughput(total_duration);
    metrics
}

async fn test_concurrent_operations(
    storage: &Arc<findag::storage::PersistentStorage>, 
    operations_per_thread: u64, 
    num_threads: u64
) -> PerformanceMetrics {
    let mut metrics = PerformanceMetrics::new("Concurrent Operations".to_string());
    let start_time = Instant::now();

    let mut handles = vec![];
    let storage_clone = storage.clone();

    for thread_id in 0..num_threads {
        let storage_thread = storage_clone.clone();
        let handle = tokio::spawn(async move {
            let mut thread_metrics = PerformanceMetrics::new(format!("Thread {}", thread_id));
            
            for i in 0..operations_per_thread {
                let key = format!("concurrent_key_{}_{}", thread_id, i);
                let value = format!("concurrent_value_{}_{}", thread_id, i);
                
                let op_start = Instant::now();
                let result = storage_thread.store_parameter(&key, &value);
                let duration = op_start.elapsed().as_millis() as u64;
                
                thread_metrics.add_measurement(duration, result.is_ok());
            }
            
            thread_metrics
        });
        handles.push(handle);
    }

    // Collect results from all threads
    for handle in handles {
        if let Ok(thread_metrics) = handle.await {
            metrics.count += thread_metrics.count;
            metrics.total_time_ms += thread_metrics.total_time_ms;
            metrics.errors += thread_metrics.errors;
        }
    }

    let total_duration = start_time.elapsed().as_secs_f64();
    metrics.calculate_throughput(total_duration);
    metrics.avg_time_ms = if metrics.count > 0 {
        metrics.total_time_ms as f64 / metrics.count as f64
    } else {
        0.0
    };
    metrics
}

async fn test_memory_usage(storage: &Arc<findag::storage::PersistentStorage>, count: u64) -> PerformanceMetrics {
    let mut metrics = PerformanceMetrics::new("Memory Usage".to_string());
    let start_time = Instant::now();

    // Monitor memory usage during operations
    let initial_memory = get_memory_usage();
    
    for i in 0..count {
        let key = format!("memory_test_key_{}", i);
        let value = format!("memory_test_value_{}_with_large_data_to_test_memory_usage_{}", i, i * 3);
        
        let mem_start = Instant::now();
        let result = storage.store_parameter(&key, &value);
        let duration = mem_start.elapsed().as_millis() as u64;
        
        metrics.add_measurement(duration, result.is_ok());
        
        if i % 1000 == 0 {
            let current_memory = get_memory_usage();
            let memory_increase = current_memory - initial_memory;
            println!("  Processed {} operations, memory increase: {} KB", i, memory_increase / 1024);
        }
    }

    let total_duration = start_time.elapsed().as_secs_f64();
    metrics.calculate_throughput(total_duration);
    metrics
}

async fn stress_test(storage: &Arc<findag::storage::PersistentStorage>, count: u64) -> PerformanceMetrics {
    let mut metrics = PerformanceMetrics::new("Stress Test".to_string());
    let start_time = Instant::now();

    // Mix of different operations under stress
    for i in 0..count {
        let operation_type = i % 4;
        
        let stress_start = Instant::now();
        let success = match operation_type {
            0 => {
                // Write operation
                let key = format!("stress_write_{}", i);
                let value = format!("stress_value_{}", i);
                storage.store_parameter(&key, &value).is_ok()
            },
            1 => {
                // Read operation
                let key = format!("stress_read_{}", i % 1000);
                storage.load_parameter(&key).is_ok()
            },
            2 => {
                // Flush operation (every 100 operations)
                if i % 100 == 0 {
                    storage.flush().is_ok()
                } else {
                    true
                }
            },
            _ => {
                // Mixed operation
                let key = format!("stress_mixed_{}", i);
                let value = format!("stress_mixed_value_{}", i);
                let write_success = storage.store_parameter(&key, &value).is_ok();
                let read_success = storage.load_parameter(&key).is_ok();
                write_success && read_success
            }
        };
        
        let duration = stress_start.elapsed().as_millis() as u64;
        metrics.add_measurement(duration, success);
        
        if i % 5000 == 0 {
            println!("  Processed {} stress operations...", i);
        }
    }

    let total_duration = start_time.elapsed().as_secs_f64();
    metrics.calculate_throughput(total_duration);
    metrics
}

fn get_memory_usage() -> u64 {
    #[cfg(target_os = "linux")]
    {
        if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("VmRSS:") {
                    if let Some(kb_str) = line.split_whitespace().nth(1) {
                        if let Ok(kb) = kb_str.parse::<u64>() {
                            return kb * 1024; // Convert KB to bytes
                        }
                    }
                }
            }
        }
    }
    
    0 // Default if we can't get memory info
}

fn print_overall_summary(metrics: &[PerformanceMetrics]) {
    println!("\n📊 OVERALL PERFORMANCE SUMMARY");
    println!("================================");
    
    let mut total_operations = 0;
    let mut total_errors = 0;
    let mut avg_throughput = 0.0;
    let mut count = 0;

    for metric in metrics {
        total_operations += metric.count;
        total_errors += metric.errors;
        avg_throughput += metric.throughput_ops_per_sec;
        count += 1;
    }

    if count > 0 {
        avg_throughput /= count as f64;
    }

    let overall_error_rate = if total_operations > 0 {
        (total_errors as f64 / total_operations as f64) * 100.0
    } else {
        0.0
    };

    println!("Total operations: {}", total_operations);
    println!("Total errors: {}", total_errors);
    println!("Overall error rate: {:.2}%", overall_error_rate);
    println!("Average throughput: {:.2} ops/sec", avg_throughput);
    println!("Success rate: {:.2}%", 100.0 - overall_error_rate);
}

async fn generate_performance_report(metrics: &[PerformanceMetrics]) -> anyhow::Result<()> {
    let report = json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "test_suite": "FinDAG Performance Test",
        "version": env!("CARGO_PKG_VERSION"),
        "metrics": metrics.iter().map(|m| json!({
            "operation": m.operation,
            "count": m.count,
            "total_time_ms": m.total_time_ms,
            "avg_time_ms": m.avg_time_ms,
            "min_time_ms": m.min_time_ms,
            "max_time_ms": m.max_time_ms,
            "throughput_ops_per_sec": m.throughput_ops_per_sec,
            "errors": m.errors,
            "error_rate_percent": if m.count > 0 { (m.errors as f64 / m.count as f64) * 100.0 } else { 0.0 }
        })).collect::<Vec<_>>()
    });

    let report_json = serde_json::to_string_pretty(&report)?;
    std::fs::write("performance_report.json", report_json)?;
    println!("📄 Performance report saved to: performance_report.json");

    Ok(())
} 