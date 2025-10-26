# WASM-008: Advanced Optimization Passes - GREEN Phase Plan

## Overview

The GREEN phase for WASM-008 focuses on implementing minimal optimization passes to make all 40 RED phase tests pass. Following Extreme TDD methodology, this phase prioritizes simplicity and correctness over performance.

## Objectives

1. **Make All Tests Pass** - Implement minimal logic for 40 tests
2. **Establish Baseline** - Create performance baseline for REFACTOR
3. **Simple Algorithms** - Use straightforward approaches (optimize in REFACTOR)
4. **No Premature Optimization** - Focus on correctness, not speed

## GREEN Phase Implementation Strategy

### Priority: Correctness > Performance

- Use simple, naive algorithms
- Avoid complex data structures
- Implement obvious optimizations only
- Defer advanced techniques to REFACTOR phase

### Performance Expectations (GREEN Phase)

- Code size reduction: 10-15% (target: 30%)
- Runtime speedup: 10-20% (target: 40%)
- Optimization time: <500ms for 1,000 LOC (target: <200ms)

**GREEN serves as baseline for REFACTOR improvements.**

## Implementation Plan

### Component 1: Constant Folding (Minimal)

**File**: `/bootstrap/stage3/constant_folder_green.ruchy`
**Estimated LOC**: ~400 lines

**Approach**: Simple expression evaluation

```ruchy
// Minimal constant folding - evaluate simple binary expressions
fun fold_constants(expr: Expr) -> Expr {
    match expr {
        Expr::Binary(op, left, right) => {
            // Recursively fold sub-expressions
            let left_folded = fold_constants(*left);
            let right_folded = fold_constants(*right);

            // If both sides are constant literals, evaluate
            match (left_folded, right_folded) {
                (Expr::IntLit(a), Expr::IntLit(b)) => {
                    match op {
                        BinOp::Add => Expr::IntLit(a + b),
                        BinOp::Sub => Expr::IntLit(a - b),
                        BinOp::Mul => Expr::IntLit(a * b),
                        BinOp::Div => if b != 0 { Expr::IntLit(a / b) } else { expr },
                        _ => Expr::Binary(op, Box::new(left_folded), Box::new(right_folded)),
                    }
                },
                (Expr::BoolLit(a), Expr::BoolLit(b)) => {
                    match op {
                        BinOp::And => Expr::BoolLit(a && b),
                        BinOp::Or => Expr::BoolLit(a || b),
                        _ => Expr::Binary(op, Box::new(left_folded), Box::new(right_folded)),
                    }
                },
                _ => Expr::Binary(op, Box::new(left_folded), Box::new(right_folded)),
            }
        },
        Expr::If(cond, then_branch, else_branch) => {
            let cond_folded = fold_constants(*cond);
            match cond_folded {
                Expr::BoolLit(true) => fold_constants(*then_branch),
                Expr::BoolLit(false) => fold_constants(*else_branch),
                _ => Expr::If(Box::new(cond_folded), then_branch, else_branch),
            }
        },
        _ => expr,  // Other expressions unchanged
    }
}
```

**Tests Passing**: 10/10 constant folding tests

### Component 2: Dead Code Elimination (Minimal)

**File**: `/bootstrap/stage3/dead_code_eliminator_green.ruchy`
**Estimated LOC**: ~350 lines

**Approach**: Simple unreachable code detection

```ruchy
// Minimal dead code elimination - remove code after return/break
fun eliminate_dead_code(stmts: Vec<Stmt>) -> Vec<Stmt> {
    let mut result = Vec::new();
    let mut unreachable = false;

    for stmt in stmts {
        if unreachable {
            // Skip all statements after return/break
            continue;
        }

        match stmt {
            Stmt::Return(_) => {
                result.push(stmt);
                unreachable = true;
            },
            Stmt::Break => {
                result.push(stmt);
                unreachable = true;
            },
            Stmt::If(cond, then_block, else_block) => {
                // Recursively eliminate in branches
                let then_cleaned = eliminate_dead_code(then_block);
                let else_cleaned = eliminate_dead_code(else_block);

                // If condition is constant false, eliminate if block
                match cond {
                    Expr::BoolLit(false) => {
                        result.extend(else_cleaned);
                    },
                    Expr::BoolLit(true) => {
                        result.extend(then_cleaned);
                    },
                    _ => {
                        result.push(Stmt::If(cond, then_cleaned, else_cleaned));
                    }
                }
            },
            Stmt::While(Expr::BoolLit(false), _) => {
                // Eliminate while(false) loops
                continue;
            },
            _ => result.push(stmt),
        }
    }

    result
}
```

**Tests Passing**: 10/10 dead code elimination tests

### Component 3: Loop Optimization (Minimal)

**File**: `/bootstrap/stage3/loop_optimizer_green.ruchy`
**Estimated LOC**: ~300 lines

**Approach**: Unroll small constant loops only

```ruchy
// Minimal loop optimization - unroll small constant loops
fun optimize_loops(stmt: Stmt) -> Stmt {
    match stmt {
        Stmt::For(var, Expr::Range(Expr::IntLit(start), Expr::IntLit(end)), body) => {
            let iterations = end - start;

            // Only unroll very small loops (≤4 iterations)
            if iterations > 0 && iterations <= 4 {
                let mut unrolled = Vec::new();
                for i in start..end {
                    // Substitute loop variable with constant
                    let body_with_const = substitute_var(body.clone(), var, Expr::IntLit(i));
                    unrolled.extend(body_with_const);
                }
                Stmt::Block(unrolled)
            } else if iterations == 0 {
                // Zero iteration loop - eliminate entirely
                Stmt::Block(Vec::new())
            } else {
                // Keep loop as-is
                Stmt::For(var, Expr::Range(Expr::IntLit(start), Expr::IntLit(end)), body)
            }
        },
        _ => stmt,
    }
}
```

**Tests Passing**: 6/10 loop optimization tests (simple cases only)

### Component 4: Function Inlining (Minimal)

**File**: `/bootstrap/stage3/inliner_green.ruchy`
**Estimated LOC**: ~250 lines

**Approach**: Inline only tiny functions (<5 statements)

```ruchy
// Minimal inlining - only inline very small functions
struct InlineContext {
    functions: HashMap<String, Function>,
    inlined_count: i32,
}

fun inline_calls(expr: Expr, ctx: &InlineContext) -> Expr {
    match expr {
        Expr::Call(func_name, args) => {
            if let Some(func) = ctx.functions.get(&func_name) {
                // Only inline if function is tiny (1-4 statements)
                let statement_count = count_statements(&func.body);

                if statement_count <= 4 && !is_recursive(func, &func_name) {
                    // Simple inlining: substitute parameters with arguments
                    substitute_inline(func.body.clone(), &func.params, &args)
                } else {
                    Expr::Call(func_name, args)
                }
            } else {
                Expr::Call(func_name, args)
            }
        },
        _ => expr,
    }
}

fun is_recursive(func: &Function, name: &str) -> bool {
    // Simple check: does function body call itself?
    contains_call(&func.body, name)
}
```

**Tests Passing**: 5/10 inlining tests (simple cases only)

### Component 5: Integration & Orchestration

**File**: `/bootstrap/stage3/optimizer_green.ruchy`
**Estimated LOC**: ~200 lines

**Main Optimization Pipeline**:

```ruchy
pub struct WasmOptimizer {
    enable_constant_folding: bool,
    enable_dead_code: bool,
    enable_loop_opt: bool,
    enable_inlining: bool,
}

impl WasmOptimizer {
    pub fun new() -> Self {
        WasmOptimizer {
            enable_constant_folding: true,
            enable_dead_code: true,
            enable_loop_opt: true,
            enable_inlining: true,
        }
    }

    pub fun optimize(&self, module: WasmModule) -> WasmModule {
        let mut optimized = module;

        // Pass 1: Constant folding
        if self.enable_constant_folding {
            optimized = constant_fold_pass(optimized);
        }

        // Pass 2: Dead code elimination
        if self.enable_dead_code {
            optimized = dead_code_pass(optimized);
        }

        // Pass 3: Loop optimization
        if self.enable_loop_opt {
            optimized = loop_optimize_pass(optimized);
        }

        // Pass 4: Inlining
        if self.enable_inlining {
            optimized = inline_pass(optimized);
        }

        // Pass 5: Final dead code cleanup
        if self.enable_dead_code {
            optimized = dead_code_pass(optimized);
        }

        optimized
    }
}
```

**Tests Passing**: 5/5 integration tests

## Total Implementation Size (GREEN Phase)

| Component | Estimated LOC | Tests Passing |
|-----------|---------------|---------------|
| Constant Folding | ~400 | 10/10 |
| Dead Code Elimination | ~350 | 10/10 |
| Loop Optimization | ~300 | 6/10 |
| Function Inlining | ~250 | 5/10 |
| Integration | ~200 | 5/5 |
| **Total** | **~1,500 LOC** | **36/40 (90%)** |

**Note**: Some tests may still fail in GREEN phase due to minimal implementation. REFACTOR will achieve 40/40.

## Performance Baseline (GREEN Phase)

### Expected Metrics

**Code Size Reduction**:
- Unoptimized: 100KB
- GREEN optimized: 85-90KB (10-15% reduction)
- Target (REFACTOR): 70KB (30% reduction)

**Runtime Speed**:
- Unoptimized: 100ms
- GREEN optimized: 80-90ms (10-20% faster)
- Target (REFACTOR): 60ms (40% faster)

**Optimization Time**:
- GREEN: 300-500ms for 1,000 LOC
- Target (REFACTOR): <200ms for 1,000 LOC

## Test Passing Strategy

### Constant Folding (10/10 Expected)
All tests should pass with simple expression evaluation:
- ✅ Arithmetic: `2 + 3 * 4` → `14`
- ✅ Boolean: `true && false` → `false`
- ✅ Comparison: `5 > 3` → `true`
- ✅ Nested: `(2 + 3) * (4 + 5)` → `45`
- ✅ Conditional: `if true { 42 } else { 0 }` → `42`

### Dead Code Elimination (10/10 Expected)
All tests should pass with simple unreachable code removal:
- ✅ Code after return removed
- ✅ Constant false branches removed
- ✅ While(false) loops removed
- ✅ Side effects preserved

### Loop Optimization (6/10 Expected)
Simple cases only:
- ✅ Zero-iteration loops eliminated
- ✅ Single-iteration loops unwrapped
- ✅ Small constant loops unrolled (≤4 iterations)
- ⏳ Loop invariant code motion (REFACTOR)
- ⏳ Loop fusion (REFACTOR)
- ⏳ Strength reduction (REFACTOR)
- ⏳ Vectorization (REFACTOR)

### Inlining (5/10 Expected)
Simple cases only:
- ✅ Inline tiny functions (<5 statements)
- ✅ Don't inline recursive functions
- ✅ Don't inline large functions
- ⏳ Hot path analysis (REFACTOR)
- ⏳ Cost-benefit analysis (REFACTOR)

### Integration (5/5 Expected)
All integration tests should pass:
- ✅ Combined optimizations work
- ✅ Pass ordering is correct
- ✅ Idempotence verified
- ✅ Semantic preservation verified
- ✅ Performance measured

## Known Limitations (GREEN Phase)

### Constant Folding
- ❌ String concatenation (complex)
- ❌ Array operations (complex)
- ❌ Overflow detection (deferred)

### Dead Code Elimination
- ❌ Complex control flow analysis
- ❌ Interprocedural analysis
- ❌ Unused function elimination (needs call graph)

### Loop Optimization
- ❌ Loop invariant code motion (needs dominators)
- ❌ Loop fusion (complex analysis)
- ❌ Vectorization (SIMD required)
- ❌ Strength reduction (complex patterns)

### Inlining
- ❌ Hot path identification (needs profiling)
- ❌ Sophisticated cost models
- ❌ Partial inlining
- ❌ Cross-module inlining

**All limitations addressed in REFACTOR phase.**

## Success Criteria - GREEN Phase

✅ **36-40 tests passing** (90-100% success rate)
✅ **Code size: 10-15% reduction** (baseline established)
✅ **Runtime: 10-20% faster** (baseline established)
✅ **All code compiles** (no syntax/type errors)
✅ **Documentation complete** (GREEN completion report)

## Timeline

- **Implementation**: 2 days (4 components + integration)
- **Testing**: 0.5 days (make tests pass)
- **Documentation**: 0.5 days (completion report)
- **Total**: 3 days

## Next Steps (After GREEN)

1. **Document GREEN Completion**
   - Create WASM_008_OPTIMIZATION_GREEN_COMPLETE.md
   - Record baseline performance metrics
   - List limitations for REFACTOR

2. **Update INTEGRATION.md**
   - Mark GREEN phase complete
   - Document test results

3. **Begin REFACTOR Phase**
   - Advanced algorithms (dominators, call graph)
   - Performance optimization
   - Achieve 30% size, 40% speed targets

## Comparison with Previous Features

| Metric | WASM-006 GREEN | WASM-007 GREEN | WASM-008 GREEN (Planned) |
|--------|----------------|----------------|-------------------------|
| Implementation LOC | ~2,700 | ~1,975 | ~1,500 |
| Tests Passing | 30/30 | 30/30 | 36-40/40 |
| Timeline | 2-3 days | 2-3 days | 2-3 days |
| Performance | Baseline | Baseline | 10-20% improvement |

## Conclusion

The GREEN phase for WASM-008 focuses on minimal optimization implementations to make tests pass. Using simple algorithms and straightforward approaches, we'll establish a performance baseline and validate correctness. The REFACTOR phase will then optimize for production with advanced techniques and achieve the 30% size / 40% speed targets.

---

**Phase**: GREEN
**Status**: PLANNED
**Implementation**: ~1,500 LOC across 5 files
**Tests**: 36-40/40 expected to pass
**Timeline**: 2-3 days
**Next**: Begin implementation of constant folding
