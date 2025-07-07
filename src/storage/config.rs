use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub cache_capacity_mb: u64,
    pub use_compression: bool,
    pub compression_factor: u32,
    pub segment_size_mb: u64,
    pub flush_interval_ms: u64,
    pub background_threads: u32,
    pub io_buffer_size_kb: u64,
    pub memory_budget_mb: u64,
    pub prefetch_enabled: bool,
    pub snapshot_interval: u64,
    pub maintenance_interval_seconds: u64,
    pub health_check_interval_seconds: u64,
    pub backup_enabled: bool,
    pub backup_retention_days: u32,
    pub backup_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub enabled: bool,
    pub metrics_export_interval_seconds: u64,
    pub prometheus_endpoint: String,
    pub grafana_dashboard_enabled: bool,
    pub alerts: AlertThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub cache_hit_rate_threshold: f64,
    pub error_rate_threshold: f64,
    pub disk_usage_threshold: f64,
    pub response_time_threshold_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    pub s3_enabled: bool,
    pub s3_bucket: String,
    pub s3_region: String,
    pub s3_prefix: String,
    pub local_enabled: bool,
    pub local_path: String,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
    pub schedule: String,
    pub retention_days: u32,
    pub max_backup_size_gb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTuningConfig {
    pub auto_tune_enabled: bool,
    pub tune_interval_hours: u64,
    pub cache_tuning_enabled: bool,
    pub min_cache_size_mb: u64,
    pub max_cache_size_mb: u64,
    pub io_tuning_enabled: bool,
    pub preferred_io_size_kb: u64,
    pub max_io_requests: u64,
    pub memory_tuning_enabled: bool,
    pub target_memory_usage_percent: u64,
    pub max_memory_usage_percent: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteDatabaseConfig {
    pub default: DatabaseConfig,
    pub development: DatabaseConfig,
    pub production: DatabaseConfig,
    pub high_frequency: DatabaseConfig,
    pub storage_efficient: DatabaseConfig,
    pub monitoring: MonitoringConfig,
    pub backup: BackupConfig,
    pub performance_tuning: PerformanceTuningConfig,
}

impl DatabaseConfig {
    /// Load configuration from a TOML file
    pub fn load_from_file<P: AsRef<Path>>(path: P, profile: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: CompleteDatabaseConfig = toml::from_str(&content)?;
        
        match profile {
            "development" => Ok(config.development),
            "production" => Ok(config.production),
            "high_frequency" => Ok(config.high_frequency),
            "storage_efficient" => Ok(config.storage_efficient),
            _ => Ok(config.default),
        }
    }

    /// Load configuration from environment variables
    pub fn load_from_env() -> Self {
        Self {
            cache_capacity_mb: std::env::var("FINDAG_DB_CACHE_CAPACITY_MB")
                .unwrap_or_else(|_| "1024".to_string())
                .parse()
                .unwrap_or(1024),
            use_compression: std::env::var("FINDAG_DB_USE_COMPRESSION")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            compression_factor: std::env::var("FINDAG_DB_COMPRESSION_FACTOR")
                .unwrap_or_else(|_| "2".to_string())
                .parse()
                .unwrap_or(2),
            segment_size_mb: std::env::var("FINDAG_DB_SEGMENT_SIZE_MB")
                .unwrap_or_else(|_| "64".to_string())
                .parse()
                .unwrap_or(64),
            flush_interval_ms: std::env::var("FINDAG_DB_FLUSH_INTERVAL_MS")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .unwrap_or(100),
            background_threads: std::env::var("FINDAG_DB_BACKGROUND_THREADS")
                .unwrap_or_else(|_| "4".to_string())
                .parse()
                .unwrap_or(4),
            io_buffer_size_kb: std::env::var("FINDAG_DB_IO_BUFFER_SIZE_KB")
                .unwrap_or_else(|_| "64".to_string())
                .parse()
                .unwrap_or(64),
            memory_budget_mb: std::env::var("FINDAG_DB_MEMORY_BUDGET_MB")
                .unwrap_or_else(|_| "2048".to_string())
                .parse()
                .unwrap_or(2048),
            prefetch_enabled: std::env::var("FINDAG_DB_PREFETCH_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            snapshot_interval: std::env::var("FINDAG_DB_SNAPSHOT_INTERVAL")
                .unwrap_or_else(|_| "10000".to_string())
                .parse()
                .unwrap_or(10000),
            maintenance_interval_seconds: std::env::var("FINDAG_DB_MAINTENANCE_INTERVAL_SECONDS")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .unwrap_or(3600),
            health_check_interval_seconds: std::env::var("FINDAG_DB_HEALTH_CHECK_INTERVAL_SECONDS")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .unwrap_or(300),
            backup_enabled: std::env::var("FINDAG_DB_BACKUP_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            backup_retention_days: std::env::var("FINDAG_DB_BACKUP_RETENTION_DAYS")
                .unwrap_or_else(|_| "7".to_string())
                .parse()
                .unwrap_or(7),
            backup_path: std::env::var("FINDAG_DB_BACKUP_PATH").ok(),
        }
    }

    /// Convert configuration to Sled Config
    pub fn to_sled_config(&self) -> sled::Config {
        sled::Config::default()
            .cache_capacity(self.cache_capacity_mb * 1024 * 1024)
            .use_compression(self.use_compression)
            .compression_factor(self.compression_factor.try_into().unwrap())
            .segment_size((self.segment_size_mb * 1024 * 1024).try_into().unwrap())
            .flush_every_ms(Some(self.flush_interval_ms))
    }

    /// Get recommended configuration based on system resources
    pub fn auto_detect() -> Self {
        let total_memory = Self::get_total_memory_mb();
        let cpu_cores = num_cpus::get() as u32;
        
        // Auto-detect based on available resources
        let cache_capacity_mb = (total_memory / 4).min(4096).max(256);
        let memory_budget_mb = (total_memory / 2).min(8192).max(1024);
        let background_threads = cpu_cores.min(16).max(2);
        
        Self {
            cache_capacity_mb,
            use_compression: total_memory < 4096, // Use compression if memory is limited
            compression_factor: if total_memory < 2048 { 3 } else { 2 },
            segment_size_mb: if total_memory > 8192 { 128 } else { 64 },
            flush_interval_ms: if total_memory > 4096 { 50 } else { 100 },
            background_threads,
            io_buffer_size_kb: if total_memory > 4096 { 128 } else { 64 },
            memory_budget_mb,
            prefetch_enabled: total_memory > 2048,
            snapshot_interval: if total_memory > 4096 { 5000 } else { 10000 },
            maintenance_interval_seconds: 3600,
            health_check_interval_seconds: 300,
            backup_enabled: true,
            backup_retention_days: 7,
            backup_path: None,
        }
    }

    /// Get total system memory in MB
    fn get_total_memory_mb() -> u64 {
        #[cfg(target_os = "linux")]
        {
            if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
                for line in meminfo.lines() {
                    if line.starts_with("MemTotal:") {
                        if let Some(kb_str) = line.split_whitespace().nth(1) {
                            if let Ok(kb) = kb_str.parse::<u64>() {
                                return kb / 1024; // Convert KB to MB
                            }
                        }
                    }
                }
            }
        }
        
        #[cfg(target_os = "windows")]
        {
            // Windows memory detection would go here
            // For now, return a reasonable default
        }
        
        #[cfg(target_os = "macos")]
        {
            // macOS memory detection would go here
            // For now, return a reasonable default
        }
        
        4096 // Default to 4GB if we can't detect
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        if self.cache_capacity_mb == 0 {
            errors.push("Cache capacity must be greater than 0".to_string());
        }
        
        if self.segment_size_mb == 0 {
            errors.push("Segment size must be greater than 0".to_string());
        }
        
        if self.flush_interval_ms == 0 {
            errors.push("Flush interval must be greater than 0".to_string());
        }
        
        if self.background_threads == 0 {
            errors.push("Background threads must be greater than 0".to_string());
        }
        
        if self.memory_budget_mb < self.cache_capacity_mb {
            errors.push("Memory budget must be greater than or equal to cache capacity".to_string());
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Get configuration summary for logging
    pub fn summary(&self) -> String {
        format!(
            "DB Config: cache={}MB, compression={}, segments={}MB, flush={}ms, threads={}, memory={}MB",
            self.cache_capacity_mb,
            self.use_compression,
            self.segment_size_mb,
            self.flush_interval_ms,
            self.background_threads,
            self.memory_budget_mb
        )
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            cache_capacity_mb: 1024,
            use_compression: true,
            compression_factor: 2,
            segment_size_mb: 64,
            flush_interval_ms: 100,
            background_threads: 4,
            io_buffer_size_kb: 64,
            memory_budget_mb: 2048,
            prefetch_enabled: true,
            snapshot_interval: 10000,
            maintenance_interval_seconds: 3600,
            health_check_interval_seconds: 300,
            backup_enabled: true,
            backup_retention_days: 7,
            backup_path: None,
        }
    }
} 