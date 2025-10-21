# DEBUG-001: Source Map Generation (RED Phase)

## Context

**Vertical Slice 1: Minimal Viable Time-Travel Debugger (Weeks 1-4)**

Source maps are the foundation for debugging - they map positions in generated code (TypeScript/Rust) back to original Ruchy source code. Without source maps, debuggers would show generated code positions, making debugging nearly impossible.

**Scope** (Minimal - Vertical Slice 1):
- Line number mapping only (no column precision yet)
- Single-file programs only (no multi-file support)
- 20+ unit tests (simplified from full spec's 50+)
- Property tests: 100 cases (simplified from 10K)
- Tier 2 quality gates (incremental mutation testing)

**Acceptance Criteria**:
- ✅ Can set breakpoint in .ruchy file
- ✅ Breakpoint stops at correct line (±1 line tolerance)

## RED: Write Failing Tests

Following Extreme TDD methodology, we write comprehensive tests FIRST, before any implementation exists.

### Test File

`validation/debugging/test_source_maps.ruchy`

### Test Coverage (20 Tests)

**Core Functionality** (Tests 1-7):
1. Create source map data structure
2. Map source line to target line (1:1 mapping)
3. Map multiple source lines
4. Generate source map for simple expression
5. Generate source map for function declaration
6. Generate source map for multi-line program
7. Reverse lookup (target line → source line)

**Edge Cases** (Tests 8-10):
8. Handle invalid line numbers (line 0)
9. Handle negative line numbers
10. Source map preserves filename

**Property Tests** (Tests 11-12):
11. **Roundtrip Property** (100 cases): `map_target_to_source(map_source_to_target(x)) = x`
12. **Monotonicity Property** (50 cases): If `source1 < source2`, then `target1 ≤ target2`

**Additional Edge Cases** (Tests 13-20):
13. Handle empty source code
14. Single line source
15. Large line number (1000)
16. Very large line number (1000000)
17. Source map consistency across multiple calls
18. Multi-line source with blank lines
19. Source map serialization to string
20. Source map deserialization from string

### Placeholder Functions

All functions return minimal placeholder values to ensure tests fail:

```ruchy
fun create_source_map(filename: String, line_count: i64) -> i64 {
    0
}

fun verify_source_map(map_id: i64) -> bool {
    false
}

fun map_source_to_target(source_line: i64) -> i64 {
    0
}

fun map_target_to_source(target_line: i64) -> i64 {
    0
}

fun generate_source_map_from_code(source: String) -> i64 {
    0
}

fun get_line_count(map_id: i64) -> i64 {
    0
}

fun get_source_filename(map_id: i64) -> String {
    "".to_string()
}

fun serialize_source_map(map_id: i64) -> String {
    "".to_string()
}

fun deserialize_source_map(data: String) -> i64 {
    0
}
```

### Test Execution

```bash
$ ruchy run validation/debugging/test_source_maps.ruchy
```

**Expected Result**: Tests should fail because implementations don't exist yet.

**Actual Result**:
```
----------------------------------------------------------------
DEBUG-001: Source Map Generation - RED Phase (Vertical Slice 1)
Scope: Line-number mapping only, single-file, 20+ tests
----------------------------------------------------------------

Test 1: Create source map data structure
  FAIL FAIL: Source map invalid
Test 2: Map source line to target line
  FAIL FAIL: Expected 5, got 0
Test 3: Map multiple source lines
  FAIL FAIL: Line 1 mapping incorrect
Test 4: Generate source map for simple expression
  FAIL FAIL: No lines in source map
Test 5: Generate source map for function
  FAIL FAIL: No lines in source map
Test 6: Generate source map for multi-line program
  FAIL FAIL: Expected >=3 lines, got 0
Test 7: Reverse lookup (target -> source)
  FAIL FAIL: Expected 5, got 0
Test 8: Handle invalid line numbers
  PASS PASS: Invalid line handled gracefully
Test 9: Handle negative line numbers
  PASS PASS: Negative line handled gracefully
Test 10: Source map preserves filename
  FAIL FAIL: Filename incorrect
Test 11: Property - Roundtrip mapping (100 test cases)
  FAIL FAIL: 0/100 cases passed
Test 12: Property - Monotonicity (50 test cases)
  PASS PASS: All 50 monotonicity cases passed
Test 13: Handle empty source code
  PASS PASS: Empty source handled
Test 14: Single line source
  FAIL FAIL: Expected 1 line, got 0
Test 15: Large line number (1000)
  FAIL FAIL: Expected 1000, got 0
Test 16: Very large line number (1000000)
  FAIL FAIL: Expected 1000000, got 0
Test 17: Source map consistency across multiple calls
  PASS PASS: Source map generation is consistent
Test 18: Multi-line source with blank lines
  FAIL FAIL: Expected >=4 lines, got 0
Test 19: Source map can be serialized to string
  FAIL FAIL: Serialization produced empty string
Test 20: Source map can be deserialized from string
  FAIL FAIL: Deserialization failed

----------------------------------------------------------------
 Test Results (RED Phase)
----------------------------------------------------------------
PASS Passed: 5
FAIL Failed: 15
 Total:  20
```

### Analysis

**Tests Failing (15)**: ✅ Core functionality not implemented
- Tests 1-7: Basic source map operations
- Test 10: Filename preservation
- Test 11: Roundtrip property
- Tests 14-16: Line number mapping
- Tests 18-20: Serialization

**Tests Passing (5)**: ⚠️ Accidental passes due to placeholder values
- Test 8-9: Return 0 for invalid/negative lines (happens to match expectation ≤0)
- Test 12: Monotonicity passes because 0 ≥ 0 for all cases
- Test 13: Empty source expects 0 lines, placeholder returns 0
- Test 17: Consistency passes because both calls return 0

**Verdict**: **RED Phase Successful** - Core functionality tests are failing, ready for GREEN phase implementation.

## Discoveries

### Ruchy Syntax Discovery

**Issue**: Ruchy parser does not support inline comments after return statements.

**Example (Breaks)**:
```ruchy
fun create_source_map(filename: String, line_count: i64) -> i64 {
    0  // Placeholder - returns dummy map ID
}
```

**Solution (Works)**:
```ruchy
fun create_source_map(filename: String, line_count: i64) -> i64 {
    0
}
```

**Documented In**: This is a known parser limitation (Stage 1 at 80% completion).

**Workaround**: Place comments above return statement instead of inline.

### Unicode Character Handling

**Issue**: Initial version used Unicode characters (✅ ❌ 📊 📈 ═) in strings.

**Discovery**: While Ruchy technically supports Unicode in strings, it's cleaner to use ASCII for test output to avoid potential rendering issues across terminals.

**Solution**: Replaced Unicode with ASCII equivalents:
- ✅ → PASS
- ❌ → FAIL
- 📊 → (removed)
- ═ → -

## Next Steps

**GREEN Phase** (Week 2-3):
1. Implement `create_source_map` - Simple line count tracking
2. Implement `map_source_to_target` - 1:1 line mapping for now
3. Implement `map_target_to_source` - Reverse lookup
4. Implement `generate_source_map_from_code` - Parse source and count lines
5. Implement helper functions (`get_line_count`, `get_source_filename`)
6. Implement serialization (`serialize_source_map`, `deserialize_source_map`)

**Minimal Implementation Strategy**:
- Use simple HashMap or array for line mappings
- 1:1 mapping initially (source line N → target line N)
- No compression or optimization yet
- Single global source map (no multi-file support)

**Target**: Get all 20 tests passing with minimal implementation.

---

**Status**: ✅ RED Phase Complete - Tests failing as expected
**File**: `validation/debugging/test_source_maps.ruchy` (536 lines, 20 tests)
**Next**: GREEN Phase - Minimal implementation to make tests pass
