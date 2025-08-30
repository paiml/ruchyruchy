#!/bin/bash
# Quality Gates Script - Following ../ruchy-book Toyota Way Standards
# Zero tolerance for failures - BLOCKING commits that don't meet 100% standards

set -e

GATES_PASSED=0
GATES_FAILED=0

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Toyota Way Quality Gates - Following ../ruchy-book Standards${NC}"
echo "=================================================================="
echo ""

check_gate() {
    local gate_name="$1"
    local command="$2"
    local required="$3"
    
    echo -e "${BLUE}🔍 ${gate_name}${NC}"
    echo "   Command: ${command}"
    echo "   Required: ${required}"
    
    if eval "$command" > /dev/null 2>&1; then
        echo -e "   ${GREEN}✅ PASS${NC}"
        ((GATES_PASSED++))
    else
        echo -e "   ${RED}❌ FAIL${NC}"
        ((GATES_FAILED++))
        
        # Show actual error for debugging
        echo "   Error output:"
        eval "$command" 2>&1 | sed 's/^/   /'
    fi
    echo ""
}

echo -e "${YELLOW}📋 Testing all validation files with 100% coverage requirements${NC}"
echo ""

# Gate 1: Test Compilation (MANDATORY)
check_gate "Test Compilation" \
    "ruchy test validation/self_compilation_harness.ruchy && ruchy test validation/property_test_framework.ruchy && ruchy test validation/fuzz_testing_harness.ruchy" \
    "All validation files must compile and run via ruchy test"

# Gate 2: 100% Coverage (MANDATORY - ../ruchy-book standard)
check_gate "100% Line Coverage" \
    "ruchy test --coverage --threshold 100 validation/self_compilation_harness.ruchy && ruchy test --coverage --threshold 100 validation/property_test_framework.ruchy && ruchy test --coverage --threshold 100 validation/fuzz_testing_harness.ruchy" \
    "100% line coverage on all validation files"

# Gate 3: Lint A+ Grade (MANDATORY - ../ruchy-book standard)
check_gate "Lint A+ Grade" \
    "ruchy lint --strict validation/self_compilation_harness.ruchy && ruchy lint --strict validation/property_test_framework.ruchy && ruchy lint --strict validation/fuzz_testing_harness.ruchy" \
    "A+ grade via ruchy lint --strict"

# Gate 4: Zero SATD (MANDATORY - ../ruchy-book elimination standard)
check_gate "Zero SATD" \
    "! grep -r 'TODO\\|FIXME\\|HACK\\|placeholder\\|unimplemented' validation/" \
    "No SATD (Software Architecture Technical Debt) allowed"

# Gate 5: Formal Verification (MANDATORY)
check_gate "Formal Verification" \
    "ruchy prove validation/self_compilation_harness.ruchy && ruchy prove validation/property_test_framework.ruchy && ruchy prove validation/fuzz_testing_harness.ruchy" \
    "All properties must be formally verified"

# Gate 6: Quality Score >0.8 (MANDATORY - Toyota Way)
check_gate "Quality Score >0.8" \
    "ruchy score validation/self_compilation_harness.ruchy && ruchy score validation/property_test_framework.ruchy && ruchy score validation/fuzz_testing_harness.ruchy" \
    "Quality score >0.8 via ruchy score"

# Gate 7: TDD Test Harness (MANDATORY)
check_gate "TDD Test Harness" \
    "ruchy test scripts/tdd-harness.ruchy" \
    "TDD test harness must compile and run"

echo "=================================================================="
echo -e "${BLUE}📊 QUALITY GATE RESULTS:${NC}"
echo ""
echo -e "   Gates Passed: ${GREEN}${GATES_PASSED}${NC}"
echo -e "   Gates Failed: ${RED}${GATES_FAILED}${NC}"

if [ $GATES_FAILED -eq 0 ]; then
    echo ""
    echo -e "${GREEN}✅ ALL QUALITY GATES PASSED${NC}"
    echo -e "${GREEN}🎉 ../ruchy-book TDD Standard Achieved${NC}"
    echo -e "${GREEN}✅ COMMIT APPROVED${NC}"
    echo ""
    echo -e "${BLUE}📋 Sprint Summary:${NC}"
    echo "   • 100% line coverage achieved"
    echo "   • A+ lint grade maintained"
    echo "   • Zero SATD (TODO/FIXME/HACK) maintained"
    echo "   • Quality score >0.8 maintained"
    echo "   • All properties formally verified"
    echo ""
    exit 0
else
    echo ""
    echo -e "${RED}❌ QUALITY GATES FAILED${NC}"
    echo -e "${RED}🚫 COMMIT BLOCKED${NC}"
    echo ""
    echo -e "${YELLOW}📚 Following ../ruchy-book success pattern:${NC}"
    echo "   1. FIX failing gates above"
    echo "   2. NEVER bypass quality gates"
    echo "   3. DELETE broken examples rather than fix"
    echo "   4. Maintain 100% coverage at all times"
    echo "   5. Achieve A+ lint grade before commit"
    echo ""
    echo -e "${RED}🚫 Will not proceed until ALL gates pass${NC}"
    exit 1
fi