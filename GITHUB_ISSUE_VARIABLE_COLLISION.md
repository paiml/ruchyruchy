# Bug Report: Variable Corruption After Tuple Unpacking in Nested Function Calls

## Summary

**Title**: Variable name collision in nested function calls with tuple unpacking causes type corruption

**Labels**: `bug`, `runtime`, `type-safety`, `high-severity`

**Priority**: HIGH

**Affects**: Ruchy v3.96.0 (confirmed via `ruchy --version`)

## Description

When unpacking tuples returned from functions with nested calls, variable names in outer scope can collide with variable names in deeper call stack frames, causing **runtime type corruption**. A variable that should be a `String` can be replaced with an `i32` value from a completely different function scope.

This is a critical type safety violation that causes runtime errors and makes complex nested function calls unreliable.

## Environment

- **Ruchy Version**: v3.96.0
- **OS**: Linux (6.8.0-85-generic)
- **Install Method**: Cargo-installed binary
- **Project**: RuchyRuchy Bootstrap Compiler (https://github.com/paiml/ruchyruchy)
- **Context**: VALID-003-EXTENDED property testing implementation

## Minimal Reproduction

Save as `bug_variable_collision.ruchy`:

```ruchy
// Minimal reproduction of variable collision bug
// Expected: Variable 'a' in main() should be a String
// Actual: Variable 'a' corrupted to integer from nested function

fun next_random(seed: i32) -> i32 {
    let a = 1103515245;  // ⚠️ Local variable 'a'
    let c = 12345;
    let m = 2147483647;

    let temp = a * seed + c;
    if temp < 0 {
        (temp + m) % m
    } else {
        temp % m
    }
}

fun random_in_range(seed: i32, max: i32) -> (i32, i32) {
    let new_seed = next_random(seed);
    let value = if max > 0 {
        if new_seed < 0 {
            ((new_seed + 2147483647) % max)
        } else {
            new_seed % max
        }
    } else {
        0
    };
    (value, new_seed)
}

fun random_string(seed: i32, max_len: i32) -> (String, i32) {
    let result = random_in_range(seed, 100);
    let num = result.0;
    let new_seed = result.1;

    if num < 10 {
        ("x".to_string(), new_seed)
    } else if num < 20 {
        ("xy".to_string(), new_seed)
    } else {
        ("hello".to_string(), new_seed)
    }
}

fun main() {
    let r1 = random_string(42, 5);
    let a = r1.0;  // ⚠️ Should be String, but will be corrupted
    let seed1 = r1.1;

    let r2 = random_string(seed1, 5);
    let b = r2.0;

    println("a = {}", a);  // 🐛 Shows: 1103515245 (integer!) instead of String
    println("b = {}", b);  // ✅ Shows: "hello" correctly

    let result = a + b;  // ❌ ERROR: Cannot add integer and string
    println("result = {}", result);
}

main();
```

### Steps to Reproduce

1. Save the above code as `bug_variable_collision.ruchy`
2. Run: `ruchy run bug_variable_collision.ruchy`
3. Observe the error

### Expected Output

```
a = "hello"
b = "hello"
result = "hellohello"
```

### Actual Output

```
a = 1103515245
b = "hello"
Error: Evaluation error: Type error: Cannot add integer and string
```

## Root Cause Analysis

The variable `a` in `main()` is being corrupted with the value of the local variable `a` from the `next_random()` function (the constant `1103515245`).

**Call Stack When Bug Occurs**:
```
main()
  └─ random_string(42, 5)
      └─ random_in_range(42, 100)
          └─ next_random(42)
              └─ let a = 1103515245;  // This value corrupts outer 'a'
```

The runtime appears to be using variable names for lookups across scope boundaries, causing the constant from the deepest function to overwrite the tuple element extracted in the outermost function.

## Impact

**Severity**: HIGH - Type safety violation

**Blocks**:
- ✅ VALID-003-EXTENDED property testing with random generation
- ✅ Any complex tuple-returning functions with nested calls
- ✅ Functions that use common variable names (a, b, c, i, x, y, etc.)

**Affects**:
- Variable scoping semantics
- Type system guarantees
- Runtime safety
- Compiler reliability

**Risk**:
- Silent corruption in some cases (if types happen to match)
- Difficult to debug (requires understanding entire call stack)
- Breaks fundamental expectations about variable scoping

## Workaround

**Rename variables to avoid name collisions across the entire call stack:**

```ruchy
fun next_random(seed: i32) -> i32 {
    let multiplier = 1103515245;  // ✅ Renamed from 'a'
    let increment = 12345;         // ✅ Renamed from 'c'
    let modulus = 2147483647;      // ✅ Renamed from 'm'

    let temp = multiplier * seed + increment;
    if temp < 0 {
        (temp + modulus) % modulus
    } else {
        temp % modulus
    }
}

// ... rest of code unchanged
```

**Validation**: This workaround has been tested with 5000+ test cases and eliminates the corruption completely.

## Additional Context

This bug was discovered during implementation of enhanced property testing for the RuchyRuchy bootstrap compiler. We were testing string concatenation properties with a Linear Congruential Generator (LCG) for random test case generation.

The LCG implementation uses standard constants `a`, `c`, `m` which are common in numerical code. When combined with tuple unpacking in outer scopes that also use common variable names like `a`, the collision occurs.

**Project Context**:
- Repository: https://github.com/paiml/ruchyruchy
- File: `validation/property/property_framework_extended.ruchy`
- Documentation: `BOUNDARIES.md` (complete analysis)
- Book: https://paiml.github.io/ruchyruchy/phase2_validation/tickets/valid-003-extended-enhanced-testing.html

## Suggested Fix

The Ruchy runtime should:

1. **Use fully qualified variable names** including scope information (function name, depth, etc.)
2. **Implement proper lexical scoping** where inner function variables cannot affect outer scope
3. **Add scope analysis** to detect and prevent this class of bugs at compile time
4. **Add regression tests** for nested function scope with tuple unpacking

## Testing

Once fixed, this test should pass:

```ruchy
// Regression test for variable collision bug
fun inner() -> (String, i32) {
    let a = 1103515245;  // Should NOT affect outer scope
    ("test".to_string(), a)
}

fun main() {
    let result = inner();
    let a = result.0;  // Should be String "test"
    let num = result.1;  // Should be i32 1103515245

    assert(a == "test", "Variable 'a' should be String");
    assert(num == 1103515245, "Variable 'num' should be i32");
}
```

## References

- **Discovery**: 2025-10-19 during VALID-003-EXTENDED implementation
- **Documentation**: RuchyRuchy BOUNDARIES.md (lines 21-120)
- **Workaround**: Validated with 5000+ property test cases
- **Book Chapter**: https://paiml.github.io/ruchyruchy/phase2_validation/tickets/valid-003-extended-enhanced-testing.html

---

**Thank you for Ruchy!** This bug discovery is part of comprehensive dogfooding efforts for the RuchyRuchy bootstrap compiler project. The systematic testing approach helped uncover this critical issue, and the workaround allows us to continue while awaiting a fix.
