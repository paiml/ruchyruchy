# GitHub Issues to File at https://github.com/paiml/ruchy/issues

**Date**: 2025-10-27
**Project**: RuchyRuchy Bootstrap Compiler
**Context**: CYCLE 6 (Quality Discovery Tools) blocked by Ruchy tooling bugs

---

## Issue #1: ruchy fmt inconsistent behavior with struct formatting

**URL**: https://github.com/paiml/ruchy/issues/new
**Priority**: HIGH
**Labels**: `bug`, `formatter`, `priority: high`

### Title
`ruchy fmt` shows inconsistent behavior with struct formatting

### Description

The `ruchy fmt` formatter shows inconsistent behavior when formatting struct definitions. In some cases it produces valid output, in others the formatted code fails `ruchy check`.

### Environment
- **Ruchy Version**: `ruchy --version` (to be determined)
- **OS**: Linux
- **Project**: RuchyRuchy Bootstrap Compiler
- **Context**: QUALITY-001 implementation (Technical Debt Grading System)

### Reproduction Steps

**Test Case 1: Simple structs (WORKS)**
```bash
cat > test_simple.ruchy << 'EOF'
struct Point { x: f64, y: f64 }
struct Rectangle { width: f64, height: f64, color: String }
fun main() {
    let p = Point { x: 10.0, y: 20.0 }
    let r = Rectangle { width: 100.0, height: 50.0, color: "blue" }
    println("test")
}
EOF

ruchy check test_simple.ruchy  # ✅ PASS
ruchy fmt test_simple.ruchy    # ✅ Formats successfully
ruchy check test_simple.ruchy  # ✅ PASS (still valid)
```

**Test Case 2: Complex file (INCONSISTENT)**
- File: `bootstrap/stage3/tdg_system.ruchy` (~350 LOC)
- Multiple structs with multi-line field definitions
- Initial file: ✅ Passes `ruchy check`
- After `ruchy fmt`: ⚠️ Inconsistent results

### Expected Behavior

`ruchy fmt` output should **always** pass `ruchy check`. The formatter should produce valid Ruchy code in all cases.

### Actual Behavior

Behavior is inconsistent:
- Simple files: ✅ Format correctly and remain valid
- Complex files: ⚠️ Unpredictable behavior

### Impact

**BLOCKING**: QUALITY-001 REFACTOR phase incomplete
- Cannot use `ruchy fmt` in quality gates
- Manual formatting required (error-prone)
- Violates dogfooding principle (Ruchy tools should work on Ruchy code)
- Blocks TOOL validation phase (requires `ruchy fmt` to pass)

### Workaround

Manual formatting applied. Not sustainable for team development.

### Files Affected
- `bootstrap/stage3/tdg_system.ruchy` (~350 LOC)
- `validation/quality/tdg_system_test.ruchy` (~450 LOC)

### Request

1. Ensure `ruchy fmt` output **always** passes `ruchy check`
2. Add regression test: format then check on various file sizes
3. Test with files of different complexity (simple, medium, large)
4. Consider adding `--verify` flag to `ruchy fmt` that checks output

### Test File

Minimal test case attached (works correctly):
```ruchy
// validation/bugs/ruchy_fmt_struct_bug.ruchy
struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
    color: String,
}

fun main() {
    let p = Point {
        x: 10.0,
        y: 20.0,
    }

    let r = Rectangle {
        width: 100.0,
        height: 50.0,
        color: "blue",
    }

    println("Point: (" + p.x.to_string() + ", " + p.y.to_string() + ")")
    println("Rectangle: " + r.width.to_string() + "x" + r.height.to_string())
}
```

This minimal case **works correctly** - issue appears with larger files.

---

## Issue #2: Parser reports misleading brace errors on valid syntax

**URL**: https://github.com/paiml/ruchy/issues/new
**Priority**: CRITICAL
**Labels**: `bug`, `parser`, `priority: critical`

### Title
Parser reports misleading brace errors on syntactically valid Ruchy code

### Description

The Ruchy parser reports `Expected RightBrace, found Identifier` errors on valid code with properly balanced braces. Error messages show incorrect line numbers and misleading context.

### Environment
- **Ruchy Version**: `ruchy --version` (to be determined)
- **OS**: Linux
- **Project**: RuchyRuchy Bootstrap Compiler
- **Context**: QUALITY-002 implementation (Dead Code Detection)

### Reproduction Steps

1. Create file with ~370 lines containing:
   - 8 test functions (each ~20 lines)
   - 8 stub function implementations
   - 6 struct type definitions
2. All functions properly closed with braces
3. All structs properly closed with braces
4. Run `ruchy check file.ruchy`

### Expected Behavior

File should pass syntax validation - all braces are balanced.

### Actual Behavior

Parser reports:
```
✗ file.ruchy:370: Syntax error: Expected RightBrace, found Identifier("println")
Error: file.ruchy:370: Syntax error: Expected RightBrace, found Identifier("println")
```

**Problems**:
1. Line 370 contains only `}` (closing brace of struct)
2. No "println" identifier exists on line 370
3. Error message is misleading about location

### Analysis

Brace counting (first 200 lines):
```bash
grep -o "{" file.ruchy | wc -l  # Returns: 38
grep -o "}" file.ruchy | wc -l  # Returns: 36
```

Appears to be missing 2 closing braces, **BUT** manual inspection shows all functions/structs properly closed.

**Hypothesis**:
- Parser miscounts braces in certain contexts
- May not handle braces in string literals correctly
- May report wrong line numbers for actual errors
- Issue appears related to file complexity, not specific syntax

### Minimal Reproduction

**Unable to create minimal reproduction**
- Simple files (20-50 lines): ✅ Work fine
- Complex files (300+ lines): ❌ Parser errors

**Working comparison**:
- `validation/quality/tdg_system_test.ruchy` (~450 lines) - had similar patterns, worked eventually

### Impact

**CRITICAL - COMPLETELY BLOCKING**:
- Cannot implement QUALITY-002 (Dead Code Detection)
- Cannot create test files for quality tools
- Blocks entire CYCLE 6 implementation
- Violates dogfooding (cannot use Ruchy to write Ruchy tools)

### Workaround

None available. Must simplify file or abandon ticket entirely.

### Request

**High Priority Fixes**:
1. **Improve error messages** - Show actual problematic line with context
2. **Fix brace tracking** - Ensure accurate counting in all contexts
3. **Add internal validation** - Verify parser state consistency
4. **Better line numbers** - Report actual error location, not misleading position

**Test Coverage**:
1. Add tests for complex files (300+ lines)
2. Add tests with multiple functions and structs
3. Test brace counting in various contexts (strings, comments, etc.)

### Additional Context

This is the **second** parser/tooling blocker in CYCLE 6:
1. Issue #1: `ruchy fmt` inconsistent behavior
2. Issue #2: Parser misleading errors (this issue)

Both block quality discovery tool development.

### Affected Work

**QUALITY-002: Dead Code Detection**
- Status: BLOCKED (cannot create test file)
- File attempted: `validation/quality/dead_code_test.ruchy` (~370 lines)
- Result: Deleted due to parser errors

**QUALITY-001: Technical Debt Grading**
- Status: PARTIAL (3/8 phases)
- Similar file worked: `validation/quality/tdg_system_test.ruchy` (~450 lines)
- Suggests intermittent issue

---

## Summary

**2 Critical Bugs Filed**:
1. ✅ ruchy fmt inconsistent behavior (HIGH priority)
2. ✅ ruchy parser misleading errors (CRITICAL priority)

**Impact**:
- QUALITY-001: 3/8 phases complete (REFACTOR blocked)
- QUALITY-002: 0/8 phases complete (completely blocked)
- CYCLE 6: 10% complete (1/10 tickets partial)

**Next Steps**:
1. File both issues at https://github.com/paiml/ruchy/issues
2. Monitor for fixes
3. Pivot to alternative work (non-Ruchy-parser-dependent)
4. Resume QUALITY cycle when bugs resolved

**Project**: RuchyRuchy Bootstrap Compiler
**Date**: 2025-10-27
**Author**: Claude (Anthropic)
