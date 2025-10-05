# Rust Web Server - Lab 1 Complete Implementation

## Overview

This is a complete implementation of **Lab 1: Web Server Lab** written in Rust, based on the Python skeleton provided in the assignment. The server demonstrates socket programming fundamentals and HTTP protocol implementation.

## Features Implemented

### ✅ Core Requirements
- **TCP Socket Programming**: Creates and binds sockets, handles connections
- **HTTP Request Parsing**: Correctly parses HTTP GET requests
- **File System Access**: Serves static files from the server directory
- **HTTP Response Generation**: Sends proper HTTP responses with headers
- **Error Handling**: Returns 404 Not Found for missing files

### ✅ Optional Exercises
- **Multithreaded Server**: Handles multiple concurrent requests using threads
- **Custom HTTP Client**: Command-line client for testing the server

## Files Structure

```
web-server/
├── main.rs              # Main web server implementation
├── client.rs            # HTTP client for testing
├── test.ps1             # Automated test script (PowerShell)
├── test.bat             # Automated test script (Batch)
├── HelloWorld.html      # Test file (as specified in assignment)
├── index.html           # Default homepage
├── about.html           # Additional test file
├── proposal.md          # Assignment specification
├── README.md           # This documentation
└── Cargo.toml          # Rust project configuration
```

## Prerequisites

### Install Rust (Windows)
If Rust is not installed, install it from [rustup.rs](https://rustup.rs/):

**Windows (PowerShell):**
```powershell
Invoke-WebRequest -Uri "https://win.rustup.rs/" -OutFile "rustup-init.exe"
.\rustup-init.exe
```

**Alternative: Using winget (if available):**
```powershell
winget install --id Rustlang.Rust --source winget
```

After installation, restart your terminal and verify:
```powershell
rustc --version
cargo --version
```

## Compilation and Usage

### Method 1: Using Cargo (Recommended)
```powershell
# Compile both server and client
cargo build --release

# Run the server
cargo run --bin server

# Run the client (in another terminal)
cargo run --bin client 127.0.0.1 6789 HelloWorld.html
```

### Method 2: Direct Compilation (Windows)
```powershell
# Compile the server
rustc main.rs -o server.exe

# Compile the client
rustc client.rs -o client.exe

# Run the server
.\server.exe

# Run the client (in another terminal)
.\client.exe 127.0.0.1 6789 HelloWorld.html
```

### 2. Run the Server
```bash
./server
```

The server will start on `http://127.0.0.1:6789` (as specified in assignment, fallback to port 8080 if needed).

**Output:**
```
Web server running on http://127.0.0.1:6789
Ready to serve...
Try visiting: http://127.0.0.1:6789/HelloWorld.html
```

### 3. Test with Browser

Visit these URLs in your browser:
- `http://127.0.0.1:6789/HelloWorld.html` ✅ (should display hello page)
- `http://127.0.0.1:6789/index.html` ✅ (homepage)
- `http://127.0.0.1:6789/about.html` ✅ (about page)
- `http://127.0.0.1:6789/nonexistent.html` ❌ (should show 404 error)

### 4. Test with Custom Client (Optional Exercise 2)

**PowerShell:**
```powershell
# Compile the client
rustc client.rs -o client.exe

# Test successful request
.\client.exe 127.0.0.1 6789 HelloWorld.html

# Test 404 error
.\client.exe 127.0.0.1 6789 nonexistent.html
```

**Command Prompt:**
```batch
REM Compile the client
rustc client.rs -o client.exe

REM Test successful request
client.exe 127.0.0.1 6789 HelloWorld.html

REM Test 404 error
client.exe 127.0.0.1 6789 nonexistent.html
```

## Implementation Details

### Differences from Python Skeleton

| Python Skeleton | Rust Implementation | Notes |
|-----------------|-------------------|-------|
| `socket(AF_INET, SOCK_STREAM)` | `TcpListener::bind()` | Rust's built-in TCP abstraction |
| `connectionSocket.recv(1024)` | `BufReader::read_line()` | More robust HTTP parsing |
| `f = open(filename[1:])` | `fs::read_to_string()` | Safe file reading with error handling |
| Single-threaded | Multi-threaded | Each connection in separate thread |

### Key Features

1. **Robust Error Handling**: Graceful handling of connection errors, file not found, and invalid requests

2. **HTTP Compliance**: Proper HTTP/1.1 responses with:
   - Status codes (200 OK, 404 Not Found, 500 Internal Server Error)
   - Content-Type headers based on file extensions
   - Content-Length headers
   - Connection: close headers

3. **Multithreading**: Each client connection runs in a separate thread for concurrent request handling

4. **Security**: Safe file path handling to prevent directory traversal attacks

## Testing Screenshots

### ✅ Successful File Request
![Success](screenshots/hello-world-success.png)
*Browser displaying HelloWorld.html served by the Rust web server*

### ❌ 404 Error Handling  
![404 Error](screenshots/404-error.png)
*Custom 404 page when requesting non-existent files*

### 🔧 Custom Client Output
```powershell
PS C:\Users\crusa\Code\ai-journey\networking\web-server> .\client.exe 127.0.0.1 6789 HelloWorld.html
Connecting to 127.0.0.1:6789...
✅ Connected to server successfully!
Sending HTTP request:
GET /HelloWorld.html HTTP/1.1
Host: 127.0.0.1
Connection: close

==================================================
📄 Server Response (1234 bytes):
==================================================
HTTP/1.1 200 OK
Content-Type: text/html
Content-Length: 1180
Connection: close

<!DOCTYPE html>
<html lang="en">
...
</html>
==================================================
✅ Client finished successfully!
```

## Technical Achievements

### 🎯 Assignment Requirements Met:
- ✅ Socket programming fundamentals
- ✅ TCP connection handling  
- ✅ HTTP request parsing
- ✅ File system access
- ✅ HTTP response generation
- ✅ Error handling (404 Not Found)
- ✅ Multithreaded server (Optional Exercise 1)
- ✅ Custom HTTP client (Optional Exercise 2)

### 🚀 Additional Improvements:
- Modern Rust idioms and safety
- Comprehensive error handling
- Beautiful HTML test pages
- Detailed logging and debugging output
- Proper HTTP header handling
- Content-Type detection based on file extensions

## Learning Outcomes

This implementation demonstrates:

1. **Network Programming**: Understanding of TCP sockets and client-server architecture
2. **HTTP Protocol**: Practical knowledge of HTTP request/response format
3. **Systems Programming**: File I/O, error handling, and resource management
4. **Concurrent Programming**: Multi-threading for handling multiple clients
5. **Rust Programming**: Modern systems programming with safety guarantees

## Conclusion

This Rust implementation successfully fulfills all requirements of Lab 1: Web Server Lab, providing a robust, safe, and efficient web server that demonstrates fundamental networking concepts while leveraging Rust's modern systems programming capabilities.