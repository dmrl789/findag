use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: u64,
    pub memory_available_mb: u64,
    pub disk_io_read_mb: u64,
    pub disk_io_write_mb: u64,
    pub network_rx_mb: u64,
    pub network_tx_mb: u64,
    pub thread_count: u32,
    pub open_file_descriptors: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceProfile {
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub duration: Duration,
    pub metrics: Vec<SystemMetrics>,
    pub summary: ProfileSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSummary {
    pub avg_cpu_usage: f64,
    pub max_cpu_usage: f64,
    pub avg_memory_usage: f64,
    pub max_memory_usage: f64,
    pub total_disk_read: u64,
    pub total_disk_write: u64,
    pub total_network_rx: u64,
    pub total_network_tx: u64,
    pub peak_thread_count: u32,
    pub peak_file_descriptors: u32,
    pub memory_leak_detected: bool,
    pub cpu_bottleneck_detected: bool,
    pub io_bottleneck_detected: bool,
}

#[derive(Debug)]
pub struct PerformanceProfiler {
    metrics: Arc<RwLock<Vec<SystemMetrics>>>,
    sampling_interval: Duration,
    is_running: Arc<RwLock<bool>>,
}

impl PerformanceProfiler {
    pub fn new(sampling_interval: Duration) -> Self {
        Self {
            metrics: Arc::new(RwLock::new(Vec::new())),
            sampling_interval,
            is_running: Arc::new(RwLock::new(false)),
        }
    }
    
    pub async fn start_profiling(&self) {
        let mut is_running = self.is_running.write().await;
        *is_running = true;
        drop(is_running);
        
        let metrics = self.metrics.clone();
        let sampling_interval = self.sampling_interval;
        let is_running = self.is_running.clone();
        
        tokio::spawn(async move {
            while *is_running.read().await {
                let metric = Self::collect_system_metrics().await;
                metrics.write().await.push(metric);
                tokio::time::sleep(sampling_interval).await;
            }
        });
    }
    
    pub async fn stop_profiling(&self) -> PerformanceProfile {
        let mut is_running = self.is_running.write().await;
        *is_running = false;
        drop(is_running);
        
        // Wait a bit for the profiling task to finish
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        let metrics = self.metrics.read().await.clone();
        let start_time = metrics.first().map(|m| m.timestamp).unwrap_or_else(chrono::Utc::now);
        let end_time = metrics.last().map(|m| m.timestamp).unwrap_or_else(chrono::Utc::now);
        let duration = end_time.signed_duration_since(start_time).to_std().unwrap_or(Duration::from_secs(0));
        
        let summary = Self::calculate_summary(&metrics);
        
        PerformanceProfile {
            start_time,
            end_time,
            duration,
            metrics,
            summary,
        }
    }
    
    async fn collect_system_metrics() -> SystemMetrics {
        let timestamp = chrono::Utc::now();
        
        // Collect CPU usage
        let cpu_usage = Self::get_cpu_usage().await;
        
        // Collect memory usage
        let (memory_usage, memory_available) = Self::get_memory_usage().await;
        
        // Collect disk I/O
        let (disk_read, disk_write) = Self::get_disk_io().await;
        
        // Collect network I/O
        let (network_rx, network_tx) = Self::get_network_io().await;
        
        // Collect process info
        let (thread_count, open_files) = Self::get_process_info().await;
        
        SystemMetrics {
            timestamp,
            cpu_usage_percent: cpu_usage,
            memory_usage_mb: memory_usage,
            memory_available_mb: memory_available,
            disk_io_read_mb: disk_read,
            disk_io_write_mb: disk_write,
            network_rx_mb: network_rx,
            network_tx_mb: network_tx,
            thread_count,
            open_file_descriptors: open_files,
        }
    }
    
    async fn get_cpu_usage() -> f64 {
        #[cfg(target_os = "windows")]
        {
            // Windows-specific CPU usage collection
            if let Ok(output) = std::process::Command::new("wmic")
                .args(&["cpu", "get", "loadpercentage", "/value"])
                .output() {
                if let Ok(output_str) = String::from_utf8(output.stdout) {
                    if let Some(line) = output_str.lines().find(|l| l.starts_with("LoadPercentage=")) {
                        if let Some(value) = line.split('=').nth(1) {
                            if let Ok(usage) = value.parse::<f64>() {
                                return usage;
                            }
                        }
                    }
                }
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            // Linux-specific CPU usage collection
            if let Ok(stat) = std::fs::read_to_string("/proc/stat") {
                if let Some(first_line) = stat.lines().next() {
                    let parts: Vec<u64> = first_line
                        .split_whitespace()
                        .skip(1)
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    
                    if parts.len() >= 4 {
                        let total = parts.iter().sum::<u64>();
                        let idle = parts[3];
                        let usage = 100.0 - (idle as f64 / total as f64 * 100.0);
                        return usage;
                    }
                }
            }
        }
        
        0.0 // Default fallback
    }
    
    async fn get_memory_usage() -> (u64, u64) {
        #[cfg(target_os = "windows")]
        {
            if let Ok(output) = std::process::Command::new("wmic")
                .args(&["computersystem", "get", "TotalPhysicalMemory", "/value"])
                .output() {
                if let Ok(output_str) = String::from_utf8(output.stdout) {
                    if let Some(line) = output_str.lines().find(|l| l.starts_with("TotalPhysicalMemory=")) {
                        if let Some(value) = line.split('=').nth(1) {
                            if let Ok(total_mb) = value.parse::<u64>() {
                                return (total_mb / 1024 / 1024, total_mb / 1024 / 1024);
                            }
                        }
                    }
                }
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
                let mut total = 0;
                let mut available = 0;
                
                for line in meminfo.lines() {
                    if line.starts_with("MemTotal:") {
                        if let Some(value) = line.split_whitespace().nth(1) {
                            if let Ok(kb) = value.parse::<u64>() {
                                total = kb / 1024; // Convert KB to MB
                            }
                        }
                    } else if line.starts_with("MemAvailable:") {
                        if let Some(value) = line.split_whitespace().nth(1) {
                            if let Ok(kb) = value.parse::<u64>() {
                                available = kb / 1024; // Convert KB to MB
                            }
                        }
                    }
                }
                
                if total > 0 {
                    return (total - available, available);
                }
            }
        }
        
        (0, 0) // Default fallback
    }
    
    async fn get_disk_io() -> (u64, u64) {
        #[cfg(target_os = "linux")]
        {
            if let Ok(diskstats) = std::fs::read_to_string("/proc/diskstats") {
                let mut total_read = 0;
                let mut total_write = 0;
                
                for line in diskstats.lines() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 14 {
                        if let (Ok(read_sectors), Ok(write_sectors)) = (parts[5].parse::<u64>(), parts[9].parse::<u64>()) {
                            total_read += read_sectors * 512 / 1024 / 1024; // Convert sectors to MB
                            total_write += write_sectors * 512 / 1024 / 1024;
                        }
                    }
                }
                
                return (total_read, total_write);
            }
        }
        
        (0, 0) // Default fallback
    }
    
    async fn get_network_io() -> (u64, u64) {
        #[cfg(target_os = "linux")]
        {
            if let Ok(netdev) = std::fs::read_to_string("/proc/net/dev") {
                let mut total_rx = 0;
                let mut total_tx = 0;
                
                for line in netdev.lines().skip(2) { // Skip header lines
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 10 {
                        if let (Ok(rx_bytes), Ok(tx_bytes)) = (parts[1].parse::<u64>(), parts[9].parse::<u64>()) {
                            total_rx += rx_bytes / 1024 / 1024; // Convert to MB
                            total_tx += tx_bytes / 1024 / 1024;
                        }
                    }
                }
                
                return (total_rx, total_tx);
            }
        }
        
        (0, 0) // Default fallback
    }
    
    async fn get_process_info() -> (u32, u32) {
        #[cfg(target_os = "linux")]
        {
            let pid = std::process::id();
            let mut thread_count = 0;
            let mut open_files = 0;
            
            // Get thread count
            if let Ok(status) = std::fs::read_to_string(format!("/proc/{}/status", pid)) {
                for line in status.lines() {
                    if line.starts_with("Threads:") {
                        if let Some(value) = line.split_whitespace().nth(1) {
                            if let Ok(count) = value.parse::<u32>() {
                                thread_count = count;
                            }
                        }
                    }
                }
            }
            
            // Get open file descriptors count
            if let Ok(entries) = std::fs::read_dir(format!("/proc/{}/fd", pid)) {
                open_files = entries.count() as u32;
            }
            
            return (thread_count, open_files);
        }
        
        (0, 0) // Default fallback
    }
    
    fn calculate_summary(metrics: &[SystemMetrics]) -> ProfileSummary {
        if metrics.is_empty() {
            return ProfileSummary {
                avg_cpu_usage: 0.0,
                max_cpu_usage: 0.0,
                avg_memory_usage: 0.0,
                max_memory_usage: 0.0,
                total_disk_read: 0,
                total_disk_write: 0,
                total_network_rx: 0,
                total_network_tx: 0,
                peak_thread_count: 0,
                peak_file_descriptors: 0,
                memory_leak_detected: false,
                cpu_bottleneck_detected: false,
                io_bottleneck_detected: false,
            };
        }
        
        let avg_cpu_usage = metrics.iter().map(|m| m.cpu_usage_percent).sum::<f64>() / metrics.len() as f64;
        let max_cpu_usage = metrics.iter().map(|m| m.cpu_usage_percent).fold(0.0, f64::max);
        let avg_memory_usage = metrics.iter().map(|m| m.memory_usage_mb as f64).sum::<f64>() / metrics.len() as f64;
        let max_memory_usage = metrics.iter().map(|m| m.memory_usage_mb).max().unwrap_or(0) as f64;
        
        let total_disk_read = metrics.iter().map(|m| m.disk_io_read_mb).sum();
        let total_disk_write = metrics.iter().map(|m| m.disk_io_write_mb).sum();
        let total_network_rx = metrics.iter().map(|m| m.network_rx_mb).sum();
        let total_network_tx = metrics.iter().map(|m| m.network_tx_mb).sum();
        
        let peak_thread_count = metrics.iter().map(|m| m.thread_count).max().unwrap_or(0);
        let peak_file_descriptors = metrics.iter().map(|m| m.open_file_descriptors).max().unwrap_or(0);
        
        // Detect memory leak (monotonic increase)
        let memory_leak_detected = Self::detect_memory_leak(metrics);
        
        // Detect CPU bottleneck
        let cpu_bottleneck_detected = max_cpu_usage > 90.0;
        
        // Detect I/O bottleneck
        let io_bottleneck_detected = total_disk_read > 1000 || total_disk_write > 1000; // MB threshold
        
        ProfileSummary {
            avg_cpu_usage,
            max_cpu_usage,
            avg_memory_usage,
            max_memory_usage,
            total_disk_read,
            total_disk_write,
            total_network_rx,
            total_network_tx,
            peak_thread_count,
            peak_file_descriptors,
            memory_leak_detected,
            cpu_bottleneck_detected,
            io_bottleneck_detected,
        }
    }
    
    fn detect_memory_leak(metrics: &[SystemMetrics]) -> bool {
        if metrics.len() < 10 {
            return false;
        }
        
        // Check if memory usage is consistently increasing
        let memory_values: Vec<f64> = metrics.iter().map(|m| m.memory_usage_mb as f64).collect();
        let mut increasing_count = 0;
        
        for i in 1..memory_values.len() {
            if memory_values[i] > memory_values[i-1] {
                increasing_count += 1;
            }
        }
        
        // If 80% of measurements show increasing memory, consider it a leak
        (increasing_count as f64 / (memory_values.len() - 1) as f64) > 0.8
    }
}

impl PerformanceProfile {
    pub fn print_summary(&self) {
        println!("📊 Performance Profile Summary");
        println!("==============================");
        println!("Duration: {:.2}s", self.duration.as_secs_f64());
        println!("Samples collected: {}", self.metrics.len());
        println!();
        println!("CPU Usage:");
        println!("  Average: {:.2}%", self.summary.avg_cpu_usage);
        println!("  Maximum: {:.2}%", self.summary.max_cpu_usage);
        println!();
        println!("Memory Usage:");
        println!("  Average: {:.2} MB", self.summary.avg_memory_usage);
        println!("  Maximum: {:.2} MB", self.summary.max_memory_usage);
        println!();
        println!("Disk I/O:");
        println!("  Total Read: {} MB", self.summary.total_disk_read);
        println!("  Total Write: {} MB", self.summary.total_disk_write);
        println!();
        println!("Network I/O:");
        println!("  Total RX: {} MB", self.summary.total_network_rx);
        println!("  Total TX: {} MB", self.summary.total_network_tx);
        println!();
        println!("Process Info:");
        println!("  Peak Thread Count: {}", self.summary.peak_thread_count);
        println!("  Peak File Descriptors: {}", self.summary.peak_file_descriptors);
        println!();
        println!("Issues Detected:");
        println!("  Memory Leak: {}", if self.summary.memory_leak_detected { "❌ YES" } else { "✅ NO" });
        println!("  CPU Bottleneck: {}", if self.summary.cpu_bottleneck_detected { "❌ YES" } else { "✅ NO" });
        println!("  I/O Bottleneck: {}", if self.summary.io_bottleneck_detected { "❌ YES" } else { "✅ NO" });
        println!();
    }
    
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
} 