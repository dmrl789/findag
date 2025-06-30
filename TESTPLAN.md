# 📋 FinDAG Test Plan Template

## Project: FinDAG

## Version: v1.x Testnet

## Scope: Multi-node permissioned DAG chain for financial asset tracking

---

## ✅ 1. Test Objectives

* ✔️ Verify that the Block DAG and Round DAG produce unique blocks and finalize rounds correctly.
* ✔️ Prove deterministic ordering using FinDAG Time and HashTimers.
* ✔️ Test quorum rotation and round finality thresholds.
* ✔️ Validate handle registry operations (registration, rotation, revocation).
* ✔️ Ensure asset lifecycle operations work (Load, Transfer, Unload).
* ✔️ Simulate real-world latency with geographically distributed nodes.
* ✔️ Prove the system can recover from partial validator failure.

---

## ✅ 2. Test Environment

| Component    | Spec                                            |
| ------------ | ----------------------------------------------- |
| Nodes        | 2–3 servers (e.g., NY, London, test datacenter) |
| OS           | Linux or Windows Server, production build       |
| Networking   | Real P2P, with NAT/firewall rules where needed  |
| Consensus    | Quorum rotation enabled, 2/3+1 threshold        |
| Explorer/API | Enabled for search and audit                    |
| Wallet CLI   | Enabled to load/transfer assets                 |

---

## ✅ 3. Test Cases

### 🔹 Block DAG / Round DAG

| ID     | Test                                   | Expected Result                                 |
| ------ | -------------------------------------- | ----------------------------------------------- |
| DAG-01 | Produce blocks with real instructions  | Each block hash is unique; payload size matches |
| DAG-02 | Verify HashTimers are unique & ordered | Timestamps always increase                      |
| DAG-03 | Run for 10,000 blocks                  | No forks or double-spends                       |
| DAG-04 | Replay blocks from genesis             | Final asset state matches replay                |

---

### 🔹 Finality & Quorum

| ID     | Test                            | Expected Result                                  |
| ------ | ------------------------------- | ------------------------------------------------ |
| QRM-01 | Run with quorum rotation        | At least 2/3+1 of selected validators sign       |
| QRM-02 | Drop one validator mid-round    | Fallback quorum finalizes round                  |
| QRM-03 | Simulate all validators offline | System halts finality safely, no data corruption |

---

### 🔹 Handle Registry

| ID     | Test                 | Expected Result                                 |
| ------ | -------------------- | ----------------------------------------------- |
| HDL-01 | Register root handle | `@bank.fd` visible in registry                  |
| HDL-02 | Register sub-handle  | `@bank.branch.fd` signed by parent              |
| HDL-03 | Rotate key           | Old key revoked, new key active                 |
| HDL-04 | Revoke sub-handle    | Sub-handle marked inactive, audit trail correct |

---

### 🔹 Asset Lifecycle

| ID     | Test                    | Expected Result                        |
| ------ | ----------------------- | -------------------------------------- |
| AST-01 | Load asset              | Appears in registry, ownership correct |
| AST-02 | Transfer asset          | Balance updates correctly              |
| AST-03 | Unload asset            | Asset state marked closed              |
| AST-04 | Double-transfer attempt | Rejected at consensus level            |

---

### 🔹 Network & Time Sync

| ID     | Test                         | Expected Result                   |
| ------ | ---------------------------- | --------------------------------- |
| NTP-01 | NY ↔ London time drift       | Ping-Pong offset < 5ms average    |
| NTP-02 | Simulate 200ms latency       | HashTimers stay monotonic         |
| NTP-03 | Kill node network connection | Peers detect and re-rotate quorum |

---

## ✅ 4. Reporting

For each test:

* Keep logs of:
  * Block hashes
  * FinDAG Time values
  * HashTimers
  * RoundCommitments and validator signatures
  * Quorum status
* Store explorer snapshots for auditors.
* Save test wallets' key rotations and handle updates.
* Produce a **final state snapshot** to prove replay matches expected balances.

---

## ✅ 5. Sign-Off Criteria

✔️ Block DAG stays correct and unique for full test window.
✔️ All finality thresholds met, no stuck rounds.
✔️ Handles registered, rotated, and revoked with proper parent signing.
✔️ Assets remain consistent: no double-spends, no lost state.
✔️ System recovers from partial node failure gracefully.
✔️ API/Explorer returns full state at any point.

---

## ✅ 6. Bonus: Suggested Roles

| Role                   | Owner                         |
| ---------------------- | ----------------------------- |
| Test Coordinator       | [Your Name]                   |
| Node Ops               | [Team or Contractor]          |
| Handle/Wallet Operator | [Your QA Wallet User]         |
| Auditor/Replay         | [Ops or Independent Auditor]  |

---

## ✅ 7. Runbook & Rollback

* Keep a script to stop all nodes safely.
* Use backups for key material.
* If state corruption is found, snapshot and debug before restarting.

---

## 🟢 Deliverable

**✅ One zip file or repo with:**

* Final block and round logs
* Final handle registry snapshot
* Asset ledger balances
* Replay output that proves determinism
* Node logs for time drift and quorum changes

---

## ⚡ Template usage

Use this as your `TESTPLAN.md` or Notion doc.
Check off each test case as you go.
Add real test results as tables or JSON artifacts for the sign-off folder. 