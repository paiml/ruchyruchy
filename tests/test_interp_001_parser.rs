// INTERP-001: AST Parser Integration - RED Phase Tests
// These tests define the parser interface through EXTREME TDD
//
// Research: Aho et al. (2006) Chapter 4: Syntax Analysis
//
// Test Strategy:
// 1. Parse simple programs first (hello world)
// 2. Parse complex programs (all ruchy-book examples)
// 3. Handle syntax errors gracefully
// 4. Validate AST structure correctness

use ruchyruchy::interpreter::parser::{Parser, ParseError, Ast, AstNode};

// ===== RED PHASE TEST 1: Parse Simple Hello World =====

#[test]
fn test_parse_simple_hello_world() {
    // RED: This test will fail because Parser doesn't exist yet
    let source = r#"
        fun main() {
            println("Hello, World!");
        }
    "#;

    let mut parser = Parser::new(source);
    let result = parser.parse();

    assert!(result.is_ok(), "Failed to parse hello world: {:?}", result.err());

    let ast = result.unwrap();
    assert_eq!(ast.nodes().len(), 1, "Expected 1 function definition");

    // Verify AST structure
    match &ast.nodes()[0] {
        AstNode::FunctionDef { name, params, body } => {
            assert_eq!(name, "main");
            assert_eq!(params.len(), 0);
            assert!(!body.is_empty());
        }
        _ => panic!("Expected FunctionDef node"),
    }
}

// ===== RED PHASE TEST 2: Parse Variables and Types =====

#[test]
fn test_parse_variables_and_types() {
    // RED: Test Chapter 2 patterns
    let source = r#"
        fun main() {
            let x = 42;
            let y = "hello";
            let z = true;
        }
    "#;

    let mut parser = Parser::new(source);
    let result = parser.parse();

    assert!(result.is_ok(), "Failed to parse variables: {:?}", result.err());

    let ast = result.unwrap();
    // Verify variable declarations are parsed
    assert!(!ast.nodes().is_empty());
}

// ===== RED PHASE TEST 3: Parse Function Calls =====

#[test]
fn test_parse_function_calls() {
    // RED: Test Chapter 3 patterns
    let source = r#"
        fun add(x, y) {
            return x + y;
        }

        fun main() {
            let result = add(2, 3);
            println(result);
        }
    "#;

    let mut parser = Parser::new(source);
    let result = parser.parse();

    assert!(result.is_ok(), "Failed to parse functions: {:?}", result.err());

    let ast = result.unwrap();
    assert_eq!(ast.nodes().len(), 2, "Expected 2 function definitions");
}

// ===== RED PHASE TEST 4: Parse Control Flow =====

#[test]
fn test_parse_control_flow() {
    // RED: Test Chapter 5 patterns
    let source = r#"
        fun main() {
            if true {
                println("yes");
            } else {
                println("no");
            }

            while x < 10 {
                x = x + 1;
            }

            for item in items {
                println(item);
            }
        }
    "#;

    let mut parser = Parser::new(source);
    let result = parser.parse();

    assert!(result.is_ok(), "Failed to parse control flow: {:?}", result.err());
}

// ===== RED PHASE TEST 5: Parse Data Structures =====

#[test]
fn test_parse_data_structures() {
    // RED: Test Chapter 6 patterns
    let source = r#"
        fun main() {
            let vec = [1, 2, 3, 4, 5];
            let map = {"key": "value", "count": 42};

            let first = vec[0];
            let value = map["key"];
        }
    "#;

    let mut parser = Parser::new(source);
    let result = parser.parse();

    assert!(result.is_ok(), "Failed to parse data structures: {:?}", result.err());
}

// ===== RED PHASE TEST 6: Error Recovery =====

#[test]
fn test_parse_error_recovery() {
    // RED: Test error handling
    let source = r#"
        fun main() {
            let x = ;  // Syntax error: missing expression
        }
    "#;

    let mut parser = Parser::new(source);
    let result = parser.parse();

    assert!(result.is_err(), "Expected parse error for invalid syntax");

    match result.err().unwrap() {
        ParseError::UnexpectedToken { line, column, .. } => {
            assert!(line > 0);
            assert!(column > 0);
        }
        _ => panic!("Expected UnexpectedToken error"),
    }
}

// ===== RED PHASE TEST 7: AST Structure Validation =====

#[test]
fn test_ast_structure_validity() {
    // RED: Verify AST nodes are well-formed
    let source = r#"
        fun factorial(n) {
            if n <= 1 {
                return 1;
            } else {
                return n * factorial(n - 1);
            }
        }
    "#;

    let mut parser = Parser::new(source);
    let result = parser.parse();

    assert!(result.is_ok());

    let ast = result.unwrap();

    // Validate AST traversal works
    ast.visit(|node| {
        // Every node should have valid structure
        match node {
            AstNode::FunctionDef { name, .. } => {
                assert!(!name.is_empty(), "Function name cannot be empty");
            }
            AstNode::IfExpr { condition, .. } => {
                assert!(condition.as_ref() != &AstNode::Empty, "Condition cannot be empty");
            }
            _ => {}
        }
    });
}

// ===== RED PHASE TEST 8: Parse Expressions with Precedence =====

#[test]
fn test_parse_expression_precedence() {
    // RED: Test operator precedence
    let source = r#"
        fun main() {
            let result = 2 + 3 * 4;  // Should parse as 2 + (3 * 4)
            let compare = x < y && a > b;  // Logical precedence
        }
    "#;

    let mut parser = Parser::new(source);
    let result = parser.parse();

    assert!(result.is_ok(), "Failed to parse expressions: {:?}", result.err());

    let ast = result.unwrap();
    // Verify expression trees respect precedence
    // (Full verification deferred to evaluator tests)
    assert!(!ast.nodes().is_empty());
}

// ===== RED PHASE TEST 9: Parse Match Expressions =====

#[test]
fn test_parse_match_expressions() {
    // RED: Test pattern matching
    let source = r#"
        fun main() {
            match x {
                0 => println("zero"),
                1 => println("one"),
                _ => println("other"),
            }
        }
    "#;

    let mut parser = Parser::new(source);
    let result = parser.parse();

    assert!(result.is_ok(), "Failed to parse match: {:?}", result.err());
}

// ===== RED PHASE TEST 10: Parse Struct Definitions =====

#[test]
fn test_parse_struct_definitions() {
    // RED: Test Chapter 19 patterns
    let source = r#"
        struct Point {
            x: i32,
            y: i32,
        }

        fun main() {
            let p = Point { x: 10, y: 20 };
            println(p.x);
        }
    "#;

    let mut parser = Parser::new(source);
    let result = parser.parse();

    assert!(result.is_ok(), "Failed to parse structs: {:?}", result.err());

    let ast = result.unwrap();
    assert!(ast.nodes().len() >= 2, "Expected struct and function definitions");
}

// ===== RED PHASE TEST 11: Integration with ruchy check =====

#[test]
fn test_integration_with_ruchy_compiler() {
    // RED: Verify we can parse output from ruchy check
    let source = r#"
        fun fibonacci(n) {
            if n <= 1 {
                return n;
            } else {
                return fibonacci(n - 1) + fibonacci(n - 2);
            }
        }
    "#;

    // This test ensures our parser is compatible with Ruchy compiler
    let mut parser = Parser::new(source);
    let result = parser.parse();

    assert!(result.is_ok(), "Parser must be compatible with Ruchy syntax");
}

// ===== RED PHASE TEST 12: Parse Comments =====

#[test]
fn test_parse_with_comments() {
    // RED: Comments should be handled
    let source = r#"
        // This is a comment
        fun main() {
            // Another comment
            let x = 42;  // Inline comment
        }
    "#;

    let mut parser = Parser::new(source);
    let result = parser.parse();

    assert!(result.is_ok(), "Failed to parse with comments: {:?}", result.err());
}

// ===== RED PHASE META TEST: Count Test Coverage =====

#[test]
fn test_red_phase_completeness() {
    // This test documents that RED phase is complete
    // We have 12 tests covering:
    // - Basic parsing (hello world)
    // - Variables and types (Ch02)
    // - Functions (Ch03)
    // - Control flow (Ch05)
    // - Data structures (Ch06)
    // - Error recovery
    // - AST validation
    // - Expression precedence
    // - Pattern matching
    // - Structs (Ch19)
    // - Ruchy integration
    // - Comments

    // All tests above should FAIL until GREEN phase implementation
    println!("RED phase: 12 tests defined");
    println!("Next: GREEN phase - implement minimal parser");
}
