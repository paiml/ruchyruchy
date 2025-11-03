// Code Generator: AST → Rust source code
//
// Takes RuchyRuchy interpreter AST and generates equivalent Rust code

/// Code generator for transpiling Ruchy AST to Rust
pub struct CodeGenerator {
    /// Generated Rust code buffer
    output: String,
    /// Current indentation level
    indent: usize,
}

impl CodeGenerator {
    /// Create a new code generator
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent: 0,
        }
    }

    /// Get the generated code
    pub fn output(&self) -> &str {
        &self.output
    }

    /// Clear the output buffer
    pub fn clear(&mut self) {
        self.output.clear();
        self.indent = 0;
    }

    /// Emit a line of code with current indentation
    ///
    /// Note: Will be used in COMPILE-002 for expression codegen
    #[allow(dead_code)]
    fn emit_line(&mut self, line: &str) {
        for _ in 0..self.indent {
            self.output.push_str("    ");
        }
        self.output.push_str(line);
        self.output.push('\n');
    }

    /// Emit raw code without indentation
    ///
    /// Note: Will be used in COMPILE-002 for inline code emission
    #[allow(dead_code)]
    fn emit(&mut self, code: &str) {
        self.output.push_str(code);
    }

    /// Increase indentation level
    ///
    /// Note: Will be used in COMPILE-003 for block structures
    #[allow(dead_code)]
    fn indent(&mut self) {
        self.indent += 1;
    }

    /// Decrease indentation level
    ///
    /// Note: Will be used in COMPILE-003 for block structures
    #[allow(dead_code)]
    fn dedent(&mut self) {
        if self.indent > 0 {
            self.indent -= 1;
        }
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_codegen_empty_output() {
        let codegen = CodeGenerator::new();
        assert_eq!(codegen.output(), "");
    }

    #[test]
    fn test_emit_line() {
        let mut codegen = CodeGenerator::new();
        codegen.emit_line("fn main() {");
        assert_eq!(codegen.output(), "fn main() {\n");
    }

    #[test]
    fn test_emit_line_with_indent() {
        let mut codegen = CodeGenerator::new();
        codegen.emit_line("fn main() {");
        codegen.indent();
        codegen.emit_line("println!(\"Hello\");");
        codegen.dedent();
        codegen.emit_line("}");

        let expected = "fn main() {\n    println!(\"Hello\");\n}\n";
        assert_eq!(codegen.output(), expected);
    }

    #[test]
    fn test_clear() {
        let mut codegen = CodeGenerator::new();
        codegen.emit_line("fn main() {}");
        codegen.clear();
        assert_eq!(codegen.output(), "");
        assert_eq!(codegen.indent, 0);
    }
}
