@echo off
echo Starting Mirror Health MVP...
echo.

REM Check Rust
where cargo >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Error: Rust is not installed or not in PATH.
    echo Please install Rust from https://rust-lang.org/
    pause
    exit /b 1
)

REM Check Ollama
echo Checking Ollama for local mode...
tasklist | findstr /I "ollama" >nul
if %ERRORLEVEL% neq 0 (
    echo Warning: Ollama is not running. Local mode will not work.
    echo Run "ollama serve" in another terminal if you need local mode.
)

echo.
echo Building application...
cargo run

if %ERRORLEVEL% neq 0 (
    echo.
    echo Build failed. Common fixes:
    echo 1. Install Rustup: https://rustup.rs/
    echo 2. Run: rustup default stable
    echo 3. Install Visual C++ Build Tools: https://visualstudio.microsoft.com/visual-cpp-build-tools/
    pause
)
