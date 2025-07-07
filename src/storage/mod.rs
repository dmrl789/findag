pub mod persistent;
pub mod state;
pub mod config;
pub mod db_monitor;

pub use persistent::PersistentStorage;
pub use config::DatabaseConfig;
pub use db_monitor::{DatabaseMonitor, DatabaseAnalyzer, DatabaseMetrics, DatabaseHealthStatus};

use std::sync::Arc;
use tokio::time::Duration;

/// Initialize storage with optimized configuration
pub async fn initialize_storage(
    _db_path: &str,
    config_profile: &str,
) -> Result<(Arc<PersistentStorage>, Arc<DatabaseMonitor>), Box<dyn std::error::Error + Send + Sync>> {
    // Load database configuration
    let config = if let Ok(config) = DatabaseConfig::load_from_file("configs/database.toml", config_profile) {
        tracing::info!("Loaded database config from file: {}", config.summary());
        config
    } else {
        let config = DatabaseConfig::auto_detect();
        tracing::info!("Using auto-detected database config: {}", config.summary());
        config
    };

    // Validate configuration
    if let Err(errors) = config.validate() {
        tracing::warn!("Database configuration validation warnings: {:?}", errors);
    }

    // Create optimized storage
    let storage = Arc::new(PersistentStorage::new(_db_path)?);

    // Create database monitor
    let monitor = Arc::new(DatabaseMonitor::new(
        storage.clone(),
        Duration::from_secs(config.maintenance_interval_seconds),
        Duration::from_secs(config.health_check_interval_seconds),
    ));

    // Start monitoring in background
    let monitor_clone = monitor.clone();
    tokio::spawn(async move {
        monitor_clone.start_monitoring().await;
    });

    tracing::info!("Storage initialized with profile: {}", config_profile);
    Ok((storage, monitor))
}

/// Initialize storage with high-frequency optimization
pub async fn initialize_high_frequency_storage(
    db_path: &str,
) -> Result<(Arc<PersistentStorage>, Arc<DatabaseMonitor>), Box<dyn std::error::Error + Send + Sync>> {
    initialize_storage(db_path, "high_frequency").await
}

/// Initialize storage with production optimization
pub async fn initialize_production_storage(
    db_path: &str,
) -> Result<(Arc<PersistentStorage>, Arc<DatabaseMonitor>), Box<dyn std::error::Error + Send + Sync>> {
    initialize_storage(db_path, "production").await
}

/// Initialize storage with development optimization
pub async fn initialize_development_storage(
    db_path: &str,
) -> Result<(Arc<PersistentStorage>, Arc<DatabaseMonitor>), Box<dyn std::error::Error + Send + Sync>> {
    initialize_storage(db_path, "development").await
}

/// Get storage health status
pub async fn get_storage_health(
    monitor: &Arc<DatabaseMonitor>,
) -> Result<DatabaseHealthStatus, Box<dyn std::error::Error + Send + Sync>> {
    Ok(monitor.get_health_status().await)
}

/// Get storage metrics
pub fn get_storage_metrics(
    monitor: &Arc<DatabaseMonitor>,
) -> Result<DatabaseMetrics, Box<dyn std::error::Error + Send + Sync>> {
    monitor.get_metrics().map_err(|e| Box::<dyn std::error::Error + Send + Sync>::from(e))
}

/// Perform storage maintenance
pub async fn perform_storage_maintenance(
    storage: &Arc<PersistentStorage>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    storage.optimize()?;
    tracing::info!("Storage maintenance completed");
    Ok(())
}

/// Create storage backup
pub async fn create_storage_backup(
    storage: &Arc<PersistentStorage>,
    backup_path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    storage.create_backup(backup_path)?;
    tracing::info!("Storage backup created at: {}", backup_path);
    Ok(())
} 