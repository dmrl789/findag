# FinDAG FAQ

## General
**What is FinDAG?**
FinDAG is a high-performance, deterministic blockchain platform for financial applications, featuring advanced governance, dynamic asset management, and robust compliance features.

**Who should participate in the pilot?**
Financial institutions, fintechs, developers, and anyone interested in next-generation blockchain infrastructure.

## Deployment & Operations
**What are the hardware requirements?**
At least 2 CPU cores and 4GB RAM per node.

**How do I deploy a cluster?**
See the [Pilot Quickstart Guide](./pilot_quickstart.md).

## Assets & Governance
**How are assets managed?**
Assets are managed on-chain via governance proposals. See the docs for details.

**How do I propose a new asset?**
Use the governance API. Instructions are in the main documentation.

## Support & Feedback
**How do I report a bug or request a feature?**
Open a GitHub issue or email support@your-org.com.

**Where can I find the feedback form?**
[docs/pilot_feedback_form.md]

### Q: Why does FinDAG use strict, high-frequency Round intervals?

**A:**
- To ensure predictable, low-latency finality for every transaction.
- To make throughput and auditability easy to measure and verify.
- To keep the network efficient: blocks are finalized frequently, so no need to wait for large batches.
- To guarantee that Rounds are strictly sequential, with no overlap—making the system easy to reason about and audit.

**Typical config:**
- `round_interval_ms = 100..250` (Rounds every 100–250 ms)
- `block_production_interval_ms = 10..50` (Blocks every 10–50 ms)

---
For more, see the full documentation or contact the team! 