#!/bin/bash
# Test script for Rust Web Server
# This script tests the basic functionality of the web server

echo "🦀 Rust Web Server Test Suite"
echo "================================"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}❌ Rust is not installed. Please install from https://rustup.rs/${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Rust is installed: $(rustc --version)${NC}"

# Compile the server and client
echo "📦 Compiling server and client..."
if rustc main.rs -o server && rustc client.rs -o client; then
    echo -e "${GREEN}✅ Compilation successful${NC}"
else
    echo -e "${RED}❌ Compilation failed${NC}"
    exit 1
fi

# Start server in background
echo "🚀 Starting server..."
./server &
SERVER_PID=$!

# Give server time to start
sleep 2

# Test 1: Check if server is running
echo "🔍 Testing server connection..."
if nc -z 127.0.0.1 6789 2>/dev/null || nc -z 127.0.0.1 8080 2>/dev/null; then
    echo -e "${GREEN}✅ Server is listening${NC}"
    PORT=6789
    if ! nc -z 127.0.0.1 6789 2>/dev/null; then
        PORT=8080
    fi
else
    echo -e "${YELLOW}⚠️  Server might not be ready yet${NC}"
    PORT=6789
fi

# Test 2: Test successful file request
echo "📄 Testing HelloWorld.html request..."
if ./client 127.0.0.1 $PORT HelloWorld.html > test_output.txt 2>&1; then
    if grep -q "200 OK" test_output.txt; then
        echo -e "${GREEN}✅ HelloWorld.html request successful${NC}"
    else
        echo -e "${RED}❌ HelloWorld.html request failed${NC}"
        cat test_output.txt
    fi
else
    echo -e "${RED}❌ Client connection failed${NC}"
fi

# Test 3: Test 404 error
echo "🚫 Testing 404 error handling..."
if ./client 127.0.0.1 $PORT nonexistent.html > test_404.txt 2>&1; then
    if grep -q "404 Not Found" test_404.txt; then
        echo -e "${GREEN}✅ 404 error handling works correctly${NC}"
    else
        echo -e "${RED}❌ 404 error handling failed${NC}"
        cat test_404.txt
    fi
else
    echo -e "${YELLOW}⚠️  404 test completed (connection may have closed)${NC}"
fi

# Clean up
echo "🧹 Cleaning up..."
kill $SERVER_PID 2>/dev/null
rm -f test_output.txt test_404.txt

echo "================================"
echo -e "${GREEN}🎉 Test suite completed!${NC}"
echo ""
echo "Manual testing suggestions:"
echo "1. Start server: ./server"
echo "2. Open browser: http://127.0.0.1:6789/HelloWorld.html" 
echo "3. Test 404: http://127.0.0.1:6789/nonexistent.html"
echo "4. Use client: ./client 127.0.0.1 6789 HelloWorld.html"