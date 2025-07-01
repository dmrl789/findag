use std::net::TcpListener;
use std::io::{Read, Write};
use std::thread;
use std::time::Instant;

fn main() {
    println!("🌐 FinDAG Network Tap");
    println!("=====================");
    println!("Listening on port 8080...");
    println!("Forwarding to http://127.0.0.1:3000");
    println!("Press Ctrl+C to stop");
    
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to port 8080");
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 8192];
    let mut request_data = Vec::new();
    
    // Read the request
    loop {
        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                request_data.extend_from_slice(&buffer[..n]);
                if n < buffer.len() {
                    break;
                }
            }
            Ok(_) => break,
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                return;
            }
        }
    }
    
    let request_str = String::from_utf8_lossy(&request_data);
    println!("📥 CAPTURED REQUEST:");
    println!("{}", request_str);
    println!("📥 END REQUEST");
    
    // Forward to the actual node
    let start = Instant::now();
    let mut node_stream = match std::net::TcpStream::connect("127.0.0.1:3000") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to connect to node: {}", e);
            return;
        }
    };
    
    // Send request to node
    if let Err(e) = node_stream.write_all(&request_data) {
        eprintln!("Failed to forward request: {}", e);
        return;
    }
    
    // Read response from node
    let mut response_data = Vec::new();
    let mut response_buffer = [0; 8192];
    
    loop {
        match node_stream.read(&mut response_buffer) {
            Ok(n) if n > 0 => {
                response_data.extend_from_slice(&response_buffer[..n]);
                if n < response_buffer.len() {
                    break;
                }
            }
            Ok(_) => break,
            Err(e) => {
                eprintln!("Error reading from node: {}", e);
                break;
            }
        }
    }
    
    let response_str = String::from_utf8_lossy(&response_data);
    println!("📤 NODE RESPONSE:");
    println!("{}", response_str);
    println!("📤 END RESPONSE");
    println!("⏱️  Round-trip time: {:?}", start.elapsed());
    
    // Send response back to client
    if let Err(e) = stream.write_all(&response_data) {
        eprintln!("Failed to send response to client: {}", e);
    }
} 