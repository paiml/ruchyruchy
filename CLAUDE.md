# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

RuchyRuchy is a self-hosting compiler bootstrap project for the Ruchy programming language. The project implements a progressive bootstrap sequence where each compiler stage can compile the next stage, ultimately achieving self-compilation.

## Architecture

### Bootstrap Progression
The compiler follows a four-stage bootstrap sequence:
- **Stage 0 (Lexer)**: 1K LOC - Tokenizes source code including itself
- **Stage 1 (Parser)**: 3K LOC - Parses Stage 0+1 using Pratt parser and recursive descent
- **Stage 2 (TypeCheck)**: 5K LOC - Type checks all stages using Algorithm W
- **Stage 3 (CodeGen)**: 6K LOC - Generates Rust code for all stages

### Repository Structure
```
ruchyruchy/
├── bootstrap/           # Progressive compiler stages
│   ├── stage0/         # Lexical analysis (tokens, lexer)
│   ├── stage1/         # Syntax analysis (parser, AST)
│   ├── stage2/         # Type inference (Algorithm W, unification)
│   └── stage3/         # Code generation (Rust emission, optimization)
├── validation/         # Testing and validation
└── docs/              # Specifications and documentation
```

## Development Commands

### Bootstrap Build Commands
```bash
# Progressive bootstrap stages (each depends on previous)
make stage0              # Build Stage 0: Lexer (1K LOC, tokenizes itself)
make stage1              # Build Stage 1: Parser (3K LOC, parses Stage 0+1)
make stage2              # Build Stage 2: TypeCheck (5K LOC, types all stages) 
make stage3              # Build Stage 3: CodeGen (6K LOC, compiles everything)
make bootstrap-all       # Build all stages in sequence with validation
```

### Self-Compilation Testing
```bash
# Test each stage's self-compilation capability
make test-stage0         # Test self-tokenization: ./lexer < lexer.ruchy
make test-stage1         # Test self-parsing with roundtrip validation
make test-stage2         # Test self-type-checking with Algorithm W
make test-stage3         # Test self-compilation (bit-identical output)
make test-self-compilation  # Complete self-compilation test suite
make test-differential   # Compare output with production compiler
```

### Toyota Way Quality Gates (MANDATORY - BLOCKING)
```bash
make quality-gate        # Run ALL mandatory quality checks (BLOCKING)
make validate           # Comprehensive validation including quality gates
make lint              # Zero-warning linting with clippy -D warnings  
make test              # All test suites (unit, integration, self-compilation)
make complexity        # Ensure all functions <20 cyclomatic complexity
make coverage          # Test coverage analysis (≥80% required)
make security          # Security vulnerability scan
```

### Ruchy Formal Verification Integration
```bash
# Showcase Ruchy's advanced capabilities on bootstrap code
make verify-all         # Run formal verification on all stages
make complexity-analysis # BigO complexity analysis with ruchy runtime
make provability-check  # Mathematical correctness proofs
make quality-scoring    # Unified quality assessment with ruchy score
```

### Toyota Way Continuous Improvement
```bash
make analyze-complexity  # Find complexity hotspots (Genchi Genbutsu)
make kaizen-refactor    # Generate continuous improvement plan
make quality-report     # Comprehensive quality metrics dashboard
```

## Development Workflow

### Quality Requirements
- All functions must have <20 cyclomatic complexity
- Code must pass `ruchy lint` checks
- Performance target: <5% overhead vs production compiler
- Memory usage: <100MB peak RSS for 10K LOC input

### Testing Strategy
- **Differential Testing**: Compare output with production Ruchy compiler
- **Self-compilation**: Each stage must compile itself
- **Performance Validation**: Throughput targets vary by stage
- **Property Testing**: Roundtrip validation for parser (parse(ast.emit()) == ast)

### File Extensions and Languages
- `.ruchy` files contain Ruchy language source code
- Generated output is Rust code (`.rs` files)
- Build artifacts go in `build/` directory
- JSON intermediate representations for AST and typed AST

## Key Implementation Details

### Stage 0 - Lexer
- Implements minimal token set (12 keywords, literals, operators, delimiters)
- Target: >10K LOC/s throughput
- Self-validation: tokenizes own source code

### Stage 1 - Parser  
- Uses Pratt parser for expressions with operator precedence
- Recursive descent for declarations and statements
- Target: >5K LOC/s throughput
- Roundtrip property testing required

### Stage 2 - Type Inference
- Implements Algorithm W (Hindley-Milner type inference)
- Includes constraint solving and generalization
- Must handle occurs check and infinite type prevention
- Target: O(n log n) complexity

### Stage 3 - Code Generation
- Emits idiomatic Rust code
- Generated code must compile with rustc without warnings
- Target: >10K LOC/s throughput
- Must achieve bit-identical output with production compiler

## Validation and Integration

The project maintains an INTEGRATION.md file tracking:
- Build matrix status for each stage
- Differential testing results comparing with production compiler
- Performance regression tracking
- Quality gate compliance

Success is measured by achieving self-compilation where the final stage can compile the entire bootstrap compiler, producing bit-identical output to the production Ruchy compiler.

## Project Management Protocol (Toyota Way)

### Sacred Rules (Zero Tolerance - BLOCKING)
1. **NEVER bypass quality gates** - `git commit --no-verify` is FORBIDDEN
2. **ALWAYS test self-compilation** - Every stage must compile itself progressively  
3. **NEVER exceed complexity budget** - All functions <20 cognitive complexity
4. **ZERO SATD tolerance** - No TODO/FIXME/HACK comments allowed
5. **ALWAYS use formal verification** - Every algorithm must pass `ruchy provability`
6. **NEVER implement without specification** - All work must reference ROADMAP.md tasks

### Sprint-Based Development Process (MANDATORY)
**CRITICAL**: Every sprint ends with commit and push to GitHub

```bash
# Sprint Structure (1-2 week cycles aligned with ROADMAP.md)
Sprint N: BOOTSTRAP-XXX Implementation
├── Day 1-3: Implementation following Toyota Way
├── Day 4-6: Self-compilation testing and validation  
├── Day 7-10: Formal verification and performance tuning
└── END: git commit && git push (MANDATORY)

# Sprint Commit Message Format (MANDATORY)
git commit -m "BOOTSTRAP-XXX: Implement [component] with formal verification

Stage: [0|1|2|3] - [Component Name]
Verification: ruchy provability score [X.X/100] 
Complexity: All functions <20 cognitive complexity
Performance: [actual] vs [target] throughput
Self-Compilation: [✓/✗] Progressive validation passes

Toyota Way: [Kaizen/Genchi Genbutsu/Jidoka] principle applied

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

### Quality Gate Integration Protocol
**Pre-commit hooks are MANDATORY and BLOCKING:**
```bash
# Install quality gates (REQUIRED for all contributors)
make install-hooks

# Every commit automatically validates:
# 1. Ruchy formal verification (ruchy provability)
# 2. Self-compilation tests (progressive validation)
# 3. Complexity analysis (functions <20)
# 4. Lint and test standards
```

### INTEGRATION.md Tracking (Single Source of Truth)
Following the pattern from ../ruchy-book, all progress must be tracked in `INTEGRATION.md`:
- **Real-time bootstrap progress** (Stage 0/1/2/3 completion)
- **Self-compilation test results** with performance metrics
- **Formal verification status** for each stage
- **Quality gate compliance** with Toyota Way standards
- **Version tracking** of Ruchy compiler used

**MANDATORY**: Update INTEGRATION.md after every sprint completion.

### Version Management Protocol
```bash
# FOOLPROOF version update process (like ../ruchy-book)
make sync-version       # Updates everything automatically
                       # - Detects latest Ruchy version
                       # - Updates all references
                       # - Tests bootstrap compatibility
                       # - Updates integration docs
```

### Performance Target Validation
All stages must meet empirical performance targets:
- **Stage 0 Lexer**: >10K LOC/s throughput measurement
- **Stage 1 Parser**: >5K LOC/s with roundtrip validation
- **Stage 2 TypeCheck**: O(n log n) complexity proof via `ruchy runtime`
- **Stage 3 CodeGen**: >10K LOC/s with bit-identical output validation

### Continuous Deployment Protocol
Following ../rosetta-ruchy pattern:
1. **Every stage completion** triggers automatic release
2. **GitHub push MANDATORY** after stage validation
3. **Version bumping** follows semantic versioning
4. **Quality metrics** tracked in release notes

### The Kaizen Refactoring Loop
```bash
# Step 1: Find complexity hotspots (Genchi Genbutsu)
make analyze-complexity

# Step 2: Generate improvement plan (Jidoka)
make kaizen-refactor --target bootstrap/stage[N]/

# Step 3: Apply improvements and validate
make quality-gate
```
- push changes at end of each sprint to GitHub