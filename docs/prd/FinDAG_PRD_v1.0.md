# FinDAG Product Requirements Document (PRD)

**Product Name:** FinDAG  
**Version:** 1.1  
**Owner:** DMRL789 LLC  
**Last Updated:** 2025-01-27

---

## 1. Overview

FinDAG is a high-performance, low-latency, deterministic blockchain system purpose-built for financial applications. **It is implemented in Rust for maximum safety, concurrency, and performance.** Leveraging a dual-layer Directed Acyclic Graph (DAG) structure for both Blocks and Rounds, FinDAG achieves near real-time transaction finality, microsecond audit precision, and multi-million transactions per second (TPS) throughput.

- **No fees or rewards**
- **Ed25519** for address signatures
- **Automatic node discovery**
- **Authorized validation**: Only authorized nodes can validate blocks and rounds
- **Persistent storage**: Sled-based crash-safe database for all state
- **Hierarchical handles**: Institutional-grade identity management system

---

## 2. Goals

- Achieve sub-second finality for all financial transactions
- Support 1M–10M TPS throughput without compromising ordering determinism
- Ensure regulatory compliance with precision timestamping
- Prevent front-running and improve fairness in high-frequency financial markets
- Provide institutional-grade transparency and auditability

---

## 3. System Design Summary

| Parameter           | Value                        |
|---------------------|-----------------------------|
| Block frequency     | 10–50 ms                    |
| Round frequency     | 100–250 ms                  |
| Finality            | < 500 ms                    |
| Block size          | 4–32 KB (128 KB max)        |
| Transaction size    | ~250 bytes                  |
| Throughput goal     | 1M–10M TPS                  |
| Ordering mechanism  | HashTimer (FinDAG Time + hash) |
| Storage             | Sled embedded database       |
| Identity system     | Hierarchical handles         |

---

## 4. Key Features

### 4.1 FinDAG Time & HashTimer

- Precise time synchronization with 100ns resolution
- Timestamp-based ordering for auditability and manipulation prevention
- **HashTimer is a three-part hash:**
  - FinDAG Time (u64) for chronological ordering
  - Content hash (serialized parent blocks) for deterministic structure
  - Nonce (u32) for uniqueness and distribution
  - Provides precision up to 1/10 of a microsecond

### 4.2 DAG-Based Block & Round Design

- Blocks form a DAG within micro-intervals (10–50ms)
- Rounds serve as checkpoint layers (~100–250ms) to finalize DAG paths
- Enables parallel block inclusion with global ordering
- Persistent storage of complete DAG structure

### 4.3 Finality in <500ms

- Deterministic finality within half a second
- Suitable for real-time financial systems and trading environments
- Eliminates ambiguity for settlement or regulatory audits

### 4.4 Skip-When-Empty Optimization

- Blocks omitted if no transactions are pending
- Maintains consistent timing without wasting network resources
- Dynamically adjusts to transaction flow

### 4.5 High Throughput

- Block size of 4–32KB (128KB max) for fast propagation
- ~250-byte transaction size allows 100+ tx per block
- Scales to millions of TPS across hundreds or thousands of nodes

### 4.6 Sharded In-RAM Transaction Pool

- The transaction pool (mempool) is fully in-memory and sharded for high throughput.
- Supports millions of transactions per second (TPS) by parallelizing across CPU cores.
- No disk persistence for the pool; only finalized blocks/rounds are persisted.
- Ensures low latency, deduplication, and prioritization by FinDAG Time.

### 4.7 Multi-Node Network Propagation

- FinDAG nodes support multi-node operation, each running on a unique network port and identified by a unique validator ID.
- Nodes connect to each other using a static peer list and propagate transactions, blocks, and rounds using a UDP-based gossip protocol.
- The network layer ensures all validator nodes receive and relay new data, enabling distributed consensus and high-throughput operation.
- The system is designed for easy multi-node testing and deployment, supporting both local and distributed setups.

### 4.8 Asset Whitelist Enforcement

- Only whitelisted assets are supported for all transactions and balance queries in both the CLI wallet and HTTP API.
- Attempting to send, receive, or query unsupported assets will result in an error.
- The current list of supported assets is:

  EUR, USD, GBP, JPY, CHF, SGD, AED, CNY, BUND, OAT, BTP, GILT, UST, JGB, T-BILL, CP, CD, XAU, XAG, XPT, XPD, XS1234567890, FR0000120271, BE0003796134, DE0001135275, ETF1, UCITS1, BTC, ETH, USDT, USDC

- This list is enforced at the mempool, API, and wallet layers for security and compliance.

### 4.9 Validator Management and Monitoring

- **Advanced Monitoring:**
  - FinDAG nodes export real-time Prometheus metrics at `/metrics` (default port 9898).
  - Metrics include: Transactions per second (TPS), blocks per second, mempool size, peer count, and totals.
  - Metrics are suitable for integration with Prometheus, Grafana, and alerting systems.

- **Dynamic Validator Set:**
  - Validators are managed as a persistent, runtime-mutable set.
  - The validator set is loaded from storage on startup and can be changed at runtime (add, remove, slash).
  - All changes are immediately persisted and reflected in consensus logic.
  - Only active validators are eligible for block/round production and validation.

- **Validator Management HTTP API:**
  - Operators can manage validators via the HTTP API:
    - `GET /validators` — List all validators
    - `POST /validators` — Add a validator (address, public key, metadata, admin token)
    - `DELETE /validators/:address` — Remove a validator (admin token)
    - `POST /validators/:address/slash` — Slash a validator (admin token)
  - All endpoints require an admin token for security (replace with robust auth for production).
  - All changes are persisted and consensus logic is updated live.

- **Consensus Integration:**
  - The consensus engine (e.g., round finalization, block validation) always uses the current, persistent validator set.
  - Only active validators are considered for assignment and signature verification.

- **Documentation Roadmap:**
  - All future enhancements to monitoring, validator management, and consensus will be documented in this section.
  - This includes governance, on-chain voting, advanced metrics, and operational best practices.

### 4.10 API Security and Access Control

- **JWT-Based Authentication:**
  - All sensitive HTTP API endpoints (validator management, governance) require a valid JWT in the `Authorization: Bearer <jwt>` header.
  - JWTs must include a `role` claim (e.g., `admin`, `validator`, `observer`). Only `admin` can perform management actions.
  - The JWT secret is hardcoded for demo; replace with a secure secret in production.

- **Generating a Test Admin JWT:**
  - Use the following Python snippet to generate a test token (replace the secret as needed):
    ```python
    import jwt, time
    token = jwt.encode({
        'sub': 'adminuser',
        'role': 'admin',
        'exp': int(time.time()) + 3600
    }, 'changeme_jwt_secret', algorithm='HS256')
    print(token)
    ```
  - Use the output as the Bearer token in API requests.

- **Endpoint Usage Example:**
  - To add a validator:
    ```bash
    curl -X POST http://127.0.0.1:8080/validators \
      -H "Authorization: Bearer <your_admin_jwt>" \
      -H "Content-Type: application/json" \
      -d '{"address": "fdg1q...", "public_key": "...", "metadata": "Test"}'
    ```
  - To submit a governance proposal:
    ```bash
    curl -X POST http://127.0.0.1:8080/governance/proposals \
      -H "Authorization: Bearer <your_admin_jwt>" \
      -H "Content-Type: application/json" \
      -d '{"proposer": "fdg1q...", "proposal_type": "add", "address": "fdg1q...", "public_key": "...", "metadata": "Test"}'
    ```

- **Production Note:**
  - Always use a strong, secret key for JWT signing in production.
  - Consider integrating with enterprise auth (OAuth2, mTLS, etc.) for advanced deployments.

### 4.11 Advanced Monitoring and Grafana Dashboard

- **Prometheus Setup:**
  - Add the following scrape config to your `prometheus.yml`:
    ```yaml
    scrape_configs:
      - job_name: 'findag_node'
        static_configs:
          - targets: ['localhost:9898']
    ```
  - Start Prometheus and ensure it is scraping the FinDAG node metrics endpoint.

- **Grafana Dashboard:**
  - Import the provided `findag_grafana_dashboard.json` into Grafana (Dashboards → Import).
  - Select your Prometheus data source.
  - The dashboard includes panels for TPS, per-asset TPS, block/round latency, mempool size, API calls, and error rates.

- **Available Metrics:**
  - `findag_tps`: Transactions per second
  - `findag_per_asset_tps{asset=...}`: Per-asset TPS
  - `findag_blocks_per_sec`: Blocks per second
  - `findag_block_latency_seconds`: Block production latency (histogram)
  - `findag_round_latency_seconds`: Round checkpoint latency (histogram)
  - `findag_mempool_size`: Current mempool size
  - `findag_api_calls{endpoint=...}`: API call count by endpoint
  - `findag_error_count{type=...}`: Error count by type
  - `findag_peer_count`: Current peer count

- **Alerting:**
  - You can add Prometheus alert rules for conditions such as node offline, high error rate, or mempool full.

- **Operational Best Practices:**
  - Monitor all panels for anomalies or performance drops.
  - Set up alerts for critical metrics to ensure high availability and rapid incident response.

### 4.12 Deployment and Operations

- **Docker Deployment:**
  - Build the Docker image:
    ```sh
    docker build -t findag-node .
    ```
  - Run a single node:
    ```sh
    docker run -p 8080:8080 -p 9898:9898 -p 9000:9000 findag-node
    ```

- **Multi-Node Local Cluster (docker-compose):**
  - Use the provided `docker-compose.yml` to launch a 2-node cluster:
    ```sh
    docker-compose up --build
    ```
  - Node 1 API: http://localhost:8081
  - Node 2 API: http://localhost:8082
  - Metrics: http://localhost:9891, http://localhost:9892

- **Operational Notes:**
  - Each node can be configured via environment variables (e.g., `NODE_ID`, `PEERS`).
  - For production, mount persistent volumes for data directories.
  - Expose or secure ports as needed for your environment.

- **Extending to Systemd or Kubernetes:**
  - For Linux services, create a `systemd` unit file pointing to the node binary.
  - For Kubernetes, create a StatefulSet or Deployment using the Docker image.

- **Best Practices:**
  - Monitor all nodes with Prometheus/Grafana.
  - Use secure secrets and authentication in production.
  - Automate deployment and upgrades for reliability.

### 4.13 Compliance and Auditability

- **Audit Logging:**
  - All sensitive actions (validator changes, governance, API calls) are logged with timestamp, user, and action details in a persistent audit log (`audit.log`).
  - Logs can be exported and reviewed for compliance and regulatory audits.

- **Data Retention:**
  - Persistent storage of all blocks, rounds, state, and validator/governance history.
  - Operators should implement regular backups and define retention policies according to regulatory requirements.

- **Traceability:**
  - Every transaction, block, and round is timestamped and cryptographically signed.
  - All state changes are auditable and can be reconstructed from logs and persistent storage.

- **Regulatory Readiness:**
  - FinDAG supports microsecond-precision audit trails and deterministic ordering for full auditability.
  - Designed to help meet ISO/IEC 20022, MiFID II, and other financial compliance standards.

- **Best Practices for Operators:**
  - Regularly export and review audit logs for anomalies or unauthorized actions.
  - Monitor all compliance-relevant metrics and logs with Prometheus/Grafana and alerting.
  - Automate compliance procedures and document operational runbooks for audits.

### 4.14 Dynamic Asset Governance

- **Governance-Controlled Asset Whitelist:**
  - The list of supported assets is managed via on-chain governance proposals (AddAsset, RemoveAsset).
  - Any validator or authorized user can submit a proposal to add or remove an asset (with code, description, and metadata).
  - Proposals are voted on by validators; if approved, the asset whitelist is updated and persisted.

- **Enforcement:**
  - The mempool, API, and CLI wallet all use the dynamic asset whitelist from persistent storage.
  - Only assets in the current whitelist can be used for transactions, queries, or wallet operations.
  - The `/assets` API endpoint returns the current whitelist for clients and users.

- **User & Operator Workflow:**
  - To propose a new asset, submit a governance proposal via the API (see governance section).
  - To check which assets are currently supported, query the `/assets` endpoint or use the CLI wallet (which fetches the list before sending or querying balances).
  - All changes are auditable and logged for compliance.

- **Example:**
  - Add a new asset:
    ```bash
    curl -X POST http://localhost:8080/governance/proposals \
      -H "Authorization: Bearer <admin_jwt>" \
      -H "Content-Type: application/json" \
      -d '{"proposer": "fdg1q...", "proposal_type": "add_asset", "code": "USDC", "description": "USD Coin", "metadata": "Stablecoin"}'
    ```
  - Check the current whitelist:
    ```bash
    curl http://localhost:8080/assets
    ```

### 4.15 Merkle Proofs and Block Integrity

- **Merkle Root in Block Header:**
  - Every block includes a Merkle root of all transaction hashes, enabling cryptographic inclusion proofs.
- **API Endpoints:**
  - `GET /block/:id` — Returns block info, Merkle root, and transaction hashes.
  - `GET /block/:id/merkle_proof/:tx_hash` — Returns a Merkle proof for a transaction in the block.
- **SDK Support:**
  - TypeScript SDK provides methods to fetch blocks, fetch Merkle proofs, and verify proofs (Node.js and browser support planned).
- **Use Cases:**
  - Wallets, bridges, and auditors can verify that a transaction is included in a block without trusting the node.
- **Browser Compatibility:**
  - Node.js SDK supports proof verification; browser support is planned using Web Crypto APIs.
- **Documentation and Examples:**
  - See SDK and API docs for example usage and integration flows.

### 4.16 Hierarchical Handle System & Identity Management

- **Hierarchical Handle Structure:**
  - Handles follow a hierarchical, dot-separated format: `@organization.location.department.fd`
  - Examples: `@hsbc.london.trading.fd`, `@ubs.zurich.compliance.fd`
  - Parent handles control subhandle registration, key rotation, and revocation
  - All handle operations are on-chain, auditable, and signed

- **Handle Operations:**
  - **Register Subhandle:** Parent signs registration of new subhandle with new public key
  - **Rotate Key:** Handle owner signs key rotation, maintaining key history
  - **Revoke Handle:** Parent signs revocation with reason and timestamp
  - **Resolve Handle:** Query current handle info, key history, and children

- **Key Management & Audit Trail:**
  - Complete key history maintained on-chain with timestamps
  - Each key rotation is signed and immutable
  - Revoked handles maintain audit trail with revocation reason
  - All operations include cryptographic signatures for non-repudiation

- **CLI Wallet Integration:**
  - `findag-handle-wallet` binary for handle management
  - Commands: `register-subhandle`, `rotate-key`, `revoke-handle`, `resolve`, `list-children`
  - Generates signed JSON instructions for on-chain processing
  - Supports metadata and hierarchical relationships

### 4.17 Persistent Storage & Institutional Transparency

- **Sled-Based Persistent Storage:**
  - Embedded, crash-safe database for all blockchain state
  - No external dependencies, pure Rust implementation
  - Supports key-value storage with range queries for efficient searching

- **Stored Data Types:**
  - **Blocks & Rounds:** Complete DAG structure with signatures and timestamps
  - **Asset State:** Current ownership, balances, and complete transaction history
  - **Handle Registry:** All handle records, key history, and hierarchical relationships
  - **Validator Set:** Current validators, status, and metadata
  - **Governance State:** Proposals, votes, and governance history

- **Data Schema:**
  - **Assets:** `asset:{asset_id}` → AssetRecord (owner, status, amount, history)
  - **Handles:** `handle:{handle}` → HandleRecord (parent, pubkey, key_history, children)
  - **Blocks:** `block:{hash}` → SerializableBlock (instructions, signatures, timestamp)
  - **Rounds:** `round:{number}` → SerializableRound (finalized_blocks, committee, signatures)

- **Institutional-Grade Auditability:**
  - Every node maintains complete local copy of all state
  - All data is queryable via HTTP API for real-time transparency
  - Immutable audit trail with cryptographic proofs
  - Support for regulatory compliance and external audits

### 4.18 HTTP API for Transparency & Integration

- **RESTful API Endpoints:**
  - **Assets:** `GET /assets/{asset_id}`, `GET /assets?owner={handle}`
  - **Handles:** `GET /handles/{handle}`, `GET /handles?parent={handle}`
  - **Blocks:** `GET /blocks/{block_hash}`, `GET /blocks?round={number}`
  - **Rounds:** `GET /rounds/{round_number}`, `GET /rounds?finalized={block_hash}`
  - **Ownership:** `GET /ownership?handle={handle}`, `GET /ownership?asset={asset_id}`
  - **Transactions:** `GET /tx/{tx_id}`, `GET /tx?asset={asset_id}&from={timestamp}&to={timestamp}`

- **API Response Format:**
  - All responses in JSON format for easy integration
  - Include complete audit trails and cryptographic proofs
  - Support for filtering, pagination, and time-range queries
  - Error responses with detailed error codes and messages

- **Use Cases:**
  - **CLI Wallet Integration:** Query balances, resolve handles, verify transactions
  - **Block Explorer:** Real-time blockchain state visualization
  - **Auditor Tools:** Export proofs, verify finality, trace asset ownership
  - **Regulatory Reporting:** Automated compliance reporting and monitoring
  - **Client Integration:** Embed blockchain data in trading systems and dashboards

- **Security & Access Control:**
  - Read-only endpoints for transparency (no authentication required)
  - Administrative endpoints require JWT authentication
  - Rate limiting and request validation
  - CORS support for web-based explorers

### 4.19 Block Explorer & Transparency Interface

- **Web-Based Block Explorer:**
  - Real-time visualization of FinDAG state
  - Search by handle, asset ID, block hash, or transaction ID
  - DAG visualization showing block and round relationships
  - Asset ownership tracking and history visualization

- **Explorer Features:**
  - **Dashboard:** Latest blocks, rounds, TPS, and network status
  - **Search:** Multi-criteria search across all data types
  - **Visualization:** Interactive DAG graphs and timeline views
  - **Export:** Download proofs, transaction history, and audit reports
  - **API Integration:** Direct access to underlying HTTP API

- **Institutional Benefits:**
  - Real-time transparency for all participants
  - Self-service audit capabilities
  - Regulatory compliance support
  - Integration with existing financial systems

### 4.20 Quorum Rotation & Committee Management

- **Dynamic Committee Selection:**
  - Rotating committees of 20 validators per round (configurable)
  - 12/20 signatures required for finality (2/3+1 threshold)
  - Deterministic selection based on round number and validator set
  - Fallback to full validator set if committee fails

- **Committee Features:**
  - **Reputation System:** Track validator performance and reliability
  - **Automatic Rotation:** Committees change every round for security
  - **Fault Tolerance:** Fallback mechanisms for committee failures
  - **Audit Trail:** Complete record of committee assignments and signatures

- **Benefits:**
  - Improved scalability with smaller quorum sizes
  - Enhanced security through rotation
  - Maintained finality guarantees
  - Practical for networks with 1000+ validators

### 4.21 Asset Instruction Model

- **Permissioned Asset Tracking:**
  - Assets are tracked via explicit instructions rather than coin balances
  - Instructions: `load_asset`, `transfer_asset`, `unload_asset`, `update_asset`
  - Each instruction is signed by the appropriate handle
  - Complete audit trail of all asset movements

- **Instruction Types:**
  - **Load Asset:** Create new asset ownership record
  - **Transfer Asset:** Move asset between handles
  - **Unload Asset:** Remove asset from system
  - **Update Asset:** Modify asset metadata or status

- **Security Model:**
  - Only authorized handles can perform asset operations
  - All instructions require cryptographic signatures
  - Immutable audit trail with timestamps and finality proofs
  - Support for complex ownership structures and compliance requirements

---

## 5. Production Readiness Status

### 5.1 Implemented Features ✅

- **Core Protocol:** DAG-based block and round production
- **High-Throughput Transaction Pool:** Sharded in-memory mempool
- **Persistent Storage:** Sled-based database for all state
- **Hierarchical Handle System:** Complete identity management
- **Asset Governance:** Dynamic asset whitelist and management
- **HTTP API:** Comprehensive REST API for all operations
- **Validator Management:** Dynamic validator set with persistence
- **Monitoring:** Prometheus metrics and Grafana dashboards
- **CLI Tools:** Wallet and handle management binaries
- **Docker Support:** Containerized deployment
- **Security:** JWT authentication and cryptographic signatures

### 5.2 Known Issues & Warnings ⚠️

- **Deprecated Base64 Functions:** Multiple warnings for deprecated `base64::encode` and `base64::decode` usage
- **Unused Imports:** Various unused import warnings across modules
- **Unused Variables:** Some unused parameters in state management
- **Type Mismatches:** Minor type conversion issues in block production

### 5.3 Production Deployment Checklist

- [x] Core blockchain functionality
- [x] Persistent storage implementation
- [x] HTTP API endpoints
- [x] Security and authentication
- [x] Monitoring and metrics
- [x] Docker containerization
- [x] CLI tools and wallets
- [ ] Full P2P networking integration
- [ ] Advanced consensus mechanisms
- [ ] Performance optimization
- [ ] Security audit completion
- [ ] Compliance documentation
- [ ] Production deployment guides

### 5.4 Next Steps for Production

1. **Fix Remaining Compilation Warnings:**
   - Update deprecated base64 functions to use Engine API
   - Clean up unused imports and variables
   - Resolve type mismatches

2. **Complete P2P Integration:**
   - Wire libp2p networking with consensus
   - Implement message validation and security
   - Add peer scoring and rate limiting

3. **Performance Optimization:**
   - Benchmark and optimize critical paths
   - Implement caching strategies
   - Optimize database queries

4. **Security Hardening:**
   - Complete security audit
   - Implement advanced authentication
   - Add rate limiting and DDoS protection

5. **Production Deployment:**
   - Create production deployment guides
   - Implement backup and recovery procedures
   - Add operational monitoring and alerting

---

## 6. Use Cases

- **High-Frequency Trading (HFT):** Algorithmic trading with deterministic microsecond timing
- **Interbank Settlement:** Real-time clearing with <500ms latency
- **Tokenized Securities:** Precision audit trail for compliance
- **CBDC Infrastructure:** Scalable backend with finality guarantees
- **Cross-Border Payments:** Fast, secure, and cost-effective transactions
- **Institutional Compliance:** Regulatory reporting and audit trails
- **Asset Management:** Hierarchical handle-based asset tracking

---

## 7. Performance Scenarios

- **Conservative (100 nodes):**  
  100 nodes × 100 tx/block × 100 blocks/sec = **1M TPS**
- **Aggressive (1000 nodes):**  
  1000 nodes × 100 tx/block × 100 blocks/sec = **10M TPS**

---

## 8. Compliance & Security

- FinDAG Time enables microsecond audit trails
- <500ms finality satisfies regulatory reporting demands
- Deterministic ordering ensures fairness, reducing risk of front-running
- Hierarchical handles provide institutional-grade identity management
- Persistent storage ensures complete auditability

---

## 9. Risks & Mitigations

| Risk                    | Mitigation Strategy                                 |
|-------------------------|-----------------------------------------------------|
| Network latency         | Regional clustering and peering strategies          |
| DAG complexity          | Pruning and checkpointing mechanisms                |
| Node sync at high freq. | Time-synchronized consensus and deterministic sync  |
| Storage growth          | Compression and sharding for block/round storage    |
| Identity management     | Hierarchical handle system with key rotation        |
| Regulatory compliance   | Persistent audit trails and transparency APIs       |

---

## 10. KPIs

- **Transaction Finality Time:** Target < 500ms
- **Peak TPS:** Minimum 1M, scalable to 10M
- **Block Propagation Delay:** < 10ms average
- **Round Checkpoint Latency:** < 250ms
- **Auditability:** 100% with microsecond timestamp accuracy
- **Storage Performance:** Sub-millisecond query response times
- **API Response Time:** < 100ms for all endpoints

---

## 11. Getting Started

- **Clone the Repository:**
  ```sh
  git clone https://github.com/your-org/findag.git
  cd findag
  ```

- **Build and Run (Rust):**
  ```sh
  cargo build --release
  ./target/release/findag-node
  ```

- **Run with Docker:**
  ```sh
  docker build -t findag-node .
  docker run -p 8080:8080 -p 9898:9898 -p 9000:9000 findag-node
  ```

- **Join a Local Cluster:**
  ```sh
  docker-compose up --build
  # Access APIs at http://localhost:8081 and http://localhost:8082
  ```

- **API Usage Examples:**
  - Submit a transaction, check balance, manage validators, and submit governance proposals (see API docs above).

- **Wallet & CLI:**
  - Generate/import/export keys, check balances, and send transactions using the CLI wallet:
    ```sh
    cargo run --bin cli_wallet -- generate
    cargo run --bin cli_wallet -- send --file mykey.txt --to <address> --amount 1000 --currency USD --node-url http://127.0.0.1:8080
    ```

- **Testing & Fuzzing:**
  - Run all tests:
    ```sh
    cargo test
    ```
  - Run fuzzers:
    ```sh
    cargo fuzz run fuzz_transaction
    cargo fuzz run fuzz_block
    cargo fuzz run fuzz_round
    cargo fuzz run fuzz_api_transaction
    ```

## 12. Contributing

- **Development Workflow:**
  - Fork the repo and create a feature branch.
  - Follow Rust best practices and code style.
  - Add tests for new features or bugfixes.
  - Open a pull request with a clear description.

- **Code of Conduct:**
  - Be respectful and collaborative.
  - Report bugs or vulnerabilities responsibly.

- **Support & Community:**
  - For help, open an issue or join the project chat/discussion board. 