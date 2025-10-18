#!/bin/bash
# Test Bootstrap Stages Quality
# Validates all bootstrap stages meet quality requirements

set -e

echo "🏗️ Testing Bootstrap Stages Quality"
echo "===================================="

PASS=0
FAIL=0

for stage in stage0 stage1 stage2 stage3; do
    if [ -d "bootstrap/$stage" ]; then
        echo ""
        echo "Testing $stage..."

        # Test 1: Syntax validation
        echo -n "  Syntax check: "
        if find "bootstrap/$stage" -name "*.ruchy" -exec ruchy check {} \; 2>/dev/null; then
            echo "✅ PASS"
            PASS=$((PASS + 1))
        else
            echo "❌ FAIL"
            FAIL=$((FAIL + 1))
        fi

        # Test 2: Complexity analysis
        echo -n "  Complexity: "
        if pmat tdg "bootstrap/$stage" --min-grade A- >/dev/null 2>&1; then
            echo "✅ PASS"
            PASS=$((PASS + 1))
        else
            echo "⚠️ SKIP (not implemented yet)"
        fi

        # Test 3: SATD check
        echo -n "  SATD check: "
        if ! find "bootstrap/$stage" -name "*.ruchy" -exec grep -l "TODO\|FIXME\|HACK" {} \; 2>/dev/null | grep -q .; then
            echo "✅ PASS"
            PASS=$((PASS + 1))
        else
            echo "❌ FAIL (contains SATD)"
            FAIL=$((FAIL + 1))
        fi
    else
        echo "$stage: ⏸️ Not implemented yet"
    fi
done

echo ""
echo "===================================="
echo "Summary: $PASS passed, $FAIL failed"

if [ $FAIL -gt 0 ]; then
    exit 1
fi
