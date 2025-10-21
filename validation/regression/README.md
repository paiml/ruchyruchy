# Regression Test Suite

This directory contains regression tests for previously discovered and fixed bugs in the Ruchy language.

## Purpose

Regression tests ensure that bugs, once fixed, stay fixed. These tests:
- Validate that fixes remain in place across Ruchy versions
- Provide historical context for language evolution
- Serve as examples of edge cases that were problematic
- Prevent regressions when new features are added

## Test Files

### Issue #40: String Iteration Hang

**Status**: ✅ Fixed in Ruchy v3.100.0

#### `test_issue_40_string_iteration.ruchy`

Comprehensive test suite for string iteration using `.chars().nth(i)` pattern.

**Test Coverage**:
1. Simple string iteration (3 characters)
2. Longer string iteration (11 characters)
3. Empty string iteration
4. Single character string

**Test Results** (v3.100.0):
```bash
$ ruchy run validation/regression/test_issue_40_string_iteration.ruchy

Test 1: Simple string iteration (3 characters)
  ✅ PASS: Counted 3 characters correctly
Test 2: Longer string iteration (11 characters)
  ✅ PASS: Counted 11 characters correctly
Test 3: Empty string iteration
  ✅ PASS: Empty string handled correctly
Test 4: Single character string
  ✅ PASS: Single character handled correctly

📊 Results: 4 passed, 0 failed
✅ SUCCESS: All tests passed!
```

**Pattern Tested**:
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
                i = i + 1;  // ✅ Now works correctly
            },
            None => break
        }
    }
}
```

**Historical Issues**:
- v3.99.1: Infinite hang at `.chars().nth(i)`
- v3.99.2: Mutation bug (i not incrementing in match arm)
- v3.100.0: Both issues completely resolved ✅

#### `test_issue_40_minimal.ruchy`

Minimal reproduction test for quick validation.

**Test Results** (v3.100.0):
```bash
$ ruchy run validation/regression/test_issue_40_minimal.ruchy

Testing string iteration...
Character 0: a
Character 1: b
Character 2: c
✅ SUCCESS: Completed without hanging!
```

## Running Regression Tests

### Run All Regression Tests
```bash
# From project root
ruchy run validation/regression/test_issue_40_string_iteration.ruchy
ruchy run validation/regression/test_issue_40_minimal.ruchy
```

### Expected Results

All tests should pass with current Ruchy version (v3.100.0+):
- ✅ 4/4 tests passing in comprehensive suite
- ✅ Minimal test completes without hanging
- ✅ No mutation bugs
- ✅ Correct character iteration

### If Tests Fail

If regression tests fail on a newer Ruchy version:

1. 🚨 **STOP THE LINE** - Regression detected!
2. 📋 **FILE GITHUB ISSUE** - Report regression immediately
3. 🔍 **BISECT VERSION** - Identify when regression was introduced
4. 📝 **UPDATE BOUNDARIES.md** - Document regression
5. 🔄 **DOWNGRADE TEMPORARILY** - Use last known good version
6. ✅ **VALIDATE FIX** - Test when regression is resolved

## Adding New Regression Tests

When a new bug is discovered and fixed:

1. Create test file: `test_issue_XX_description.ruchy`
2. Include comprehensive test cases
3. Document the issue and fix version
4. Update this README with new test information
5. Add to regression test suite

## Test Maintenance

- Run regression tests before each sprint
- Run after Ruchy version upgrades
- Keep tests passing at 100%
- Archive obsolete tests only after careful consideration

---

**Last Updated**: October 21, 2025
**Ruchy Version**: v3.100.0
**Test Suite**: 2 files, 5 tests total
**Success Rate**: 100% (5/5 passing)
