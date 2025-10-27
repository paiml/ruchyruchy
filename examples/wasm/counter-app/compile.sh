#!/bin/bash
# Compile and run the counter example

set -euo pipefail

echo "Compiling Ruchy counter example to WebAssembly..."

# Ensure we're in the example directory
cd "$(dirname "$0")"

# Check if ruchy is available
if ! command -v ruchy &> /dev/null; then
    echo "Error: ruchy compiler not found. Please install it first."
    exit 1
fi

# Compile the counter.ruchy file to WebAssembly
ruchy compile counter.ruchy --target wasm \
    --debug-info true \
    --source-maps true \
    --opt-level 2 \
    --mode development

echo "Compilation successful!"
echo "- counter.wasm: WebAssembly binary"
echo "- counter.js: JavaScript glue code"

# Check if we should start a web server
if [[ "$#" -gt 0 && "$1" == "run" ]]; then
    echo "Starting web server..."
    
    # Check if python is available
    if command -v python3 &> /dev/null; then
        python3 -m http.server 8000
    elif command -v python &> /dev/null; then
        python -m http.server 8000
    else
        echo "Warning: Python not found. Please start a web server manually and open index.html."
    fi
else
    echo ""
    echo "To run the example:"
    echo "1. Start a web server in this directory:"
    echo "   python -m http.server 8000"
    echo "2. Open http://localhost:8000 in your browser"
    echo ""
    echo "Or simply run: $0 run"
fi