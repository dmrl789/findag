pub mod utils;
pub mod blockchain;
pub mod network;
pub mod wallet;
pub mod consensus {
    pub mod finality;
}
pub mod types {
    pub mod finality;
}

pub mod ai; // Add AI module

pub mod registry {
    pub mod reputation;
    // ...
}

pub mod storage; // Ensure this line is p
pub mod api;
pub mod sync; // Added sync module for chain replay
pub mod dht;
pub mod validation;
pub mod cli;
pub mod domain; // <-- Add this
pub mod auth;
pub mod governance;
pub mod vote;
pub mod mempool;

