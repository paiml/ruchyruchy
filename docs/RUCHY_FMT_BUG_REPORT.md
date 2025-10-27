# Bug Report: ruchy fmt breaks struct syntax

**Date**: 2025-10-27
**Project**: RuchyRuchy Bootstrap Compiler
**Ticket**: QUALITY-001 (blocked by this bug)
**Ruchy Version**: (run `ruchy --version` to determine)

---

## Summary

`ruchy fmt` reformats multi-line struct definitions to single-line format, but the reformatted code then fails `ruchy check` with "Unexpected token: RightBrace" syntax error.

## Reproduction Steps

1. Create a valid Ruchy file with multi-line struct definitions:
```ruchy
// test_struct_format.ruchy
struct TdgComponentBreakdown {
    complexity: f64,
    maintainability: f64,
    security: f64,
    performance: f64,
    test_coverage: f64,
}

fun main() {
    let breakdown = TdgComponentBreakdown {
        complexity: 0.9,
        maintainability: 0.85,
        security: 0.95,
        performance: 0.88,
        test_coverage: 0.75,
    }
    println("Test")
}
```

2. Run `ruchy check test_struct_format.ruchy` - should pass ✅
3. Run `ruchy fmt test_struct_format.ruchy` - formats file
4. Run `ruchy check test_struct_format.ruchy` - NOW FAILS ❌

## Expected Behavior

`ruchy fmt` should either:
1. Preserve multi-line struct format (preferred)
2. OR format to single-line in a way that still passes `ruchy check`

The formatted output should ALWAYS pass `ruchy check`.

## Actual Behavior

After `ruchy fmt`, the file contains:
```ruchy
struct TdgComponentBreakdown { complexity: f64, maintainability: f64, security: f64, performance: f64, test_coverage: f64 }
```

Then `ruchy check` fails with:
```
✗ test_struct_format.ruchy:1: Syntax error: Unexpected token: RightBrace
Error: test_struct_format.ruchy:1: Syntax error: Unexpected token: RightBrace
```

## Full Error Output

```
$ ruchy fmt test_struct_format.ruchy
✓ Formatted test_struct_format.ruchy

$ ruchy check test_struct_format.ruchy
✗ test_struct_format.ruchy:1: Syntax error: Unexpected token: RightBrace
Error: test_struct_format.ruchy:1: Syntax error: Unexpected token: RightBrace
```

## Context

Working on QUALITY-001: Technical Debt Grading (TDG) System implementation. Attempting to complete REFACTOR phase of EXTREME TDD workflow (RED-GREEN-REFACTOR-TOOL-MUTATION-PROPERTY-FUZZ-PMAT).

The REFACTOR phase requires running `ruchy fmt` on all code, but this bug blocks progress.

## Impact

**BLOCKING**: Cannot complete QUALITY-001 REFACTOR phase without working formatter.

Impacts:
- QUALITY-001 implementation blocked at REFACTOR phase
- Cannot ensure code follows canonical Ruchy style
- Quality gates cannot pass (ruchy fmt is part of quality validation)
- Breaks dogfooding principle (Ruchy tools should work on Ruchy code)

## Workaround

**Current workaround**: Skip `ruchy fmt` and manually format code

**Issues with workaround**:
- Manual formatting is error-prone
- Violates "ruchy fmt must pass" quality gate
- Team members may have inconsistent formatting
- Pre-commit hooks that require `ruchy fmt` will fail

## Environment

- **OS**: Linux (Ubuntu/Debian - run `uname -a` to verify)
- **Ruchy install**: Cargo (run `which ruchy` to verify)
- **Project**: RuchyRuchy Bootstrap Compiler

## Request

Please fix `ruchy fmt` so that:
1. Formatted output ALWAYS passes `ruchy check`
2. Struct definitions either stay multi-line OR are formatted to valid single-line syntax
3. Add regression test: `ruchy fmt` output must pass `ruchy check`

## Related Issues

- May be related to parser's handling of struct syntax
- Possible interaction between formatter and parser on closing braces
- Could be whitespace-sensitive parsing issue

## Next Steps

1. File this as GitHub issue at https://github.com/paiml/ruchy/issues
2. Proceed with QUALITY-001 using manual formatting
3. Add test case to ruchy test suite for this scenario
4. Document in BOUNDARIES.md

---

**Status**: 🔴 BLOCKING - Filed GitHub issue #XXX (to be created)
