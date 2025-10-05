# Lab 1: Web Server Lab

## Running the Server

### Setup Steps:
1. Put an HTML file (e.g., `HelloWorld.html`) in the same directory that the server is in
2. Run the server program
3. Determine the IP address of the host that is running the server (e.g., `128.238.251.26`)

### Testing:
From another host, open a browser and provide the corresponding URL:

```
http://128.238.251.26:6789/HelloWorld.html
```

**Notes:**
- `HelloWorld.html` is the name of the file you placed in the server directory
- Note the use of the port number after the colon (`:6789`)
- Replace this port number with whatever port you have used in the server code
- The browser should display the contents of `HelloWorld.html`
- If you omit `:6789`, the browser will assume port 80 and you will get the web page from the server only if your server is listening at port 80

### Error Testing:
Try to get a file that is not present at the server. You should get a **"404 Not Found"** message.
In this lab, you will learn the basics of socket programming for TCP connections in Rust: 
- How to create a socket
- How to bind it to a specific address and port
- How to send and receive HTTP packets
- Learn some basics of HTTP header format

## Objective

You will develop a web server that handles **one HTTP request at a time**. Your web server should:

1. **Accept and parse** the HTTP request
2. **Get the requested file** from the server's file system
3. **Create an HTTP response message** consisting of the requested file preceded by header lines
4. **Send the response** directly to the client
5. **Handle errors**: If the requested file is not present in the server, send an HTTP "404 Not Found" message back to the clientWeb Server Lab
In this lab, you will learn the basics of socket programming for TCP connections in Rust: how to create
a socket, bind it to a specific address and port, as well as send and receive a HTTP packet. You will also
learn some basics of HTTP header format.
You will develop a web server that handles one HTTP request at a time. Your web server should accept
and parse the HTTP request, get the requested file from the server’s file system, create an HTTP response
message consisting of the requested file preceded by header lines, and then send the response directly to
the client. If the requested file is not present in the server, the server should send an HTTP “404 Not
Found” message back to the client.
## Code Implementation

Below you will find the skeleton code for the Web server. You are to complete the skeleton code. The places where you need to fill in code are marked with `#Fill in start` and `#Fill in end`. Each place may require one or more lines of code.
Running the Server
Put an HTML file (e.g., HelloWorld.html) in the same directory that the server is in. Run the server
program. Determine the IP address of the host that is running the server (e.g., 128.238.251.26). From
another host, open a browser and provide the corresponding URL. For example:
http://128.238.251.26:6789/HelloWorld.html
‘HelloWorld.html’ is the name of the file you placed in the server directory. Note also the use of the port
number after the colon. You need to replace this port number with whatever port you have used in the
server code. In the above example, we have used the port number 6789. The browser should then display
the contents of HelloWorld.html. If you omit ":6789", the browser will assume port 80 and you will get
the web page from the server only if your server is listening at port 80.
Then try to get a file that is not present at the server. You should get a “404 Not Found” message.
## Deliverables

You will hand in:
1. **Complete server code** 
2. **Screenshots** of your client browser, verifying that you actually receive the contents of the HTML file from the server
## Skeleton Python Code for the Web Server

```python
# Import socket module
from socket import *
import sys  # In order to terminate the program

serverSocket = socket(AF_INET, SOCK_STREAM)

# Prepare a server socket
#Fill in start
#Fill in end

while True:
    # Establish the connection
    print('Ready to serve...')
    connectionSocket, addr = #Fill in start #Fill in end
    
    try:
        message = #Fill in start #Fill in end
        filename = message.split()[1]
        f = open(filename[1:])
        outputdata = #Fill in start #Fill in end
        
        # Send one HTTP header line into socket
        #Fill in start
        #Fill in end
        
        # Send the content of the requested file to the client
        for i in range(0, len(outputdata)):
            connectionSocket.send(outputdata[i].encode())
            
        connectionSocket.send("\r\n".encode())
        connectionSocket.close()
        
    except IOError:
        # Send response message for file not found
        #Fill in start
        #Fill in end
        
        # Close client socket
        #Fill in start
        #Fill in end

serverSocket.close()
sys.exit()  # Terminate the program after sending the corresponding data
```
## Optional Exercises

### 1. Multithreaded Server
Currently, the web server handles only **one HTTP request at a time**. 

**Task**: Implement a multithreaded server that is capable of serving multiple requests simultaneously.

**Implementation Details**:
- Using threading, first create a **main thread** in which your modified server listens for clients at a fixed port
- When it receives a TCP connection request from a client, it will set up the TCP connection through another port and services the client request in a **separate thread**
- There will be a separate TCP connection in a separate thread for each request/response pair

### 2. Custom HTTP Client
Instead of using a browser, write your own HTTP client to test your server.

**Requirements**:
- Your client will connect to the server using a TCP connection
- Send an HTTP request to the server
- Display the server response as an output
- You can assume that the HTTP request sent is a GET method

**Command Line Interface**:
The client should take command line arguments specifying:
- Server IP address or host name
- The port at which the server is listening  
- The path at which the requested object is stored at the server

**Usage Format**:
```bash
python client.py server_host server_port filename
```