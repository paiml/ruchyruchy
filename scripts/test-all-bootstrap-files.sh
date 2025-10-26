#!/bin/bash
# TESTING-001: Apply Extreme Testing to Bootstrap Stages
# Systematically test ALL bootstrap files with ruchy tooling
#
# Exit status: 0 = all passed, 1 = failures detected

set -euo pipefail

TOTAL_FILES=0
PASSED_CHECK=0
PASSED_RUN=0
FAILED_CHECK=0
FAILED_RUN=0
ERRORS_FOUND=0

echo '🧪 TESTING-001: Bootstrap Stage Comprehensive Testing'
echo '====================================================='
echo ''

# Function to test a single file
test_file() {
    local file="$1"
    TOTAL_FILES=$((TOTAL_FILES + 1))

    echo "Testing: ${file}"

    # Test 1: ruchy check (syntax validation)
    if ruchy check "${file}" > /dev/null 2>&1; then
        echo "  ✅ ruchy check: PASS"
        PASSED_CHECK=$((PASSED_CHECK + 1))
    else
        echo "  ❌ ruchy check: FAIL"
        ruchy check "${file}" 2>&1 | head -5 | sed 's/^/     /'
        FAILED_CHECK=$((FAILED_CHECK + 1))
        ERRORS_FOUND=$((ERRORS_FOUND + 1))
    fi

    # Test 2: ruchy run (execution test)
    if timeout 5 ruchy run "${file}" > /dev/null 2>&1; then
        echo "  ✅ ruchy run: PASS"
        PASSED_RUN=$((PASSED_RUN + 1))
    else
        local exit_code=$?
        if [ $exit_code -eq 124 ]; then
            echo "  ⏱️  ruchy run: TIMEOUT (>5s)"
        else
            echo "  ❌ ruchy run: FAIL (exit code: $exit_code)"
        fi
        FAILED_RUN=$((FAILED_RUN + 1))
        ERRORS_FOUND=$((ERRORS_FOUND + 1))
    fi

    echo ""
}

# Test Stage 0 files
echo '📁 Testing Stage 0 Files'
echo '────────────────────────'
for file in bootstrap/stage0/*.ruchy; do
    if [ -f "${file}" ]; then
        test_file "${file}"
    fi
done

echo ''
echo '📁 Testing Stage 1 Files'
echo '────────────────────────'
for file in bootstrap/stage1/*.ruchy; do
    if [ -f "${file}" ]; then
        test_file "${file}"
    fi
done

# Summary
echo '═══════════════════════════════════════════════════'
echo '📊 TESTING SUMMARY'
echo '═══════════════════════════════════════════════════'
echo "Total files tested: ${TOTAL_FILES}"
echo ""
echo "ruchy check results:"
echo "  ✅ Passed: ${PASSED_CHECK}/${TOTAL_FILES}"
echo "  ❌ Failed: ${FAILED_CHECK}/${TOTAL_FILES}"
echo ""
echo "ruchy run results:"
echo "  ✅ Passed: ${PASSED_RUN}/${TOTAL_FILES}"
echo "  ❌ Failed: ${FAILED_RUN}/${TOTAL_FILES}"
echo ""
echo "Total errors found: ${ERRORS_FOUND}"
echo ""

if [ ${ERRORS_FOUND} -gt 0 ]; then
    echo "⚠️  TESTING-001: FAILURES DETECTED"
    echo "Action required: File GitHub issues for all failures"
    exit 1
else
    echo "✅ TESTING-001: ALL TESTS PASSED"
    exit 0
fi
