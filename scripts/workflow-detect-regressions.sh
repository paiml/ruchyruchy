#!/bin/bash
# Workflow: Detect Regressions
#
# Detect regressions between compiler versions using differential testing.
# This script compares test results between two versions to find regressions.
#
# Usage:
#   ./workflow-detect-regressions.sh [old-version] [new-version] [test-suite-dir]
#
# Example:
#   ./workflow-detect-regressions.sh v3.145.0 v3.146.0 tests/

set -euo pipefail

# Configuration
OLD_VERSION="${1:-v3.145.0}"
NEW_VERSION="${2:-v3.146.0}"
TEST_SUITE="${3:-tests/}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔍 Regression Detection Workflow${NC}"
echo "===================================="
echo ""
echo "Old version: $OLD_VERSION"
echo "New version: $NEW_VERSION"
echo "Test suite:  $TEST_SUITE"
echo ""

# Step 1: Verify versions exist
echo -e "${BLUE}Step 1: Verify compiler versions...${NC}"

if ! command -v ruchy &> /dev/null; then
    echo -e "${RED}   ❌ ruchy not found in PATH${NC}"
    echo "   Install: cargo install ruchy"
    exit 1
fi

CURRENT_VERSION=$(ruchy --version 2>&1 | grep -oP 'v\d+\.\d+\.\d+' || echo "unknown")
echo -e "${GREEN}   ✅ Current Ruchy version: $CURRENT_VERSION${NC}"

if [ "$CURRENT_VERSION" != "$NEW_VERSION" ]; then
    echo -e "${YELLOW}   ⚠️  Warning: Testing with $CURRENT_VERSION, expected $NEW_VERSION${NC}"
fi
echo ""

# Step 2: Run tests on current version
echo -e "${BLUE}Step 2: Run tests on current version ($CURRENT_VERSION)...${NC}"

TEST_COUNT=0
PASS_COUNT=0
FAIL_COUNT=0

mkdir -p regression_results

for test_file in "$TEST_SUITE"/*.ruchy; do
    if [ ! -f "$test_file" ]; then
        continue
    fi

    TEST_COUNT=$((TEST_COUNT + 1))
    test_name=$(basename "$test_file")

    echo -n "   Testing $test_name... "

    if ruchy test "$test_file" > "regression_results/${test_name}.out" 2>&1; then
        echo -e "${GREEN}✅${NC}"
        PASS_COUNT=$((PASS_COUNT + 1))
        echo "PASS" > "regression_results/${test_name}.status"
    else
        echo -e "${RED}❌${NC}"
        FAIL_COUNT=$((FAIL_COUNT + 1))
        echo "FAIL" > "regression_results/${test_name}.status"
    fi
done

echo ""
echo -e "${GREEN}   📊 Results: $PASS_COUNT passed, $FAIL_COUNT failed (of $TEST_COUNT tests)${NC}"
echo ""

# Step 3: Compare with baseline (if exists)
echo -e "${BLUE}Step 3: Compare with baseline...${NC}"

if [ -d "regression_baseline" ]; then
    echo -e "${GREEN}   ✅ Baseline found: regression_baseline/${NC}"
    echo ""

    REGRESSION_COUNT=0
    NEW_FAILURE_COUNT=0

    for test_file in "$TEST_SUITE"/*.ruchy; do
        if [ ! -f "$test_file" ]; then
            continue
        fi

        test_name=$(basename "$test_file")
        current_status=$(cat "regression_results/${test_name}.status" 2>/dev/null || echo "UNKNOWN")
        baseline_status=$(cat "regression_baseline/${test_name}.status" 2>/dev/null || echo "UNKNOWN")

        if [ "$baseline_status" = "PASS" ] && [ "$current_status" = "FAIL" ]; then
            echo -e "${RED}   ❌ REGRESSION: $test_name (PASS -> FAIL)${NC}"
            REGRESSION_COUNT=$((REGRESSION_COUNT + 1))

            # Create regression report
            cat > "regression_results/${test_name}.regression" <<EOF
# Regression Report: $test_name

## Status Change
- Baseline ($OLD_VERSION): PASS
- Current ($CURRENT_VERSION): FAIL

## Error Output
\`\`\`
$(cat "regression_results/${test_name}.out")
\`\`\`

## Test File
\`\`\`ruchy
$(cat "$test_file")
\`\`\`

## Investigation Steps
1. Bisect between $OLD_VERSION and $CURRENT_VERSION
2. Minimize failing test case
3. File GitHub issue if confirmed bug
EOF
        elif [ "$baseline_status" = "UNKNOWN" ] && [ "$current_status" = "FAIL" ]; then
            echo -e "${YELLOW}   ⚠️  NEW FAILURE: $test_name (no baseline)${NC}"
            NEW_FAILURE_COUNT=$((NEW_FAILURE_COUNT + 1))
        fi
    done

    echo ""

    if [ "$REGRESSION_COUNT" -gt 0 ]; then
        echo -e "${RED}❌ Found $REGRESSION_COUNT regressions${NC}"
        echo ""
        echo "Regression reports:"
        for regression_file in regression_results/*.regression; do
            if [ -f "$regression_file" ]; then
                echo "  - $regression_file"
            fi
        done
        echo ""
        echo -e "${YELLOW}💡 Next steps:${NC}"
        echo "   1. Review regression reports"
        echo "   2. Bisect to find introducing commit: git bisect start $NEW_VERSION $OLD_VERSION"
        echo "   3. Minimize failing tests with delta debugging"
        echo "   4. File GitHub issues for confirmed regressions"
        exit 1
    else
        echo -e "${GREEN}✅ No regressions detected${NC}"

        if [ "$NEW_FAILURE_COUNT" -gt 0 ]; then
            echo -e "${YELLOW}   ⚠️  $NEW_FAILURE_COUNT new failures (no baseline to compare)${NC}"
        fi
    fi
else
    echo -e "${YELLOW}   ⚠️  No baseline found - creating new baseline${NC}"
    cp -r regression_results regression_baseline
    echo -e "${GREEN}   ✅ Baseline created: regression_baseline/${NC}"
    echo ""
    echo "   Run this script again after updating to $NEW_VERSION to detect regressions"
fi

echo ""
echo -e "${GREEN}✅ Regression detection complete${NC}"
echo ""
echo "Generated files:"
echo "  - regression_results/ (current test results)"
if [ -d "regression_baseline" ]; then
    echo "  - regression_baseline/ (baseline for comparison)"
fi
if ls regression_results/*.regression 1> /dev/null 2>&1; then
    echo "  - regression_results/*.regression (regression reports)"
fi
