// use crate::wallet::{create_wallet, load_wallet, save_wallet, add_asset, add_currency};
use crate::wallet::Wallet;
// use crate::config::WalletConfig;
use crate::types::RecoveryShare;
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
    
    let wallet = Wallet::new()?;
    
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
    
    if let Some(mnemonic) = wallet.get_mnemonic() {
        println!("Mnemonic Phrase: {}", mnemonic);
    }
    
    if let Some(recovery_data) = &wallet.recovery_data {
        println!("\nRecovery Information:");
        println!("Number of Shares: {}", recovery_data.len());
        println!("\nRecovery Shares:");
        for share in recovery_data {
            println!("Index: {}", share.index);
            println!("Share: {}", BASE64.encode(&share.share));
            println!("Signature: {}", BASE64.encode(&share.signature));
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
    
    print!("Enter recovery threshold (number of shares needed): ");
    stdout().flush()?;
    let mut threshold = String::new();
    stdin().read_line(&mut threshold)?;
    let threshold: u32 = threshold.trim().parse()?;
    
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
    
    wallet.setup_recovery(threshold.into(), holders).await?;
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
        
        let index: u32 = parts[0].parse()?;
        let share_data = BASE64.decode(parts[1])?;
        let signature = BASE64.decode(parts[2])?;
        
        shares.push(RecoveryShare {
            index: index as u8,
            share: share_data,
            signature,
        });
    }
    
    wallet.recover_from_shares(shares).await?;
    wallet.save_to_file(Path::new(path.trim()))?;
    
    println!("Wallet recovered successfully!");
    Ok(())
}
