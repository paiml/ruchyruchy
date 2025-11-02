#!/bin/bash
# Automated rust-gdb wrapper - runs and captures debug info automatically
# DEBUGGER-048 prototype (non-interactive version)

set -euo pipefail

if [ $# -lt 1 ]; then
    echo "Usage: $0 <ruchy_file> [breakpoint_function] [breakpoint_condition]"
    echo ""
    echo "Examples:"
    echo "  $0 test.ruchy dispatch_method_call 'strcmp(method,\"read_line\")==0'"
    echo "  $0 test.ruchy eval_method_dispatch"
    exit 1
fi

RUCHY_FILE="$1"
BREAKPOINT="${2:-dispatch_method_call}"
CONDITION="${3:-}"

RUCHY_BIN="../ruchy/target/debug/ruchy"

if [ ! -f "$RUCHY_BIN" ]; then
    echo "❌ Ruchy binary not found. Build it first:"
    echo "   cd ../ruchy && cargo build --bin ruchy"
    exit 1
fi

# Create GDB batch commands
GDB_COMMANDS=$(mktemp)
cat > "$GDB_COMMANDS" <<EOF
set pagination off
set print pretty on
set print array on

# Set breakpoint
break $BREAKPOINT
$([ -n "$CONDITION" ] && echo "condition 1 $CONDITION")

# Commands to run when breakpoint hits
commands 1
  echo ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\\n
  echo 🔍 BREAKPOINT HIT: $BREAKPOINT\\n
  echo ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\\n
  echo \\n📋 Backtrace:\\n
  bt 10
  echo \\n📊 Local variables:\\n
  info locals
  echo \\n📍 Arguments:\\n
  info args
  echo \\n💾 Receiver value (if available):\\n
  print receiver
  echo \\n🔧 Method name (if available):\\n
  print method
  echo \\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\\n
  continue
end

# Run the program
run run "$RUCHY_FILE"

# If we get here, program exited
echo \\n✅ Program completed\\n
quit
EOF

echo "🔍 Running automated rust-gdb session..."
echo "📋 Breakpoint: $BREAKPOINT"
[ -n "$CONDITION" ] && echo "🎯 Condition: $CONDITION"
echo "📄 File: $RUCHY_FILE"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Run rust-gdb in batch mode
rust-gdb -batch -x "$GDB_COMMANDS" "$RUCHY_BIN" 2>&1

# Cleanup
rm "$GDB_COMMANDS"
