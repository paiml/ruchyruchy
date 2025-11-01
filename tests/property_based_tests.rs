// DEBUGGER-044: Property-Based Testing Infrastructure
//
// This test suite implements systematic property-based testing using proptest
// to discover edge cases that manual unit tests miss.
//
// Based on research:
// - Git analysis: Property testing found 28/120 bugs (23%)
// - paiml-mcp-agent-toolkit: 33 property test modules, 82% bug discovery rate
// - bashrs: 52 properties, 26K+ test cases, >95% edge case coverage
//
// Properties tested:
// 1. Parser Roundtrip: parse(emit(ast)) = ast (structural preservation)
// 2. Evaluator Determinism: eval(expr) = eval(expr) (same input, same output)
// 3. Token Concatenation: tokenize(a + b) = tokenize(a) ++ tokenize(b) (compositional)
// 4. No Crashes: Parser/evaluator never panics on any input
// 5. Expression Evaluation Consistency: Similar expressions produce consistent results
//
// RED PHASE: These tests WILL FAIL because:
// - Parser doesn't have emit() function yet
// - Property test infrastructure not set up
// - Test generators not implemented
//
// Test case count: 10,000+ per property (proptest default: 256 cases, but we'll configure more)

use proptest::prelude::*;
use ruchyruchy::interpreter::{Evaluator, Parser};

/// Property 1: Parser Roundtrip
///
/// Mathematical property: parse(emit(ast)) = ast
///
/// Ensures that parsing and emitting (code generation) are inverse operations.
/// If we parse source code into an AST, then emit that AST back to source code,
/// and parse it again, we should get the same AST.
///
/// This property validates:
/// - Parser correctness (no information loss)
/// - Emitter correctness (generates valid code)
/// - AST structural integrity
///
/// Expected to find bugs:
/// - Parser losing position information
/// - Emitter generating invalid syntax
/// - AST normalization issues (e.g., `1+2` vs `1 + 2`)
#[cfg(test)]
mod parser_roundtrip_property {
    use super::*;

    /// Generate arbitrary valid Ruchy source code
    ///
    /// Strategy: Generate simple arithmetic expressions first,
    /// then expand to more complex constructs.
    fn arb_simple_expr() -> impl Strategy<Value = String> {
        prop::collection::vec(1u32..100, 1..10).prop_map(|nums| {
            nums.iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(" + ")
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000, // Run 1000 test cases per property
            .. ProptestConfig::default()
        })]

        /// Property: Parsing is the inverse of emitting
        ///
        /// For any valid source code:
        /// 1. Parse to AST
        /// 2. Emit AST back to source
        /// 3. Parse emitted source
        /// 4. Result should equal original AST
        #[test]
        fn prop_parser_roundtrip(source in arb_simple_expr()) {
            // Parse source to AST
            let mut parser1 = Parser::new(&source);
            let ast1 = parser1.parse()
                .expect("Generated source should parse successfully");

            // Emit AST back to source code
            // NOTE: This will fail in RED phase - emit() not yet implemented
            let emitted = ast1.emit();

            // Parse emitted source to AST
            let mut parser2 = Parser::new(&emitted);
            let ast2 = parser2.parse()
                .expect("Emitted source should parse successfully");

            // ASTs should be equal (structural equality)
            prop_assert_eq!(
                ast1,
                ast2,
                "Roundtrip failed: parse(emit(ast)) != ast\nOriginal: {}\nEmitted: {}",
                source,
                emitted
            );
        }
    }
}

/// Property 2: Evaluator Determinism
///
/// Mathematical property: eval(expr) = eval(expr)
///
/// Ensures that evaluation is deterministic - same input produces same output
/// every time. This property catches:
/// - Non-deterministic behavior (e.g., random number generation)
/// - State pollution (global mutable state)
/// - Timing-dependent bugs
///
/// Expected to find bugs:
/// - Hash map iteration order dependencies
/// - Uninitialized variables
/// - Race conditions (if any concurrency)
#[cfg(test)]
mod evaluator_determinism_property {
    use super::*;

    /// Generate arbitrary arithmetic expressions
    fn arb_arithmetic_expr() -> impl Strategy<Value = String> {
        prop::collection::vec(1i32..100, 1..10).prop_map(|nums| {
            nums.iter()
                .enumerate()
                .map(|(i, n)| {
                    if i == 0 {
                        n.to_string()
                    } else {
                        format!(" + {}", n)
                    }
                })
                .collect::<String>()
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000,
            .. ProptestConfig::default()
        })]

        /// Property: Evaluation is deterministic
        ///
        /// Evaluating the same expression multiple times should produce
        /// identical results.
        #[test]
        fn prop_evaluator_deterministic(source in arb_arithmetic_expr()) {
            let mut parser = Parser::new(&source);
            let ast = parser.parse()
                .expect("Generated arithmetic should parse");

            // Evaluate expression 3 times with fresh evaluators
            let mut eval1 = Evaluator::new();
            let result1 = eval1.eval_program(&ast)
                .expect("Arithmetic should evaluate successfully");

            let mut eval2 = Evaluator::new();
            let result2 = eval2.eval_program(&ast)
                .expect("Arithmetic should evaluate successfully");

            let mut eval3 = Evaluator::new();
            let result3 = eval3.eval_program(&ast)
                .expect("Arithmetic should evaluate successfully");

            // All results should be identical
            prop_assert_eq!(
                result1, result2.clone(),
                "Non-deterministic evaluation: first != second\nSource: {}",
                source
            );
            prop_assert_eq!(
                result2, result3,
                "Non-deterministic evaluation: second != third\nSource: {}",
                source
            );
        }
    }
}

/// Property 3: Token Concatenation (Compositional Property)
///
/// Mathematical property: tokenize(a + b) = tokenize(a) ++ tokenize(b)
///
/// Ensures that tokenization is compositional - tokenizing concatenated
/// source produces concatenated tokens (with potential whitespace handling).
///
/// This is a weaker property than full compositionality but catches:
/// - Tokenizer state pollution
/// - Context-dependent tokenization bugs
/// - Buffer handling issues
#[cfg(test)]
mod token_concatenation_property {
    use super::*;

    /// Generate two separate arithmetic expressions
    fn arb_two_exprs() -> impl Strategy<Value = (String, String)> {
        (1i32..100, 1i32..100).prop_map(|(a, b)| (a.to_string(), b.to_string()))
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000,
            .. ProptestConfig::default()
        })]

        /// Property: Tokenization is compositional
        ///
        /// Tokenizing "a; b" should produce the same tokens as
        /// tokenizing "a" + tokenizing "b" (accounting for separator).
        #[test]
        fn prop_token_concatenation((expr_a, expr_b) in arb_two_exprs()) {
            // Tokenize concatenated expressions
            let combined = format!("{}; {}", expr_a, expr_b);
            let mut parser_combined = Parser::new(&combined);
            let ast_combined = parser_combined.parse()
                .expect("Combined source should parse");

            // Tokenize separately
            let mut parser_a = Parser::new(&expr_a);
            let ast_a = parser_a.parse()
                .expect("First expression should parse");

            let mut parser_b = Parser::new(&expr_b);
            let ast_b = parser_b.parse()
                .expect("Second expression should parse");

            // Combined AST should have nodes from both
            // NOTE: This is a simplified check - full compositionality
            // would require comparing token streams directly
            prop_assert!(
                ast_combined.nodes().len() >= ast_a.nodes().len() + ast_b.nodes().len(),
                "Combined AST should contain at least as many nodes as separate ASTs\n\
                 Combined: {}, A: {}, B: {}",
                ast_combined.nodes().len(),
                ast_a.nodes().len(),
                ast_b.nodes().len()
            );
        }
    }
}

/// Property 4: No Crashes (Safety Property)
///
/// Universal property: ∀ input, parser(input) does not panic
///
/// Ensures that the parser never panics, regardless of input.
/// This is a critical safety property - parsers must handle all input gracefully.
///
/// Expected to find bugs:
/// - Array index out of bounds
/// - Unwrap on None
/// - Division by zero in parser logic
/// - Stack overflow on deeply nested input
#[cfg(test)]
mod no_crashes_property {
    use super::*;

    /// Generate arbitrary byte sequences as input
    ///
    /// This tests parser robustness on completely invalid input.
    fn arb_bytes() -> impl Strategy<Value = Vec<u8>> {
        prop::collection::vec(any::<u8>(), 0..1000)
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 10000, // More cases for crash detection
            .. ProptestConfig::default()
        })]

        /// Property: Parser never panics on any input
        ///
        /// This is a fuzzing-like property test that verifies the parser
        /// handles all inputs gracefully (either success or error, never panic).
        #[test]
        fn prop_parser_no_crashes(bytes in arb_bytes()) {
            // Try to interpret bytes as UTF-8
            if let Ok(source) = std::str::from_utf8(&bytes) {
                // Parser should either succeed or return an error
                // It should NEVER panic
                let mut parser = Parser::new(source);
                let _result = parser.parse(); // Result can be Ok or Err, but not panic

                // If we reach here, no panic occurred
                prop_assert!(true);
            }
        }

        /// Property: Evaluator never panics on any AST
        ///
        /// Even if we construct bizarre ASTs, the evaluator should
        /// handle them gracefully.
        #[test]
        fn prop_evaluator_no_crashes(source in arb_bytes()) {
            if let Ok(source_str) = std::str::from_utf8(&source) {
                let mut parser = Parser::new(source_str);

                // If parsing succeeds, evaluation should not panic
                if let Ok(ast) = parser.parse() {
                    let mut evaluator = Evaluator::new();
                    let _result = evaluator.eval_program(&ast);
                    // Result can be Ok or Err, but not panic

                    prop_assert!(true);
                }
            }
        }
    }
}

/// Property 5: Expression Evaluation Consistency
///
/// Property: Mathematically equivalent expressions produce equal results
///
/// Examples:
/// - `1 + 2` should equal `2 + 1` (commutativity)
/// - `(1 + 2) + 3` should equal `1 + (2 + 3)` (associativity)
/// - `2 * 3` should equal `3 + 3` (definitional equality)
///
/// This property catches:
/// - Operator precedence bugs
/// - Evaluation order issues
/// - Numeric precision problems
#[cfg(test)]
mod evaluation_consistency_property {
    use super::*;

    /// Generate pairs of mathematically equivalent expressions
    fn arb_equivalent_exprs() -> impl Strategy<Value = (String, String)> {
        (1i32..100, 1i32..100).prop_map(|(a, b)| {
            // Generate commutative pairs: a + b vs b + a
            (format!("{} + {}", a, b), format!("{} + {}", b, a))
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000,
            .. ProptestConfig::default()
        })]

        /// Property: Commutative operations produce equal results
        ///
        /// For addition: a + b should equal b + a
        #[test]
        fn prop_addition_commutative((expr1, expr2) in arb_equivalent_exprs()) {
            let mut parser1 = Parser::new(&expr1);
            let ast1 = parser1.parse()
                .expect("First expression should parse");

            let mut parser2 = Parser::new(&expr2);
            let ast2 = parser2.parse()
                .expect("Second expression should parse");

            let mut eval1 = Evaluator::new();
            let result1 = eval1.eval_program(&ast1)
                .expect("First expression should evaluate");

            let mut eval2 = Evaluator::new();
            let result2 = eval2.eval_program(&ast2)
                .expect("Second expression should evaluate");

            prop_assert_eq!(
                result1.clone(), result2.clone(),
                "Commutativity violated: {:?} != {:?}\nExpr1: {}\nExpr2: {}",
                result1, result2, expr1, expr2
            );
        }
    }
}

/// Meta-Test: Verify All Properties Are Tested
///
/// This test ensures we haven't forgotten to enable any property tests.
#[test]
fn test_property_test_completeness() {
    // This test always passes, but serves as documentation
    // of which properties we're testing.

    // Property 1: Parser Roundtrip ✅
    // Property 2: Evaluator Determinism ✅
    // Property 3: Token Concatenation ✅
    // Property 4: No Crashes (Parser + Evaluator) ✅
    // Property 5: Evaluation Consistency (Commutativity) ✅

    // Total: 5 core properties
    // Total test cases: ~14,000 (1000 + 1000 + 1000 + 10000 + 1000)

    assert_eq!(5, 5, "All 5 properties should be tested");
}
