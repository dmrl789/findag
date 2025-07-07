# 🏛️ FinDAG System Architecture Overview

## 1. High-Level Architecture Diagram

```mermaid
graph TD
  subgraph Clients
    A[Web UI / Wallets]
    B[API Clients]
  end
  subgraph API Layer
    C[HTTP API Server]
    D[Bridge APIs (SWIFT, ISO20022, FIX, Corda, Fabric)]
  end
  subgraph Core
    E[Consensus Engine]
    F[RoundChain Scheduler]
    G[Block Producer]
    H[Transaction Pool]
    I[Governance Module]
    J[Audit & Compliance]
    K[Analytics Engine]
    L[Multi-Tenancy]
    M[API Management]
  end
  subgraph Network
    N[P2P Networking (libp2p)]
    O[Encryption & Security]
  end
  subgraph Storage
    P[Persistent Storage (sled)]
    Q[Backup & Recovery]
  end
  subgraph Monitoring
    R[Prometheus Metrics]
    S[Grafana Dashboards]
    T[Audit Logs]
  end

  A-->|REST/gRPC/WebSocket|C
  B-->|REST/gRPC|C
  C-->|Internal Calls|D
  C-->|Core API|E
  D-->|Bridge Events|E
  E-->|Consensus Events|F
  F-->|Block Scheduling|G
  G-->|Block Data|P
  H-->|Tx Pool|G
  I-->|Governance Actions|E
  J-->|Audit Events|T
  K-->|Analytics Data|S
  L-->|Tenant Data|P
  M-->|API Keys|C
  N-->|P2P Messages|E
  O-->|Encryption|N
  P-->|Data|Q
  R-->|Metrics|S
  S-->|Dashboards|A
  T-->|Logs|J
```

## 2. Component Descriptions

### **Clients**
- **Web UI / Wallets**: User-facing interfaces for interacting with FinDAG (transaction submission, monitoring, governance, etc.)
- **API Clients**: Programmatic access for integration, bots, and external systems.

### **API Layer**
- **HTTP API Server**: Main entry point for all client requests, handles authentication, routing, and validation.
- **Bridge APIs**: Integrations for financial messaging standards and external DLTs (SWIFT, ISO20022, FIX, Corda, Fabric).

### **Core**
- **Consensus Engine**: Implements RoundChain consensus, block validation, and finality.
- **RoundChain Scheduler**: High-frequency round/block scheduling for deterministic ordering.
- **Block Producer**: Assembles transactions into blocks, manages block lifecycle.
- **Transaction Pool**: Manages pending transactions, mempool logic.
- **Governance Module**: On-chain proposal, voting, and parameter management.
- **Audit & Compliance**: Immutable audit logs, compliance checks, export, and reporting.
- **Analytics Engine**: Business intelligence, reporting, and advanced analytics.
- **Multi-Tenancy**: Tenant isolation, quotas, and billing.
- **API Management**: API versioning, keys, developer portal.

### **Network**
- **P2P Networking**: Secure peer-to-peer communication, message propagation, peer discovery.
- **Encryption & Security**: End-to-end encryption, authentication, DDoS protection.

### **Storage**
- **Persistent Storage**: Sled-based key-value store for blocks, state, and metadata.
- **Backup & Recovery**: Data backup, restore, and disaster recovery mechanisms.

### **Monitoring**
- **Prometheus Metrics**: System and application metrics collection.
- **Grafana Dashboards**: Visualization and alerting for operational health.
- **Audit Logs**: Immutable logs for compliance and forensics.

## 3. Data Flow Summary

1. **Client Request**: User/API client sends a transaction or query to the HTTP API Server.
2. **Validation & Routing**: API server authenticates, validates, and routes the request to the appropriate core module.
3. **Consensus & Processing**: Core modules (consensus, block producer, governance, etc.) process the request, update state, and trigger events.
4. **Persistence**: All state changes, blocks, and audit events are persisted in sled storage.
5. **Network Propagation**: Relevant events/blocks are propagated to peers via P2P networking.
6. **Monitoring & Audit**: All actions are logged, metrics are updated, and dashboards reflect real-time system state.

---

*This document provides a high-level overview. See module-specific docs for implementation details.* 