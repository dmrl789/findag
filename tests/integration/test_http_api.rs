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
} 