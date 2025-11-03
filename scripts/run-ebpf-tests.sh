#!/bin/bash
# Run eBPF tests with root privileges
# Usage: sudo ./scripts/run-ebpf-tests.sh

set -euo pipefail

# Get the user who invoked sudo (for finding cargo path)
REAL_USER="${SUDO_USER:-$USER}"
USER_HOME=$(eval echo "~$REAL_USER")

# Set up Rust environment
export CARGO_HOME="$USER_HOME/.cargo"
export RUSTUP_HOME="$USER_HOME/.rustup"
export PATH="$USER_HOME/.cargo/bin:$PATH"

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}🔧 Running eBPF tests with root privileges${NC}"
echo -e "${YELLOW}User: $REAL_USER${NC}"
echo -e "${YELLOW}Cargo: $CARGO_HOME${NC}"
echo ""

# Verify cargo is available
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ cargo not found in PATH${NC}"
    echo "Expected: $CARGO_HOME/bin/cargo"
    exit 1
fi

echo -e "${GREEN}✅ cargo found: $(which cargo)${NC}"
echo ""

# Run the eBPF tests
echo -e "${GREEN}🧪 Running eBPF syscall tracing tests (--ignored --nocapture)${NC}"
echo ""

cargo test --test test_ebpf_syscall_tracing --features ebpf -- --ignored --nocapture

echo ""
echo -e "${GREEN}✅ eBPF tests complete!${NC}"
