# Rust Web Server Test Suite for Windows
# This script tests the basic functionality of the web server

Write-Host "🦀 Rust Web Server Test Suite" -ForegroundColor Cyan
Write-Host "================================" -ForegroundColor Cyan

# Check if Rust is installed
try {
    $rustVersion = rustc --version 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Rust is installed: $rustVersion" -ForegroundColor Green
    } else {
        throw "Rust not found"
    }
} catch {
    Write-Host "❌ Rust is not installed. Please install from https://rustup.rs/" -ForegroundColor Red
    exit 1
}

# Compile the server and client
Write-Host "📦 Compiling server and client..." -ForegroundColor Yellow
try {
    rustc main.rs -o server.exe 2>$null
    if ($LASTEXITCODE -ne 0) { throw "Server compilation failed" }

    rustc client.rs -o client.exe 2>$null
    if ($LASTEXITCODE -ne 0) { throw "Client compilation failed" }

    Write-Host "✅ Compilation successful" -ForegroundColor Green
} catch {
    Write-Host "❌ Compilation failed: $_" -ForegroundColor Red
    exit 1
}

# Start server in background
Write-Host "🚀 Starting server..." -ForegroundColor Yellow
$serverProcess = Start-Process -FilePath ".\server.exe" -NoNewWindow -PassThru

# Give server time to start
Start-Sleep -Seconds 3

# Test 1: Check if server is running
Write-Host "🔍 Testing server connection..." -ForegroundColor Yellow
$port = 6789
try {
    $connection = Test-NetConnection -ComputerName "127.0.0.1" -Port $port -WarningAction SilentlyContinue
    if ($connection.TcpTestSucceeded) {
        Write-Host "✅ Server is listening on port $port" -ForegroundColor Green
    } else {
        # Try alternative port
        $port = 8080
        $connection = Test-NetConnection -ComputerName "127.0.0.1" -Port $port -WarningAction SilentlyContinue
        if ($connection.TcpTestSucceeded) {
            Write-Host "✅ Server is listening on port $port" -ForegroundColor Green
        } else {
            Write-Host "⚠️  Server might not be ready yet" -ForegroundColor Yellow
            $port = 6789
        }
    }
} catch {
    Write-Host "⚠️  Could not test connection (Test-NetConnection not available)" -ForegroundColor Yellow
    $port = 6789
}

# Test 2: Test successful file request
Write-Host "📄 Testing HelloWorld.html request..." -ForegroundColor Yellow
try {
    $output = & ".\client.exe" "127.0.0.1" $port "HelloWorld.html" 2>&1
    if ($output -match "200 OK") {
        Write-Host "✅ HelloWorld.html request successful" -ForegroundColor Green
    } else {
        Write-Host "❌ HelloWorld.html request failed" -ForegroundColor Red
        Write-Host "Output:" -ForegroundColor Red
        $output | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    }
} catch {
    Write-Host "❌ Client execution failed: $_" -ForegroundColor Red
}

# Test 3: Test 404 error
Write-Host "🚫 Testing 404 error handling..." -ForegroundColor Yellow
try {
    $output = & ".\client.exe" "127.0.0.1" $port "nonexistent.html" 2>&1
    if ($output -match "404 Not Found") {
        Write-Host "✅ 404 error handling works correctly" -ForegroundColor Green
    } else {
        Write-Host "❌ 404 error handling failed" -ForegroundColor Red
        Write-Host "Output:" -ForegroundColor Red
        $output | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    }
} catch {
    Write-Host "⚠️  404 test completed (connection may have closed)" -ForegroundColor Yellow
}

# Clean up
Write-Host "🧹 Cleaning up..." -ForegroundColor Yellow
try {
    Stop-Process -Id $serverProcess.Id -Force -ErrorAction SilentlyContinue
    Write-Host "✅ Server stopped" -ForegroundColor Green
} catch {
    Write-Host "⚠️  Could not stop server process" -ForegroundColor Yellow
}

# Remove test files
Remove-Item -Path "test_output.txt", "test_404.txt" -ErrorAction SilentlyContinue

Write-Host "================================" -ForegroundColor Cyan
Write-Host "🎉 Test suite completed!" -ForegroundColor Green
Write-Host ""
Write-Host "Manual testing suggestions:" -ForegroundColor Cyan
Write-Host "1. Start server: .\server.exe" -ForegroundColor White
Write-Host "2. Open browser: http://127.0.0.1:$port/HelloWorld.html" -ForegroundColor White
Write-Host "3. Test 404: http://127.0.0.1:$port/nonexistent.html" -ForegroundColor White
Write-Host "4. Use client: .\client.exe 127.0.0.1 $port HelloWorld.html" -ForegroundColor White