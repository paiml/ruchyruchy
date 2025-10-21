#!/bin/bash
# validate-debugging-tools.sh
#
# Fast-feedback debugging tools validation for ../ruchy pre-commit hooks
#
# Integration with ../ruchy pre-commit hook:
#   - Source map validation: <2 seconds
#   - Time-travel smoke test: <3 seconds
#   - Performance regression: <1 second
#   - Total: <6 seconds (fast feedback!)
#
# Usage:
#   ./scripts/validate-debugging-tools.sh
#
# Returns:
#   0 if all validations pass
#   1 if any validation fails

# Path to ruchydbg tool (relative to ruchyruchy repository)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RUCHY_DBG_TOOL="$SCRIPT_DIR/../validation/debugging/ruchydbg.ruchy"

# Check if ruchy is available
if ! command -v ruchy &> /dev/null; then
    echo "⚠️  Warning: ruchy not found in PATH"
    echo "   Install Ruchy to enable debugging tools validation"
    exit 0  # Non-blocking
fi

# Check if ruchydbg tool exists
if [ ! -f "$RUCHY_DBG_TOOL" ]; then
    echo "⚠️  Warning: ruchydbg tool not found at $RUCHY_DBG_TOOL"
    exit 0  # Non-blocking
fi

# Run all validations (ruchy run doesn't support args yet, so tool defaults to "all")
OUTPUT=$(ruchy run "$RUCHY_DBG_TOOL" 2>&1)
EXIT_CODE=$?

# Display output
echo "$OUTPUT"

# Check if validation passed (look for success indicator in output)
if echo "$OUTPUT" | grep -q "Exit code: 0"; then
    exit 0
else
    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "RuchyRuchy debugging tools validation failed."
    echo "This indicates a regression in debugging capabilities."
    echo ""
    echo "To debug:"
    echo "  ruchy run $RUCHY_DBG_TOOL"
    echo ""
    echo "To bypass (NOT RECOMMENDED): git commit --no-verify"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    exit 1
fi
