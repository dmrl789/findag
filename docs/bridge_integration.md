# FinDAG Bridge Integration Guide

**Version:** 1.0  
**Last Updated:** 2025-01-27

## Overview

The FinDAG Bridge system enables seamless integration with external blockchain networks through cryptographic proof verification. This guide covers implementation, deployment, and operational aspects of the bridge functionality.

## Architecture

### Bridge Module Structure

```
src/bridge/
├── mod.rs         # Module declarations and tests
├── proofs.rs      # SettlementProof trait (generic interface)
├── corda.rs       # Corda blockchain integration
├── fabric.rs      # Hyperledger Fabric integration
└── api.rs         # HTTP API endpoints
```

### Core Components

1. **SettlementProof Trait**: Generic interface for cross-chain proofs
2. **Blockchain-Specific Implementations**: Corda and Fabric proof structures
3. **HTTP API**: RESTful endpoints for proof submission
4. **Verification Engine**: Cryptographic signature verification
5. **Storage Integration**: Mempool and block header integration

## Supported Blockchains

### Corda Integration

**Proof Structure:**
```rust
pub struct CordaSettlementProof {
    pub state_hash: Vec<u8>,        // Corda state hash
    pub notary_signature: Vec<u8>,  // Ed25519 notary signature
}
```

**Verification:**
- Ed25519 signature verification using trusted notary public keys
- State hash validation and integrity checks
- Timestamp validation for replay protection

**API Endpoint:**
```http
POST /bridge/corda/submit
Content-Type: application/json

{
  "state_hash": "01020304AABBCCDD",
  "notary_signature": "99AABBCCDDEEFF"
}
```

### Hyperledger Fabric Integration

**Proof Structure:**
```rust
pub struct FabricEndorsementProof {
    pub state_root: Vec<u8>,              // Fabric state root
    pub endorsement_signatures: Vec<Vec<u8>>, // Multiple peer signatures
}
```

**Verification:**
- ECDSA signature verification using MSP certificates
- Multiple endorsement signature validation
- State root integrity verification

**API Endpoint:**
```http
POST /bridge/fabric/submit
Content-Type: application/json

{
  "state_root": "05060708AABBCCDD",
  "endorsement_signatures": [
    "AABBCCDD11",
    "1122334455"
  ]
}
```

## Implementation Guide

### 1. Cryptographic Verification

**Corda (Ed25519):**
```rust
use ed25519_dalek::{PublicKey, Signature, Verifier};

impl SettlementProof for CordaSettlementProof {
    fn verify(&self, trusted_pubkey: &[u8]) -> Result<()> {
        let pubkey = PublicKey::from_bytes(trusted_pubkey)?;
        let signature = Signature::from_bytes(&self.notary_signature)?;
        pubkey.verify(&self.state_hash, &signature)?;
        Ok(())
    }
}
```

**Fabric (ECDSA):**
```rust
use ring::signature::{UnparsedPublicKey, ECDSA_P256_SHA256_ASN1};

impl SettlementProof for FabricEndorsementProof {
    fn verify(&self, trusted_msp_pubkeys: &[Vec<u8>]) -> Result<()> {
        for (sig, pubkey_bytes) in self.endorsement_signatures.iter().zip(trusted_msp_pubkeys) {
            let pubkey = UnparsedPublicKey::new(&ECDSA_P256_SHA256_ASN1, pubkey_bytes);
            pubkey.verify(&self.state_root, sig)?;
        }
        Ok(())
    }
}
```

### 2. Configuration Management

**Trusted Keys Configuration:**
```toml
# config/bridge.toml
[corda]
notary_pubkeys = [
    "base64_or_hex_pubkey_1",
    "base64_or_hex_pubkey_2"
]

[fabric]
msp_pubkeys = [
    "base64_or_hex_pubkey_1",
    "base64_or_hex_pubkey_2"
]

[security]
rate_limit_per_minute = 100
audit_log_enabled = true
timestamp_tolerance_seconds = 300
```

### 3. Storage Integration

**Mempool Integration:**
```rust
pub struct BridgeTx {
    pub proof_type: String,     // "corda" or "fabric"
    pub state_hash: Vec<u8>,    // Original state hash/root
    pub raw_proof: Vec<u8>,     // Serialized proof
    pub timestamp: u64,         // Submission timestamp
    pub relayer_id: String,     // Relayer identifier
}
```

**Block Header Integration:**
```rust
pub struct Block {
    // ... existing fields ...
    pub bridge_proofs: Vec<BridgeProof>, // Bridge proofs included in block
}

pub struct BridgeProof {
    pub proof_type: String,
    pub state_hash: Vec<u8>,
    pub proof_hash: Vec<u8>,    // Hash of the original proof
    pub timestamp: u64,
}
```

## Deployment Guide

### 1. Production Setup

**HTTPS Configuration:**
```bash
# Generate TLS certificates
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes

# Set environment variables
export TLS_CERT_PATH=/path/to/cert.pem
export TLS_KEY_PATH=/path/to/key.pem
```

**Trusted Keys Deployment:**
```bash
# Load trusted keys from secure storage
export CORDA_NOTARY_PUBKEYS="key1,key2,key3"
export FABRIC_MSP_PUBKEYS="cert1,cert2,cert3"
```

### 2. Monitoring and Metrics

**Prometheus Metrics:**
```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'findag_bridge'
    static_configs:
      - targets: ['localhost:9898']
    metrics_path: '/metrics'
```

**Available Metrics:**
- `findag_bridge_proofs_total{type="corda"}`: Total Corda proofs processed
- `findag_bridge_proofs_total{type="fabric"}`: Total Fabric proofs processed
- `findag_bridge_verification_duration_seconds`: Proof verification latency
- `findag_bridge_errors_total{type="verification"}`: Verification errors

### 3. Security Best Practices

**Key Management:**
- Store trusted keys in secure key management systems (HashiCorp Vault, AWS KMS)
- Rotate keys regularly and maintain key versioning
- Use hardware security modules (HSM) for production deployments

**Network Security:**
- Implement IP whitelisting for bridge endpoints
- Use mutual TLS (mTLS) for relayer authentication
- Monitor and log all bridge operations

**Audit and Compliance:**
- Log all proof submissions with timestamps and relayer information
- Maintain audit trails for regulatory compliance
- Implement alerting for suspicious bridge activity

## Relayer Integration

### 1. Corda Relayer

**Example Rust Relayer:**
```rust
use reqwest::Client;
use serde_json::json;

pub async fn submit_corda_proof(
    state_hash: &[u8],
    notary_signature: &[u8],
    findag_url: &str,
) -> Result<String> {
    let client = Client::new();
    let payload = json!({
        "state_hash": hex::encode(state_hash),
        "notary_signature": hex::encode(notary_signature)
    });

    let response = client
        .post(&format!("{}/bridge/corda/submit", findag_url))
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        Ok("Proof accepted".to_string())
    } else {
        Err(anyhow!("Proof rejected: {}", response.text().await?))
    }
}
```

### 2. Fabric Relayer

**Example Java Relayer:**
```java
import okhttp3.*;
import com.fasterxml.jackson.databind.ObjectMapper;

public class FabricRelayer {
    private final OkHttpClient client = new OkHttpClient();
    private final ObjectMapper mapper = new ObjectMapper();

    public String submitFabricProof(byte[] stateRoot, List<byte[]> endorsements, String findagUrl) throws Exception {
        Map<String, Object> payload = new HashMap<>();
        payload.put("state_root", Hex.encodeHexString(stateRoot));
        payload.put("endorsement_signatures", endorsements.stream()
            .map(Hex::encodeHexString)
            .collect(Collectors.toList()));

        String json = mapper.writeValueAsString(payload);
        RequestBody body = RequestBody.create(json, MediaType.get("application/json"));

        Request request = new Request.Builder()
            .url(findagUrl + "/bridge/fabric/submit")
            .post(body)
            .build();

        try (Response response = client.newCall(request).execute()) {
            if (response.isSuccessful()) {
                return "Proof accepted";
            } else {
                throw new RuntimeException("Proof rejected: " + response.body().string());
            }
        }
    }
}
```

## Testing

### 1. Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corda_proof_verification() {
        let proof = CordaSettlementProof {
            state_hash: vec![1, 2, 3, 4],
            notary_signature: vec![9, 9, 9],
        };
        let trusted_key = vec![0u8; 32];
        assert!(proof.verify(&trusted_key).is_ok());
    }

    #[test]
    fn test_fabric_proof_verification() {
        let proof = FabricEndorsementProof {
            state_root: vec![5, 6, 7, 8],
            endorsement_signatures: vec![vec![8, 8, 8]],
        };
        let trusted_keys = vec![vec![1u8; 32]];
        assert!(proof.verify(&trusted_keys).is_ok());
    }
}
```

### 2. Integration Tests

```rust
#[tokio::test]
async fn test_bridge_api_endpoints() {
    // Test Corda proof submission
    let corda_payload = json!({
        "state_hash": "01020304AABBCCDD",
        "notary_signature": "99AABBCCDDEEFF"
    });

    let response = client
        .post("http://localhost:8080/bridge/corda/submit")
        .json(&corda_payload)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
```

## Troubleshooting

### Common Issues

1. **Signature Verification Failures:**
   - Verify trusted keys are correctly configured
   - Check signature format (hex vs base64)
   - Ensure timestamp tolerance is appropriate

2. **Rate Limiting:**
   - Monitor rate limit configuration
   - Implement exponential backoff in relayers
   - Consider increasing limits for high-volume deployments

3. **Network Connectivity:**
   - Verify HTTPS certificates are valid
   - Check firewall rules for bridge endpoints
   - Monitor network latency and timeouts

### Debug Commands

```bash
# Check bridge metrics
curl http://localhost:9898/metrics | grep bridge

# Test bridge endpoint
curl -X POST http://localhost:8080/bridge/corda/submit \
  -H "Content-Type: application/json" \
  -d '{"state_hash":"01020304","notary_signature":"99AABBCC"}'

# View bridge logs
tail -f /var/log/findag/bridge.log
```

## Future Enhancements

1. **Additional Blockchain Support:**
   - Ethereum/EVM compatibility
   - Polkadot parachain integration
   - Cosmos IBC support

2. **Advanced Features:**
   - Multi-hop bridge routing
   - Cross-chain atomic swaps
   - Bridge liquidity pools

3. **Performance Optimizations:**
   - Batch proof processing
   - Parallel verification
   - Caching and optimization

## Support

For technical support and questions about bridge integration:

- **Documentation**: Check this guide and API reference
- **Issues**: Report bugs and feature requests through GitHub
- **Community**: Join the FinDAG community for discussions and updates
