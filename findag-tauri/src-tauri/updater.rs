use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime, Window};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub version: String,
    pub date: String,
    pub notes: String,
    pub url: String,
    pub size: u64,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateStatus {
    pub available: bool,
    pub current_version: String,
    pub latest_version: Option<String>,
    pub download_progress: f64,
    pub is_downloading: bool,
    pub is_installing: bool,
    pub error: Option<String>,
}

pub struct Updater<R: Runtime> {
    app: AppHandle<R>,
    update_url: String,
    current_version: String,
}

impl<R: Runtime> Updater<R> {
    pub fn new(app: AppHandle<R>, update_url: String) -> Self {
        let current_version = env!("CARGO_PKG_VERSION").to_string();
        Self {
            app,
            update_url,
            current_version,
        }
    }

    /// Check for available updates
    pub async fn check_for_updates(&self) -> Result<UpdateInfo, String> {
        let client = reqwest::Client::new();
        let response = client
            .get(&format!("{}/latest", self.update_url))
            .header("User-Agent", "FinDAG-Desktop/1.0.0")
            .send()
            .await
            .map_err(|e| format!("Failed to check for updates: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Update server returned error: {}",
                response.status()
            ));
        }

        let update_info: UpdateInfo = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse update info: {}", e))?;

        // Check if the new version is actually newer
        if self.compare_versions(&self.current_version, &update_info.version) >= 0 {
            return Err("No updates available".to_string());
        }

        Ok(update_info)
    }

    /// Download and install update
    pub async fn download_and_install(&self, update_info: &UpdateInfo) -> Result<(), String> {
        let window = self
            .app
            .get_window("main")
            .ok_or("Main window not found")?;

        // Emit download started event
        window
            .emit("update-download-started", ())
            .map_err(|e| format!("Failed to emit event: {}", e))?;

        // Download the update
        let client = reqwest::Client::new();
        let response = client
            .get(&update_info.url)
            .header("User-Agent", "FinDAG-Desktop/1.0.0")
            .send()
            .await
            .map_err(|e| format!("Failed to download update: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Download failed with status: {}",
                response.status()
            ));
        }

        let total_size = response.content_length().unwrap_or(0);
        let mut downloaded: u64 = 0;
        let mut stream = response.bytes_stream();

        // Create temporary file for download
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join(format!("findag-update-{}.tmp", update_info.version));

        let mut file = tokio::fs::File::create(&temp_file)
            .await
            .map_err(|e| format!("Failed to create temp file: {}", e))?;

        // Download with progress tracking
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| format!("Download error: {}", e))?;
            file.write_all(&chunk)
                .await
                .map_err(|e| format!("Failed to write to temp file: {}", e))?;

            downloaded += chunk.len() as u64;
            let progress = if total_size > 0 {
                (downloaded as f64 / total_size as f64) * 100.0
            } else {
                0.0
            };

            // Emit progress event
            window
                .emit("update-download-progress", progress)
                .map_err(|e| format!("Failed to emit progress: {}", e))?;
        }

        // Verify signature
        self.verify_signature(&temp_file, &update_info.signature)?;

        // Emit download completed event
        window
            .emit("update-download-completed", ())
            .map_err(|e| format!("Failed to emit event: {}", e))?;

        // Install the update
        self.install_update(&temp_file, update_info).await?;

        Ok(())
    }

    /// Verify the downloaded file signature
    fn verify_signature(&self, file_path: &PathBuf, expected_signature: &str) -> Result<(), String> {
        use sha2::{Digest, Sha256};
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(file_path)
            .map_err(|e| format!("Failed to open file for verification: {}", e))?;

        let mut hasher = Sha256::new();
        let mut buffer = [0; 1024];

        loop {
            let n = file
                .read(&mut buffer)
                .map_err(|e| format!("Failed to read file: {}", e))?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        let actual_signature = format!("{:x}", hasher.finalize());

        if actual_signature != expected_signature {
            return Err("Signature verification failed".to_string());
        }

        Ok(())
    }

    /// Install the downloaded update
    async fn install_update(&self, file_path: &PathBuf, update_info: &UpdateInfo) -> Result<(), String> {
        let window = self
            .app
            .get_window("main")
            .ok_or("Main window not found")?;

        // Emit installation started event
        window
            .emit("update-installation-started", ())
            .map_err(|e| format!("Failed to emit event: {}", e))?;

        // Platform-specific installation
        #[cfg(target_os = "windows")]
        {
            self.install_windows_update(file_path, update_info).await?;
        }

        #[cfg(target_os = "macos")]
        {
            self.install_macos_update(file_path, update_info).await?;
        }

        #[cfg(target_os = "linux")]
        {
            self.install_linux_update(file_path, update_info).await?;
        }

        // Emit installation completed event
        window
            .emit("update-installation-completed", ())
            .map_err(|e| format!("Failed to emit event: {}", e))?;

        Ok(())
    }

    #[cfg(target_os = "windows")]
    async fn install_windows_update(&self, file_path: &PathBuf, _update_info: &UpdateInfo) -> Result<(), String> {
        use std::process::Command;

        // For Windows, we typically restart the application and let it handle the update
        // This is a simplified version - in production you'd want more sophisticated update logic
        
        // Create a batch script to handle the update
        let update_script = format!(
            r#"
@echo off
timeout /t 2 /nobreak > nul
copy "{}" "%~dp0findag-desktop-new.exe"
start "" "%~dp0findag-desktop-new.exe"
del "%~f0"
"#,
            file_path.display()
        );

        let script_path = std::env::temp_dir().join("findag-update.bat");
        tokio::fs::write(&script_path, update_script)
            .await
            .map_err(|e| format!("Failed to create update script: {}", e))?;

        // Execute the update script
        Command::new("cmd")
            .args(&["/c", script_path.to_str().unwrap()])
            .spawn()
            .map_err(|e| format!("Failed to start update process: {}", e))?;

        // Exit the current application
        std::process::exit(0);
    }

    #[cfg(target_os = "macos")]
    async fn install_macos_update(&self, file_path: &PathBuf, _update_info: &UpdateInfo) -> Result<(), String> {
        use std::process::Command;

        // For macOS, we mount the DMG and copy the app
        let mount_point = std::env::temp_dir().join("findag-update-mount");
        tokio::fs::create_dir_all(&mount_point)
            .await
            .map_err(|e| format!("Failed to create mount point: {}", e))?;

        // Mount the DMG
        Command::new("hdiutil")
            .args(&["attach", file_path.to_str().unwrap(), "-mountpoint", mount_point.to_str().unwrap()])
            .output()
            .map_err(|e| format!("Failed to mount DMG: {}", e))?;

        // Copy the app to Applications
        let app_source = mount_point.join("FinDAG Desktop.app");
        let app_dest = PathBuf::from("/Applications/FinDAG Desktop.app");

        if app_dest.exists() {
            tokio::fs::remove_dir_all(&app_dest)
                .await
                .map_err(|e| format!("Failed to remove existing app: {}", e))?;
        }

        tokio::fs::rename(&app_source, &app_dest)
            .await
            .map_err(|e| format!("Failed to copy app: {}", e))?;

        // Unmount the DMG
        Command::new("hdiutil")
            .args(&["detach", mount_point.to_str().unwrap()])
            .output()
            .map_err(|e| format!("Failed to unmount DMG: {}", e))?;

        // Launch the new version
        Command::new("open")
            .arg("/Applications/FinDAG Desktop.app")
            .spawn()
            .map_err(|e| format!("Failed to launch new version: {}", e))?;

        // Exit the current application
        std::process::exit(0);
    }

    #[cfg(target_os = "linux")]
    async fn install_linux_update(&self, file_path: &PathBuf, _update_info: &UpdateInfo) -> Result<(), String> {
        use std::process::Command;

        // For Linux, we typically use AppImage or package manager
        // This is a simplified version for AppImage updates
        
        let app_dir = std::env::current_exe()
            .map_err(|e| format!("Failed to get current exe: {}", e))?
            .parent()
            .ok_or("Failed to get app directory")?
            .to_path_buf();

        let new_app_path = app_dir.join("findag-desktop-new");
        
        // Copy the new version
        tokio::fs::copy(file_path, &new_app_path)
            .await
            .map_err(|e| format!("Failed to copy new version: {}", e))?;

        // Make it executable
        Command::new("chmod")
            .args(&["+x", new_app_path.to_str().unwrap()])
            .output()
            .map_err(|e| format!("Failed to make executable: {}", e))?;

        // Launch the new version
        Command::new(new_app_path)
            .spawn()
            .map_err(|e| format!("Failed to launch new version: {}", e))?;

        // Exit the current application
        std::process::exit(0);
    }

    /// Compare two version strings
    fn compare_versions(&self, version1: &str, version2: &str) -> i32 {
        let v1_parts: Vec<u32> = version1
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        let v2_parts: Vec<u32> = version2
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();

        let max_len = std::cmp::max(v1_parts.len(), v2_parts.len());
        
        for i in 0..max_len {
            let v1 = v1_parts.get(i).unwrap_or(&0);
            let v2 = v2_parts.get(i).unwrap_or(&0);
            
            match v1.cmp(v2) {
                std::cmp::Ordering::Less => return -1,
                std::cmp::Ordering::Greater => return 1,
                std::cmp::Ordering::Equal => continue,
            }
        }
        
        0
    }

    /// Get current update status
    pub fn get_status(&self) -> UpdateStatus {
        UpdateStatus {
            available: false,
            current_version: self.current_version.clone(),
            latest_version: None,
            download_progress: 0.0,
            is_downloading: false,
            is_installing: false,
            error: None,
        }
    }
}

// Tauri commands
#[tauri::command]
pub async fn check_for_updates<R: Runtime>(
    app: AppHandle<R>,
    update_url: String,
) -> Result<UpdateInfo, String> {
    let updater = Updater::new(app, update_url);
    updater.check_for_updates().await
}

#[tauri::command]
pub async fn download_update<R: Runtime>(
    app: AppHandle<R>,
    update_url: String,
    update_info: UpdateInfo,
) -> Result<(), String> {
    let updater = Updater::new(app, update_url);
    updater.download_and_install(&update_info).await
}

#[tauri::command]
pub fn get_update_status<R: Runtime>(
    app: AppHandle<R>,
    update_url: String,
) -> UpdateStatus {
    let updater = Updater::new(app, update_url);
    updater.get_status()
} 