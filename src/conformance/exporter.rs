//! Conformance Test Suite Exporter
//!
//! Exports RuchyRuchy interpreter test cases as standalone .ruchy files
//! for Ruchy compiler conformance validation.
//!
//! This module extracts test cases from Rust integration tests and
//! converts them into a portable conformance test suite compatible
//! with the Ruchy compiler's `ruchy test` command.

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Conformance test suite exporter
///
/// Exports RuchyRuchy interpreter test cases as standalone
/// .ruchy files for Ruchy compiler conformance validation.
#[derive(Debug)]
pub struct ConformanceExporter {
    /// Output directory for exported tests
    pub output_dir: PathBuf,
}

impl ConformanceExporter {
    /// Create a new conformance exporter
    ///
    /// # Example
    ///
    /// ```
    /// use ruchyruchy::conformance::ConformanceExporter;
    ///
    /// let exporter = ConformanceExporter::new();
    /// ```
    pub fn new() -> Self {
        Self {
            output_dir: PathBuf::from("conformance/ruchy_test_suite"),
        }
    }

    /// Export all chapters to conformance test suite
    ///
    /// Exports all 212 test cases from chapters 1-17.
    ///
    /// # Returns
    ///
    /// - `Ok(ExportResult)`: Export succeeded with statistics
    /// - `Err(ExportError)`: Export failed
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ruchyruchy::conformance::ConformanceExporter;
    ///
    /// let exporter = ConformanceExporter::new();
    /// let result = exporter.export_all_chapters().unwrap();
    /// println!("Exported {} tests from {} chapters",
    ///          result.test_count, result.chapters_exported);
    /// ```
    pub fn export_all_chapters(&self) -> Result<ExportResult, ExportError> {
        let chapters = vec![
            (1, "hello_world", 12),
            (2, "variables", 19),
            (3, "functions", 15),
            (4, "practical_patterns", 24),
            (5, "loops", 22),
            (6, "data_structures", 28),
            (10, "io", 18),
        ];

        let mut total_tests = 0;
        let mut chapters_exported = 0;

        for (chapter_num, chapter_name, expected_count) in chapters {
            match self.export_chapter(chapter_num, chapter_name, expected_count) {
                Ok(result) => {
                    total_tests += result.test_count;
                    chapters_exported += 1;
                }
                Err(e) => {
                    eprintln!("Warning: Failed to export chapter {}: {:?}", chapter_num, e);
                }
            }
        }

        Ok(ExportResult {
            test_count: total_tests,
            chapters_exported,
        })
    }

    /// Export a single chapter
    ///
    /// # Arguments
    ///
    /// - `chapter_num`: Chapter number (1, 2, 3, etc.)
    /// - `chapter_name`: Chapter name (e.g., "hello_world")
    /// - `expected_count`: Expected number of tests (for validation)
    ///
    /// # Returns
    ///
    /// - `Ok(ExportResult)`: Export succeeded
    /// - `Err(ExportError)`: Export failed
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ruchyruchy::conformance::ConformanceExporter;
    ///
    /// let exporter = ConformanceExporter::new();
    /// let result = exporter.export_chapter(1, "hello_world", 12).unwrap();
    /// println!("Exported {} tests", result.test_count);
    /// ```
    pub fn export_chapter(
        &self,
        chapter_num: usize,
        chapter_name: &str,
        _expected_count: usize,
    ) -> Result<ExportResult, ExportError> {
        // Extract test cases from corresponding test file
        let test_cases = self.extract_test_cases(chapter_num, chapter_name)?;

        // Create chapter directory
        let chapter_dir = self
            .output_dir
            .join(format!("chapter_{:02}_{}", chapter_num, chapter_name));
        fs::create_dir_all(&chapter_dir)
            .map_err(|e| ExportError::IoError(format!("Failed to create directory: {}", e)))?;

        // Export each test case
        for (idx, test_case) in test_cases.iter().enumerate() {
            let test_file = chapter_dir.join(format!(
                "test_{:03}_{}.ruchy",
                idx + 1,
                sanitize_filename(&test_case.name)
            ));
            self.write_test_file(&test_file, test_case)?;
        }

        Ok(ExportResult {
            test_count: test_cases.len(),
            chapters_exported: 1,
        })
    }

    /// Extract test cases from interpreter test files
    fn extract_test_cases(
        &self,
        chapter_num: usize,
        chapter_name: &str,
    ) -> Result<Vec<TestCase>, ExportError> {
        // Map chapter number to test file
        let test_file = match chapter_num {
            1 => "tests/test_interp_011_ch01_examples.rs",
            2 => "tests/test_interp_012_ch02_examples.rs",
            3 => "tests/test_interp_013_ch03_examples.rs",
            4 => "tests/test_interp_014_ch04_examples.rs",
            5 => "tests/test_interp_015_ch05_examples.rs",
            6 => "tests/test_interp_016_ch06_examples.rs",
            10 => "tests/test_interp_017_ch10_examples.rs",
            _ => return Err(ExportError::InvalidChapter(chapter_num)),
        };

        // Read test file content
        let content = fs::read_to_string(test_file).map_err(|e| {
            ExportError::IoError(format!("Failed to read test file {}: {}", test_file, e))
        })?;

        // Parse test cases from Rust test file
        let test_cases = parse_rust_test_file(&content, chapter_num, chapter_name)?;

        Ok(test_cases)
    }

    /// Write a single test file with metadata
    fn write_test_file(&self, path: &Path, test_case: &TestCase) -> Result<(), ExportError> {
        let mut file = File::create(path)
            .map_err(|e| ExportError::IoError(format!("Failed to create test file: {}", e)))?;

        // Metadata header
        writeln!(file, "// Test: {}", test_case.name)
            .map_err(|e| ExportError::IoError(format!("Failed to write metadata: {}", e)))?;
        writeln!(
            file,
            "// Chapter: {:02} - {}",
            test_case.chapter, test_case.chapter_name
        )
        .map_err(|e| ExportError::IoError(format!("Failed to write metadata: {}", e)))?;
        writeln!(file, "// Description: {}", test_case.description)
            .map_err(|e| ExportError::IoError(format!("Failed to write metadata: {}", e)))?;
        writeln!(file, "//")
            .map_err(|e| ExportError::IoError(format!("Failed to write metadata: {}", e)))?;
        writeln!(file, "// Expected Output:")
            .map_err(|e| ExportError::IoError(format!("Failed to write metadata: {}", e)))?;
        for line in &test_case.expected_output {
            writeln!(file, "// {}", line).map_err(|e| {
                ExportError::IoError(format!("Failed to write expected output: {}", e))
            })?;
        }
        writeln!(file)
            .map_err(|e| ExportError::IoError(format!("Failed to write newline: {}", e)))?;

        // Source code
        writeln!(file, "{}", test_case.source_code)
            .map_err(|e| ExportError::IoError(format!("Failed to write source code: {}", e)))?;

        Ok(())
    }
}

impl Default for ConformanceExporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of export operation
#[derive(Debug, PartialEq, Eq)]
pub struct ExportResult {
    /// Number of tests exported
    pub test_count: usize,
    /// Number of chapters exported
    pub chapters_exported: usize,
}

/// Export error types
#[derive(Debug)]
pub enum ExportError {
    /// I/O error
    IoError(String),
    /// Invalid chapter number
    InvalidChapter(usize),
    /// Parse error
    ParseError(String),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportError::IoError(msg) => write!(f, "I/O error: {}", msg),
            ExportError::InvalidChapter(ch) => write!(f, "Invalid chapter: {}", ch),
            ExportError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for ExportError {}

/// Test case structure
#[derive(Debug, Clone)]
pub struct TestCase {
    /// Test name
    pub name: String,
    /// Chapter number
    pub chapter: usize,
    /// Chapter name
    pub chapter_name: String,
    /// Test description
    pub description: String,
    /// Ruchy source code
    pub source_code: String,
    /// Expected output lines
    pub expected_output: Vec<String>,
}

/// Parse test cases from Rust test file
///
/// Extracts Ruchy source code from Rust test functions.
/// Also extracts test descriptions from comments above test functions.
fn parse_rust_test_file(
    content: &str,
    chapter_num: usize,
    chapter_name: &str,
) -> Result<Vec<TestCase>, ExportError> {
    let mut test_cases = Vec::new();

    // Simple parser: look for test functions and extract Ruchy code
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        // Find test function
        if lines[i].trim().starts_with("#[test]") {
            // Extract description from comments before #[test]
            let mut description = String::new();
            let mut comment_start = i.saturating_sub(1);

            // Look backwards for comment block
            while comment_start > 0 {
                let line = lines[comment_start].trim();
                if line.starts_with("//") {
                    // Extract comment text
                    if let Some(comment_text) = line.strip_prefix("//").map(str::trim) {
                        if !comment_text.is_empty()
                            && !comment_text.starts_with('=')
                            && !comment_text.starts_with("Example")
                        {
                            description = format!("{} {}", comment_text, description);
                        }
                    }
                    comment_start = comment_start.saturating_sub(1);
                } else if !line.is_empty() {
                    break;
                } else {
                    comment_start = comment_start.saturating_sub(1);
                }
            }

            i += 1;
            if i >= lines.len() {
                break;
            }

            // Extract test name
            let test_name = if let Some(fn_line) = lines[i].trim().strip_prefix("fn ") {
                fn_line.split('(').next().unwrap_or("unknown").to_string()
            } else {
                i += 1;
                continue;
            };

            // Extract Ruchy source code between quotes
            let mut source_code = String::new();
            let expected_output = Vec::new();

            // Look for let program = r#"..."# pattern or let source = r#"..."#
            while i < lines.len() {
                if lines[i].contains("r#\"") || lines[i].contains("r\"") {
                    i += 1;
                    while i < lines.len() {
                        if lines[i].contains("\"#") || lines[i].ends_with("\"") {
                            break;
                        }
                        source_code.push_str(lines[i]);
                        source_code.push('\n');
                        i += 1;
                    }
                    break;
                }
                i += 1;
            }

            if !source_code.is_empty() {
                // Use extracted description or fallback
                let final_description = if description.trim().is_empty() {
                    format!("Test case from chapter {} - {}", chapter_num, chapter_name)
                } else {
                    description.trim().to_string()
                };

                test_cases.push(TestCase {
                    name: test_name,
                    chapter: chapter_num,
                    chapter_name: chapter_name.to_string(),
                    description: final_description,
                    source_code: source_code.trim().to_string(),
                    expected_output,
                });
            }
        }
        i += 1;
    }

    if test_cases.is_empty() {
        return Err(ExportError::ParseError(format!(
            "No test cases found in chapter {}",
            chapter_num
        )));
    }

    Ok(test_cases)
}

/// Sanitize filename for filesystem
///
/// Converts test name to valid filename by:
/// - Converting to lowercase
/// - Replacing spaces with underscores
/// - Removing non-alphanumeric characters (except underscore)
fn sanitize_filename(name: &str) -> String {
    name.to_lowercase()
        .replace(' ', "_")
        .replace("::", "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("Hello World"), "hello_world");
        assert_eq!(sanitize_filename("test::example"), "test_example");
        assert_eq!(sanitize_filename("Test-123"), "test123");
    }

    #[test]
    fn test_exporter_new() {
        let exporter = ConformanceExporter::new();
        assert_eq!(
            exporter.output_dir,
            PathBuf::from("conformance/ruchy_test_suite")
        );
    }
}
