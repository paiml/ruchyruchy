#!/bin/bash
# O(1) metric validation for pre-commit hooks
# Adapted from ../paiml-mcp-agent-toolkit/scripts/validate-metrics.sh
# Validates metrics against thresholds in .pmat-metrics.toml
set -euo pipefail

METRICS_DIR=".pmat-metrics"
CONFIG_FILE=".pmat-metrics.toml"
FAILURES_ONLY="${1:-false}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default thresholds (can be overridden by .pmat-metrics.toml)
LINT_MAX_MS=15000       # 15 seconds for lint (smaller project than paiml-mcp)
TEST_FAST_MAX_MS=120000  # 2 minutes for fast tests
COVERAGE_MAX_MS=180000   # 3 minutes for coverage
BINARY_MAX_BYTES=20000000 # 20MB for ruchydbg binary
MAX_AGE_DAYS=7

# Check if metrics directory exists
if [ ! -d "$METRICS_DIR" ]; then
    echo -e "${YELLOW}⚠️  No metrics cache found (.pmat-metrics/)${NC}"
    echo "   Run 'make lint' or 'make test-fast' to generate metrics"
    exit 0
fi

# Function to check if metric is stale
is_stale() {
    local file=$1
    local max_age_seconds=$((MAX_AGE_DAYS * 24 * 60 * 60))

    if [ ! -f "$file" ]; then
        return 0 # Missing = stale
    fi

    local file_time
    file_time="$(stat -c %Y "$file" 2>/dev/null || stat -f %m "$file" 2>/dev/null)"
    local current_time
    current_time="$(date +%s)"
    local age_seconds=$((current_time - file_time))

    [ "$age_seconds" -gt "$max_age_seconds" ]
}

# Function to extract JSON value (simple grep-based)
json_value() {
    local file=$1
    local key=$2
    grep "\"$key\"" "$file" | sed -E 's/.*: ([0-9.]+).*/\1/' | head -1
}

# Validation counters
VIOLATIONS=0
WARNINGS=0

echo -e "${BLUE}🔍 RuchyRuchy Quality Gate Validation (O(1))${NC}"
echo ""

# Validate lint
if [ -f "$METRICS_DIR/lint.result" ]; then
    DURATION_MS="$(json_value "$METRICS_DIR/lint.result" "duration_ms")"
    if is_stale "$METRICS_DIR/lint.result"; then
        echo -e "${YELLOW}⚠️  make lint: Stale (>$MAX_AGE_DAYS days old)${NC}"
        WARNINGS=$((WARNINGS + 1))
    elif [ "${DURATION_MS%.*}" -gt "$LINT_MAX_MS" ]; then
        HEADROOM=$(echo "scale=1; 100 * ($DURATION_MS - $LINT_MAX_MS) / $LINT_MAX_MS" | bc)
        echo -e "${RED}❌ make lint: ${DURATION_MS}ms (exceeds ${LINT_MAX_MS}ms threshold by ${HEADROOM}%)${NC}"
        VIOLATIONS=$((VIOLATIONS + 1))
    elif [ "$FAILURES_ONLY" != "--failures-only" ]; then
        HEADROOM=$(echo "scale=1; 100 * ($LINT_MAX_MS - $DURATION_MS) / $LINT_MAX_MS" | bc)
        echo -e "${GREEN}✅ make lint: ${DURATION_MS}ms (${HEADROOM}% headroom, target: ${LINT_MAX_MS}ms)${NC}"
    fi
fi

# Validate test-fast
if [ -f "$METRICS_DIR/test-fast.result" ]; then
    DURATION_MS="$(json_value "$METRICS_DIR/test-fast.result" "duration_ms")"
    if is_stale "$METRICS_DIR/test-fast.result"; then
        echo -e "${YELLOW}⚠️  make test-fast: Stale (>$MAX_AGE_DAYS days old)${NC}"
        WARNINGS=$((WARNINGS + 1))
    elif [ "${DURATION_MS%.*}" -gt "$TEST_FAST_MAX_MS" ]; then
        HEADROOM=$(echo "scale=1; 100 * ($DURATION_MS - $TEST_FAST_MAX_MS) / $TEST_FAST_MAX_MS" | bc)
        echo -e "${RED}❌ make test-fast: ${DURATION_MS}ms (exceeds ${TEST_FAST_MAX_MS}ms threshold by ${HEADROOM}%)${NC}"
        VIOLATIONS=$((VIOLATIONS + 1))
    elif [ "$FAILURES_ONLY" != "--failures-only" ]; then
        HEADROOM=$(echo "scale=1; 100 * ($TEST_FAST_MAX_MS - $DURATION_MS) / $TEST_FAST_MAX_MS" | bc)
        DURATION_SEC=$((DURATION_MS / 1000))
        DURATION_MIN=$((DURATION_SEC / 60))
        DURATION_REMAIN=$((DURATION_SEC % 60))
        echo -e "${GREEN}✅ make test-fast: ${DURATION_MIN}m ${DURATION_REMAIN}s (${HEADROOM}% headroom)${NC}"
    fi
fi

# Validate coverage
if [ -f "$METRICS_DIR/coverage.result" ]; then
    DURATION_MS="$(json_value "$METRICS_DIR/coverage.result" "duration_ms")"
    COVERAGE_PCT="$(json_value "$METRICS_DIR/coverage.result" "coverage_pct")"
    if is_stale "$METRICS_DIR/coverage.result"; then
        echo -e "${YELLOW}⚠️  make coverage: Stale (>$MAX_AGE_DAYS days old)${NC}"
        WARNINGS=$((WARNINGS + 1))
    elif [ "${DURATION_MS%.*}" -gt "$COVERAGE_MAX_MS" ]; then
        HEADROOM=$(echo "scale=1; 100 * ($DURATION_MS - $COVERAGE_MAX_MS) / $COVERAGE_MAX_MS" | bc)
        echo -e "${RED}❌ make coverage: ${DURATION_MS}ms (exceeds ${COVERAGE_MAX_MS}ms threshold by ${HEADROOM}%)${NC}"
        VIOLATIONS=$((VIOLATIONS + 1))
    elif [ "$FAILURES_ONLY" != "--failures-only" ]; then
        HEADROOM=$(echo "scale=1; 100 * ($COVERAGE_MAX_MS - $DURATION_MS) / $COVERAGE_MAX_MS" | bc)
        DURATION_SEC=$((DURATION_MS / 1000))
        DURATION_MIN=$((DURATION_SEC / 60))
        DURATION_REMAIN=$((DURATION_SEC % 60))
        echo -e "${GREEN}✅ make coverage: ${DURATION_MIN}m ${DURATION_REMAIN}s (${COVERAGE_PCT}% coverage, ${HEADROOM}% headroom)${NC}"
    fi
fi

# Validate binary size
if [ -f "$METRICS_DIR/build-release.result" ]; then
    BINARY_SIZE="$(json_value "$METRICS_DIR/build-release.result" "binary_size")"
    if is_stale "$METRICS_DIR/build-release.result"; then
        echo -e "${YELLOW}⚠️  make release: Stale (>$MAX_AGE_DAYS days old)${NC}"
        WARNINGS=$((WARNINGS + 1))
    elif [ "${BINARY_SIZE%.*}" -gt "$BINARY_MAX_BYTES" ]; then
        BINARY_MB=$(echo "scale=1; $BINARY_SIZE / 1048576" | bc)
        MAX_MB=$(echo "scale=1; $BINARY_MAX_BYTES / 1048576" | bc)
        echo -e "${RED}❌ ruchydbg binary: ${BINARY_MB}MB (exceeds ${MAX_MB}MB threshold)${NC}"
        VIOLATIONS=$((VIOLATIONS + 1))
    elif [ "$FAILURES_ONLY" != "--failures-only" ]; then
        BINARY_MB=$(echo "scale=1; $BINARY_SIZE / 1048576" | bc)
        MAX_MB=$(echo "scale=1; $BINARY_MAX_BYTES / 1048576" | bc)
        echo -e "${GREEN}✅ ruchydbg binary: ${BINARY_MB}MB (target: <${MAX_MB}MB)${NC}"
    fi
fi

echo ""

# Summary
if [ $VIOLATIONS -gt 0 ]; then
    echo -e "${RED}❌ $VIOLATIONS violation(s), $WARNINGS warning(s)${NC}"
    echo "   Run the failing targets to update metrics and re-commit"
    exit 1
elif [ $WARNINGS -gt 0 ]; then
    echo -e "${YELLOW}⚠️  $WARNINGS warning(s) (stale metrics)${NC}"
    echo "   Consider refreshing metrics with 'make lint test-fast'"
    exit 0
else
    echo -e "${GREEN}✅ All quality gates passed!${NC}"
    exit 0
fi
