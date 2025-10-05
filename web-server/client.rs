use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Check command line arguments
    if args.len() != 4 {
        eprintln!("Usage: {} server_host server_port filename", args[0]);
        eprintln!("Example: {} 127.0.0.1 6789 HelloWorld.html", args[0]);
        std::process::exit(1);
    }
    
    let server_host = &args[1];
    let server_port = &args[2];
    let filename = &args[3];
    
    // Parse port number
    let port: u16 = server_port.parse().unwrap_or_else(|_| {
        eprintln!("Error: Invalid port number '{}'", server_port);
        std::process::exit(1);
    });
    
    // Connect to server
    let server_address = format!("{}:{}", server_host, port);
    println!("Connecting to {}...", server_address);
    
    let mut stream = match TcpStream::connect(&server_address) {
        Ok(stream) => {
            println!("✅ Connected to server successfully!");
            stream
        }
        Err(e) => {
            eprintln!("❌ Failed to connect to {}: {}", server_address, e);
            std::process::exit(1);
        }
    };
    
    // Create HTTP GET request
    let request = format!(
        "GET /{} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        filename, server_host
    );
    
    println!("Sending HTTP request:");
    println!("{}", request.trim());
    println!("{}", "=".repeat(50));
    
    // Send request
    if let Err(e) = stream.write_all(request.as_bytes()) {
        eprintln!("❌ Failed to send request: {}", e);
        std::process::exit(1);
    }
    
    // Read response
    let mut response = String::new();
    match stream.read_to_string(&mut response) {
        Ok(bytes_read) => {
            println!("📄 Server Response ({} bytes):", bytes_read);
            println!("{}", "=".repeat(50));
            println!("{}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to read response: {}", e);
            std::process::exit(1);
        }
    }
    
    println!("{}", "=".repeat(50));
    println!("✅ Client finished successfully!");
}

// Compilation and usage instructions:
// 
// To compile:
// $ rustc -o client client.rs
//
// To use (examples):
// $ ./client 127.0.0.1 6789 HelloWorld.html
// $ ./client 127.0.0.1 6789 index.html  
// $ ./client 127.0.0.1 6789 nonexistent.html  (to test 404 error)