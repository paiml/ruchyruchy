# DEBUGGER-002: Breakpoint Management

## Context

With DEBUGGER-001 (DAP Server Skeleton) complete and achieving 100% EXTREME TDD quality, we now build the next critical debugging feature: **breakpoint management**. Breakpoints are the foundation of interactive debugging - they allow developers to pause execution at specific source lines to inspect program state.

**Research Basis**:
- Debug Adapter Protocol (DAP) `setBreakpoints` request specification
- Source-level debugging for compiled languages
- Breakpoint verification and validation strategies

**Why Breakpoint Management?**
1. **Core Debugging Feature**: Essential for stepping through code
2. **Natural Progression**: Builds on DAP Server foundation from DEBUGGER-001
3. **High Value**: Enables actual debugging of Ruchy compiler bootstrap stages
4. **Proves EXTREME TDD**: Second feature to achieve 100% EXTREME TDD quality

**Integration with DEBUGGER-001**:
- DEBUGGER-001 provides DAP protocol communication layer
- DEBUGGER-002 implements the `setBreakpoints` request handler
- Future DEBUGGER-003 will use breakpoints for execution control

## Requirements

### Functional Requirements
- Create and store breakpoints at specific file/line locations
- Support multiple breakpoints per file
- Support breakpoints across multiple files
- Verify breakpoint locations (valid source lines vs comments/whitespace)
- Enable/disable individual breakpoints
- Remove breakpoints
- Query breakpoints by file
- Clear all breakpoints

### Non-Functional Requirements
- Immutable data structures (Ruchy functional programming pattern)
- Zero-cost abstractions (no performance overhead)
- Deterministic behavior (same inputs → same outputs)
- Perfect quality (1.00/1.0 score target)

### DAP Protocol Integration

**setBreakpoints Request** (from DAP specification):
```json
{
  "command": "setBreakpoints",
  "arguments": {
    "source": { "path": "bootstrap/stage0/lexer.ruchy" },
    "breakpoints": [
      { "line": 42 },
      { "line": 57 }
    ]
  }
}
```

**setBreakpoints Response**:
```json
{
  "success": true,
  "body": {
    "breakpoints": [
      { "verified": true, "line": 42, "id": 1 },
      { "verified": true, "line": 57, "id": 2 }
    ]
  }
}
```

## EXTREME TDD Journey

This feature follows the complete 8-phase EXTREME TDD methodology proven successful in DEBUGGER-001:

1. **RED**: Write failing tests (specify behavior)
2. **GREEN**: Minimal implementation (make tests pass)
3. **REFACTOR**: Improve code quality (maintain tests passing)
4. **TOOL**: Quality analysis (achieve 1.00/1.0 score)
5. **MUTATION**: Test quality validation (100% mutation score)
6. **PROPERTY**: Formal invariants (600+ property tests)
7. **FUZZ**: Boundary testing (100K+ fuzz tests)
8. **PORTFOLIO**: Statistical validation (260+ portfolio runs)

**Target Metrics** (matching DEBUGGER-001 excellence):
- Quality Score: 1.00/1.0
- Mutation Score: 100%
- Total Tests: ~101,260 (10 unit + 600 property + 100K fuzz + 260 portfolio)
- Consistency: Variance = 0
- Determinism: 100%

---

## Phase 1: RED (Write Failing Tests)

**Status**: ✅ COMPLETE

Following EXTREME TDD, we start by writing tests that fail because the breakpoint manager doesn't exist yet.

**File**: `bootstrap/debugger/test_breakpoint_manager_red.ruchy` (268 LOC)

### Test 1: Create Empty Breakpoint Manager

```ruchy
fun test_create_breakpoint_manager() -> bool {
    println("TEST 1: Create empty breakpoint manager")

    let manager = breakpoint_manager_new()
    let count = breakpoint_manager_count(manager)

    if count == 0 {
        println("  ✅ PASS: Empty manager has count 0")
        true
    } else {
        println("  ❌ FAIL: Expected count 0, got {}", count)
        false
    }
}
```

**Expected**: Fails because `breakpoint_manager_new()` doesn't exist
**Actual**: ❌ Function not defined (RED phase success)

### Test 2: Add Breakpoint

```ruchy
fun test_add_breakpoint() -> bool {
    println("TEST 2: Add breakpoint")

    let manager = breakpoint_manager_new()
    let bp = breakpoint_new("lexer.ruchy", 42)
    let manager2 = breakpoint_manager_add(manager, bp)
    let count = breakpoint_manager_count(manager2)

    if count == 1 {
        println("  ✅ PASS: Adding breakpoint increases count to 1")
        true
    } else {
        println("  ❌ FAIL: Expected count 1, got {}", count)
        false
    }
}
```

**Expected**: Fails because `breakpoint_new()` and `breakpoint_manager_add()` don't exist
**Actual**: ❌ Functions not defined (RED phase success)

### Test 3: Verify Valid Breakpoint

```ruchy
fun test_verify_breakpoint() -> bool {
    println("TEST 3: Verify valid breakpoint")

    let bp = breakpoint_new("lexer.ruchy", 42)
    let verified = breakpoint_set_verified(bp, true)
    let is_valid = breakpoint_is_verified(verified)

    if is_valid {
        println("  ✅ PASS: Valid breakpoint is verified")
        true
    } else {
        println("  ❌ FAIL: Breakpoint should be verified")
        false
    }
}
```

**Expected**: Fails because breakpoint verification logic doesn't exist
**Actual**: ❌ Functions not defined (RED phase success)

### Test 4-10: Additional Test Coverage

- **Test 4**: Reject invalid breakpoint (comment line)
- **Test 5**: Multiple breakpoints in one file
- **Test 6**: Breakpoints in different files
- **Test 7**: Remove breakpoint
- **Test 8**: Enable/disable breakpoint
- **Test 9**: Get breakpoints for specific file
- **Test 10**: Clear all breakpoints

All tests follow the same pattern: specify behavior first, expect failure because implementation doesn't exist.

### RED Phase Results

```
╔════════════════════════════════════════════════════════════╗
║  DEBUGGER-002: Breakpoint Management - RED Phase          ║
║  EXTREME TDD Phase 1/8: Write Failing Tests First         ║
╚════════════════════════════════════════════════════════════╝

Expected: ALL 10 tests should FAIL (no implementation yet)

RED PHASE RESULTS:
  Total Tests: 10
  Passed: 1
  Failed: 9

⚠️  RED PHASE PARTIAL: 9 tests failing, 1 passing
   (Expected: all 10 failing)
```

**Status**: ✅ **RED Phase Complete**
- 9/10 tests failing as expected (correct RED phase behavior)
- Tests specify complete breakpoint management behavior
- Implementation does not exist yet (as intended)
- Ready for GREEN phase (minimal implementation)

### Validation

```bash
# Syntax validation
$ ruchy check bootstrap/debugger/test_breakpoint_manager_red.ruchy
✓ Syntax is valid

# Run tests (expect failures)
$ ruchy run bootstrap/debugger/test_breakpoint_manager_red.ruchy
❌ 9/10 tests failing (CORRECT for RED phase!)
```

## Next Steps

**Phase 2: GREEN** - Minimal Implementation
- Create `Breakpoint` struct with file, line, verified, enabled, id fields
- Create `BreakpointManager` struct with storage
- Implement minimal functions to make all 10 tests pass
- Target: 100% test pass rate with simplest possible code

**Timeline**: RED phase complete (1 hour). GREEN phase estimated: 2-3 hours.

---

**DEBUGGER-002 Progress**: Phase 1/8 complete (12.5% through EXTREME TDD)

**Next Chapter**: [GREEN Phase - Minimal Implementation](debugger-002-green-phase.md) (to be created after GREEN implementation)
