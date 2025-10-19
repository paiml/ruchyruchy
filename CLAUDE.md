# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

RuchyRuchy is an educational compiler infrastructure project supporting the Ruchy programming language ecosystem. The project provides educational resources, development tools, and extensive validation frameworks for understanding compiler construction and testing compiler robustness.

## 🚨 CRITICAL: Bug Discovery Protocol

**MANDATORY PROCEDURE - ZERO TOLERANCE**

When you discover ANY bug or not-implemented feature in Ruchy:

1. **STOP THE LINE** - Immediately halt all work
2. **FILE GITHUB ISSUE** - Create issue at https://github.com/paiml/ruchy/issues
3. **EXTREME DETAIL REQUIRED** - Issue MUST contain:
   - Exact reproduction steps
   - Minimal reproduction code (pure Ruchy)
   - Expected behavior vs actual behavior
   - Ruchy version (`ruchy --version`)
   - Full error output (copy-paste verbatim)
   - Context: what you were trying to accomplish
   - Impact: how this blocks current work
   - Workaround: if any exists

**Issue Template**:
```markdown
## Bug Report: [Short Description]

**Ruchy Version**: [output of `ruchy --version`]
**Project**: RuchyRuchy Bootstrap Compiler
**Ticket**: [BOOTSTRAP-XXX or VALID-XXX]

### Reproduction Steps
1. [Step 1]
2. [Step 2]
3. [etc]

### Minimal Reproduction Code
```ruchy
// Exact code that triggers the bug
[paste code here]
```

### Expected Behavior
[What should happen]

### Actual Behavior
[What actually happens]

### Full Error Output
```
[Complete error message, copy-paste verbatim]
```

### Context
[What you were trying to accomplish when you found this]

### Impact
[How this blocks current work - e.g., "Blocks BOOTSTRAP-002", "Prevents character stream implementation"]

### Workaround
[If any workaround exists, describe it]

### Environment
- OS: [Linux/Mac/Windows]
- Ruchy install: [Cargo/Binary/etc]
```

**After Filing Issue**:
1. Document the bug in BOUNDARIES.md
2. Implement workaround if possible
3. Continue with alternative approach
4. Reference GitHub issue # in commit messages

**Example**:
```
Discovered: Enum tuple variants cause "No match arm matched" runtime error
Action: Filed https://github.com/paiml/ruchy/issues/XXX
Workaround: Using unit variants only for Position tracking
Status: BOOTSTRAP-002 proceeding with simplified enum design
```

## Critical Requirements

### MUST Use Pure Ruchy Tooling (Dogfooding)
**ALL testing and validation infrastructure in this project MUST use Ruchy binary tools:**
- `ruchy test` - Run tests (pure Ruchy test files)
- `ruchy lint` - Lint code with A+ quality requirements
- `ruchy fmt` - Format code to canonical style
- `ruchy prove` - Formal verification and property testing
- `ruchy score` - Quality scoring and complexity analysis
- `ruchy runtime` - Performance analysis and boundary testing
- `ruchy check` - Type checking and syntax validation

**Rationale**: A Ruchy project MUST dogfood Ruchy tools. Using external toolchains undermines credibility and prevents self-hosting validation.

## Phase 2: Validation & Robustness (Current Focus)

### Mission
Extensive validation of Ruchy tooling against Ruchy code compiled by Ruchy, with heavy focus on property testing and fuzz testing to discover the exact boundaries where our tools work and where they fail.

### Core Validation Objectives
1. **Self-Compilation Testing**: Validate tools against Ruchy-compiled code
2. **Property-Based Testing**: Mathematical property validation via `ruchy prove`
3. **Fuzz Testing**: Boundary and edge case discovery
4. **Pure Ruchy Dogfooding**: All tests written in Ruchy using Ruchy tools
5. **Performance Analysis**: Comprehensive boundary mapping via `ruchy runtime`

### Property Testing Requirements
All property tests MUST:
- Be written in pure Ruchy (.ruchy files)
- Use `ruchy prove` for mathematical property validation
- Test mathematical properties (e.g., roundtrip: `parse(emit(ast)) = ast`)
- Run minimum 10,000 test cases per property via `ruchy test`
- Include shrinking for minimal failure cases
- Achieve >80% coverage via `ruchy score`

### Fuzz Testing Requirements
All fuzz tests MUST:
- Be implemented in pure Ruchy
- Generate both valid and invalid inputs using Ruchy
- Use grammar-based generation for valid cases
- Track crash/hang/timeout statistics via `ruchy runtime`
- Minimize failing test cases using Ruchy tooling
- Store corpus for regression testing in .ruchy format

### Validation Pipeline (Pure Ruchy Only)
```bash
# All validation MUST use Ruchy tools
ruchy test validation/**/*.ruchy       # Run all validation tests
ruchy fmt validation/**/*.ruchy        # Format all validation code
ruchy lint validation/**/*.ruchy       # Lint with A+ requirement
ruchy prove validation/**/*.ruchy      # Formal verification
ruchy score validation/**/*.ruchy      # Quality analysis >0.8
ruchy runtime validation/**/*.ruchy    # Performance validation
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
├── validation/         # Pure Ruchy testing and validation
│   ├── property/       # Property-based tests (.ruchy files)
│   ├── fuzz/          # Fuzz testing infrastructure (.ruchy files)
│   ├── boundary/      # Boundary analysis tests (.ruchy files)
│   └── regression/    # Regression test suite (.ruchy files)
└── docs/              # Specifications and documentation
```

## Development Commands

### Phase 2 Validation Commands (Pure Ruchy Only)
```bash
# Property Testing (via Ruchy)
ruchy test validation/property/lexer_test.ruchy
ruchy test validation/property/parser_test.ruchy
ruchy test validation/property/types_test.ruchy
ruchy test validation/property/codegen_test.ruchy

# Property Verification (Mathematical Proofs)
ruchy prove validation/property/lexer_test.ruchy
ruchy prove validation/property/parser_test.ruchy
ruchy prove validation/property/types_test.ruchy
ruchy prove validation/property/codegen_test.ruchy

# Fuzz Testing (via Ruchy)
ruchy test validation/fuzz/fuzzer.ruchy
ruchy test validation/fuzz/grammar_gen.ruchy
ruchy test validation/fuzz/differential.ruchy

# Boundary Analysis (via Ruchy)
ruchy runtime validation/boundary/perf_limits.ruchy
ruchy test validation/boundary/feature_matrix.ruchy
ruchy test validation/boundary/error_recovery.ruchy

# Quality Analysis
ruchy score validation/**/*.ruchy  # Must achieve >0.8 score
ruchy lint validation/**/*.ruchy   # Must achieve A+ grade
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

### Toyota Way Quality Gates (MANDATORY - BLOCKING - Pure Ruchy Only)
```bash
make quality-gate        # Run ALL mandatory quality checks (BLOCKING)
make validate           # Comprehensive validation including quality gates
make ruchy-lint         # A+ grade requirement via ruchy lint
make ruchy-test         # All test suites via ruchy test
make ruchy-prove        # Formal verification via ruchy prove
make ruchy-score        # Quality score >0.8 via ruchy score
make ruchy-runtime      # Performance analysis via ruchy runtime
```

### Ruchy Formal Verification Integration (Dogfooding Excellence)
```bash
# Showcase Ruchy's advanced capabilities on bootstrap code
make verify-all         # Run formal verification on all stages
make complexity-analysis # BigO complexity analysis with ruchy runtime
make provability-check  # Mathematical correctness proofs via ruchy prove
make quality-scoring    # Unified quality assessment with ruchy score
```

### Toyota Way Continuous Improvement
```bash
make analyze-complexity  # Find complexity hotspots (Genchi Genbutsu)
make kaizen-refactor    # Generate continuous improvement plan
make quality-report     # Comprehensive quality metrics dashboard
```

## Development Workflow

### Quality Requirements (Pure Ruchy Dogfooding)
- All functions must have <20 cyclomatic complexity
- Code must pass `ruchy lint` checks with A+ grade
- Performance target: <5% overhead vs production compiler
- Memory usage: <100MB peak RSS for 10K LOC input
- All tests must run via `ruchy test` (.ruchy files only)
- Coverage must exceed 80% via `ruchy score`
- All validation code must be written in pure Ruchy

### Testing Strategy
- **Differential Testing**: Compare output with production Ruchy compiler
- **Self-compilation**: Each stage must compile itself
- **Performance Validation**: Throughput targets vary by stage
- **Property Testing**: Roundtrip validation for parser (parse(ast.emit()) == ast)
- **Fuzz Testing**: Grammar-based and mutation-based fuzzing
- **Boundary Testing**: Find exact limits of functionality

### File Extensions and Languages (Pure Ruchy Only)
- `.ruchy` files contain Ruchy language source code
- Generated output is TypeScript (`.ts`) or Rust code (`.rs`) 
- Test files MUST be pure Ruchy (`.ruchy`) run via `ruchy test`
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
- Property: `parse(emit(ast)) = ast` (verified via `ruchy prove`)

### Stage 2 - Type Inference
- Implements Algorithm W (Hindley-Milner type inference)
- Includes constraint solving and generalization
- Must handle occurs check and infinite type prevention
- Target: O(n log n) complexity
- Property: Well-typed programs don't crash

### Stage 3 - Code Generation
- Emits idiomatic TypeScript and Rust code
- Generated code must pass `ruchy fmt` and `ruchy lint`
- Target: >10K LOC/s throughput
- Must achieve semantic equivalence with source
- Property: Behavior preservation across targets

## Validation and Integration

The project maintains an INTEGRATION.md file tracking:
- Build matrix status for each stage
- Differential testing results comparing with production compiler
- Performance regression tracking
- Quality gate compliance
- Ruchy tool validation results

Success is measured by:
1. Self-compilation achieving bit-identical output
2. All property tests passing (10,000+ cases each)
3. Zero crashes from fuzz testing (1M+ inputs)
4. Complete boundary documentation
5. 80%+ test coverage via `ruchy score`

## Project Management Protocol (Toyota Way)

### Sacred Rules (Zero Tolerance - BLOCKING)
1. **NEVER bypass quality gates** - `git commit --no-verify` is FORBIDDEN
2. **ALWAYS test self-compilation** - Every stage must compile itself progressively  
3. **NEVER exceed complexity budget** - All functions <20 cognitive complexity
4. **ZERO SATD tolerance** - No TODO/FIXME/HACK comments allowed
5. **ALWAYS use formal verification** - Every algorithm must pass `ruchy provability`
6. **NEVER implement without specification** - All work must reference ROADMAP.md tasks
7. **ALWAYS use pure Ruchy** - All tests/validation via ruchy binary

### Sprint-Based Development Process (MANDATORY)
**CRITICAL**: Every sprint ends with commit and push to GitHub

```bash
# Sprint Structure (1-2 week cycles aligned with ROADMAP_PHASE2.md)
Sprint N: VALID-XXX/PROP-XXX/FUZZ-XXX Implementation
├── Day 1-3: Implementation following Toyota Way
├── Day 4-6: Property/fuzz testing via ruchy test  
├── Day 7-10: Boundary analysis and documentation
└── END: git commit && git push (MANDATORY)

# Sprint Commit Message Format (MANDATORY - Pure Ruchy Only)
git commit -m "VALID-XXX: Implement [component] with pure Ruchy validation

Component: [Validation/Property/Fuzz/Boundary]
Tests: [X] property tests, [Y] fuzz cases via ruchy test
Coverage: [Z]% via ruchy score (>0.8 required)
Boundaries: [List of discovered limits]
Performance: [actual] vs [target] metrics via ruchy runtime

Toyota Way: [Kaizen/Genchi Genbutsu/Jidoka] principle applied
Dogfooding: 100% pure Ruchy implementation and testing

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

### Quality Gate Integration Protocol
**Pre-commit hooks are MANDATORY and BLOCKING:**
```bash
# Install quality gates (REQUIRED for all contributors)
make install-hooks

# Every commit automatically validates:
# 1. Ruchy formal verification (ruchy prove)
# 2. Self-compilation tests (progressive validation)
# 3. Complexity analysis (functions <20)
# 4. Lint and test standards (ruchy lint A+ grade)
# 5. Format check (ruchy fmt --check)
# 6. Quality score (ruchy score >0.8)
```

### INTEGRATION.md Tracking (Single Source of Truth)
Following the pattern from ../ruchy-book, all progress must be tracked in `INTEGRATION.md`:
- **Real-time validation progress** (VALID/PROP/FUZZ/BOUND completion)
- **Property test results** with case counts
- **Fuzz test statistics** (crashes, hangs, coverage)
- **Boundary documentation** (limits and capabilities)
- **Ruchy tool dogfooding** results

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
- **Property Tests**: >1000 cases/second via `ruchy test`
- **Fuzz Tests**: >10K inputs/second generation rate

### Continuous Deployment Protocol
Following ../rosetta-ruchy pattern:
1. **Every stage completion** triggers automatic release
2. **GitHub push MANDATORY** after stage validation
3. **Version bumping** follows semantic versioning
4. **Quality metrics** tracked in release notes
5. **Ruchy quality reports** (score, lint, prove) included in releases

### The Kaizen Refactoring Loop
```bash
# Step 1: Find complexity hotspots (Genchi Genbutsu)
make analyze-complexity

# Step 2: Generate improvement plan (Jidoka)
make kaizen-refactor --target bootstrap/stage[N]/

# Step 3: Apply improvements and validate
make quality-gate

# Step 4: Validate with Ruchy tools
ruchy test validation/**/*.ruchy
ruchy fmt validation/**/*.ruchy
ruchy lint validation/**/*.ruchy
ruchy prove validation/**/*.ruchy
ruchy score validation/**/*.ruchy
```
- push changes at end of each sprint to GitHub
# important-instruction-reminders
Do what has been asked; nothing more, nothing less.
NEVER create files unless they're absolutely necessary for achieving your goal.
ALWAYS prefer editing an existing file to creating a new one.
NEVER proactively create documentation files (*.md) or README files. Only create documentation files if explicitly requested by the User.

      
      IMPORTANT: this context may or may not be relevant to your tasks. You should not respond to this context unless it is highly relevant to your task.
- no feature can be added or code changed without following TDD.  All examples must be PURE ruchy and use ruchy tooling to test like ../ruchy-repl-demos.
- lets look at the work from ../ruchy-repl-demo and ../ruchy-book and ensure we copy the style.  in particulare we want TDD.  Ruchy tooling dogfooded, and to never work on code that isn't in roadmap and doesn't have a ticket.