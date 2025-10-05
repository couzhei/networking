@echo off
REM Rust Web Server Test Suite for Windows (Batch Version)
REM This script tests the basic functionality of the web server

echo 🦀 Rust Web Server Test Suite
echo ================================

REM Check if Rust is installed
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Rust is not installed. Please install from https://rustup.rs/
    pause
    exit /b 1
)

echo ✅ Rust is installed
echo.

REM Compile the server and client
echo 📦 Compiling server and client...
rustc main.rs -o server.exe
if %errorlevel% neq 0 (
    echo ❌ Server compilation failed
    pause
    exit /b 1
)

rustc client.rs -o client.exe
if %errorlevel% neq 0 (
    echo ❌ Client compilation failed
    pause
    exit /b 1
)

echo ✅ Compilation successful
echo.

REM Start server in background
echo 🚀 Starting server...
start /B server.exe
timeout /t 3 /nobreak >nul

REM Test 1: Check if server is running (basic connectivity test)
echo 🔍 Testing server connection...
powershell -Command "try { $c = Test-NetConnection -ComputerName '127.0.0.1' -Port 6789 -WarningAction SilentlyContinue; if ($c.TcpTestSucceeded) { exit 0 } else { exit 1 } } catch { exit 1 }" >nul 2>&1
if %errorlevel% equ 0 (
    echo ✅ Server is listening on port 6789
    set PORT=6789
) else (
    powershell -Command "try { $c = Test-NetConnection -ComputerName '127.0.0.1' -Port 8080 -WarningAction SilentlyContinue; if ($c.TcpTestSucceeded) { exit 0 } else { exit 1 } } catch { exit 1 }" >nul 2>&1
    if %errorlevel% equ 0 (
        echo ✅ Server is listening on port 8080
        set PORT=8080
    ) else (
        echo ⚠️  Server might not be ready yet
        set PORT=6789
    )
)
echo.

REM Test 2: Test successful file request
echo 📄 Testing HelloWorld.html request...
client.exe 127.0.0.1 %PORT% HelloWorld.html > test_output.txt 2>&1
findstr "200 OK" test_output.txt >nul 2>&1
if %errorlevel% equ 0 (
    echo ✅ HelloWorld.html request successful
) else (
    echo ❌ HelloWorld.html request failed
    echo Output:
    type test_output.txt
)
echo.

REM Test 3: Test 404 error
echo 🚫 Testing 404 error handling...
client.exe 127.0.0.1 %PORT% nonexistent.html > test_404.txt 2>&1
findstr "404 Not Found" test_404.txt >nul 2>&1
if %errorlevel% equ 0 (
    echo ✅ 404 error handling works correctly
) else (
    echo ❌ 404 error handling failed
    echo Output:
    type test_404.txt
)
echo.

REM Clean up
echo 🧹 Cleaning up...
taskkill /F /IM server.exe >nul 2>&1
if exist test_output.txt del test_output.txt
if exist test_404.txt del test_404.txt
echo ✅ Cleanup completed
echo.

echo ================================
echo 🎉 Test suite completed!
echo.
echo Manual testing suggestions:
echo 1. Start server: server.exe
echo 2. Open browser: http://127.0.0.1:%PORT%/HelloWorld.html
echo 3. Test 404: http://127.0.0.1:%PORT%/nonexistent.html
echo 4. Use client: client.exe 127.0.0.1 %PORT% HelloWorld.html
echo.
pause