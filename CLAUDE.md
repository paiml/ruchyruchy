# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

RuchyRuchy is an educational compiler infrastructure project supporting the Ruchy programming language ecosystem. The project provides educational resources, development tools, and extensive validation frameworks for understanding compiler construction and testing compiler robustness.

## Critical Requirements

### MUST Use Deno Binary Tools
**ALL testing and validation infrastructure in this project MUST use Deno binary tools:**
- `deno run` - Execute TypeScript/JavaScript code
- `deno fmt` - Format code
- `deno lint` - Lint code  
- `deno test` - Run tests
- `deno bench` - Run benchmarks
- `deno compile` - Compile to executables
- `deno doc` - Generate documentation
- `deno coverage` - Test coverage analysis

**Rationale**: Deno provides a secure, TypeScript-first runtime with built-in tooling that aligns with our validation goals.

## Phase 2: Validation & Robustness (Current Focus)

### Mission
Extensive validation of Ruchy tooling against Ruchy code compiled by Ruchy, with heavy focus on property testing and fuzz testing to discover the exact boundaries where our tools work and where they fail.

### Core Validation Objectives
1. **Self-Compilation Testing**: Validate tools against Ruchy-compiled code
2. **Property-Based Testing**: Mathematical property validation
3. **Fuzz Testing**: Boundary and edge case discovery
4. **Deno Integration**: All tests run via Deno toolchain
5. **Performance Analysis**: Comprehensive boundary mapping

### Property Testing Requirements
All property tests MUST:
- Use `deno test` infrastructure
- Test mathematical properties (e.g., roundtrip: `parse(emit(ast)) = ast`)
- Run minimum 10,000 test cases per property
- Include shrinking for minimal failure cases
- Track coverage metrics via `deno coverage`

### Fuzz Testing Requirements
All fuzz tests MUST:
- Generate both valid and invalid inputs
- Use grammar-based generation for valid cases
- Track crash/hang/timeout statistics
- Minimize failing test cases
- Store corpus for regression testing

### Validation Pipeline
```bash
# All validation must use Deno tools
deno test --allow-all validation/      # Run all validation tests
deno fmt --check generated/            # Validate generated code format
deno lint generated/                   # Lint generated code
deno bench validation/benchmarks/      # Performance validation
deno coverage --lcov validation/       # Coverage analysis
```

## Architecture

### Bootstrap Progression
The compiler follows a four-stage bootstrap sequence:
- **Stage 0 (Lexer)**: 1K LOC - Tokenizes source code including itself
- **Stage 1 (Parser)**: 3K LOC - Parses Stage 0+1 using Pratt parser and recursive descent
- **Stage 2 (TypeCheck)**: 5K LOC - Type checks all stages using Algorithm W
- **Stage 3 (CodeGen)**: 6K LOC - Generates TypeScript/Rust code for all stages

### Repository Structure
```
ruchyruchy/
├── bootstrap/           # Progressive compiler stages
│   ├── stage0/         # Lexical analysis (tokens, lexer)
│   ├── stage1/         # Syntax analysis (parser, AST)
│   ├── stage2/         # Type inference (Algorithm W, unification)
│   └── stage3/         # Code generation (TypeScript/Rust emission)
├── validation/         # Deno-based testing and validation
│   ├── property/       # Property-based tests
│   ├── fuzz/          # Fuzz testing infrastructure
│   ├── boundary/      # Boundary analysis tests
│   └── regression/    # Regression test suite
└── docs/              # Specifications and documentation
```

## Development Commands

### Phase 2 Validation Commands
```bash
# Property Testing (via Deno)
deno test validation/property/lexer_test.ts
deno test validation/property/parser_test.ts
deno test validation/property/types_test.ts
deno test validation/property/codegen_test.ts

# Fuzz Testing (via Deno)
deno run --allow-all validation/fuzz/fuzzer.ts
deno run --allow-all validation/fuzz/grammar_gen.ts
deno run --allow-all validation/fuzz/differential.ts

# Boundary Analysis (via Deno)
deno bench validation/boundary/perf_limits.ts
deno test validation/boundary/feature_matrix.ts
deno test validation/boundary/error_recovery.ts

# Coverage Analysis
deno coverage --lcov validation/
```

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
- All tests must run via `deno test`
- Coverage must exceed 80% via `deno coverage`

### Testing Strategy
- **Differential Testing**: Compare output with production Ruchy compiler
- **Self-compilation**: Each stage must compile itself
- **Performance Validation**: Throughput targets vary by stage
- **Property Testing**: Roundtrip validation for parser (parse(ast.emit()) == ast)
- **Fuzz Testing**: Grammar-based and mutation-based fuzzing
- **Boundary Testing**: Find exact limits of functionality

### File Extensions and Languages
- `.ruchy` files contain Ruchy language source code
- Generated output is TypeScript (`.ts`) or Rust code (`.rs`) 
- Test files are TypeScript (`.test.ts`) run via Deno
- Build artifacts go in `build/` directory
- JSON intermediate representations for AST and typed AST

## Key Implementation Details

### Stage 0 - Lexer
- Implements minimal token set (12 keywords, literals, operators, delimiters)
- Target: >10K LOC/s throughput
- Self-validation: tokenizes own source code
- Property: `concat(tokenize(a), tokenize(b)) = tokenize(a + b)`

### Stage 1 - Parser  
- Uses Pratt parser for expressions with operator precedence
- Recursive descent for declarations and statements
- Target: >5K LOC/s throughput
- Roundtrip property testing required
- Property: `parse(emit(ast)) = ast`

### Stage 2 - Type Inference
- Implements Algorithm W (Hindley-Milner type inference)
- Includes constraint solving and generalization
- Must handle occurs check and infinite type prevention
- Target: O(n log n) complexity
- Property: Well-typed programs don't crash

### Stage 3 - Code Generation
- Emits idiomatic TypeScript and Rust code
- Generated code must pass `deno fmt` and `deno lint`
- Target: >10K LOC/s throughput
- Must achieve semantic equivalence with source
- Property: Behavior preservation across targets

## Validation and Integration

The project maintains an INTEGRATION.md file tracking:
- Build matrix status for each stage
- Differential testing results comparing with production compiler
- Performance regression tracking
- Quality gate compliance
- Deno tool validation results

Success is measured by:
1. Self-compilation achieving bit-identical output
2. All property tests passing (10,000+ cases each)
3. Zero crashes from fuzz testing (1M+ inputs)
4. Complete boundary documentation
5. 80%+ test coverage via `deno coverage`

## Project Management Protocol (Toyota Way)

### Sacred Rules (Zero Tolerance - BLOCKING)
1. **NEVER bypass quality gates** - `git commit --no-verify` is FORBIDDEN
2. **ALWAYS test self-compilation** - Every stage must compile itself progressively  
3. **NEVER exceed complexity budget** - All functions <20 cognitive complexity
4. **ZERO SATD tolerance** - No TODO/FIXME/HACK comments allowed
5. **ALWAYS use formal verification** - Every algorithm must pass `ruchy provability`
6. **NEVER implement without specification** - All work must reference ROADMAP.md tasks
7. **ALWAYS use Deno tools** - All tests/validation via Deno binary

### Sprint-Based Development Process (MANDATORY)
**CRITICAL**: Every sprint ends with commit and push to GitHub

```bash
# Sprint Structure (1-2 week cycles aligned with ROADMAP_PHASE2.md)
Sprint N: VALID-XXX/PROP-XXX/FUZZ-XXX Implementation
├── Day 1-3: Implementation following Toyota Way
├── Day 4-6: Property/fuzz testing via Deno  
├── Day 7-10: Boundary analysis and documentation
└── END: git commit && git push (MANDATORY)

# Sprint Commit Message Format (MANDATORY)
git commit -m "VALID-XXX: Implement [component] with Deno validation

Component: [Validation/Property/Fuzz/Boundary]
Tests: [X] property tests, [Y] fuzz cases via deno test
Coverage: [Z]% via deno coverage
Boundaries: [List of discovered limits]
Performance: [actual] vs [target] metrics

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
# 5. Deno format check (deno fmt --check)
# 6. Deno lint (deno lint)
```

### INTEGRATION.md Tracking (Single Source of Truth)
Following the pattern from ../ruchy-book, all progress must be tracked in `INTEGRATION.md`:
- **Real-time validation progress** (VALID/PROP/FUZZ/BOUND completion)
- **Property test results** with case counts
- **Fuzz test statistics** (crashes, hangs, coverage)
- **Boundary documentation** (limits and capabilities)
- **Deno tool compatibility** results

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
- **Property Tests**: >1000 cases/second via `deno test`
- **Fuzz Tests**: >10K inputs/second generation rate

### Continuous Deployment Protocol
Following ../rosetta-ruchy pattern:
1. **Every stage completion** triggers automatic release
2. **GitHub push MANDATORY** after stage validation
3. **Version bumping** follows semantic versioning
4. **Quality metrics** tracked in release notes
5. **Deno coverage reports** included in releases

### The Kaizen Refactoring Loop
```bash
# Step 1: Find complexity hotspots (Genchi Genbutsu)
make analyze-complexity

# Step 2: Generate improvement plan (Jidoka)
make kaizen-refactor --target bootstrap/stage[N]/

# Step 3: Apply improvements and validate
make quality-gate

# Step 4: Validate with Deno tools
deno test validation/
deno fmt --check
deno lint
```
- push changes at end of each sprint to GitHub
# important-instruction-reminders
Do what has been asked; nothing more, nothing less.
NEVER create files unless they're absolutely necessary for achieving your goal.
ALWAYS prefer editing an existing file to creating a new one.
NEVER proactively create documentation files (*.md) or README files. Only create documentation files if explicitly requested by the User.

      
      IMPORTANT: this context may or may not be relevant to your tasks. You should not respond to this context unless it is highly relevant to your task.