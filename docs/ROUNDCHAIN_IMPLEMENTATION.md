# FinDAG Simple Linear RoundChain Implementation

## Overview

The FinDAG RoundChain implements a **simple, linear chain** of Rounds for deterministic finality. This design prioritizes simplicity, auditability, and high-frequency operation over complex DAG structures.

## 🎯 **Key Design Principles**

### **Simple Linear Chain**
- Each Round references **only the immediately previous Round**
- **No Round DAG logic** — finality is strict, ordered, and single-parent
- Sequential round numbers ensure deterministic ordering
- Lightweight and pipelined for high throughput

### **Deterministic Finality**
- Rounds finalize a set of blocks in the BlockDAG
- Quorum signatures from validators confirm finality
- FinDAG Time ensures deterministic ordering
- Clear audit trail with immutable round history

### **High-Frequency Operation**
- Rounds are processed in strict order
- Pipelined processing where possible
- Minimal computational overhead
- Optimized for financial trading scenarios

## 📋 **Round Structure**

```rust
pub struct Round {
    pub round_number: u64,                    // Monotonically increasing round number
    pub parent_round_hash: [u8; 32],          // Hash of the immediately previous Round only
    pub finalized_block_hashes: Vec<[u8; 32]>, // List of finalized block hashes
    pub block_hashtimers: Vec<[u8; 32]>,      // HashTimers for each finalized block
    pub quorum_signature: [u8; 64],           // Threshold signature from validators
    pub findag_time: u64,                     // FinDAG Time for deterministic ordering
    pub proposer: Address,                    // Round proposer address
    pub proposer_signature: Signature,        // Proposer's signature
    pub proposer_public_key: PublicKey,       // Proposer's public key
}
```

## 🔧 **Core Components**

### **RoundChain**
The main structure managing the linear chain of Rounds:

```rust
pub struct RoundChain {
    pub rounds: HashMap<u64, Round>,          // round_number -> Round
    pub latest_round_number: u64,             // Latest finalized round
    pub genesis_round_hash: [u8; 32],         // Hash of genesis round
    pub validator_set: ValidatorSet,          // Validator set for quorum signatures
}
```

### **Key Methods**

#### **Creating Rounds**
```rust
// Create a new Round with finalized blocks
let round = roundchain.create_round(
    round_number,
    finalized_blocks,
    findag_time,
    proposer_keypair,
    proposer_address,
)?;

// Add round to the chain
roundchain.add_round(round)?;
```

#### **Quorum Signing**
```rust
// Sign a Round with quorum signature
roundchain.sign_round_with_quorum(
    round_number,
    &committee,
    &signatures,
)?;

// Verify quorum signature
let is_valid = roundchain.verify_round_quorum(&round);
```

#### **Block Finalization Tracking**
```rust
// Check if a block is finalized
let is_finalized = roundchain.is_block_finalized(&block_hash);

// Get the round where a block was finalized
let round_num = roundchain.get_block_finalization_round(&block_hash);
```

## 🚀 **Usage Example**

```rust
use findag::consensus::roundchain::RoundChain;
use findag::consensus::validator_set::ValidatorSet;

// Create validator set
let mut validator_set = ValidatorSet::new();
// Add validators...

// Create RoundChain
let mut roundchain = RoundChain::new(validator_set);

// Create Round 1
let round1 = roundchain.create_round(
    1,
    vec![block1, block2],
    1000,
    &proposer_keypair,
    proposer_address,
)?;

// Add to chain
roundchain.add_round(round1)?;

// Create Round 2 (references Round 1)
let round2 = roundchain.create_round(
    2,
    vec![block3],
    1100,
    &proposer_keypair,
    proposer_address,
)?;

roundchain.add_round(round2)?;
```

## 🔐 **Security Features**

### **Sequential Validation**
- Rounds must be created in strict sequential order
- Attempting to create round N+2 without round N+1 fails
- Parent round hash validation ensures chain integrity

### **Quorum Signatures**
- Threshold signatures from validator committee
- Configurable quorum size and threshold
- Signature verification for all round content

### **Deterministic Ordering**
- FinDAG Time ensures consistent ordering across nodes
- Round content includes all finalized blocks and hashtimers
- Immutable round history prevents tampering

## 📊 **Performance Characteristics**

### **Throughput**
- **Sequential processing**: Rounds processed in strict order
- **Pipelined operations**: Block finalization and round creation overlap
- **Minimal overhead**: Simple hash-based validation

### **Latency**
- **Fast finality**: Single round confirmation
- **Deterministic timing**: FinDAG Time-based ordering
- **Low computational cost**: Simple cryptographic operations

### **Scalability**
- **Horizontal scaling**: Multiple validators can propose rounds
- **Committee rotation**: Dynamic validator selection
- **Efficient storage**: Compact round representation

## 🔄 **Integration with Existing Systems**

### **BlockDAG Integration**
```rust
// Round checkpoint loop uses RoundChain
pub async fn run_round_checkpoint_loop(
    dag: &mut DagEngine,
    proposer: Address,
    keypair: &Keypair,
    interval_ms: u64,
    time_manager: &FinDAGTimeManager,
    persist_tx: UnboundedSender<PersistMsg>,
    roundchain: &mut RoundChain,  // New parameter
) {
    // Create rounds from finalized blocks
    let round = roundchain.create_round(
        round_number,
        new_blocks,
        findag_time,
        keypair,
        proposer,
    )?;
    
    roundchain.add_round(round.clone())?;
    dag.add_round(round).await;  // For compatibility
}
```

### **Network Integration**
- Rounds are broadcast using existing gossip protocol
- Signature verification updated for new Round structure
- Backward compatibility maintained where possible

## 🧪 **Testing**

### **Unit Tests**
```bash
# Run RoundChain tests
cargo test roundchain

# Run example binary
cargo run --bin roundchain_example
```

### **Integration Tests**
- Sequential round creation validation
- Quorum signature verification
- Block finalization tracking
- Performance benchmarks

## 📈 **Monitoring & Observability**

### **Statistics**
```rust
let stats = roundchain.get_statistics();
println!("Total Rounds: {}", stats.total_rounds);
println!("Total Finalized Blocks: {}", stats.total_finalized_blocks);
println!("Average Blocks per Round: {:.2}", stats.average_blocks_per_round);
```

### **Metrics**
- Round creation rate
- Block finalization latency
- Quorum signature collection time
- Chain growth rate

## 🔮 **Future Enhancements**

### **Planned Features**
- **Threshold signature schemes**: Proper BLS or similar
- **Round pruning**: Remove old rounds for storage efficiency
- **Cross-shard rounds**: Support for multi-shard finality
- **Governance integration**: Round-based parameter changes

### **Performance Optimizations**
- **Batch processing**: Multiple rounds in single operation
- **Parallel validation**: Concurrent signature verification
- **Caching**: Frequently accessed round data
- **Compression**: Efficient round storage format

## 🎯 **Commit Message Template**

When implementing RoundChain features, use this commit message format:

```
feat(rounds): Implement simple RoundChain for deterministic finality

- Linear Rounds referencing previous Round only
- Quorum signatures for finality confirmation
- Sequential validation and deterministic ordering
- Block finalization tracking and statistics
```

## 📚 **Related Documentation**

- [FinDAG Architecture Overview](../prd/FinDAG_PRD_v1.0.md)
- [Consensus Protocol Design](consensus_design.md)
- [BlockDAG Implementation](blockdag_implementation.md)
- [Validator Set Management](validator_set.md)

## ⏱️ Precise Round Intervals & Deterministic Scheduling

FinDAG's RoundChain is designed for high-frequency, deterministic operation. Rounds are triggered on a strict, configurable interval (e.g., every 100–250 ms), ensuring predictable finality and auditability.

### How it Works
- **BlockDAG**: Produces blocks as transactions arrive (every 10–50 ms).
- **RoundChain**: Every `round_interval_ms`, collects and finalizes all new blocks since the last Round.
- **No Overlap**: Each Round finalizes up to a cutoff; the next starts where the previous left off.

### Example Config
```toml
# findag_config.toml
round_interval_ms = 200
block_production_interval_ms = 10
```

### Rust Scheduler Example
```rust
loop {
    let new_blocks = dag.collect_new_blocks();
    if round_timer.elapsed() >= round_interval_ms {
        roundchain.create_and_finalize_round(new_blocks);
        round_timer.reset();
    }
    sleep(block_production_interval_ms);
}
```

### Recommended Parameters
| Parameter       | Example Value     |
| --------------- | ----------------- |
| Block frequency | 10–50 ms          |
| Round frequency | 100–250 ms        |
| Round type      | Simple chain      |
| Finality type   | Quorum signatures |
| Auditability    | Deterministic     |

### Why This Matters
- **Predictable latency**: Each transaction knows when it will be finalized.
- **Deterministic audit**: Finality timestamps are precise and predictable.
- **Efficient network use**: No waiting for large blocks; frequent, small Rounds.

**Important:** Rounds are strictly sequential—no overlap or parallel finality. This keeps FinDAG Time and HashTimers strictly ordered.

---

**Note**: This implementation follows the **"Simple RoundChain"** principle - strictly sequential finality with no Round DAG complexity. The focus is on auditability, high-frequency operation, and deterministic ordering for financial applications. 