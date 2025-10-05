use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::fs;
use std::thread;

fn main() {
    // Configuration - can be changed to use different ports (6789 as mentioned in proposal)
    let bind_address = "127.0.0.1:6789";
    
    let listener = TcpListener::bind(bind_address).unwrap_or_else(|e| {
        eprintln!("Failed to bind to {}: {}", bind_address, e);
        eprintln!("Trying alternative port 8080...");
        TcpListener::bind("127.0.0.1:8080").unwrap_or_else(|e| {
            eprintln!("Failed to bind to any port: {}", e);
            std::process::exit(1);
        })
    });
    
    let local_addr = listener.local_addr().unwrap();
    println!("Web server running on http://{}", local_addr);
    println!("Ready to serve...");
    println!("Try visiting: http://{}/HelloWorld.html", local_addr);
    
    // Listen for incoming connections with multithreading support
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let peer_addr = stream.peer_addr().unwrap_or_else(|_| "unknown".parse().unwrap());
                println!("New connection from: {}", peer_addr);
                
                // Handle each connection in a separate thread (Optional Exercise 1)
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    // Read the HTTP request (similar to Python skeleton's message = connectionSocket.recv(1024))
    let mut buf_reader = BufReader::new(&stream);
    let mut request_lines = Vec::new();
    
    // Read all HTTP headers
    loop {
        let mut line = String::new();
        match buf_reader.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {
                if line == "\r\n" || line == "\n" {
                    break; // End of headers
                }
                request_lines.push(line.trim_end().to_string());
            }
            Err(e) => {
                eprintln!("Error reading request: {}", e);
                return;
            }
        }
    }
    
    if request_lines.is_empty() {
        eprintln!("Empty request received");
        return;
    }
    
    let request_line = &request_lines[0];
    println!("Request: {}", request_line);
    
    // Parse the request line to extract the file path (similar to Python's filename = message.split()[1])
    let requested_file = parse_request(request_line);
    println!("Requested file: {}", requested_file);
    
    // Generate and send the response
    match generate_response(&requested_file) {
        Ok(response) => {
            if let Err(e) = stream.write_all(response.as_bytes()) {
                eprintln!("Error sending response: {}", e);
            } else if let Err(e) = stream.flush() {
                eprintln!("Error flushing stream: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error generating response: {}", e);
            // Send 500 Internal Server Error
            let error_response = "HTTP/1.1 500 Internal Server Error\r\n\r\n";
            let _ = stream.write_all(error_response.as_bytes());
        }
    }
}

fn parse_request(request_line: &str) -> String {
    // Parse HTTP request line: "GET /path/to/file HTTP/1.1"
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    
    if parts.len() >= 2 && parts[0] == "GET" {
        let mut path = parts[1].to_string();
        
        // If root path is requested, serve index.html
        if path == "/" {
            path = "/index.html".to_string();
        }
        
        // Remove leading slash for file system access
        if path.starts_with('/') {
            path.remove(0);
        }
        
        path
    } else {
        // Default to index.html for invalid requests
        "index.html".to_string()
    }
}

fn generate_response(file_path: &str) -> Result<String, std::io::Error> {
    // Try to read the requested file (similar to Python's f = open(filename[1:]) and outputdata = f.read())
    match fs::read_to_string(file_path) {
        Ok(content) => {
            // File found - generate 200 OK response
            // Send HTTP header line into socket (matching Python skeleton requirement)
            let content_type = get_content_type(file_path);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                content_type,
                content.len(),
                content
            );
            Ok(response)
        }
        Err(_) => {
            // File not found - generate 404 response (matching Python's IOError handling)
            let not_found_html = generate_404_page(file_path);
            let response = format!(
                "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                not_found_html.len(),
                not_found_html
            );
            Ok(response)
        }
    }
}

fn generate_404_page(requested_file: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>404 Not Found</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .error {{ background: #f8d7da; border: 1px solid #f5c6cb; color: #721c24; padding: 20px; border-radius: 5px; }}
    </style>
</head>
<body>
    <div class="error">
        <h1>404 Not Found</h1>
        <p>The requested file <strong>{}</strong> was not found on this server.</p>
        <p>Please check the file path and try again.</p>
        <hr>
        <small>Rust Web Server</small>
    </div>
</body>
</html>"#,
        requested_file
    )
}

fn get_content_type(file_path: &str) -> &'static str {
    // Determine content type based on file extension
    if file_path.ends_with(".html") || file_path.ends_with(".htm") {
        "text/html"
    } else if file_path.ends_with(".css") {
        "text/css"
    } else if file_path.ends_with(".js") {
        "application/javascript"
    } else if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") {
        "image/jpeg"
    } else if file_path.ends_with(".png") {
        "image/png"
    } else if file_path.ends_with(".gif") {
        "image/gif"
    } else {
        "text/plain"
    }
}

// enter the following commands to compile and run the file:
// $ rustc main.rs
// $ ./main