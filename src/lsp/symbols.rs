// Symbol Tracking and Resolution
// Provides go-to-definition and find-references support

use super::protocol::{Location, Position, Range};
use std::collections::HashMap;

/// Symbol kind
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    Function,
    Variable,
    Type,
    Constant,
}

/// Symbol information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub location: Location,
}

/// Symbol table for tracking definitions and references
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
    references: HashMap<String, Vec<Location>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            references: HashMap::new(),
        }
    }

    /// Add a symbol definition
    pub fn add_symbol(&mut self, name: String, kind: SymbolKind, location: Location) {
        let symbol = Symbol {
            name: name.clone(),
            kind,
            location,
        };
        self.symbols.insert(name, symbol);
    }

    /// Add a symbol reference
    pub fn add_reference(&mut self, name: String, location: Location) {
        self.references.entry(name).or_default().push(location);
    }

    /// Get symbol definition
    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    /// Get all references to a symbol
    pub fn get_references(&self, name: &str) -> Vec<Location> {
        self.references.get(name).cloned().unwrap_or_default()
    }

    /// Find symbol at position (simple word-based lookup)
    pub fn find_symbol_at_position(&self, text: &str, position: Position) -> Option<String> {
        let lines: Vec<&str> = text.lines().collect();
        if position.line as usize >= lines.len() {
            return None;
        }

        let line = lines[position.line as usize];
        if position.character as usize >= line.len() {
            return None;
        }

        // Find word boundaries around cursor position
        let chars: Vec<char> = line.chars().collect();
        let pos = position.character as usize;

        // Find start of word
        let mut start = pos;
        while start > 0 && is_identifier_char(chars[start - 1]) {
            start -= 1;
        }

        // Find end of word
        let mut end = pos;
        while end < chars.len() && is_identifier_char(chars[end]) {
            end += 1;
        }

        if start < end {
            let word: String = chars[start..end].iter().collect();
            if !word.is_empty() {
                return Some(word);
            }
        }

        None
    }

    /// Parse document and build symbol table
    pub fn parse_document(&mut self, uri: &str, text: &str) {
        // Simple parser to find function definitions
        // Format: fun name() { ... }

        let lines: Vec<&str> = text.lines().collect();
        for (line_num, line) in lines.iter().enumerate() {
            // Look for function definitions
            if let Some(pos) = line.find("fun ") {
                if let Some(name) = extract_function_name(&line[pos + 4..]) {
                    let location = Location::new(
                        uri.to_string(),
                        Range::new(
                            Position::new(line_num as u32, pos as u32),
                            Position::new(line_num as u32, (pos + 3 + name.len()) as u32),
                        ),
                    );
                    self.add_symbol(name, SymbolKind::Function, location);
                }
            }

            // Look for let bindings
            if let Some(pos) = line.find("let ") {
                if let Some(name) = extract_variable_name(&line[pos + 4..]) {
                    let location = Location::new(
                        uri.to_string(),
                        Range::new(
                            Position::new(line_num as u32, pos as u32),
                            Position::new(line_num as u32, (pos + 4 + name.len()) as u32),
                        ),
                    );
                    self.add_symbol(name, SymbolKind::Variable, location);
                }
            }
        }
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if character is valid in identifier
fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Extract function name from text after "fun "
fn extract_function_name(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if let Some(paren_pos) = trimmed.find('(') {
        let name = trimmed[..paren_pos].trim();
        if !name.is_empty()
            && name.chars().all(is_identifier_char)
            && !name.chars().next().unwrap().is_numeric()
        {
            return Some(name.to_string());
        }
    }
    None
}

/// Extract variable name from text after "let "
fn extract_variable_name(text: &str) -> Option<String> {
    let trimmed = text.trim();
    // Find first word before = or space
    let end_pos = trimmed.find(['=', ' ', ':']).unwrap_or(trimmed.len());

    let name = trimmed[..end_pos].trim();
    if !name.is_empty() && name.chars().all(is_identifier_char) {
        return Some(name.to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_symbol() {
        let mut table = SymbolTable::new();
        let location = Location::new(
            "file:///test.ruchy".to_string(),
            Range::new(Position::new(0, 0), Position::new(0, 3)),
        );

        table.add_symbol("main".to_string(), SymbolKind::Function, location.clone());

        let symbol = table.get_symbol("main");
        assert!(symbol.is_some());
        assert_eq!(symbol.unwrap().name, "main");
        assert_eq!(symbol.unwrap().kind, SymbolKind::Function);
    }

    #[test]
    fn test_add_reference() {
        let mut table = SymbolTable::new();
        let location = Location::new(
            "file:///test.ruchy".to_string(),
            Range::new(Position::new(5, 10), Position::new(5, 14)),
        );

        table.add_reference("main".to_string(), location.clone());

        let refs = table.get_references("main");
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].uri, "file:///test.ruchy");
    }

    #[test]
    fn test_find_symbol_at_position() {
        let table = SymbolTable::new();
        let text = "fun main() {\n    let x = 5\n}";

        // Position on "main"
        let symbol = table.find_symbol_at_position(text, Position::new(0, 5));
        assert_eq!(symbol, Some("main".to_string()));

        // Position on "x"
        let symbol = table.find_symbol_at_position(text, Position::new(1, 8));
        assert_eq!(symbol, Some("x".to_string()));
    }

    #[test]
    fn test_parse_document() {
        let mut table = SymbolTable::new();
        let text = "fun main() {\n    let x = 5\n    let y = 10\n}";

        table.parse_document("file:///test.ruchy", text);

        // Should find "main" function
        let main_symbol = table.get_symbol("main");
        assert!(main_symbol.is_some());
        assert_eq!(main_symbol.unwrap().kind, SymbolKind::Function);

        // Should find "x" variable
        let x_symbol = table.get_symbol("x");
        assert!(x_symbol.is_some());
        assert_eq!(x_symbol.unwrap().kind, SymbolKind::Variable);

        // Should find "y" variable
        let y_symbol = table.get_symbol("y");
        assert!(y_symbol.is_some());
        assert_eq!(y_symbol.unwrap().kind, SymbolKind::Variable);
    }

    #[test]
    fn test_extract_function_name() {
        assert_eq!(extract_function_name("main()"), Some("main".to_string()));
        assert_eq!(
            extract_function_name("foo(x: i32)"),
            Some("foo".to_string())
        );
        assert_eq!(
            extract_function_name("  bar  (  )  "),
            Some("bar".to_string())
        );
        assert_eq!(extract_function_name(""), None);
        assert_eq!(extract_function_name("123()"), None); // Invalid identifier
    }

    #[test]
    fn test_extract_variable_name() {
        assert_eq!(extract_variable_name("x = 5"), Some("x".to_string()));
        assert_eq!(extract_variable_name("foo = bar"), Some("foo".to_string()));
        assert_eq!(
            extract_variable_name("  count  =  "),
            Some("count".to_string())
        );
        assert_eq!(extract_variable_name("x: i32"), Some("x".to_string()));
        assert_eq!(extract_variable_name(""), None);
    }

    #[test]
    fn test_is_identifier_char() {
        assert!(is_identifier_char('a'));
        assert!(is_identifier_char('Z'));
        assert!(is_identifier_char('0'));
        assert!(is_identifier_char('_'));
        assert!(!is_identifier_char(' '));
        assert!(!is_identifier_char('('));
        assert!(!is_identifier_char('='));
    }
}
