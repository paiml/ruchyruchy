#!/bin/bash
# Capture golden traces for renacer performance validation
# Based on RENACER_INTEGRATION_SUMMARY.md specifications
set -euo pipefail

echo "📸 Capturing golden traces for renacer validation"
echo ""

# Check if ruchy compiler is installed
if ! command -v ruchy >/dev/null 2>&1; then
    echo "❌ ERROR: ruchy compiler not found"
    echo "Install: cargo install ruchy"
    exit 1
fi

# Check if renacer is installed
if ! command -v renacer >/dev/null 2>&1; then
    echo "❌ ERROR: renacer not found"
    echo "Install: cargo install renacer"
    exit 1
fi

# Create golden traces directory
mkdir -p golden_traces

# Create test Ruchy files for tracing
mkdir -p /tmp/renacer-test
cat > /tmp/renacer-test/simple.ruchy <<'EOF'
fun main() {
    println("Hello from RuchyRuchy!");
}
EOF

cat > /tmp/renacer-test/fibonacci.ruchy <<'EOF'
fun fibonacci(n: i32) -> i32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fun main() {
    let result = fibonacci(10);
    println("Fibonacci(10) = {}", result);
}
EOF

echo "1. Capturing JIT compilation trace..."
renacer capture \
    --scenario jit_compilation \
    --command "ruchy compile /tmp/renacer-test/simple.ruchy --jit" \
    --output golden_traces/jit_compilation.trace \
    --budget syscalls:10000,memory_mb:512,time_ms:1000 || echo "⚠️  JIT trace capture failed (expected if JIT not implemented)"

echo ""
echo "2. Capturing interpreter execution trace..."
renacer capture \
    --scenario interpreter_execution \
    --command "ruchy run /tmp/renacer-test/fibonacci.ruchy" \
    --output golden_traces/interpreter_execution.trace \
    --budget syscalls:10000,memory_mb:512,time_ms:500 || echo "⚠️  Interpreter trace capture failed (expected if interpreter not ready)"

echo ""
echo "3. Capturing debugger attach trace..."
if [ -f target/debug/ruchydbg ]; then
    renacer capture \
        --scenario debugger_attach \
        --command "./target/debug/ruchydbg validate /tmp/renacer-test/simple.ruchy" \
        --output golden_traces/debugger_attach.trace \
        --budget syscalls:10000,memory_mb:512,time_ms:100 || echo "⚠️  Debugger trace capture failed"
else
    echo "⚠️  ruchydbg binary not found (run 'cargo build' first)"
fi

echo ""
echo "✅ Golden trace capture complete!"
echo ""
echo "Traces captured in golden_traces/:"
ls -lh golden_traces/*.trace 2>/dev/null || echo "  (No traces captured - expected for early development)"
echo ""
echo "To validate performance:"
echo "  make renacer-validate"
echo ""
echo "To recapture traces after changes:"
echo "  make renacer-capture"
