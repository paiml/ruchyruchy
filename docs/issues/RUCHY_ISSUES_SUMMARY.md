# Ruchy Issues Summary: October 20, 2025

## Overview

Two issues identified for the Ruchy team:
1. **Mutation Bug in Match Arms** (v3.99.2) - NEW, HIGH priority
2. **WASM Build Failing** - MEDIUM priority, has workarounds

---

## Issue 1: Mutation Bug in Match Arms (NEW)

### Summary

**Severity**: HIGH
**Version**: Ruchy v3.99.2
**Status**: NEW BUG (discovered while testing Issue #40 fix)

Mutable variable updates inside match statement arms **do not persist** after the match expression completes.

### Minimal Reproduction

```ruchy
fun main() {
    let mut i = 0;
    match Some(1) {
        Some(n) => {
            i = i + n;  // ❌ Doesn't persist
        },
        None => {}
    }
    println("{}", i);  // Prints 0, should print 1
}
```

### Impact

- Causes infinite loops in string iteration
- Breaks state machines
- Breaks parsers and loop counters
- Violates Rust semantics

### Workaround (WORKS)

```ruchy
let mut i = 0;
let found = match Some(1) {
    Some(n) => true,
    None => false
};
if found {
    i = i + 1;  // ✅ Works outside match
}
```

### Test Files

- `test_issue_40_minimal.ruchy` - Demonstrates bug
- `test_issue_40_simple_workaround.ruchy` - Demonstrates workaround (runs successfully!)
- `GITHUB_ISSUE_MUTATION_BUG.md` - Full GitHub issue text

### GitHub Issue

Ready to file at: https://github.com/paiml/ruchy/issues/new

Use content from: `GITHUB_ISSUE_MUTATION_BUG.md`

---

## Issue 2: WASM Build Failing

### Summary

**Severity**: MEDIUM (has workaround)
**Component**: WASM compilation
**Status**: Blocks WASM deployment

WASM build fails due to non-WASM code not being properly feature-gated.

### Error

```
error[E0282]: type annotations needed
  --> src/bench/http.rs:137
```

### Root Cause

Non-WASM-compatible code (HTTP, benchmarks) not behind `#[cfg(not(target_arch = "wasm32"))]` guards.

### Recommended Fix

Gate non-WASM code:

```rust
#[cfg(not(target_arch = "wasm32"))]
mod http {
    // HTTP code here
}
```

Update `Cargo.toml`:

```toml
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
reqwest = "0.x"
mio = "0.x"
```

### Workarounds

1. **Use published version** from crates.io (pre-built WASM)
2. **Use native binary** instead of WASM
3. **Build with feature flags**: `cargo build --no-default-features`

### Test Files

- `GITHUB_ISSUE_WASM_BUILD.md` - Full GitHub issue text

### GitHub Issue

Ready to file at: https://github.com/paiml/ruchy/issues/new

Use content from: `GITHUB_ISSUE_WASM_BUILD.md`

---

## Relationship Between Issues

### Issue #40 Timeline

| Version | Status |
|---------|--------|
| v3.99.1 | Infinite hang at `.chars().nth(i)` ❌ |
| v3.99.2 | Hang fixed ✅, but mutation bug introduced ❌ |
| Future | Need mutation bug fix |

### Progress Made

**Good News**:
- ✅ Issue #40 original hang is FIXED in v3.99.2
- ✅ Working workaround available for mutation bug
- ✅ WASM REPL fix is complete (commit 7cfd31dd)
- ✅ BOOTSTRAP-004 can proceed using workaround

**Remaining Work**:
- ❌ Fix mutation-in-match-arms bug
- ❌ Fix WASM compilation with feature gates

---

## Impact on RuchyRuchy Bootstrap Compiler

### Before v3.99.2

- ❌ **BOOTSTRAP-004 blocked** - String iteration hung infinitely
- ❌ Could not iterate through characters
- ❌ Error recovery impossible

### After v3.99.2 (Current)

- ✅ **BOOTSTRAP-004 unblocked** - Workaround available
- ✅ Can iterate through characters using workaround pattern
- ✅ Error recovery can proceed
- ⚠️ Must use workaround (mutate outside match)

### Implementation Strategy

Use workaround pattern in BOOTSTRAP-004:

```ruchy
loop {
    if i >= input.len() { break; }

    let ch_opt = input.chars().nth(i);
    let valid = match ch_opt {
        Some(c) => is_valid_char(c),
        None => false
    };

    // Mutate OUTSIDE match
    if valid {
        i = i + 1;
    } else {
        // Skip invalid character
        i = i + 1;
    }
}
```

---

## Files Created

### Mutation Bug Files

1. **`test_issue_40_minimal.ruchy`** (67 LOC)
   - Demonstrates infinite loop bug
   - Shows mutation doesn't persist

2. **`test_issue_40_simple_workaround.ruchy`** (37 LOC)
   - Demonstrates working workaround
   - ✅ Runs successfully, outputs "Counted 3 characters"

3. **`GITHUB_ISSUE_MUTATION_BUG.md`** (Comprehensive)
   - Full GitHub issue text
   - Minimal reproduction
   - Root cause analysis
   - Workarounds
   - Expected vs actual behavior

4. **`ISSUE_40_FINAL_REPORT.md`** (Comprehensive)
   - Complete analysis
   - Version history
   - Test results
   - Recommendations

### WASM Build Files

5. **`GITHUB_ISSUE_WASM_BUILD.md`** (Comprehensive)
   - Full GitHub issue text
   - Build errors
   - Root cause analysis
   - Recommended fixes
   - Workarounds

### Summary Files

6. **`RUCHY_ISSUES_SUMMARY.md`** (This file)
   - Executive summary
   - Both issues documented
   - Impact analysis
   - Next steps

---

## Next Steps

### For Ruchy Team

1. **File Mutation Bug Issue**
   - Use content from `GITHUB_ISSUE_MUTATION_BUG.md`
   - Attach test files
   - Priority: HIGH

2. **File WASM Build Issue**
   - Use content from `GITHUB_ISSUE_WASM_BUILD.md`
   - Reference commit 7cfd31dd
   - Priority: MEDIUM

3. **Fixes Needed**
   - Fix mutable variable capture in match arms
   - Add feature gates for non-WASM code
   - Add tests for both scenarios

### For RuchyRuchy Project

1. **Proceed with BOOTSTRAP-004**
   - Use mutation workaround pattern
   - Implement error recovery
   - Document workaround usage

2. **Update Documentation**
   - Document mutation bug and workaround
   - Update BOUNDARIES.md
   - Note Issue #40 partial fix

3. **Test Coverage**
   - Ensure tests use workaround pattern
   - Add regression tests for when fix arrives

---

## Success Metrics

### Mutation Bug

**FIXED when**:
```ruchy
let mut i = 0;
match Some(1) {
    Some(n) => { i = i + n; }
    None => {}
}
assert!(i == 1);  // Currently fails, should pass
```

### WASM Build

**FIXED when**:
```bash
$ wasm-pack build --target web
   Compiling ruchy v3.x.x
   Finished release [optimized] target(s)
✅ Successfully built WASM package
```

---

## Acknowledgments

### Ruchy Team's Quick Response

- ✅ v3.99.2 fixed the original hang (Issue #40)
- ⚠️ Introduced mutation bug (unintended side effect)
- ✅ WASM REPL fix complete (7cfd31dd)

### Progress Despite Issues

- BOOTSTRAP-004 can proceed (workaround available)
- No work is blocked (all have workarounds)
- Clear path forward for fixes

---

**Report Date**: October 20, 2025
**Ruchy Version Tested**: v3.99.2
**Project**: RuchyRuchy Bootstrap Compiler
**Status**: Issues documented, workarounds available, work can proceed

---

## Quick Reference

### Report Mutation Bug

1. Go to: https://github.com/paiml/ruchy/issues/new
2. Copy content from: `GITHUB_ISSUE_MUTATION_BUG.md`
3. Attach: `test_issue_40_minimal.ruchy`, `test_issue_40_simple_workaround.ruchy`
4. Title: "Mutable variables don't update inside match arms (v3.99.2)"
5. Labels: bug, high-priority, language-semantics

### Report WASM Build

1. Go to: https://github.com/paiml/ruchy/issues/new
2. Copy content from: `GITHUB_ISSUE_WASM_BUILD.md`
3. Reference: Commit 7cfd31dd
4. Title: "WASM build failing: feature gate issues"
5. Labels: bug, wasm, build, medium-priority

### Use Workaround Pattern

```ruchy
// DON'T: Mutate inside match
match value {
    Pattern => { x = x + 1; }  // ❌ Broken in v3.99.2
}

// DO: Mutate outside match
let flag = match value {
    Pattern => true,
    _ => false
};
if flag {
    x = x + 1;  // ✅ Works in v3.99.2
}
```

---

*Generated with [Claude Code](https://claude.com/claude-code)*
*Co-Authored-By: Claude <noreply@anthropic.com>*
