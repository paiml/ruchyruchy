# Next Steps Recommendation

**Date**: October 21, 2025
**Current Status**: v0.2.0 released, DEBUGGER-001 complete (100% EXTREME TDD)
**Momentum**: 🔥 HIGH - Perfect time to continue!

---

## 🎯 PRIMARY RECOMMENDATION: DEBUGGER-002 (Breakpoint Management)

### Why This Is The Best Next Step

1. **Natural Progression** - Builds on DEBUGGER-001 DAP Server foundation
2. **Prove EXTREME TDD Is Repeatable** - Show it works for multiple features
3. **High Value** - Breakpoints are critical debugging infrastructure
4. **Maintains Momentum** - Keep the debugging roadmap moving forward
5. **Production Impact** - Enables actual debugging of Ruchy compiler

---

## DEBUGGER-002 Specification

### Overview
**Component**: Breakpoint Management
**Foundation**: DEBUGGER-001 (DAP Server Skeleton)
**Methodology**: EXTREME TDD (all 8 phases)
**Target**: 100% completion with world-class quality

### Core Features

#### 1. Source-Level Breakpoints
Break at specific lines in Ruchy source code:
```ruchy
// File: bootstrap/stage0/lexer.ruchy
fun tokenize_single(input: String, start: i32) -> Token {
    let ch = char_at(input, start);  // <- Breakpoint here
    match ch {
        '{' => Token::LeftBrace,
        '}' => Token::RightBrace,
        _ => Token::Unknown(ch)
    }
}
```

**DAP Protocol**:
```json
{
  "command": "setBreakpoints",
  "arguments": {
    "source": { "path": "bootstrap/stage0/lexer.ruchy" },
    "breakpoints": [{ "line": 42 }]
  }
}
```

#### 2. Breakpoint Storage
- In-memory breakpoint registry
- File → Line → Breakpoint mapping
- Verified vs unverified breakpoints
- Enable/disable individual breakpoints

#### 3. Breakpoint Verification
- Validate breakpoint locations (valid source lines)
- Check if line is executable (not comment, not whitespace)
- Return verified breakpoint locations to client
- Handle invalid breakpoint requests gracefully

#### 4. DAP Integration
- `setBreakpoints` request handler
- `breakpoint` event emission
- Breakpoint hit detection (future: DEBUGGER-003)
- Multiple file support

---

## EXTREME TDD Plan for DEBUGGER-002

### Phase 1: RED (Write Failing Tests)
**Duration**: ~1-2 hours
**Deliverable**: 5-10 failing tests

**Tests To Write**:
```ruchy
// Test 1: Create empty breakpoint manager
fun test_create_breakpoint_manager() -> bool {
    let manager = breakpoint_manager_new()
    breakpoint_manager_count(manager) == 0
}

// Test 2: Add breakpoint
fun test_add_breakpoint() -> bool {
    let manager = breakpoint_manager_new()
    let bp = breakpoint_new("lexer.ruchy", 42)
    let manager2 = breakpoint_manager_add(manager, bp)
    breakpoint_manager_count(manager2) == 1
}

// Test 3: Verify valid breakpoint
fun test_verify_breakpoint() -> bool {
    let bp = breakpoint_new("lexer.ruchy", 42)
    let verified = breakpoint_verify(bp, source_map)
    verified.is_valid == true
}

// Test 4: Reject invalid breakpoint (comment line)
fun test_reject_comment_breakpoint() -> bool {
    let bp = breakpoint_new("lexer.ruchy", 5)  // Line 5 is a comment
    let verified = breakpoint_verify(bp, source_map)
    verified.is_valid == false
}

// Test 5: Multiple breakpoints in one file
fun test_multiple_breakpoints() -> bool {
    let manager = breakpoint_manager_new()
    let bp1 = breakpoint_new("lexer.ruchy", 42)
    let bp2 = breakpoint_new("lexer.ruchy", 57)
    let manager2 = breakpoint_manager_add(manager, bp1)
    let manager3 = breakpoint_manager_add(manager2, bp2)
    breakpoint_manager_count(manager3) == 2
}

// Test 6: Breakpoints in different files
fun test_multiple_files() -> bool {
    let manager = breakpoint_manager_new()
    let bp1 = breakpoint_new("lexer.ruchy", 42)
    let bp2 = breakpoint_new("parser.ruchy", 100)
    // ... add and verify
}

// Test 7: Remove breakpoint
fun test_remove_breakpoint() -> bool {
    let manager = breakpoint_manager_new()
    let bp = breakpoint_new("lexer.ruchy", 42)
    let manager2 = breakpoint_manager_add(manager, bp)
    let manager3 = breakpoint_manager_remove(manager2, "lexer.ruchy", 42)
    breakpoint_manager_count(manager3) == 0
}

// Test 8: Enable/disable breakpoint
fun test_toggle_breakpoint() -> bool {
    let bp = breakpoint_new("lexer.ruchy", 42)
    let bp_disabled = breakpoint_disable(bp)
    bp_disabled.enabled == false
}

// Test 9: Get breakpoints for file
fun test_get_file_breakpoints() -> bool {
    let manager = // ... add breakpoints to multiple files
    let lexer_bps = breakpoint_manager_get_for_file(manager, "lexer.ruchy")
    lexer_bps.len() == 2
}

// Test 10: Clear all breakpoints
fun test_clear_all() -> bool {
    let manager = // ... add breakpoints
    let manager2 = breakpoint_manager_clear_all(manager)
    breakpoint_manager_count(manager2) == 0
}
```

**Status After RED**: ❌ 10/10 tests failing (expected)

### Phase 2: GREEN (Minimal Implementation)
**Duration**: ~2-3 hours
**Deliverable**: All tests passing

**Structures**:
```ruchy
struct Breakpoint {
    file: String,
    line: i32,
    verified: bool,
    enabled: bool,
    id: i32
}

struct BreakpointManager {
    breakpoints: Vec<Breakpoint>,
    next_id: i32
}
```

**Functions**:
```ruchy
fun breakpoint_manager_new() -> BreakpointManager
fun breakpoint_manager_add(manager: BreakpointManager, bp: Breakpoint) -> BreakpointManager
fun breakpoint_manager_remove(manager: BreakpointManager, file: String, line: i32) -> BreakpointManager
fun breakpoint_manager_get_for_file(manager: BreakpointManager, file: String) -> Vec<Breakpoint>
fun breakpoint_new(file: String, line: i32) -> Breakpoint
fun breakpoint_verify(bp: Breakpoint, source_map: SourceMap) -> Breakpoint
fun breakpoint_enable(bp: Breakpoint) -> Breakpoint
fun breakpoint_disable(bp: Breakpoint) -> Breakpoint
```

**Status After GREEN**: ✅ 10/10 tests passing

### Phase 3: REFACTOR (Code Quality)
**Duration**: ~1-2 hours
**Deliverable**: Improved code with 0% duplication

**Improvements**:
- Extract helper functions
- Add constants
- Reduce duplication
- Apply `ruchy fmt`
- LOC reduction target: 15-20%

**Status After REFACTOR**: ✅ 10/10 tests passing, code clean

### Phase 4: TOOL (Quality Analysis)
**Duration**: ~1 hour
**Deliverable**: Quality metrics verified

**Tools To Run**:
```bash
ruchy score breakpoint_manager.ruchy     # Target: 1.00/1.0
ruchy lint breakpoint_manager.ruchy      # Target: A+ grade
ruchy check breakpoint_manager.ruchy     # Target: ✓ Valid
ruchy prove breakpoint_manager.ruchy     # Target: Proofs pass
ruchy provability breakpoint_manager.ruchy  # Target: >80
ruchy runtime breakpoint_manager.ruchy   # Target: Performance OK
```

**Status After TOOL**: ✅ Perfect quality scores

### Phase 5: MUTATION (Test Quality)
**Duration**: ~2-3 hours
**Deliverable**: 100% mutation score

**Mutations To Test**:
1. Change `==` to `!=` in breakpoint matching
2. Remove verification check
3. Skip adding breakpoint to storage
4. Return wrong count
5. Don't mark as verified
6. etc.

**Status After MUTATION**: ✅ 100% mutation score

### Phase 6: PROPERTY (Formal Specifications)
**Duration**: ~3-4 hours
**Deliverable**: 600+ property tests, 6 formal invariants

**Invariants**:
```ruchy
// @invariant: count() returns number of enabled breakpoints
// ∀m. count(m) = |{bp ∈ m.breakpoints | bp.enabled}|

// @invariant: add() increases count by 1
// ∀m,bp. count(add(m,bp)) = count(m) + 1

// @invariant: remove() decreases count by 1 (if exists)
// ∀m,f,l. exists(m,f,l) → count(remove(m,f,l)) = count(m) - 1

// @invariant: verified breakpoints have valid line numbers
// ∀bp. bp.verified → bp.line > 0

// @invariant: get_for_file returns only matching file
// ∀m,f. all(bp ∈ get_for_file(m,f), bp.file == f)

// @invariant: clear_all results in count 0
// ∀m. count(clear_all(m)) = 0
```

**Status After PROPERTY**: ✅ 600+ tests, 6 invariants verified

### Phase 7: FUZZ (Boundary Testing)
**Duration**: ~2-3 hours
**Deliverable**: 100K+ fuzz tests, 0 crashes

**Fuzz Tests**:
- Add 100K breakpoints (high volume)
- Random line numbers (-1000 to 999999)
- Random file names (empty, very long, special chars)
- Massive file lists (1000+ files)
- Rapid add/remove cycling

**Status After FUZZ**: ✅ 100K+ tests, 0 crashes

### Phase 8: PORTFOLIO (Statistical Validation)
**Duration**: ~1-2 hours
**Deliverable**: 260 portfolio runs, 100% consistency

**Portfolio Tests**:
- N=30 statistical validation
- N=50 extended validation
- N=100 high-volume consistency
- N=30 different files
- N=50 determinism validation

**Status After PORTFOLIO**: ✅ 260 runs, variance = 0

---

## Estimated Timeline

**Total Duration**: ~15-20 hours
**Can Be Completed In**: 2-3 work sessions

**Breakdown**:
- Day 1 (6-8h): RED + GREEN + REFACTOR + TOOL
- Day 2 (5-7h): MUTATION + PROPERTY
- Day 3 (4-5h): FUZZ + PORTFOLIO + Documentation

---

## Expected Deliverables

### Code Files
1. `bootstrap/debugger/breakpoint_manager.ruchy` (~200-300 LOC)
2. `bootstrap/debugger/test_breakpoint_manager_red.ruchy` (RED phase tests)
3. `bootstrap/debugger/test_breakpoint_manager_mutation.ruchy` (Mutation tests)
4. `bootstrap/debugger/breakpoint_manager_properties.ruchy` (600+ property tests)
5. `bootstrap/debugger/breakpoint_manager_fuzz.ruchy` (100K+ fuzz tests)
6. `bootstrap/debugger/breakpoint_manager_portfolio.ruchy` (260 statistical runs)

### Documentation
1. `bootstrap/debugger/BREAKPOINT_MANAGER_SUMMARY.md`
2. Updated `INTEGRATION.md`
3. Book chapter: `book/src/phase4_debugger/debugger-002-breakpoint-management.md`

### Test Results
- **Total Tests**: ~101,260 (10 unit + 600 property + 100K fuzz + 260 portfolio)
- **Success Rate**: 100%
- **Quality Score**: 1.00/1.0
- **Mutation Score**: 100%
- **Provability**: 85-90/100

---

## Integration with DEBUGGER-001

### How They Work Together

**DEBUGGER-001** (DAP Server Skeleton):
- Handles DAP protocol communication
- Manages server lifecycle (initialize, launch, disconnect)
- Routes DAP requests to appropriate handlers

**DEBUGGER-002** (Breakpoint Management):
- Handles `setBreakpoints` DAP requests
- Stores and verifies breakpoints
- Provides breakpoint query interface

**Integration Point**:
```ruchy
// In DEBUGGER-001 DAP Server
fun handle_set_breakpoints_request(server: DAPServer, request: Request) -> Response {
    let file = request.arguments.source.path
    let lines = request.arguments.breakpoints.map(|bp| bp.line)

    // Delegate to DEBUGGER-002
    let breakpoints = breakpoint_manager_set_for_file(
        server.breakpoint_manager,
        file,
        lines
    )

    // Return verified breakpoints to client
    Response::success(breakpoints)
}
```

---

## Alternative Options (Not Recommended Right Now)

### Option B: DEBUGGER-003 (Execution Control)
**Why Not Now**: Requires DEBUGGER-002 to be complete first

### Option C: Document EXTREME TDD Methodology
**Why Not Now**: Better after proving it works for 2+ features

### Option D: File Ruchy GitHub Issue (Early Return Bug)
**Why Not Now**: Lower priority than building infrastructure

### Option E: Work on Bootstrap Compiler Stages
**Why Not Now**: All 4 stages already complete (100%)

---

## Success Metrics for DEBUGGER-002

### Quality Metrics
- ✅ Quality Score: 1.00/1.0 (perfect)
- ✅ Mutation Score: 100%
- ✅ Test Count: 101,260+ comprehensive tests
- ✅ Success Rate: 100%
- ✅ Provability: 85-90/100
- ✅ Consistency: Perfect (variance = 0)

### Functional Metrics
- ✅ Can set breakpoints at valid source lines
- ✅ Can verify breakpoint locations
- ✅ Can store breakpoints for multiple files
- ✅ Can enable/disable breakpoints
- ✅ Can remove breakpoints
- ✅ Can query breakpoints by file
- ✅ DAP protocol integration working

### Process Metrics
- ✅ All 8 EXTREME TDD phases complete
- ✅ Zero regressions (all DEBUGGER-001 tests still passing)
- ✅ Documentation comprehensive
- ✅ Book chapter published
- ✅ Quality gates passing

---

## Long-Term Roadmap Preview

**After DEBUGGER-002**, the natural progression is:

1. **DEBUGGER-003**: Execution Control (launch, pause, continue, step)
2. **DEBUGGER-004**: Variable Inspection (view compiler state)
3. **DEBUGGER-005**: Time-Travel Debugging (record/replay)
4. **DEBUGGER-006**: AST Visualization
5. **DEBUGGER-007**: Fault Localization (SBFL)
6. **DEBUGGER-008**: Full VS Code Extension

**End Goal**: Complete debugging infrastructure for Ruchy compiler

---

## Recommendation

🎯 **START DEBUGGER-002 (Breakpoint Management) NOW**

**Reasons**:
1. ✅ Perfect momentum after DEBUGGER-001 success
2. ✅ Builds on solid foundation
3. ✅ Proves EXTREME TDD is repeatable
4. ✅ High-value feature (breakpoints critical for debugging)
5. ✅ Clear specification and plan
6. ✅ 2-3 session completion timeline
7. ✅ Continues debugging infrastructure roadmap

**What To Do**:
1. Start with RED phase (write 10 failing tests)
2. Apply EXTREME TDD methodology (all 8 phases)
3. Maintain perfect quality (1.00/1.0 score)
4. Document journey in book
5. Commit after each phase
6. Achieve 100% completion

**Expected Outcome**:
- DEBUGGER-002 complete with world-class quality
- Proof that EXTREME TDD works for multiple features
- Debugging infrastructure 2/8 complete
- v0.3.0 release candidate material

---

## Ready To Begin?

**Answer**: YES! 🚀

The foundation is solid, the methodology is proven, and the specification is clear. Let's continue building world-class debugging infrastructure for the Ruchy compiler!

**First Step**: Start DEBUGGER-002 RED phase (write failing tests for breakpoint management)
