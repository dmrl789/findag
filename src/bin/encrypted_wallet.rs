use clap::{Parser, Subcommand};
use findag::core::wallet::{Wallet, WalletManager, prompt_password, prompt_password_confirm};
use findag::core::address::Address;
use serde::{Serialize, Deserialize};
use reqwest;
use std::path::PathBuf;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "FinDAG Encrypted Wallet")]
#[command(about = "A secure encrypted wallet for FinDAG with password protection", long_about = None)]
struct Cli {
    #[arg(long, default_value = "http://127.0.0.1:8080")]
    node_url: String,
    #[arg(long, default_value = "wallet.dat")]
    wallet_file: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new encrypted wallet
    Create {},
    /// Unlock and load existing wallet
    Unlock {},
    /// Show wallet information
    Info {},
    /// Show wallet address
    Address {},
    /// Show balance (via node API)
    Balance {
        #[arg(long, default_value = "USD")]
        currency: String,
    },
    /// Send a payment (sign and submit tx)
    Send {
        #[arg(long)]
        to: String,
        #[arg(long)]
        amount: u64,
        #[arg(long, default_value = "USD")]
        currency: String,
    },
    /// Export private key (for backup)
    Export {},
    /// Import private key from hex
    Import {
        #[arg(long)]
        private_key: String,
    },
    /// Add a new account to the wallet
    AddAccount {
        #[arg(long)]
        name: String,
    },
    /// List all accounts
    ListAccounts {},
    /// Change wallet password
    ChangePassword {},
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ApiTransaction {
    from: String,
    to: String,
    amount: u64,
    currency: String,
    signature: String,
    public_key: String,
}

async fn fetch_asset_whitelist(node_url: &str) -> Vec<String> {
    let url = format!("{}/assets", node_url);
    match reqwest::get(&url).await {
        Ok(resp) => match resp.json::<serde_json::Value>().await {
            Ok(json) => json["assets"].as_array().map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()).unwrap_or_default(),
            Err(_) => vec![],
        },
        Err(_) => vec![],
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let wallet_manager = WalletManager::new(&cli.wallet_file);
    
    match &cli.command {
        Commands::Create {} => {
            println!("Creating new encrypted wallet...");
            let password = match prompt_password_confirm() {
                Ok(pwd) => pwd,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };
            
            match wallet_manager.create_wallet(&password) {
                Ok(wallet) => {
                    println!("✅ Wallet created successfully!");
                    println!("Address: {}", wallet.address().as_str());
                    println!("Public Key: {}", wallet.public_key_hex());
                    println!("Wallet file: {}", cli.wallet_file);
                    println!("\n⚠️  IMPORTANT: Keep your password safe! If you lose it, you cannot recover your wallet.");
                }
                Err(e) => {
                    eprintln!("Error creating wallet: {}", e);
                }
            }
        }
        
        Commands::Unlock {} => {
            if !wallet_manager.wallet_exists() {
                eprintln!("Error: Wallet file does not exist. Create one first with 'create' command.");
                return;
            }
            
            let password = match prompt_password("Enter wallet password") {
                Ok(pwd) => pwd,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };
            
            match wallet_manager.load_wallet(&password) {
                Ok(wallet) => {
                    println!("✅ Wallet unlocked successfully!");
                    println!("Address: {}", wallet.address().as_str());
                }
                Err(e) => {
                    eprintln!("Error unlocking wallet: {}", e);
                }
            }
        }
        
        Commands::Info {} => {
            if !wallet_manager.wallet_exists() {
                eprintln!("Error: Wallet file does not exist.");
                return;
            }
            
            let password = match prompt_password("Enter wallet password") {
                Ok(pwd) => pwd,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };
            
            match wallet_manager.load_wallet(&password) {
                Ok(wallet) => {
                    println!("=== Wallet Information ===");
                    println!("Address: {}", wallet.address().as_str());
                    println!("Public Key: {}", wallet.public_key_hex());
                    println!("Accounts: {}", wallet.accounts().len());
                    for account in wallet.accounts() {
                        println!("  - {}: {} {}", 
                            account.name, 
                            account.address,
                            if account.is_default { "(default)" } else { "" }
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Error loading wallet: {}", e);
                }
            }
        }
        
        Commands::Address {} => {
            if !wallet_manager.wallet_exists() {
                eprintln!("Error: Wallet file does not exist.");
                return;
            }
            
            let password = match prompt_password("Enter wallet password") {
                Ok(pwd) => pwd,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };
            
            match wallet_manager.load_wallet(&password) {
                Ok(wallet) => {
                    println!("{}", wallet.address().as_str());
                }
                Err(e) => {
                    eprintln!("Error loading wallet: {}", e);
                }
            }
        }
        
        Commands::Balance { currency } => {
            if !wallet_manager.wallet_exists() {
                eprintln!("Error: Wallet file does not exist.");
                return;
            }
            
            let password = match prompt_password("Enter wallet password") {
                Ok(pwd) => pwd,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };
            
            let wallet = match wallet_manager.load_wallet(&password) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Error loading wallet: {}", e);
                    return;
                }
            };
            
            let whitelist = fetch_asset_whitelist(&cli.node_url).await;
            if !whitelist.contains(&currency.to_string()) {
                println!("Error: '{}' is not a supported asset.", currency);
                return;
            }
            
            let url = format!("{}/balance/{}/{}", cli.node_url, wallet.address().as_str(), currency);
            match reqwest::get(&url).await {
                Ok(resp) => {
                    match resp.json::<serde_json::Value>().await {
                        Ok(json) => {
                            println!("Balance for {}: {} {}", wallet.address().as_str(), json["balance"], currency);
                        }
                        Err(_) => {
                            eprintln!("Error: Invalid response from node");
                        }
                    }
                }
                Err(_) => {
                    eprintln!("Error: Failed to connect to node at {}", cli.node_url);
                }
            }
        }
        
        Commands::Send { to, amount, currency } => {
            if !wallet_manager.wallet_exists() {
                eprintln!("Error: Wallet file does not exist.");
                return;
            }
            
            let password = match prompt_password("Enter wallet password") {
                Ok(pwd) => pwd,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };
            
            let mut wallet = match wallet_manager.load_wallet(&password) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Error loading wallet: {}", e);
                    return;
                }
            };
            
            let whitelist = fetch_asset_whitelist(&cli.node_url).await;
            if !whitelist.contains(&currency.to_string()) {
                println!("Error: '{}' is not a supported asset.", currency);
                return;
            }
            
            let from_address = wallet.address().as_str().to_string();
            let to_address = to.clone();
            let signature = wallet.sign(b"mock-tx");
            let tx = ApiTransaction {
                from: from_address,
                to: to_address,
                amount: *amount,
                currency: currency.clone(),
                signature: hex::encode(signature),
                public_key: wallet.public_key_hex(),
            };
            
            let url = format!("{}/tx", cli.node_url);
            let client = reqwest::Client::new();
            match client.post(&url).json(&tx).send().await {
                Ok(resp) => {
                    match resp.json::<serde_json::Value>().await {
                        Ok(json) => {
                            println!("✅ Transaction sent successfully!");
                            println!("Response: {}", json);
                        }
                        Err(_) => {
                            eprintln!("Error: Invalid response from node");
                        }
                    }
                }
                Err(_) => {
                    eprintln!("Error: Failed to send transaction to node");
                }
            }
        }
        
        Commands::Export {} => {
            if !wallet_manager.wallet_exists() {
                eprintln!("Error: Wallet file does not exist.");
                return;
            }
            
            let password = match prompt_password("Enter wallet password") {
                Ok(pwd) => pwd,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };
            
            let wallet = match wallet_manager.load_wallet(&password) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Error loading wallet: {}", e);
                    return;
                }
            };
            
            println!("⚠️  WARNING: Keep this private key secure! Anyone with this key can access your funds.");
            println!("Private Key: {}", wallet.export_private_key());
        }
        
        Commands::Import { private_key } => {
            if wallet_manager.wallet_exists() {
                eprintln!("Error: Wallet already exists. Remove the existing wallet file first.");
                return;
            }
            
            let wallet = match Wallet::from_private_key_hex(private_key) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Error importing private key: {}", e);
                    return;
                }
            };
            
            let password = match prompt_password_confirm() {
                Ok(pwd) => pwd,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };
            
            match wallet_manager.save_wallet(&wallet, &password) {
                Ok(_) => {
                    println!("✅ Wallet imported successfully!");
                    println!("Address: {}", wallet.address().as_str());
                    println!("Public Key: {}", wallet.public_key_hex());
                }
                Err(e) => {
                    eprintln!("Error saving wallet: {}", e);
                }
            }
        }
        
        Commands::AddAccount { name } => {
            if !wallet_manager.wallet_exists() {
                eprintln!("Error: Wallet file does not exist.");
                return;
            }
            
            let password = match prompt_password("Enter wallet password") {
                Ok(pwd) => pwd,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };
            
            let mut wallet = match wallet_manager.load_wallet(&password) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Error loading wallet: {}", e);
                    return;
                }
            };
            
            match wallet.add_account(name) {
                Ok(_) => {
                    // Save the updated wallet
                    if let Err(e) = wallet_manager.save_wallet(&wallet, &password) {
                        eprintln!("Error saving wallet: {}", e);
                        return;
                    }
                    
                    println!("✅ Account '{}' added successfully!", name);
                    if let Some(account) = wallet.accounts().last() {
                        println!("Address: {}", account.address);
                        println!("Public Key: {}", account.public_key);
                    }
                }
                Err(e) => {
                    eprintln!("Error adding account: {}", e);
                }
            }
        }
        
        Commands::ListAccounts {} => {
            if !wallet_manager.wallet_exists() {
                eprintln!("Error: Wallet file does not exist.");
                return;
            }
            
            let password = match prompt_password("Enter wallet password") {
                Ok(pwd) => pwd,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };
            
            let wallet = match wallet_manager.load_wallet(&password) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Error loading wallet: {}", e);
                    return;
                }
            };
            
            println!("=== Wallet Accounts ===");
            for (i, account) in wallet.accounts().iter().enumerate() {
                println!("{}. {}: {} {}", 
                    i + 1,
                    account.name, 
                    account.address,
                    if account.is_default { "(default)" } else { "" }
                );
            }
        }
        
        Commands::ChangePassword {} => {
            if !wallet_manager.wallet_exists() {
                eprintln!("Error: Wallet file does not exist.");
                return;
            }
            
            let old_password = match prompt_password("Enter current password") {
                Ok(pwd) => pwd,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };
            
            let wallet = match wallet_manager.load_wallet(&old_password) {
                Ok(w) => w,
                Err(e) => {
                    eprintln!("Error loading wallet: {}", e);
                    return;
                }
            };
            
            let new_password = match prompt_password_confirm() {
                Ok(pwd) => pwd,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };
            
            match wallet_manager.save_wallet(&wallet, &new_password) {
                Ok(_) => {
                    println!("✅ Password changed successfully!");
                }
                Err(e) => {
                    eprintln!("Error saving wallet: {}", e);
                }
            }
        }
    }
} 