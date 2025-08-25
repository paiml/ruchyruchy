# Phase 2 Validation Infrastructure

This directory contains the comprehensive validation suite for testing Ruchy tools against Ruchy-compiled code, as specified in Phase 2 of the RuchyRuchy roadmap.

## Sprint 1 Implementation (VALID-001 & VALID-002)

### Components

1. **Self-Compilation Test Harness** (`self_compilation_harness.ts`)
   - Tests Ruchy's ability to compile itself
   - Compares self-compiled output with reference compiler
   - Generates detailed validation reports

2. **Differential Testing Framework** (`differential_test_runner.ts`)
   - Systematically compares outputs from different compilation paths
   - Supports bit-identical, semantic, and optimized comparison modes
   - Executes both outputs and compares behavior

3. **Output Comparison Tools** (`output_comparator.ts`)
   - Advanced comparison utilities for compiler outputs
   - AST-based comparison for TypeScript code
   - Similarity scoring and difference categorization

4. **Continuous Validation Pipeline** (`continuous_pipeline.ts`)
   - Automated validation on every change
   - Performance regression detection
   - Comprehensive reporting

5. **Deno Toolchain Validator** (`deno_toolchain_validator.ts`)
   - Tests compatibility with all Deno tools
   - Validates: deno run, deno fmt, deno lint, deno test, deno bench, deno check

## Running the Tests

### Prerequisites
- Deno installed (latest version)
- Ruchy compiler available in PATH
- Build directory permissions

### Quick Start

Run the complete Sprint 1 validation suite:
```bash
make validate-sprint1
```

Or run directly with Deno:
```bash
cd validation
deno run --allow-all run_validation_suite.ts
```

### Individual Test Suites

#### Self-Compilation Testing
```bash
deno run --allow-all self_compilation_harness.ts
```

#### Differential Testing
```bash
deno run --allow-all differential_test_runner.ts
```

#### Deno Toolchain Validation
```bash
deno run --allow-all deno_toolchain_validator.ts
```

#### Continuous Pipeline (Single Run)
```bash
deno run --allow-all continuous_pipeline.ts
```

#### Continuous Pipeline (Watch Mode)
```bash
deno run --allow-all continuous_pipeline.ts --watch
```

## Test Structure

### VALID-001: Self-Compilation Test Harness
Tests the bootstrap compiler stages:
- Stage 0: Lexer self-tokenization
- Stage 1: Parser self-parsing
- Stage 2: Type checker self-type-checking
- Stage 3: Code generator self-compilation

Success Criteria:
- Automated pipeline compiling Ruchy with Ruchy
- Bit-for-bit output comparison
- Performance metrics tracking
- Regression detection

### VALID-002: Deno Toolchain Validation
Validates generated TypeScript with:
- `deno check`: Type checking
- `deno run`: Execution validation
- `deno fmt`: Format compatibility
- `deno lint`: Linting compliance
- `deno test`: Test framework compatibility
- `deno bench`: Benchmarking support

Success Criteria:
- All generated TypeScript runs with `deno run --allow-all`
- Format compatibility with `deno fmt --check`
- Lint compliance with `deno lint`
- Test harness using `deno test`

## Output Structure

```
build/
├── validation/          # Self-compilation outputs
│   ├── input/          # Source files
│   ├── output/         # Self-compiled outputs
│   └── reference/      # Reference compiler outputs
├── differential/       # Differential testing
│   ├── execution/      # Execution results
│   └── reports/        # Comparison reports
├── deno_validation/    # Deno toolchain tests
│   ├── source/         # Input TypeScript
│   ├── formatted/      # deno fmt results
│   ├── tests/          # Generated test files
│   └── reports/        # Validation reports
└── continuous/         # Pipeline artifacts
    ├── reports/        # Timestamped reports
    ├── metrics/        # Performance metrics
    └── artifacts/      # Build artifacts
```

## Success Metrics

- **Validation Coverage**: 100% of Ruchy features tested
- **Self-Compilation**: All stages compile themselves successfully
- **Deno Compatibility**: Full toolchain compatibility achieved
- **Performance**: No regressions >10% from baseline
- **Stability**: Compilation converges to fixpoint

## Next Steps (Sprint 2)

After Sprint 1 validation passes:
- VALID-003: AST Validation Framework
- PROP-001: Lexer Property Testing

See [ROADMAP_PHASE2.md](../ROADMAP_PHASE2.md) for complete Phase 2 planning.