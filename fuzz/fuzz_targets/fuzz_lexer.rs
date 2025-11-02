// DEBUGGER-048: Lexer Fuzz Target
//
// This fuzz target tests the Ruchy lexer/scanner with random byte sequences.
// Tests tokenization of arbitrary input at the lowest level of the interpreter.
//
// Objective: Zero crashes, panics, or hangs when tokenizing arbitrary byte sequences.
//
// Strategy:
// 1. Convert random bytes to UTF-8 string (lossy)
// 2. Attempt to tokenize the input
// 3. Verify lexer handles all inputs gracefully (tokens or error, never crash)
//
// Note: The lexer is part of the Parser in our architecture.
// We test it by parsing and examining tokenization behavior.

#![no_main]

use libfuzzer_sys::fuzz_target;
use ruchyruchy::interpreter::parser::Parser;

fuzz_target!(|data: &[u8]| {
    // Convert random bytes to UTF-8 string (lossy)
    let input = String::from_utf8_lossy(data);

    // Skip empty inputs
    if input.is_empty() {
        return;
    }

    // Create parser (which includes lexer/scanner)
    let mut parser = Parser::new(&input);

    // Attempt to parse (which triggers tokenization)
    // Lexer should handle all byte sequences gracefully
    match parser.parse() {
        Ok(program) => {
            // Success: Lexer tokenized and parser accepted
            let node_count = program.nodes().len();

            // Sanity check: Prevent pathological tokenization
            // (e.g., infinite loop generating tokens)
            assert!(
                node_count < 100_000,
                "Lexer/Parser generated unreasonably large AST ({} nodes)",
                node_count
            );
        }
        Err(_error) => {
            // Failure: Lexer or parser rejected input
            // This is expected for invalid byte sequences
            // Examples: invalid UTF-8, unknown characters, syntax errors
            // No assertion - error handling is correct behavior
        }
    }

    // Alternative: If we had direct lexer access, we would fuzz it separately:
    // let scanner = Scanner::new(&input);
    // for token in scanner.scan_tokens() {
    //     // Process token
    // }

    // If we reach here, lexer handled input gracefully ✓
});
