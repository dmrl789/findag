pub mod utils;
pub mod blockchain;
pub mod network;

pub mod consensus {
    pub mod finality;
}
pub mod types {
    pub mod finality;
}


pub mod registry {
    pub mod reputation;
    // ...
}


pub mod storage; // Ensure this line is present

// Inside storage module, these two files will be auto included if using mod.rs
// If you're not using mod.rs, add:
pub mod storage::round_index;
pub mod storage::asset_index;
pub mod api;
pub mod sync; // Added sync module for chain replay
pub mod dht;
pub mod validation;
pub mod sync;

pub mod auth;
pub mod cli;
pub mod domain; // <-- Add this

pub mod auth;
pub mod governance;

pub mod registry::multisig; // ✅ add this line
pub mod types::multisig;    // ✅ and this one

pub mod vote;
pub mod mempool;
pub mod blockchain::assembler;
pub mod consensus::reputation;
