#!/bin/bash
# validate-wasm-001.sh
# Validates the WASM-001 implementation using all Ruchy tools
# Exit status: 0 = success, 1 = failure

set -euo pipefail

echo "🧰 Running TOOL phase validation for WASM-001"
echo "=============================================="

# Paths to validate
WASM_EMITTER_PATH="bootstrap/stage3/wasm_emitter_refactored.ruchy"
WASM_TEST_PATH="validation/wasm/test_wasm_emitter_refactored.ruchy"

# Output files
VALIDATION_RESULTS="validation/wasm/tool_validation_results.md"
VALIDATION_SUMMARY="validation/wasm/tool_validation_summary.md"

# Create validation results directory if it doesn't exist
mkdir -p "validation/wasm"

# Initialize results file
cat > "$VALIDATION_RESULTS" << EOF
# WASM-001: Tool Validation Results

This document contains the results of running all 16 Ruchy tools on the WASM Type Mapping implementation.

## Tool Results

EOF

# Function to run a tool and record the result
run_tool() {
    local tool_name=$1
    local tool_command=$2
    local success_criteria=$3
    
    echo -e "\n🔍 Running $tool_name..."
    echo -e "\n### $tool_name\n\n\`\`\`bash\n$tool_command\n\`\`\`\n" >> "$VALIDATION_RESULTS"
    
    # Run the tool and capture output
    if output=$(eval "$tool_command" 2>&1); then
        echo "✅ $tool_name passed"
        echo -e "\n**Result**: ✅ PASS\n\n\`\`\`\n$output\n\`\`\`\n" >> "$VALIDATION_RESULTS"
        echo -e "\n**Success Criteria**: $success_criteria\n" >> "$VALIDATION_RESULTS"
        return 0
    else
        echo "❌ $tool_name failed"
        echo -e "\n**Result**: ❌ FAIL\n\n\`\`\`\n$output\n\`\`\`\n" >> "$VALIDATION_RESULTS"
        echo -e "\n**Success Criteria**: $success_criteria\n" >> "$VALIDATION_RESULTS"
        return 1
    fi
}

# Track overall success
OVERALL_SUCCESS=true

# 1. Syntax and Type Checking
if ! run_tool "ruchy check" "ruchy check $WASM_EMITTER_PATH" "No syntax or type errors"; then
    OVERALL_SUCCESS=false
fi

# 2. Test Execution
if ! run_tool "ruchy test" "ruchy test $WASM_TEST_PATH" "All tests pass"; then
    OVERALL_SUCCESS=false
fi

# 3. Linting
if ! run_tool "ruchy lint" "ruchy lint $WASM_EMITTER_PATH" "A+ grade"; then
    OVERALL_SUCCESS=false
fi

# 4. Formatting
if ! run_tool "ruchy fmt" "ruchy fmt --check $WASM_EMITTER_PATH" "No formatting issues"; then
    OVERALL_SUCCESS=false
fi

# 5. Formal Verification
if ! run_tool "ruchy prove" "ruchy prove $WASM_EMITTER_PATH" "All properties verified"; then
    OVERALL_SUCCESS=false
fi

# 6. Quality Scoring
if ! run_tool "ruchy score" "ruchy score $WASM_EMITTER_PATH" "Score >0.8"; then
    OVERALL_SUCCESS=false
fi

# 7. Runtime Analysis
if ! run_tool "ruchy runtime" "ruchy runtime $WASM_EMITTER_PATH" "Acceptable complexity"; then
    OVERALL_SUCCESS=false
fi

# 8. Build
if ! run_tool "ruchy build" "ruchy build $WASM_EMITTER_PATH" "Successful compilation"; then
    OVERALL_SUCCESS=false
fi

# 9. Run
if ! run_tool "ruchy run" "ruchy run $WASM_TEST_PATH" "Successful execution"; then
    OVERALL_SUCCESS=false
fi

# 10. Documentation
if ! run_tool "ruchy doc" "ruchy doc $WASM_EMITTER_PATH" "Documentation generated"; then
    OVERALL_SUCCESS=false
fi

# 11. Benchmarking
if ! run_tool "ruchy bench" "ruchy bench $WASM_EMITTER_PATH" "Performance within targets"; then
    OVERALL_SUCCESS=false
fi

# 12. Profiling
if ! run_tool "ruchy profile" "ruchy profile $WASM_EMITTER_PATH" "No performance issues"; then
    OVERALL_SUCCESS=false
fi

# 13. Coverage
if ! run_tool "ruchy coverage" "ruchy coverage $WASM_TEST_PATH" ">80% coverage"; then
    OVERALL_SUCCESS=false
fi

# 14. Dependency Analysis
if ! run_tool "ruchy deps" "ruchy deps $WASM_EMITTER_PATH" "No unnecessary dependencies"; then
    OVERALL_SUCCESS=false
fi

# 15. Security Scanning
if ! run_tool "ruchy security" "ruchy security $WASM_EMITTER_PATH" "No security issues"; then
    OVERALL_SUCCESS=false
fi

# 16. Complexity Analysis
if ! run_tool "ruchy complexity" "ruchy complexity $WASM_EMITTER_PATH" "Complexity <20 per function"; then
    OVERALL_SUCCESS=false
fi

# Create validation summary
cat > "$VALIDATION_SUMMARY" << EOF
# WASM-001: Tool Validation Summary

## Overview

This document summarizes the results of running all 16 Ruchy tools on the WASM Type Mapping implementation.

## Summary

EOF

if $OVERALL_SUCCESS; then
    echo -e "\n✅ All tools passed validation!"
    echo -e "✅ **Overall Result**: PASS\n" >> "$VALIDATION_SUMMARY"
else
    echo -e "\n❌ Some tools failed validation. See $VALIDATION_RESULTS for details."
    echo -e "❌ **Overall Result**: FAIL\n" >> "$VALIDATION_SUMMARY"
fi

# Add tool results table to summary
cat >> "$VALIDATION_SUMMARY" << EOF
## Tool Results

| Tool | Status | Notes |
|------|--------|-------|
EOF

# Function to extract result from validation results
extract_result() {
    local tool_name=$1
    if grep -q "### $tool_name\n\n\`\`\`bash" "$VALIDATION_RESULTS" && grep -q "**Result**: ✅ PASS" "$VALIDATION_RESULTS"; then
        echo "✅ PASS"
    else
        echo "❌ FAIL"
    fi
}

# Add each tool to the table
echo "| ruchy check | $(extract_result "ruchy check") | Syntax and type checking |" >> "$VALIDATION_SUMMARY"
echo "| ruchy test | $(extract_result "ruchy test") | Test execution |" >> "$VALIDATION_SUMMARY"
echo "| ruchy lint | $(extract_result "ruchy lint") | Code quality analysis |" >> "$VALIDATION_SUMMARY"
echo "| ruchy fmt | $(extract_result "ruchy fmt") | Code formatting |" >> "$VALIDATION_SUMMARY"
echo "| ruchy prove | $(extract_result "ruchy prove") | Formal verification |" >> "$VALIDATION_SUMMARY"
echo "| ruchy score | $(extract_result "ruchy score") | Quality metrics |" >> "$VALIDATION_SUMMARY"
echo "| ruchy runtime | $(extract_result "ruchy runtime") | Performance analysis |" >> "$VALIDATION_SUMMARY"
echo "| ruchy build | $(extract_result "ruchy build") | Compilation |" >> "$VALIDATION_SUMMARY"
echo "| ruchy run | $(extract_result "ruchy run") | Execution |" >> "$VALIDATION_SUMMARY"
echo "| ruchy doc | $(extract_result "ruchy doc") | Documentation generation |" >> "$VALIDATION_SUMMARY"
echo "| ruchy bench | $(extract_result "ruchy bench") | Benchmarking |" >> "$VALIDATION_SUMMARY"
echo "| ruchy profile | $(extract_result "ruchy profile") | Performance profiling |" >> "$VALIDATION_SUMMARY"
echo "| ruchy coverage | $(extract_result "ruchy coverage") | Code coverage |" >> "$VALIDATION_SUMMARY"
echo "| ruchy deps | $(extract_result "ruchy deps") | Dependency analysis |" >> "$VALIDATION_SUMMARY"
echo "| ruchy security | $(extract_result "ruchy security") | Security scanning |" >> "$VALIDATION_SUMMARY"
echo "| ruchy complexity | $(extract_result "ruchy complexity") | Complexity analysis |" >> "$VALIDATION_SUMMARY"

cat >> "$VALIDATION_SUMMARY" << EOF

## Next Steps

Based on these results, the next steps are:

1. ${OVERALL_SUCCESS:+"Proceed to the next phase - WASM-002: Closure Compilation"}${OVERALL_SUCCESS:-"Fix the issues identified by the failing tools"}
2. ${OVERALL_SUCCESS:+"Document the successful TOOL phase completion"}${OVERALL_SUCCESS:-"Re-run validation until all tools pass"}

## Conclusion

The WASM-001 Type Mapping implementation has ${OVERALL_SUCCESS:+"successfully passed"}${OVERALL_SUCCESS:-"not yet passed"} all 16 Ruchy tools validation. ${OVERALL_SUCCESS:+"This confirms the implementation meets our quality standards."}${OVERALL_SUCCESS:-"The issues must be addressed before proceeding to the next phase."}

For detailed results, see [Tool Validation Results](./tool_validation_results.md).
EOF

echo -e "\nValidation summary written to $VALIDATION_SUMMARY"

# Set exit status based on overall success
if $OVERALL_SUCCESS; then
    exit 0
else
    exit 1
fi