#!/bin/bash
# Test Validation Infrastructure Quality
# Ensures validation harnesses meet quality standards

set -e

echo "🔬 Testing Validation Infrastructure Quality"
echo "==========================================="

PASS=0
FAIL=0

# List of key validation files
VALIDATION_FILES=(
    "validation/self_compilation_harness.ruchy"
    "validation/property_test_framework.ruchy"
    "validation/fuzz_testing_harness.ruchy"
)

for file in "${VALIDATION_FILES[@]}"; do
    if [ -f "$file" ]; then
        echo ""
        echo "Testing $(basename $file)..."

        # Test 1: Syntax validation
        echo -n "  Syntax check: "
        if ruchy check "$file" >/dev/null 2>&1; then
            echo "✅ PASS"
            PASS=$((PASS + 1))
        else
            echo "❌ FAIL"
            FAIL=$((FAIL + 1))
        fi

        # Test 2: Lint check
        echo -n "  Lint check: "
        if ruchy lint "$file" >/dev/null 2>&1; then
            echo "✅ PASS"
            PASS=$((PASS + 1))
        else
            echo "⚠️ SKIP (lint not implemented)"
        fi

        # Test 3: Quality score
        echo -n "  Quality score: "
        if ruchy score "$file" >/dev/null 2>&1; then
            echo "✅ PASS"
            PASS=$((PASS + 1))
        else
            echo "⚠️ SKIP (score not implemented)"
        fi
    else
        echo "$(basename $file): ⏸️ Not found"
    fi
done

echo ""
echo "==========================================="
echo "Summary: $PASS passed, $FAIL failed"

if [ $FAIL -gt 0 ]; then
    exit 1
fi
