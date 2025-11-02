#!/bin/bash
# Quick prototype: Interactive rust-gdb wrapper for Ruchy debugging
# Will be formalized as DEBUGGER-048 ticket

set -euo pipefail

if [ $# -lt 1 ]; then
    echo "Usage: $0 <ruchy_file> [--break <function>] [--condition <expr>]"
    echo ""
    echo "Examples:"
    echo "  $0 test.ruchy --break dispatch_method_call --condition 'strcmp(method, \"read_line\") == 0'"
    echo "  $0 test.ruchy --break eval_method_dispatch"
    exit 1
fi

RUCHY_FILE="$1"
shift

# Check if ruchy binary exists
RUCHY_BIN="../ruchy/target/debug/ruchy"
if [ ! -f "$RUCHY_BIN" ]; then
    echo "Building ruchy with debug symbols..."
    (cd ../ruchy && cargo build --bin ruchy)
fi

# Build GDB commands
GDB_COMMANDS=$(mktemp)
cat > "$GDB_COMMANDS" <<'EOF'
# Rust pretty-printers
set print pretty on
set print array on
set print array-indexes on

# Common breakpoints for Ruchy debugging
# Uncomment as needed:
# break ruchy::runtime::interpreter::Interpreter::dispatch_method_call
# break ruchy::runtime::eval_method_dispatch::eval_method_dispatch

EOF

# Parse arguments for custom breakpoints
while [ $# -gt 0 ]; do
    case "$1" in
        --break)
            shift
            echo "break $1" >> "$GDB_COMMANDS"
            shift
            ;;
        --condition)
            shift
            echo "condition \$bpnum $1" >> "$GDB_COMMANDS"
            shift
            ;;
        *)
            shift
            ;;
    esac
done

# Add run command
echo "run run \"$RUCHY_FILE\"" >> "$GDB_COMMANDS"

echo "🔍 Launching rust-gdb with breakpoints..."
echo "📋 GDB commands:"
cat "$GDB_COMMANDS"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Launch rust-gdb
rust-gdb -x "$GDB_COMMANDS" "$RUCHY_BIN"

# Cleanup
rm "$GDB_COMMANDS"
