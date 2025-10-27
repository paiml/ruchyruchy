# GitHub Issue: ruchy parser reports misleading brace errors

**To file at**: https://github.com/paiml/ruchy/issues

---

## Title
Parser reports misleading brace errors on valid Ruchy syntax

## Labels
- `bug`
- `parser`
- `priority: high`

## Description

The Ruchy parser reports `Expected RightBrace, found Identifier` errors on syntactically valid code. The error message is misleading as the line numbers don't correspond to actual syntax errors, and manual inspection shows all braces are properly balanced.

## Environment

- **Ruchy Version**: (run `ruchy --version`)
- **OS**: Linux
- **Project**: RuchyRuchy Bootstrap Compiler
- **Context**: QUALITY-002 implementation (Dead Code Detection)

## Reproduction Steps

1. Create a Ruchy file with multiple functions and struct definitions
2. File size: ~370 lines
3. Content includes:
   - 8 test functions
   - 8 stub function implementations
   - 6 struct type definitions
4. Run `ruchy check <file>`

## Expected Behavior

File should pass syntax check - all functions are complete, all braces balanced.

## Actual Behavior

Parser reports:
```
✗ validation/quality/dead_code_test.ruchy:370: Syntax error: Expected RightBrace, found Identifier("println")
Error: validation/quality/dead_code_test.ruchy:370: Syntax error: Expected RightBrace, found Identifier("println")
```

**Issue**:
- Line 370 contains only `}` (closing brace of a struct)
- No "println" identifier on or near line 370
- Error message is misleading

## Analysis

Brace counting on first 200 lines:
```bash
grep -o "{" file.ruchy | wc -l  # Returns: 38
grep -o "}" file.ruchy | wc -l  # Returns: 36
```

**Apparent issue**: Missing 2 closing braces according to grep count

**However**: Manual inspection shows all functions and structs are properly closed

**Hypothesis**: Parser may be:
1. Miscounting braces in certain contexts (string literals, comments)
2. Having issues with specific syntax patterns
3. Reporting incorrect line numbers for errors

## Minimal Reproduction

Unable to create minimal reproduction - simple files work fine.

**Working**: Simple 2-struct, 1-function file (~20 lines)
**Failing**: Complex multi-function file (~370 lines)

**Suggests**: Issue may be related to file complexity, not specific syntax pattern

## Impact

**BLOCKING**: Cannot implement QUALITY-002 (Dead Code Detection)
- Cannot create test files for quality discovery tools
- Blocks entire CYCLE 6 quality discovery implementation
- Violates dogfooding principle (cannot use Ruchy on Ruchy code)

## Workaround

None available. File must be simplified or split, which defeats the purpose of comprehensive testing.

## Additional Context

This is the second parser/tooling issue discovered during CYCLE 6 implementation:
1. ISSUE #1: `ruchy fmt` inconsistent behavior
2. ISSUE #2: This parser error (current issue)

Both issues block quality discovery tool implementation in RuchyRuchy project.

## Request

1. Improve error messages - show actual problematic line/code
2. Fix brace tracking - ensure accurate counting
3. Add test case for complex files (300+ lines, multiple functions/structs)
4. Consider internal AST validation to catch parser state issues

## Related Files

File attempted (deleted due to errors):
- `validation/quality/dead_code_test.ruchy` (~370 lines)
- 8 test functions
- 8 stub implementations
- 6 struct definitions

Similar working file for comparison:
- `validation/quality/tdg_system_test.ruchy` (~450 lines)
- Note: This file had similar complexity and also encountered issues

## Next Steps

1. File this GitHub issue
2. Mark QUALITY-002 as blocked in RuchyRuchy roadmap
3. Pivot to alternative work (EDUCATION-002) to avoid Ruchy parser
4. Resume QUALITY cycle when bug is fixed

---

**Project**: RuchyRuchy Bootstrap Compiler
**Ticket**: QUALITY-002 (blocked)
**Date**: 2025-10-27
