# RuchyRuchy - Self-Hosting Compiler Bootstrap 🚀

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Ruchy Version](https://img.shields.io/badge/Ruchy-v1.8.0+-blue.svg)](https://github.com/ruchy-lang/ruchy)
[![Toyota Way](https://img.shields.io/badge/Toyota%20Way-Quality%20Built--in-green.svg)](https://lean.org/toyota-production-system/)
[![Bootstrap Progress](https://img.shields.io/badge/Bootstrap-Stage%200%2F4-orange.svg)](./docs/specifications/ruchy-ruchy-repo-spec.md)

**The world's first self-hosting programming language compiler with built-in formal verification and automatic BigO complexity analysis.** Empirical validation of Ruchy's self-compilation capability through incremental bootstrap stages, demonstrating complete language self-sufficiency.

## 🎯 **HISTORIC GOAL: Complete Self-Hosting Bootstrap**

**Objective**: Prove Ruchy's self-compilation capability through a **four-stage bootstrap sequence**, each stage compiling the next with increasing feature coverage:

```
Stage 0 (Lexer)     → 1K LOC  → Tokenizes itself
Stage 1 (Parser)    → 3K LOC  → Parses Stage 0+1
Stage 2 (TypeCheck) → 5K LOC  → Types Stage 0+1+2
Stage 3 (CodeGen)   → 6K LOC  → Compiles all stages
```

**Success Metric**: Stage 3 compiles the entire bootstrap compiler, producing **bit-identical Rust output** to the production Ruchy compiler.

## 🚀 Quick Start

```bash
# Install the Ruchy compiler
cargo install ruchy

# Clone the bootstrap project
git clone https://github.com/pragmatic-ai-labs/ruchyruchy.git
cd ruchyruchy

# Begin bootstrap development
make stage0         # Build Stage 0 (Lexer)
make validate      # Run quality gates
make test          # Test self-compilation
```

## ✨ Revolutionary Architecture

### 📊 Progressive Bootstrap Design

**Design Principle**: Each stage compiles the next with **increasing feature coverage**, providing **incremental validation** of Ruchy's self-compilation capability.

| Stage | Component | LOC | Capability | Self-Compilation Test |
|-------|-----------|-----|------------|----------------------|
| **Stage 0** | Lexer | 1K | Tokenizes itself | ✓ `./lexer < lexer.ruchy` |
| **Stage 1** | Parser | 3K | Parses Stage 0+1 | ✓ Roundtrip `parse(ast.emit()) == ast` |
| **Stage 2** | TypeCheck | 5K | Types all stages | ✓ Algorithm W with constraints |
| **Stage 3** | CodeGen | 6K | Compiles everything | ✓ Bit-identical Rust output |

### 🏗️ Repository Structure

```
ruchyruchy/
├── bootstrap/                   # Progressive compiler stages
│   ├── stage0/                 # Minimal tokenizer (1K LOC)
│   │   ├── lexer.ruchy        # Self-tokenizing lexer
│   │   └── token.ruchy        # Token definitions
│   ├── stage1/                 # Parser (3K LOC)
│   │   ├── parser.ruchy       # Pratt + recursive descent
│   │   └── ast.ruchy          # AST representation
│   ├── stage2/                 # Type inference (5K LOC)
│   │   ├── infer.ruchy        # Algorithm W implementation
│   │   └── unify.ruchy        # Constraint solver
│   └── stage3/                 # Code generation (6K LOC)
│       ├── emit.ruchy         # Rust code generation
│       └── opt.ruchy          # Peephole optimizer
├── validation/                 # Quality assurance
│   ├── differential.ruchy     # Output equivalence testing
│   ├── property.ruchy         # QuickCheck properties
│   └── bench.ruchy           # Performance regression
├── INTEGRATION.md             # Quality gate results
└── Makefile                   # Toyota Way automation
```

## 🔬 **Advanced Development Tools**

### Revolutionary Formal Verification Workflow

Every bootstrap stage leverages **Ruchy's unique advanced tooling capabilities**:

#### 1. Syntax Validation & AST Analysis
```bash
# Comprehensive AST inspection with metrics
ruchy ast bootstrap/stage0/lexer.ruchy --metrics --json
# → Complete syntax tree with complexity metrics
# → Cyclomatic complexity calculation  
# → Symbol usage analysis with unused detection
```

#### 2. Formal Verification & Correctness Proofs
```bash
# Mathematical correctness guarantees
ruchy provability bootstrap/stage1/parser.ruchy --verify --contracts
# → Function purity detection with side-effect analysis
# → Recursive function identification and complexity scoring
# → Mathematical proof of termination and correctness
```

#### 3. Performance Analysis & BigO Detection
```bash
# Automatic algorithmic complexity detection
ruchy runtime bootstrap/stage2/infer.ruchy --bigo --profile
# → Automatic BigO detection (O(1), O(n), O(n²), O(n³))
# → Performance bottleneck identification
# → Optimization scoring with specific recommendations
```

#### 4. Quality Scoring & Hardware Optimization
```bash
# Unified quality assessment with hardware-aware optimization
ruchy score bootstrap/stage3/emit.ruchy
ruchy optimize bootstrap/stage3/emit.ruchy --hardware-aware
# → Quality Score Report with confidence intervals
# → Hardware-specific optimization suggestions
```

### 🎯 Self-Compilation Validation Examples

#### Stage 0: Lexical Self-Analysis
```rust
// bootstrap/stage0/lexer.ruchy - Self-tokenizing lexer
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    current: char,
}

impl Lexer {
    pub fun scan(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            self.skip_whitespace();
            
            match self.current {
                '0'..='9' => tokens.push(self.number()),
                'a'..='z' | 'A'..='Z' | '_' => {
                    tokens.push(self.ident_or_keyword())
                }
                '"' => tokens.push(self.string()),
                '+' => tokens.push(self.single(Token::Plus)),
                _ => self.advance(), // Skip unknown
            }
        }
        
        tokens.push(Token::Eof);
        tokens
    }
}
```

**Validation**: `./lexer < lexer.ruchy` produces 500+ tokens in <10ms

#### Stage 1: Parser Self-Compilation
```rust
// bootstrap/stage1/parser.ruchy - Self-parsing parser
impl Parser {
    pub fun parse(&mut self) -> Result<Module, ParseError> {
        let mut items = Vec::new();
        
        while !self.is_at_end() {
            items.push(self.declaration()?);
        }
        
        Ok(Module { items })
    }
    
    // Pratt parser for expressions
    fun expression(&mut self, min_prec: i32) -> Result<Expr, ParseError> {
        let mut left = self.primary()?;
        
        while let Some(op) = self.peek_operator() {
            let prec = self.precedence(op);
            if prec < min_prec { break; }
            
            self.advance();
            let right = self.expression(prec + 1)?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        
        Ok(left)
    }
}
```

**Validation**: Roundtrip property `parse(ast.emit()) == ast`

## 🛠️ Development Commands

### Toyota Way Quality Gates

```bash
# Progressive bootstrap build
make stage0        # Build Stage 0 (Lexer) 
make stage1        # Build Stage 1 (Parser) - requires stage0
make stage2        # Build Stage 2 (Type Checker) - requires stage1
make stage3        # Build Stage 3 (Code Generator) - requires stage2

# Quality gates (MANDATORY - BLOCKING)
make validate      # Run PMAT quality checks (complexity ≤20)
make lint          # Zero clippy warnings allowed (-D warnings)
make test          # All self-compilation tests must pass
make coverage      # Minimum 80% test coverage
make complexity    # Ensure all functions <20 cyclomatic complexity

# Toyota Way continuous improvement
make analyze-complexity    # Find complexity hotspots  
make kaizen-refactor      # Generate improvement plan
make quality-report       # Generate comprehensive metrics
```

### Self-Compilation Workflow

```bash
# 1. Write Ruchy compiler features in Ruchy itself
echo 'struct Lexer { input: String, position: i32 }' > bootstrap/stage0/new_feature.ruchy

# 2. Test with enhanced type inference (Algorithm W)
ruchy run bootstrap/stage0/new_feature.ruchy

# 3. Validate with formal verification
ruchy provability bootstrap/stage0/new_feature.ruchy --verify --contracts

# 4. Transpile with minimal codegen for integration
ruchy transpile bootstrap/stage0/new_feature.ruchy --minimal --output integrated.rs

# 5. The self-hosting cycle is complete!
```

## 📊 **Quality Excellence Standards**

### Mandatory Performance Targets

| Stage | Metric | Target | Measurement |
|-------|--------|--------|-------------|
| **Lexer** | Throughput | >10K LOC/s | `hyperfine ./lexer < large.ruchy` |
| **Parser** | Throughput | >5K LOC/s | `hyperfine ./parser < large.ruchy` |
| **TypeCheck** | Complexity | O(n log n) | Empirical scaling test |
| **CodeGen** | Throughput | >10K LOC/s | `hyperfine ./codegen < typed.json` |

### Toyota Way Correctness Validation

```rust
// validation/differential.ruchy - Bit-identical output verification
pub fun validate_equivalence(source: &str) -> Result<(), DiffError> {
    // Compile with production Ruchy
    let expected = Command::new("ruchy")
        .arg("compile")
        .arg("-")
        .stdin(source)
        .output()?;
    
    // Compile with bootstrap compiler  
    let actual = Command::new("./ruchyruchy")
        .arg("compile")
        .arg("-")
        .stdin(source)
        .output()?;
    
    // Compare generated Rust - MUST BE IDENTICAL
    if expected.stdout != actual.stdout {
        return Err(DiffError::OutputMismatch);
    }
    
    // Both should compile with rustc
    let rust_check = Command::new("rustc")
        .arg("--emit=metadata")
        .stdin(actual.stdout)
        .status()?;
    
    if !rust_check.success() {
        return Err(DiffError::InvalidRust);
    }
    
    Ok(())
}
```

## 📈 Current Status

### 🔧 **Phase 0: Infrastructure Setup** (Current)
- [x] Repository structure established
- [x] Quality gates implemented (Toyota Way)
- [x] Specification completed (4-stage bootstrap)  
- [x] Development tooling configured
- [ ] **Stage 0 Implementation**: Minimal tokenizer in Ruchy
- [ ] Self-tokenization validation

### 🎯 **Upcoming Phases**

**Phase 1: Stage 0 - Lexical Foundation** (2-3 weeks)
- [ ] Token definitions (12 essential keywords)
- [ ] Scanner core implementation  
- [ ] Self-tokenization test: `./lexer < lexer.ruchy`
- [ ] Performance optimization (>10K LOC/s target)

**Phase 2: Stage 1 - Parser Bootstrap** (3-4 weeks) 
- [ ] AST definitions for compiler patterns
- [ ] Recursive descent + Pratt expression parser
- [ ] Self-parsing validation  
- [ ] Roundtrip testing: `parse(ast.emit()) == ast`

**Phase 3: Stage 2 - Type System** (4-5 weeks)
- [ ] Algorithm W type inference implementation
- [ ] Constraint solving with unification  
- [ ] Generalization and instantiation
- [ ] Self-type-checking validation

**Phase 4: Stage 3 - Code Generation** (3-4 weeks)  
- [ ] Rust AST mapping and emission
- [ ] Expression and pattern compilation
- [ ] Self-compilation test: bit-identical output
- [ ] Bootstrap fixpoint validation

## 🤝 Contributing

This project follows the **Toyota Way** methodology with zero-tolerance quality standards:

### Sacred Rules (BLOCKING)
1. **Never bypass quality gates** - `git commit --no-verify` is FORBIDDEN
2. **Always test self-compilation** - Every stage must compile itself
3. **Maintain complexity budget** - All functions <20 cognitive complexity
4. **Zero SATD tolerance** - No TODO/FIXME/HACK comments allowed
5. **Evidence-based development** - All claims backed by formal verification

### Contribution Workflow
```bash
# 1. Check current roadmap status
cat docs/execution/roadmap.md

# 2. Run quality gates before changes
make quality-gate

# 3. Implement following Toyota Way principles  
# 4. Validate all changes pass gates
make lint && make test && make complexity

# 5. Commit with task reference
git commit -m "BOOTSTRAP-XXX: Brief description

Validates: Specification Section X.Y
Performance: Within target bounds  
Complexity: Under 20 threshold
Self-Compilation: ✓ Passes validation"
```

## 📊 Success Metrics

### Historical Achievement Markers
1. **Bit-identical Rust Output**: Stage 3 generates identical code to production compiler
2. **Performance Parity**: <5% overhead vs hand-written Rust compiler
3. **Complexity Management**: All functions <20 cyclomatic complexity  
4. **Memory Efficiency**: Peak RSS <100MB for 10K LOC input
5. **Binary Compactness**: Stripped binary <5MB per stage
6. **Self-Sufficiency**: Zero dependencies on external Ruchy compiler

### Quality Validation Dashboard
```
Bootstrap Progress:    Stage 0/4 (0% complete)
Self-Compilation:     ❌ Not yet achieved
Performance Target:   ⏳ Pending implementation  
Quality Gates:        ✅ All passing
Complexity Budget:    ✅ Under limits
Test Coverage:        ✅ >80% maintained
```

## 📄 License

MIT License - See [LICENSE](./LICENSE) for details.

---

**🌸 Built with Toyota Way principles: Quality built-in, not bolted-on**

**The Ultimate Compiler Validation**: *Self-hosting represents the highest form of language implementation correctness - when a language can compile itself, it demonstrates complete self-sufficiency and implementation maturity.*