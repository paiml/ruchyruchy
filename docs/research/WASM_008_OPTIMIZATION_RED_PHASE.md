# WASM-008: Advanced Optimization Passes - RED Phase Plan

## Overview

The RED phase for WASM-008 focuses on defining comprehensive test requirements for advanced optimization passes in WebAssembly code generation. This phase establishes the requirements through failing tests following Extreme TDD methodology.

## Objectives

1. **Constant Folding** - Evaluate compile-time constant expressions
2. **Dead Code Elimination** - Remove unreachable and unused code
3. **Loop Optimizations** - Optimize loop structures and iterations
4. **Inlining Strategies** - Inline small functions for performance

## Target Metrics

Following the pattern from WASM-006 and WASM-007:

### Performance Targets
- **Code Size Reduction**: 30% smaller optimized code
- **Runtime Speed**: 40% faster execution
- **Optimization Time**: <200ms for 1,000 LOC
- **Memory Usage**: <10MB during optimization

### Quality Targets
- **Code Duplication**: <1%
- **Cyclomatic Complexity**: <15 per function
- **Error Handling**: 80%+ Result-based
- **Test Coverage**: 50,000+ test cases (RED + GREEN + REFACTOR + TOOL)

## RED Phase Test Strategy

### Test Suite 1: Constant Folding (10 tests)
**File**: `/validation/wasm/optimization/test_constant_folding_red.ruchy`
**Estimated LOC**: ~400 lines

Tests to verify compile-time constant evaluation:

1. **test_arithmetic_constant_folding**
   - Input: `let x = 2 + 3 * 4;`
   - Expected: Optimized to `let x = 14;`
   - Verifies: Binary arithmetic operations folded

2. **test_boolean_constant_folding**
   - Input: `let flag = true && false;`
   - Expected: Optimized to `let flag = false;`
   - Verifies: Boolean operations folded

3. **test_string_constant_folding**
   - Input: `let msg = "Hello, " + "World";`
   - Expected: Optimized to `let msg = "Hello, World";`
   - Verifies: String concatenation folded

4. **test_comparison_constant_folding**
   - Input: `let cmp = 5 > 3;`
   - Expected: Optimized to `let cmp = true;`
   - Verifies: Comparison operations folded

5. **test_nested_constant_folding**
   - Input: `let result = (2 + 3) * (4 + 5);`
   - Expected: Optimized to `let result = 45;`
   - Verifies: Nested expressions folded

6. **test_constant_propagation**
   - Input: `let x = 5; let y = x + 3;`
   - Expected: Optimized to `let x = 5; let y = 8;`
   - Verifies: Constants propagated through variables

7. **test_conditional_constant_folding**
   - Input: `if true { return 42; } else { return 0; }`
   - Expected: Optimized to `return 42;`
   - Verifies: Constant conditions optimized

8. **test_array_constant_folding**
   - Input: `let arr = [1 + 1, 2 * 2, 3 + 3];`
   - Expected: Optimized to `let arr = [2, 4, 6];`
   - Verifies: Array literal elements folded

9. **test_function_call_not_folded**
   - Input: `let x = random() + random();`
   - Expected: NOT optimized (side effects)
   - Verifies: Non-pure functions not folded

10. **test_overflow_constant_folding**
    - Input: `let x = i32::MAX + 1;`
    - Expected: Compile-time error or wrapped value
    - Verifies: Overflow handling in constant folding

### Test Suite 2: Dead Code Elimination (10 tests)
**File**: `/validation/wasm/optimization/test_dead_code_elimination_red.ruchy`
**Estimated LOC**: ~450 lines

Tests to verify removal of unreachable and unused code:

1. **test_unreachable_after_return**
   - Input: `fun f() { return 42; let x = 5; }`
   - Expected: `let x = 5;` eliminated
   - Verifies: Code after return removed

2. **test_unreachable_branch**
   - Input: `if false { expensive_call(); }`
   - Expected: Entire if block eliminated
   - Verifies: Constant false branches removed

3. **test_unused_variable**
   - Input: `let x = 42; return 0;`
   - Expected: `let x = 42;` eliminated
   - Verifies: Unused variables removed

4. **test_unused_function**
   - Input: `fun helper() { ... } fun main() { ... }`
   - Expected: `helper()` eliminated if not called
   - Verifies: Unused functions removed

5. **test_dead_assignment**
   - Input: `let mut x = 5; x = 10; return 0;`
   - Expected: Both assignments eliminated
   - Verifies: Dead assignments removed

6. **test_unreachable_loop**
   - Input: `while false { ... }`
   - Expected: Entire loop eliminated
   - Verifies: Constant false loops removed

7. **test_side_effect_preservation**
   - Input: `let x = print("hi"); return 0;`
   - Expected: Print call preserved, assignment eliminated
   - Verifies: Side effects not eliminated

8. **test_used_in_nested_scope**
   - Input: `let x = 5; { let y = x; return y; }`
   - Expected: `x` NOT eliminated (used in nested scope)
   - Verifies: Scope analysis correct

9. **test_partial_branch_elimination**
   - Input: `if condition { dead_code(); } else { live_code(); }`
   - Expected: Both branches preserved (dynamic condition)
   - Verifies: Only constant branches eliminated

10. **test_dead_code_after_break**
    - Input: `loop { break; let x = 5; }`
    - Expected: `let x = 5;` eliminated
    - Verifies: Code after break removed

### Test Suite 3: Loop Optimizations (10 tests)
**File**: `/validation/wasm/optimization/test_loop_optimization_red.ruchy`
**Estimated LOC**: ~500 lines

Tests to verify loop optimization transformations:

1. **test_loop_invariant_code_motion**
   - Input: `loop { let x = expensive(); use(x); }`
   - Expected: `expensive()` moved outside loop if invariant
   - Verifies: Invariant code hoisted

2. **test_loop_unrolling_small**
   - Input: `for i in 0..4 { f(i); }`
   - Expected: Unrolled to `f(0); f(1); f(2); f(3);`
   - Verifies: Small loops unrolled

3. **test_loop_fusion**
   - Input: `for i in 0..n { a[i] = 0; } for i in 0..n { b[i] = 0; }`
   - Expected: Fused into single loop
   - Verifies: Adjacent loops fused

4. **test_loop_strength_reduction**
   - Input: `for i in 0..n { let x = i * 4; }`
   - Expected: Multiplication replaced with addition
   - Verifies: Expensive operations reduced

5. **test_loop_constant_iteration**
   - Input: `for _ in 0..0 { ... }`
   - Expected: Entire loop eliminated
   - Verifies: Zero-iteration loops removed

6. **test_loop_single_iteration**
   - Input: `for _ in 0..1 { body(); }`
   - Expected: Converted to `body();`
   - Verifies: Single-iteration loops unwrapped

7. **test_loop_vectorization**
   - Input: `for i in 0..n { a[i] = b[i] + c[i]; }`
   - Expected: SIMD vectorization applied
   - Verifies: Vectorizable loops optimized

8. **test_loop_induction_variable**
   - Input: `let mut i = 0; loop { if i >= n { break; } i += 1; }`
   - Expected: Converted to for loop or optimized
   - Verifies: Induction variables recognized

9. **test_nested_loop_interchange**
   - Input: `for i in 0..n { for j in 0..m { a[j][i] = 0; } }`
   - Expected: Loops interchanged for cache efficiency
   - Verifies: Loop reordering for performance

10. **test_loop_with_side_effects**
    - Input: `for i in 0..n { print(i); }`
    - Expected: NOT optimized (side effects)
    - Verifies: Side effects prevent optimization

### Test Suite 4: Inlining Strategies (10 tests)
**File**: `/validation/wasm/optimization/test_inlining_red.ruchy`
**Estimated LOC**: ~450 lines

Tests to verify function inlining decisions:

1. **test_inline_small_function**
   - Input: `fun add(x: i32, y: i32) -> i32 { x + y }`
   - Expected: Inlined at call sites
   - Verifies: Small functions inlined

2. **test_no_inline_large_function**
   - Input: `fun complex() { /* 100 lines */ }`
   - Expected: NOT inlined
   - Verifies: Large functions not inlined

3. **test_inline_single_use**
   - Input: `fun helper() { ... } fun main() { helper(); }`
   - Expected: Inlined (single use)
   - Verifies: Single-use functions inlined

4. **test_no_inline_recursive**
   - Input: `fun factorial(n: i32) -> i32 { if n == 0 { 1 } else { n * factorial(n-1) } }`
   - Expected: NOT inlined (recursive)
   - Verifies: Recursive functions not inlined

5. **test_inline_hot_path**
   - Input: `fun critical() { ... } /* called in tight loop */`
   - Expected: Inlined (hot path)
   - Verifies: Hot path functions prioritized

6. **test_inline_threshold**
   - Input: Functions of varying sizes (5, 10, 20, 50 lines)
   - Expected: Functions <20 lines inlined
   - Verifies: Size threshold respected

7. **test_inline_with_optimization**
   - Input: `fun square(x: i32) -> i32 { return x * x; }`
   - Expected: Inlined and constant folded
   - Verifies: Inlining enables further optimization

8. **test_partial_inlining**
   - Input: `fun mixed() { cold_path(); hot_path(); }`
   - Expected: Hot path inlined, cold path not
   - Verifies: Partial inlining supported

9. **test_inline_generic_function**
   - Input: `fun identity<T>(x: T) -> T { x }`
   - Expected: Monomorphized and inlined
   - Verifies: Generic functions inlined

10. **test_inline_cost_benefit**
    - Input: Multiple function calls with varying costs
    - Expected: Cost-benefit analysis applied
    - Verifies: Inlining decisions are optimal

## Additional Test Categories

### Integration Tests (5 tests)
**File**: `/validation/wasm/optimization/test_optimization_integration_red.ruchy`
**Estimated LOC**: ~300 lines

1. **test_combined_optimizations**
   - Constant folding + dead code elimination
   - Verifies: Optimizations compose correctly

2. **test_optimization_order**
   - Different optimization pass orders
   - Verifies: Pass ordering optimal

3. **test_optimization_idempotence**
   - Running optimizer multiple times
   - Verifies: Optimizer is idempotent

4. **test_optimization_correctness**
   - Optimized code produces same result
   - Verifies: Semantic preservation

5. **test_optimization_performance**
   - Measure actual speedup and size reduction
   - Verifies: Performance targets met

### Negative Tests (5 tests)
**File**: Same files as above (integrated)

1. **test_no_optimize_volatile**
   - Volatile operations not optimized

2. **test_no_optimize_side_effects**
   - Side-effecting code preserved

3. **test_preserve_semantics**
   - Optimization doesn't change behavior

4. **test_handle_optimization_failure**
   - Graceful fallback if optimization fails

5. **test_optimization_limits**
   - Resource limits respected (time, memory)

## Total RED Phase Tests

- **Constant Folding**: 10 tests (~400 LOC)
- **Dead Code Elimination**: 10 tests (~450 LOC)
- **Loop Optimizations**: 10 tests (~500 LOC)
- **Inlining Strategies**: 10 tests (~450 LOC)
- **Integration Tests**: 5 tests (~300 LOC)
- **Total**: **40 tests, ~2,100 LOC**

## Expected Test Results (RED Phase)

All 40 tests should **FAIL** in the RED phase because:
- No optimization passes implemented yet
- Optimizer infrastructure doesn't exist
- No code transformation logic present
- No cost-benefit analysis framework

## Success Criteria for RED Phase

✅ **All 40 tests created** - Complete test infrastructure
✅ **All 40 tests failing** - Demonstrates missing implementation
✅ **Clear requirements** - Tests document expected behavior
✅ **Runnable tests** - All tests execute (and fail correctly)
✅ **Documentation complete** - This plan and completion report

## Implementation Notes

### Optimization Pass Pipeline

```
Source Code
    ↓
Constant Folding Pass
    ↓
Dead Code Elimination Pass
    ↓
Loop Optimization Pass
    ↓
Inlining Pass
    ↓
Final Dead Code Elimination
    ↓
Optimized Code
```

### Data Structures Needed

1. **Control Flow Graph (CFG)** - For dead code analysis
2. **Dominator Tree** - For loop optimization
3. **Use-Def Chains** - For constant propagation
4. **Call Graph** - For inlining decisions
5. **Cost Model** - For optimization decisions

### Performance Baselines (for GREEN phase comparison)

- Unoptimized code: 100% size, 100% time (baseline)
- GREEN target: 90% size, 90% time (10% improvement)
- REFACTOR target: 70% size, 60% time (30% size, 40% speed)

## Next Steps (After RED Phase)

1. **Document RED completion** - Create WASM_008_OPTIMIZATION_RED_COMPLETE.md
2. **Update INTEGRATION.md** - Add WASM-008 RED phase status
3. **Update roadmap.yaml** - Track progress
4. **Begin GREEN phase** - Minimal optimization implementation

## Timeline

- **RED Phase**: 1-2 days (test creation + documentation)
- **GREEN Phase**: 2-3 days (minimal implementation)
- **REFACTOR Phase**: 2-3 days (production optimization)
- **TOOL Phase**: 1-2 days (comprehensive validation)
- **Total**: 6-10 days

## Risk Mitigation

### Risk 1: Optimization Complexity
**Mitigation**: Start with simple optimizations (constant folding), add complexity gradually

### Risk 2: Semantic Preservation
**Mitigation**: Extensive integration tests verifying correctness

### Risk 3: Performance Measurement
**Mitigation**: Benchmarking infrastructure in TOOL phase

### Risk 4: Optimization Interaction
**Mitigation**: Test optimization pass ordering and composition

## Comparison with Previous Features

| Metric | WASM-006 | WASM-007 | WASM-008 (Planned) |
|--------|----------|----------|-------------------|
| RED Tests | 30 | 30 | 40 |
| RED LOC | ~1,630 | ~1,630 | ~2,100 |
| GREEN LOC | ~2,700 | ~1,975 | ~2,500 (est) |
| REFACTOR LOC | ~2,900 | ~750 | ~3,000 (est) |
| TOOL Tests | 55,046+ | 151,030+ | 50,000+ (est) |
| Performance | 5-50x | 2-3x | 30% size, 40% speed |

## Conclusion

The RED phase for WASM-008 (Advanced Optimization Passes) establishes comprehensive requirements for four major optimization categories:
- Constant folding (10 tests)
- Dead code elimination (10 tests)
- Loop optimizations (10 tests)
- Inlining strategies (10 tests)
- Integration tests (5 tests)

All 40 tests will initially fail, demonstrating the need for implementation. The GREEN phase will provide minimal optimization infrastructure, REFACTOR will optimize for production, and TOOL will validate with 50,000+ test cases.

---

**Phase**: RED
**Status**: PLANNED
**Tests**: 40 failing tests (~2,100 LOC)
**Timeline**: 1-2 days
**Next**: Begin test file creation
