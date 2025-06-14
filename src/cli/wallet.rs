use crate::wallet::{create_wallet, load_wallet, save_wallet, add_asset, add_currency};
use std::io::{stdin, stdout, Write};

pub fn wallet_cli() {
    println!("Wallet CLI:");
    println!("1. Create new wallet");
    println!("2. Load existing wallet");
    print!("Select option: ");
    stdout().flush().unwrap();
    let mut opt = String::new();
    stdin().read_line(&mut opt).unwrap();

    match opt.trim() {
        "1" => {
            print!("Enter password: "); stdout().flush().unwrap();
            let mut pass = String::new(); stdin().read_line(&mut pass).unwrap();

            print!("Enter address: "); stdout().flush().unwrap();
            let mut addr = String::new(); stdin().read_line(&mut addr).unwrap();

            let wallet = create_wallet(pass.trim(), addr.trim().to_string());
            save_wallet(&wallet, "mywallet.json").unwrap();
            println!("Wallet created and saved to mywallet.json");
        },
        "2" => {
            print!("Enter wallet path: "); stdout().flush().unwrap();
            let mut path = String::new(); stdin().read_line(&mut path).unwrap();

            let wallet = load_wallet(path.trim()).expect("Failed to load wallet");
            println!("Loaded wallet for address: {}", wallet.address);
        },
        _ => println!("Invalid option.")
    }
}
