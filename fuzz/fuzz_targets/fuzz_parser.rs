// DEBUGGER-048: Parser Fuzz Target
//
// This fuzz target tests the Ruchy parser with random byte sequences interpreted as Ruchy source code.
// Coverage-guided mutation fuzzing discovers edge cases that grammar-based fuzzing misses.
//
// Objective: Zero crashes, panics, or hangs when parsing arbitrary input.
//
// Strategy:
// 1. Convert random bytes to UTF-8 string (with lossy conversion)
// 2. Attempt to parse as Ruchy source code
// 3. Verify parser handles all inputs gracefully (error or success, never crash)

#![no_main]

use libfuzzer_sys::fuzz_target;
use ruchyruchy::interpreter::parser::Parser;

fuzz_target!(|data: &[u8]| {
    // Convert random bytes to UTF-8 string (lossy - replaces invalid UTF-8 with �)
    let input = String::from_utf8_lossy(data);

    // Skip empty inputs (no interesting behavior)
    if input.is_empty() {
        return;
    }

    // Attempt to parse the random input
    let mut parser = Parser::new(&input);

    // Parse should either succeed or fail gracefully
    // NEVER crash, panic, or hang
    match parser.parse() {
        Ok(program) => {
            // Success: Parser accepted the input
            // Verify program has reasonable structure
            let node_count = program.nodes().len();

            // Sanity check: Ensure node count is reasonable
            // (Prevents pathological cases like infinite loops in parser)
            assert!(
                node_count < 100_000,
                "Parser generated unreasonably large AST ({} nodes) - possible infinite loop",
                node_count
            );
        }
        Err(_error) => {
            // Failure: Parser rejected the input
            // This is expected and acceptable for invalid input
            // No assertion - error handling is correct behavior
        }
    }

    // If we reach here, parser handled input gracefully ✓
});
