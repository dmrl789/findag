use chrono::Utc;
use ed25519_dalek::{Keypair, Signer, PublicKey};
use serde_json::json;
use std::fs;
use std::path::Path;
use clap::{Parser, Subcommand};
use crate::core::handle_registry::{
    RegisterSubhandleInstruction, 
    RotateKeyInstruction, 
    RevokeHandleInstruction,
    HandleRegistry
};

#[derive(Parser)]
#[command(name = "findag-handle-wallet")]
#[command(about = "FinDAG Handle Management Wallet")]
pub struct HandleWalletCli {
    #[command(subcommand)]
    command: HandleCommands,
}

#[derive(Subcommand)]
pub enum HandleCommands {
    /// Register a new subhandle under a parent
    RegisterSubhandle {
        /// Parent handle (e.g., "@hsbc.london.fd")
        #[arg(long)]
        parent: String,
        /// Subdomain name (e.g., "trading")
        #[arg(long)]
        sub: String,
        /// Path to new public key file (base64 or hex)
        #[arg(long)]
        pubkey: String,
        /// Path to parent's private key file
        #[arg(long)]
        parent_key: String,
        /// Optional metadata JSON
        #[arg(long)]
        metadata: Option<String>,
    },
    /// Rotate key for an existing handle
    RotateKey {
        /// Handle to rotate key for
        #[arg(long)]
        handle: String,
        /// Path to new public key file
        #[arg(long)]
        new_pubkey: String,
        /// Path to current private key file
        #[arg(long)]
        current_key: String,
    },
    /// Revoke a handle (must be signed by parent)
    RevokeHandle {
        /// Handle to revoke
        #[arg(long)]
        handle: String,
        /// Reason for revocation
        #[arg(long)]
        reason: String,
        /// Path to parent's private key file
        #[arg(long)]
        parent_key: String,
    },
    /// Resolve a handle to get its information
    Resolve {
        /// Handle to resolve
        #[arg(long)]
        handle: String,
    },
    /// List all children of a handle
    ListChildren {
        /// Parent handle
        #[arg(long)]
        parent: String,
    },
}

pub fn run_handle_wallet() {
    let cli = HandleWalletCli::parse();
    
    match &cli.command {
        HandleCommands::RegisterSubhandle { parent, sub, pubkey, parent_key, metadata } => {
            register_subhandle_cli(parent, sub, pubkey, parent_key, metadata.as_deref());
        }
        HandleCommands::RotateKey { handle, new_pubkey, current_key } => {
            rotate_key_cli(handle, new_pubkey, current_key);
        }
        HandleCommands::RevokeHandle { handle, reason, parent_key } => {
            revoke_handle_cli(handle, reason, parent_key);
        }
        HandleCommands::Resolve { handle } => {
            resolve_handle_cli(handle);
        }
        HandleCommands::ListChildren { parent } => {
            list_children_cli(parent);
        }
    }
}

fn register_subhandle_cli(
    parent_handle: &str,
    sub: &str,
    new_pubkey_path: &str,
    parent_keypair_path: &str,
    metadata: Option<&str>,
) {
    // 1. Load parent keypair
    let parent_key_bytes = fs::read(parent_keypair_path)
        .expect("Failed to read parent keypair file");
    let parent_keypair = Keypair::from_bytes(&parent_key_bytes)
        .expect("Invalid parent keypair format");

    // 2. Load new pubkey
    let new_pubkey_bytes = fs::read(new_pubkey_path)
        .expect("Failed to read new pubkey file");
    let new_pubkey = if new_pubkey_bytes.len() == 32 {
        // Raw bytes
        PublicKey::from_bytes(&new_pubkey_bytes)
            .expect("Invalid new pubkey bytes")
    } else {
        // Try base64
        let pubkey_str = String::from_utf8(new_pubkey_bytes)
            .expect("Invalid pubkey file encoding");
        let decoded = base64::decode(pubkey_str.trim())
            .expect("Invalid base64 pubkey");
        PublicKey::from_bytes(&decoded)
            .expect("Invalid pubkey bytes")
    };
    let new_pubkey_b64 = base64::encode(new_pubkey.to_bytes());

    // 3. Compose handle
    let handle = if parent_handle.starts_with('@') {
        format!("@{}.{}", sub, parent_handle.trim_start_matches('@'))
    } else {
        format!("@{}.{}", sub, parent_handle)
    };

    // 4. Parse metadata
    let metadata_json = metadata.map(|m| {
        serde_json::from_str(m).expect("Invalid metadata JSON")
    });

    // 5. Timestamp
    let timestamp = Utc::now().to_rfc3339();

    // 6. Build instruction (without signature)
    let mut instr = RegisterSubhandleInstruction {
        handle: handle.clone(),
        parent: parent_handle.to_string(),
        new_pubkey: new_pubkey_b64,
        metadata: metadata_json,
        timestamp: timestamp.clone(),
        parent_signature: "".to_string(),
    };

    // 7. Sign payload
    let payload = HandleRegistry::subhandle_payload_to_sign(&instr);
    let sig = parent_keypair.sign(payload.as_bytes());
    instr.parent_signature = base64::encode(sig.to_bytes());

    // 8. Output JSON
    let json = serde_json::to_string_pretty(&instr).unwrap();
    println!("{}", json);
}

fn rotate_key_cli(
    handle: &str,
    new_pubkey_path: &str,
    current_keypair_path: &str,
) {
    // 1. Load current keypair
    let current_key_bytes = fs::read(current_keypair_path)
        .expect("Failed to read current keypair file");
    let current_keypair = Keypair::from_bytes(&current_key_bytes)
        .expect("Invalid current keypair format");

    // 2. Load new pubkey
    let new_pubkey_bytes = fs::read(new_pubkey_path)
        .expect("Failed to read new pubkey file");
    let new_pubkey = if new_pubkey_bytes.len() == 32 {
        PublicKey::from_bytes(&new_pubkey_bytes)
            .expect("Invalid new pubkey bytes")
    } else {
        let pubkey_str = String::from_utf8(new_pubkey_bytes)
            .expect("Invalid pubkey file encoding");
        let decoded = base64::decode(pubkey_str.trim())
            .expect("Invalid base64 pubkey");
        PublicKey::from_bytes(&decoded)
            .expect("Invalid pubkey bytes")
    };
    let new_pubkey_b64 = base64::encode(new_pubkey.to_bytes());

    // 3. Timestamp
    let timestamp = Utc::now().to_rfc3339();

    // 4. Build instruction (without signature)
    let mut instr = RotateKeyInstruction {
        handle: handle.to_string(),
        new_pubkey: new_pubkey_b64,
        timestamp: timestamp.clone(),
        signature: "".to_string(),
    };

    // 5. Sign payload
    let payload = HandleRegistry::rotate_key_payload_to_sign(&instr);
    let sig = current_keypair.sign(payload.as_bytes());
    instr.signature = base64::encode(sig.to_bytes());

    // 6. Output JSON
    let json = serde_json::to_string_pretty(&instr).unwrap();
    println!("{}", json);
}

fn revoke_handle_cli(
    handle: &str,
    reason: &str,
    parent_keypair_path: &str,
) {
    // 1. Load parent keypair
    let parent_key_bytes = fs::read(parent_keypair_path)
        .expect("Failed to read parent keypair file");
    let parent_keypair = Keypair::from_bytes(&parent_key_bytes)
        .expect("Invalid parent keypair format");

    // 2. Timestamp
    let timestamp = Utc::now().to_rfc3339();

    // 3. Build instruction (without signature)
    let mut instr = RevokeHandleInstruction {
        handle: handle.to_string(),
        reason: reason.to_string(),
        timestamp: timestamp.clone(),
        parent_signature: "".to_string(),
    };

    // 4. Sign payload
    let payload = HandleRegistry::revoke_handle_payload_to_sign(&instr);
    let sig = parent_keypair.sign(payload.as_bytes());
    instr.parent_signature = base64::encode(sig.to_bytes());

    // 5. Output JSON
    let json = serde_json::to_string_pretty(&instr).unwrap();
    println!("{}", json);
}

fn resolve_handle_cli(handle: &str) {
    // This would typically query a node's handle registry
    // For now, just show the handle format
    println!("Resolving handle: {}", handle);
    println!("(This would query the network for handle information)");
    println!("Example output:");
    println!("{{");
    println!("  \"handle\": \"{}\"", handle);
    println!("  \"parent\": \"@parent.fd\"");
    println!("  \"public_key\": \"base64-pubkey\"");
    println!("  \"metadata\": {{ \"role\": \"trading desk\" }}");
    println!("  \"registered_at\": \"2024-06-01T12:34:56Z\"");
    println!("  \"revoked\": false");
    println!("}}");
}

fn list_children_cli(parent: &str) {
    // This would typically query a node's handle registry
    println!("Children of handle: {}", parent);
    println!("(This would query the network for child handles)");
    println!("Example output:");
    println!("[");
    println!("  \"@child1.{}\"", parent.trim_start_matches('@'));
    println!("  \"@child2.{}\"", parent.trim_start_matches('@'));
    println!("]");
}

// Utility function to generate a new keypair and save to file
pub fn generate_keypair(output_path: &str) -> Result<(), String> {
    let keypair = Keypair::generate(&mut rand::rngs::OsRng);
    let keypair_bytes = keypair.to_bytes();
    
    fs::write(output_path, keypair_bytes)
        .map_err(|e| format!("Failed to write keypair: {}", e))?;
    
    println!("Generated new keypair: {}", output_path);
    println!("Public key (base64): {}", base64::encode(keypair.public.to_bytes()));
    println!("Public key (hex): {}", hex::encode(keypair.public.to_bytes()));
    
    Ok(())
}

// Utility function to extract public key from keypair file
pub fn extract_pubkey(keypair_path: &str) -> Result<PublicKey, String> {
    let keypair_bytes = fs::read(keypair_path)
        .map_err(|e| format!("Failed to read keypair file: {}", e))?;
    
    let keypair = Keypair::from_bytes(&keypair_bytes)
        .map_err(|e| format!("Invalid keypair format: {}", e))?;
    
    Ok(keypair.public)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_generate_keypair() {
        let temp_file = NamedTempFile::new().unwrap();
        let result = generate_keypair(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());
        
        // Verify we can read it back
        let pubkey = extract_pubkey(temp_file.path().to_str().unwrap());
        assert!(pubkey.is_ok());
    }
} 