use findag::storage::state::StateDB;

fn main() {
    println!("🔍 Checking account balances...");
    
    let state_db = StateDB::new("state_db");
    
    let test_accounts = vec![
        "fdg1qalice1234567890",
        "fdg1qbob1234567890", 
        "fdg1qcharlie1234567890",
        "fdg1qdiana1234567890",
        "fdg1qedward1234567890",
    ];
    
    let bot_accounts = vec![
        "fdg1qbot5c8461aa",
        "fdg1qbot9e8b20c7",
        "fdg1qbotbb1e586b",
        "fdg1qbot821642",
        "fdg1qbot519950",
        "fdg1qbot260960",
        "fdg1qbot730226",
        "fdg1qbot301129",
        "fdg1qbot12345678",
        "fdg1qbotabcdef12",
        "fdg1qbotdeadbeef",
        "fdg1qbotfeedcafe",
        "fdg1qbotcafebabe",
        "fdg1qbotbaddad00",
        "fdg1qbot00000000",
        "fdg1qbot11111111",
        "fdg1qbot22222222",
        "fdg1qbot33333333",
        "fdg1qbot44444444",
        "fdg1qbot55555555",
        "fdg1qbot66666666",
        "fdg1qbot77777777",
        "fdg1qbot88888888",
        "fdg1qbot99999999",
        "fdg1qbotaaaaaaaa",
        "fdg1qbotbbbbbbbb",
        "fdg1qbotcccccccc",
        "fdg1qbotdddddddd",
        "fdg1qboteeeeeeee",
        "fdg1qbotffffffff",
        "fdg1qbotd60e1cdd",
    ];
    
    println!("📊 Test Account Balances:");
    for account in test_accounts {
        let balance = state_db.get_balance(0, account, "USD");
        println!("  {account}: {balance} USD");
    }
    
    println!("\n🤖 Bot Account Balances:");
    for account in bot_accounts {
        let balance = state_db.get_balance(0, account, "USD");
        println!("  {account}: {balance} USD");
    }
} 