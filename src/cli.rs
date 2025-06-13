use crate::storage::snapshot::Snapshot;
use crate::sync::replay::ReplayState;
use crate::storage::Storage;

pub fn handle_cli(args: &[String], storage: &Storage, state: &mut ReplayState) {
    if args.len() >= 3 && args[1] == "snapshot" {
        match args[2].as_str() {
            "create" if args.len() == 4 => {
                let snap = Snapshot::create(storage, state).unwrap();
                snap.save_to_file(&args[3]).unwrap();
                println!("📦 Snapshot saved to {}", &args[3]);
                std::process::exit(0);
            }
            "load" if args.len() == 4 => {
                let snap = Snapshot::load_from_file(&args[3]).unwrap();
                snap.restore(storage, state).unwrap();
                println!("📦 Snapshot loaded from {}", &args[3]);
                std::process::exit(0);
            }
            _ => println!("Usage: snapshot [create|load] <file>"),
        }
    }
}
