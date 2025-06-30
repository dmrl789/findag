# FinDAG Wallet Usage Examples

This document provides practical examples of how to use the FinDAG CLI wallet for different scenarios.

## 🏦 **Institutional User (Bank/Issuer)**

### 1. Generate a new wallet for Deutsche Bank
```bash
# Generate new keypair and set handle
findag-wallet keygen --handle "@deutschebank.fd"
# Enter password when prompted
```

**Output:**
```
✅ Key generated successfully!
Handle: @deutschebank.fd
Public Key: 8f7e6d5c4b3a2918...
Wallet file: wallet.dat
⚠️  Keep your password safe - it cannot be recovered!
```

### 2. Load a new bond asset
```bash
# Load a 10M EUR bond
findag-wallet load-asset \
  --asset-id "ISIN-DE0001135275" \
  --amount "10000000" \
  --currency "EUR" \
  --metadata '{"maturity":"2030-12-31","coupon_rate":"2.5%","face_value":"1000"}'
```

**Output (signed instruction):**
```json
{
  "instruction": {
    "type": "load_asset",
    "asset_id": "ISIN-DE0001135275",
    "amount": "10000000",
    "currency": "EUR",
    "issuer": "@deutschebank.fd",
    "metadata": {
      "maturity": "2030-12-31",
      "coupon_rate": "2.5%",
      "face_value": "1000"
    }
  },
  "signature": "a1b2c3d4e5f6...",
  "public_key": "8f7e6d5c4b3a2918..."
}
```

### 3. Transfer part of the bond to a client
```bash
# Transfer 1M EUR worth to client
findag-wallet transfer-asset \
  --asset-id "ISIN-DE0001135275" \
  --to "@client123.fd" \
  --amount "1000000"
```

### 4. Check current balances
```bash
findag-wallet balances
```

**Output:**
```
Querying balances for @deutschebank.fd...
Asset Balances:
Asset ID             Amount          Currency   Issuer
------------------------------------------------------
ISIN-DE0001135275    9000000         EUR        @deutschebank.fd
```

---

## 👤 **Private Trader**

### 1. Import existing key and set handle
```bash
# Import from backup file
findag-wallet import-key --file backup.key --handle "@johndoe.fd"
# Enter password when prompted
```

### 2. Check current asset holdings
```bash
findag-wallet balances
```

**Output:**
```
Querying balances for @johndoe.fd...
Asset Balances:
Asset ID             Amount          Currency   Issuer
------------------------------------------------------
ISIN-DE0001135275    1000000         EUR        @deutschebank.fd
GOLD-BAR-ABC         2               bar        @goldvault.fd
```

### 3. Transfer assets to another trader
```bash
# Transfer 500K EUR worth of bonds
findag-wallet transfer-asset \
  --asset-id "ISIN-DE0001135275" \
  --to "@trader456.fd" \
  --amount "500000"
```

### 4. Sign proof of ownership (for audit/verification)
```bash
findag-wallet sign-ownership --asset-id "ISIN-DE0001135275"
```

**Output:**
```json
{
  "instruction": {
    "type": "ownership_proof",
    "asset_id": "ISIN-DE0001135275",
    "owner": "@johndoe.fd",
    "timestamp": 1705756800,
    "message": "I own ISIN-DE0001135275 at timestamp 1705756800"
  },
  "signature": "f9e8d7c6b5a4...",
  "public_key": "1a2b3c4d5e6f..."
}
```

---

## 🔄 **Complete Workflow Example**

### Step 1: Issuer creates and loads asset
```bash
# Bank generates wallet
findag-wallet keygen --handle "@hsbc.fd"

# Bank loads a new asset
findag-wallet load-asset \
  --asset-id "ISIN-GB0001234567" \
  --amount "5000000" \
  --currency "GBP" \
  --metadata '{"maturity":"2025-06-30","coupon_rate":"3.0%"}'
```

### Step 2: Save signed instruction to file
```bash
# Save the output to a file
findag-wallet load-asset \
  --asset-id "ISIN-GB0001234567" \
  --amount "5000000" \
  --currency "GBP" \
  --metadata '{"maturity":"2025-06-30","coupon_rate":"3.0%"}' > load_instruction.json
```

### Step 3: Broadcast to network
```bash
# Submit to local node
findag-wallet broadcast --file load_instruction.json
```

**Output:**
```
Node response (200): {"status":"accepted","instruction_id":"abc123def456","message":"Instruction submitted successfully"}
```

### Step 4: Client receives and manages assets
```bash
# Client imports their key
findag-wallet import-key --file client.key --handle "@client789.fd"

# Check what they received
findag-wallet balances

# Transfer part to another client
findag-wallet transfer-asset \
  --asset-id "ISIN-GB0001234567" \
  --to "@client999.fd" \
  --amount "1000000" > transfer_instruction.json

# Broadcast the transfer
findag-wallet broadcast --file transfer_instruction.json
```

---

## 📋 **Advanced Usage**

### 1. Sign custom instruction from file
```bash
# Create custom instruction
cat > custom_instruction.json << EOF
{
  "type": "update_metadata",
  "asset_id": "ISIN-XYZ123",
  "updater": "@johndoe.fd",
  "metadata": {
    "status": "matured",
    "redemption_date": "2024-01-15"
  }
}
EOF

# Sign it
findag-wallet sign-instruction --file custom_instruction.json
```

### 2. Use different node URL
```bash
# Connect to different node
findag-wallet --node-url "http://192.168.1.44:8080" balances
```

### 3. Use different wallet file
```bash
# Use separate wallet for different purposes
findag-wallet --wallet-file "trading_wallet.dat" keygen --handle "@trading.fd"
```

---

## 🔒 **Security Best Practices**

### 1. Backup your private key
```bash
# Export private key (keep secure!)
findag-wallet export
```

### 2. Use strong passwords
```bash
# When prompted, use passwords like:
# "MyBank2024!Secure#Key"
# "Tr@d3r$3cur3P@ssw0rd!"
```

### 3. Verify instructions before signing
```bash
# Always review the JSON output before broadcasting
findag-wallet transfer-asset \
  --asset-id "ISIN-XYZ123" \
  --to "@recipient.fd" \
  --amount "1000000" | jq '.instruction'
```

### 4. Test on development network first
```bash
# Use test node for development
findag-wallet --node-url "http://test-node:8080" balances
```

---

## 🚨 **Troubleshooting**

### Common Issues

1. **"No wallet config found"**
   ```bash
   # Run keygen first
   findag-wallet keygen --handle "@yourhandle.fd"
   ```

2. **"Failed to query node"**
   ```bash
   # Check if node is running
   curl http://localhost:8080/status
   ```

3. **"Invalid metadata JSON"**
   ```bash
   # Use proper JSON format
   --metadata '{"key":"value","number":123}'
   ```

4. **"Failed to decrypt wallet"**
   ```bash
   # Double-check password
   # Ensure caps lock is off
   # Try typing in text editor first, then copy-paste
   ```

### Recovery Procedures

1. **Lost password**: Cannot recover - use backup private key
2. **Corrupted wallet**: Restore from backup file
3. **Wrong handle**: Update config file manually

---

## 📊 **Integration Examples**

### With Node.js application
```javascript
const { exec } = require('child_process');

// Query balances
exec('findag-wallet balances', (error, stdout, stderr) => {
  if (error) {
    console.error('Error:', error);
    return;
  }
  console.log('Balances:', stdout);
});
```

### With Python application
```python
import subprocess
import json

# Sign instruction
result = subprocess.run([
    'findag-wallet', 'transfer-asset',
    '--asset-id', 'ISIN-XYZ123',
    '--to', '@recipient.fd',
    '--amount', '1000000'
], capture_output=True, text=True)

signed_instruction = json.loads(result.stdout)
print(f"Signed instruction: {signed_instruction}")
```

This wallet provides a complete interface for managing FinDAG assets in a secure, permissioned environment! 