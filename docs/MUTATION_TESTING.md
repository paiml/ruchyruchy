# Mutation Testing Integration (DEBUGGER-045)

**Status**: Infrastructure Complete (Manual/CI Execution Pending)
**Tool**: cargo-mutants 25.3.1
**Target**: ≥90% mutation kill rate (based on bashrs 96.6%, paiml-mcp-agent-toolkit >90%)

## Overview

Mutation testing validates test suite quality by introducing small code changes (mutations) and verifying tests catch them. This report documents our mutation testing integration following extreme TDD principles.

## Quick Start

```bash
# Install cargo-mutants
cargo install cargo-mutants

# Run mutation testing on parser (takes ~10-30 min)
cargo mutants --file src/interpreter/parser.rs --timeout 60 --no-shuffle

# Run mutation testing on evaluator
cargo mutants --file src/interpreter/evaluator.rs --timeout 60 --no-shuffle

# Generate JSON report
cargo mutants --file src/interpreter/parser.rs --timeout 60 --output target/mutants/parser.json
```

## Configuration

### Recommended Settings

- `--timeout 60`: 60-second timeout per test (prevents hangs)
- `--no-shuffle`: Deterministic mutation order for reproducibility
- `--jobs 4`: Parallel execution (adjust based on CPU)
- `--output`: JSON output for automated analysis

### Current Blockers

**Property-Based Test Timeouts**: Some property tests (test_property_based_tests.rs) exceed 10s timeout
**Solution**: Configure longer timeouts or skip slow tests during mutation testing

```bash
# Skip slow tests
cargo mutants --file src/interpreter/parser.rs --timeout 60 --test-timeout 120
```

## Mutation Kill Rate Goals

**Target**: ≥90% mutation kill rate

### Industry Standards
- **bashrs**: 96.6% mutation kill rate (gold standard)
- **paiml-mcp-agent-toolkit**: >90% mutation kill rate
- **Industry Average**: 70-80% mutation kill rate

### Current Status

**Parser**: Not yet measured (baseline blocked by test timeouts)
**Evaluator**: Not yet measured (baseline blocked by test timeouts)

### Methodology

1. **Baseline**: Run `cargo test` to ensure all tests pass
2. **Mutate**: Apply mutations to source code (e.g., `+` → `-`, `==` → `!=`)
3. **Test**: Run full test suite against mutated code
4. **Classify**:
   - **Caught**: Test failed → mutation killed (GOOD)
   - **Missed**: Tests passed → mutation survived (BAD - test gap!)
   - **Timeout**: Test took >60s → mutation timeout (investigate)

## Mutation Types

cargo-mutants applies these mutation types:

1. **Arithmetic**: `+` → `-`, `*` → `/`
2. **Comparison**: `==` → `!=`, `<` → `>=`
3. **Logical**: `&&` → `||`, `!x` → `x`
4. **Return**: `return x` → `return Default::default()`
5. **Delete**: Remove function bodies, statements

## Test Suite Coverage Validation

### Parser Tests

**Test Files**:
- `tests/test_interp_001_parser.rs` (7 tests)
- `tests/test_interp_011-017_ch*_examples.rs` (Chapter examples)
- Property-based parser tests

**Expected Coverage**:
- Token parsing mutations
- Expression parsing mutations
- Statement parsing mutations
- Error recovery mutations

### Evaluator Tests

**Test Files**:
- `tests/test_interp_004_evaluator.rs` (7 tests)
- `tests/test_interp_005_functions.rs` (18 tests)
- `tests/test_interp_028_property_based.rs` (5 tests)
- `tests/test_interp_029_fuzzing.rs` (7 tests)

**Expected Coverage**:
- Expression evaluation mutations
- Function call mutations
- Scope management mutations
- Error handling mutations

## Survivor Analysis Workflow

When mutation testing completes:

### 1. Identify Survivors

```bash
# View mutation report
cargo mutants --file src/interpreter/parser.rs --timeout 60 --list-files

# Check JSON output
cat target/mutants/parser.json | jq '.[] | select(.outcome == "missed")'
```

### 2. Analyze Each Survivor

For each surviving mutant:

**Document**:
- What mutation survived
- Why tests didn't catch it
- Is this a real test gap or benign mutation?

**Example**:
```
Survivor: src/interpreter/parser.rs:145
Mutation: Changed `+` to `-` in token position calculation
Analysis: This is internal bookkeeping; position doesn't affect parse correctness
Action: Benign - no test needed
```

### 3. Write Tests to Kill Survivors

For real test gaps:

```rust
// Example: Mutation survived because we didn't test negative edge case
#[test]
fn test_parser_handles_negative_offset() {
    // This test would kill the mutation
    let code = "...";
    let ast = Parser::new(code).parse().unwrap();
    // Assert correct behavior
}
```

### 4. Re-run Mutation Testing

After writing new tests:
```bash
cargo mutants --file src/interpreter/parser.rs --timeout 60
```

Verify mutation kill rate improved.

## Integration with CI

### GitHub Actions Workflow

```yaml
# .github/workflows/mutation-testing.yml
name: Mutation Testing

on:
  pull_request:
    paths:
      - 'src/interpreter/**'

jobs:
  mutation-test:
    runs-on: ubuntu-latest
    timeout-minutes: 60

    steps:
      - uses: actions/checkout@v3

      - name: Install Rust
        uses: actions-rust-lang/setup-rust-toolchain@v1

      - name: Install cargo-mutants
        run: cargo install cargo-mutants

      - name: Run mutation testing on parser
        run: |
          cargo mutants --file src/interpreter/parser.rs \
            --timeout 60 --no-shuffle \
            --output target/mutants/parser.json

      - name: Check mutation kill rate
        run: |
          KILL_RATE=$(cat target/mutants/parser.json | jq -r '.kill_rate')
          if (( $(echo "$KILL_RATE < 90.0" | bc -l) )); then
            echo "Mutation kill rate too low: $KILL_RATE% (target: ≥90%)"
            exit 1
          fi

      - name: Upload mutation report
        uses: actions/upload-artifact@v3
        with:
          name: mutation-report
          path: target/mutants/
```

## Performance Optimization

### Reduce Mutation Testing Time

1. **Parallel Execution**: Use `--jobs N` for CPU cores
2. **Skip Slow Tests**: Use test attributes to skip slow property tests during mutation runs
3. **Incremental Mutation**: Only mutate changed files in PRs
4. **Caching**: Cache build artifacts between mutation runs

### Benchmark

**Expected Duration**:
- **parser.rs** (~2000 LOC): 10-30 minutes
- **evaluator.rs** (~2500 LOC): 15-40 minutes
- **Full interpreter**: 30-60 minutes

## Troubleshooting

### Issue: "cargo test failed in an unmutated tree"

**Cause**: Baseline test suite has failures
**Solution**: Fix failing tests first
```bash
cargo test  # Ensure all tests pass
```

### Issue: Mutation timeouts

**Cause**: Property-based tests or slow tests exceed timeout
**Solution**: Increase timeout or skip slow tests
```bash
cargo mutants --file src/interpreter/parser.rs --timeout 120
```

### Issue: Out of memory

**Cause**: Too many parallel jobs
**Solution**: Reduce parallelism
```bash
cargo mutants --file src/interpreter/parser.rs --jobs 2
```

## References

- **cargo-mutants**: https://mutants.rs/
- **bashrs mutation testing**: 96.6% kill rate (gold standard)
- **paiml-mcp-agent-toolkit**: >90% kill rate
- **Mutation Testing Research**: "Mutation Testing: A Comprehensive Survey" (Jia & Harman, 2011)

## Next Steps

1. ✅ Infrastructure complete (cargo-mutants installed, tests written)
2. ⏳ Resolve baseline test timeouts
3. ⏳ Run full mutation testing on parser.rs
4. ⏳ Run full mutation testing on evaluator.rs
5. ⏳ Analyze survivors and write tests
6. ⏳ Achieve ≥90% mutation kill rate
7. ⏳ Integrate into CI pipeline

---

**Last Updated**: November 1, 2025
**DEBUGGER-045 Status**: RED-GREEN phases complete, REFACTOR pending manual/CI execution
