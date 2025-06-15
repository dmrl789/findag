// use crate::wallet::{create_wallet, load_wallet, save_wallet, add_asset, add_currency};
use crate::wallet::{Wallet, WalletConfig, RecoveryShare};
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::error::Error;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

pub async fn wallet_cli() -> Result<(), Box<dyn Error>> {
    println!("FinDAG Wallet CLI:");
    println!("1. Create new wallet");
    println!("2. Load existing wallet");
    println!("3. Show wallet info");
    println!("4. Setup key recovery");
    println!("5. Recover from shares");
    print!("Select option: ");
    stdout().flush()?;
    
    let mut opt = String::new();
    stdin().read_line(&mut opt)?;

    match opt.trim() {
        "1" => create_wallet().await?,
        "2" => load_wallet().await?,
        "3" => show_wallet_info().await?,
        "4" => setup_recovery().await?,
        "5" => recover_from_shares().await?,
        _ => println!("Invalid option.")
    }

    Ok(())
}

async fn create_wallet() -> Result<(), Box<dyn Error>> {
    println!("\nCreate New Wallet");
    println!("----------------");
    
    print!("Enable encryption? (y/n): ");
    stdout().flush()?;
    let mut encrypted = String::new();
    stdin().read_line(&mut encrypted)?;
    let encrypted = encrypted.trim().to_lowercase() == "y";
    
    print!("Enable mnemonic phrase? (y/n): ");
    stdout().flush()?;
    let mut mnemonic = String::new();
    stdin().read_line(&mut mnemonic)?;
    let mnemonic_enabled = mnemonic.trim().to_lowercase() == "y";
    
    let config = WalletConfig {
        encrypted,
        mnemonic_enabled,
        recovery_shares: None,
    };
    
    let wallet = Wallet::new(config)?;
    
    if wallet.config.encrypted {
        print!("Enter password for encryption: ");
        stdout().flush()?;
        let mut password = String::new();
        stdin().read_line(&mut password)?;
        let _ = wallet.encrypt_private_key(&password.trim()).await?;
    }
    
    if let Some(mnemonic) = wallet.get_mnemonic() {
        println!("\nIMPORTANT: Write down your mnemonic phrase:");
        println!("{}", mnemonic);
        println!("\nKeep this phrase safe. It can be used to recover your wallet.");
    }
    
    print!("\nEnter path to save wallet: ");
    stdout().flush()?;
    let mut path = String::new();
    stdin().read_line(&mut path)?;
    
    wallet.save_to_file(Path::new(path.trim()))?;
    println!("Wallet created and saved successfully!");
    
    Ok(())
}

async fn load_wallet() -> Result<(), Box<dyn Error>> {
    print!("\nEnter wallet path: ");
    stdout().flush()?;
    let mut path = String::new();
    stdin().read_line(&mut path)?;
    
    let mut wallet = Wallet::load_from_file(Path::new(path.trim()))?;
    
    if wallet.config.encrypted {
        print!("Enter password: ");
        stdout().flush()?;
        let mut password = String::new();
        stdin().read_line(&mut password)?;
        
        // Get the encrypted private key from the wallet file
        let encrypted_key = wallet.encrypt_private_key(&password.trim()).await?;
        wallet.decrypt_private_key(&password.trim(), &encrypted_key).await?;
    }
    
    println!("Wallet loaded successfully!");
    Ok(())
}

async fn show_wallet_info() -> Result<(), Box<dyn Error>> {
    print!("\nEnter wallet path: ");
    stdout().flush()?;
    let mut path = String::new();
    stdin().read_line(&mut path)?;
    
    let wallet = Wallet::load_from_file(Path::new(path.trim()))?;
    
    println!("\nWallet Information:");
    println!("------------------");
    println!("Encrypted: {}", wallet.config.encrypted);
    println!("Mnemonic Enabled: {}", wallet.config.mnemonic_enabled);
    
    if let Some(mnemonic) = wallet.get_mnemonic() {
        println!("Mnemonic Phrase: {}", mnemonic);
    }
    
    if let Some(recovery_data) = &wallet.recovery_data {
        println!("\nRecovery Information:");
        println!("Threshold: {}", recovery_data.threshold);
        println!("Number of Shares: {}", recovery_data.shares.len());
        println!("Created At: {}", recovery_data.created_at);
        
        println!("\nRecovery Shares:");
        for share in &recovery_data.shares {
            println!("Share ID: {}", share.share_id);
            println!("Holder: {}", share.holder);
            println!("Created At: {}", share.created_at);
            println!("Encrypted Share: {}", BASE64.encode(&share.encrypted_share));
            println!("---");
        }
    }
    
    Ok(())
}

async fn setup_recovery() -> Result<(), Box<dyn Error>> {
    print!("\nEnter wallet path: ");
    stdout().flush()?;
    let mut path = String::new();
    stdin().read_line(&mut path)?;
    
    let mut wallet = Wallet::load_from_file(Path::new(path.trim()))?;
    
    if wallet.config.encrypted {
        print!("Enter password: ");
        stdout().flush()?;
        let mut password = String::new();
        stdin().read_line(&mut password)?;
        
        let encrypted_key = wallet.encrypt_private_key(&password.trim()).await?;
        wallet.decrypt_private_key(&password.trim(), &encrypted_key).await?;
    }
    
    print!("Enter recovery threshold (number of shares needed): ");
    stdout().flush()?;
    let mut threshold = String::new();
    stdin().read_line(&mut threshold)?;
    let threshold: u8 = threshold.trim().parse()?;
    
    println!("Enter holder addresses (one per line, empty line to finish):");
    let mut holders = Vec::new();
    loop {
        let mut holder = String::new();
        stdin().read_line(&mut holder)?;
        let holder = holder.trim().to_string();
        if holder.is_empty() {
            break;
        }
        holders.push(holder);
    }
    
    wallet.setup_recovery(threshold, holders).await?;
    wallet.save_to_file(Path::new(path.trim()))?;
    
    println!("Recovery setup completed successfully!");
    Ok(())
}

async fn recover_from_shares() -> Result<(), Box<dyn Error>> {
    print!("\nEnter wallet path: ");
    stdout().flush()?;
    let mut path = String::new();
    stdin().read_line(&mut path)?;
    
    let mut wallet = Wallet::load_from_file(Path::new(path.trim()))?;
    
    println!("Enter recovery shares (one per line, empty line to finish):");
    println!("Format: <share_id>:<encrypted_share>:<holder>");
    let mut shares = Vec::new();
    loop {
        let mut share = String::new();
        stdin().read_line(&mut share)?;
        let share = share.trim().to_string();
        if share.is_empty() {
            break;
        }
        
        let parts: Vec<&str> = share.split(':').collect();
        if parts.len() != 3 {
            println!("Invalid share format. Skipping...");
            continue;
        }
        
        let share_id = parts[0].to_string();
        let encrypted_share = BASE64.decode(parts[1])
            .map_err(|e| format!("Invalid base64 encoding: {}", e))?;
        let holder = parts[2].to_string();
        
        shares.push(RecoveryShare {
            share_id,
            encrypted_share,
            holder,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        });
    }
    
    wallet.recover_from_shares(shares).await?;
    wallet.save_to_file(Path::new(path.trim()))?;
    
    println!("Wallet recovered successfully!");
    Ok(())
}
