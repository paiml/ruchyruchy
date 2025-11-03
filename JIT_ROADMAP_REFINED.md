# JIT Implementation Roadmap (Refined with Expert Review)

**Date**: 2025-11-03
**Status**: Expert-reviewed and refined with Toyota Way + CS research
**Goal**: Build production-quality JIT with mixed-mode execution

---

## Expert Review Insights (Integrated)

### ✅ Validated Strengths
1. **Profiler (PERF-001B)**: Direct, 100% applicable - "the brains of the JIT decision engine"
2. **Cranelift Choice**: Wise, pragmatic starting point - lowers barrier vs LLVM
3. **Mixed-Mode Strategy**: Aligns with 20+ years of proven research (HotSpot, V8, Julia)

### 🔧 Critical Refinements (CS Research-Backed)

#### 1. Evolve Profiler Granularity (Kaizen)
**Current**: Phase-level (Parse vs Eval) ✅
**Need**: Function-level and loop-level profiling

**Questions JIT Must Answer**:
- Which *function* should I compile?
- Which *loop* inside that function is hot?
- How many times has this loop iterated? (for OSR - On-Stack Replacement)

**Research Foundation**: Method JIT (Java HotSpot, V8, Julia approach)

#### 2. Add Type Stability Profiling (Critical for Optimization)
**Concept**: Profile for type stability at function call sites
**Why**: Julia's secret sauce - type specialization

**Example**:
```rust
// Type-stable (excellent JIT candidate)
fun add(a: int, b: int) { return a + b; }
// Always called with integers → JIT can specialize

// Type-unstable (poor JIT candidate)
fun add(a, b) { return a + b; }
// Called with int+int, then string+string, then float+float → cannot specialize
```

**Research Foundation**: Type specialization is THE key optimization for dynamic languages

#### 3. Bytecode VM Evolution (Long-term Kaizen)
**Current**: Tree-walking interpreter ✅
**Evolution**: Bytecode interpreter (Tier 0.5)
**Future**: JIT compiler (Tier 1+)

**Why**:
- Bytecode is better JIT input than AST (more linear, machine-like)
- Speeds up interpreter simultaneously
- Simplifies JIT code generation

**Research**: This is the proven path (Java bytecode → HotSpot, Python bytecode → PyPy)

#### 4. Jidoka Mechanism (Quality Built-In)
**Problem**: Deoptimization thrashing
- JIT compiles function with assumptions
- Assumptions violated → deoptimize to interpreter
- If repeated → "JIT thrashing" (worse than interpreter!)

**Jidoka Solution**: "Andon Cord" for JIT
```rust
struct JitRuntime {
    deopt_counters: HashMap<FunctionId, usize>,
    blacklist: HashSet<FunctionId>,
}

impl JitRuntime {
    fn handle_deoptimization(&mut self, func_id: FunctionId) {
        let count = self.deopt_counters.entry(func_id).or_insert(0);
        *count += 1;

        if *count > 3 {
            // Pull the andon cord!
            self.blacklist.insert(func_id);
            eprintln!("⚠️ JIT: Function {} blacklisted (deopt count: {})", func_id, count);
        }
    }
}
```

**Research**: Prevents pathological behavior seen in early JIT systems

---

## Refined Implementation Phases

### Phase 1: Complete Profiling Foundation (Weeks 1-2) 🔥 CURRENT

**PERF-001C: Function-Level Profiling** (New priority!)
```rust
// Enhance profiler with function-level detail
struct FunctionProfile {
    name: String,
    call_count: usize,
    total_time_us: f64,
    self_time_us: f64,  // Excluding callees
    type_signatures: Vec<TypeSignature>,  // NEW: Type stability tracking
}

// Example output:
// Function: fibonacci
//   Calls: 177
//   Time: 45.2ms (89.3% of total) ← HOT FUNCTION
//   Type stability: 100% (always called with int) ← EXCELLENT JIT CANDIDATE
```

**PERF-001D: Loop-Level Profiling**
```rust
// Track hot loops for OSR (On-Stack Replacement)
struct LoopProfile {
    function: String,
    line: usize,
    iteration_count: usize,
    avg_time_per_iteration_us: f64,
}

// Example output:
// Hot Loop: fibonacci:5
//   Iterations: 10,000
//   Time/iter: 8.5µs
//   🎯 OSR CANDIDATE (compile while running)
```

**PERF-001E: Type Stability Tracking**
```rust
// Track types at call sites
struct TypeProfile {
    call_site: Location,
    observed_types: Vec<(Vec<TypeId>, usize)>,  // (arg types, count)
    stability_score: f64,  // 1.0 = always same types, 0.0 = chaos
}

// Example output:
// Function: add (line 10)
//   Call site: main:15
//   Types observed: (Int, Int) → 95%, (Float, Float) → 5%
//   Stability: 0.95 ← GOOD JIT CANDIDATE (mostly stable)
```

**Deliverables**:
- Enhanced `Profiler` struct with function/loop/type tracking
- `ruchydbg benchmark <file>` command (function-level micro-benchmarks)
- `ruchydbg hotspots <file>` command (identify JIT candidates)
- Type stability analysis output

**Success Criteria**:
- Can identify hot functions (>30% of time)
- Can identify hot loops (>1000 iterations)
- Can measure type stability (>80% = stable)
- All data feeds into JIT compilation decisions

---

### Phase 2: Cranelift JIT Foundation (Weeks 3-4)

**JIT-001: Add Cranelift Dependency**
```toml
# Cargo.toml
[dependencies]
cranelift = "0.109"
cranelift-jit = "0.109"
cranelift-module = "0.109"
cranelift-frontend = "0.109"
```

**JIT-002: Basic JIT Infrastructure**
```rust
// src/jit/mod.rs
pub struct JitCompiler {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    module: JITModule,
}

impl JitCompiler {
    pub fn new() -> Self {
        let builder = JITBuilder::new(cranelift_module::default_libcall_names());
        let module = JITModule::new(builder);
        // ...
    }

    pub fn compile_function(&mut self, ast: &AstNode) -> Result<*const u8, JitError> {
        // AST → Cranelift IR → Machine code
        // Returns function pointer
    }
}
```

**JIT-003: Simple Expression Compilation**
```rust
// Start with arithmetic (like COMPILE-001, but to machine code)
impl JitCompiler {
    fn compile_expr(&mut self, expr: &AstNode, builder: &mut FunctionBuilder) -> Value {
        match expr {
            AstNode::Integer(n) => {
                builder.ins().iconst(types::I64, *n)
            }
            AstNode::BinaryOp { left, op, right } => {
                let lhs = self.compile_expr(left, builder);
                let rhs = self.compile_expr(right, builder);
                match op {
                    BinaryOperator::Add => builder.ins().iadd(lhs, rhs),
                    BinaryOperator::Sub => builder.ins().isub(lhs, rhs),
                    // ... Same patterns as COMPILE-001!
                }
            }
            // ...
        }
    }
}
```

**Tests** (EXTREME TDD):
```rust
#[test]
fn test_jit_compile_addition() {
    let source = "2 + 3";
    let ast = parse(source);

    let mut jit = JitCompiler::new();
    let func_ptr = jit.compile_function(&ast).unwrap();

    let func: fn() -> i64 = unsafe { std::mem::transmute(func_ptr) };
    assert_eq!(func(), 5);
}
```

**Deliverables**:
- `src/jit/` module with Cranelift integration
- Can compile simple arithmetic expressions
- Can execute compiled code
- Tests passing (arithmetic operations)

---

### Phase 3: Mixed-Mode Execution (Weeks 5-6)

**JIT-004: Execution Strategy**
```rust
pub enum ExecutionMode {
    Interpret,     // Tier 0: Always start here (fast startup)
    JitCompile,    // Tier 1: Hot functions
}

pub struct MixedModeRuntime {
    interpreter: Evaluator,
    jit: JitCompiler,
    profiler: Profiler,
    jit_runtime: JitRuntime,  // Tracks compiled code + deoptimization
}

impl MixedModeRuntime {
    pub fn execute(&mut self, ast: &AstNode) -> Result<Value, Error> {
        // Tier 0: Start with interpreter + profiling
        self.profiler.start_profiling();
        let result = self.interpreter.eval(ast)?;
        self.profiler.stop_profiling();

        // Analyze: Should we JIT compile?
        for func in ast.functions() {
            let profile = self.profiler.function_profile(func);

            if self.should_jit_compile(func, profile) {
                println!("🔥 JIT: Compiling hot function: {}", func.name);
                match self.jit.compile_function(func) {
                    Ok(compiled) => {
                        self.jit_runtime.register(func.id, compiled);
                    }
                    Err(e) => {
                        eprintln!("⚠️ JIT: Compilation failed: {}", e);
                        // Fall back to interpreter (Jidoka!)
                    }
                }
            }
        }

        Ok(result)
    }

    fn should_jit_compile(&self, func: &Function, profile: &FunctionProfile) -> bool {
        // Criteria for JIT compilation:
        // 1. Hot (>30% of execution time OR >1000 calls)
        // 2. Type-stable (>80% same types)
        // 3. Not blacklisted (Jidoka)

        let is_hot = profile.percentage > 30.0 || profile.call_count > 1000;
        let is_stable = profile.type_stability > 0.8;
        let not_blacklisted = !self.jit_runtime.is_blacklisted(func.id);

        is_hot && is_stable && not_blacklisted
    }
}
```

**JIT-005: Deoptimization & Jidoka**
```rust
pub struct JitRuntime {
    compiled_functions: HashMap<FunctionId, CompiledCode>,
    deopt_counters: HashMap<FunctionId, usize>,
    blacklist: HashSet<FunctionId>,
}

impl JitRuntime {
    pub fn execute_or_deoptimize(
        &mut self,
        func_id: FunctionId,
        args: &[Value],
    ) -> Result<Value, NeedsDeoptimization> {
        let compiled = self.compiled_functions.get(&func_id)?;

        // Check assumptions (e.g., types)
        if !compiled.assumptions_valid(args) {
            self.handle_deoptimization(func_id);
            return Err(NeedsDeoptimization);
        }

        // Execute compiled code
        unsafe { compiled.execute(args) }
    }

    fn handle_deoptimization(&mut self, func_id: FunctionId) {
        let count = self.deopt_counters.entry(func_id).or_insert(0);
        *count += 1;

        println!("⚠️ Deoptimization: {} (count: {})", func_id, count);

        if *count > 3 {
            // Pull the andon cord!
            self.blacklist.insert(func_id);
            self.compiled_functions.remove(&func_id);
            eprintln!("🚨 JIT: Function {} blacklisted (excessive deoptimization)", func_id);
        }
    }
}
```

**Deliverables**:
- Mixed-mode execution working
- JIT compilation triggered by profiling data
- Deoptimization handling with Jidoka mechanism
- Blacklist prevents JIT thrashing

---

### Phase 4: Bytecode Evolution (Weeks 7-10) - Long-term Kaizen

**WHY**: Bytecode is a better JIT input than AST
- More linear, machine-like
- Easier to optimize
- Faster interpreter baseline

**JIT-006: Bytecode Design**
```rust
pub enum Bytecode {
    // Stack-based bytecode (like Java, Python)
    LoadConst(usize),       // Push constant from constant pool
    LoadLocal(usize),       // Push local variable
    StoreLocal(usize),      // Pop and store to local
    Add,                    // Pop 2, add, push result
    Sub,
    Call(usize, usize),     // Call function (func_id, arg_count)
    Return,
    // ...
}

// Example: 2 + 3
// Bytecode:
//   LoadConst(0)  // Load 2
//   LoadConst(1)  // Load 3
//   Add           // 2 + 3
//   Return
```

**JIT-007: Bytecode Interpreter** (Tier 0.5)
```rust
impl BytecodeVM {
    pub fn execute(&mut self, bytecode: &[Bytecode]) -> Value {
        let mut stack: Vec<Value> = Vec::new();

        for inst in bytecode {
            match inst {
                Bytecode::LoadConst(idx) => {
                    stack.push(self.constants[*idx].clone());
                }
                Bytecode::Add => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a + b);
                }
                // ... Much faster than tree-walking!
            }
        }

        stack.pop().unwrap()
    }
}
```

**JIT-008: Bytecode → Cranelift**
```rust
impl JitCompiler {
    pub fn compile_bytecode(&mut self, bytecode: &[Bytecode]) -> CompiledCode {
        // Bytecode is MUCH easier to compile than AST!
        // More linear, explicit control flow, simpler patterns

        for inst in bytecode {
            match inst {
                Bytecode::LoadConst(idx) => {
                    let val = self.constants[*idx];
                    builder.ins().iconst(types::I64, val);
                }
                Bytecode::Add => {
                    let b = builder.stack_pop();
                    let a = builder.stack_pop();
                    let result = builder.ins().iadd(a, b);
                    builder.stack_push(result);
                }
                // ... Straightforward translation!
            }
        }
    }
}
```

**Deliverables**:
- Bytecode VM implementation
- AST → Bytecode compiler
- Bytecode → Cranelift JIT compiler
- Performance validation (bytecode VM should be 2-5x faster than tree-walker)

---

## Success Criteria by Phase

### Phase 1 (Profiling) ✅
- ✅ Function-level profiling (identify hot functions)
- ✅ Loop-level profiling (identify hot loops)
- ✅ Type stability tracking (identify JIT candidates)
- ✅ All data feeds JIT decisions

### Phase 2 (Cranelift Foundation) 🔧
- ✅ Can compile simple expressions (arithmetic)
- ✅ Generated code executes correctly
- ✅ Tests passing (5+ test cases)

### Phase 3 (Mixed-Mode) 🚀
- ✅ Interpreter + JIT working together
- ✅ Hot functions automatically JIT compiled
- ✅ Deoptimization handling with Jidoka
- ✅ 10-50x speedup on hot code vs interpreter

### Phase 4 (Bytecode) 🎓
- ✅ Bytecode VM 2-5x faster than tree-walker
- ✅ JIT compilation from bytecode working
- ✅ Mixed-mode with bytecode as Tier 0

---

## Performance Targets (Research-Backed)

Based on similar systems:

| System | Baseline | Tier 0 | Tier 1 JIT | Optimized |
|--------|----------|--------|------------|-----------|
| **Java HotSpot** | - | Interpreter | Method JIT | C2 Compiler |
| **V8 (JS)** | - | Ignition (bytecode) | TurboFan | - |
| **Julia** | - | Interpreter | LLVM JIT | LLVM JIT (opt) |
| **Our Target** | Tree-walker (1x) | Bytecode (3x) | Cranelift (20x) | Future (50x+) |

**Fibonacci(35) Targets**:
- Tree-walker: ~500ms (baseline) ✅ HAVE
- Bytecode VM: ~150ms (3x) 📋 Phase 4
- Cranelift JIT: ~25ms (20x) 🎯 Phase 3
- Optimized JIT: ~10ms (50x) 🚀 Future

---

## Immediate Next Steps (Week 1)

**Priority 1**: Complete PERF-001C (Function Profiling)
```bash
# RED: Write test
cargo test --test test_perf_001c_function_profiling

# GREEN: Implement function-level profiler
# - Call counts per function
# - Time per function (self + total)
# - Percentage of total execution

# REFACTOR: Clean up, optimize
```

**Priority 2**: Add Type Stability Tracking (PERF-001E)
```rust
// Track types at call sites
impl Profiler {
    pub fn record_call(&mut self, func: &str, args: &[Value]) {
        let types: Vec<TypeId> = args.iter().map(|v| v.type_id()).collect();
        self.call_site_types
            .entry((func.to_string(), types.clone()))
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    pub fn type_stability(&self, func: &str) -> f64 {
        // Calculate stability: 1.0 = always same types
        let calls = self.call_site_types.iter()
            .filter(|((f, _), _)| f == func)
            .collect::<Vec<_>>();

        if calls.is_empty() {
            return 0.0;
        }

        let max_count = calls.iter().map(|(_, count)| *count).max().unwrap();
        let total_count: usize = calls.iter().map(|(_, count)| *count).sum();

        max_count as f64 / total_count as f64
    }
}
```

---

**Status**: 🟢 REFINED AND READY
**Foundation**: ✅ Interpreter + Profiler
**Next**: 🔥 Complete profiling suite (function + type stability)
**Future**: 🚀 Cranelift JIT + Mixed-mode execution

**Validated by**: Toyota Way + 20+ years of JIT research (HotSpot, V8, Julia, PyPy)
