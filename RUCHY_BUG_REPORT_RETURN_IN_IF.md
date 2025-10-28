# CRITICAL BUG: return statements inside if blocks do not actually return

## Bug Report for Ruchy

**Severity**: 🔴 CRITICAL - Breaks basic control flow
**Ruchy Version**: v3.139.0
**Project**: RuchyRuchy Bootstrap Compiler
**Ticket**: QUALITY-004 (Duplicate Code Detection)
**Discovered**: 2025-10-28

---

## Summary

`return` statements inside `if` blocks do not terminate function execution. The function continues executing code after the if block, ignoring the return statement.

---

## Minimal Reproduction

```ruchy
fun test_boolean_if() -> f64 {
    let code1 = "factorial"
    let code2 = "fac"

    let check1 = code1.contains("factorial") || code1.contains("fac")
    let check2 = code2.contains("factorial") || code2.contains("fac")

    println("check1 = " + check1.to_string())
    println("check2 = " + check2.to_string())
    println("check1 && check2 = " + (check1 && check2).to_string())

    if check1 && check2 {
        println("Inside if block - about to return 0.95")
        return 0.95
    }

    println("Outside if block - returning 0.5")
    return 0.5
}
```

**Full reproduction file**: `validation/quality/bug_minimal_reproduction.ruchy`

---

## Expected Behavior

When the if condition is true, the function should:
1. Enter the if block
2. Execute `return 0.95`
3. **Terminate function execution**
4. Return 0.95 to caller

---

## Actual Behavior

```
check1 = true
check2 = true
check1 && check2 = true
Inside if block - about to return 0.95  ← Enters if block
Outside if block - returning 0.5        ← ❌ Should NOT execute!
Actual: 0.5                             ← ❌ Wrong return value!
```

The function:
1. ✅ Enters the if block (condition is true)
2. ✅ Prints "Inside if block - about to return 0.95"
3. ❌ **Ignores `return 0.95`**
4. ❌ **Continues executing code after if block**
5. ❌ Returns 0.5 instead of 0.95

---

## Impact

**HIGH - Breaks fundamental control flow**

This bug affects:
- ✅ All functions with early returns in if blocks
- ✅ Pattern matching and classification functions
- ✅ Guard clause patterns
- ✅ Error handling with early returns

Example from QUALITY-004:
```ruchy
fun detect_clone_type(code1: String, code2: String) -> String {
    // Type I: Exact match
    if code1 == code2 {
        return "type_1"  // ❌ Does not return!
    }

    // Type II: Renamed
    if some_condition {
        return "type_2"  // ❌ Does not return!
    }

    return "unknown"  // ❌ Always executes!
}
```

---

## Environment

- **OS**: Linux 6.8.0-85-generic
- **Ruchy Version**: `v3.139.0` (output of `ruchy --version`)
- **Project**: RuchyRuchy Bootstrap Compiler
- **Context**: Implementing QUALITY-004 (Duplicate Code Detection)

---

## Steps to Reproduce

1. Create file `bug_minimal_reproduction.ruchy` with code above
2. Run: `ruchy run bug_minimal_reproduction.ruchy`
3. Observe output shows both "Inside if block" and "Outside if block"
4. Observe return value is 0.5 instead of 0.95

---

## Additional Context

### Discovery Process

While implementing QUALITY-004 (Duplicate Code Detection), pattern matching functions consistently returned wrong values. Initially suspected:
- String operations behaving incorrectly
- Boolean evaluation issues
- Scoping problems

Systematic debugging revealed the root cause: `return` statements inside if blocks are ignored.

### Workaround

**None effective**. Cannot use guard clauses or early returns in if blocks.

Attempted workarounds:
1. ❌ Using nested if statements (same bug)
2. ❌ Storing result in variable and returning at end (changes logic)
3. ❌ Using match expressions (not applicable to all cases)

### Related Files

- **Minimal reproduction**: `validation/quality/bug_minimal_reproduction.ruchy`
- **Comprehensive reproduction**: `validation/quality/bug_reproduction_string_contains.ruchy`
- **Original context**: `validation/quality/duplicate_code_test.ruchy`

---

## Expected Fix

`return` statements inside if blocks should:
1. Immediately terminate function execution
2. Return the specified value to the caller
3. Not execute any code after the if block

This is standard behavior in all mainstream languages (Rust, JavaScript, Python, Go, etc.).

---

## Request

Please fix this critical bug as it blocks development of:
- QUALITY-004: Duplicate Code Detection
- Any code using guard clauses
- Any code with early returns based on conditions

**Priority**: 🔴 CRITICAL - Blocks fundamental programming patterns

---

**Filed by**: Claude Code (AI Assistant)
**On behalf of**: Noah (RuchyRuchy Project)
**Date**: 2025-10-28
**GitHub Repository**: https://github.com/paiml/ruchy

---

## Additional Test Cases

### Test Case 1: Simple early return
```ruchy
fun test1(x: i32) -> i32 {
    if x > 10 {
        return 100
    }
    return 0
}

// Expected: test1(15) == 100
// Actual: test1(15) == 0  ❌
```

### Test Case 2: Nested if with return
```ruchy
fun test2(x: i32, y: i32) -> i32 {
    if x > 0 {
        if y > 0 {
            return 50
        }
        return 25
    }
    return 0
}

// Expected: test2(5, 5) == 50
// Actual: test2(5, 5) == 0  ❌
```

### Test Case 3: Multiple returns in chain
```ruchy
fun test3(s: String) -> i32 {
    if s.contains("error") {
        return -1
    }
    if s.contains("warning") {
        return 0
    }
    if s.contains("info") {
        return 1
    }
    return 2
}

// Expected: test3("error here") == -1
// Actual: test3("error here") == 2  ❌
```

---

**This bug must be fixed for Ruchy to be production-ready.**
