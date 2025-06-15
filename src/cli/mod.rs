use clap::{Parser, Subcommand};
use crate::types::address::FinDagKeypair;
use crate::utils::address::{generate_address, AddressType};

pub mod wallet;

#[derive(Parser)]
#[command(name = "findag-cli")]
#[command(about = "FinDAG CLI Tools", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a new FinDAG address
    GenAddr {
        #[arg(short, long, default_value = "standard")]
        r#type: String,
    },
}
