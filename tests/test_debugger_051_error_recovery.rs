// DEBUGGER-051: Parser Error Recovery Testing
//
// EXTREME TDD - RED Phase
//
// Mission: Test RuchyRuchy parser's resilience to malformed input and error recovery strategies.
//
// Error Recovery Strategies (Compiler Design Principles):
// 1. Panic Mode: Skip tokens until synchronization point (statement boundary)
// 2. Phrase-Level: Insert/delete/replace tokens to recover
// 3. Error Productions: Grammar rules for common errors
// 4. Global Correction: Minimal changes to make input valid
//
// Quality Gates:
// 1. Parser MUST recover from missing parens (continue parsing)
// 2. Parser MUST recover from missing semicolons (implicit insertion)
// 3. Parser MUST suggest typo corrections ("Did you mean 'return'?")
// 4. Parser MUST report multiple errors (not just first)
// 5. Parser errors MUST have line/column/message/help_text
//
// Toyota Way Principles:
// - Jidoka: Stop on first error VS collect all errors (policy decision)
// - Genchi Genbutsu: Go and see actual parser error messages
// - Kaizen: Improve error quality based on user feedback
// - Heijunka: Consistent error format across all parse errors
//
// Acceptance Criteria:
// - All 5 tests passing
// - Parser recovers from common syntax errors
// - Error messages are helpful and actionable
// - Multiple errors reported in single pass

/// Test 1: Parser recovers from missing closing parenthesis
///
/// Validates that parser can continue parsing after detecting missing ')' error.
/// Uses panic-mode recovery: skip to next statement boundary (semicolon or brace).
#[test]
fn test_parser_recovers_from_missing_paren() {
    let source = r#"
fun main() {
    let x = (10 + 5;  // Missing closing paren
    let y = 20;       // Should still parse this line
    return x + y;
}
"#;

    // Parse with error recovery
    let result = ruchyruchy::debugger::error_recovery::parse_with_recovery(source);

    assert!(result.is_ok(), "Parser must return Ok with recovered AST");

    let recovery_result = result.unwrap();

    // Must detect the missing paren error
    assert!(
        !recovery_result.errors.is_empty(),
        "Must detect missing closing paren error"
    );

    // Must continue parsing and find subsequent statements
    assert!(
        recovery_result.ast_nodes > 1,
        "Must parse statements after error (found {} AST nodes)",
        recovery_result.ast_nodes
    );

    // Error must reference the problematic line
    let error = &recovery_result.errors[0];
    assert!(
        error.line == 3,
        "Error must reference line 3 (found line {})",
        error.line
    );

    assert!(
        error.message.contains("missing") || error.message.contains("expected"),
        "Error message must mention missing/expected token: {}",
        error.message
    );

    println!(
        "Parser recovered: {} errors, {} AST nodes parsed",
        recovery_result.errors.len(),
        recovery_result.ast_nodes
    );
}

/// Test 2: Parser recovers from missing semicolon (implicit insertion)
///
/// Validates automatic semicolon insertion (ASI) like JavaScript/Go.
/// Parser should insert semicolon at statement boundaries.
#[test]
fn test_parser_recovers_from_missing_semicolon() {
    let source = r#"
fun main() {
    let x = 42  // Missing semicolon
    let y = 10  // Missing semicolon
    return x + y
}
"#;

    let result = ruchyruchy::debugger::error_recovery::parse_with_recovery(source);

    assert!(
        result.is_ok(),
        "Parser must recover from missing semicolons"
    );

    let recovery_result = result.unwrap();

    // Either:
    // 1. No errors (implicit semicolon insertion successful), OR
    // 2. Warnings about missing semicolons but still parsed
    if !recovery_result.errors.is_empty() {
        // If reported as errors, they must be non-fatal warnings
        for error in &recovery_result.errors {
            assert!(
                error.severity == "warning" || error.recoverable,
                "Missing semicolon errors must be warnings/recoverable"
            );
        }
    }

    // Must parse all statements successfully
    assert!(
        recovery_result.ast_nodes >= 3,
        "Must parse all 3 statements (found {} nodes)",
        recovery_result.ast_nodes
    );

    println!(
        "Semicolon recovery: {} warnings, {} AST nodes",
        recovery_result.errors.len(),
        recovery_result.ast_nodes
    );
}

/// Test 3: Parser suggests typo corrections
///
/// Validates "Did you mean?" suggestions using edit distance (Levenshtein).
/// Common typos: "retrun" -> "return", "prit" -> "print", "fucn" -> "fun"
#[test]
fn test_parser_suggests_fix() {
    let source = r#"
fun main() {
    retrun 42;  // Typo: should be "return"
}
"#;

    let result = ruchyruchy::debugger::error_recovery::parse_with_recovery(source);

    assert!(result.is_ok(), "Parser must parse with typo suggestion");

    let recovery_result = result.unwrap();

    // Must detect the typo
    assert!(
        !recovery_result.errors.is_empty(),
        "Must detect unknown identifier 'retrun'"
    );

    let error = &recovery_result.errors[0];

    // Must suggest correction
    assert!(
        error.suggestion.is_some(),
        "Error must include typo correction suggestion"
    );

    let suggestion = error.suggestion.as_ref().unwrap();

    assert!(
        suggestion.contains("return"),
        "Must suggest 'return' for 'retrun' (got: {})",
        suggestion
    );

    assert!(
        suggestion.contains("Did you mean"),
        "Suggestion must use 'Did you mean' phrasing: {}",
        suggestion
    );

    println!("Typo detection: 'retrun' -> suggestion: '{}'", suggestion);
}

/// Test 4: Parser reports multiple errors (not just first)
///
/// Validates that parser collects ALL errors in single pass (not fail-fast).
/// This is critical for IDE integration and batch error reporting.
#[test]
fn test_parser_multiple_errors() {
    let source = r#"
fun main() {
    let x = (10 + 5;      // Error 1: Missing closing paren
    let y = 20 * ;        // Error 2: Missing right operand
    retrun x + y;         // Error 3: Typo "retrun"
    let z = 30            // Error 4: Missing semicolon (if enforced)
}
"#;

    let result = ruchyruchy::debugger::error_recovery::parse_with_recovery(source);

    assert!(
        result.is_ok(),
        "Parser must collect multiple errors without panicking"
    );

    let recovery_result = result.unwrap();

    // Must report at least 3 errors (paren, operand, typo)
    assert!(
        recovery_result.errors.len() >= 3,
        "Must report at least 3 errors (found {})",
        recovery_result.errors.len()
    );

    // Errors must be on different lines
    let error_lines: Vec<usize> = recovery_result.errors.iter().map(|e| e.line).collect();

    let unique_lines: std::collections::HashSet<_> = error_lines.iter().collect();

    assert!(
        unique_lines.len() >= 3,
        "Errors must span multiple lines (found {} unique lines)",
        unique_lines.len()
    );

    println!(
        "Multiple errors collected: {} errors across {} lines",
        recovery_result.errors.len(),
        unique_lines.len()
    );
}

/// Test 5: Error quality validation (line, column, message, help text)
///
/// Validates that all error reports meet quality standards:
/// - Accurate line/column information
/// - Clear, actionable error message
/// - Help text with suggestions/examples
/// - Error codes for documentation lookup
#[test]
fn test_parser_error_quality() {
    let source = r#"
fun main() {
    let x = (10 + 5;  // Error: missing closing paren
}
"#;

    let result = ruchyruchy::debugger::error_recovery::parse_with_recovery(source);

    assert!(result.is_ok(), "Parser must return error details");

    let recovery_result = result.unwrap();

    assert!(
        !recovery_result.errors.is_empty(),
        "Must report parsing error"
    );

    let error = &recovery_result.errors[0];

    // Quality Check 1: Line/Column information
    assert!(error.line > 0, "Error must have line number");
    assert!(error.column > 0, "Error must have column number");

    // Quality Check 2: Clear error message
    assert!(
        !error.message.is_empty(),
        "Error must have descriptive message"
    );
    assert!(
        error.message.len() > 10,
        "Error message must be descriptive (found {} chars)",
        error.message.len()
    );

    // Quality Check 3: Help text with guidance
    assert!(
        error.help_text.is_some(),
        "Error must include help text for users"
    );

    let help = error.help_text.as_ref().unwrap();
    assert!(
        help.len() > 10,
        "Help text must be descriptive (found {} chars)",
        help.len()
    );

    // Quality Check 4: Error code for documentation
    assert!(
        error.code.is_some(),
        "Error must have code for documentation lookup"
    );

    println!(
        "Error quality: Line {}, Col {}, Code {:?}, Message: '{}', Help: '{}'",
        error.line, error.column, error.code, error.message, help
    );
}

// Data structures for test assertions
// These will be implemented in src/debugger/error_recovery.rs

#[allow(dead_code)]
struct RecoveryResult {
    ast_nodes: usize,
    errors: Vec<ParseError>,
}

#[allow(dead_code)]
struct ParseError {
    line: usize,
    column: usize,
    message: String,
    severity: String,
    recoverable: bool,
    suggestion: Option<String>,
    help_text: Option<String>,
    code: Option<String>,
}
