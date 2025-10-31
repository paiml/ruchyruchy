// Diagnostics Provider
// Integrates with ruchy check to provide diagnostics

use super::protocol::{Diagnostic, DiagnosticSeverity, Position, Range};
use std::process::Command;

/// Diagnostic provider using ruchy check
pub struct DiagnosticsProvider;

impl DiagnosticsProvider {
    /// Create a new diagnostics provider
    pub fn new() -> Self {
        Self
    }

    /// Run ruchy check and parse diagnostics
    pub fn check_file(&self, file_path: &str, _content: &str) -> Vec<Diagnostic> {
        // Run ruchy check on the file
        let output = Command::new("ruchy").arg("check").arg(file_path).output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    // No errors
                    vec![]
                } else {
                    // Parse stderr for errors
                    let stderr = String::from_utf8_lossy(&result.stderr);
                    self.parse_diagnostics(&stderr)
                }
            }
            Err(_) => {
                // ruchy command failed
                vec![Diagnostic {
                    range: Range::new(Position::new(0, 0), Position::new(0, 1)),
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: Some("ruchy-lsp".to_string()),
                    message: "Failed to run ruchy check".to_string(),
                }]
            }
        }
    }

    /// Parse ruchy check output into diagnostics
    fn parse_diagnostics(&self, output: &str) -> Vec<Diagnostic> {
        let mut diagnostics = Vec::new();

        // Parse error format: "Error at line X, column Y: message"
        // This is a simplified parser - real implementation would be more robust
        for line in output.lines() {
            if line.contains("Error") || line.contains("error") {
                // Extract line/column if possible
                let diag = self.parse_error_line(line);
                diagnostics.push(diag);
            }
        }

        // If no specific errors parsed, create a general error
        if diagnostics.is_empty() && !output.is_empty() {
            diagnostics.push(Diagnostic {
                range: Range::new(Position::new(0, 0), Position::new(0, 1)),
                severity: Some(DiagnosticSeverity::Error),
                code: None,
                source: Some("ruchy".to_string()),
                message: output.lines().next().unwrap_or("Unknown error").to_string(),
            });
        }

        diagnostics
    }

    /// Parse a single error line
    fn parse_error_line(&self, line: &str) -> Diagnostic {
        // Try to extract line and column numbers
        // Format examples:
        // - "Error at line 5, column 10: message"
        // - "error: message"

        // Default to line 0, col 0
        let mut line_num = 0;
        let mut col_num = 0;
        let mut message = line.to_string();

        // Try to parse "line X, column Y" pattern
        if let Some(line_pos) = line.find("line ") {
            if let Some(rest) = line.get(line_pos + 5..) {
                if let Some(comma_pos) = rest.find(',') {
                    if let Ok(num) = rest[..comma_pos].trim().parse::<u32>() {
                        line_num = num.saturating_sub(1); // Convert to 0-based

                        // Try to parse column
                        if let Some(col_pos) = rest.find("column ") {
                            if let Some(col_rest) = rest.get(col_pos + 7..) {
                                if let Some(colon_pos) = col_rest.find(':') {
                                    if let Ok(col) = col_rest[..colon_pos].trim().parse::<u32>() {
                                        col_num = col.saturating_sub(1); // Convert to 0-based
                                    }

                                    // Extract message after colon
                                    if let Some(msg) = col_rest.get(colon_pos + 1..) {
                                        message = msg.trim().to_string();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Diagnostic {
            range: Range::new(
                Position::new(line_num, col_num),
                Position::new(line_num, col_num + 1),
            ),
            severity: Some(DiagnosticSeverity::Error),
            code: None,
            source: Some("ruchy".to_string()),
            message,
        }
    }
}

impl Default for DiagnosticsProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_line_with_position() {
        let provider = DiagnosticsProvider::new();
        let line = "Error at line 5, column 10: unexpected token";
        let diag = provider.parse_error_line(line);

        assert_eq!(diag.range.start.line, 4); // 0-based
        assert_eq!(diag.range.start.character, 9); // 0-based
        assert!(diag.message.contains("unexpected token"));
    }

    #[test]
    fn test_parse_error_line_without_position() {
        let provider = DiagnosticsProvider::new();
        let line = "error: file not found";
        let diag = provider.parse_error_line(line);

        assert_eq!(diag.range.start.line, 0);
        assert_eq!(diag.range.start.character, 0);
        assert_eq!(diag.message, "error: file not found");
    }

    #[test]
    fn test_parse_diagnostics_empty() {
        let provider = DiagnosticsProvider::new();
        let diagnostics = provider.parse_diagnostics("");
        assert_eq!(diagnostics.len(), 0);
    }

    #[test]
    fn test_parse_diagnostics_with_error() {
        let provider = DiagnosticsProvider::new();
        let output = "Error at line 3, column 5: syntax error\n";
        let diagnostics = provider.parse_diagnostics(output);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].range.start.line, 2);
        assert!(diagnostics[0].message.contains("syntax error"));
    }
}
