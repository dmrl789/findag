# 👤 FinDAG User Guides

## Overview

This guide provides comprehensive instructions for FinDAG users, including wallet setup, API integration, governance participation, and best practices for secure and efficient usage.

---

## 🎯 User Categories

### **End Users**
- Individual users managing wallets and transactions
- Business users conducting financial operations
- Governance participants voting on proposals

### **Integrators**
- Developers building applications on FinDAG
- Financial institutions integrating with the platform
- Third-party service providers

### **Validators**
- Network validators participating in consensus
- Governance participants with voting power
- Infrastructure providers

---

## 💼 Wallet Management

### **Getting Started**

#### **Creating a Wallet**
```bash
# Using CLI wallet tool
cargo run --bin cli_wallet -- create-wallet

# Example output:
# Wallet created successfully!
# Address: 0x1234567890abcdef...
# Public Key: base64_encoded_public_key
# Backup your private key securely!
```

#### **Wallet Security Best Practices**
```bash
# 1. Generate strong passwords
openssl rand -base64 32

# 2. Use hardware wallets for large amounts
# 3. Keep private keys offline
# 4. Use multi-signature wallets
# 5. Regular security audits
```

### **Wallet Operations**

#### **Checking Balance**
```bash
# CLI method
cargo run --bin cli_wallet -- balance 0x1234567890abcdef...

# API method
curl http://localhost:8080/balance/0x1234567890abcdef.../USD

# Response:
{
  "address": "0x1234567890abcdef...",
  "asset": "USD",
  "balance": 1000000,
  "last_updated": "2025-01-01T12:00:00Z"
}
```

#### **Sending Transactions**
```bash
# CLI method
cargo run --bin cli_wallet -- send \
  --from 0x1234567890abcdef... \
  --to 0xfedcba0987654321... \
  --amount 100000 \
  --currency USD

# API method
curl -X POST http://localhost:8080/tx \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "from": "0x1234567890abcdef...",
    "to": "0xfedcba0987654321...",
    "amount": 100000,
    "currency": "USD",
    "shard_id": 0
  }'
```

#### **Transaction History**
```bash
# Get transaction history
curl -X GET "http://localhost:8080/transactions?address=0x1234567890abcdef..." \
  -H "Authorization: Bearer $JWT_TOKEN"

# Response:
{
  "transactions": [
    {
      "tx_hash": "0xabcd...",
      "from": "0x1234567890abcdef...",
      "to": "0xfedcba0987654321...",
      "amount": 100000,
      "currency": "USD",
      "status": "confirmed",
      "block_number": 12345,
      "timestamp": "2025-01-01T12:00:00Z"
    }
  ]
}
```

---

## 🔌 API Integration

### **Authentication**

#### **Getting API Access**
```bash
# 1. Register for API access
curl -X POST http://localhost:8080/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "your_username",
    "email": "your_email@example.com",
    "organization": "Your Company"
  }'

# 2. Login to get JWT token
curl -X POST http://localhost:8080/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "your_username",
    "password": "your_password"
  }'

# Response:
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_in": 86400,
  "role": "user"
}
```

#### **Using JWT Tokens**
```bash
# Include token in requests
curl -X GET http://localhost:8080/validators \
  -H "Authorization: Bearer $JWT_TOKEN"
```

### **Core API Operations**

#### **Transaction Management**
```bash
# Submit transaction
curl -X POST http://localhost:8080/tx \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "from": "0x1234567890abcdef...",
    "to": "0xfedcba0987654321...",
    "amount": 100000,
    "currency": "USD",
    "shard_id": 0,
    "signature": "base64_encoded_signature",
    "payload": "base64_encoded_payload",
    "findag_time": 1234567890,
    "hashtimer": "base64_encoded_hashtimer",
    "public_key": "base64_encoded_public_key"
  }'

# Get transaction status
curl -X GET http://localhost:8080/tx/0xabcd... \
  -H "Authorization: Bearer $JWT_TOKEN"
```

#### **Account Management**
```bash
# Get account balance
curl -X GET http://localhost:8080/balance/0x1234567890abcdef.../USD

# Get account history
curl -X GET "http://localhost:8080/account/0x1234567890abcdef.../history" \
  -H "Authorization: Bearer $JWT_TOKEN"

# Get account analytics
curl -X GET "http://localhost:8080/account/0x1234567890abcdef.../analytics" \
  -H "Authorization: Bearer $JWT_TOKEN"
```

### **SDK Integration**

#### **TypeScript/JavaScript SDK**
```typescript
import { FinDAGClient } from '@findag/sdk';

// Initialize client
const client = new FinDAGClient({
  baseUrl: 'https://api.findag.com',
  apiKey: 'your_api_key'
});

// Submit transaction
const tx = await client.submitTransaction({
  from: '0x1234567890abcdef...',
  to: '0xfedcba0987654321...',
  amount: 100000,
  currency: 'USD'
});

// Get balance
const balance = await client.getBalance('0x1234567890abcdef...', 'USD');

// Monitor transaction
const status = await client.getTransactionStatus(tx.tx_hash);
```

#### **Python SDK**
```python
from findag import FinDAGClient

# Initialize client
client = FinDAGClient(
    base_url="https://api.findag.com",
    api_key="your_api_key"
)

# Submit transaction
tx = client.submit_transaction(
    from_address="0x1234567890abcdef...",
    to_address="0xfedcba0987654321...",
    amount=100000,
    currency="USD"
)

# Get balance
balance = client.get_balance("0x1234567890abcdef...", "USD")
```

---

## 🏛️ Governance Participation

### **Understanding Governance**

#### **Governance Overview**
```bash
# Get governance statistics
curl http://localhost:8080/governance/stats

# Response:
{
  "total_proposals": 50,
  "active_proposals": 3,
  "passed_proposals": 45,
  "rejected_proposals": 2,
  "total_voters": 25,
  "total_stake": 100000000
}
```

#### **Proposal Types**
- **Parameter Changes**: Modify system parameters
- **Validator Management**: Add/remove validators
- **Protocol Upgrades**: System upgrades
- **Emergency Actions**: Emergency pause/resume

### **Participating in Governance**

#### **Viewing Proposals**
```bash
# List all proposals
curl http://localhost:8080/governance/proposals

# Get specific proposal
curl http://localhost:8080/governance/proposals/123

# Get proposal votes
curl http://localhost:8080/governance/proposals/123/votes
```

#### **Voting on Proposals**
```bash
# Submit vote
curl -X POST http://localhost:8080/governance/proposals/123/vote \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "voter": "0x1234567890abcdef...",
    "approve": true,
    "stake_weight": 1000000,
    "reason": "Supports the proposal"
  }'
```

#### **Submitting Proposals**
```bash
# Submit new proposal
curl -X POST http://localhost:8080/governance/proposals \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "proposer": "0x1234567890abcdef...",
    "title": "Increase Transaction Limit",
    "description": "Proposal to increase max transaction size",
    "proposal_type": "parameter_change",
    "parameter": "max_tx_size",
    "new_value": "1000000",
    "duration": 604800
  }'
```

### **Governance Analytics**

#### **Voting Analytics**
```bash
# Get governance analytics
curl http://localhost:8080/governance/analytics

# Get top voters
curl "http://localhost:8080/governance/top-voters?limit=10"

# Get governance events
curl "http://localhost:8080/governance/events?start_date=2025-01-01&end_date=2025-01-31"
```

---

## 🌉 Bridge Operations

### **Cross-Chain Transactions**

#### **Outbound Bridge**
```bash
# Bridge to external network
curl -X POST http://localhost:8080/bridge/outbound \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "source_network": "findag",
    "target_network": "ethereum",
    "amount": 1000000,
    "asset": "USD",
    "from_address": "0x1234567890abcdef...",
    "to_address": "0xfedcba0987654321...",
    "metadata": {}
  }'
```

#### **Inbound Bridge**
```bash
# Bridge from external network
curl -X POST http://localhost:8080/bridge/inbound \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "source_network": "ethereum",
    "target_network": "findag",
    "amount": 1000000,
    "asset": "USD",
    "from_address": "0x1234567890abcdef...",
    "to_address": "0xfedcba0987654321...",
    "proof": "base64_encoded_proof"
  }'
```

#### **Bridge Status**
```bash
# Check bridge transaction status
curl http://localhost:8080/bridge/status/0xabcd...
```

---

## 🔐 Security Best Practices

### **Wallet Security**

#### **Private Key Management**
```bash
# Generate secure private key
openssl rand -hex 32

# Store securely (hardware wallet recommended)
# Never share private keys
# Use multi-signature wallets for large amounts
# Regular security audits
```

#### **Transaction Security**
```bash
# Verify transaction details before signing
# Double-check addresses
# Use small amounts for testing
# Monitor for suspicious activity
```

### **API Security**

#### **Secure API Usage**
```bash
# Use HTTPS only
# Rotate API keys regularly
# Implement rate limiting
# Monitor API usage
# Use webhooks for real-time updates
```

#### **Error Handling**
```bash
# Implement proper error handling
# Log errors securely
# Retry with exponential backoff
# Validate all inputs
```

---

## 📊 Monitoring and Analytics

### **Transaction Monitoring**

#### **Real-time Monitoring**
```bash
# WebSocket connection for real-time updates
const ws = new WebSocket('wss://api.findag.com/ws');

ws.onmessage = function(event) {
  const data = JSON.parse(event.data);
  if (data.type === 'transaction') {
    console.log('New transaction:', data.transaction);
  }
};
```

#### **Analytics Dashboard**
```bash
# Get transaction analytics
curl -X GET "http://localhost:8080/analytics/transactions" \
  -H "Authorization: Bearer $JWT_TOKEN"

# Get performance metrics
curl -X GET "http://localhost:8080/analytics/performance" \
  -H "Authorization: Bearer $JWT_TOKEN"
```

### **System Health Monitoring**

#### **Health Checks**
```bash
# Check system health
curl http://localhost:8080/health

# Get detailed health information
curl http://localhost:8080/health/detailed

# Monitor system metrics
curl http://localhost:9090/metrics
```

---

## 🛠️ Troubleshooting

### **Common Issues**

#### **Transaction Failures**
```bash
# Check transaction status
curl -X GET http://localhost:8080/tx/0xabcd...

# Common causes:
# - Insufficient balance
# - Invalid address format
# - Network congestion
# - Invalid signature
```

#### **API Errors**
```bash
# Check error response
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid address format",
    "details": {
      "field": "address",
      "value": "invalid_address"
    }
  }
}

# Common error codes:
# - AUTHENTICATION_FAILED: Invalid token
# - AUTHORIZATION_FAILED: Insufficient permissions
# - VALIDATION_ERROR: Invalid request data
# - RATE_LIMIT_EXCEEDED: Too many requests
```

#### **Network Issues**
```bash
# Check network connectivity
curl -I https://api.findag.com

# Check peer connections
curl http://localhost:8080/validators

# Monitor network status
curl http://localhost:8080/network/status
```

### **Getting Help**

#### **Support Channels**
- **Documentation**: https://docs.findag.com
- **API Reference**: https://api.findag.com/docs
- **Community Forum**: https://community.findag.com
- **Support Email**: support@findag.com
- **Emergency**: +1-555-9999

#### **Debugging Information**
```bash
# Collect debugging information
curl -X GET http://localhost:8080/debug/info \
  -H "Authorization: Bearer $JWT_TOKEN"

# Get system logs (if authorized)
curl -X GET http://localhost:8080/admin/logs \
  -H "Authorization: Bearer $ADMIN_TOKEN"
```

---

## 📚 Advanced Topics

### **Multi-Signature Wallets**

#### **Creating Multi-Sig Wallet**
```bash
# Create multi-signature wallet
curl -X POST http://localhost:8080/wallet/multisig/create \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "owners": [
      "0x1234567890abcdef...",
      "0xfedcba0987654321...",
      "0x9876543210fedcba..."
    ],
    "required_signatures": 2
  }'
```

#### **Multi-Sig Transactions**
```bash
# Submit multi-sig transaction
curl -X POST http://localhost:8080/wallet/multisig/submit \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "wallet_address": "0xmultisig...",
    "to": "0xrecipient...",
    "amount": 1000000,
    "currency": "USD"
  }'

# Sign multi-sig transaction
curl -X POST http://localhost:8080/wallet/multisig/sign \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "transaction_id": "tx_id",
    "signature": "base64_encoded_signature"
  }'
```

### **Confidential Transactions**

#### **Creating Confidential Transaction**
```bash
# Submit confidential transaction
curl -X POST http://localhost:8080/confidential/tx \
  -H "Authorization: Bearer $JWT_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "encrypted_data": "base64_encoded_encrypted_data",
    "proof": "base64_encoded_zero_knowledge_proof",
    "public_inputs": {
      "from": "0x1234567890abcdef...",
      "to": "0xfedcba0987654321...",
      "amount": 1000000
    }
  }'
```

---

## 📖 Quick Reference

### **API Endpoints**
```bash
# Authentication
POST /auth/login
POST /auth/register

# Transactions
POST /tx
GET /tx/{tx_hash}
GET /transactions

# Balances
GET /balance/{address}/{asset}
GET /assets

# Governance
GET /governance/proposals
POST /governance/proposals
POST /governance/proposals/{id}/vote

# Bridge
POST /bridge/outbound
POST /bridge/inbound
GET /bridge/status/{tx_id}

# Health
GET /health
GET /health/detailed
```

### **Error Codes**
```bash
# Common error codes
AUTHENTICATION_FAILED - Invalid or missing token
AUTHORIZATION_FAILED - Insufficient permissions
VALIDATION_ERROR - Invalid request data
RATE_LIMIT_EXCEEDED - Too many requests
RESOURCE_NOT_FOUND - Requested resource not found
INTERNAL_ERROR - Server error
```

### **Rate Limits**
```bash
# API rate limits
100 requests per minute per client
1MB maximum request size
Rate limit headers included in responses
```

---

*This user guide should be updated regularly to reflect new features and improvements. Last updated: January 2025* 