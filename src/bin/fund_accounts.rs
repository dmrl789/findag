use findag::storage::state::StateDB;
use std::env;

fn main() {
    println!("🔧 Funding test accounts for FinDAG...");
    
    // Create state database
    let state_db = StateDB::new("state_db");
    
    // Fund some test accounts with USD
    let test_accounts = vec![
        "fdg1qalice1234567890",
        "fdg1qbob1234567890", 
        "fdg1qcharlie1234567890",
        "fdg1qdiana1234567890",
        "fdg1qedward1234567890",
    ];
    
    for account in test_accounts {
        // Fund with 10,000 USD each
        match state_db.set_balance(0, account, 10000) {
            Ok(_) => println!("✅ Funded {} with 10,000 USD", account),
            Err(e) => println!("❌ Failed to fund {}: {}", account, e),
        }
    }
    
    // Also fund some bot accounts that are being used
    let bot_accounts = vec![
        "fdg1qbot821642",
        "fdg1qbot519950", 
        "fdg1qbot260960",
        "fdg1qbot730226",
        "fdg1qbot301129",
    ];
    
    for account in bot_accounts {
        // Fund with 5,000 USD each
        match state_db.set_balance(0, account, 5000) {
            Ok(_) => println!("✅ Funded {} with 5,000 USD", account),
            Err(e) => println!("❌ Failed to fund {}: {}", account, e),
        }
    }
    
    println!("🎉 Account funding complete!");
    println!("Now you can send valid transactions and see the DAG building up!");
} 