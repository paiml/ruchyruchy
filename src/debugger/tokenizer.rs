// DEBUGGER-050: Tokenization Debugging Tools
//
// GREEN Phase: Priority 1 Implementation
// Toyota Way (Genchi Genbutsu): Every function addresses real pain from PARSER-079
//
// Pain Points Addressed:
// - Couldn't see raw token stream during PARSER-079 debugging (110k tokens spent)
// - Couldn't identify pattern conflicts (String vs Lifetime priority)
// - Manual comparison of working vs broken code took hours
// - Root cause (lexer issue) hidden from parser error messages

use crate::interpreter::parser::Parser;

/// Show detailed token stream with source locations
///
/// Addresses PARSER-079 pain: "Couldn't see raw token stream"
pub fn tokenize(source: &str) -> String {
    let mut parser = Parser::new(source);
    let mut output = String::new();

    output.push_str("Token Stream:\n");
    output.push_str("=============\n\n");

    match parser.debug_get_tokens() {
        Ok(tokens) => {
            for (i, token) in tokens.iter().enumerate() {
                output.push_str(&format!("Token #{}: {}\n", i + 1, token));
            }
            output.push_str(&format!("\nTotal tokens: {}\n", tokens.len()));
        }
        Err(_) => {
            output.push_str("Error: Failed to tokenize source\n");
        }
    }

    output
}

/// Show token stream with error highlighting (Bang = error recovery)
///
/// Addresses PARSER-079 pain: "Couldn't see when error recovery triggered"
pub fn tokenize_with_errors(source: &str) -> String {
    let mut parser = Parser::new(source);
    let mut output = String::new();

    output.push_str("Token Stream (with error detection):\n");
    output.push_str("====================================\n\n");

    match parser.debug_get_tokens() {
        Ok(tokens) => {
            let mut error_count = 0;

            for (i, token) in tokens.iter().enumerate() {
                // Check if this is an error token (Note: interpreter parser has no Bang tokens)
                if parser.debug_is_error_token(i) {
                    error_count += 1;
                    output.push_str(&format!(
                        "Token #{}: {} ⚠️  ERROR RECOVERY TRIGGERED\n",
                        i + 1,
                        token
                    ));
                    output.push_str("^^ DIAGNOSTIC: Parser encountered unexpected token\n\n");
                } else {
                    output.push_str(&format!("Token #{}: {}\n", i + 1, token));
                }
            }

            if error_count > 0 {
                output.push_str(&format!(
                    "\n⚠️  {} error recovery points detected\n",
                    error_count
                ));
            }

            output.push_str(&format!("Total tokens: {}\n", tokens.len()));
        }
        Err(_) => {
            output.push_str("Error: Failed to tokenize source\n");
        }
    }

    output
}

/// Analysis result with warnings about pattern conflicts
pub struct TokenAnalysis {
    /// List of warnings about potential token pattern conflicts
    pub warnings: Vec<String>,
}

/// Detect pattern conflicts (String vs Lifetime priority)
///
/// Addresses PARSER-079 pain: "Root cause was String pattern having higher priority than Lifetime"
pub fn tokenize_analyze(source: &str) -> TokenAnalysis {
    let mut parser = Parser::new(source);
    let mut warnings = Vec::new();

    match parser.debug_get_tokens() {
        Ok(tokens) => {
            for token in tokens.iter() {
                // Check for potential pattern conflicts
                // Look for StringLit tokens that might be intended as lifetime tokens
                if token.contains("StringLit") && token.contains("'") {
                    // Extract the string content if possible
                    if let Some(start) = token.find("StringLit(\"") {
                        if let Some(content_start) = token[start..].find('"') {
                            if let Some(content_end) = token[start + content_start + 1..].find('"')
                            {
                                let content = &token[start + content_start + 1
                                    ..start + content_start + 1 + content_end];
                                if content.starts_with('\'') && content.len() > 1 {
                                    warnings.push(format!(
                                        "Potential pattern conflict: String \"{}\" might be intended as Lifetime. \
                                         String pattern may have higher priority than Lifetime pattern.",
                                        content
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(_) => {
            warnings.push("Error: Failed to tokenize source for analysis".to_string());
        }
    }

    TokenAnalysis { warnings }
}

/// Side-by-side token comparison with diagnostic hints
///
/// Addresses PARSER-079 pain: "Manual comparison took hours"
pub fn compare_tokens(working: &str, broken: &str) -> String {
    let mut working_parser = Parser::new(working);
    let mut broken_parser = Parser::new(broken);
    let mut output = String::new();

    output.push_str("Token Comparison:\n");
    output.push_str("=================\n\n");

    let working_tokens = working_parser.debug_get_tokens().unwrap_or_default();
    let broken_tokens = broken_parser.debug_get_tokens().unwrap_or_default();

    let max_len = working_tokens.len().max(broken_tokens.len());
    let mut mismatches = 0;

    for i in 0..max_len {
        let working_token = working_tokens.get(i).map(|s| s.as_str()).unwrap_or("EOF");
        let broken_token = broken_tokens.get(i).map(|s| s.as_str()).unwrap_or("EOF");

        if working_token != broken_token {
            mismatches += 1;
            output.push_str(&format!("Position {}:\n", i + 1));
            output.push_str(&format!("  Working:    {}\n", working_token));
            output.push_str(&format!("  Broken:     {} ⚠️  MISMATCH\n", broken_token));
            output.push_str("^^ HINT: Token type differs between working and broken code\n\n");
        }
    }

    if mismatches == 0 {
        output.push_str("✅ No token mismatches found\n");
    } else {
        output.push_str(&format!("\n⚠️  {} mismatch(es) detected\n", mismatches));
    }

    output
}

/// Compare tokens with root cause suggestions
///
/// Addresses PARSER-079 pain: "Took hours to identify String vs Lifetime pattern priority"
pub fn compare_tokens_with_hints(working: &str, broken: &str) -> String {
    let mut working_parser = Parser::new(working);
    let mut broken_parser = Parser::new(broken);
    let mut output = String::new();

    output.push_str("Token Comparison (with root cause hints):\n");
    output.push_str("=========================================\n\n");

    let working_tokens = working_parser.debug_get_tokens().unwrap_or_default();
    let broken_tokens = broken_parser.debug_get_tokens().unwrap_or_default();

    let max_len = working_tokens.len().max(broken_tokens.len());

    for i in 0..max_len {
        let working_token = working_tokens.get(i).map(|s| s.as_str()).unwrap_or("EOF");
        let broken_token = broken_tokens.get(i).map(|s| s.as_str()).unwrap_or("EOF");

        if working_token != broken_token {
            output.push_str(&format!("Position {}:\n", i + 1));
            output.push_str(&format!("  Working:    {}\n", working_token));
            output.push_str(&format!("  Broken:     {} ⚠️  MISMATCH\n", broken_token));

            // Provide root cause hints based on token types
            // Check for Lifetime vs String pattern conflicts
            if (working_token.contains("Lifetime") && broken_token.contains("StringLit"))
                || (working_token.contains("StringLit") && broken_token.contains("Lifetime"))
            {
                output.push_str(
                    "^^ HINT: String pattern has higher priority than Lifetime pattern\n",
                );
                output.push_str(
                    "   Root Cause: Lexer pattern ordering issue - String matches before Lifetime\n",
                );
            }
            // Check for Char vs String mismatches
            else if (working_token.contains("Char") && broken_token.contains("StringLit"))
                || (working_token.contains("StringLit") && broken_token.contains("Char"))
            {
                output.push_str("^^ HINT: Character literal vs String literal mismatch\n");
            } else {
                output.push_str("^^ HINT: Token type mismatch - check lexer rules\n");
            }
            output.push('\n');
        }
    }

    output
}

/// Show parser state at failure point
///
/// Addresses PARSER-079 pain: "Couldn't see what parser expected vs what it got"
pub fn parser_trace(source: &str) -> String {
    let mut parser = Parser::new(source);
    let mut output = String::new();

    output.push_str("Parser Trace:\n");
    output.push_str("=============\n\n");

    match parser.parse() {
        Ok(_) => {
            output.push_str("✅ Parse successful - no errors detected\n");
        }
        Err(e) => {
            output.push_str("❌ Parse failed:\n");
            output.push_str(&format!("Error: {:?}\n\n", e));

            output.push_str("Parser state at error:\n");
            output.push_str("Expected: [See error message above]\n");
            output.push_str(&format!("Got: {}\n", e));
        }
    }

    output
}

/// Parser trace with root cause analysis
///
/// Addresses PARSER-079 pain: "Root cause (lexer issue) was hidden from parser error"
pub fn parser_trace_with_analysis(source: &str) -> String {
    let mut parser = Parser::new(source);
    let mut output = String::new();

    output.push_str("Parser Trace (with root cause analysis):\n");
    output.push_str("========================================\n\n");

    match parser.parse() {
        Ok(_) => {
            output.push_str("✅ Parse successful - no errors detected\n");
        }
        Err(e) => {
            output.push_str("❌ Parse failed:\n");
            output.push_str(&format!("Error: {:?}\n\n", e));

            // Try to get tokens to analyze
            if let Ok(tokens) = parser.debug_get_tokens() {
                if !tokens.is_empty() {
                    output.push_str(&format!("Current tokens: {} total\n", tokens.len()));
                }
            }

            // Root cause analysis
            output.push_str("\nRoot Cause Analysis:\n");

            // Check error message for clues
            let error_msg = format!("{:?}", e);
            if error_msg.contains("StringLit") || error_msg.contains("String") {
                output.push_str("Root Cause: Lexer may have tokenized input incorrectly\n");
                output.push_str(
                    "Hint: Check if String pattern is matching when Lifetime or other pattern should match\n",
                );
            } else if error_msg.contains("Unexpected") {
                output.push_str("Root Cause: Parser encountered unexpected token\n");
                output.push_str("Hint: Check syntax and token sequence\n");
            } else {
                output.push_str("Root Cause: Parser expected different token type\n");
                output.push_str("Hint: See error message for details\n");
            }
        }
    }

    output
}

/// Show only the failing portion of parse trace
///
/// Addresses PARSER-079 pain: "Full parser trace too verbose, needed focused view"
pub fn parser_trace_errors_only(source: &str) -> String {
    let mut parser = Parser::new(source);
    let mut output = String::new();

    output.push_str("Parser Trace (errors only):\n");
    output.push_str("===========================\n\n");

    match parser.parse() {
        Ok(_) => {
            output.push_str("✅ No errors - parse successful\n");
        }
        Err(e) => {
            output.push_str("❌ ERROR detected:\n\n");
            output.push_str(&format!("Error message: {:?}\n", e));

            // Show some context from the source around potential error location
            output.push_str("\nSource context:\n");
            let lines: Vec<&str> = source.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                output.push_str(&format!("Line {}: {}\n", i + 1, line));
            }

            // Only show context around the error, not all successful parses
            output.push_str("\n(Showing only error context, not successful parse steps)\n");
        }
    }

    output
}
