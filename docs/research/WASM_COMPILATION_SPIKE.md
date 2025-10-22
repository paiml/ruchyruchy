# WASM Compilation Spike (Genchi Genbutsu)

**Date**: 2025-10-22
**Status**: Proposed - Pre-Implementation Validation
**Duration**: 1-2 days (time-boxed)
**Philosophy**: "Go and See for Yourself" - Verify assumptions before full commitment

---

## Purpose

**Critical Principle**: Before committing to a multi-week WASM compilation project, we must practice **Genchi Genbutsu** (現地現物) - "Go to the source to find the facts."

**Problem**: The WASM research document makes architectural assumptions without empirical validation of the hardest technical challenges.

**Solution**: Execute a small, time-boxed spike to **validate or challenge** our assumptions about:
1. Closure compilation (hardest problem)
2. Garbage collection strategy
3. Performance characteristics
4. Development velocity

---

## Critical Assumptions to Validate

### Assumption 1: Lambda Lifting is Straightforward

**Claim**: "Lambdas can be lifted to top-level functions"

**Reality Check Needed**:
```ruchy
// Can we handle captured variables?
fun make_counter() {
    let mut count = 0
    (x) => {
        count = count + x
        count
    }
}
```

**Question**: How do we compile mutable captured state in WASM?
- Option A: Closure environment in linear memory?
- Option B: Global variables (breaks referential transparency)?
- Option C: Explicit closure records passed as parameters?

**Spike Test**: Compile this exact example to WAT and verify it executes correctly.

### Assumption 2: Host GC is Sufficient

**Claim**: "Use JavaScript GC via imports (Option B)"

**Reality Check Needed**:
- What's the performance cost of JS-WASM boundary crossings?
- Can we allocate Ruchy objects from WASM efficiently?
- How do we represent reference types?

**Spike Test**: Create a simple allocation-heavy program and measure:
- Allocation throughput
- GC pause impact
- Memory overhead

### Assumption 3: "~2 weeks" Timeline is Achievable

**Claim**: "Medium effort, comparable to Rust target"

**Reality Check Needed**:
- How long does it actually take to implement ONE expression type correctly?
- What's the iteration cycle (code → WAT → test → debug)?
- How many unexpected issues arise?

**Spike Test**: Implement 3 expression types (Number, Binary, Lambda) and measure:
- Lines of code written
- Time to working implementation
- Number of issues discovered
- Extrapolate to full implementation

---

## Spike Implementation Plan

### Day 1: Minimum Viable WASM Emitter

**Objective**: Compile the simplest possible Ruchy program to WAT

**Program**:
```ruchy
fun add(x, y) { x + y }
```

**Expected WAT Output**:
```wat
(module
  (func $add (param $x i32) (param $y i32) (result i32)
    (i32.add (local.get $x) (local.get $y)))
  (export "add" (func $add)))
```

**Tasks**:
1. ⏱️ 1 hour: Set up WAT toolchain (wat2wasm, wasm-interp)
2. ⏱️ 2 hours: Implement `emit_expr_wasm()` for:
   - Numbers (`i32.const`)
   - Identifiers (`local.get`)
   - Binary operations (`i32.add`, etc.)
3. ⏱️ 1 hour: Implement `emit_function_wasm()`
4. ⏱️ 1 hour: Test compilation and execution
5. ⏱️ 1 hour: Document issues and learnings

**Success Criteria**:
- ✅ WAT compiles to WASM binary (wat2wasm succeeds)
- ✅ WASM executes correctly (wasm-interp or Node.js)
- ✅ add(2, 3) returns 5

**Failure Criteria** (valuable learning!):
- ❌ More than 6 hours spent without working output
- ❌ Fundamental architectural mismatch discovered
- ❌ Toolchain issues block progress

### Day 2: The Hard Problem - Closures

**Objective**: Validate lambda lifting strategy with captured variables

**Program**:
```ruchy
fun make_adder(n) {
    (x) => x + n  // n is captured
}

let add5 = make_adder(5)
add5(3)  // Should return 8
```

**Approach Options to Test**:

**Option A: Closure Records in Linear Memory**
```wat
;; Allocate closure struct: [function_index, captured_n]
(func $make_adder (param $n i32) (result i32)
  (local $closure_addr i32)
  ;; Allocate 8 bytes in linear memory
  (local.set $closure_addr (call $malloc (i32.const 8)))
  ;; Store function index
  (i32.store (local.get $closure_addr) (i32.const 0))  ;; $lambda_0 index
  ;; Store captured value
  (i32.store (i32.add (local.get $closure_addr) (i32.const 4)) (local.get $n))
  (local.get $closure_addr))  ;; Return closure address

;; Lambda that takes closure pointer
(func $lambda_0 (param $closure_ptr i32) (param $x i32) (result i32)
  (local $n i32)
  ;; Load captured n from closure
  (local.set $n (i32.load (i32.add (local.get $closure_ptr) (i32.const 4))))
  (i32.add (local.get $x) (local.get $n)))
```

**Option B: Global Variables (Simpler but Impure)**
```wat
(global $captured_n (mut i32) (i32.const 0))

(func $make_adder (param $n i32) (result i32)
  (global.set $captured_n (local.get $n))
  (i32.const 0))  ;; Return function index

(func $lambda_0 (param $x i32) (result i32)
  (i32.add (local.get $x) (global.get $captured_n)))
```

**Tasks**:
1. ⏱️ 2 hours: Implement Option A (closure records)
2. ⏱️ 1 hour: Test and measure
3. ⏱️ 1 hour: Implement Option B (globals) as fallback
4. ⏱️ 1 hour: Compare approaches
5. ⏱️ 1 hour: Document findings and recommendation

**Success Criteria**:
- ✅ At least ONE approach compiles and executes correctly
- ✅ Clear understanding of tradeoffs (memory, performance, complexity)
- ✅ Confidence in chosen strategy for full implementation

**Failure Criteria**:
- ❌ Neither approach works after 6 hours
- ❌ Fundamental WASM limitation discovered
- ❌ Complexity far exceeds expectations

---

## Measurement Criteria

### 1. Technical Feasibility

**Measure**:
- ✅ Can compile and execute basic expressions? (YES/NO)
- ✅ Can handle closures with captured variables? (YES/NO/PARTIAL)
- ⚠️ Unexpected blockers discovered? (LIST)

### 2. Development Velocity

**Measure**:
- ⏱️ Hours to implement 3 expression types: ____ hours
- ⏱️ Hours to implement closure strategy: ____ hours
- 📊 Extrapolated time for 10 expression types: ____ hours
- 📊 Extrapolated time for full emitter: ____ days

**Reality Check**:
- Original estimate: ~2-3 days for BOOTSTRAP-018
- Spike-based estimate: ____ days (actual data)
- Adjustment factor: ____x

### 3. Complexity Assessment

**Measure**:
- 📏 Lines of code for 3 expressions: ____ LOC
- 📏 Estimated LOC for full emitter: ____ LOC
- 📊 Complexity vs TypeScript emitter: ____x (more/less)
- 📊 Complexity vs Rust emitter: ____x (more/less)

### 4. Performance Characteristics

**Measure** (if time permits):
- ⏱️ WAT compilation time: ____ ms
- ⏱️ WASM execution time vs native Ruchy: ____x overhead
- 💾 Binary size: ____ bytes for simple function

---

## Decision Framework

### Outcome A: Validation ✅

**Indicators**:
- All basic features work
- Closure strategy proven
- Development velocity matches or beats estimate
- No fundamental blockers

**Action**: Proceed with full BOOTSTRAP-018 implementation
**Timeline**: Use spike-based velocity data for realistic estimate

### Outcome B: Partial Validation ⚠️

**Indicators**:
- Basic features work
- Closures require more research
- Development velocity slower than expected
- Solvable issues discovered

**Action**:
1. Document issues clearly
2. Revise approach based on learnings
3. Consider simpler initial scope (no closures)
4. Extend timeline based on actual data

### Outcome C: Invalidation ❌

**Indicators**:
- Fundamental architectural mismatch
- WASM limitations block essential features
- Complexity far exceeds value
- Requires technologies not yet stable (e.g., WasmGC)

**Action**:
1. Document findings thoroughly
2. Defer WASM target until blockers resolved
3. Focus on other high-value work
4. Revisit when ecosystem matures

**This is a SUCCESS**: Genchi Genbutsu saved us from investing weeks in an unfeasible approach!

---

## Deliverables

### 1. Working Code (Even if Minimal)
- `spike/wasm_emitter_minimal.ruchy` - Basic emitter
- `spike/test_simple.wat` - Generated WAT
- `spike/test_simple.wasm` - Compiled binary

### 2. Execution Evidence
```bash
# Compilation
wat2wasm test_simple.wat -o test_simple.wasm

# Execution
node -e "
  const fs = require('fs');
  const bytes = fs.readFileSync('test_simple.wasm');
  WebAssembly.instantiate(bytes).then(m => {
    console.log('add(2, 3) =', m.instance.exports.add(2, 3));
  });
"
# Expected: add(2, 3) = 5
```

### 3. Measurement Report
- `spike/MEASUREMENT_REPORT.md`
  - Technical feasibility: ✅/⚠️/❌
  - Development velocity: X hours for Y features
  - Complexity assessment: Z LOC, A issues
  - Recommendation: Proceed / Revise / Defer

### 4. Updated Research Document
- Revise `WASM_COMPILATION_TARGET.md` based on empirical findings
- Update feasibility rating with evidence
- Revise timeline with measured velocity
- Document discovered issues and mitigations

---

## Time-Boxing Rules

**Strict Limits**:
- Day 1: Max 6 hours
- Day 2: Max 6 hours
- **Total: Max 12 hours (1.5 days)**

**If Time Box Exceeded**:
- Stop immediately
- Document progress and blockers
- Classify outcome (Validation/Partial/Invalidation)
- Make recommendation based on data

**Philosophy**: Time-boxing prevents sunk cost fallacy. We're gathering data, not committing to success.

---

## Success Definition

**Spike is Successful If**:
We have empirical data to make an informed decision about the full WASM implementation, regardless of whether that decision is "go" or "no-go."

**Failure Would Be**:
Proceeding to full implementation without this data, discovering fundamental issues weeks into development, and wasting team time and morale.

---

## Hansei (Reflection) After Spike

After completing the spike, hold a brief reflection meeting:

1. **What did we learn?** (Technical insights)
2. **What surprised us?** (Unexpected findings)
3. **What should we do differently?** (Process improvements)
4. **Do we proceed?** (Go/Revise/Defer decision)
5. **What's the realistic timeline?** (Evidence-based estimate)

**Document this reflection** in `spike/REFLECTION.md` for future reference.

---

## Conclusion

This spike embodies **Genchi Genbutsu** - we will "go and see for ourselves" before committing to a multi-week effort. It respects the team (**Respect for People**) by preventing overburden from an overly optimistic timeline. It embraces **learning** over assumptions.

**Investment**: 1-2 days
**Return**: High-confidence decision backed by empirical data
**Risk Mitigation**: Prevents weeks of potentially wasted effort

**If the spike succeeds**: We have working code, measured velocity, and confidence to proceed.
**If the spike fails**: We learned valuable lessons and avoided a costly mistake.

Both outcomes are victories for the **Toyota Way**.
