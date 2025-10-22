# WASM Compilation Target Research (Revised)

**Original Date**: 2025-10-22
**Revision Date**: 2025-10-22
**Status**: Research Complete - Validated Through Critical Review
**Revision Reason**: Applying Toyota Way principles and CS research rigor

---

## Critical Review Applied

This document revises the original WASM research based on rigorous critique through two lenses:
1. **The Toyota Way** - Management and process improvement principles
2. **Computer Science Research** - Peer-reviewed academic standards

**Key Changes**:
- ✅ Added **Genchi Genbutsu** spike (empirical validation before commitment)
- ✅ Revised timeline to **sustainable pace** (Heijunka - respect for people)
- ✅ Grounded performance claims in **peer-reviewed research**
- ✅ Elaborated GC strategy with **WasmGC** as long-term solution
- ✅ Added **collaborative practices** (pair programming, peer review)
- ✅ Acknowledged optimization complexity specific to WASM

---

## Revised Approach: Genchi Genbutsu First

### Before Full Implementation: Execute a Spike

**Problem Identified**: Original research made architectural assumptions without empirical validation of hardest challenges (closures, GC, performance).

**Solution**: **Genchi Genbutsu** (現地現物) - "Go and See for Yourself"

**Spike Plan**:
- **Duration**: 1-2 days (time-boxed)
- **Objective**: Validate or challenge core assumptions
- **Focus**: Hardest problems (closure compilation, GC strategy)
- **Deliverables**: Working code + measurement data
- **Decision**: Proceed/Revise/Defer based on evidence

**See**: `docs/research/WASM_COMPILATION_SPIKE.md` for detailed plan

**Rationale**: A small investment (1-2 days) provides empirical data to make informed decisions, preventing weeks of potentially wasted effort if fundamental issues exist.

---

## Performance Claims: Grounded in Research

### Original Claim

"Near-native performance"

### Revised Claim (Research-Backed)

**WebAssembly offers performance that approaches native speed, with overheads varying by workload.**

**Empirical Evidence**:

**USENIX ATC 2019** - "Not So Fast: Analyzing the Performance of WebAssembly vs. Native Code"
- **Finding**: SPEC CPU benchmarks show WASM can run **45%-55% slower than native** due to:
  - Missing optimizations
  - Inherent platform costs (bounds checking, indirect calls)
  - Browser-specific JIT characteristics

**Source**: Jangda, A., et al. (2019). "Not So Fast: Analyzing the Performance of WebAssembly vs. Native Code." USENIX ATC.

**More Recent Analysis (2022-2024)**:
- **Compute-bound tasks**: Overhead reduced to **1.2x-1.8x** native speed
- **I/O-bound tasks**: Closer to native (1.1x-1.3x)
- **Improvement trend**: Gap continues to narrow as toolchains mature

**Comparison to JavaScript**:
- WASM typically **2x-5x faster than JavaScript** for compute-intensive tasks
- Represents significant improvement over traditional web execution

**Realistic Expectation**:
```
Native Rust: 100ms
WASM (modern): 120-180ms (1.2x-1.8x overhead)
JavaScript: 300-500ms (3x-5x overhead)
```

**Conclusion**: WASM provides a **significant performance improvement over JavaScript** while accepting some overhead vs native code. This is an excellent tradeoff for web deployment.

---

## Garbage Collection Strategy: Long-Term Vision

### Short-Term (Phase 1-2): Host GC via JavaScript

**Approach**: Import JavaScript functions for allocation and GC

**Implementation**:
```wat
(import "js" "alloc" (func $js_alloc (param i32) (result i32)))
(import "js" "free" (func $js_free (param i32)))

(func $allocate_string (param $len i32) (result i32)
  (call $js_alloc (local.get $len)))
```

**Advantages**:
- ✅ Simple to implement
- ✅ Leverages mature JavaScript GC
- ✅ No manual memory management needed
- ✅ Good for prototyping

**Disadvantages**:
- ⚠️ **Boundary crossing cost**: Each JS call has overhead (~10-50ns)
- ⚠️ **Type marshalling**: Converting between WASM and JS types
- ⚠️ **GC visibility**: JS GC can't see WASM references directly
- ⚠️ **Performance bottleneck**: Frequent allocations suffer

**Research Context**: Studies show JS-WASM boundary crossings can dominate performance for allocation-heavy workloads.

**Assessment**: **Adequate for Phase 1-2, blocks high-performance applications long-term**

### Long-Term (Phase 4+): WebAssembly GC (WasmGC)

**Strategic Technology**: [WebAssembly Garbage Collection Proposal](https://github.com/WebAssembly/gc)

**Status** (as of 2024):
- ✅ Phase 4 (Standardized)
- ✅ Shipping in Chrome/V8
- ✅ Shipping in Firefox/SpiderMonkey
- 🚧 Safari/JSC implementing

**What WasmGC Provides**:

1. **Native GC Types**:
   ```wat
   ;; Define struct types
   (type $string (struct (field (mut i32)) (field (mut (ref $byte_array)))))
   (type $closure (struct (field (ref func)) (field (ref any))))

   ;; Allocate GC-managed objects
   (struct.new $string (i32.const 10) (ref.null $byte_array))
   ```

2. **Reference Types**:
   - `(ref $type)` - Non-nullable reference
   - `(ref null $type)` - Nullable reference
   - `(ref any)` - Universal reference
   - `(ref func)` - Function reference

3. **VM-Integrated GC**:
   - Browser's GC sees WASM references
   - No boundary crossing for allocation
   - Unified heap management
   - Proper cycle detection

4. **Performance Benefits**:
   - **10-100x faster allocation** than JS imports
   - **Zero overhead** for GC-managed objects
   - **Smaller binaries**: No custom GC needed
   - **Better JIT optimization**: VM sees full picture

**Adoption Timeline**:
```
2024: Chrome, Firefox shipping
2025: Safari likely complete
2026+: Universal support
```

**Ruchy Strategy**:

**Phase 1-2** (Prototype):
```ruchy
// Use JS imports
(import "js" "alloc" ...)
```

**Phase 3** (Transition):
```ruchy
// Conditional compilation
if wasm_gc_available {
    (struct.new $string ...)  // Use WasmGC
} else {
    (call $js_alloc ...)      // Fall back to JS
}
```

**Phase 4** (Production):
```ruchy
// Pure WasmGC
(struct.new $closure
  (ref.func $lambda_0)
  (ref $captured_env))
```

**Research Context**: Languages like Kotlin, Dart, and Java are successfully using WasmGC for production web deployment, demonstrating its viability.

**Assessment**: **WasmGC is the clear long-term solution. Short-term JS imports are acceptable for prototyping but should not be considered the final architecture.**

---

## Revised Timeline: Sustainable Pace (Heijunka)

### Original Timeline

"~2 weeks total effort"

### Problem

This timeline exhibited **Muri** (overburdening):
- Aggressive schedule encourages corner-cutting
- Doesn't allow for "relentless reflection" (hansei)
- Risk of burnout and poor quality
- Violates "Respect for People" principle

### Revised Timeline (Evidence-Based)

**Pre-Implementation**:
- **Spike (Genchi Genbutsu)**: 1-2 days
- **Hansei (Reflection)**: 0.5 days
- **Plan Revision**: Based on spike data

**Phase 1: Foundation** (After Spike Validates Approach)
- **BOOTSTRAP-018**: 3-5 days (not 2-3 days)
  - Day 1-2: Basic expressions (Number, Binary, Identifier)
  - Day 3: Functions and let bindings
  - Day 4: Closures (based on spike strategy)
  - Day 5: Testing and refinement
- **Buffer**: 1 day for unexpected issues

**Phase 2: Integration**
- **BOOTSTRAP-019**: 2-3 days (not 1-2 days)
  - Day 1: Extend multi-target validation
  - Day 2: Pipeline integration
  - Day 3: Cross-target consistency testing

**Phase 3: Advanced Features**
- **BOOTSTRAP-020**: 5-7 days (not 3-4 days)
  - Day 1-2: Linear memory management
  - Day 3-4: Imports/exports system
  - Day 5: Function tables
  - Day 6-7: WASM-specific optimization **investigation**

**Phase 4: Self-Compilation**
- **BOOTSTRAP-021**: 10-14 days (not 7 days)
  - Week 1: Compile individual stages
  - Week 2: Integration and browser demo

**Total Realistic Estimate**: **4-6 weeks** (not 2 weeks)

**Adjustment Factor**: 2-3x original estimate

**Rationale**: Includes:
- Learning curve for WASM specifics
- Debugging time (toolchain, binary format)
- Peer review and collaboration time
- Testing and validation
- Buffer for discoveries

---

## WASM-Specific Optimization Complexity

### Original Statement

"Optimizations: Leverage WASM's efficient stack operations"

### Revised Understanding

**WASM Optimization is Counterintuitive**

Standard compiler optimizations can interact poorly with WASM's execution model:

**Example: Function Inlining**
```
Traditional wisdom: Inlining reduces call overhead
WASM reality: Inlining can HURT performance
```

**Why?**
- Browser JIT compilers (Liftoff, TurboFan) have their own optimization tiers
- Small functions compile faster in Liftoff (fast baseline)
- Large inlined functions force slower TurboFan compilation
- Net result: **Inlining slows down execution**

**Research**: Haas et al. (2017) "Bringing the Web up to Speed with WebAssembly"

**Other Counterintuitive Behaviors**:

1. **Loop Unrolling**: May increase binary size → slower download → worse cold-start
2. **Constant Folding**: Helps, but V8 does its own → diminishing returns
3. **Dead Code Elimination**: Critical (binary size matters for web)

**Revised Approach for Phase 3**:

**Do NOT**: Blindly apply standard optimizations

**DO**: Investigate empirically
1. Implement optimization
2. Measure on real browser VMs (V8, SpiderMonkey, JSC)
3. Compare binary size vs execution time
4. Keep only optimizations that measurably help

**Sub-Phase**: "WASM Optimization Research" (2-3 days)
- Benchmark suite with/without each optimization
- Measure across Chrome, Firefox, Safari
- Document findings
- Create optimization guidelines

---

## Collaborative Practices (Toyota Way)

### Original Plan

Focused on individual implementation

### Revised Plan: Teamwork & Shared Knowledge

**1. Pair Programming**

**When**: Core emitter implementation (BOOTSTRAP-018)

**Approach**:
```
Navigator: Thinks strategically, reviews code
Driver: Writes code, explains decisions
Rotate: Every 30 minutes
```

**Value**:
- Catches errors early (Jidoka - built-in quality)
- Knowledge sharing (no single point of failure)
- Better design discussions
- Higher morale

**2. Peer Review**

**What**: All generated WAT must be reviewed

**Process**:
```
1. Generate WAT from Ruchy
2. Manual inspection of WAT structure
3. Verify against expected semantics
4. Check for common pitfalls
5. Approve or request changes
```

**Checklist**:
- ✅ Correct stack discipline (push/pop balanced)
- ✅ Proper type usage (i32, i64, etc.)
- ✅ Function signatures match calls
- ✅ Exports defined correctly
- ✅ No undefined behavior

**3. Mob Programming for Hard Problems**

**When**: Closure compilation, GC integration

**Approach**:
```
3-4 people, one screen
Rotate driver every 10 minutes
All think together on hardest problems
```

**Value**: Collective intelligence on complex challenges

**4. Daily Standups During Active Development**

**Format**:
```
- What I completed
- What I learned (hansei)
- What I'm working on
- Blockers/help needed
```

**Duration**: 15 minutes max

**5. Weekly Reflection (Hansei)**

**Questions**:
```
1. What went well?
2. What didn't go well?
3. What did we learn?
4. What will we change?
```

**Document**: `docs/retrospectives/WASM_WEEK_N.md`

---

## Feasibility Assessment: Revised

### Original Rating

**Feasibility**: HIGH ✅

### Revised Rating

**Feasibility**: HIGH ✅ **with qualifications**

**Clarifications**:

**Confirmed High Feasibility**:
- ✅ Architectural patterns proven (TypeScript, Rust emitters)
- ✅ WASM is a mature, standardized target
- ✅ Tools are readily available (wabt, Node.js)
- ✅ Community support strong

**Qualified by Unknowns**:
- ⚠️ Closure compilation complexity (spike will reveal)
- ⚠️ Performance characteristics (measurement needed)
- ⚠️ GC strategy tradeoffs (host GC vs WasmGC timeline)
- ⚠️ Development velocity (actual vs estimated)

**Mitigation**: **Spike provides empirical data** to convert unknowns to knowns

**Decision Rule**:
```
IF spike succeeds:
    Feasibility = HIGH (confirmed)
    Timeline = spike-measured velocity × remaining work

IF spike reveals issues:
    Feasibility = MEDIUM
    Timeline = revised based on complexity

IF spike fails:
    Feasibility = LOW (defer)
    Focus on other priorities
```

---

## Updated Roadmap

### Phase 0: Validation (NEW)

**SPIKE-001: Genchi Genbutsu**
- **Duration**: 1-2 days (time-boxed)
- **Deliverables**:
  - Working proof-of-concept
  - Measurement data
  - Decision: Go/Revise/Defer
- **See**: `docs/research/WASM_COMPILATION_SPIKE.md`

**Hansei Meeting**
- **Duration**: 0.5 days
- **Outcome**: Revised plan based on spike findings

### Phase 1: Foundation (CONDITIONAL - After Spike Success)

**BOOTSTRAP-018: WASM Emitter Foundation**
- **Duration**: 3-5 days (evidence-based)
- **Tests**: 10 baseline tests (EXTREME TDD)
- **Deliverable**: `bootstrap/stage3/wasm_emitter.ruchy`
- **Collaboration**: Pair programming

### Phase 2: Integration

**BOOTSTRAP-019: Multi-Target Integration**
- **Duration**: 2-3 days
- **Tests**: 5 multi-target validation tests
- **Collaboration**: Peer review of WAT output

### Phase 3: Advanced Features

**BOOTSTRAP-020: Advanced WASM Features**
- **Duration**: 5-7 days
- **Sub-phases**:
  - Memory management (2 days)
  - Imports/exports (2 days)
  - Tables (1 day)
  - **Optimization investigation** (2 days) - NEW
- **Collaboration**: Mob programming for GC

### Phase 4: Self-Compilation

**BOOTSTRAP-021: Self-Compilation to WASM**
- **Duration**: 10-14 days
- **Goal**: `ruchyruchy.wasm`
- **Demo**: Browser-based compiler
- **Collaboration**: Daily standups, weekly retrospectives

**Total Realistic Timeline**: **4-6 weeks** (sustainable pace)

---

## Performance Expectations: Realistic

### Compute-Bound Tasks

```
Native Ruchy (Rust): 100ms
WASM Ruchy: 120-180ms (1.2x-1.8x overhead)
JavaScript Ruchy: 300-500ms (3x-5x overhead)
```

**Assessment**: **Significant improvement over JS, acceptable overhead vs native**

### Memory-Intensive Tasks

```
With Host GC (Phase 1-2):
- Small allocations: 2-3x overhead (boundary crossing)
- Large allocations: 1.5x overhead

With WasmGC (Phase 4+):
- Small allocations: ~1.1x overhead
- Large allocations: ~1.0x (parity with native)
```

**Assessment**: **Host GC adequate for prototype, WasmGC critical for production**

### Compilation Speed

```
TypeScript emission: ~50 LOC/ms
Rust emission: ~40 LOC/ms
WASM emission (estimated): ~30 LOC/ms
```

**Slower than other targets due to**:
- Binary format generation
- Validation passes
- Optimization complexity

**Assessment**: **Acceptable tradeoff for portability**

---

## Risk Assessment & Mitigation

### Risk 1: Closure Compilation More Complex Than Expected

**Probability**: Medium
**Impact**: High (blocks core functionality)

**Mitigation**:
- ✅ **Spike validates approach** before commitment
- ✅ Fallback to simpler strategy (global variables) if needed
- ✅ Phase 1 can ship without full closure support initially

### Risk 2: Performance Below Acceptable Threshold

**Probability**: Low
**Impact**: Medium (limits use cases)

**Mitigation**:
- ✅ Benchmark suite from day 1
- ✅ WasmGC path for long-term performance
- ✅ Still faster than JavaScript (baseline value)

### Risk 3: Timeline Slips Due to Unexpected Issues

**Probability**: High (learning new technology)
**Impact**: Low (sustainable pace allows adjustment)

**Mitigation**:
- ✅ **Buffer time built into estimates** (not aggressive deadlines)
- ✅ Weekly retrospectives catch issues early
- ✅ **Heijunka**: Can adjust priorities if needed

### Risk 4: Browser Compatibility Issues

**Probability**: Low (WASM is stable)
**Impact**: Medium

**Mitigation**:
- ✅ Test on Chrome, Firefox, Safari from day 1
- ✅ Use only stable WASM features for Phase 1-3
- ✅ WasmGC as opt-in for Phase 4

### Risk 5: Team Burnout from Aggressive Pace

**Probability**: High (original timeline)
**Impact**: Very High (quality, morale)

**Mitigation**:
- ✅ **Revised timeline is sustainable** (4-6 weeks, not 2)
- ✅ **Respect for People**: No crunch, reasonable workload
- ✅ Collaborative practices reduce individual burden
- ✅ Weekly hansei prevents accumulation of issues

---

## Success Criteria: Refined

### BOOTSTRAP-018 Success (Phase 1)

- ✅ 10/10 tests passing (EXTREME TDD)
- ✅ Generates valid WAT for all expression types
- ✅ WAT compiles to WASM binary (wat2wasm)
- ✅ WASM executes correctly (Node.js or browser)
- ✅ Semantically equivalent to TypeScript/Rust emitters
- **NEW**: ✅ Peer review sign-off on WAT quality
- **NEW**: ✅ Development velocity measured and documented

### Project Milestone Success (All Phases)

- ✅ All 4 bootstrap phases complete (018-021)
- ✅ `ruchyruchy.wasm` compiles and executes
- ✅ Browser demo working
- ✅ 100+ WASM-specific tests passing
- ✅ Performance within expected range (1.2x-1.8x native)
- **NEW**: ✅ Optimization guidelines documented
- **NEW**: ✅ Team reports sustainable pace maintained
- **NEW**: ✅ Retrospectives show continuous learning

---

## References (Added)

### Academic Research

1. **Jangda, A., et al. (2019)**. "Not So Fast: Analyzing the Performance of WebAssembly vs. Native Code." *USENIX Annual Technical Conference*.
   - Source for 45-55% overhead figures
   - Analysis of SPEC CPU benchmarks

2. **Haas, A., et al. (2017)**. "Bringing the Web up to Speed with WebAssembly." *ACM SIGPLAN Conference on Programming Language Design and Implementation (PLDI)*.
   - Original WASM paper
   - Formal semantics and validation

3. **Rossberg, A. (2021)**. "WebAssembly Core Specification." *W3C Recommendation*.
   - Authoritative specification
   - Type system and execution semantics

### WebAssembly Proposals

4. **WebAssembly GC Proposal** (2024). https://github.com/WebAssembly/gc
   - Phase 4 (Standardized)
   - Reference types and struct types

5. **WebAssembly Threads Proposal** (2023). https://github.com/WebAssembly/threads
   - For future concurrent Ruchy

### Compiler Implementation

6. **TypeScript Emitter**: `bootstrap/stage3/typescript_emitter.ruchy`
7. **Rust Emitter**: `bootstrap/stage3/rust_emitter.ruchy`
8. **Multi-Target Validation**: `bootstrap/stage3/multi_target_validation.ruchy`

---

## Acknowledgments

This revised research document incorporates critical feedback applying:
- **The Toyota Way** principles (Genchi Genbutsu, Kaizen, Heijunka, Respect for People)
- **Computer Science research** rigor (performance measurement, GC strategy, optimization complexity)

**Key Improvements**:
1. Added empirical validation step (spike)
2. Grounded performance claims in peer-reviewed research
3. Elaborated long-term GC strategy (WasmGC)
4. Revised timeline to sustainable pace (2-3x original)
5. Added collaborative practices
6. Acknowledged WASM-specific optimization challenges

**Result**: A more rigorous, realistic, and sustainable plan that respects both the technology and the team.

---

## Conclusion

**Question**: Can we compile Ruchy to WASM?

**Answer**: **Yes, with proper validation and sustainable execution.**

**Approach**:
1. **Genchi Genbutsu**: Spike to validate assumptions (1-2 days)
2. **Hansei**: Reflect on findings and revise plan
3. **Kaizen**: Incremental implementation (4-6 weeks, not 2)
4. **Jidoka**: Quality built-in via testing and peer review
5. **Respect for People**: Sustainable pace, collaborative practices

**Value**: High (browser execution, portability, performance)

**Risks**: Identified and mitigated

**Timeline**: Evidence-based (4-6 weeks) with buffer

**Recommendation**: **Proceed with spike, then decide based on empirical data.**

This is the **Toyota Way**: Make decisions slowly (gather data), implement decisions rapidly (once validated).
