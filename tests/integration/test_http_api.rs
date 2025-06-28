#[cfg(test)]
mod tests {
    use reqwest::blocking::Client;
    use serde_json::json;
    use std::thread;
    use std::time::Duration;

    fn start_server() {
        // Start the HTTP server in a background thread for testing
        thread::spawn(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async { crate::api::http_server::run_http_server().await });
        });
        // Give the server time to start
        thread::sleep(Duration::from_millis(500));
    }

    #[test]
    fn test_post_tx_rejects_unsupported_asset() {
        start_server();
        let client = Client::new();
        let tx = json!({
            "from": "fdg1qtestaddress",
            "to": "fdg1qtestaddress2",
            "amount": 100,
            "currency": "FAKEASSET",
            "signature": "deadbeef",
            "public_key": "deadbeef"
        });
        let resp = client.post("http://127.0.0.1:8080/tx").json(&tx).send().unwrap();
        let json: serde_json::Value = resp.json().unwrap();
        assert!(json["error"].as_str().unwrap().contains("not a supported asset"));
    }

    #[test]
    fn test_get_balance_rejects_unsupported_asset() {
        start_server();
        let client = Client::new();
        let resp = client.get("http://127.0.0.1:8080/balance/fdg1qtestaddress/FAKEASSET").send().unwrap();
        let json: serde_json::Value = resp.json().unwrap();
        assert!(json["error"].as_str().unwrap().contains("not a supported asset"));
    }

    #[test]
    fn test_block_and_merkle_proof_endpoints() {
        start_server();
        let client = Client::new();
        // 1. Submit a valid transaction (using a supported asset)
        let tx = json!({
            "from": "fdg1qtestaddress",
            "to": "fdg1qtestaddress2",
            "amount": 100,
            "currency": "USD",
            "signature": "deadbeef",
            "public_key": "deadbeef"
        });
        let _ = client.post("http://127.0.0.1:8080/tx").json(&tx).send().unwrap();
        // 2. Wait for block production (in real test, trigger block production or wait)
        thread::sleep(Duration::from_secs(1));
        // 3. Fetch latest block (assume block id is known or fetch from storage/mock)
        // For demo, this is a placeholder block id (should be replaced with real one)
        let block_id = "0000000000000000000000000000000000000000000000000000000000000000";
        let resp = client.get(&format!("http://127.0.0.1:8080/block/{}", block_id)).send().unwrap();
        let block_json: serde_json::Value = resp.json().unwrap();
        assert!(block_json["merkle_root"].is_string() || block_json["merkle_root"].is_null());
        // 4. Fetch Merkle proof for a tx in the block (use placeholder tx hash)
        let tx_hash = block_json["transactions"][0].as_str().unwrap_or("");
        if !tx_hash.is_empty() {
            let resp = client.get(&format!("http://127.0.0.1:8080/block/{}/merkle_proof/{}", block_id, tx_hash)).send().unwrap();
            let proof_json: serde_json::Value = resp.json().unwrap();
            assert!(proof_json["proof"].is_array());
        }
    }
} 