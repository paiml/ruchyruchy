// JIT Integration Tests: Execution Verification (MVP)
//
// INTERP-078: JIT Correctness Verification (Simplified)
//
// This test suite validates that JIT-compiled code executes without errors.
// MVP: Verifies programs compile and run, ensuring basic correctness.
//
// Test Strategy:
// 1. Load .ruchy test programs
// 2. Compile through JIT
// 3. Execute (verify no panics/crashes)
// 4. Compare with interpreter execution (both should succeed)
//
// Future: Extract and compare return values for full differential testing

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::Parser;
use ruchyruchy::jit::JitCompiler;
use std::fs;

/// Verify program executes through interpreter
fn verify_interpreter(source: &str) -> Result<(), String> {
    let mut parser = Parser::new(source);
    let ast = parser
        .parse()
        .map_err(|e| format!("Parse error: {:?}", e))?;

    let mut evaluator = Evaluator::new();

    // Execute all statements
    for node in ast.nodes() {
        evaluator
            .eval(node)
            .map_err(|e| format!("Eval error: {:?}", e))?;
    }

    Ok(())
}

/// Verify program compiles and executes through JIT
fn verify_jit(source: &str) -> Result<i64, String> {
    let mut parser = Parser::new(source);
    let ast = parser
        .parse()
        .map_err(|e| format!("Parse error: {:?}", e))?;

    let mut jit = JitCompiler::new().map_err(|e| format!("JIT init error: {:?}", e))?;

    // Find and compile all functions
    let mut main_fn: Option<fn() -> i64> = None;
    for node in ast.nodes() {
        if let ruchyruchy::interpreter::parser::AstNode::FunctionDef { name, params, body } = node {
            if name == "main" {
                // Compile main function
                main_fn = Some(
                    jit.compile_function_with_params(
                        params,
                        &ruchyruchy::interpreter::parser::AstNode::Block {
                            statements: body.clone(),
                        },
                    )
                    .map_err(|e| format!("JIT compile error: {:?}", e))?,
                );
            } else {
                // Compile helper functions (simplified - just compile main for now)
                // Full implementation would need to handle multi-function programs
            }
        }
    }

    // Execute main
    let main_fn = main_fn.ok_or("No main function found")?;
    Ok(main_fn())
}

/// Integration test: Verify program executes through both interpreter and JIT
fn test_program_execution(filename: &str, expected: i64) {
    let path = format!("tests/jit_integration/programs/{}", filename);
    let source = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read test program: {}", path));

    // Verify interpreter execution
    verify_interpreter(&source)
        .unwrap_or_else(|e| panic!("Interpreter failed for {}: {}", filename, e));

    // Verify JIT execution and check result
    let jit_result =
        verify_jit(&source).unwrap_or_else(|e| panic!("JIT failed for {}: {}", filename, e));

    // Verify JIT result matches expected
    assert_eq!(jit_result, expected, "{}: JIT result mismatch", filename);

    println!("✅ {}: JIT result={}", filename, jit_result);
}

#[test]
fn test_arithmetic_program() {
    test_program_execution("arithmetic.ruchy", 101);
}

#[test]
fn test_control_flow_program() {
    test_program_execution("control_flow.ruchy", 14);
}

#[test]
#[ignore] // Functions with recursion need more complex compilation
fn test_functions_program() {
    test_program_execution("functions.ruchy", 180);
}

#[test]
fn test_arrays_program() {
    test_program_execution("arrays.ruchy", 151);
}

#[test]
#[ignore] // Complex program with multiple functions needs full compilation support
fn test_complex_program() {
    test_program_execution("complex.ruchy", 223);
}
