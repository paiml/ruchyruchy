// DEBUGGER-048: Evaluator Fuzz Target
//
// This fuzz target tests the Ruchy evaluator with random Ruchy programs.
// It combines parsing and evaluation to discover runtime crashes in the interpreter.
//
// Objective: Zero crashes, panics, or hangs when evaluating arbitrary valid/invalid programs.
//
// Strategy:
// 1. Parse random input into AST (if possible)
// 2. Attempt to evaluate the AST
// 3. Verify evaluator handles all cases gracefully (value, error, or controlled panic, never uncontrolled crash)

#![no_main]

use libfuzzer_sys::fuzz_target;
use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::Parser;

fuzz_target!(|data: &[u8]| {
    // Convert random bytes to UTF-8 string
    let input = String::from_utf8_lossy(data);

    // Skip empty inputs
    if input.is_empty() {
        return;
    }

    // Parse the input
    let mut parser = Parser::new(&input);

    let program = match parser.parse() {
        Ok(prog) => prog,
        Err(_) => {
            // Parsing failed - skip evaluation
            // This is expected for invalid input
            return;
        }
    };

    // Successfully parsed - now fuzz the evaluator
    let mut evaluator = Evaluator::new();

    // Evaluate each statement in the program
    for statement in program.nodes() {
        // Evaluation should either succeed, return error, or panic gracefully
        // NEVER cause undefined behavior or segfault
        match evaluator.eval(statement) {
            Ok(_value) => {
                // Success: Evaluator produced a value
                // No assertion - this is correct behavior
            }
            Err(_error) => {
                // Failure: Evaluator rejected the statement
                // This is expected for semantically invalid programs
                // (e.g., type errors, undefined variables)
                // No assertion - error handling is correct behavior
            }
        }
    }

    // If we reach here, evaluator handled program gracefully ✓
});
