mod utils;
mod blockchain;
mod network;
mod sync;
mod storage;
mod api;
mod cli;

use storage::Storage;
use sync::replay::{replay_chain, ReplayState};
use std::sync::{Arc, Mutex};

fn main() {
    println!("🔗 Starting FinDAG node...");

    let storage = Arc::new(Storage::init("findag_db"));
    let state = Arc::new(Mutex::new(ReplayState::default()));

    let args: Vec<String> = std::env::args().collect();
    cli::handle_cli(&args, &storage, &mut state.lock().unwrap());

    // Full chain replay
    {
        let mut st = state.lock().unwrap();
        if let Err(e) = replay_chain(&storage, &mut st) {
            eprintln!("❌ Replay failed: {}", e);
            return;
        }
    }

    // API and Network setup
    let api_routes = api::snapshot::routes(storage.clone(), state.clone());
    let (_addr, _server) = warp::serve(api_routes).bind_ephemeral(([127, 0, 0, 1], 8080));
    println!("🌐 API running on http://127.0.0.1:8080");

    network::setup_network();
}
