use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::fs;

fn main() {
    // Bind to port 8080 (avoiding port 80 which might be used by other web servers)
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Web server running on http://127.0.0.1:8080");
    
    // Listen for incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    // Read the HTTP request
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    println!("Request: {}", request_line);
    
    // Parse the request line to extract the file path
    let requested_file = parse_request(&request_line);
    
    // Generate and send the response
    let response = generate_response(&requested_file);
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
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

fn generate_response(file_path: &str) -> String {
    // Try to read the requested file
    match fs::read_to_string(file_path) {
        Ok(content) => {
            // File found - generate 200 OK response
            let content_type = get_content_type(file_path);
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                content_type,
                content.len(),
                content
            )
        }
        Err(_) => {
            // File not found - generate 404 response
            let not_found_html = "<html><body><h1>404 Not Found</h1><p>The requested file was not found on this server.</p></body></html>";
            format!(
                "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                not_found_html.len(),
                not_found_html
            )
        }
    }
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