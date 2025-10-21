# New Issue: Mutable Variable Updates Don't Persist in Match Arms (v3.99.2)

## Issue Summary

**Discovered in**: Ruchy v3.99.2 (while testing fix for Issue #40)
**Severity**: HIGH
**Impact**: Infinite loops in code that should work correctly

## Problem Description

Mutable variable updates inside match statement arms **do not persist** after the match expression completes. This causes infinite loops in patterns that should work correctly.

## Minimal Reproduction

```ruchy
fun main() {
    let mut i = 0;

    match Some(1) {
        Some(n) => {
            println("Before: i = {}", i);
            i = i + n;
            println("After: i = {}", i);
        },
        None => {}
    }

    println("Outside match: i = {}", i);
}
```

**Expected Output**:
```
Before: i = 0
After: i = 1
Outside match: i = 1
```

**Actual Output** (v3.99.2):
```
Before: i = 0
After: i = 1
Outside match: i = 0    ← WRONG! Mutation didn't persist
```

## Real-World Impact: String Iteration

This causes infinite loops in string iteration code:

```ruchy
fun main() {
    let s = "abc".to_string();
    let mut i = 0;

    loop {
        if i >= s.len() { break; }

        let ch_opt = s.chars().nth(i);
        match ch_opt {
            Some(c) => {
                println("Character {}: {}", i, c);
                i = i + 1;  // ❌ DOESN'T PERSIST
            },
            None => break
        }
    }

    println("Done");
}
```

**Result**: Infinite loop printing "Character 0: a" forever because `i` never increments.

## Workaround (WORKS)

Move mutations outside the match statement:

```ruchy
fun main() {
    let s = "abc".to_string();
    let mut i = 0;

    loop {
        if i >= s.len() { break; }

        let ch_opt = s.chars().nth(i);
        let found = match ch_opt {
            Some(_c) => true,
            None => false
        };

        // ✅ Mutate OUTSIDE match
        if found {
            i = i + 1;  // This works!
        } else {
            break;
        }
    }
}
```

**Result**: ✅ Works correctly, completes successfully.

## Root Cause Hypothesis

The mutation inside the match arm appears to:
1. Happen on a **copy** of the variable, not the original
2. Or create a **new scope** where mutations don't escape
3. Or have **closure capture semantics** that capture by value

### Evidence

Multiple patterns demonstrate this:

**Pattern 1**: Direct mutation
```ruchy
let mut x = 0;
match value {
    Pattern => { x = x + 1; }  // ❌ Doesn't persist
}
```

**Pattern 2**: Multiple variables
```ruchy
let mut i = 0;
let mut count = 0;
match value {
    Pattern => {
        i = i + 1;        // ❌ Neither persists
        count = count + 1;  // ❌
    }
}
```

**Pattern 3**: Complex expressions
```ruchy
let mut state = 0;
match value {
    Pattern => {
        state = complex_computation(state);  // ❌ Doesn't persist
    }
}
```

All fail the same way: mutations inside match arms don't escape.

## Comparison with Working Code

This **works** (mutation outside match):
```ruchy
let mut x = 0;
let should_update = match value {
    Pattern => true,
    _ => false
};
if should_update {
    x = x + 1;  // ✅ Works
}
```

This **fails** (mutation inside match):
```ruchy
let mut x = 0;
match value {
    Pattern => {
        x = x + 1;  // ❌ Fails
    }
}
```

## Expected Behavior

Mutable variables should be **borrowed mutably** by match arms, allowing updates to persist:

```ruchy
let mut x = 0;
match Some(1) {
    Some(n) => {
        x = x + n;  // Should mutate the original x
    },
    None => {}
}
assert!(x == 1);  // Should pass
```

This is standard behavior in Rust:
```rust
let mut x = 0;
match Some(1) {
    Some(n) => { x += n; }  // Works in Rust
    None => {}
}
assert_eq!(x, 1);  // Passes in Rust
```

## Test Files

1. **`test_issue_40_minimal.ruchy`** - Demonstrates infinite loop bug
2. **`test_issue_40_simple_workaround.ruchy`** - Demonstrates working workaround
3. **Reproduction script**: See minimal example above

## Environment

- **Ruchy Version**: v3.99.2
- **Platform**: Linux
- **Context**: Discovered while testing Issue #40 fix

## Impact

**HIGH**: This breaks many common Rust patterns:
- Loop counters in match statements
- State machines with match transitions
- Parsers using match for token processing
- Any code that updates state based on matched patterns

## Recommended Fix

Ensure match arm closures/blocks:
1. Capture mutable variables **by mutable reference**
2. Allow mutations to **persist** after match completes
3. Behave consistently with **Rust semantics**

## Related Issues

- **Issue #40**: String iteration hang (partially fixed in v3.99.2)
- This bug was discovered while testing the Issue #40 fix

## Priority

**HIGH** - This breaks fundamental language semantics and causes difficult-to-debug infinite loops.

---

**Reporter**: RuchyRuchy Bootstrap Compiler Project
**Date**: October 20, 2025
**Version**: Ruchy v3.99.2
