use crate::types::transaction::{Transaction, TxType};
use crate::types::block::{Block, BlockHeader};
use crate::types::address::Address;
use crate::utils::time::now_timestamp;
use crate::storage::DB;
use blake3::Hasher;
use uuid::Uuid;

pub fn assemble_block(pending_txs: Vec<Transaction>, creator: Address, previous_hash: String) -> Block {
    let timestamp = now_timestamp();
    let mut hasher = Hasher::new();

    // Sort transactions by timestamp for deterministic order
    let mut sorted_txs = pending_txs.clone();
    sorted_txs.sort_by_key(|tx| tx.timestamp);

    for tx in &sorted_txs {
        hasher.update(tx.hash().as_bytes());
    }

    let tx_root = hex::encode(hasher.finalize());

    let header = BlockHeader {
        id: Uuid::new_v4().to_string(),
        timestamp,
        previous_hash,
        creator,
        tx_root,
    };

    Block {
        header,
        transactions: sorted_txs,
    }
}
