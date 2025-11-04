# RuchyRuchy Debugging Guide for Ruchy Team

**Quick Start for Ruchy Compiler Developers**

RuchyRuchy provides advanced debugging tools for the Ruchy compiler development. This guide shows you how to use the latest features.

## Latest Features (v1.27.0)

### 1. **Parser Error Recovery** (DEBUGGER-051) - NEW! 🎉

Automatically detect and recover from parser errors with helpful suggestions.

**Use Case**: Testing parser resilience to malformed input

```rust
use ruchyruchy::debugger::error_recovery;

// Parse code with errors and get recovery information
let source = r#"
fun main() {
    let x = (10 + 5;  // Missing closing paren
    retrun 42;        // Typo: should be "return"
}
"#;

let result = error_recovery::parse_with_recovery(source)?;

// Check results
println!("Parsed {} AST nodes", result.ast_nodes);
println!("Found {} errors:", result.errors.len());

for error in &result.errors {
    println!("  Line {}: {}", error.line, error.message);

    if let Some(suggestion) = &error.suggestion {
        println!("    Hint: {}", suggestion);  // "Did you mean 'return'?"
    }

    if let Some(help) = &error.help_text {
        println!("    Help: {}", help);
    }
}
```

**Features**:
- ✅ **Panic-Mode Recovery**: Skip to next statement boundary
- ✅ **Typo Suggestions**: Levenshtein distance (≤2) for common mistakes
- ✅ **Multiple Errors**: Collect all errors, not fail-fast (IDE-friendly)
- ✅ **Quality Metadata**: Line/column/message/help text/error codes

**Common Typos Detected**:
- `retrun` → "Did you mean 'return'?"
- `prit` → "Did you mean 'print'?"
- `fucn` → "Did you mean 'fun'?"

---

### 2. **Differential Testing** (DEBUGGER-053)

Compare interpreter vs JIT execution to catch discrepancies.

```rust
use ruchyruchy::debugger::differential;

let source = "fun add(a: i64, b: i64) { return a + b; }";

// Run differential test
let result = differential::diff_test(source, "add", vec![10, 5])?;

if result.interpreter_result == result.jit_result {
    println!("✅ Match: {} == {}", result.interpreter_result, result.jit_result);
} else {
    println!("❌ MISMATCH: interp={}, jit={}",
        result.interpreter_result, result.jit_result);
}
```

**Coverage Analysis**:
```rust
let test_suite = vec![
    ("fun int_literal() { return 42; }", "int_literal"),
    ("fun arithmetic() { return 10 + 5 * 2; }", "arithmetic"),
];

let coverage = differential::check_coverage(&test_suite)?;

println!("Coverage: {}/{} tests passed", coverage.passed, coverage.total);
println!("AST nodes covered: {}", coverage.ast_nodes_covered);
println!("Mismatches: {} (zero tolerance!)", coverage.mismatches);
```

---

### 3. **JIT Compiler Debugging** (DEBUGGER-052)

Inspect Cranelift IR for JIT compilation issues.

```rust
use ruchyruchy::debugger::jit;

let source = "fun main() { return 42; }";

// Get Cranelift IR
let ir = jit::show_cranelift_ir(source, "main");
println!("Cranelift IR:\n{}", ir);

// Trace JIT execution
let trace = jit::trace_jit_execution(source, "main")?;
println!("Execution trace: {} steps", trace.steps.len());
```

---

### 4. **Parser Debugging** (DEBUGGER-050)

Visualize AST and tokenization.

```rust
use ruchyruchy::debugger;

let source = "fun main() { return 42; }";

// Tokenize
let tokens = debugger::tokenize(source);
println!("Tokens:\n{}", tokens);

// Visualize AST as JSON
let ast_json = debugger::visualize_ast(source);
println!("AST:\n{}", ast_json);

// Generate Graphviz diagram
let graphviz = debugger::visualize_ast_graphviz(source);
// Save to file and render with: dot -Tpng ast.dot -o ast.png
```

---

### 5. **Automated Quality Gates** (DEBUGGER-054)

Run all debugger validations automatically.

```rust
use ruchyruchy::debugger::quality_gates;

// Validate parser tests
let parser_tests = quality_gates::get_all_parser_tests();
let stats = quality_gates::validate_parser_visualizations(&parser_tests)?;
println!("Parser: {}/{} tests passed", stats.passed, stats.total);

// Validate JIT tests
let jit_tests = quality_gates::get_all_jit_tests();
let stats = quality_gates::validate_jit_inspections(&jit_tests)?;
println!("JIT: {}/{} tests passed", stats.passed, stats.total);

// Check differential coverage
let test_suite = quality_gates::get_differential_test_suite();
let coverage = quality_gates::check_differential_coverage(&test_suite)?;
println!("Coverage: {} AST nodes", coverage.ast_nodes_covered);

// Verify zero mismatches
let mismatches = quality_gates::find_known_mismatches()?;
assert_eq!(mismatches.len(), 0, "Jidoka violation!");
```

---

## CLI Tool: `ruchydbg`

**Note**: The `ruchydbg` CLI wraps the production `ruchy` compiler (not the educational interpreter). Use it for debugging production Ruchy code.

```bash
# Interactive debugging with rust-gdb
ruchydbg debug run test.ruchy

# Automated trace capture
ruchydbg debug analyze test.ruchy

# Run with timeout detection
ruchydbg run test.ruchy --timeout 5s

# Trace execution
ruchydbg trace test.ruchy
```

---

## Integration with CI/CD

Add to your `.github/workflows/ci.yml`:

```yaml
- name: Run RuchyRuchy Quality Gates
  run: |
    cargo test --package ruchyruchy --test test_debugger_054_quality_gates
    cargo test --package ruchyruchy --test test_debugger_051_error_recovery
    cargo test --package ruchyruchy --test test_debugger_053_differential
```

---

## Error Recovery API Reference

### `parse_with_recovery(source: &str) -> Result<RecoveryResult, String>`

**Returns**:
```rust
pub struct RecoveryResult {
    pub ast_nodes: usize,     // Number of nodes parsed
    pub errors: Vec<ParseError>,  // All errors found
}

pub struct ParseError {
    pub line: usize,              // Line number (1-indexed)
    pub column: usize,            // Column number (1-indexed)
    pub message: String,          // Error message
    pub severity: String,         // "error" | "warning"
    pub recoverable: bool,        // Parser recovered?
    pub suggestion: Option<String>,  // "Did you mean...?"
    pub help_text: Option<String>,   // How to fix
    pub code: Option<String>,        // Error code (e.g., "E0001")
}
```

**Error Recovery Strategies**:
1. **Panic Mode**: Skip to next `;` or `}`
2. **Phrase-Level**: Implicit semicolon insertion (ASI)
3. **Error Productions**: Detect typos using Levenshtein distance
4. **Global Correction**: Collect all errors (not fail-fast)

---

## Testing Philosophy

RuchyRuchy follows **EXTREME TDD** with Toyota Way principles:

- **Jidoka**: Stop the line on failures (zero tolerance)
- **Genchi Genbutsu**: Go and see actual errors
- **Kaizen**: Continuous improvement through automation
- **Heijunka**: Consistent quality across all tools

All features have:
- ✅ 100% test coverage
- ✅ Zero clippy warnings
- ✅ <20 cognitive complexity per function
- ✅ Comprehensive documentation

---

## Performance

- **Parser Error Recovery**: <1ms for typical files
- **Differential Testing**: <10ms per test case
- **JIT IR Inspection**: <5ms per function
- **Quality Gates**: <1s for full suite

---

## Contributing

Found a bug? Want a feature?

1. **File GitHub Issue**: https://github.com/paiml/ruchyruchy/issues
2. **Follow EXTREME TDD**: RED → GREEN → REFACTOR
3. **Pass All Quality Gates**: 6/6 (tests, fmt, clippy, complexity, SATD, TDG)
4. **Document in Book**: MANDATORY per CLAUDE.md

---

## Questions?

- **Docs**: https://github.com/paiml/ruchyruchy/tree/main/book
- **Issues**: https://github.com/paiml/ruchyruchy/issues
- **Ruchy Compiler**: https://github.com/paiml/ruchy

---

**Version**: 1.27.0
**Last Updated**: 2025-11-04
**New in v1.27.0**: Parser Error Recovery (DEBUGGER-051) with typo suggestions and panic-mode recovery
