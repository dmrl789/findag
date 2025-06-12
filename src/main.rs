mod utils {
    pub mod time;
    pub mod tx;
}

mod types {
    pub mod asset;
    pub mod transaction;
    pub mod address;
}

mod blockchain {
    pub mod block;
}

mod validation {
    pub mod transaction;
    pub use super::validation::ValidationError;
}

mod consensus {
    pub mod round;
}

use utils::tx::create_load_tx;
use types::asset::AssetType;
use blockchain::block::Block;
use validation::transaction::validate_transaction;
use consensus::round::Round;

fn main() {
    env_logger::init();

    // Create a transaction
    let tx = create_load_tx("1abc...", "BOND-XYZ", AssetType::Bond, "{\"coupon\":\"3.5%\"}");

    // Validate the transaction
    match validate_transaction(&tx) {
        Ok(_) => println!("✅ Transaction is valid"),
        Err(e) => {
            println!("❌ Transaction invalid: {}", e);
            return;
        }
    }

    // Create a block from the transaction
    let block = Block::new(vec![], vec![tx.clone()], "1abc...".to_string());
    println!("🧱 Block created:\n{:#?}", block);

    // Create a round including the block
    let round = Round::new(
        1,
        vec![],                      // No parent rounds yet
        vec![block],                // Blocks in the round
        "1abc...".to_string(),      // Authorized validator
    );

    println!("🔁 Round created:\n{:#?}", round);
}
