use clap::{Parser, Subcommand, Args};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;

use findag::core::wallet::{Wallet, WalletManager, prompt_password, prompt_password_confirm};

#[derive(Parser)]
#[command(name = "findag-wallet")]
#[command(about = "FinDAG CLI Wallet: Secure key manager and asset instruction signer", long_about = None)]
struct Cli {
    #[arg(long, default_value = "wallet.dat")]
    wallet_file: String,
    #[arg(long, default_value = "http://127.0.0.1:8080")]
    node_url: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new Ed25519 keypair and set handle
    Keygen {
        #[arg(long)]
        handle: String,
    },
    /// Import existing private key and set handle
    ImportKey {
        #[arg(long)]
        file: PathBuf,
        #[arg(long)]
        handle: String,
    },
    /// Show wallet identity (handle, public key)
    Identity {},
    /// Query asset balances from the ledger
    Balances {},
    /// Build and sign a LoadAsset instruction
    LoadAsset(LoadAssetArgs),
    /// Build and sign a TransferAsset instruction
    TransferAsset(TransferAssetArgs),
    /// Sign a proof of ownership for an asset
    SignOwnership {
        #[arg(long)]
        asset_id: String,
    },
    /// Sign an arbitrary instruction JSON file
    SignInstruction {
        #[arg(long)]
        file: PathBuf,
    },
    /// Broadcast a signed instruction to a node API
    Broadcast {
        #[arg(long)]
        file: PathBuf,
    },
    /// Rotate your signing key (requires old key)
    RotateKey {
        #[arg(long)]
        new_key: PathBuf, // file containing new private key (hex)
        #[arg(long, default_value = "rotate_key_instruction.json")]
        out: PathBuf,     // output file for the signed instruction
    },
}

#[derive(Args)]
struct LoadAssetArgs {
    #[arg(long)]
    asset_id: String,
    #[arg(long)]
    amount: String,
    #[arg(long)]
    currency: String,
    #[arg(long)]
    metadata: String, // JSON string
}

#[derive(Args)]
struct TransferAssetArgs {
    #[arg(long)]
    asset_id: String,
    #[arg(long)]
    amount: String,
    #[arg(long)]
    to: String,
}

#[derive(Serialize, Deserialize)]
struct LoadAssetInstruction {
    r#type: String,
    asset_id: String,
    amount: String,
    currency: String,
    issuer: String,
    metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
struct TransferAssetInstruction {
    r#type: String,
    asset_id: String,
    amount: String,
    from: String,
    to: String,
}

#[derive(Serialize, Deserialize)]
struct OwnershipProof {
    r#type: String,
    asset_id: String,
    owner: String,
    timestamp: u64,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct SignedInstruction {
    instruction: serde_json::Value,
    signature: String,
    public_key: String,
}

#[derive(Serialize, Deserialize)]
struct AssetBalance {
    asset_id: String,
    amount: String,
    currency: Option<String>,
    unit: Option<String>,
    issuer: String,
    metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
struct WalletConfig {
    handle: String,
    public_key: String,
}

#[derive(Serialize, Deserialize)]
struct RotateKeyInstruction {
    r#type: String,
    handle: String,
    new_pubkey: String,
    prev_pubkey: String,
    timestamp: u64,
}

#[derive(Serialize, Deserialize)]
struct RotateKeySignatures {
    old_key: String,
    // Optionally, add authority signature here
}

#[derive(Serialize, Deserialize)]
struct SignedRotateKeyInstruction {
    instruction: RotateKeyInstruction,
    signatures: RotateKeySignatures,
    public_key: String, // old public key
}

async fn query_balances(node_url: &str, handle: &str) -> Result<Vec<AssetBalance>, String> {
    let url = format!("{node_url}/assets?owner={handle}");
    let client = reqwest::Client::new();
    
    let response = client.get(&url)
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| format!("Failed to query node: {e}"))?;
    
    if !response.status().is_success() {
        return Err(format!("Node returned error: {}", response.status()));
    }
    
    let balances: Vec<AssetBalance> = response.json().await
        .map_err(|e| format!("Failed to parse response: {e}"))?;
    
    Ok(balances)
}

fn load_wallet_config(wallet_file: &str) -> Result<WalletConfig, String> {
    let config_file = format!("{wallet_file}.config");
    let config_data = fs::read_to_string(&config_file)
        .map_err(|_| "No wallet config found. Run 'keygen' first.".to_string())?;
    
    serde_json::from_str(&config_data)
        .map_err(|e| format!("Invalid config format: {e}"))
}

fn save_wallet_config(wallet_file: &str, config: &WalletConfig) -> Result<(), String> {
    let config_file = format!("{wallet_file}.config");
    let config_data = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {e}"))?;
    
    fs::write(&config_file, config_data)
        .map_err(|e| format!("Failed to write config: {e}"))
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let wallet_manager = WalletManager::new(&cli.wallet_file);

    match &cli.command {
        Commands::Keygen { handle } => {
            let password = match prompt_password_confirm() {
                Ok(pwd) => pwd,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            let wallet = match wallet_manager.create_wallet(&password) {
                Ok(w) => w,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            
            // Save wallet config with handle
            let config = WalletConfig {
                handle: handle.clone(),
                public_key: wallet.public_key_hex(),
            };
            if let Err(e) = save_wallet_config(&cli.wallet_file, &config) {
                eprintln!("Warning: Failed to save config: {e}");
            }
            
            println!("✅ Key generated successfully!");
            println!("Handle: {handle}");
            println!("Public Key: {}", wallet.public_key_hex());
            println!("Wallet file: {}", cli.wallet_file);
            println!("⚠️  Keep your password safe - it cannot be recovered!");
        }
        
        Commands::ImportKey { file, handle } => {
            let password = match prompt_password_confirm() {
                Ok(pwd) => pwd,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            
            // Read private key from file
            let private_key_hex = match fs::read_to_string(file) {
                Ok(s) => s.trim().to_string(),
                Err(e) => { eprintln!("Failed to read key file: {e}"); return; }
            };
            
            let wallet = match Wallet::from_private_key_hex(&private_key_hex) {
                Ok(w) => w,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            
            // Save encrypted wallet
            if let Err(e) = wallet_manager.save_wallet(&wallet, &password) {
                eprintln!("Error saving wallet: {e}");
                return;
            }
            
            // Save wallet config with handle
            let config = WalletConfig {
                handle: handle.clone(),
                public_key: wallet.public_key_hex(),
            };
            if let Err(e) = save_wallet_config(&cli.wallet_file, &config) {
                eprintln!("Warning: Failed to save config: {e}");
            }
            
            println!("✅ Key imported successfully!");
            println!("Handle: {handle}");
            println!("Public Key: {}", wallet.public_key_hex());
        }
        
        Commands::Identity {} => {
            let config = match load_wallet_config(&cli.wallet_file) {
                Ok(c) => c,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            
            println!("Handle: {}", config.handle);
            println!("Public Key: {}", config.public_key);
        }
        
        Commands::Balances {} => {
            let config = match load_wallet_config(&cli.wallet_file) {
                Ok(c) => c,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            
            println!("Querying balances for {}...", config.handle);
            
            match query_balances(&cli.node_url, &config.handle).await {
                Ok(balances) => {
                    if balances.is_empty() {
                        println!("No assets found for {}", config.handle);
                    } else {
                        println!("Asset Balances:");
                        println!("{:<20} {:<15} {:<10} {:<20}", "Asset ID", "Amount", "Currency", "Issuer");
                        println!("{:-<70}", "");
                        for balance in balances {
                            let currency = balance.currency.as_deref().unwrap_or("N/A");
                            println!("{:<20} {:<15} {:<10} {:<20}", 
                                balance.asset_id, balance.amount, currency, balance.issuer);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error querying balances: {e}");
                }
            }
        }
        
        Commands::LoadAsset(args) => {
            let config = match load_wallet_config(&cli.wallet_file) {
                Ok(c) => c,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            
            let password = match prompt_password("Enter wallet password") {
                Ok(pwd) => pwd,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            let wallet = match wallet_manager.load_wallet(&password) {
                Ok(w) => w,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            
            let metadata: serde_json::Value = match serde_json::from_str(&args.metadata) {
                Ok(m) => m,
                Err(e) => { eprintln!("Invalid metadata JSON: {e}"); return; }
            };
            
            let instr = LoadAssetInstruction {
                r#type: "load_asset".to_string(),
                asset_id: args.asset_id.clone(),
                amount: args.amount.clone(),
                currency: args.currency.clone(),
                issuer: config.handle,
                metadata,
            };
            
            let instr_json = serde_json::to_value(&instr).unwrap();
            let sig = wallet.sign(serde_json::to_string(&instr_json).unwrap().as_bytes());
            let signed = SignedInstruction {
                instruction: instr_json,
                signature: hex::encode(sig),
                public_key: wallet.public_key_hex(),
            };
            
            let out = serde_json::to_string_pretty(&signed).unwrap();
            println!("{out}");
        }
        
        Commands::TransferAsset(args) => {
            let config = match load_wallet_config(&cli.wallet_file) {
                Ok(c) => c,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            
            let password = match prompt_password("Enter wallet password") {
                Ok(pwd) => pwd,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            let wallet = match wallet_manager.load_wallet(&password) {
                Ok(w) => w,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            
            let instr = TransferAssetInstruction {
                r#type: "transfer_asset".to_string(),
                asset_id: args.asset_id.clone(),
                amount: args.amount.clone(),
                from: config.handle,
                to: args.to.clone(),
            };
            
            let instr_json = serde_json::to_value(&instr).unwrap();
            let sig = wallet.sign(serde_json::to_string(&instr_json).unwrap().as_bytes());
            let signed = SignedInstruction {
                instruction: instr_json,
                signature: hex::encode(sig),
                public_key: wallet.public_key_hex(),
            };
            
            let out = serde_json::to_string_pretty(&signed).unwrap();
            println!("{out}");
        }
        
        Commands::SignOwnership { asset_id } => {
            let config = match load_wallet_config(&cli.wallet_file) {
                Ok(c) => c,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            
            let password = match prompt_password("Enter wallet password") {
                Ok(pwd) => pwd,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            let wallet = match wallet_manager.load_wallet(&password) {
                Ok(w) => w,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            
            let timestamp = chrono::Utc::now().timestamp() as u64;
            let message = format!("I own {asset_id} at timestamp {timestamp}");
            
            let proof = OwnershipProof {
                r#type: "ownership_proof".to_string(),
                asset_id: asset_id.clone(),
                owner: config.handle,
                timestamp,
                message: message.clone(),
            };
            
            let proof_json = serde_json::to_value(&proof).unwrap();
            let sig = wallet.sign(message.as_bytes());
            let signed = SignedInstruction {
                instruction: proof_json,
                signature: hex::encode(sig),
                public_key: wallet.public_key_hex(),
            };
            
            let out = serde_json::to_string_pretty(&signed).unwrap();
            println!("{out}");
        }
        
        Commands::SignInstruction { file } => {
            let password = match prompt_password("Enter wallet password") {
                Ok(pwd) => pwd,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            let wallet = match wallet_manager.load_wallet(&password) {
                Ok(w) => w,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            
            let instr_json: serde_json::Value = match fs::read_to_string(file) {
                Ok(s) => match serde_json::from_str(&s) {
                    Ok(j) => j,
                    Err(e) => { eprintln!("Invalid JSON: {e}"); return; }
                },
                Err(e) => { eprintln!("Failed to read file: {e}"); return; }
            };
            
            let sig = wallet.sign(serde_json::to_string(&instr_json).unwrap().as_bytes());
            let signed = SignedInstruction {
                instruction: instr_json,
                signature: hex::encode(sig),
                public_key: wallet.public_key_hex(),
            };
            
            let out = serde_json::to_string_pretty(&signed).unwrap();
            println!("{out}");
        }
        
        Commands::Broadcast { file } => {
            let signed_json = match fs::read_to_string(file) {
                Ok(s) => s,
                Err(e) => { eprintln!("Failed to read file: {e}"); return; }
            };
            
            let client = reqwest::Client::new();
            let url = format!("{}/submit_instruction", cli.node_url);
            
            match client.post(&url)
                .body(signed_json)
                .header("Content-Type", "application/json")
                .send()
                .await 
            {
                Ok(resp) => {
                    let status = resp.status();
                    let text = resp.text().await.unwrap_or_default();
                    println!("Node response ({status}): {text}");
                }
                Err(e) => {
                    eprintln!("Failed to broadcast: {e}");
                }
            }
        }
        
        Commands::RotateKey { new_key, out } => {
            // 1. Load wallet config and old key
            let config = match load_wallet_config(&cli.wallet_file) {
                Ok(c) => c,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            let password = match prompt_password("Enter wallet password (old key)") {
                Ok(pwd) => pwd,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            let old_wallet = match wallet_manager.load_wallet(&password) {
                Ok(w) => w,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };
            let prev_pubkey = old_wallet.public_key_hex();

            // 2. Read new private key from file
            let new_key_hex = match fs::read_to_string(new_key) {
                Ok(s) => s.trim().to_string(),
                Err(e) => { eprintln!("Failed to read new key file: {e}"); return; }
            };
            let new_wallet = match Wallet::from_private_key_hex(&new_key_hex) {
                Ok(w) => w,
                Err(e) => { eprintln!("Invalid new private key: {e}"); return; }
            };
            let new_pubkey = new_wallet.public_key_hex();

            // 3. Build rotate_key instruction
            let timestamp = chrono::Utc::now().timestamp() as u64;
            let instr = RotateKeyInstruction {
                r#type: "rotate_key".to_string(),
                handle: config.handle.clone(),
                new_pubkey: new_pubkey.clone(),
                prev_pubkey: prev_pubkey.clone(),
                timestamp,
            };

            // 4. Sign with old key
            let instr_json = serde_json::to_string(&instr).unwrap();
            let sig = old_wallet.sign(instr_json.as_bytes());
            let signatures = RotateKeySignatures {
                old_key: hex::encode(sig),
            };

            let signed = SignedRotateKeyInstruction {
                instruction: instr,
                signatures,
                public_key: prev_pubkey.clone(),
            };

            // 5. Output signed instruction to file
            let out_json = serde_json::to_string_pretty(&signed).unwrap();
            if let Err(e) = fs::write(out, out_json) {
                eprintln!("Failed to write signed instruction: {e}");
                return;
            }
            println!("✅ RotateKey instruction signed and saved to {out:?}");

            // 6. Update local wallet to use new key
            if let Err(e) = wallet_manager.save_wallet(&new_wallet, &password) {
                eprintln!("Failed to update wallet to new key: {e}");
                return;
            }
            // Update config with new public key
            let new_config = WalletConfig {
                handle: config.handle,
                public_key: new_pubkey,
            };
            if let Err(e) = save_wallet_config(&cli.wallet_file, &new_config) {
                eprintln!("Warning: Failed to update wallet config: {e}");
            }
            println!("✅ Local wallet updated to new key.");
        }
    }
} 