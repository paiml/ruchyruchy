# Issue #40 Update for v3.99.2: Partial Fix - New Bug Discovered

## Status: PARTIALLY FIXED (v3.99.2) - NEW BUG FOUND

Ruchy v3.99.2 partially fixes Issue #40, but introduces a new bug: **mutable variables are not being incremented correctly inside match statements within loops**.

### Previous Behavior (v3.99.1)
- Program **hung indefinitely** at `.chars().nth(i)`
- Never produced any output
- Required force-kill to terminate

### New Behavior (v3.99.2)
- Program **no longer hangs** ✅
- But enters **infinite loop** printing same character ❌
- Mutable variable `i` is **not incrementing** inside match statement

### Minimal Reproduction

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
                i = i + 1;  // ❌ THIS INCREMENT IS NOT HAPPENING
            },
            None => break
        }
    }

    println("Done");
}
```

### Expected Output
```
Testing string iteration...
Character 0: a
Character 1: b
Character 2: c
✅ SUCCESS: Completed without hanging!
```

### Actual Output (v3.99.2)
```
Testing string iteration...
Character 0: a
Character 0: a
Character 0: a
Character 0: a
Character 0: a
[repeats infinitely...]
```

The variable `i` **never increments** from 0 to 1, causing the loop to process character 0 forever.

### Root Cause Hypothesis

The mutation `i = i + 1` inside the `match` arm is either:
1. Not being executed
2. Not persisting after the match expression
3. Being reset each loop iteration

This appears to be a **variable capture/mutation bug** in match expressions within loops.

### Comparison: Working Pattern

This similar pattern **works correctly**:

```ruchy
fun main() {
    let mut i = 0;
    loop {
        if i >= 3 { break; }
        println("Count: {}", i);
        i = i + 1;  // This works fine without match
    }
    println("Done");
}
```

Output (correct):
```
Count: 0
Count: 1
Count: 2
Done
```

The difference: **mutation inside match statement** vs **mutation in loop body**.

### Impact

This means the "fix" in v3.99.2:
- ✅ Resolved the hang issue
- ❌ Introduced a mutation bug in match expressions
- ❌ Still blocks BOOTSTRAP-004 (Error Recovery)

### Test Files

1. `test_issue_40_minimal.ruchy` - Demonstrates the infinite loop
2. `test_issue_40_string_iteration.ruchy` - Comprehensive test suite (all fail with infinite loops)

### Recommended Fix

The fix should ensure that:
1. `.chars().nth(i)` doesn't hang ✅ (this part works in v3.99.2)
2. Mutable variables update correctly inside match arms ❌ (broken in v3.99.2)
3. Loop can proceed to next iteration with updated value ❌ (broken in v3.99.2)

### Workaround

For now, **avoid mutations inside match statements within loops**. This pattern should work:

```ruchy
loop {
    if i >= s.len() { break; }

    let ch_opt = s.chars().nth(i);
    let should_continue = match ch_opt {
        Some(c) => {
            println("Character: {}", c);
            true
        },
        None => false
    };

    if should_continue {
        i = i + 1;  // Mutate OUTSIDE match statement
    } else {
        break;
    }
}
```

### Priority

**HIGH** - The string iteration functionality is still broken, just in a different way. Error recovery implementation remains blocked.

---

**Tested with**: Ruchy v3.99.2
**Date**: October 20, 2025
**Project**: RuchyRuchy Bootstrap Compiler
**Ticket**: BOOTSTRAP-004 (Error Recovery)
**Severity**: CRITICAL - Infinite loop instead of hang
