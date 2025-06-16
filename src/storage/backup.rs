use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use std::sync::Arc;
use crate::storage::Storage;
use crate::types::block::Block;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackupConfig {
    pub backup_dir: String,
    pub max_backups: usize,
    pub backup_interval: u64, // in seconds
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BackupMetadata {
    pub timestamp: u64,
    pub block_height: u64,
    pub last_block_hash: String,
    pub size_bytes: u64,
    pub checksum: String,
    pub is_incremental: bool,
    pub base_backup: Option<String>,
}

pub struct BackupManager {
    pub config: BackupConfig,
    storage: Arc<Storage>,
    last_backup: RwLock<Option<BackupMetadata>>,
}

impl BackupManager {
    pub fn new(config: BackupConfig, storage: Arc<Storage>) -> Self {
        Self {
            config,
            storage,
            last_backup: RwLock::new(None),
        }
    }

    pub async fn create_backup(&self) -> Result<BackupMetadata, String> {
        let backup_path = self.get_backup_path()?;
        let blocks = self.storage.get_blocks().map_err(|e| e.to_string())?;
        
        // Create backup metadata
        let metadata = BackupMetadata {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            block_height: blocks.len() as u64,
            last_block_hash: hex::encode(&blocks.last().unwrap().hash),
            size_bytes: 0, // Will be updated after backup
            checksum: String::new(), // Will be updated after backup
            is_incremental: false,
            base_backup: None,
        };

        // Perform backup
        self.perform_backup(&backup_path, &blocks, &metadata).await?;
        
        // Update last backup info
        let mut last_backup = self.last_backup.write().await;
        *last_backup = Some(metadata.clone());

        // Cleanup old backups
        self.cleanup_old_backups().await?;

        Ok(metadata)
    }

    async fn perform_backup(
        &self,
        path: &Path,
        blocks: &[Block],
        metadata: &BackupMetadata,
    ) -> Result<(), String> {
        // Create backup directory if it doesn't exist
        fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;

        // Serialize and compress data
        let data = bincode::serialize(blocks).map_err(|e| e.to_string())?;
        let compressed = if self.config.compression_enabled {
            self.compress_data(&data)?
        } else {
            data
        };

        // Encrypt if enabled
        let final_data = if self.config.encryption_enabled {
            self.encrypt_data(&compressed)?
        } else {
            compressed
        };

        // Write backup file
        fs::write(path, &final_data).map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn cleanup_old_backups(&self) -> Result<(), String> {
        let mut backups = self.list_backups()?;
        if backups.len() > self.config.max_backups {
            backups.sort_by_key(|b| b.timestamp);
            let to_delete = backups.len() - self.config.max_backups;
            
            for backup in backups.iter().take(to_delete) {
                let path = Path::new(&self.config.backup_dir)
                    .join(format!("backup_{}.dat", backup.timestamp));
                fs::remove_file(path).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }

    fn get_backup_path(&self) -> Result<std::path::PathBuf, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Ok(Path::new(&self.config.backup_dir)
            .join(format!("backup_{}.dat", timestamp)))
    }

    pub fn list_backups(&self) -> Result<Vec<BackupMetadata>, String> {
        let mut backups = Vec::new();
        for entry in fs::read_dir(&self.config.backup_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            if entry.path().extension().and_then(|s| s.to_str()) == Some("dat") {
                if let Ok(metadata) = self.read_backup_metadata(&entry.path()) {
                    backups.push(metadata);
                }
            }
        }
        Ok(backups)
    }

    fn read_backup_metadata(&self, path: &Path) -> Result<BackupMetadata, String> {
        let data = fs::read(path).map_err(|e| e.to_string())?;
        bincode::deserialize(&data).map_err(|e| e.to_string())
    }

    fn compress_data(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // Implement compression (e.g., using zstd or lz4)
        Ok(data.to_vec())
    }

    fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // Implement encryption
        Ok(data.to_vec())
    }
} 