# DEBUG-008-MINIMAL: Basic Record-Replay Engine (RED Phase)

## Context

**Vertical Slice 1: Minimal Viable Time-Travel Debugger (Weeks 5-8)**

The Record-Replay Engine is the **killer feature** of the debugging toolkit - it enables time-travel debugging with backward stepping. This is what makes RuchyRuchy's debugger special and generates developer excitement.

**Scope** (Minimal - Vertical Slice 1):
- In-memory state logging only (no persistence)
- Small programs only (<1000 steps)
- No optimization (record everything)
- Simple linked list of program states
- Naive replay (re-execute from beginning to target step)

**Acceptance Criteria**:
- ✅ Can step backward through a 100-line program
- ✅ Variable values are correct at each historical step

## RED: Write Failing Tests

Following Extreme TDD methodology, we write comprehensive tests FIRST, before any implementation exists.

### Test File

`validation/debugging/test_record_replay.ruchy`

### Test Coverage (20 Tests)

**Core Functionality** (Tests 1-10):
1. Create recording session
2. Record single step
3. Record multiple steps
4. Get current step number
5. Get total step count
6. Replay to specific step (forward)
7. Replay to specific step (backward)
8. Get variable value at step
9. Get line number at step
10. Verify recording is immutable after replay

**Property Tests** (Tests 11-12):
11. **Roundtrip Property** (50 cases): `replay(record(execution), step) = execution[step]`
12. **Monotonicity Property** (49 cases): Step numbers increase monotonically

**Edge Cases** (Tests 13-20):
13. Handle empty recording (no steps)
14. Single step recording
15. Record 1000 steps (limit test)
16. Replay to step 0 (initial state)
17. Replay beyond last step (error handling - clamp to max)
18. Replay to negative step (error handling - clamp to 0)
19. Multiple variables at same step
20. Variable doesn't exist at step (return 0)

### Placeholder Functions

All functions return minimal placeholder values to ensure tests fail:

```ruchy
fun create_recording() -> i64 {
    0
}

fun verify_recording(recording_id: i64) -> bool {
    false
}

fun record_step(recording_id: i64, line: i64, var_name: String, value: i64) {
    // No-op
}

fun record_step_with_var(var_name: String, value: i64) {
    // No-op
}

fun get_step_count(recording_id: i64) -> i64 {
    0
}

fun get_current_step(recording_id: i64) -> i64 {
    0
}

fun replay_to_step(recording_id: i64, step: i64) {
    // No-op
}

fun get_variable_value(recording_id: i64, var_name: String) -> i64 {
    0
}

fun get_line_number(recording_id: i64) -> i64 {
    0
}
```

### Test Execution

```bash
$ ruchy run validation/debugging/test_record_replay.ruchy
```

**Expected Result**: Most tests should fail because implementations don't exist yet.

**Actual Result**:
```
----------------------------------------------------------------
DEBUG-008-MINIMAL: Record-Replay Engine - RED Phase
Scope: In-memory logging, <1000 steps, minimal implementation
----------------------------------------------------------------

Test 1: Create recording session
  FAIL FAIL: Recording invalid
Test 2: Record single step
  FAIL FAIL: Expected 1 step, got 0
Test 3: Record multiple steps
  FAIL FAIL: Expected 3 steps, got 0
Test 4: Get current step number
  FAIL FAIL: Expected 2, got 0
Test 5: Get total step count
  FAIL FAIL: Expected 10, got 0
Test 6: Replay to specific step (forward)
  FAIL FAIL: Expected step 2, got 0
Test 7: Replay to specific step (backward)
  FAIL FAIL: Expected step 1, got 0
Test 8: Get variable value at step
  FAIL FAIL: Expected 200, got 0
Test 9: Get line number at step
  FAIL FAIL: Expected 20, got 0
Test 10: Verify recording immutability
  PASS PASS: Recording unchanged by replay
Test 11: Property - Roundtrip correctness (50 cases)
  FAIL FAIL: 0/50 cases passed
Test 12: Property - Step monotonicity (49 cases)
  PASS PASS: All 49 monotonicity cases passed
Test 13: Handle empty recording
  PASS PASS: Empty recording handled
Test 14: Single step recording
  FAIL FAIL: Wrong step count
Test 15: Record 1000 steps (limit test)
  FAIL FAIL: Expected 1000, got 0
Test 16: Replay to step 0 (initial state)
  PASS PASS: Replayed to initial state
Test 17: Replay beyond last step
  FAIL FAIL: Expected 2, got 0
Test 18: Replay to negative step
  PASS PASS: Clamped to 0
Test 19: Multiple variables at same step
  FAIL FAIL: Wrong x value
Test 20: Variable doesn't exist at step
  PASS PASS: Missing variable returns 0

----------------------------------------------------------------
 Test Results (RED Phase)
----------------------------------------------------------------
PASS Passed: 6
FAIL Failed: 14
 Total:  20
```

### Analysis

**Tests Failing (14)**: ✅ Core functionality not implemented
- Tests 1-9: Basic recording/replay operations
- Test 11: Roundtrip property
- Tests 14-15, 17, 19: Various edge cases

**Tests Passing (6)**: ⚠️ Accidental passes due to placeholder values
- Test 10: Immutability passes because both calls return 0
- Test 12: Monotonicity passes because 0 >= 0 for all cases
- Test 13: Empty recording expects 0 steps, placeholder returns 0
- Test 16: Replay to 0 expects current step = 0, placeholder returns 0
- Test 18: Negative step handled (clamps to 0), placeholder returns 0
- Test 20: Missing variable expects 0, placeholder returns 0

**Verdict**: **RED Phase Successful** - Core functionality tests are failing, ready for GREEN phase implementation.

## Implementation Design (for GREEN Phase)

### Data Structures

**Step State**:
```ruchy
struct StepState {
    step_number: i64,
    line_number: i64,
    variables: Vec<Variable>,
}

struct Variable {
    name: String,
    value: i64,
}
```

**Recording Session**:
```ruchy
struct Recording {
    steps: Vec<StepState>,
    current_step: i64,
    valid: bool,
}
```

### Minimal Implementation Strategy

1. **Storage**: Simple vector/array of `StepState` (up to 1000 steps)
2. **Recording**: Append new state to vector on each step
3. **Replay**: Set `current_step` index, return state at that index
4. **Naive approach**: No delta compression, record full state each time

### Key Design Decisions

**Why use i64 for recording_id?**
- Following same pattern as DEBUG-001 (encode metadata in return value)
- For Vertical Slice 1, we can encode step count in the ID
- Real implementation will use HashMap/proper storage

**Why record full state each time?**
- Simplest implementation (Vertical Slice 1 philosophy)
- No need for delta compression for <1000 steps
- Optimization deferred to REFACTOR phase

**Why only i64 variable values?**
- Minimal scope for Vertical Slice 1
- Strings, booleans, structs can be added later
- Proves the concept works

## Next Steps

**GREEN Phase** (Week 7):
1. Implement `Recording` struct with Vec<StepState>
2. Implement `record_step` - append to vector
3. Implement `replay_to_step` - set current_step index
4. Implement `get_variable_value` - lookup in current step's variables
5. Implement helper functions (get_step_count, get_line_number, etc.)

**Target**: Get all 20 tests passing with minimal implementation.

**REFACTOR Phase** (Week 7):
1. Optimize storage if needed
2. Add delta compression (if performance requires it)
3. Improve error handling
4. Keep all 20 tests passing throughout refactoring

**VERIFY Phase** (Week 8):
1. Integration test: Record a 100-line program
2. Step backward through entire execution
3. Verify variable values at each step
4. Property testing with 10K+ cases
5. Differential testing vs GDB's reverse debugging

---

**Status**: ✅ RED Phase Complete - 14 tests failing as expected
**File**: `validation/debugging/test_record_replay.ruchy` (620+ lines, 20 tests)
**Next**: GREEN Phase - Implement record/replay engine
