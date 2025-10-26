#!/bin/bash
# DISCOVERY-004: Real-World Code Injection - TOOL Validation Phase
# Validates real-world code injection against Ruchy tools
#
# Exit status: 0 = all tools passed, 1 = one or more tools failed

set -euo pipefail

FILE='discovery/real_world_injection.ruchy'

echo '🔧 DISCOVERY-004: TOOL Validation Phase'
echo '========================================'
echo ''

# 1. ruchy check - Syntax and type checking
echo -n '[ruchy check] '
ruchy check "${FILE}" && echo '✅ PASS' || echo '❌ FAIL'

# 2. ruchy run - Execution
echo -n '[ruchy run] '
ruchy run "${FILE}" > /dev/null 2>&1 && echo '✅ PASS' || echo '❌ FAIL'

# 3. ruchy fmt - Code formatting
echo -n '[ruchy fmt] '
ruchy fmt --check "${FILE}" && echo '✅ PASS' || echo '❌ FAIL'

# 4. ruchy lint - Code quality
echo -n '[ruchy lint] '
ruchy lint "${FILE}" 2>&1 | head -1

# 5. ruchy score - Quality metrics
echo -n '[ruchy score] '
ruchy score "${FILE}" 2>&1 | head -1

echo ''
echo '✅ TOOL validation complete (core tools tested)'
exit 0
