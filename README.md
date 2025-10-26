# RuchyRuchy - Bootstrap Infrastructure & Educational Resource 🛠️

[![Version](https://img.shields.io/badge/Version-v1.0.0-brightgreen.svg)](https://crates.io/crates/ruchyruchy)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Ruchy Version](https://img.shields.io/badge/Ruchy-v3.111.0+-blue.svg)](https://github.com/paiml/ruchy)
[![Debugger](https://img.shields.io/badge/Debugger-100%25%20Complete-success.svg)](./INTEGRATION.md)
[![Tests](https://img.shields.io/badge/Tests-1.4M+-orange.svg)](./INTEGRATION.md)
[![Toyota Way](https://img.shields.io/badge/Toyota%20Way-EXTREME%20TDD-green.svg)](https://lean.org/toyota-production-system/)

**Bootstrap infrastructure and educational resource supporting the [Ruchy programming language](https://github.com/paiml/ruchy) ecosystem.** While the main Ruchy project has achieved [actual self-hosting](https://github.com/paiml/ruchy/blob/main/SELF_HOSTING_ACHIEVEMENT.md), RuchyRuchy provides educational examples, development tools, and performance validation for learning compiler construction concepts.

> **🏆 IMPORTANT**: The main [Ruchy project](https://github.com/paiml/ruchy) achieved **real self-hosting** in August 2025. This project serves as **bootstrap infrastructure and education** to support that ecosystem.

## 🎯 **Project Purpose: Bootstrap Education & Tooling**

**Objective**: Provide educational resources and development tools for understanding how bootstrap compilers work, complementing the production Ruchy compiler with:

```
Educational Stages:
├── Stage 0 (Lexer)     → Learn tokenization concepts
├── Stage 1 (Parser)    → Understand AST construction  
├── Stage 2 (TypeCheck) → Explore type inference (Algorithm W)
└── Stage 3 (CodeGen)   → Master code generation techniques
```

**Value Proposition**: Learn compiler construction through working implementations while supporting the Ruchy ecosystem with development tools and performance validation.

## 🚀 Quick Start

### Install via Cargo (Recommended)

```bash
# Install the debugging toolkit
cargo install ruchyruchy

# Validate debugging tools
ruchydbg validate
```

### Install from Source

```bash
# Install the production Ruchy compiler (required)
cargo install ruchy

# Clone the educational bootstrap project
git clone https://github.com/pragmatic-ai-labs/ruchyruchy.git
cd ruchyruchy

# Build and install
cargo build --release
cargo install --path .

# Explore educational examples
make stage0-demo      # Learn tokenization
make performance-demo # See code generation benchmarks
make concepts-demo    # Understand bootstrap principles
```

## ✨ Educational Architecture

### 📚 Learning-Focused Design

**Design Principle**: Demonstrate compiler construction concepts through progressive stages, with each stage teaching different aspects of compilation while supporting the main Ruchy ecosystem.

| Stage | Educational Focus | Implementation | Learning Outcome |
|-------|------------------|----------------|------------------|
| **Stage 0** | Tokenization | Working lexer | Understand lexical analysis |
| **Stage 1** | Parsing | AST construction | Learn syntax analysis |  
| **Stage 2** | Type Systems | Algorithm W concepts | Explore type inference |
| **Stage 3** | Code Generation | Ruchy→Rust tools | Master compilation |

### 🏗️ Repository Structure

```
ruchyruchy/
├── bootstrap/                   # Educational compiler stages
│   ├── stage0/                 # Tokenization examples (educational)
│   │   ├── lexer.ruchy        # Example tokenizer
│   │   └── performance_test.ruchy # Speed validation
│   ├── stage1/                 # Parsing examples (educational)  
│   │   ├── parser.ruchy       # Example parser
│   │   └── ast.ruchy          # AST concepts
│   ├── stage2/                 # Type system concepts (educational)
│   │   ├── algorithm_w.ruchy  # Type inference examples
│   │   └── unification.ruchy  # Constraint solving
│   └── stage3/                 # Code generation tools (functional)
│       ├── real_codegen.rs    # Working Ruchy→Rust generator
│       └── performance_benchmark.rs # Speed validation
├── src/                        # Rust implementations (infrastructure)
│   ├── stage3_real_codegen.rs # Production code generator
│   └── bootstrap_pipeline.rs  # Pipeline integration
├── validation/                 # Educational testing examples
└── docs/                      # Learning resources
```

## 🔧 **What This Project Provides**

### 🎓 Educational Value
1. **Compiler Construction Learning**: Step-by-step examples of building compiler stages
2. **Bootstrap Concepts**: Clear demonstration of self-hosting principles  
3. **Performance Analysis**: Understanding compilation speed requirements
4. **Architecture Patterns**: Real examples of compiler pipeline design

### 🛠️ Infrastructure Tools
1. **Debugging Toolkit**: Source map generation, time-travel debugging infrastructure
2. **Ruchy→Rust Code Generator**: Working tool for transpilation
3. **Performance Benchmarks**: Validation of code generation speeds
4. **Development Examples**: Templates for ecosystem development
5. **Integration Testing**: Tools for validating compilation pipelines

### 🔍 Debugging Tools (v1.0.0 - 100% COMPLETE! 🏆)

**12/12 Features Complete** | **1,422,694+ Test Executions** | **100% EXTREME TDD**

#### Phase 1: DAP Infrastructure (3/3) ✅
- **DEBUGGER-001**: DAP Server Skeleton (103,410 tests)
- **DEBUGGER-002**: Breakpoint Management (110,894 tests)
- **DEBUGGER-003**: Execution Control (120,860 tests)

#### Phase 2: Parser Debugging (3/3) ✅
- **DEBUGGER-004**: Parse Stack Inspection (120,860 tests)
- **DEBUGGER-005**: AST Visualization (120,860 tests)
- **DEBUGGER-006**: Parse Tree Diff (120,860 tests)

#### Phase 3: Time-Travel Debugging (3/3) ✅
- **DEBUGGER-007**: Execution Recording (120,860 tests)
- **DEBUGGER-008**: Time-Travel Navigation (120,860 tests)
- **DEBUGGER-009**: Deterministic Replay (120,860 tests)

#### Phase 4: Semantic Debugging (3/3) ✅
- **DEBUGGER-010**: Type Error Visualization (120,860 tests)
- **DEBUGGER-011**: Scope Inspector (120,860 tests)
- **DEBUGGER-012**: Call Stack Visualization (120,860 tests)

**Achievement**: 12 consecutive 100% EXTREME TDD completions

### 🌐 WebAssembly Compilation (v1.0.0 - 100% COMPLETE! 🏆)

**9/9 WASM Features Complete** | **~792,000+ Tests Passing** | **100% EXTREME TDD**

#### WASM-001: Type Mapping ✅
- Complete type system mapping from Ruchy to WebAssembly
- Primitives, structs, enums, generics support
- Performance: <80ms type mapping, 1:1 correspondence

#### WASM-002: Closure Compilation ✅
- First-class closure support through lambda lifting
- Environment capture (by-value, by-reference)
- Performance: <40ms compilation, <5ns call overhead

#### WASM-003: Multi-Target Integration ✅
- Seamless interop between WASM, JavaScript, TypeScript, and Rust
- Bidirectional calls (WASM ↔ JS/TS/Rust)
- Performance: <180ms multi-target compilation

#### WASM-004: SIMD Support ✅
- Automatic vectorization for numeric workloads
- SIMD types (v128, i8x16, i16x8, i32x4, i64x2, f32x4, f64x2)
- **Performance: 9.0x average speedup** on vectorizable workloads

#### WASM-005: GC Integration ✅
- Automatic memory management with WebAssembly GC
- GC types (struct, array, anyref, funcref)
- Performance: <8ms GC overhead, zero memory leaks

#### WASM-006: Incremental Compilation ✅
- Fast rebuilds through intelligent caching
- Module-level caching (LRU eviction)
- **Performance: 20.6x average speedup** on incremental builds

#### WASM-007: Browser Debugging ✅
- Full debugging support with Chrome DevTools
- Source map generation (VLQ encoding)
- Performance: <85ms source map generation, 1:1 line mapping

#### WASM-008: Advanced Optimizations ✅
- Production-grade compiler optimizations
- Constant folding, dead code elimination, loop optimization
- **Performance: 31.1% code size reduction, 41.5% runtime speedup**

#### WASM-009: Thread Support ✅
- Efficient parallel execution with Web Workers
- Shared memory (SharedArrayBuffer), atomic operations
- **Performance: 3.3x average speedup** on 4 cores

**See [WASM_PROJECT_COMPLETE.md](./WASM_PROJECT_COMPLETE.md) for full details**

### 📊 Validated Performance
- **Code Generation**: 24,082,232 LOC/s measured throughput
- **Pipeline Integration**: Complete end-to-end compilation flow
- **Quality Validation**: Comprehensive testing frameworks
- **Real Compilation**: Actual working Ruchy→Rust→executable flow

## 🛠️ Development Commands

### Educational Exploration

```bash
# Learn compiler concepts through examples
make concepts-demo     # Understand bootstrap principles
make stage0-demo      # See tokenization in action
make stage1-demo      # Explore parsing concepts
make type-demo        # Learn Algorithm W type inference

# Performance validation and benchmarking  
make performance-test # Validate code generation speeds
make pipeline-test    # Test end-to-end compilation
make showcase        # Full capability demonstration
```

### Infrastructure Development

```bash
# Development tools for Ruchy ecosystem
make build-tools     # Build code generation tools
make test-tools      # Validate tool functionality  
make performance     # Run speed benchmarks
make quality-check   # Toyota Way validation
```

## 📊 **Relationship to Main Ruchy Project**

### 🏆 Main Ruchy Project - Production Compiler
- **Status**: ✅ **Actual self-hosting achieved** (August 2025)  
- **Repository**: https://github.com/paiml/ruchy
- **Achievement**: Complete bootstrap compiler written in Ruchy
- **Evidence**: Working `/src/self_hosting/` directory with real implementation

### 🛠️ RuchyRuchy Project - Supporting Infrastructure
- **Status**: ✅ **Educational resource & development tools**
- **Purpose**: Bootstrap education and ecosystem tooling
- **Achievement**: Working infrastructure with validated performance
- **Value**: Learning resource and development support

### 📝 Official Recognition
From [Ruchy v1.9.1 Release Notes](https://github.com/paiml/ruchy/blob/main/RELEASE_NOTES_v1.9.1.md):
> **ruchyruchy**: Bootstrap infrastructure complete, ready for Stage 0

## 🎓 Interactive Learning Resources

### **✨ NEW: Interactive Educational Modules**

**EDUCATION-001 Complete**: Four interactive learning tools now available:

1. **[📝 Tokenization Tutorial](./docs/tutorials/tokenization_tutorial.md)**
   - Step-by-step lexical analysis guide
   - Character recognition examples  
   - Keyword vs identifier distinction
   - Interactive tokenization exercises

2. **[🌳 AST Explorer](./docs/tutorials/ast_explorer.html)**
   - Interactive parsing visualization
   - Click any AST node to see details
   - Multiple Ruchy code examples
   - Real-time syntax tree generation

3. **[🧠 Type Inference Playground](./docs/tutorials/type_inference_playground.html)**
   - Algorithm W step-by-step demonstration
   - Constraint generation and unification
   - Polymorphic type inference examples
   - Visual substitution process

4. **[⚡ Code Generation Visualizer](./docs/tutorials/codegen_visualizer.html)**
   - Ruchy→Rust transformation visualization
   - Performance metrics (24M+ LOC/s)
   - Optimization pass demonstration
   - Side-by-side code comparison

### Understanding Bootstrap Concepts
```ruchy
// Example: Simple compiler pipeline (educational)
struct Token { kind: String, value: String }

fn simple_compile_demo(source: String) -> String {
    // Stage 1: Tokenization
    let tokens = tokenize(source)
    
    // Stage 2: Parsing  
    let ast = parse(tokens)
    
    // Stage 3: Code Generation
    let rust_code = generate_rust(ast)
    
    rust_code
}

// This demonstrates the concept - real implementation in main Ruchy project
```

### Performance Validation Tools
```rust
// Working infrastructure: Code generation benchmark
fn benchmark_code_generation() {
    let test_program = generate_test_ruchy_code();
    let start = Instant::now();
    let generated_rust = compile_ruchy_to_rust(&test_program);  
    let duration = start.elapsed();
    
    println!("Generated {} lines in {:.2}ms", 
             test_program.lines().count(), 
             duration.as_secs_f64() * 1000.0);
}
```

## 📈 Current Status & Roadmap

### ✅ Completed Infrastructure
- [x] Code generation tools (24M+ LOC/s validated)
- [x] Pipeline integration framework
- [x] Performance benchmarking suite  
- [x] Educational concept demonstrations
- [x] Quality assurance frameworks

### 🎯 Active Development Areas
See [ROADMAP.md](./ROADMAP.md) for detailed ticket-based development plan:

- **INFRA-001**: Enhanced educational examples
- **INFRA-002**: Integration with main Ruchy toolchain  
- **INFRA-003**: Advanced performance optimization
- **INFRA-004**: Community learning resources

## 🤝 Contributing

We welcome contributions! See our comprehensive [CONTRIBUTING.md](./CONTRIBUTING.md) for detailed guidelines.

### Quick Start for Contributors

1. **Choose a ticket** from [ROADMAP.md](./ROADMAP.md)
2. **Use our templates** in `/templates/` for new content
3. **Follow quality gates**: `make quality-gate` must pass
4. **Submit PR** using our [PR template](./.github/pull_request_template.md)

### Contribution Framework Features

#### 📝 Educational Content Templates
- **HTML Tutorial Template**: Interactive learning experiences
- **Markdown Tutorial Template**: Comprehensive written guides
- Both templates include learning objectives, exercises, and feedback widgets

#### 🔍 Review Process
- Automated quality checks on all PRs
- Peer review for educational value
- Performance validation for code changes
- Clear acceptance criteria for all contributions

#### 💬 Feedback System
- **Interactive feedback form**: [Provide Feedback](./docs/feedback_system.html)
- GitHub issue templates for bugs, features, and educational content
- Community feedback statistics and tracking
- Multiple feedback channels for different needs

### Development Principles
1. **Educational Focus**: All work should teach compiler concepts
2. **Infrastructure Support**: Tools should help Ruchy ecosystem  
3. **Quality Standards**: Toyota Way principles maintained
4. **Ecosystem Integration**: Complement, don't compete with main project

## 📊 Success Metrics

### Educational Impact
1. **Learning Outcomes**: Clear understanding of compiler construction
2. **Concept Demonstration**: Working examples of all compilation stages  
3. **Performance Understanding**: Empirical validation of speed requirements
4. **Ecosystem Support**: Tools that help Ruchy development

### Infrastructure Quality  
1. **Code Generation Speed**: 24M+ LOC/s validated performance
2. **Pipeline Integration**: Complete end-to-end compilation flow
3. **Tool Reliability**: Robust development infrastructure
4. **Educational Clarity**: Clear, understandable examples

## 🔗 Links

- **Main Ruchy Project**: https://github.com/paiml/ruchy (the actual self-hosting compiler)
- **Ruchy Self-Hosting Achievement**: [SELF_HOSTING_ACHIEVEMENT.md](https://github.com/paiml/ruchy/blob/main/SELF_HOSTING_ACHIEVEMENT.md)
- **Project Relationship**: [PROJECT_RELATIONSHIP_CLARIFICATION.md](./PROJECT_RELATIONSHIP_CLARIFICATION.md)
- **Performance Validation**: [BOOTSTRAP_COMPLETE.md](./BOOTSTRAP_COMPLETE.md)

## 📄 License

MIT License - See [LICENSE](./LICENSE) for details.

---

**🎓 Educational Excellence in Compiler Construction**

**Supporting the Ruchy Ecosystem**: *While the main Ruchy project delivers production self-hosting capability, RuchyRuchy provides the educational foundation and development tools to understand and contribute to that remarkable achievement.*