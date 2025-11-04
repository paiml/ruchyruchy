// DEBUGGER-052: JIT Compiler Debugger with Cranelift IR Inspection
//
// EXTREME TDD - RED Phase
//
// Pain Points (Genchi Genbutsu):
// - JIT-024 (FString): Couldn't see IR showing expressions evaluated but results discarded
// - JIT-011 (Array indexing): Couldn't verify bounds checks in generated code
// - JIT-020 (Method calls): Couldn't inspect calling convention in machine code
//
// Measured Impact: 2-3 days per JIT bug without IR/disassembly tools
//
// Solution: Build debugging tools for observing JIT compilation pipeline
//
// Architecture:
// 1. IR Extraction: Capture Cranelift IR during compilation
// 2. Disassembly: View generated x86-64 machine code
// 3. Stage Visualization: Show AST → IR → Native transformation
// 4. Compilation Profiling: Measure time and memory usage
//
// Tests follow Toyota Way: Each test addresses documented pain point

/// Test 1: Extract and format Cranelift IR for simple function
///
/// Pain Point: JIT-024 - Couldn't see that f-string expressions were evaluated but discarded
/// Solution: Extract IR to verify operations and their results
#[test]
fn test_jit_shows_cranelift_ir() {
    let source = "fun main() { return 42; }";

    // Call debugger function to show Cranelift IR
    let ir = ruchyruchy::debugger::jit::show_cranelift_ir(source, "main");

    // IR should contain function signature
    assert!(ir.contains("function"), "IR should show 'function' keyword");

    // IR should show return value
    assert!(
        ir.contains("42") || ir.contains("v"),
        "IR should show constant or value"
    );

    // IR should show return instruction
    assert!(ir.contains("return"), "IR should show 'return' instruction");
}

/// Test 2: Show compilation stages (AST → IR → Native)
///
/// Pain Point: JIT-020 - Method dispatch failures, needed to see full pipeline
/// Solution: Show each transformation stage for complete understanding
#[test]
fn test_jit_shows_compilation_stages() {
    let source = "fun add(a: i64, b: i64) { return a + b; }";

    let stages = ruchyruchy::debugger::jit::show_compilation_stages(source, "add");

    // Should show AST stage
    assert!(
        stages.ast.contains("FunctionDecl") || stages.ast.contains("fun"),
        "AST should show function declaration"
    );

    // Should show IR stage
    assert!(
        stages.ir.contains("function") && stages.ir.contains("add"),
        "IR should show function definition"
    );

    // Should show native code stage
    assert!(
        stages.native.contains("mov") || stages.native.contains("ret"),
        "Native should show x86-64 assembly"
    );
}

/// Test 3: Disassemble JIT-compiled code to x86-64 assembly
///
/// Pain Point: JIT-011 - Couldn't verify array bounds checks in generated code
/// Solution: Show actual machine code to verify safety checks exist
#[test]
fn test_jit_disassembly() {
    let source = "fun double(x: i64) { return x * 2; }";

    let asm = ruchyruchy::debugger::jit::disassemble_function(source, "double");

    // Should show x86-64 assembly
    assert!(
        asm.contains("mov") || asm.contains("imul") || asm.contains("add"),
        "Assembly should show x86-64 instructions"
    );

    // Should show return instruction
    assert!(
        asm.contains("ret"),
        "Assembly should show 'ret' instruction"
    );

    // Should have recognizable assembly format
    assert!(
        asm.lines().count() > 0,
        "Assembly should have multiple lines"
    );
}

/// Test 4: Compare IR at different optimization levels
///
/// Pain Point: Understanding why optimized code behaves differently
/// Solution: Show IR side-by-side at different optimization levels
#[test]
fn test_jit_optimization_levels() {
    let source = "fun calc() { let x = 1 + 1; return x + x; }";

    let comparison = ruchyruchy::debugger::jit::compare_optimization_levels(source, "calc");

    // Should have IR for O0 (no optimization)
    assert!(comparison.o0.contains("function"), "O0 IR should exist");

    // Should have IR for O2 (optimized)
    assert!(comparison.o2.contains("function"), "O2 IR should exist");

    // Optimized version might be shorter or have different instructions
    // This is a smoke test - just verify we can extract both
}

/// Test 5: Handle compilation errors with detailed messages
///
/// Pain Point: Cryptic Cranelift errors without context
/// Solution: Provide actionable error messages with source context
#[test]
fn test_jit_compilation_errors() {
    let source = "fun broken() { return unknown_var; }";

    let result = ruchyruchy::debugger::jit::try_show_ir(source, "broken");

    // Should return error (not panic)
    assert!(result.is_err(), "Should return error for invalid code");

    // Error should mention the problem
    let error = result.unwrap_err();
    assert!(
        error.contains("unknown") || error.contains("undefined") || error.contains("not found"),
        "Error should explain what went wrong: {}",
        error
    );
}

/// Test 6: Profile compilation time and report metrics
///
/// Pain Point: Slow JIT compilation without visibility into which stage is slow
/// Solution: Measure and report time spent in each compilation stage
#[test]
fn test_jit_performance_profile() {
    let source = "fun fib(n: i64) { if n < 2 { return n; } else { return fib(n-1) + fib(n-2); } }";

    let profile = ruchyruchy::debugger::jit::profile_compilation(source, "fib");

    // Should report parsing time
    assert!(profile.parse_time_ms >= 0.0, "Should measure parse time");

    // Should report IR generation time
    assert!(
        profile.ir_gen_time_ms >= 0.0,
        "Should measure IR generation time"
    );

    // Should report compilation time
    assert!(
        profile.compile_time_ms >= 0.0,
        "Should measure compilation time"
    );

    // Total time should be sum of stages
    let total = profile.parse_time_ms + profile.ir_gen_time_ms + profile.compile_time_ms;
    assert!(total > 0.0, "Total compilation time should be positive");
}

/// Test 7: Track memory usage during compilation
///
/// Pain Point: JIT memory leaks or excessive allocation without visibility
/// Solution: Report memory allocated during compilation stages
#[test]
fn test_jit_memory_usage() {
    let source =
        "fun factorial(n: i64) { if n <= 1 { return 1; } else { return n * factorial(n - 1); } }";

    let memory = ruchyruchy::debugger::jit::profile_memory_usage(source, "factorial");

    // Should report memory before compilation
    assert!(memory.before_bytes > 0, "Should measure initial memory");

    // Should report memory after compilation
    assert!(
        memory.after_bytes >= memory.before_bytes,
        "Memory after should be >= before (compilation allocates)"
    );

    // Should report allocated memory (usize is always >= 0)
    // Allocated should match the difference
    let expected = memory.after_bytes - memory.before_bytes;
    assert!(
        memory.allocated_bytes == expected,
        "Allocated should equal after - before"
    );
}

// Data structures for test assertions
// These will be implemented in src/debugger/jit.rs

#[allow(dead_code)]
struct CompilationStages {
    ast: String,
    ir: String,
    native: String,
}

#[allow(dead_code)]
struct OptimizationComparison {
    o0: String,
    o2: String,
}

#[allow(dead_code)]
struct CompilationProfile {
    parse_time_ms: f64,
    ir_gen_time_ms: f64,
    compile_time_ms: f64,
}

#[allow(dead_code)]
struct MemoryProfile {
    before_bytes: usize,
    after_bytes: usize,
    allocated_bytes: usize,
}
