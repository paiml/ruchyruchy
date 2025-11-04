# RuchyRuchy Debugger Enhancements Specification

**Date**: 2025-11-04
**Status**: Proposal - Extreme TDD Implementation Plan
**Priority**: HIGH - Addresses critical gap in parser/compile debugging

---

## Executive Summary

**Problem**: `ruchydbg` currently shells out to production `ruchy` compiler and ONLY validates runtime behavior (timeouts, hangs, profiling). It does NOT test or debug RuchyRuchy's own parser, evaluator, or JIT compiler.

**Impact**:
- Parser bugs in RuchyRuchy interpreter go undetected
- JIT compilation issues have no debugging tools
- AST transformation errors are invisible
- No differential testing between interpreter and JIT

**Solution**: Build dedicated debugging tools for RuchyRuchy's own compilation pipeline:
1. Parser debugger with AST visualization
2. JIT compilation debugger with IR inspection
3. Differential testing framework (interpreter vs JIT)
4. Parser error recovery testing tools
5. AST transformation validation tools

---

## Current State Analysis

### What `ruchydbg` Does (Runtime Only)

```rust
// src/bin/ruchydbg.rs:178
cmd.arg("ruchy");  // Shells out to PRODUCTION ruchy!
cmd.arg("run");
cmd.arg(file_path);
```

**Capabilities** (all runtime-focused):
- ✅ Timeout detection for infinite loops
- ✅ Type-aware tracing (via production ruchy)
- ✅ Stack depth profiling
- ✅ Pathological input detection
- ✅ Performance benchmarking

### What `ruchydbg` Does NOT Do

- ❌ Test RuchyRuchy parser implementation
- ❌ Debug RuchyRuchy AST construction
- ❌ Validate JIT compilation correctness
- ❌ Inspect Cranelift IR generation
- ❌ Differential test interpreter vs JIT
- ❌ Parse error recovery testing
- ❌ AST transformation validation

### The Gap

```bash
# Testing RuchyRuchy parser changes:
cargo test test_interp_014  # ✅ Tests YOUR parser
ruchydbg run test.ruchy     # ❌ Tests production ruchy, NOT your parser!

# Testing JIT compilation:
cargo test test_jit_001     # ✅ Tests YOUR JIT compiler
ruchydbg run test.ruchy     # ❌ Doesn't even use JIT!
```

**Critical Finding**: No debugging tools exist for RuchyRuchy's own compilation pipeline.

---

## Enhancement Goals

### Primary Objectives

1. **Parser Debugging** - Visualize and validate RuchyRuchy parser behavior
2. **JIT Debugging** - Inspect Cranelift IR and compilation stages
3. **Differential Testing** - Compare interpreter vs JIT results
4. **Error Recovery** - Test parser resilience to malformed input
5. **AST Validation** - Ensure AST transformations preserve semantics

### Success Metrics

- ✅ 100% coverage of RuchyRuchy parser debugging
- ✅ Real-time AST visualization during parsing
- ✅ Cranelift IR inspection for JIT-compiled code
- ✅ Automated differential testing (interp vs JIT)
- ✅ Parser error recovery validation suite
- ✅ Zero parser bugs reach production

---

## EXTREME TDD Implementation Plan

### Phase 1: Parser Debugger (DEBUGGER-050)

**Objective**: Build tools to debug RuchyRuchy's own parser implementation.

#### DEBUGGER-050: AST Visualization Tool

**RED Phase** - Write Failing Tests First
```rust
// tests/test_debugger_050_ast_viz.rs

#[test]
fn test_ast_viz_generates_json() {
    // Test: Parser debugger outputs AST as JSON
    let source = "fun main() { return 42; }";
    let ast_json = ruchyruchy::debugger::visualize_ast(source);

    assert!(ast_json.contains("FunctionDef"));
    assert!(ast_json.contains("main"));
    assert!(ast_json.contains("Return"));
    assert!(ast_json.contains("42"));
}

#[test]
fn test_ast_viz_generates_graphviz() {
    // Test: Parser debugger outputs AST as Graphviz DOT format
    let source = "let x = 10 + 5;";
    let dot = ruchyruchy::debugger::visualize_ast_graphviz(source);

    assert!(dot.contains("digraph AST"));
    assert!(dot.contains("LetDecl"));
    assert!(dot.contains("BinaryOp"));
}

#[test]
fn test_ast_viz_shows_source_locations() {
    // Test: AST visualization includes line/column info
    let source = "fun test() {\n  return 1 + 2;\n}";
    let ast_json = ruchyruchy::debugger::visualize_ast_with_locations(source);

    // Should include source location metadata
    assert!(ast_json.contains("line"));
    assert!(ast_json.contains("column"));
}

#[test]
fn test_ast_viz_handles_parse_errors() {
    // Test: Parser debugger shows partial AST on error
    let source = "fun broken(";
    let result = ruchyruchy::debugger::visualize_ast_partial(source);

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Expected closing paren"));
}

#[test]
fn test_ast_viz_diff_mode() {
    // Test: Compare ASTs from two versions
    let before = "let x = 1;";
    let after = "let x = 2;";
    let diff = ruchyruchy::debugger::ast_diff(before, after);

    assert!(diff.contains("IntegerLiteral: 1 -> 2"));
}

#[test]
fn test_ast_viz_step_by_step() {
    // Test: Show AST construction step-by-step
    let source = "1 + 2 * 3";
    let steps = ruchyruchy::debugger::visualize_ast_steps(source);

    // Should show parser decisions at each token
    assert!(steps.len() > 0);
    assert!(steps[0].contains("Parse IntegerLiteral: 1"));
}

#[test]
fn test_ast_viz_with_types() {
    // Test: Show inferred types in AST
    let source = "let x = 42;";
    let typed_ast = ruchyruchy::debugger::visualize_typed_ast(source);

    assert!(typed_ast.contains("type: i64"));
}
```

**Implementation Strategy**:
```rust
// src/debugger/ast_visualizer.rs

pub struct AstVisualizer {
    format: OutputFormat,
    show_locations: bool,
    show_types: bool,
}

pub enum OutputFormat {
    Json,
    Graphviz,
    TreeText,
}

impl AstVisualizer {
    pub fn visualize(&self, source: &str) -> Result<String, String> {
        let mut parser = Parser::new(source);
        let ast = parser.parse()?;

        match self.format {
            OutputFormat::Json => self.to_json(&ast),
            OutputFormat::Graphviz => self.to_graphviz(&ast),
            OutputFormat::TreeText => self.to_tree(&ast),
        }
    }

    fn to_json(&self, ast: &Ast) -> Result<String, String> {
        // Serialize AST to JSON with metadata
        todo!()
    }

    fn to_graphviz(&self, ast: &Ast) -> Result<String, String> {
        // Generate Graphviz DOT format
        todo!()
    }
}
```

**GREEN Phase** - Implement Minimal Solution
- Implement JSON serialization of AST
- Add Graphviz DOT format generation
- Include source location tracking

**REFACTOR Phase** - Improve Quality
- Add caching for repeated visualizations
- Optimize for large ASTs (>1000 nodes)
- Add color coding for node types

**CLI Integration**:
```bash
# New command: ruchydbg parse-debug
ruchydbg parse-debug test.ruchy --format json
ruchydbg parse-debug test.ruchy --format graphviz > ast.dot
ruchydbg parse-debug test.ruchy --show-locations
ruchydbg parse-debug test.ruchy --step-by-step
```

---

#### DEBUGGER-051: Parser Error Recovery Testing

**RED Phase** - Write Failing Tests First
```rust
// tests/test_debugger_051_error_recovery.rs

#[test]
fn test_parser_recovers_from_missing_paren() {
    let source = "fun test( { return 1; }";
    let result = ruchyruchy::debugger::parse_with_recovery(source);

    // Should report error but continue parsing
    assert!(result.errors.len() > 0);
    assert!(result.partial_ast.is_some());
}

#[test]
fn test_parser_recovers_from_missing_semicolon() {
    let source = "let x = 1\nlet y = 2;";
    let result = ruchyruchy::debugger::parse_with_recovery(source);

    // Should insert implicit semicolon
    assert!(result.errors.len() == 0);
    assert!(result.ast.nodes().len() == 2);
}

#[test]
fn test_parser_suggests_fix() {
    let source = "fun test() { retrun 42; }";  // Typo: retrun
    let result = ruchyruchy::debugger::parse_with_suggestions(source);

    assert!(result.suggestions.contains("Did you mean 'return'?"));
}

#[test]
fn test_parser_multiple_errors() {
    let source = "fun test( { let x = ; }";
    let result = ruchyruchy::debugger::parse_with_recovery(source);

    // Should report multiple errors
    assert!(result.errors.len() >= 2);
}

#[test]
fn test_parser_error_quality() {
    let source = "fun test() { if x { } }";  // Missing condition parens
    let result = ruchyruchy::debugger::parse_with_detailed_errors(source);

    let error = &result.errors[0];
    assert!(error.line == 1);
    assert!(error.column > 0);
    assert!(error.message.contains("Expected '('"));
    assert!(error.help_text.is_some());
}
```

**Implementation Strategy**:
```rust
// src/debugger/error_recovery.rs

pub struct ParseResult {
    pub ast: Option<Ast>,
    pub partial_ast: Option<Ast>,
    pub errors: Vec<ParseError>,
    pub warnings: Vec<ParseWarning>,
    pub suggestions: Vec<String>,
}

pub struct ParseError {
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub help_text: Option<String>,
    pub severity: ErrorSeverity,
}

pub fn parse_with_recovery(source: &str) -> ParseResult {
    // Implement error recovery strategies:
    // 1. Skip to next synchronization point (semicolon, closing brace)
    // 2. Insert missing tokens (semicolons, parens)
    // 3. Suggest corrections (typos, common mistakes)
    todo!()
}
```

**GREEN Phase** - Implement Basic Recovery
- Skip to next statement on error
- Continue parsing after error
- Collect all errors (not just first)

**REFACTOR Phase** - Improve Recovery Quality
- Add intelligent error suggestions
- Implement token insertion heuristics
- Provide helpful error messages

---

### Phase 2: JIT Compiler Debugger (DEBUGGER-052)

**Objective**: Build tools to debug RuchyRuchy's JIT compilation pipeline.

#### DEBUGGER-052: Cranelift IR Inspector

**RED Phase** - Write Failing Tests First
```rust
// tests/test_debugger_052_jit_debug.rs

#[test]
fn test_jit_shows_cranelift_ir() {
    // Test: JIT debugger shows Cranelift IR
    let source = "fun main() { return 42; }";
    let ir = ruchyruchy::debugger::show_cranelift_ir(source, "main");

    assert!(ir.contains("function main()"));
    assert!(ir.contains("return"));
}

#[test]
fn test_jit_shows_compilation_stages() {
    // Test: Show AST -> IR -> Native compilation stages
    let source = "fun add(x, y) { return x + y; }";
    let stages = ruchyruchy::debugger::show_jit_stages(source, "add");

    assert!(stages.ast.is_some());
    assert!(stages.cranelift_ir.is_some());
    assert!(stages.machine_code.is_some());
}

#[test]
fn test_jit_disassembly() {
    // Test: Disassemble JIT-compiled native code
    let source = "fun square(x) { return x * x; }";
    let asm = ruchyruchy::debugger::jit_disassemble(source, "square");

    // Should show x86-64 assembly
    assert!(asm.contains("imul") || asm.contains("mul"));
}

#[test]
fn test_jit_optimization_levels() {
    // Test: Compare IR at different optimization levels
    let source = "fun test() { return 1 + 2; }";
    let ir_none = ruchyruchy::debugger::show_ir_opt_level(source, "test", 0);
    let ir_full = ruchyruchy::debugger::show_ir_opt_level(source, "test", 2);

    // Optimized should constant-fold to return 3
    assert!(ir_full.contains("iconst 3"));
}

#[test]
fn test_jit_compilation_errors() {
    // Test: Show detailed JIT compilation errors
    let source = "fun test() { return unknown_var; }";
    let result = ruchyruchy::debugger::jit_compile_debug(source, "test");

    assert!(result.is_err());
    assert!(result.unwrap_err().contains("unknown_var"));
}

#[test]
fn test_jit_performance_profile() {
    // Test: Profile JIT compilation time
    let source = "fun fib(n) { if n < 2 { return n; } return fib(n-1) + fib(n-2); }";
    let profile = ruchyruchy::debugger::jit_profile_compilation(source, "fib");

    assert!(profile.parse_time_us > 0);
    assert!(profile.ir_gen_time_us > 0);
    assert!(profile.compile_time_us > 0);
}

#[test]
fn test_jit_memory_usage() {
    // Test: Track memory usage during JIT compilation
    let source = "fun test() { let x = 1; return x; }";
    let mem = ruchyruchy::debugger::jit_memory_profile(source, "test");

    assert!(mem.peak_bytes > 0);
    assert!(mem.code_cache_bytes > 0);
}
```

**Implementation Strategy**:
```rust
// src/debugger/jit_inspector.rs

pub struct JitInspector {
    compiler: JitCompiler,
}

pub struct CompilationStages {
    pub ast: Option<String>,
    pub cranelift_ir: Option<String>,
    pub machine_code: Option<Vec<u8>>,
    pub disassembly: Option<String>,
}

impl JitInspector {
    pub fn show_ir(&mut self, source: &str, function: &str) -> Result<String, String> {
        // Parse source
        let mut parser = Parser::new(source);
        let ast = parser.parse()?;

        // Find function
        for node in ast.nodes() {
            if let AstNode::FunctionDef { name, params, body } = node {
                if name == function {
                    // Compile and extract IR
                    return self.extract_ir(params, body);
                }
            }
        }

        Err(format!("Function '{}' not found", function))
    }

    fn extract_ir(&mut self, params: &[String], body: &AstNode) -> Result<String, String> {
        // Use Cranelift's IR formatting
        todo!()
    }

    pub fn show_stages(&mut self, source: &str, function: &str) -> CompilationStages {
        // Show AST, IR, and machine code for each stage
        todo!()
    }
}
```

**GREEN Phase** - Implement Basic IR Inspection
- Extract and format Cranelift IR
- Show function signatures and basic blocks
- Display instruction stream

**REFACTOR Phase** - Add Advanced Features
- Show optimization passes
- Annotate IR with comments
- Add performance counters

**CLI Integration**:
```bash
# New command: ruchydbg jit-debug
ruchydbg jit-debug test.ruchy --function main --show-ir
ruchydbg jit-debug test.ruchy --function add --disassemble
ruchydbg jit-debug test.ruchy --function fib --stages
ruchydbg jit-debug test.ruchy --function test --profile
```

---

### Phase 3: Differential Testing Framework (DEBUGGER-053)

**Objective**: Automatically compare interpreter vs JIT execution to catch discrepancies.

#### DEBUGGER-053: Interpreter vs JIT Differential Tester

**RED Phase** - Write Failing Tests First
```rust
// tests/test_debugger_053_differential.rs

#[test]
fn test_differential_simple_arithmetic() {
    let source = "fun main() { return 10 + 5; }";
    let result = ruchyruchy::debugger::differential_test(source, "main", vec![]);

    assert!(result.interpreter_result == result.jit_result);
    assert_eq!(result.interpreter_result, Ok(15));
}

#[test]
fn test_differential_catches_jit_bug() {
    // Test: Catch case where JIT and interpreter disagree
    let source = "fun test() { let x = 10; return x * 2; }";
    let result = ruchyruchy::debugger::differential_test(source, "test", vec![]);

    if result.interpreter_result != result.jit_result {
        panic!("Differential test failed: {:?} vs {:?}",
               result.interpreter_result, result.jit_result);
    }
}

#[test]
fn test_differential_with_params() {
    let source = "fun add(x, y) { return x + y; }";
    let result = ruchyruchy::debugger::differential_test(
        source,
        "add",
        vec![Value::Integer(10), Value::Integer(20)]
    );

    assert!(result.interpreter_result == result.jit_result);
    assert_eq!(result.interpreter_result, Ok(30));
}

#[test]
fn test_differential_fuzzing() {
    // Test: Fuzz test with random inputs
    let source = "fun square(x) { return x * x; }";

    for i in -100..100 {
        let result = ruchyruchy::debugger::differential_test(
            source,
            "square",
            vec![Value::Integer(i)]
        );

        assert_eq!(result.interpreter_result, result.jit_result,
                   "Mismatch at input {}", i);
    }
}

#[test]
fn test_differential_performance_comparison() {
    let source = "fun loop_test() { let sum = 0; let i = 0; while i < 100 { sum = sum + i; i = i + 1; } return sum; }";
    let perf = ruchyruchy::debugger::differential_benchmark(source, "loop_test", 1000);

    assert!(perf.jit_time_us < perf.interpreter_time_us);
    assert_eq!(perf.interpreter_result, perf.jit_result);
}

#[test]
fn test_differential_coverage() {
    // Test: Ensure all AST nodes are tested
    let sources = vec![
        "fun test() { return 42; }",
        "fun test() { let x = 10; return x; }",
        "fun test() { if true { return 1; } return 0; }",
        "fun test() { let i = 0; while i < 5 { i = i + 1; } return i; }",
        // ... etc for all AST nodes
    ];

    for source in sources {
        let result = ruchyruchy::debugger::differential_test(source, "test", vec![]);
        assert_eq!(result.interpreter_result, result.jit_result);
    }
}
```

**Implementation Strategy**:
```rust
// src/debugger/differential.rs

pub struct DifferentialResult {
    pub interpreter_result: Result<i64, String>,
    pub jit_result: Result<i64, String>,
    pub interpreter_time_us: u64,
    pub jit_time_us: u64,
    pub match_status: MatchStatus,
}

pub enum MatchStatus {
    Perfect,      // Results match exactly
    Mismatch,     // Results differ (BUG!)
    InterpreterError,  // Only interpreter failed
    JitError,     // Only JIT failed
    BothError,    // Both failed (expected)
}

pub fn differential_test(
    source: &str,
    function: &str,
    args: Vec<Value>
) -> DifferentialResult {
    // Run through interpreter
    let interp_start = Instant::now();
    let interp_result = run_interpreter(source, function, &args);
    let interp_time = interp_start.elapsed().as_micros() as u64;

    // Run through JIT
    let jit_start = Instant::now();
    let jit_result = run_jit(source, function, &args);
    let jit_time = jit_start.elapsed().as_micros() as u64;

    // Compare results
    let match_status = compare_results(&interp_result, &jit_result);

    DifferentialResult {
        interpreter_result: interp_result,
        jit_result,
        interpreter_time_us: interp_time,
        jit_time_us: jit_time,
        match_status,
    }
}
```

**GREEN Phase** - Implement Basic Comparison
- Run same code through interpreter and JIT
- Compare return values
- Report mismatches

**REFACTOR Phase** - Add Comprehensive Testing
- Property-based testing with random inputs
- Automated fuzzing
- Coverage-guided test generation

**CLI Integration**:
```bash
# New command: ruchydbg diff-test
ruchydbg diff-test test.ruchy --function main
ruchydbg diff-test test.ruchy --function add --args 10,20
ruchydbg diff-test test.ruchy --fuzz --iterations 10000
ruchydbg diff-test test.ruchy --coverage-report
```

---

### Phase 4: Integration & Automation (DEBUGGER-054)

#### DEBUGGER-054: Automated Quality Gates

**Objective**: Run all debugger validations in CI/CD pipeline.

**RED Phase** - Write Failing Tests First
```rust
// tests/test_debugger_054_quality_gates.rs

#[test]
fn test_all_parser_tests_visualized() {
    // Test: All parser tests produce valid AST visualizations
    let test_files = glob("tests/test_interp_*.rs").unwrap();

    for file in test_files {
        let ast_viz = ruchyruchy::debugger::visualize_test_file(&file);
        assert!(ast_viz.is_ok());
    }
}

#[test]
fn test_all_jit_tests_inspected() {
    // Test: All JIT tests produce valid IR dumps
    let test_files = glob("tests/test_jit_*.rs").unwrap();

    for file in test_files {
        let ir = ruchyruchy::debugger::inspect_test_ir(&file);
        assert!(ir.is_ok());
    }
}

#[test]
fn test_differential_coverage_complete() {
    // Test: Differential testing covers all AST nodes
    let coverage = ruchyruchy::debugger::differential_coverage_report();

    assert_eq!(coverage.nodes_covered, coverage.total_nodes);
    assert_eq!(coverage.percentage, 100.0);
}

#[test]
fn test_no_interpreter_jit_mismatches() {
    // Test: No known mismatches between interpreter and JIT
    let mismatches = ruchyruchy::debugger::find_all_mismatches();

    assert_eq!(mismatches.len(), 0,
               "Found {} interpreter/JIT mismatches", mismatches.len());
}
```

**Implementation Strategy**:
```bash
#!/bin/bash
# scripts/run-debugger-quality-gates.sh

set -euo pipefail

echo "🔍 Running Debugger Quality Gates..."

# 1. Parser debugging
echo "📊 Checking parser debug tools..."
cargo test --test test_debugger_050_ast_viz
cargo test --test test_debugger_051_error_recovery

# 2. JIT debugging
echo "⚡ Checking JIT debug tools..."
cargo test --test test_debugger_052_jit_debug

# 3. Differential testing
echo "🔬 Running differential tests..."
cargo test --test test_debugger_053_differential

# 4. Integration checks
echo "✅ Running quality gates..."
cargo test --test test_debugger_054_quality_gates

echo "✅ All debugger quality gates passed!"
```

**GREEN Phase** - Implement Quality Gates
- Add script to run all debugger tests
- Integrate with pre-commit hooks
- Generate coverage reports

**REFACTOR Phase** - Automate Everything
- Add to CI/CD pipeline
- Generate HTML reports
- Send notifications on failures

---

## Implementation Timeline

### Sprint 1 (1-2 weeks): DEBUGGER-050 - Parser Debugger
- RED: Write 7 failing tests for AST visualization
- GREEN: Implement JSON and Graphviz output
- REFACTOR: Add performance optimizations
- COMMIT: "DEBUGGER-050: Add AST visualization debugger"

### Sprint 2 (1-2 weeks): DEBUGGER-051 - Error Recovery
- RED: Write 5 failing tests for error recovery
- GREEN: Implement basic error recovery
- REFACTOR: Improve error messages and suggestions
- COMMIT: "DEBUGGER-051: Add parser error recovery testing"

### Sprint 3 (1-2 weeks): DEBUGGER-052 - JIT Inspector
- RED: Write 7 failing tests for JIT debugging
- GREEN: Implement Cranelift IR extraction
- REFACTOR: Add disassembly and profiling
- COMMIT: "DEBUGGER-052: Add JIT compilation debugger"

### Sprint 4 (1-2 weeks): DEBUGGER-053 - Differential Testing
- RED: Write 6 failing tests for differential testing
- GREEN: Implement interpreter vs JIT comparison
- REFACTOR: Add fuzzing and coverage-guided testing
- COMMIT: "DEBUGGER-053: Add differential testing framework"

### Sprint 5 (1 week): DEBUGGER-054 - Quality Gates
- RED: Write 4 failing tests for automation
- GREEN: Implement quality gate scripts
- REFACTOR: Add CI/CD integration
- COMMIT: "DEBUGGER-054: Add automated debugger quality gates"

**Total Timeline**: 6-10 weeks to completion

---

## Success Criteria

### Functional Requirements
- ✅ Parser debugger shows AST in JSON and Graphviz formats
- ✅ JIT debugger shows Cranelift IR and disassembly
- ✅ Differential tester finds interpreter/JIT discrepancies
- ✅ Error recovery tester validates parser resilience
- ✅ All tools integrated into `ruchydbg` CLI

### Quality Requirements
- ✅ 100% test coverage for all debugger features
- ✅ All tests pass in <5 seconds
- ✅ Zero known interpreter/JIT mismatches
- ✅ Parser error messages are helpful and actionable
- ✅ Documentation complete for all new commands

### Integration Requirements
- ✅ Pre-commit hooks run debugger tests
- ✅ CI/CD pipeline includes debugger validation
- ✅ Quality gates block commits with regressions
- ✅ HTML reports generated automatically

---

## Risk Assessment

### High Risk
- **Cranelift IR extraction complexity** - May require deep Cranelift internals knowledge
  - Mitigation: Start with simple cases, use Cranelift's built-in formatting

### Medium Risk
- **Differential testing performance** - Running both interpreter and JIT may be slow
  - Mitigation: Parallelize tests, use caching, run in CI only

### Low Risk
- **AST visualization** - Well-understood problem with existing solutions
  - Mitigation: Use standard JSON/Graphviz libraries

---

## Alternatives Considered

### 1. Use External Debuggers (GDB, LLDB)
- **Rejected**: Only works for runtime debugging, doesn't help with parser bugs

### 2. Add Logging to Parser/JIT
- **Rejected**: Clutters code, hard to maintain, not interactive

### 3. Use Ruchy's Built-in Debugger
- **Rejected**: Ruchy debugger is for Ruchy programs, not for debugging Ruchy itself

### 4. Manual Inspection Only
- **Rejected**: Not scalable, error-prone, no automation

---

## Appendix A: CLI Command Reference

```bash
# Parser Debugging
ruchydbg parse-debug <file> [--format json|graphviz|tree]
ruchydbg parse-debug <file> --show-locations
ruchydbg parse-debug <file> --step-by-step
ruchydbg parse-debug <file> --with-types

# JIT Debugging
ruchydbg jit-debug <file> --function <name> --show-ir
ruchydbg jit-debug <file> --function <name> --disassemble
ruchydbg jit-debug <file> --function <name> --stages
ruchydbg jit-debug <file> --function <name> --profile

# Differential Testing
ruchydbg diff-test <file> --function <name>
ruchydbg diff-test <file> --function <name> --args <arg1,arg2,...>
ruchydbg diff-test <file> --fuzz --iterations <N>
ruchydbg diff-test <file> --coverage-report

# Quality Gates
ruchydbg quality-gates --all
ruchydbg quality-gates --parser
ruchydbg quality-gates --jit
ruchydbg quality-gates --differential
```

---

## Appendix B: Test File Structure

```
tests/
├── test_debugger_050_ast_viz.rs          # AST visualization tests
├── test_debugger_051_error_recovery.rs   # Parser error recovery tests
├── test_debugger_052_jit_debug.rs        # JIT debugging tests
├── test_debugger_053_differential.rs     # Differential testing tests
└── test_debugger_054_quality_gates.rs    # Integration/automation tests

src/debugger/
├── mod.rs                                # Module exports
├── ast_visualizer.rs                     # AST visualization
├── error_recovery.rs                     # Parser error recovery
├── jit_inspector.rs                      # JIT compilation debugging
├── differential.rs                       # Differential testing
└── quality_gates.rs                      # Automation and integration
```

---

## Appendix C: Ticket Dependencies

```
DEBUGGER-050 (AST Visualization)
  ↓
DEBUGGER-051 (Error Recovery) ← depends on AST viz
  ↓
DEBUGGER-052 (JIT Inspector) ← independent
  ↓
DEBUGGER-053 (Differential Testing) ← depends on JIT inspector
  ↓
DEBUGGER-054 (Quality Gates) ← depends on all above
```

**Critical Path**: DEBUGGER-050 → DEBUGGER-051 → DEBUGGER-053 → DEBUGGER-054

---

## Conclusion

This specification addresses the critical gap where `ruchydbg` only tests runtime behavior and provides no tools for debugging RuchyRuchy's own parser, evaluator, or JIT compiler.

**Key Deliverables**:
1. Parser debugger with AST visualization
2. JIT debugger with IR inspection
3. Differential testing framework
4. Automated quality gates

**Timeline**: 6-10 weeks following EXTREME TDD methodology

**Status**: Ready for implementation - all tickets defined with RED-GREEN-REFACTOR cycles
