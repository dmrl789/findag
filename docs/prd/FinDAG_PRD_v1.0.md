# FinDAG Product Requirements Document (PRD)

**Product Name:** FinDAG  
**Version:** 1.0  
**Owner:** DMRL789 LLC  
**Last Updated:** 2025-06-26

---

## 1. Overview

FinDAG is a high-performance, low-latency, deterministic blockchain system purpose-built for financial applications. **It is implemented in Rust for maximum safety, concurrency, and performance.** Leveraging a dual-layer Directed Acyclic Graph (DAG) structure for both Blocks and Rounds, FinDAG achieves near real-time transaction finality, microsecond audit precision, and multi-million transactions per second (TPS) throughput.

- **No fees or rewards**
- **Ed25519** for address signatures
- **Automatic node discovery**
- **Authorized validation**: Only authorized nodes can validate blocks and rounds

---

## 2. Goals

- Achieve sub-second finality for all financial transactions
- Support 1M–10M TPS throughput without compromising ordering determinism
- Ensure regulatory compliance with precision timestamping
- Prevent front-running and improve fairness in high-frequency financial markets

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

---

## 4. Key Features

### 4.1 FinDAG Time & HashTimer

- Precise time synchronization with 100ns resolution
- Timestamp-based ordering for auditability and manipulation prevention
- **HashTimer is a two-part hash:**
  - The first part is 13–14 hex characters representing the FinDAG Time, which is the basis for chronologically organizing all transactions, blocks, and rounds. This provides precision up to 1/10 of a microsecond.
  - The second part is a cryptographic hash for deterministic ordering and time-lock logic

### 4.2 DAG-Based Block & Round Design

- Blocks form a DAG within micro-intervals (10–50ms)
- Rounds serve as checkpoint layers (~100–250ms) to finalize DAG paths
- Enables parallel block inclusion with global ordering

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

### 4.12 Merkle Proofs and Block Integrity

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

---

## 11. Current Limitations & Next Steps for Production Readiness

While FinDAG's core protocol, high-throughput transaction pool, DAG engine, block/round production, and basic P2P networking are implemented, the following areas are recommended for full production deployment:

- **Full Advanced P2P Integration:** Complete wiring of the libp2p-based P2P module with mempool, DAG, and round logic. Ensure all network messages are validated and processed.
- **Message Validation & Security:** Implement full cryptographic signature verification, replay protection, and rate limiting for all incoming transactions, blocks, and rounds. Add peer scoring to mitigate spam and abuse.
- **Persistent Storage:** Integrate a durable database (e.g., RocksDB, sled) for blocks, rounds, state (balances/assets), and transaction history to enable crash recovery and long-term auditability.
- **Dynamic Validator Set / Governance:** Support dynamic validator membership, governance mechanisms (voting, proposals), and optional staking or permissioning.
- **BFT Consensus (Optional):** Consider implementing a Byzantine Fault Tolerant consensus protocol (e.g., HotStuff, Tendermint) for adversarial or high-stakes environments.
- **Expanded API/CLI/Wallet:** Extend the HTTP API and CLI wallet to support transaction history, block/round queries, state queries, asset management, and real-time node interaction.
- **Monitoring & Observability:** Integrate with monitoring tools (e.g., Prometheus, Grafana) for real-time dashboards, logs, and alerting.
- **Deployment & DevOps:** Develop Docker/Kubernetes configs, CI/CD pipelines, and automated deployment scripts for easy scaling, upgrades, and operational reliability.
- **Documentation & Compliance:** Complete API documentation, operational runbooks, and compliance materials for audits and regulatory requirements.

These enhancements will ensure FinDAG is robust, secure, and ready for mission-critical financial deployments.

---

## 5. Use Cases

- **High-Frequency Trading (HFT):** Algorithmic trading with deterministic microsecond timing
- **Interbank Settlement:** Real-time clearing with <500ms latency
- **Tokenized Securities:** Precision audit trail for compliance
- **CBDC Infrastructure:** Scalable backend with finality guarantees
- **Cross-Border Payments:** Fast, secure, and cost-effective transactions

---

## 6. Performance Scenarios

- **Conservative (100 nodes):**  
  100 nodes × 100 tx/block × 100 blocks/sec = **1M TPS**
- **Aggressive (1000 nodes):**  
  1000 nodes × 100 tx/block × 100 blocks/sec = **10M TPS**

---

## 7. Compliance & Security

- FinDAG Time enables microsecond audit trails
- <500ms finality satisfies regulatory reporting demands
- Deterministic ordering ensures fairness, reducing risk of front-running

---

## 8. Risks & Mitigations

| Risk                    | Mitigation Strategy                                 |
|-------------------------|-----------------------------------------------------|
| Network latency         | Regional clustering and peering strategies          |
| DAG complexity          | Pruning and checkpointing mechanisms                |
| Node sync at high freq. | Time-synchronized consensus and deterministic sync  |
| Storage growth          | Compression and sharding for block/round storage    |

---

## 9. KPIs

- **Transaction Finality Time:** Target < 500ms
- **Peak TPS:** Minimum 1M, scalable to 10M
- **Block Propagation Delay:** < 10ms average
- **Round Checkpoint Latency:** < 250ms
- **Auditability:** 100% with microsecond timestamp accuracy

---

## 10. Next Steps

1. Formalize FinDAG Time consensus protocol
2. Develop prototype DAG block engine
3. Simulate TPS at different network sizes (100/500/1000 nodes)
4. Benchmark propagation and finality under load
5. Validate compliance with ISO/IEC 20022 and MiFID II audit requirements

## 5. Getting Started

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

## 6. Contributing

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