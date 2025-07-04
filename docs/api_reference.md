# FinDAG API Reference

**Version:** 1.1  
**Last Updated:** 2025-01-27

## Overview

This document provides a comprehensive reference for all FinDAG API endpoints, data structures, and integration patterns. The FinDAG API is designed for institutional-grade transparency and compliance, providing real-time access to all blockchain state and operations.

## Base URL

- **Development:** `http://localhost:8080`
- **Production:** Configure based on deployment

## Authentication

### JWT Authentication

Most administrative endpoints require JWT authentication. Include the token in the Authorization header:

```bash
Authorization: Bearer <your_jwt_token>
```

### Generating Test JWT

```python
import jwt
import time

token = jwt.encode({
    'sub': 'adminuser',
    'role': 'admin',
    'exp': int(time.time()) + 3600
}, 'changeme_jwt_secret', algorithm='HS256')
print(token)
```

## Core Endpoints

### Assets

#### Get All Assets
```http
GET /assets
```

Returns the current asset whitelist.

**Response:**
```json
{
  "assets": [
    {
      "code": "USD",
      "description": "US Dollar",
      "metadata": "Fiat currency"
    },
    {
      "code": "EUR", 
      "description": "Euro",
      "metadata": "Fiat currency"
    }
  ]
}
```

#### Get Asset Details
```http
GET /assets/{asset_id}
```

Returns detailed information about a specific asset.

**Response:**
```json
{
  "asset_id": "USD",
  "owner": "@bank.trading.fd",
  "status": "active",
  "amount": 1000000,
  "history": [
    {
      "timestamp": 1640995200000,
      "action": "load",
      "amount": 1000000,
      "from": "system"
    }
  ]
}
```

#### Get Assets by Owner
```http
GET /assets?owner={handle}
```

Returns all assets owned by a specific handle.

### Handles

#### Get Handle Information
```http
GET /handles/{handle}
```

Returns detailed information about a handle.

**Response:**
```json
{
  "handle": "@hsbc.london.trading.fd",
  "parent": "@hsbc.london.fd",
  "public_key": "base64_encoded_key",
  "key_history": [
    {
      "timestamp": 1640995200000,
      "public_key": "base64_encoded_key",
      "signature": "base64_encoded_signature"
    }
  ],
  "children": ["@hsbc.london.trading.desk1.fd"],
  "status": "active"
}
```

#### Get Handles by Parent
```http
GET /handles?parent={handle}
```

Returns all subhandles of a parent handle.

#### Register Subhandle
```http
POST /handles/register
Authorization: Bearer <admin_jwt>
Content-Type: application/json

{
  "parent_handle": "@hsbc.london.fd",
  "subhandle": "@hsbc.london.trading.fd",
  "public_key": "base64_encoded_key",
  "parent_signature": "base64_encoded_signature"
}
```

#### Rotate Handle Key
```http
POST /handles/{handle}/rotate-key
Authorization: Bearer <admin_jwt>
Content-Type: application/json

{
  "new_public_key": "base64_encoded_key",
  "signature": "base64_encoded_signature"
}
```

### Blocks

#### Get Block Information
```http
GET /blocks/{block_hash}
```

Returns detailed information about a block.

**Response:**
```json
{
  "block_id": "hash_hex",
  "parent_blocks": ["hash1", "hash2"],
  "transactions": [
    {
      "tx_id": "tx_hash",
      "from": "fdg1q...",
      "to": "fdg1q...",
      "amount": 1000,
      "asset": "USD"
    }
  ],
  "findag_time": 1640995200000,
  "hashtimer": "hashtimer_hex",
  "proposer": "fdg1q...",
  "signature": "base64_encoded_signature",
  "public_key": "base64_encoded_key",
  "shard_id": 0,
  "merkle_root": "merkle_root_hex"
}
```

#### Get Blocks by Round
```http
GET /blocks?round={round_number}
```

Returns all blocks in a specific round.

#### Get Merkle Proof
```http
GET /blocks/{block_hash}/merkle_proof/{tx_hash}
```

Returns a Merkle proof for a transaction in a block.

**Response:**
```json
{
  "block_hash": "block_hash_hex",
  "tx_hash": "tx_hash_hex",
  "merkle_root": "merkle_root_hex",
  "proof": ["hash1", "hash2", "hash3"],
  "proof_index": 2
}
```

### Rounds

#### Get Round Information
```http
GET /rounds/{round_number}
```

Returns detailed information about a round.

**Response:**
```json
{
  "round_number": 12345,
  "finalized_blocks": ["block1", "block2"],
  "committee": ["validator1", "validator2"],
  "signatures": [
    {
      "validator": "validator1",
      "signature": "base64_encoded_signature"
    }
  ],
  "timestamp": 1640995200000,
  "status": "finalized"
}
```

#### Get Rounds by Finalized Block
```http
GET /rounds?finalized={block_hash}
```

Returns all rounds that finalized a specific block.

### Transactions

#### Get Transaction Information
```http
GET /tx/{tx_id}
```

Returns detailed information about a transaction.

**Response:**
```json
{
  "tx_id": "tx_hash",
  "from": "fdg1q...",
  "to": "fdg1q...",
  "amount": 1000,
  "asset": "USD",
  "timestamp": 1640995200000,
  "block_hash": "block_hash",
  "round_number": 12345,
  "status": "finalized"
}
```

#### Get Transactions by Asset
```http
GET /tx?asset={asset_id}&from={timestamp}&to={timestamp}
```

Returns all transactions for a specific asset within a time range.

### Ownership

#### Get Ownership by Handle
```http
GET /ownership?handle={handle}
```

Returns all assets owned by a handle.

#### Get Ownership by Asset
```http
GET /ownership?asset={asset_id}
```

Returns the current owner of an asset.

### Validators

#### Get All Validators
```http
GET /validators
```

Returns the current validator set.

**Response:**
```json
{
  "validators": [
    {
      "address": "fdg1q...",
      "public_key": "base64_encoded_key",
      "metadata": "Primary validator",
      "status": "active",
      "reputation": 100
    }
  ]
}
```

#### Add Validator
```http
POST /validators
Authorization: Bearer <admin_jwt>
Content-Type: application/json

{
  "address": "fdg1q...",
  "public_key": "base64_encoded_key",
  "metadata": "New validator"
}
```

#### Remove Validator
```http
DELETE /validators/{address}
Authorization: Bearer <admin_jwt>
```

#### Slash Validator
```http
POST /validators/{address}/slash
Authorization: Bearer <admin_jwt>
Content-Type: application/json

{
  "reason": "Double signing detected"
}
```

### Governance

#### Get All Proposals
```http
GET /governance/proposals
```

Returns all governance proposals.

#### Submit Proposal
```http
POST /governance/proposals
Authorization: Bearer <admin_jwt>
Content-Type: application/json

{
  "proposer": "fdg1q...",
  "proposal_type": "add_asset",
  "code": "USDC",
  "description": "USD Coin",
  "metadata": "Stablecoin"
}
```

#### Vote on Proposal
```http
POST /governance/proposals/{proposal_id}/vote
Authorization: Bearer <admin_jwt>
Content-Type: application/json

{
  "voter": "fdg1q...",
  "vote": "approve"
}
```

### Metrics

#### Get Prometheus Metrics
```http
GET /metrics
```

Returns Prometheus-formatted metrics.

**Available Metrics:**
- `findag_tps`: Transactions per second
- `findag_per_asset_tps{asset=...}`: Per-asset TPS
- `findag_blocks_per_sec`: Blocks per second
- `findag_block_latency_seconds`: Block production latency
- `findag_round_latency_seconds`: Round checkpoint latency
- `findag_mempool_size`: Current mempool size
- `findag_api_calls{endpoint=...}`: API call count by endpoint
- `findag_error_count{type=...}`: Error count by type
- `findag_peer_count`: Current peer count

## Data Types

### Address
```json
{
  "type": "string",
  "format": "fdg1q...",
  "description": "FinDAG address in bech32 format"
}
```

### AssetRecord
```json
{
  "owner": "string",
  "status": "string",
  "amount": "number",
  "history": "AssetHistory[]"
}
```

### HandleRecord
```json
{
  "parent": "string",
  "public_key": "string",
  "key_history": "KeyHistory[]",
  "children": "string[]"
}
```

### Block
```json
{
  "block_id": "string",
  "parent_blocks": "string[]",
  "transactions": "Transaction[]",
  "findag_time": "number",
  "hashtimer": "string",
  "proposer": "string",
  "signature": "string",
  "public_key": "string",
  "shard_id": "number",
  "merkle_root": "string"
}
```

### Transaction
```json
{
  "tx_id": "string",
  "from": "string",
  "to": "string",
  "amount": "number",
  "asset": "string",
  "timestamp": "number",
  "signature": "string"
}
```

## Error Responses

All endpoints return consistent error responses:

```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "Human readable error message",
    "details": "Additional error details"
  }
}
```

### Common Error Codes

- `400`: Bad Request - Invalid parameters
- `401`: Unauthorized - Missing or invalid authentication
- `403`: Forbidden - Insufficient permissions
- `404`: Not Found - Resource not found
- `409`: Conflict - Resource already exists
- `422`: Unprocessable Entity - Validation error
- `500`: Internal Server Error - Server error

## Rate Limiting

API endpoints are rate limited to prevent abuse:

- **Read endpoints:** 1000 requests per minute
- **Write endpoints:** 100 requests per minute
- **Admin endpoints:** 50 requests per minute

Rate limit headers are included in responses:

```
X-RateLimit-Limit: 1000
X-RateLimit-Remaining: 999
X-RateLimit-Reset: 1640995260
```

## SDK Integration

### TypeScript SDK

```typescript
import { FinDAGClient } from '@findag/sdk';

const client = new FinDAGClient('http://localhost:8080');

// Get asset information
const asset = await client.getAsset('USD');
console.log(asset);

// Submit transaction
const tx = await client.submitTransaction({
  from: 'fdg1q...',
  to: 'fdg1q...',
  amount: 1000,
  asset: 'USD'
});

// Get Merkle proof
const proof = await client.getMerkleProof(blockHash, txHash);
const isValid = await client.verifyMerkleProof(proof);
```

### Python SDK

```python
from findag import FinDAGClient

client = FinDAGClient('http://localhost:8080')

# Get handle information
handle = client.get_handle('@hsbc.london.trading.fd')
print(handle)

# Submit governance proposal
proposal = client.submit_proposal({
    'proposer': 'fdg1q...',
    'proposal_type': 'add_asset',
    'code': 'USDC',
    'description': 'USD Coin'
})
```

## WebSocket API

For real-time updates, use the WebSocket API:

```javascript
const ws = new WebSocket('ws://localhost:8080/ws');

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  
  switch (data.type) {
    case 'new_block':
      console.log('New block:', data.block);
      break;
    case 'new_round':
      console.log('New round:', data.round);
      break;
    case 'transaction_finalized':
      console.log('Transaction finalized:', data.tx);
      break;
  }
};
```

## Best Practices

### Security
- Always use HTTPS in production
- Rotate JWT secrets regularly
- Implement proper rate limiting
- Validate all input data
- Use strong authentication for admin endpoints

### Performance
- Use pagination for large result sets
- Implement caching for frequently accessed data
- Use WebSocket for real-time updates
- Batch operations when possible

### Monitoring
- Monitor API response times
- Track error rates by endpoint
- Set up alerts for critical failures
- Log all administrative actions

### Compliance
- Maintain audit logs for all operations
- Implement data retention policies
- Ensure data privacy compliance
- Regular security audits

## Support

For API support and questions:

- **Documentation:** https://docs.findag.io
- **GitHub Issues:** https://github.com/findag/findag/issues
- **Community:** https://discord.gg/findag
- **Email:** api-support@findag.io

## Consensus Timing Parameters

### `round_interval_ms`
- **Description:** Interval (in milliseconds) at which a new Round is started and blocks are finalized.
- **Recommended:** 100–250 ms

### `block_production_interval_ms`
- **Description:** Interval (in milliseconds) at which new blocks are produced in the BlockDAG.
- **Recommended:** 10–50 ms

**Example:**
```toml
round_interval_ms = 200
block_production_interval_ms = 10
```
