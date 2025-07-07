use clap::Parser;
use findag::tools::hashtimer_decoder::*;

#[derive(Parser)]
#[command(name = "audit-hashtimer")]
#[command(about = "Audit FinDAG HashTimers and extract FinDAG Time")]
struct Cli {
    /// The HashTimer to audit (hex string with or without 0x prefix)
    #[arg(value_name = "HASHTIMER")]
    hashtimer: String,
    
    /// Optional content for hash validation
    #[arg(short, long, value_name = "CONTENT")]
    content: Option<String>,
    
    /// Show only the FinDAG Time value
    #[arg(short, long)]
    time_only: bool,
    
    /// Show only the time prefix
    #[arg(short, long)]
    prefix_only: bool,
    
    /// Show only the hash suffix
    #[arg(short, long)]
    suffix_only: bool,
}

fn main() {
    let cli = Cli::parse();
    
    if cli.time_only {
        if let Some((time_value, _, _)) = decode_hashtimer(&cli.hashtimer) {
            println!("{time_value}");
        } else {
            eprintln!("Invalid HashTimer format");
            std::process::exit(1);
        }
        return;
    }
    
    if cli.prefix_only {
        if let Some((_, time_prefix, _)) = decode_hashtimer(&cli.hashtimer) {
            println!("0x{time_prefix}");
        } else {
            eprintln!("Invalid HashTimer format");
            std::process::exit(1);
        }
        return;
    }
    
    if cli.suffix_only {
        if let Some((_, _, hash_suffix)) = decode_hashtimer(&cli.hashtimer) {
            println!("0x{hash_suffix}");
        } else {
            eprintln!("Invalid HashTimer format");
            std::process::exit(1);
        }
        return;
    }
    
    // Full audit
    let content_bytes = cli.content.as_ref().map(|s| s.as_bytes());
    audit_hashtimer(&cli.hashtimer, content_bytes);
} 