# RuchyRuchy WASM Specification

**Document Type**: Implementation Specification
**Status**: Ready for Implementation (Pending Spike Validation)
**Version**: 1.0
**Date**: 2025-10-22
**Methodology**: EXTREME TDD (RED-GREEN-REFACTOR-TOOL)

---

## Executive Summary

This specification defines the implementation of WebAssembly (WASM) as a compilation target for the RuchyRuchy bootstrap compiler. WASM adds a third backend alongside TypeScript and Rust, enabling browser-based execution and universal portability.

**Core Purpose**: Validate and stress-test `../ruchy`'s WASM code generation through self-compilation and differential testing.

**Key Deliverables**:
1. `bootstrap/stage3/emit_wasm.ruchy` - WASM emitter (alongside TypeScript/Rust)
2. `ruchyruchy.wasm` - Self-compiled bootstrap compiler running in browsers
3. WASM debugger (12 features matching Phase 4 debugger)
4. Performance optimization framework for WASM targets
5. Comprehensive validation suite (property tests, fuzz tests, boundary analysis)

**Timeline**: 4-6 weeks (sustainable pace following Heijunka)

**Prerequisite**: ../ruchy must implement WASM backend first

**Methodology**: EXTREME TDD at every step (RED-GREEN-REFACTOR-TOOL)

---

## Mission & Strategic Value

### Primary Mission

**Validation & Bug Discovery for ../ruchy WASM Backend**

RuchyRuchy serves as a rigorous quality assurance tool through:
1. **Self-compilation stress testing**: Compiling complex compiler code to WASM
2. **Differential testing**: Compare WASM output vs TypeScript/Rust outputs
3. **Edge case discovery**: Find bugs through property testing and fuzzing
4. **Dogfooding excellence**: Extensive use of Ruchy tooling exposes issues

### Value Proposition

**Two-Way Benefit**:
- **../ruchy benefits**: Gets battle-tested WASM backend with bug reports
- **ruchyruchy benefits**: Gains browser execution, universal portability

**Feedback Loop**:
```
../ruchy implements WASM backend
    ↓
ruchyruchy compiles to WASM (validation)
    ↓
Discovers bugs/edge cases
    ↓
Files GitHub issues
    ↓
../ruchy improves WASM backend
    ↓
Cycle repeats
```

### Secondary Benefits

1. **Educational**: Shows WASM compilation internals
2. **Portability**: Run compiler in browsers, edge compute, serverless
3. **Demo Platform**: Interactive compiler demos in web browsers
4. **Research**: WASM optimization patterns for compiler workloads

---

## Architecture Overview

### Multi-Target Code Generation

```
Ruchy Source Code
    ↓
Bootstrap Compiler (Stage 0-2)
    ↓
Code Generation (Stage 3) → Branching:
    ├── emit_typescript.ruchy → .ts files
    ├── emit_rust.ruchy → .rs files
    └── emit_wasm.ruchy → .wat → .wasm files
```

### WASM Emitter Location

**File**: `bootstrap/stage3/emit_wasm.ruchy`

**Parallel Structure**:
- `bootstrap/stage3/emit_typescript.ruchy` (exists)
- `bootstrap/stage3/emit_rust.ruchy` (exists)
- `bootstrap/stage3/emit_wasm.ruchy` (NEW)

### Integration Points

1. **Build Pipeline**:
   ```bash
   make stage3-wasm        # Compile bootstrap to WASM
   make test-wasm          # Run WASM-specific tests
   make validate-wasm      # Differential testing
   ```

2. **Multi-Target Validation**:
   - Extend `bootstrap/stage3/multi_target_validation.ruchy`
   - Add WASM to comparison matrix
   - Verify semantic equivalence across all 3 targets

3. **Quality Gates**:
   - All existing quality gates apply to WASM emitter
   - EXTREME TDD: RED-GREEN-REFACTOR-TOOL for every feature
   - `ruchy lint`, `ruchy test`, `ruchy prove`, `ruchy score`

---

## Performance Expectations

### Compilation Speed (Ruchy→WASM)

**Measured via INFRA-001/002/003**:
```
TypeScript emission: ~50 LOC/ms
Rust emission: ~40 LOC/ms
WASM emission (target): ~30 LOC/ms
```

**Slower due to**:
- Binary format generation (not text)
- Validation passes
- Optimization complexity

**Acceptable tradeoff**: Portability justifies slower emission

### Runtime Performance (WASM Execution)

**Evidence-Based Expectations** (grounded in CS research):

**Compute-Bound Tasks**:
```
Native Rust: 100ms (baseline)
WASM: 120-180ms (1.2x-1.8x overhead)
TypeScript/JS: 300-500ms (3x-5x overhead)
```

**Source**: USENIX ATC 2019 - "Not So Fast: Analyzing the Performance of WebAssembly vs. Native Code"

**Memory-Intensive Tasks**:
```
Phase 1-2 (Host GC via JS imports):
  Small allocations: 2-3x overhead (boundary crossing)
  Large allocations: 1.5x overhead

Phase 4+ (WasmGC):
  Small allocations: ~1.1x overhead
  Large allocations: ~1.0x (parity)
```

**Assessment**:
- ✅ Significant improvement over JavaScript
- ✅ Acceptable overhead vs native for web deployment
- ✅ WasmGC closes performance gap long-term

### Performance Optimization Framework

**Apply INFRA-001/002/003 to WASM emitter**:

1. **Benchmark WASM emission speed** (N=30 runs)
2. **Measure WASM execution performance** (vs TypeScript/Rust)
3. **Statistical validation** (t-tests, confidence intervals)
4. **Find optimization opportunities** through measurement

**WASM-Specific Considerations**:
- Function inlining can HURT (JIT tier tradeoffs)
- Binary size affects cold-start performance
- Dead code elimination critical for web
- Empirical measurement required (don't assume)

---

## Implementation Roadmap

### Phase 0: Validation (MANDATORY - Genchi Genbutsu)

**Purpose**: Empirical validation before full commitment

#### WASM-SPIKE-001: Closure Compilation Validation (1-2 days)

**Objective**: Validate hardest problem (closures with captured variables)

**Test Program**:
```ruchy
fun make_adder(n) {
    (x) => x + n  // n is captured
}

let add5 = make_adder(5)
add5(3)  // Should return 8
```

**Deliverables**:
1. Working WAT output for closure example
2. Compiled .wasm binary that executes correctly
3. Measurement data:
   - Time to implement: __ hours
   - Lines of code: __ LOC
   - Issues discovered: __ count
   - Extrapolated timeline: __ weeks

**Success Criteria**:
- ✅ Closure compiles to valid WAT
- ✅ WASM executes with correct output (8)
- ✅ Clear closure strategy (closure records OR globals)
- ✅ Confidence in approach

**Failure Criteria** (valuable learning):
- ❌ >12 hours without working implementation
- ❌ Fundamental WASM limitation discovered
- ❌ Complexity far exceeds expectations

**EXTREME TDD**: RED-GREEN-REFACTOR-TOOL for spike

**See**: `docs/research/WASM_COMPILATION_SPIKE.md` for full plan

#### WASM-SPIKE-002: Hansei (Reflection) (0.5 days)

**Questions**:
1. What did we learn?
2. What surprised us?
3. What should we do differently?
4. Do we proceed? (Go/Revise/Defer)
5. What's the realistic timeline?

**Deliverable**: `spike/REFLECTION.md`

**Decision Framework**:
```
IF spike succeeds:
    → Proceed to Phase 1
    → Use measured velocity for estimates

IF spike reveals solvable issues:
    → Revise approach
    → Adjust timeline
    → Proceed with caution

IF spike fails:
    → Defer WASM implementation
    → Document findings
    → Focus on other priorities
    → Revisit when blockers resolved
```

---

### Phase 1: Foundation (After Spike Validates)

#### WASM-001: Basic Expression Emitter (3-5 days)

**Objective**: Emit WAT for core expressions

**EXTREME TDD Phases**:

**RED Phase**: Write 10 failing tests
```ruchy
// test_wasm_emitter_red.ruchy
fun test_emit_number() {
    let wat = emit_wasm_expr(NumberLiteral(42))
    assert(wat_contains(wat, "(i32.const 42)"))
}

fun test_emit_binary_add() {
    let ast = BinaryOp(Plus, NumberLiteral(2), NumberLiteral(3))
    let wat = emit_wasm_expr(ast)
    assert(wat_contains(wat, "i32.add"))
}

// ... 8 more tests ...
```

**Expected**: 0/10 tests passing (demonstrates need)

**GREEN Phase**: Minimal implementation
```ruchy
// bootstrap/stage3/emit_wasm.ruchy
fun emit_wasm_expr(expr: Expr) -> String {
    match expr {
        NumberLiteral(n) => "(i32.const " + int_to_string(n) + ")",
        BinaryOp(Plus, left, right) => {
            emit_wasm_expr(left) + " " +
            emit_wasm_expr(right) + " " +
            "(i32.add)"
        },
        Identifier(name) => "(local.get $" + name + ")",
        // ... more cases ...
    }
}

fun emit_wasm_function(func: FunctionDecl) -> String {
    // Function wrapper
}

fun emit_wasm_module(program: Program) -> String {
    // Module wrapper
}
```

**Expected**: 10/10 tests passing (minimal)

**REFACTOR Phase**: Improve quality
- Better structure (helper functions)
- Comprehensive comments
- Edge case handling
- Performance considerations

**TOOL Phase**: Quality validation
```bash
ruchy check bootstrap/stage3/emit_wasm.ruchy
ruchy lint bootstrap/stage3/emit_wasm.ruchy  # A+ required
ruchy test validation/wasm/test_wasm_emitter_*.ruchy
ruchy score bootstrap/stage3/emit_wasm.ruchy  # >0.8
```

**Deliverables**:
- `bootstrap/stage3/emit_wasm.ruchy` (~200-300 LOC)
- `validation/wasm/test_wasm_emitter_red.ruchy`
- `validation/wasm/test_wasm_emitter_green.ruchy`
- `validation/wasm/test_wasm_emitter_refactor.ruchy`
- `book/src/phase5_wasm/wasm-001-basic-emitter.md`

**Success Criteria**:
- ✅ 10/10 tests passing
- ✅ Emits valid WAT (verified with wat2wasm)
- ✅ WASM executes correctly (Node.js)
- ✅ A+ lint grade
- ✅ >0.8 quality score

---

#### WASM-002: Closure Compilation (2-3 days)

**Objective**: Implement production closure strategy (validated by spike)

**EXTREME TDD**: RED-GREEN-REFACTOR-TOOL

**Strategy Options** (chose based on spike):

**Option A: Closure Records in Linear Memory**
```wat
(func $make_adder (param $n i32) (result i32)
  (local $closure_addr i32)
  ;; Allocate closure struct: [func_idx, captured_n]
  (local.set $closure_addr (call $malloc (i32.const 8)))
  (i32.store (local.get $closure_addr) (i32.const 0))
  (i32.store
    (i32.add (local.get $closure_addr) (i32.const 4))
    (local.get $n))
  (local.get $closure_addr))

(func $lambda_0 (param $closure_ptr i32) (param $x i32) (result i32)
  (local $n i32)
  (local.set $n
    (i32.load (i32.add (local.get $closure_ptr) (i32.const 4))))
  (i32.add (local.get $x) (local.get $n)))
```

**Option B: Global Variables** (simpler, less pure)
```wat
(global $captured_n (mut i32) (i32.const 0))

(func $make_adder (param $n i32) (result i32)
  (global.set $captured_n (local.get $n))
  (i32.const 0))

(func $lambda_0 (param $x i32) (result i32)
  (i32.add (local.get $x) (global.get $captured_n)))
```

**Tests**: 8 tests covering single/multiple capture, nested closures

**Deliverables**:
- Closure emission code (~150 LOC)
- Test suite (RED-GREEN-REFACTOR)
- Book chapter

---

#### WASM-003: Multi-Target Integration (2-3 days)

**Objective**: Integrate WASM into multi-target validation framework

**EXTREME TDD**: RED-GREEN-REFACTOR-TOOL

**Extend Multi-Target Validation**:
```ruchy
// bootstrap/stage3/multi_target_validation.ruchy
fun test_semantic_equivalence() {
    let source = "fun add(x, y) { x + y }"

    let ts_output = emit_typescript(parse(source))
    let rust_output = emit_rust(parse(source))
    let wasm_output = emit_wasm(parse(source))  // NEW

    // All three should produce equivalent behavior
    assert(execute_typescript(ts_output, [2, 3]) == 5)
    assert(execute_rust(rust_output, [2, 3]) == 5)
    assert(execute_wasm(wasm_output, [2, 3]) == 5)  // NEW
}
```

**Differential Testing**:
- Compare output across all 3 targets
- Property: Semantic equivalence
- Run on entire bootstrap codebase

**Tests**: 5 multi-target validation tests

**Deliverables**:
- Extended multi-target validation (~100 LOC)
- Differential test suite
- Book chapter

---

### Phase 2: Advanced Features

#### WASM-004: Linear Memory Management (2 days)

**Objective**: Heap allocation, strings, arrays

**EXTREME TDD**: RED-GREEN-REFACTOR-TOOL

**Implementation**:
```wat
;; Memory management
(memory 1)  ;; 64KB initial

(global $heap_ptr (mut i32) (i32.const 0))

(func $malloc (param $size i32) (result i32)
  (local $addr i32)
  (local.set $addr (global.get $heap_ptr))
  (global.set $heap_ptr
    (i32.add (global.get $heap_ptr) (local.get $size)))
  (local.get $addr))

(func $alloc_string (param $len i32) (result i32)
  (call $malloc (i32.add (local.get $len) (i32.const 4))))
```

**Tests**: 6 tests (malloc, strings, arrays)

**Deliverables**:
- Memory management code (~100 LOC)
- Test suite
- Book chapter

---

#### WASM-005: Imports & Exports (2 days)

**Objective**: Host GC integration, JavaScript interop

**EXTREME TDD**: RED-GREEN-REFACTOR-TOOL

**Host GC Strategy** (Phase 1-2):
```wat
;; Import JavaScript GC
(import "js" "alloc" (func $js_alloc (param i32) (result i32)))
(import "js" "free" (func $js_free (param i32)))

;; Export functions
(export "add" (func $add))
(export "compile" (func $compile))
```

**JavaScript Integration**:
```javascript
const wasmImports = {
  js: {
    alloc: (size) => {
      const buffer = new ArrayBuffer(size);
      return heap.add(buffer);
    },
    free: (ptr) => {
      heap.delete(ptr);
    }
  }
};

WebAssembly.instantiate(wasmBytes, wasmImports);
```

**Tests**: 5 tests (imports, exports, JS interop)

**Deliverables**:
- Import/export code (~80 LOC)
- JavaScript bridge code
- Test suite
- Book chapter

---

#### WASM-006: Function Tables (1 day)

**Objective**: Indirect calls for closures

**EXTREME TDD**: RED-GREEN-REFACTOR-TOOL

**Implementation**:
```wat
;; Function table for closures
(table 100 funcref)

(elem (i32.const 0) $lambda_0 $lambda_1 $lambda_2)

(func $call_closure (param $idx i32) (param $arg i32) (result i32)
  (call_indirect (type $func_type)
    (local.get $arg)
    (local.get $idx)))
```

**Tests**: 4 tests (table creation, indirect calls)

**Deliverables**:
- Function table code (~60 LOC)
- Test suite
- Book chapter

---

#### WASM-007: Performance Optimization Investigation (2-3 days)

**Objective**: Empirical optimization research

**CAUTION**: WASM optimization is counterintuitive

**Research Questions**:
1. Does function inlining help or hurt?
2. Does loop unrolling improve performance?
3. What's the optimal function size for JIT tiers?
4. How does binary size affect cold-start?

**Methodology**:
1. Implement optimization
2. Benchmark on real browsers (Chrome, Firefox, Safari)
3. Measure binary size vs execution time
4. Keep only optimizations that measurably help

**Tests**: Benchmark suite comparing optimizations

**Deliverables**:
- `docs/research/WASM_OPTIMIZATION_FINDINGS.md`
- Optimization guidelines
- Benchmark results

---

### Phase 3: Self-Compilation

#### WASM-008: Stage 0 Self-Compilation (3 days)

**Objective**: Compile lexer to WASM

**EXTREME TDD**: RED-GREEN-REFACTOR-TOOL

**Validation**:
```bash
# Compile lexer to WASM
ruchy compile bootstrap/stage0/lexer.ruchy --target wasm

# Test self-tokenization
node -e "
  const fs = require('fs');
  const wasm = fs.readFileSync('lexer.wasm');
  const instance = await WebAssembly.instantiate(wasm);
  const tokens = instance.exports.tokenize('fun add(x, y) { x + y }');
  console.log(tokens);
"
```

**Tests**: 10 self-compilation tests

**Deliverables**:
- Stage 0 WASM compilation
- Self-tokenization validation
- Book chapter

---

#### WASM-009: Stage 1 Self-Compilation (3 days)

**Objective**: Compile parser to WASM

**EXTREME TDD**: RED-GREEN-REFACTOR-TOOL

**Validation**: Parser parses itself in browser

**Tests**: 10 self-parsing tests

**Deliverables**:
- Stage 1 WASM compilation
- Self-parsing validation
- Book chapter

---

#### WASM-010: Stage 2 Self-Compilation (3 days)

**Objective**: Compile type checker to WASM

**EXTREME TDD**: RED-GREEN-REFACTOR-TOOL

**Validation**: Type checker types itself in browser

**Tests**: 10 self-typing tests

**Deliverables**:
- Stage 2 WASM compilation
- Self-typing validation
- Book chapter

---

#### WASM-011: Stage 3 Self-Compilation (4 days)

**Objective**: Complete self-compilation - `ruchyruchy.wasm`

**EXTREME TDD**: RED-GREEN-REFACTOR-TOOL

**Validation**:
```bash
# Compile entire bootstrap to WASM
ruchy compile bootstrap/ --target wasm -o ruchyruchy.wasm

# Test in browser
open demo/browser-compiler.html
```

**Browser Demo**:
```html
<!DOCTYPE html>
<html>
<body>
  <textarea id="source">fun add(x, y) { x + y }</textarea>
  <button onclick="compile()">Compile</button>
  <pre id="output"></pre>

  <script>
    let compiler;

    WebAssembly.instantiateStreaming(fetch('ruchyruchy.wasm'))
      .then(instance => {
        compiler = instance.exports;
        console.log('Compiler loaded!');
      });

    function compile() {
      const source = document.getElementById('source').value;
      const output = compiler.compile(source);
      document.getElementById('output').textContent = output;
    }
  </script>
</body>
</html>
```

**Tests**: 15 full self-compilation tests

**Deliverables**:
- Complete `ruchyruchy.wasm`
- Browser demo
- GitHub Pages deployment
- Book chapter

---

### Phase 4: WASM Debugger

#### WASM-DBG-001 to WASM-DBG-012: Full Debugger Suite

**Objective**: Replicate Phase 4 debugger for WASM targets

**12 Features** (matching Phase 4):
1. Basic breakpoints
2. Step execution
3. Variable inspection
4. Expression evaluation
5. Watch expressions
6. Conditional breakpoints
7. Source mapping (Ruchy line ↔ WASM instruction)
8. Memory visualization
9. Time-travel debugging
10. Stack unwinding
11. Call stack visualization
12. Interactive REPL debugging

**Architecture**:
```
Ruchy Source
    ↓
Compile with debug info
    ↓
WASM + Source Maps
    ↓
Browser DevTools Integration
    ↓
Debug Ruchy at source level
```

**Each Feature**: EXTREME TDD (RED-GREEN-REFACTOR-TOOL)

**Timeline**: 4-6 weeks (parallel to Phase 4 debugger)

**Value**:
- Tests ../ruchy's WASM source maps
- Validates WASM execution semantics
- Enables debugging Ruchy-in-browser
- Educational: shows WASM debugging internals

**Deliverables**:
- 12 debugger features for WASM
- Browser-based debugger UI
- Integration with Chrome DevTools
- Book chapters (12)

---

### Phase 5: Performance Optimization

#### WASM-PERF-001: Compilation Speed Optimization (1 week)

**Objective**: Optimize WASM emission speed using INFRA-001/002/003

**EXTREME TDD**: RED-GREEN-REFACTOR-TOOL

**Methodology**:
1. Baseline measurements (N=30)
2. Identify bottlenecks via profiling
3. Implement optimizations
4. Statistical validation (t-tests)
5. Iterate

**Target**: ~30 LOC/ms emission speed

**Tests**: Benchmark suite (N=30 statistical rigor)

**Deliverables**:
- Optimized emitter
- Performance report
- Book chapter

---

#### WASM-PERF-002: Runtime Performance Optimization (1 week)

**Objective**: Optimize generated WASM code execution

**EXTREME TDD**: RED-GREEN-REFACTOR-TOOL

**Apply Optimization Findings** (from WASM-007):
- Dead code elimination
- Selective inlining (measured)
- Binary size reduction
- Memory layout optimization

**Target**: Achieve 1.2x-1.8x native overhead

**Tests**: Execution benchmarks (N=30)

**Deliverables**:
- Optimized codegen
- Performance report
- Book chapter

---

#### WASM-PERF-003: WasmGC Migration (2 weeks - FUTURE)

**Objective**: Migrate from host GC to WasmGC

**Status**: Future work (when browser support universal)

**Implementation**:
```wat
;; WasmGC types
(type $string (struct (field (mut i32)) (field (mut (ref $byte_array)))))
(type $closure (struct (field (ref func)) (field (ref any))))

;; Native allocation
(struct.new $string (i32.const 10) (ref.null $byte_array))
```

**Benefits**:
- 10-100x faster allocation
- Zero boundary crossing overhead
- Smaller binaries
- Better JIT optimization

**Timeline**: When Safari ships WasmGC (likely 2025-2026)

---

## Validation Strategy

### Property Testing

**Apply to WASM emitter**:

```ruchy
// validation/property/test_wasm_roundtrip.ruchy
fun property_wasm_semantic_equivalence() {
    forall ruchy_program in valid_programs() {
        let ts_result = compile_and_run_typescript(ruchy_program)
        let wasm_result = compile_and_run_wasm(ruchy_program)

        assert(ts_result == wasm_result)
    }
}
```

**Properties**:
1. Semantic equivalence (WASM ≡ TypeScript ≡ Rust)
2. Type preservation (well-typed Ruchy → well-typed WAT)
3. Memory safety (no out-of-bounds, no leaks)

**Run**: `ruchy prove validation/property/test_wasm_*.ruchy`

---

### Fuzz Testing

**Apply to WASM emitter**:

```ruchy
// validation/fuzz/test_wasm_fuzzer.ruchy
fun fuzz_wasm_emitter() {
    let mut crash_count = 0
    let mut i = 0

    while i < 1000000 {
        let random_ast = generate_random_ast()

        let result = emit_wasm(random_ast)

        match result {
            Ok(wat) => {
                // Validate WAT compiles
                let compiled = wat_to_wasm(wat)
                assert(compiled.is_valid())
            },
            Err(e) => {
                // Expected for invalid ASTs
            }
        }

        i = i + 1
    }

    assert(crash_count == 0)
}
```

**Run**: `ruchy test validation/fuzz/test_wasm_fuzzer.ruchy`

**Target**: 1M+ inputs, zero crashes

---

### Boundary Testing

**Find limits of WASM implementation**:

```ruchy
// validation/boundary/test_wasm_limits.ruchy
fun test_max_function_depth() {
    // Find maximum nested function depth
}

fun test_max_closure_captures() {
    // Find maximum captured variables
}

fun test_max_memory_allocation() {
    // Find heap limit
}
```

**Document**: `BOUNDARIES.md` (WASM section)

**Run**: `ruchy runtime validation/boundary/test_wasm_limits.ruchy`

---

### Differential Testing

**Compare WASM vs TypeScript vs Rust**:

```bash
# Run same program on all 3 targets
ruchy compile program.ruchy --target typescript -o out.ts
ruchy compile program.ruchy --target rust -o out.rs
ruchy compile program.ruchy --target wasm -o out.wasm

# Execute and compare
node out.ts > ts_output.txt
cargo run --bin out > rust_output.txt
node wasm_runner.js > wasm_output.txt

# Verify equivalence
diff ts_output.txt rust_output.txt
diff ts_output.txt wasm_output.txt
```

**Automated**: `make test-differential-wasm`

---

## Bug Discovery Protocol

### When WASM Issues Discovered

**MANDATORY - File GitHub Issue**:

```markdown
## Bug Report: WASM Code Generation Issue

**Ruchy Version**: [output of `ruchy --version`]
**Project**: RuchyRuchy WASM Backend
**Ticket**: WASM-XXX

### Reproduction Steps
1. Compile attached Ruchy program to WASM
2. Run in Node.js/browser
3. Observe incorrect behavior

### Minimal Reproduction Code
```ruchy
fun make_counter() {
    let mut count = 0
    (x) => {
        count = count + x
        count
    }
}
```

### Expected Behavior
Should return closure that increments counter

### Actual Behavior
[Error message or incorrect output]

### Generated WAT
```wat
[Paste generated WAT that demonstrates issue]
```

### Impact
Blocks WASM-002 (closure compilation)

### Context
Found during differential testing - TypeScript/Rust backends work correctly

### Upstream Issue
Filed at: https://github.com/paiml/ruchy/issues/XXX
```

**After Filing**:
1. Document in `BOUNDARIES.md`
2. Implement workaround if possible
3. Continue with alternative approach
4. Reference issue in commits

---

## Quality Gates

### MANDATORY for ALL WASM Code

```bash
# Syntax validation
ruchy check bootstrap/stage3/emit_wasm.ruchy

# A+ lint grade required
ruchy lint bootstrap/stage3/emit_wasm.ruchy

# All tests must pass
ruchy test validation/wasm/test_*.ruchy

# Property verification
ruchy prove validation/property/test_wasm_*.ruchy

# Quality score >0.8
ruchy score bootstrap/stage3/emit_wasm.ruchy

# Performance validation
ruchy runtime validation/benchmarks/test_wasm_perf_*.ruchy
```

**Pre-commit hooks enforce these gates** (BLOCKING)

---

## Book Documentation

### MANDATORY for Every Ticket

**Structure**:
```
book/src/phase5_wasm/
├── chapter.md                          # Phase overview
├── wasm-spike-001-validation.md        # Spike results
├── wasm-001-basic-emitter.md           # EXTREME TDD
├── wasm-002-closures.md                # EXTREME TDD
├── wasm-003-integration.md             # EXTREME TDD
├── ...
├── wasm-011-self-compilation.md        # EXTREME TDD
└── wasm-dbg-001-to-012/               # Debugger chapters
```

**Each Chapter Template**:
```markdown
# WASM-XXX: Feature Name

## Context
[Why needed, what problem solves]

## RED: Write Failing Tests
[Tests written first]
```ruchy
// Test code
```
Expected: X/Y tests failing

## GREEN: Minimal Implementation
[Code that makes tests pass]
```ruchy
// Implementation
```
Result: ✅ Y/Y tests passing

## REFACTOR: Improvements
[Quality improvements]

## TOOL: Validation
- ruchy check: ✅
- ruchy lint: ✅ A+
- ruchy test: ✅ Y/Y passing
- ruchy score: ✅ 0.XX

## Discoveries
[Bugs found, issues filed, learnings]

## Next Steps
[What this enables]
```

**GitHub Pages**: Auto-publish via GitHub Actions

---

## Timeline Summary

### Phase 0: Validation
- **WASM-SPIKE-001**: 1-2 days (time-boxed)
- **Hansei**: 0.5 days
- **Total**: 1.5-2.5 days

### Phase 1: Foundation
- **WASM-001**: 3-5 days (basic emitter)
- **WASM-002**: 2-3 days (closures)
- **WASM-003**: 2-3 days (integration)
- **Total**: 7-11 days

### Phase 2: Advanced Features
- **WASM-004**: 2 days (memory)
- **WASM-005**: 2 days (imports/exports)
- **WASM-006**: 1 day (function tables)
- **WASM-007**: 2-3 days (optimization research)
- **Total**: 7-8 days

### Phase 3: Self-Compilation
- **WASM-008**: 3 days (Stage 0)
- **WASM-009**: 3 days (Stage 1)
- **WASM-010**: 3 days (Stage 2)
- **WASM-011**: 4 days (Stage 3 + demo)
- **Total**: 13 days

### Phase 4: WASM Debugger
- **WASM-DBG-001 to 012**: 4-6 weeks (parallel effort)

### Phase 5: Performance
- **WASM-PERF-001**: 1 week (emission speed)
- **WASM-PERF-002**: 1 week (execution speed)
- **WASM-PERF-003**: 2 weeks (WasmGC - future)

**Core WASM Implementation (Phases 1-3)**: 4-5 weeks
**Full WASM Support (Including Debugger)**: 8-12 weeks
**Performance Optimization**: +2-4 weeks

**Total Realistic Timeline**: 10-16 weeks for complete WASM platform

---

## Success Criteria

### Phase 1 Success (Foundation)
- ✅ 23 tests passing (WASM-001/002/003)
- ✅ Generates valid WAT for expressions, functions, closures
- ✅ WAT compiles to .wasm (wat2wasm)
- ✅ WASM executes correctly (Node.js/browser)
- ✅ Semantically equivalent to TypeScript/Rust
- ✅ A+ lint grade, >0.8 quality score

### Phase 3 Success (Self-Compilation)
- ✅ `ruchyruchy.wasm` compiled and executing
- ✅ Browser demo working
- ✅ Self-compilation tests passing
- ✅ Differential testing shows equivalence

### Phase 4 Success (Debugger)
- ✅ All 12 debugger features working for WASM
- ✅ Source maps accurate (Ruchy ↔ WASM)
- ✅ Browser DevTools integration
- ✅ Time-travel debugging functional

### Phase 5 Success (Performance)
- ✅ Emission speed: ~30 LOC/ms
- ✅ Execution speed: 1.2x-1.8x native overhead
- ✅ Statistical validation (N=30, p<0.05)
- ✅ Optimization guidelines documented

### Overall Project Success
- ✅ All phases complete
- ✅ 100+ WASM-specific tests passing
- ✅ Zero crashes from fuzz testing (1M+ inputs)
- ✅ Complete boundary documentation
- ✅ 80%+ test coverage
- ✅ Book published to GitHub Pages
- ✅ Multiple bugs filed for ../ruchy (validation mission)

---

## Risk Assessment

### Risk 1: Closure Compilation Complexity
**Probability**: Medium
**Impact**: High
**Mitigation**: ✅ Spike validates approach first

### Risk 2: Performance Below Acceptable
**Probability**: Low
**Impact**: Medium
**Mitigation**: ✅ Benchmarking from day 1, WasmGC long-term

### Risk 3: Timeline Slips
**Probability**: Medium
**Impact**: Low
**Mitigation**: ✅ Sustainable pace, buffer built-in, weekly retrospectives

### Risk 4: Browser Compatibility
**Probability**: Low
**Impact**: Medium
**Mitigation**: ✅ Test Chrome/Firefox/Safari from start, stable WASM features only

### Risk 5: Team Burnout
**Probability**: Low (with revised timeline)
**Impact**: Very High
**Mitigation**: ✅ Sustainable 4-6 week pace, collaborative practices, Heijunka

---

## Dependencies

### Upstream Dependency: ../ruchy WASM Backend

**RuchyRuchy WASM implementation depends on**:
1. ../ruchy implements WASM code generation first
2. ../ruchy provides stable WASM API
3. ../ruchy WASM backend passes basic validation

**Coordination**:
- File GitHub issue requesting WASM backend: https://github.com/paiml/ruchy/issues
- Track upstream progress
- Begin spike when ../ruchy WASM is available
- Provide feedback/bug reports during implementation

**Fallback**: If ../ruchy WASM delayed, focus on other priorities (optimization, validation)

### Tool Dependencies

**Required**:
- `wat2wasm` - WAT to WASM compiler (from WABT)
- `wasm-interp` - WASM interpreter (from WABT)
- `wasm-objdump` - Binary inspection
- Node.js or browser for execution

**Install**:
```bash
# WABT (WebAssembly Binary Toolkit)
cargo install wabt

# Or via package manager
apt-get install wabt
brew install wabt
```

---

## References

### Academic Research
1. **Jangda, A., et al. (2019)**. "Not So Fast: Analyzing the Performance of WebAssembly vs. Native Code." USENIX ATC.
2. **Haas, A., et al. (2017)**. "Bringing the Web up to Speed with WebAssembly." PLDI.
3. **Rossberg, A. (2021)**. "WebAssembly Core Specification." W3C Recommendation.

### WebAssembly Resources
4. **WebAssembly GC Proposal**: https://github.com/WebAssembly/gc
5. **WebAssembly Specification**: https://webassembly.github.io/spec/
6. **MDN WebAssembly Guide**: https://developer.mozilla.org/en-US/docs/WebAssembly

### Project Resources
7. **WASM Compilation Spike**: `docs/research/WASM_COMPILATION_SPIKE.md`
8. **WASM Research (Revised)**: `docs/research/WASM_COMPILATION_TARGET_REVISED.md`
9. **Optimization Infrastructure**: INFRA-001/002/003
10. **Phase 4 Debugger**: 12 features (template for WASM debugger)

---

## Collaborative Practices

### Pair Programming
**When**: Core emitter implementation (WASM-001/002)
**Approach**: Rotate driver/navigator every 30 minutes
**Value**: Catches errors early, knowledge sharing

### Peer Review
**What**: All generated WAT reviewed manually
**Checklist**: Stack discipline, type usage, function signatures, exports

### Mob Programming
**When**: Hardest problems (closures, GC integration)
**Approach**: 3-4 people, rotate driver every 10 minutes

### Daily Standups
**During**: Active development periods
**Format**: Completed, learned, working on, blockers (15 min max)

### Weekly Retrospectives (Hansei)
**Questions**: What went well? What didn't? What learned? What change?
**Document**: `docs/retrospectives/WASM_WEEK_N.md`

---

## Conclusion

**Question**: Can RuchyRuchy compile to WASM?

**Answer**: **Yes, with proper validation and sustainable execution.**

**Approach**:
1. **Genchi Genbutsu**: Spike validates assumptions (1-2 days)
2. **Hansei**: Reflect on findings and revise plan
3. **Kaizen**: Incremental implementation (4-6 weeks core, 10-16 weeks full)
4. **Jidoka**: Quality built-in via EXTREME TDD
5. **Respect for People**: Sustainable pace, collaborative practices

**Value Proposition**:
- **Validation**: Stress-test ../ruchy WASM backend
- **Bug Discovery**: Find edge cases through differential/property/fuzz testing
- **Education**: Show WASM compilation internals
- **Portability**: Browser execution, universal deployment
- **Debugger**: Full debugging in browsers
- **Performance**: Optimize both emission and execution

**Strategic Position**: RuchyRuchy becomes the quality assurance tool for ../ruchy's WASM backend while gaining valuable capabilities.

**This is the Toyota Way**: Validate slowly (spike), implement carefully (EXTREME TDD), deliver sustainably (Heijunka).

---

**Status**: Ready for implementation pending:
1. ../ruchy WASM backend availability
2. Spike validation (WASM-SPIKE-001)
3. Hansei approval to proceed

**Next Action**: File GitHub issue requesting ../ruchy WASM backend support, then execute spike when ready.
