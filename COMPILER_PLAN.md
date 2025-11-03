# RuchyRuchy Compiler/Transpiler Plan

**Date**: 2025-11-03
**Goal**: Build AST-based Ruchy→Rust transpiler using our existing interpreter infrastructure
**Methodology**: EXTREME TDD + Incremental Development

---

## Context

We have:
- ✅ **Optimized interpreter** (30%+ speedup via INTERP-044 through INTERP-048)
- ✅ **Mature AST** (src/interpreter/parser.rs:1697 - complete AstNode enum)
- ✅ **Prototype codegen** (src/stage3_real_codegen.rs - string-based, not AST-integrated)

We need:
- **AST-based transpiler** that generates Rust code from interpreter AST
- **End-to-end compilation** from Ruchy source → Rust code → compiled binary
- **Performance validation** to compare interpreter vs compiled execution

---

## Architecture

### Module Structure

```
src/
├── interpreter/          # Existing (optimized)
│   ├── parser.rs        # AST definition (AstNode enum) ✅
│   ├── evaluator.rs     # Runtime execution ✅
│   └── value.rs         # Runtime values ✅
│
└── compiler/            # NEW - Transpilation
    ├── mod.rs           # Module exports
    ├── codegen.rs       # AST → Rust code generation
    ├── emit.rs          # Rust code emission (formatting)
    └── runtime.rs       # Runtime library for compiled code
```

### Key Design Decisions

1. **Reuse Interpreter AST**: No separate compiler AST - use `AstNode` enum
2. **Two-Phase Compilation**:
   - Phase 1: Ruchy → Rust (our transpiler)
   - Phase 2: Rust → Binary (delegate to `rustc`)
3. **Runtime Library**: Minimal runtime for compiled code (Value type, etc.)
4. **Optimization Strategy**: Focus on codegen quality, let rustc optimize

---

## Implementation Phases

### Phase 1: Foundation (COMPILE-001 to COMPILE-003)

**COMPILE-001: Module Structure**
- Create `src/compiler/` directory
- Create `mod.rs` with public API
- Export `CodeGenerator` struct
- Update `src/lib.rs` to include compiler module

**COMPILE-002: Basic Codegen (Expressions)**
- Literals (Integer, Float, String, Boolean)
- Variables (Identifier)
- Binary operations (+, -, *, /, %)
- Unary operations (-, !)

**Example**:
```rust
// Input AST:
AstNode::BinaryOp {
    left: Box::new(AstNode::Integer(5)),
    op: BinaryOperator::Add,
    right: Box::new(AstNode::Integer(3)),
}

// Generated Rust:
(5 + 3)
```

**COMPILE-003: Variable Declarations**
- `let x = expr;` → `let x = expr;`
- Type inference (use Rust's type inference)
- Multiple declarations

---

### Phase 2: Functions (COMPILE-004 to COMPILE-006)

**COMPILE-004: Function Definitions**
```ruchy
fun add(x, y) {
    return x + y;
}
```

↓ transpiles to ↓

```rust
fn add(x: i64, y: i64) -> i64 {
    return x + y;
}
```

**COMPILE-005: Function Calls**
- Direct calls: `add(2, 3)`
- Nested calls: `add(mul(2, 3), 4)`
- Return values

**COMPILE-006: Closures & Higher-Order Functions**
- Lambda expressions
- Closures capturing environment
- Map/filter/reduce

---

### Phase 3: Control Flow (COMPILE-007 to COMPILE-009)

**COMPILE-007: If/Else**
```ruchy
if (x > 0) {
    println("positive");
} else {
    println("negative");
}
```

**COMPILE-008: Loops**
- While loops
- For loops (C-style)
- Break/continue

**COMPILE-009: Return Statements**
- Early returns
- Implicit returns (last expression)

---

### Phase 4: Data Structures (COMPILE-010 to COMPILE-012)

**COMPILE-010: Arrays**
```ruchy
let arr = [1, 2, 3];
let first = arr[0];
```

↓

```rust
let arr = vec![1, 2, 3];
let first = arr[0];
```

**COMPILE-011: Hashmaps**
```ruchy
let map = {"key": "value"};
let v = map["key"];
```

**COMPILE-012: Tuples**
```ruchy
let pair = (1, 2);
let (a, b) = pair;
```

---

### Phase 5: Runtime Library (COMPILE-013 to COMPILE-015)

**COMPILE-013: Value Type**
- Compile-time equivalent of runtime `Value` enum
- Minimal overhead (use Rust enums, not Box<dyn>)
- Support for dynamic typing (if needed)

**COMPILE-014: Standard Library**
- `println`, `print` macros
- String operations
- Math functions
- Array/HashMap utilities

**COMPILE-015: Error Handling**
- Panic on runtime errors (for now)
- Future: Result<T, E> for recoverable errors

---

### Phase 6: End-to-End Integration (COMPILE-016 to COMPILE-018)

**COMPILE-016: Compilation Pipeline**
```rust
// High-level API
pub fn compile_to_rust(source: &str) -> Result<String, CompileError> {
    let mut parser = Parser::new(source);
    let ast = parser.parse()?;

    let mut codegen = CodeGenerator::new();
    let rust_code = codegen.generate(&ast)?;

    Ok(rust_code)
}
```

**COMPILE-017: Execute Compiled Code**
```rust
pub fn compile_and_run(source: &str) -> Result<String, CompileError> {
    let rust_code = compile_to_rust(source)?;

    // Write to temp file
    let temp_file = "/tmp/ruchy_compiled.rs";
    fs::write(temp_file, rust_code)?;

    // Compile with rustc
    Command::new("rustc")
        .arg(temp_file)
        .arg("-o")
        .arg("/tmp/ruchy_compiled")
        .status()?;

    // Run compiled binary
    let output = Command::new("/tmp/ruchy_compiled").output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
```

**COMPILE-018: Performance Comparison**
- Benchmark interpreter vs compiled
- Measure speedup (expected: 10-100x for compute-heavy code)
- Document trade-offs (compilation time vs runtime)

---

## Testing Strategy (EXTREME TDD)

### Test Structure

```
tests/
├── test_compile_001_module.rs         # Module structure
├── test_compile_002_basic_codegen.rs  # Expressions
├── test_compile_003_variables.rs      # Let declarations
├── test_compile_004_functions.rs      # Function defs
├── test_compile_005_calls.rs          # Function calls
├── test_compile_006_closures.rs       # Closures
├── test_compile_007_control_flow.rs   # If/else
├── test_compile_008_loops.rs          # While/for
├── test_compile_009_return.rs         # Return statements
├── test_compile_010_arrays.rs         # Array operations
├── test_compile_011_hashmaps.rs       # HashMap operations
├── test_compile_012_tuples.rs         # Tuple operations
├── test_compile_013_runtime.rs        # Runtime library
├── test_compile_014_stdlib.rs         # Standard library
├── test_compile_015_errors.rs         # Error handling
├── test_compile_016_pipeline.rs       # Compilation pipeline
├── test_compile_017_execution.rs      # End-to-end execution
└── test_compile_018_performance.rs    # Benchmarks
```

### Test Pattern (RED-GREEN-REFACTOR)

**RED Phase**:
```rust
#[test]
fn test_compile_integer_literal() {
    let source = "42";

    let mut parser = Parser::new(source);
    let ast = parser.parse().unwrap();

    let mut codegen = CodeGenerator::new();
    let rust_code = codegen.generate(&ast).unwrap();

    // Expected: "42"
    assert_eq!(rust_code.trim(), "42");
}
```

**GREEN Phase**: Implement minimal `CodeGenerator::generate()` to pass test

**REFACTOR Phase**: Clean up, extract helpers, improve code quality

---

## Success Criteria

### Phase 1-3 (Foundation + Functions + Control Flow)
- ✅ All tests passing (COMPILE-001 through COMPILE-009)
- ✅ Can compile simple programs (hello world, fibonacci)
- ✅ Generated Rust code compiles with `rustc`
- ✅ Compiled programs produce correct output

### Phase 4-5 (Data Structures + Runtime)
- ✅ Arrays, hashmaps, tuples working
- ✅ Runtime library complete
- ✅ Can compile complex programs (sorting, data processing)

### Phase 6 (Integration)
- ✅ End-to-end pipeline working
- ✅ Performance benchmark showing 10-100x speedup over interpreter
- ✅ Documentation complete
- ✅ Examples in `examples/compiler/` directory

---

## Non-Goals (For This Phase)

- ❌ **Type inference beyond Rust's defaults**: Use Rust's built-in type inference
- ❌ **Advanced optimizations**: Let `rustc` handle optimization
- ❌ **Garbage collection**: Use Rust's ownership (Rc/Arc where needed)
- ❌ **Incremental compilation**: Compile entire program each time
- ❌ **Error recovery**: Stop on first error
- ❌ **IDE integration**: No LSP, just CLI

---

## Roadmap Integration

These tickets will be added to `roadmap.yaml`:

```yaml
- phase: "Phase 6: Compiler/Transpiler"
  tickets:
    - id: COMPILE-001
      name: "Module structure and API"
      status: pending

    - id: COMPILE-002
      name: "Basic expression codegen"
      status: pending

    # ... (COMPILE-003 through COMPILE-018)
```

---

## Quick Start (COMPILE-001)

**Step 1**: Create module structure
```bash
mkdir -p src/compiler
touch src/compiler/mod.rs
touch src/compiler/codegen.rs
```

**Step 2**: Define public API
```rust
// src/compiler/mod.rs
pub mod codegen;

pub use codegen::CodeGenerator;

#[derive(Debug)]
pub enum CompileError {
    ParseError(String),
    CodeGenError(String),
}
```

**Step 3**: Write first test
```rust
// tests/test_compile_001_module.rs
use ruchyruchy::compiler::CodeGenerator;

#[test]
fn test_codegen_exists() {
    let _codegen = CodeGenerator::new();
}
```

**Step 4**: Implement minimal `CodeGenerator`
```rust
// src/compiler/codegen.rs
pub struct CodeGenerator {
    output: String,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self {
            output: String::new(),
        }
    }
}
```

**Step 5**: Commit with TDD
```bash
cargo test test_compile_001
git add src/compiler/ tests/test_compile_001_module.rs
git commit -m "COMPILE-001: Create compiler module structure"
```

---

**Status**: 📋 Plan complete, ready for implementation
**Next**: COMPILE-001 (Module structure)
**Methodology**: EXTREME TDD with 100% test coverage
