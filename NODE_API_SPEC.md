# FinDAG Node API Specification

This document specifies the API endpoints that the FinDAG wallet uses to interact with nodes.

## Base URL
```
http://localhost:8080
```

## Authentication
All endpoints require authentication via signed requests or API keys (TBD).

## Endpoints

### 1. Query Asset Balances

**GET** `/assets`

Query assets owned by a specific handle.

#### Query Parameters
- `owner` (required): The handle to query (e.g., `@johndoe.fd`)

#### Example Request
```bash
GET /assets?owner=@johndoe.fd
```

#### Example Response
```json
[
  {
    "asset_id": "ISIN-XYZ123",
    "amount": "1000000",
    "currency": "EUR",
    "unit": null,
    "issuer": "@deutschebank.fd",
    "metadata": {
      "maturity": "2030-12-31",
      "coupon_rate": "2.5%"
    }
  },
  {
    "asset_id": "GOLD-BAR-ABC",
    "amount": "1",
    "currency": null,
    "unit": "bar",
    "issuer": "@goldvault.fd",
    "metadata": {
      "purity": "99.99%",
      "weight": "400oz"
    }
  }
]
```

### 2. Submit Signed Instruction

**POST** `/submit_instruction`

Submit a signed instruction to the network.

#### Request Body
```json
{
  "instruction": {
    "type": "load_asset",
    "asset_id": "ISIN-XYZ123",
    "amount": "1000000",
    "currency": "EUR",
    "issuer": "@deutschebank.fd",
    "metadata": {
      "maturity": "2030-12-31"
    }
  },
  "signature": "a1b2c3d4...",
  "public_key": "e5f6g7h8..."
}
```

#### Example Response
```json
{
  "status": "accepted",
  "instruction_id": "abc123def456",
  "message": "Instruction submitted successfully"
}
```

### 3. Get Asset Details

**GET** `/assets/{asset_id}`

Get detailed information about a specific asset.

#### Example Request
```bash
GET /assets/ISIN-XYZ123
```

#### Example Response
```json
{
  "asset_id": "ISIN-XYZ123",
  "issuer": "@deutschebank.fd",
  "current_owner": "@johndoe.fd",
  "amount": "1000000",
  "currency": "EUR",
  "unit": null,
  "metadata": {
    "maturity": "2030-12-31",
    "coupon_rate": "2.5%",
    "face_value": "1000"
  },
  "created_at": "2024-01-15T10:30:00Z",
  "last_transferred": "2024-01-20T14:45:00Z"
}
```

### 4. Get Network Status

**GET** `/status`

Get current network status and node information.

#### Example Response
```json
{
  "node_id": "node-001",
  "version": "1.0.0",
  "network": "findag-mainnet",
  "status": "synced",
  "block_height": 12345,
  "peers": 15,
  "uptime": "7d 12h 30m"
}
```

### 5. Verify Ownership Proof

**POST** `/verify_ownership`

Verify a signed ownership proof.

#### Request Body
```json
{
  "instruction": {
    "type": "ownership_proof",
    "asset_id": "ISIN-XYZ123",
    "owner": "@johndoe.fd",
    "timestamp": 1705756800,
    "message": "I own ISIN-XYZ123 at timestamp 1705756800"
  },
  "signature": "a1b2c3d4...",
  "public_key": "e5f6g7h8..."
}
```

#### Example Response
```json
{
  "verified": true,
  "owner": "@johndoe.fd",
  "asset_id": "ISIN-XYZ123",
  "message": "Ownership verified successfully"
}
```

## Error Responses

All endpoints return standard HTTP status codes and error messages:

### 400 Bad Request
```json
{
  "error": "Invalid request format",
  "details": "Missing required field: owner"
}
```

### 401 Unauthorized
```json
{
  "error": "Authentication required",
  "details": "Invalid signature or missing credentials"
}
```

### 403 Forbidden
```json
{
  "error": "Permission denied",
  "details": "Handle @johndoe.fd not authorized for this operation"
}
```

### 404 Not Found
```json
{
  "error": "Asset not found",
  "details": "Asset ISIN-XYZ123 does not exist"
}
```

### 500 Internal Server Error
```json
{
  "error": "Internal server error",
  "details": "Database connection failed"
}
```

## Rate Limiting

- **Query endpoints**: 100 requests per minute per IP
- **Submit endpoints**: 10 requests per minute per handle
- **Verification endpoints**: 50 requests per minute per IP

## WebSocket Support (Future)

For real-time updates, WebSocket endpoints may be added:

```
ws://localhost:8080/ws
```

This would allow wallets to receive real-time notifications about:
- Asset transfers
- Balance changes
- Network status updates

## Implementation Notes

1. **CORS**: Enable CORS for web-based wallets
2. **HTTPS**: Use HTTPS in production
3. **Logging**: Log all API requests for audit purposes
4. **Validation**: Validate all input data and signatures
5. **Caching**: Cache frequently accessed data (asset details, balances) 