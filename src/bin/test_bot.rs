use ed25519_dalek::{Keypair, Signer};
use reqwest::Client;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::time::Duration;
use rand::rngs::OsRng;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1) Generate test keypair
    let mut csprng = OsRng {};
    let keypair = Keypair::generate(&mut csprng);

    // 2) Prepare transaction fields
    let from_pubkey = hex::encode(keypair.public.as_bytes());
    let to = "fdg1qalice1234567890";
    let amount = 123u64;
    let shard_id = 0u16;
    let findag_time = 29381272980000000u64; // Current FinDAG time
    
    // 3) Create payload (transaction data)
    let payload = format!("{}{}{}", from_pubkey, to, amount).into_bytes();
    
    // 4) Create hashtimer (32-byte array)
    let mut hashtimer = [0u8; 32];
    let mut hasher = Sha256::new();
    hasher.update(&payload);
    hasher.update(findag_time.to_be_bytes());
    let hash_result = hasher.finalize();
    hashtimer.copy_from_slice(&hash_result);

    // 5) Hash message for signing
    let mut hasher = Sha256::new();
    hasher.update(&payload);
    hasher.update(findag_time.to_be_bytes());
    hasher.update(&hashtimer);
    let msg_hash = hasher.finalize();

    // 6) Sign the hash
    let signature = keypair.sign(&msg_hash);
    let signature_bytes = signature.to_bytes().to_vec();
    let public_key_bytes = keypair.public.to_bytes().to_vec();

    println!("Bot sending TX: from={} to={} amount={} shard_id={} findag_time={}",
        from_pubkey, to, amount, shard_id, findag_time);
    println!("Signature hex: {}", hex::encode(&signature_bytes));
    println!("Public key hex: {}", hex::encode(&public_key_bytes));

    // 7) Build JSON payload with all required fields
    let tx = json!({
        "from": from_pubkey,
        "to": to,
        "amount": amount,
        "signature": signature_bytes,
        "payload": payload,
        "findag_time": findag_time,
        "hashtimer": hashtimer.to_vec(),
        "public_key": public_key_bytes,
        "shard_id": shard_id
    });

    // 8) Send to local API
    let client = Client::new();
    let resp = client.post("http://127.0.0.1:3000/tx")
        .timeout(Duration::from_secs(5))
        .json(&tx)
        .send()
        .await?;

    println!("Server response: {}", resp.status());
    let body = resp.text().await?;
    println!("Body: {}", body);

    Ok(())
} 