# WASM Compilation Target Research

**Date**: 2025-10-22
**Status**: Research Complete - Ready for Implementation
**Context**: Extending RuchyRuchy bootstrap compiler to support WebAssembly as a third compilation target

---

## Executive Summary

**Question**: If we can compile Ruchy to Ruchy (self-hosting), why not Ruchy to WASM too?

**Answer**: **We absolutely can!** Our existing code generation infrastructure (BOOTSTRAP-014, 015, 016) provides the perfect foundation. WASM is a natural third target alongside TypeScript and Rust.

**Feasibility**: **HIGH** ✅
- Existing multi-target architecture ready
- Parallel emitter pattern established
- AST structure supports all necessary constructs
- Self-generation infrastructure in place

**Effort**: **Medium** (comparable to adding Rust target)
- Follow established TypeScript/Rust emitter patterns
- Map Ruchy AST to WAT S-expressions
- Integrate with existing test infrastructure
- Add to multi-target validation

**Value**: **HIGH**
- Run Ruchy in browsers
- Portable binary format
- Near-native performance
- Cross-platform deployment
- Demonstration of compiler capabilities

---

## Current State Analysis

### Existing Code Generation Infrastructure

**Targets Currently Supported**:
1. **TypeScript** (BOOTSTRAP-014) - 10/10 tests ✅
2. **Rust** (BOOTSTRAP-015) - 10/10 tests ✅

**Architecture**:
```
Source Code → Lexer → Parser → Type Checker → Code Generator → [TypeScript|Rust|WASM?]
```

**Key Files**:
- `bootstrap/stage3/typescript_emitter.ruchy` - TypeScript code generator
- `bootstrap/stage3/rust_emitter.ruchy` - Rust code generator
- `bootstrap/stage3/multi_target_validation.ruchy` - Multi-target consistency
- `bootstrap/stage3/pipeline_integration.ruchy` - End-to-end compilation

**Code Generation Pattern**:
```ruchy
// Each target has parallel emitters
fun emit_expr_ts(expr: Expr) -> String { ... }
fun emit_expr_rust(expr: Expr) -> String { ... }
// Would add:
fun emit_expr_wasm(expr: Expr) -> String { ... }  // WAT text format

fun emit_stmt_ts(stmt: Stmt) -> String { ... }
fun emit_stmt_rust(stmt: Stmt) -> String { ... }
// Would add:
fun emit_stmt_wasm(stmt: Stmt) -> String { ... }  // WAT text format
```

**AST Structure** (from Stage 1):
```ruchy
enum Expr {
    Number(String),           // 42
    Identifier(String),       // x
    StringLit(String),        // "hello"
    BoolTrue,                 // true
    BoolFalse,                // false
    Binary(BinOp, Box<Expr>, Box<Expr>),  // 1 + 2
    Unary(UnOp, Box<Expr>),              // -x
    Group(Box<Expr>)                     // (expr)
}

enum Stmt {
    SExpr(Expr),              // expr;
    SLet(String, Expr),       // let x = expr;
    SFun(String, String, Expr) // fun f(x) { expr }
}
```

---

## WebAssembly (WASM) Analysis

### What is WebAssembly?

**WebAssembly** is a binary instruction format for a stack-based virtual machine designed as a portable compilation target for programming languages.

**Key Characteristics**:
- **Binary format** (.wasm) - compact, fast to load
- **Text format** (.wat) - human-readable S-expressions
- **Stack machine** - instructions push/pop values
- **Typed** - i32, i64, f32, f64
- **Portable** - runs in browsers and standalone runtimes

### WAT (WebAssembly Text Format)

**WAT Syntax**: S-expressions (like Lisp)

**Module Structure**:
```wat
(module
  (func $add (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.add
  )
  (export "add" (func $add))
)
```

**Key Concepts**:

1. **Stack Machine**:
   - `local.get $x` - push x onto stack
   - `i32.add` - pop 2 values, push sum
   - `i32.const 42` - push constant 42

2. **Functions**:
   ```wat
   (func $name (param $p i32) (result i32)
     local.get $p
     i32.const 1
     i32.add
   )
   ```

3. **Local Variables**:
   ```wat
   (func (param $x i32) (local $temp i32)
     i32.const 10
     local.set $temp
     local.get $temp
   )
   ```

4. **Control Flow**:
   ```wat
   (if (result i32)
     (local.get $condition)
     (then
       (i32.const 1))
     (else
       (i32.const 0)))
   ```

5. **Types**:
   - `i32` - 32-bit integer
   - `i64` - 64-bit integer
   - `f32` - 32-bit float
   - `f64` - 64-bit float

---

## Mapping Ruchy to WASM

### Translation Strategy

**Approach**: Generate WAT (text format), then compile to binary WASM

**Rationale**:
- WAT is human-readable and debuggable
- Easy to test and validate
- Tools exist to convert WAT → WASM binary
- Follows pattern of TypeScript/Rust emitters (generate text)

### AST to WAT Mapping

#### 1. Expressions

**Numbers**:
```ruchy
// Ruchy AST
Expr::Number("42")

// WAT output
"(i32.const 42)"
```

**Binary Operations**:
```ruchy
// Ruchy AST
Binary(Add, Box<Number("1")>, Box<Number("2")>)

// WAT output (stack-based)
"(i32.const 1)
 (i32.const 2)
 (i32.add)"

// Or more readable:
"(i32.add
   (i32.const 1)
   (i32.const 2))"
```

**Identifiers (Variables)**:
```ruchy
// Ruchy AST
Expr::Identifier("x")

// WAT output
"(local.get $x)"
```

**Lambda Expressions**:
```ruchy
// Ruchy: (x) => x + 1
// AST: ELam("x", Binary(Add, Identifier("x"), Number("1")))

// WAT: Define as separate function
"(func $lambda_0 (param $x i32) (result i32)
   (i32.add
     (local.get $x)
     (i32.const 1)))"
```

#### 2. Statements

**Let Bindings**:
```ruchy
// Ruchy: let x = 42;
// AST: SLet("x", Number("42"))

// WAT:
"(local $x i32)
 (local.set $x (i32.const 42))"
```

**Function Definitions**:
```ruchy
// Ruchy: fun add(x, y) { x + y }
// AST: SFun("add", "x", ELam("y", Binary(Add, ...)))

// WAT:
"(func $add (param $x i32) (param $y i32) (result i32)
   (i32.add
     (local.get $x)
     (local.get $y)))"
```

#### 3. Control Flow

**If Expressions**:
```ruchy
// Ruchy: if x > 0 { 1 } else { -1 }

// WAT:
"(if (result i32)
   (i32.gt_s (local.get $x) (i32.const 0))
   (then (i32.const 1))
   (else (i32.const -1)))"
```

### Type Mapping

| Ruchy Type | WASM Type | Notes |
|------------|-----------|-------|
| i32 | i32 | Direct mapping |
| i64 | i64 | Direct mapping |
| bool | i32 | 0=false, 1=true |
| String | i32 | Linear memory address |
| Lambda | funcref | Function reference |

---

## Implementation Strategy

### Phase 1: Foundation (BOOTSTRAP-018)

**Objective**: Create basic WASM emitter following TypeScript/Rust pattern

**Deliverables**:
1. `bootstrap/stage3/wasm_emitter.ruchy` - WASM code generator
2. Functions:
   - `emit_expr_wasm(expr: Expr) -> String` - Expression emission
   - `emit_stmt_wasm(stmt: Stmt) -> String` - Statement emission
   - `emit_module_wasm(stmts: Vec<Stmt>) -> String` - Module wrapper

**Test Suite** (10 tests, mirroring TypeScript/Rust):
- Number literals
- Binary operations (+, -, *, /)
- Variables (local.get/set)
- Basic functions
- Function calls
- Let bindings
- Lambda expressions
- If expressions
- String literals
- Boolean values

**EXTREME TDD Phases**:
1. **RED**: Write 10 failing tests (2-4 passing initially)
2. **GREEN**: Minimal implementation to pass all tests
3. **REFACTOR**: Clean up, optimize, document
4. **TOOL**: ruchy check + lint validation

### Phase 2: Multi-Target Integration (BOOTSTRAP-019)

**Objective**: Integrate WASM into existing multi-target validation

**Updates**:
1. Extend `multi_target_validation.ruchy`:
   - Add WASM to semantic equivalence tests
   - Validate WASM output matches TypeScript/Rust behavior
   - Test all three targets produce same results

2. Extend `pipeline_integration.ruchy`:
   - Add `compile_to_wasm(source: String) -> String` function
   - Full pipeline: Source → Parse → TypeCheck → CodeGen (WASM)
   - Validate end-to-end compilation

**Tests**: 5 multi-target tests (all 3 targets consistent)

### Phase 3: Advanced Features (BOOTSTRAP-020)

**Objective**: Add WASM-specific optimizations and features

**Features**:
1. **Memory Management**: Linear memory for strings/data
2. **Imports/Exports**: Module interface definitions
3. **Tables**: Function references for higher-order functions
4. **Optimizations**: Leverage WASM's efficient stack operations

**Advanced Constructs**:
- Closures with captured variables
- Recursive functions
- Pattern matching compilation
- Tail-call optimization (if available)

### Phase 4: Self-Compilation (BOOTSTRAP-021)

**Objective**: Compile RuchyRuchy compiler to WASM

**Goal**: `ruchyruchy.wasm` - Browser-based Ruchy compiler

**Steps**:
1. Compile lexer to WASM
2. Compile parser to WASM
3. Compile type checker to WASM
4. Compile code generator (including WASM emitter!) to WASM
5. Full compiler running in WebAssembly

**Demo**:
```html
<!-- Browser-based Ruchy compiler -->
<script type="module">
  import init, { compile } from './ruchyruchy.wasm';
  await init();

  const ruchy_code = "fun add(x, y) { x + y }";
  const typescript_output = compile(ruchy_code, "typescript");
  const rust_output = compile(ruchy_code, "rust");
  const wasm_output = compile(ruchy_code, "wasm");

  console.log("TypeScript:", typescript_output);
  console.log("Rust:", rust_output);
  console.log("WASM:", wasm_output);
</script>
```

---

## Technical Considerations

### 1. Stack Machine vs Register Machine

**Challenge**: WASM is stack-based, TypeScript/Rust are not

**Solution**: Generate S-expression form which handles stack implicitly
```wat
(i32.add (i32.const 1) (i32.const 2))
; Equivalent to:
; push 1
; push 2
; add
; result on stack
```

### 2. Lambda Compilation

**Challenge**: WASM doesn't have first-class lambda literals

**Solution**: Lift lambdas to top-level functions
```ruchy
// Ruchy
let f = (x) => x + 1;
f(42)

// WASM
(func $lambda_0 (param $x i32) (result i32)
  (i32.add (local.get $x) (i32.const 1)))

(func $main
  (local $f i32)  ; function reference
  (local.set $f (i32.const 0))  ; index of $lambda_0
  (call $lambda_0 (i32.const 42)))
```

### 3. String Handling

**Challenge**: WASM doesn't have string type

**Solutions**:
- **Option A**: Use linear memory + length prefix
- **Option B**: Import JavaScript string functions
- **Option C**: Use reference types (recent WASM extension)

**Recommended**: Option B for initial implementation (simpler)

### 4. Garbage Collection

**Challenge**: WASM is manual memory management

**Solutions**:
- **Option A**: Reference counting
- **Option B**: Import GC from host (JavaScript)
- **Option C**: Use WASM GC proposal (when stable)

**Recommended**: Option B initially, Option C long-term

### 5. Type System Integration

**Challenge**: Map Ruchy's type system to WASM's limited types

**Strategy**:
- Primitives (i32, i64, f32, f64) → direct mapping
- Booleans → i32 (0=false, 1=true)
- Strings → i32 (memory address or JS import)
- Functions → funcref or i32 (table index)
- Structs → encode in linear memory

---

## Comparison with Existing Targets

| Feature | TypeScript | Rust | WASM (Proposed) |
|---------|-----------|------|-----------------|
| **Output Format** | Text (.ts) | Text (.rs) | Text (.wat) → Binary (.wasm) |
| **Execution** | Node/Browser | Native | Browser/Wasm runtime |
| **Type System** | Dynamic | Static | Static (4 types) |
| **GC** | Yes | No (ownership) | Host-provided or manual |
| **Lambdas** | Native | Closures | Lifted functions |
| **Strings** | Native | String type | Linear memory |
| **Performance** | JIT | Native | Near-native |
| **Interop** | JavaScript | C FFI | JavaScript/C |
| **Debugging** | Good | Excellent | Growing |

**WASM Advantages**:
- ✅ Portable binary format
- ✅ Near-native performance
- ✅ Sandboxed execution
- ✅ Browser-native support
- ✅ Cross-platform

**WASM Challenges**:
- ⚠️ Limited type system
- ⚠️ Manual memory management
- ⚠️ No native string type
- ⚠️ Requires compilation step (WAT → binary)

---

## Roadmap

### Immediate (While Waiting for std::time)

**BOOTSTRAP-018: WASM Emitter Foundation**
- **Effort**: 2-3 days
- **Tests**: 10 baseline tests
- **Phases**: RED-GREEN-REFACTOR-TOOL
- **Deliverable**: `bootstrap/stage3/wasm_emitter.ruchy`

**BOOTSTRAP-019: Multi-Target Integration**
- **Effort**: 1-2 days
- **Tests**: 5 multi-target validation tests
- **Deliverable**: WASM added to validation suite

### Near-Term (Post-Implementation)

**BOOTSTRAP-020: Advanced WASM Features**
- **Effort**: 3-4 days
- **Features**: Memory, imports/exports, tables
- **Tests**: 15 advanced feature tests

**BOOTSTRAP-021: Self-Compilation to WASM**
- **Effort**: 1 week
- **Goal**: `ruchyruchy.wasm` - compiler running in browser
- **Demo**: Interactive web-based Ruchy compiler

### Long-Term (Future Enhancements)

**WASM Optimizations**:
- Tail-call optimization
- SIMD instructions
- Thread support (WASM threads proposal)
- GC integration (WASM GC proposal)

**Tooling**:
- Source maps for debugging
- WASM binary optimizer integration
- Performance profiling
- Size optimization

---

## Example: Complete Translation

### Input (Ruchy)
```ruchy
fun factorial(n) {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

let result = factorial(5)
```

### Output (WAT)
```wat
(module
  ;; Factorial function (recursive)
  (func $factorial (param $n i32) (result i32)
    (if (result i32)
      ;; Condition: n <= 1
      (i32.le_s (local.get $n) (i32.const 1))
      ;; Then: return 1
      (then
        (i32.const 1))
      ;; Else: return n * factorial(n - 1)
      (else
        (i32.mul
          (local.get $n)
          (call $factorial
            (i32.sub (local.get $n) (i32.const 1)))))))

  ;; Main function
  (func $main (result i32)
    (local $result i32)
    (local.set $result
      (call $factorial (i32.const 5)))
    (local.get $result))

  ;; Export main for execution
  (export "main" (func $main))
  (export "factorial" (func $factorial))
)
```

### Compilation & Execution
```bash
# 1. Compile Ruchy to WAT
ruchy compile --target wasm factorial.ruchy -o factorial.wat

# 2. Convert WAT to WASM binary (using wabt tools)
wat2wasm factorial.wat -o factorial.wasm

# 3. Run in Node.js or browser
node -e "
  const fs = require('fs');
  const bytes = fs.readFileSync('factorial.wasm');
  WebAssembly.instantiate(bytes).then(module => {
    console.log('factorial(5) =', module.instance.exports.main());
  });
"
# Output: factorial(5) = 120
```

---

## Benefits

### For RuchyRuchy Project

1. **Demonstrates Compiler Capabilities**
   - Shows mastery of multiple compilation targets
   - Highlights flexibility of architecture
   - Proves concept: "compile Ruchy to anything"

2. **Educational Value**
   - Teaches WASM compilation
   - Demonstrates stack machine code generation
   - Shows low-level code generation techniques

3. **Practical Applications**
   - Run Ruchy in browsers (no server needed)
   - Portable binary distribution
   - Cross-platform execution
   - Integration with web technologies

4. **Research Opportunities**
   - Performance comparison: TypeScript vs Rust vs WASM
   - Optimization techniques for functional languages → WASM
   - Memory management strategies
   - GC integration patterns

### For Ruchy Language

1. **Expanded Target Platforms**
   - Web browsers (Chrome, Firefox, Safari)
   - Server-side (Node.js, Deno, Bun)
   - Embedded systems (WASM runtimes)
   - Mobile (via browser)

2. **Performance**
   - Near-native execution speed
   - AOT compilation benefits
   - Sandboxed security

3. **Distribution**
   - Single binary format
   - No runtime dependencies
   - Easy deployment

---

## Known Issues & Mitigations

### Issue: Ruchy Compiler WASM Build

**Problem**: Documented in `docs/issues/GITHUB_ISSUE_WASM_BUILD.md`
- Ruchy compiler itself has non-WASM code not feature-gated
- Workaround: Use published crates.io version

**Impact on This Work**: **NONE**
- We're targeting WASM as an *output* format
- We're not compiling *Ruchy compiler* to WASM (yet)
- Our WASM emitter generates WAT text, runs on native Ruchy

**Future**: BOOTSTRAP-021 (Self-Compilation) will address this

### Issue: Missing std::time

**Problem**: Cannot measure actual compilation times (GitHub issue #55)
- Blocks optimization work
- Does not block WASM implementation

**Mitigation**: WASM work can proceed in parallel

---

## Next Steps

### Immediate Actions

1. **Create BOOTSTRAP-018 Ticket**
   - Title: "WASM Emitter Foundation"
   - Description: Basic WASM code generator
   - Tests: 10 baseline tests (RED-GREEN-REFACTOR-TOOL)

2. **Start EXTREME TDD Implementation**
   - RED Phase: Write failing tests for WASM emission
   - Follow pattern from TypeScript/Rust emitters
   - Target: 2-4 tests passing in RED phase

3. **Update Book Documentation**
   - Add WASM chapter to Phase 3 (Bootstrap Compiler)
   - Document WASM compilation strategy
   - Include examples and rationale

### Success Criteria

**BOOTSTRAP-018 Complete** when:
- ✅ 10/10 tests passing (RED-GREEN-REFACTOR-TOOL)
- ✅ `emit_expr_wasm()` handles all expression types
- ✅ `emit_stmt_wasm()` handles all statement types
- ✅ Generated WAT compiles to valid WASM binary
- ✅ Output semantically equivalent to TypeScript/Rust

**Project Milestone** when:
- ✅ BOOTSTRAP-018, 019, 020, 021 all complete
- ✅ RuchyRuchy compiler itself running as `ruchyruchy.wasm`
- ✅ Browser-based demo working
- ✅ All tests passing (100+ WASM-specific tests)

---

## Conclusion

**Question**: Can we compile Ruchy to WASM?

**Answer**: **Absolutely YES!** ✅

**Why it makes sense**:
1. Natural extension of existing multi-target architecture
2. Follows established pattern (TypeScript, Rust emitters)
3. Leverages complete bootstrap infrastructure
4. High value: browser execution, portability, performance

**Feasibility**: **HIGH**
- Infrastructure ready
- Clear translation strategy
- Well-defined roadmap
- Manageable effort (comparable to Rust target)

**Timeline**:
- BOOTSTRAP-018 (Foundation): 2-3 days
- BOOTSTRAP-019 (Integration): 1-2 days
- BOOTSTRAP-020 (Advanced): 3-4 days
- BOOTSTRAP-021 (Self-Compilation): 1 week
- **Total: ~2 weeks** to full WASM support

**Recommendation**: **Proceed with implementation!**

This is excellent timing while waiting for std::time support. WASM work can proceed in parallel and demonstrates the power and flexibility of the RuchyRuchy bootstrap compiler infrastructure.

---

## References

- **WebAssembly Specification**: https://webassembly.github.io/spec/
- **MDN WAT Guide**: https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Understanding_the_text_format
- **WABT Tools**: https://github.com/WebAssembly/wabt
- **RuchyRuchy Bootstrap**: `bootstrap/stage3/`
- **TypeScript Emitter**: `bootstrap/stage3/typescript_emitter.ruchy`
- **Rust Emitter**: `bootstrap/stage3/rust_emitter.ruchy`
- **Multi-Target Validation**: `bootstrap/stage3/multi_target_validation.ruchy`
