// INTERP-001: AST Parser Integration
// INTERP-032: Concurrency syntax support (use, ::, closures, blocks)
// INTERP-036: Grouped import syntax (use std::sync::{Arc, Mutex})
// INTERP-037: Dereference operator (*expr)
// INTERP-038: Compound assignment operators (+=, -=, *=, /=, %=)
// INTERP-039: vec! macro support
// INTERP-040: Tuple destructuring (let (a, b) = tuple)
// REFACTOR Phase: Clean up implementation while keeping tests green
//
// Research: Aho et al. (2006) Chapter 4: Syntax Analysis
//
// This is a recursive descent parser with operator precedence
// for Ruchy language syntax. Supports functions, structs, control
// flow, expressions, data structures, and concurrency primitives.
//
// Concurrency features (INTERP-032):
// - use statements: use std::sync::Mutex;
// - Path expressions: thread::spawn, Arc::new
// - Closures: || { }, |x| { }, move || { }
// - Block expressions: { let x = 1; }
//
// Grouped imports (INTERP-036):
// - Grouped use declarations: use std::sync::{Arc, Mutex};
// - Multiple items in braces: {Arc, Mutex, RwLock}
// - Nested paths: use std::sync::{Arc, Mutex};
//
// Dereference operator (INTERP-037):
// - Unary dereference: *expr
// - Extract values from mock wrappers: *counter.lock().unwrap()
// - Works in expressions: let y = *x + 1;
//
// Compound assignment operators (INTERP-038):
// - Compound assignments: +=, -=, *=, /=, %=
// - Simple form: x += 5
// - With dereference: *num += 1
// - Desugared to: lhs = lhs op rhs
//
// vec! macro (INTERP-039):
// - vec![] (empty vector)
// - vec![1, 2, 3] (vector with elements)
// - vec![0; 10] (repeated element)
// - Enables idiomatic Rust vector literals
//
// Tuple destructuring (INTERP-040):
// - let (a, b) = (1, 2) (2-tuple destructuring)
// - let (a, b, c) = (1, 2, 3) (3-tuple destructuring)
// - let (tx, rx) = mpsc::channel() (function return destructuring)
// - Note: Nested patterns like ((a, b), c) not yet supported

pub use super::ast::*;

/// Parser for Ruchy source code
pub struct Parser {
    source: String,
    tokens: Vec<Token>,
    pos: usize,
}

/// Token types
#[derive(Debug, Clone, PartialEq)]
enum Token {
    // Keywords
    Fun,
    Let,
    If,
    Else,
    While,
    For,
    In,
    Match,
    Return,
    Struct,
    As,
    Mut,
    Use,
    Move,

    // Identifiers and literals
    Identifier(String),
    Integer(i64),
    Float(f64),
    StringLit(String),
    CharLit(char),   // Character literal: 'a'
    FString(String), // F-string with interpolation: f"text {expr}"
    True,
    False,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Not,
    EqualEqual,
    NotEqual,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    AndAnd,
    OrOr,
    Pipe,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Colon,
    ColonColon,
    Arrow,
    FatArrow,
    Dot,
    DotDot,
    Equal,
    PlusEqual,    // +=
    MinusEqual,   // -=
    StarEqual,    // *=
    SlashEqual,   // /=
    PercentEqual, // %=
    Underscore,

    // End of file
    Eof,
}

impl Parser {
    /// Create a new parser for the given source code
    pub fn new(source: &str) -> Self {
        Parser {
            source: source.to_string(),
            tokens: Vec::new(),
            pos: 0,
        }
    }

    /// Parse the source code into an AST
    ///
    /// Tokenizes the source and parses top-level declarations (functions and structs)
    pub fn parse(&mut self) -> Result<Ast, ParseError> {
        // Step 1: Tokenize
        self.tokenize()?;

        // Step 2: Parse top-level declarations
        // INTERP-048: Pre-allocate nodes Vec (typical programs have 1-10 top-level items)
        let mut nodes = Vec::with_capacity(8);

        while !self.is_at_end() {
            // Skip any leading noise
            if self.check(&Token::Eof) {
                break;
            }

            // Parse top-level item
            let node = self.parse_top_level()?;
            nodes.push(node);
        }

        Ok(Ast { nodes })
    }

    /// DEBUGGER-047: Parse with performance profiling
    ///
    /// Parses source code while tracking timing and operations for performance analysis
    pub fn parse_with_profiler(
        &mut self,
        profiler: &crate::debugger::PerformanceProfiler,
    ) -> Result<Ast, ParseError> {
        use std::time::Instant;

        // Track overall parse time
        profiler.start_parse();

        // Track tokenization
        let tok_start = Instant::now();
        self.tokenize()?;
        let tok_duration = tok_start.elapsed().as_nanos();
        profiler.record_parse_operation("tokenize".to_string(), tok_duration);

        // Parse top-level declarations
        let mut nodes = Vec::new();

        while !self.is_at_end() {
            if self.check(&Token::Eof) {
                break;
            }

            let parse_start = Instant::now();
            let node = self.parse_top_level()?;
            let parse_duration = parse_start.elapsed().as_nanos();
            profiler.record_parse_operation("parse_top_level".to_string(), parse_duration);

            nodes.push(node);
        }

        profiler.end_parse();

        Ok(Ast { nodes })
    }

    /// Tokenize the source code into a vector of tokens
    ///
    /// Handles whitespace, comments, string literals, numbers, identifiers,
    /// keywords, and operators/delimiters
    fn tokenize(&mut self) -> Result<(), ParseError> {
        let mut chars = self.source.chars().peekable();
        // INTERP-048: Pre-allocate tokens Vec (estimate: 1 token per 4 chars, min 16)
        let estimated_tokens = (self.source.len() / 4).max(16);
        let mut tokens = Vec::with_capacity(estimated_tokens);

        while let Some(&ch) = chars.peek() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => { chars.next(); }
                '/' if chars.clone().nth(1) == Some('/') => Self::scan_comment(&mut chars),
                '"' => Self::scan_string(&mut chars, &mut tokens),
                '\'' => Self::scan_char(&mut chars, &mut tokens),
                '0'..='9' => Self::scan_number(&mut chars, &mut tokens),
                'f' if chars.clone().nth(1) == Some('"') => Self::scan_fstring(&mut chars, &mut tokens),
                'a'..='z' | 'A'..='Z' | '_' => Self::scan_identifier(&mut chars, &mut tokens),
                _ => Self::scan_operator(&mut chars, &mut tokens),
            }
        }

        tokens.push(Token::Eof);
        self.tokens = tokens;
        Ok(())
    }

    /// Skip a line comment (// ... \n)
    fn scan_comment(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) {
        chars.next(); // /
        chars.next(); // /
        while let Some(&ch) = chars.peek() {
            chars.next();
            if ch == '\n' {
                break;
            }
        }
    }

    /// Scan a double-quoted string literal
    fn scan_string(chars: &mut std::iter::Peekable<std::str::Chars<'_>>, tokens: &mut Vec<Token>) {
        chars.next(); // Opening "
        let mut string = String::new();
        while let Some(&ch) = chars.peek() {
            chars.next();
            if ch == '"' {
                break;
            }
            string.push(ch);
        }
        tokens.push(Token::StringLit(string));
    }

    /// Scan a character literal: 'a', '!', etc.
    fn scan_char(chars: &mut std::iter::Peekable<std::str::Chars<'_>>, tokens: &mut Vec<Token>) {
        chars.next(); // Opening '
        if let Some(&ch) = chars.peek() {
            let character = ch;
            chars.next(); // consume character
            if chars.peek() == Some(&'\'') {
                chars.next(); // consume closing '
                tokens.push(Token::CharLit(character));
            }
        }
    }

    /// Scan a numeric literal (integer or float)
    fn scan_number(chars: &mut std::iter::Peekable<std::str::Chars<'_>>, tokens: &mut Vec<Token>) {
        let mut num = String::new();
        let mut is_float = false;

        // Parse integer part
        while let Some(&ch) = chars.peek() {
            if ch.is_ascii_digit() {
                num.push(ch);
                chars.next();
            } else {
                break;
            }
        }

        // Check for decimal point
        if chars.peek() == Some(&'.') {
            // Look ahead to see if next char is a digit (not a method call like "42.abs()")
            let mut chars_clone = chars.clone();
            if let Some(next_ch) = chars_clone.nth(1) {
                if next_ch.is_ascii_digit() {
                    is_float = true;
                    num.push('.');
                    chars.next(); // consume '.'

                    // Parse fractional part
                    while let Some(&ch) = chars.peek() {
                        if ch.is_ascii_digit() {
                            num.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        // Create appropriate token
        if is_float {
            if let Ok(f) = num.parse::<f64>() {
                tokens.push(Token::Float(f));
            }
        } else if let Ok(n) = num.parse::<i64>() {
            tokens.push(Token::Integer(n));
        }
    }

    /// Scan an f-string: f"text {expr} more"
    fn scan_fstring(chars: &mut std::iter::Peekable<std::str::Chars<'_>>, tokens: &mut Vec<Token>) {
        chars.next(); // consume 'f'
        chars.next(); // consume opening "

        let mut content = String::new();
        while let Some(&ch) = chars.peek() {
            chars.next();
            if ch == '"' {
                break;
            }
            content.push(ch);
        }
        tokens.push(Token::FString(content));
    }

    /// Scan an identifier or keyword
    fn scan_identifier(chars: &mut std::iter::Peekable<std::str::Chars<'_>>, tokens: &mut Vec<Token>) {
        let mut ident = String::new();
        while let Some(&ch) = chars.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                chars.next();
            } else {
                break;
            }
        }

        let token = match ident.as_str() {
            "fun" => Token::Fun,
            "let" => Token::Let,
            "mut" => Token::Mut,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "in" => Token::In,
            "match" => Token::Match,
            "return" => Token::Return,
            "struct" => Token::Struct,
            "as" => Token::As,
            "use" => Token::Use,
            "move" => Token::Move,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Identifier(ident),
        };
        tokens.push(token);
    }

    /// Scan an operator, delimiter, or skip unknown character
    fn scan_operator(chars: &mut std::iter::Peekable<std::str::Chars<'_>>, tokens: &mut Vec<Token>) {
        let ch = *chars.peek().unwrap();
        let next = chars.clone().nth(1);

        match (ch, next) {
            // Two-character operators (check first)
            ('+', Some('=')) => { chars.next(); chars.next(); tokens.push(Token::PlusEqual); }
            ('-', Some('>')) => { chars.next(); chars.next(); tokens.push(Token::Arrow); }
            ('-', Some('=')) => { chars.next(); chars.next(); tokens.push(Token::MinusEqual); }
            ('*', Some('=')) => { chars.next(); chars.next(); tokens.push(Token::StarEqual); }
            ('/', Some('=')) => { chars.next(); chars.next(); tokens.push(Token::SlashEqual); }
            ('%', Some('=')) => { chars.next(); chars.next(); tokens.push(Token::PercentEqual); }
            (':', Some(':')) => { chars.next(); chars.next(); tokens.push(Token::ColonColon); }
            ('.', Some('.')) => { chars.next(); chars.next(); tokens.push(Token::DotDot); }
            ('=', Some('=')) => { chars.next(); chars.next(); tokens.push(Token::EqualEqual); }
            ('=', Some('>')) => { chars.next(); chars.next(); tokens.push(Token::FatArrow); }
            ('!', Some('=')) => { chars.next(); chars.next(); tokens.push(Token::NotEqual); }
            ('<', Some('=')) => { chars.next(); chars.next(); tokens.push(Token::LessEqual); }
            ('>', Some('=')) => { chars.next(); chars.next(); tokens.push(Token::GreaterEqual); }
            ('&', Some('&')) => { chars.next(); chars.next(); tokens.push(Token::AndAnd); }
            ('|', Some('|')) => { chars.next(); chars.next(); tokens.push(Token::OrOr); }

            // Single-character operators and delimiters
            ('+', _) => { chars.next(); tokens.push(Token::Plus); }
            ('-', _) => { chars.next(); tokens.push(Token::Minus); }
            ('*', _) => { chars.next(); tokens.push(Token::Star); }
            ('/', _) => { chars.next(); tokens.push(Token::Slash); }
            ('%', _) => { chars.next(); tokens.push(Token::Percent); }
            ('(', _) => { chars.next(); tokens.push(Token::LeftParen); }
            (')', _) => { chars.next(); tokens.push(Token::RightParen); }
            ('{', _) => { chars.next(); tokens.push(Token::LeftBrace); }
            ('}', _) => { chars.next(); tokens.push(Token::RightBrace); }
            ('[', _) => { chars.next(); tokens.push(Token::LeftBracket); }
            (']', _) => { chars.next(); tokens.push(Token::RightBracket); }
            (',', _) => { chars.next(); tokens.push(Token::Comma); }
            (';', _) => { chars.next(); tokens.push(Token::Semicolon); }
            (':', _) => { chars.next(); tokens.push(Token::Colon); }
            ('.', _) => { chars.next(); tokens.push(Token::Dot); }
            ('=', _) => { chars.next(); tokens.push(Token::Equal); }
            ('!', _) => { chars.next(); tokens.push(Token::Not); }
            ('<', _) => { chars.next(); tokens.push(Token::LessThan); }
            ('>', _) => { chars.next(); tokens.push(Token::GreaterThan); }
            ('|', _) => { chars.next(); tokens.push(Token::Pipe); }

            // Unknown character - skip
            _ => { chars.next(); }
        }
    }

    /// Parse a top-level item (function, struct, use statement, or statement)
    ///
    /// Supports REPL-style programming by allowing top-level statements
    /// like `println("Hello")` or `let x = 42` in addition to function/struct declarations.
    fn parse_top_level(&mut self) -> Result<AstNode, ParseError> {
        if self.check(&Token::Use) {
            self.parse_use()
        } else if self.check(&Token::Fun) {
            self.parse_function()
        } else if self.check(&Token::Struct) {
            self.parse_struct()
        } else {
            // Allow top-level statements for REPL-style programming
            // This includes: let declarations, function calls, expressions, etc.
            self.parse_statement()
        }
    }

    /// Parse a function definition
    fn parse_function(&mut self) -> Result<AstNode, ParseError> {
        self.consume(&Token::Fun)?;

        let name = if let Some(Token::Identifier(n)) = self.current().cloned() {
            self.advance();
            n
        } else {
            return Err(ParseError::InvalidSyntax {
                message: "Expected function name".to_string(),
                line: 0,
                column: 0,
            });
        };

        self.consume(&Token::LeftParen)?;

        let mut params = Vec::new();
        while !self.check(&Token::RightParen) && !self.is_at_end() {
            if let Some(Token::Identifier(p)) = self.current() {
                params.push(p.clone());
                self.advance();

                // Skip optional type annotation
                // Note: '&' in '&str' is not tokenized (skipped by tokenizer)
                // so '&str' appears as just 'str' token, making it a single-token type
                if self.check(&Token::Colon) {
                    self.advance(); // consume ':'

                    // Skip the type name (one token: 'i32', 'str', 'bool', etc.)
                    if !self.is_at_end() {
                        self.advance();
                    }
                }
            }

            if self.check(&Token::Comma) {
                self.advance();
            }
        }

        self.consume(&Token::RightParen)?;

        // Skip optional return type annotation
        // Note: '&' is not tokenized, so '&str' appears as just 'str' token, making it a single token
        if self.check(&Token::Arrow) {
            self.advance(); // consume '->'

            // Skip the return type name (one token: 'i32', 'str', 'bool', etc.)
            if !self.is_at_end() {
                self.advance();
            }
        }

        self.consume(&Token::LeftBrace)?;

        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.parse_statement()?);
        }

        self.consume(&Token::RightBrace)?;

        Ok(AstNode::FunctionDef { name, params, body })
    }

    /// Parse a struct definition
    fn parse_struct(&mut self) -> Result<AstNode, ParseError> {
        self.consume(&Token::Struct)?;

        let name = if let Some(Token::Identifier(n)) = self.current().cloned() {
            self.advance();
            n
        } else {
            return Err(ParseError::InvalidSyntax {
                message: "Expected struct name".to_string(),
                line: 0,
                column: 0,
            });
        };

        self.consume(&Token::LeftBrace)?;

        let mut fields = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            if let Some(Token::Identifier(field_name)) = self.current() {
                let field_name = field_name.clone();
                self.advance();

                let type_annotation = if self.check(&Token::Colon) {
                    self.advance();
                    if let Some(Token::Identifier(ty)) = self.current() {
                        let ty = ty.clone();
                        self.advance();
                        Some(ty)
                    } else {
                        None
                    }
                } else {
                    None
                };

                fields.push(StructField {
                    name: field_name,
                    type_annotation,
                });
            }

            if self.check(&Token::Comma) {
                self.advance();
            }
        }

        self.consume(&Token::RightBrace)?;

        Ok(AstNode::StructDef { name, fields })
    }

    /// Parse a use statement: use std::sync::Mutex;
    fn parse_use(&mut self) -> Result<AstNode, ParseError> {
        self.consume(&Token::Use)?;

        let mut path = Vec::new();

        // Parse the first identifier
        path.push(self.expect_identifier());

        // Parse :: separated path segments
        while self.check(&Token::ColonColon) {
            self.advance(); // consume ::
            path.push(self.expect_identifier());
        }

        // Check for grouped imports: use std::{Arc, Mutex};
        if self.check(&Token::LeftBrace) {
            self.advance(); // consume {

            let mut items = Vec::new();

            // Parse comma-separated items
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                items.push(self.expect_identifier());

                if self.check(&Token::Comma) {
                    self.advance();
                }
            }

            self.consume(&Token::RightBrace)?;

            // Optional semicolon
            if self.check(&Token::Semicolon) {
                self.advance();
            }

            return Ok(AstNode::GroupedUseDecl {
                base_path: path,
                items,
            });
        }

        // Optional semicolon
        if self.check(&Token::Semicolon) {
            self.advance();
        }

        Ok(AstNode::UseDecl { path })
    }

    /// Parse a statement
    fn parse_statement(&mut self) -> Result<AstNode, ParseError> {
        if self.check(&Token::Let) {
            self.parse_let()
        } else if self.check(&Token::If) {
            self.parse_if()
        } else if self.check(&Token::While) {
            self.parse_while()
        } else if self.check(&Token::For) {
            self.parse_for()
        } else if self.check(&Token::Match) {
            self.parse_match()
        } else if self.check(&Token::Return) {
            self.parse_return()
        } else {
            // Check for assignment or compound assignment
            if let Some(Token::Identifier(name)) = self.current().cloned() {
                let next_token = self.tokens.get(self.pos + 1);

                // Check for regular assignment: identifier = expr
                if next_token == Some(&Token::Equal) {
                    self.advance(); // consume identifier
                    self.advance(); // consume =
                    let value = Box::new(self.parse_expression()?);
                    if self.check(&Token::Semicolon) {
                        self.advance();
                    }
                    return Ok(AstNode::Assignment { name, value });
                }

                // Check for compound assignment: identifier += expr
                if let Some(op_token) = next_token {
                    let op = match op_token {
                        Token::PlusEqual => Some(BinaryOperator::Add),
                        Token::MinusEqual => Some(BinaryOperator::Subtract),
                        Token::StarEqual => Some(BinaryOperator::Multiply),
                        Token::SlashEqual => Some(BinaryOperator::Divide),
                        Token::PercentEqual => Some(BinaryOperator::Modulo),
                        _ => None,
                    };

                    if let Some(op) = op {
                        self.advance(); // consume identifier
                        self.advance(); // consume compound operator
                        let lhs = Box::new(AstNode::Identifier(name));
                        let rhs = Box::new(self.parse_expression()?);
                        if self.check(&Token::Semicolon) {
                            self.advance();
                        }
                        return Ok(AstNode::CompoundAssignment { lhs, op, rhs });
                    }
                }
            }

            // Check for compound assignment with dereference: *expr += value
            // Parse as expression first, then check for compound operator
            let expr = self.parse_expression()?;

            // Check if this is followed by a compound assignment operator
            if let Some(op_token) = self.current().cloned() {
                let op = match op_token {
                    Token::PlusEqual => Some(BinaryOperator::Add),
                    Token::MinusEqual => Some(BinaryOperator::Subtract),
                    Token::StarEqual => Some(BinaryOperator::Multiply),
                    Token::SlashEqual => Some(BinaryOperator::Divide),
                    Token::PercentEqual => Some(BinaryOperator::Modulo),
                    _ => None,
                };

                if let Some(op) = op {
                    self.advance(); // consume compound operator
                    let lhs = Box::new(expr);
                    let rhs = Box::new(self.parse_expression()?);
                    if self.check(&Token::Semicolon) {
                        self.advance();
                    }
                    return Ok(AstNode::CompoundAssignment { lhs, op, rhs });
                }
            }

            // Regular expression statement
            if self.check(&Token::Semicolon) {
                self.advance();
            }
            Ok(expr)
        }
    }

    /// Parse let declaration
    fn parse_let(&mut self) -> Result<AstNode, ParseError> {
        self.consume(&Token::Let)?;

        // Check for optional 'mut' keyword
        if self.check(&Token::Mut) {
            self.advance();
        }

        // Check for tuple destructuring: let (a, b) = expr
        if self.check(&Token::LeftParen) {
            self.advance(); // consume (

            let mut names = Vec::new();
            while !self.check(&Token::RightParen) && !self.is_at_end() {
                names.push(self.expect_identifier());
                if self.check(&Token::Comma) {
                    self.advance();
                }
            }
            self.consume(&Token::RightParen)?;

            self.consume(&Token::Equal)?;
            let value = Box::new(self.parse_expression()?);

            if self.check(&Token::Semicolon) {
                self.advance();
            }

            return Ok(AstNode::TupleDestruct { names, value });
        }

        // Regular let declaration: let name = expr
        let name = self.expect_identifier();

        self.consume(&Token::Equal)?;
        let value = Box::new(self.parse_expression()?);

        if self.check(&Token::Semicolon) {
            self.advance();
        }

        Ok(AstNode::LetDecl { name, value })
    }

    /// Parse if expression
    fn parse_if(&mut self) -> Result<AstNode, ParseError> {
        self.consume(&Token::If)?;

        let condition = Box::new(self.parse_expression()?);

        self.consume(&Token::LeftBrace)?;
        // INTERP-OPT-003: Pre-allocate capacity for typical if blocks (4 statements)
        let mut then_branch = Vec::with_capacity(4);
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            then_branch.push(self.parse_statement()?);
        }
        self.consume(&Token::RightBrace)?;

        let else_branch = if self.check(&Token::Else) {
            self.advance();

            // Check for 'else if' (two separate tokens)
            if self.check(&Token::If) {
                // Recursively parse the 'if' as the else branch
                // This handles 'else if' chains naturally
                let else_if = self.parse_if()?;
                Some(vec![else_if])
            } else {
                // Regular 'else { ... }' block
                self.consume(&Token::LeftBrace)?;
                // INTERP-OPT-003: Pre-allocate capacity for typical else blocks (4 statements)
                let mut else_body = Vec::with_capacity(4);
                while !self.check(&Token::RightBrace) && !self.is_at_end() {
                    else_body.push(self.parse_statement()?);
                }
                self.consume(&Token::RightBrace)?;
                Some(else_body)
            }
        } else {
            None
        };

        Ok(AstNode::IfExpr {
            condition,
            then_branch,
            else_branch,
        })
    }

    /// Parse while loop
    fn parse_while(&mut self) -> Result<AstNode, ParseError> {
        self.consume(&Token::While)?;

        let condition = Box::new(self.parse_expression()?);

        self.consume(&Token::LeftBrace)?;
        // INTERP-OPT-003: Pre-allocate capacity for typical loop bodies (4 statements)
        let mut body = Vec::with_capacity(4);
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.parse_statement()?);
        }
        self.consume(&Token::RightBrace)?;

        Ok(AstNode::WhileLoop { condition, body })
    }

    /// Parse for loop
    fn parse_for(&mut self) -> Result<AstNode, ParseError> {
        self.consume(&Token::For)?;

        // Check for tuple destructuring: for (a, b) in ...
        let var = if self.check(&Token::LeftParen) {
            self.advance(); // consume (
            let first = self.expect_identifier();
            self.consume(&Token::Comma)?;
            let second = self.expect_identifier();
            self.consume(&Token::RightParen)?;
            // Create compound identifier for tuple destructuring
            format!("({}, {})", first, second)
        } else {
            self.expect_identifier()
        };

        self.consume(&Token::In)?;

        let iterable = Box::new(self.parse_expression()?);

        self.consume(&Token::LeftBrace)?;
        // INTERP-OPT-003: Pre-allocate capacity for typical loop bodies (4 statements)
        let mut body = Vec::with_capacity(4);
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.parse_statement()?);
        }
        self.consume(&Token::RightBrace)?;

        Ok(AstNode::ForLoop {
            var,
            iterable,
            body,
        })
    }

    /// Parse match expression
    fn parse_match(&mut self) -> Result<AstNode, ParseError> {
        self.consume(&Token::Match)?;

        let expr = Box::new(self.parse_expression()?);

        self.consume(&Token::LeftBrace)?;

        let mut arms = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            let pattern = if self.check(&Token::Underscore) {
                self.advance();
                Pattern::Wildcard
            } else if let Some(Token::Integer(n)) = self.current() {
                let n = *n;
                self.advance();
                Pattern::Literal(AstNode::IntegerLiteral(n))
            } else if let Some(Token::Identifier(id)) = self.current() {
                let id = id.clone();
                self.advance();
                Pattern::Identifier(id)
            } else {
                Pattern::Wildcard
            };

            self.consume(&Token::FatArrow)?;

            let body = vec![self.parse_expression()?];

            arms.push(MatchArm { pattern, body });

            if self.check(&Token::Comma) {
                self.advance();
            }
        }

        self.consume(&Token::RightBrace)?;

        Ok(AstNode::MatchExpr { expr, arms })
    }

    /// Parse return statement
    fn parse_return(&mut self) -> Result<AstNode, ParseError> {
        self.consume(&Token::Return)?;

        let value = if !self.check(&Token::Semicolon) && !self.check(&Token::RightBrace) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        if self.check(&Token::Semicolon) {
            self.advance();
        }

        Ok(AstNode::Return { value })
    }

    /// Parse expression
    fn parse_expression(&mut self) -> Result<AstNode, ParseError> {
        self.parse_comparison()
    }

    /// Parse comparison
    fn parse_comparison(&mut self) -> Result<AstNode, ParseError> {
        let mut left = self.parse_term()?;

        while let Some(token) = self.current() {
            let op = match token {
                Token::EqualEqual => BinaryOperator::Equal,
                Token::NotEqual => BinaryOperator::NotEqual,
                Token::LessThan => BinaryOperator::LessThan,
                Token::GreaterThan => BinaryOperator::GreaterThan,
                Token::LessEqual => BinaryOperator::LessEqual,
                Token::GreaterEqual => BinaryOperator::GreaterEqual,
                Token::AndAnd => BinaryOperator::And,
                Token::OrOr => BinaryOperator::Or,
                _ => break,
            };

            self.advance();
            let right = self.parse_term()?;

            left = AstNode::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse term (+ -)
    fn parse_term(&mut self) -> Result<AstNode, ParseError> {
        let mut left = self.parse_factor()?;

        while let Some(token) = self.current() {
            let op = match token {
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Subtract,
                _ => break,
            };

            self.advance();
            let right = self.parse_factor()?;

            left = AstNode::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse factor (* / %)
    fn parse_factor(&mut self) -> Result<AstNode, ParseError> {
        let mut left = self.parse_cast()?;

        while let Some(token) = self.current() {
            let op = match token {
                Token::Star => BinaryOperator::Multiply,
                Token::Slash => BinaryOperator::Divide,
                Token::Percent => BinaryOperator::Modulo,
                _ => break,
            };

            self.advance();
            let right = self.parse_cast()?;

            left = AstNode::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse type cast (as)
    fn parse_cast(&mut self) -> Result<AstNode, ParseError> {
        let mut expr = self.parse_primary()?;

        // Check for type cast: expr as type
        while self.check(&Token::As) {
            self.advance(); // consume 'as'

            // Parse target type (for now, just expect identifier)
            let target_type = self.expect_identifier();

            expr = AstNode::TypeCast {
                expr: Box::new(expr),
                target_type,
            };
        }

        // Check for range: expr..expr
        if self.check(&Token::DotDot) {
            self.advance(); // consume '..'
            let end = Box::new(self.parse_primary()?);
            expr = AstNode::Range {
                start: Box::new(expr),
                end,
            };
        }

        Ok(expr)
    }

    /// Parse primary expression with postfix operators
    fn parse_primary(&mut self) -> Result<AstNode, ParseError> {
        // Parse base expression
        let mut expr = self.parse_primary_base()?;

        // Handle postfix operators: . (method/field), [ (index)
        loop {
            if self.check(&Token::Dot) {
                self.advance();
                let method_or_field = self.expect_identifier();

                // Check if it's a method call (followed by '(')
                if self.check(&Token::LeftParen) {
                    self.advance();
                    let mut args = Vec::new();
                    while !self.check(&Token::RightParen) && !self.is_at_end() {
                        args.push(self.parse_expression()?);
                        if self.check(&Token::Comma) {
                            self.advance();
                        }
                    }
                    self.consume(&Token::RightParen)?;
                    expr = AstNode::MethodCall {
                        receiver: Box::new(expr),
                        method: method_or_field,
                        args,
                    };
                } else {
                    // It's a field access
                    expr = AstNode::FieldAccess {
                        expr: Box::new(expr),
                        field: method_or_field,
                    };
                }
            } else if self.check(&Token::LeftBracket) {
                // Index access
                self.advance();
                let index = Box::new(self.parse_expression()?);
                self.consume(&Token::RightBracket)?;
                expr = AstNode::IndexAccess {
                    expr: Box::new(expr),
                    index,
                };
            } else {
                // No more postfix operators
                break;
            }
        }

        Ok(expr)
    }

    /// Parse base primary expression (no postfix operators)
    fn parse_primary_base(&mut self) -> Result<AstNode, ParseError> {
        match self.current() {
            Some(Token::Integer(n)) => {
                let n = *n;
                self.advance();
                Ok(AstNode::IntegerLiteral(n))
            }
            Some(Token::Float(f)) => {
                let f = *f;
                self.advance();
                Ok(AstNode::FloatLiteral(f))
            }
            Some(Token::StringLit(s)) => {
                let s = s.clone();
                self.advance();
                Ok(AstNode::StringLiteral(s))
            }
            Some(Token::CharLit(c)) => {
                let c = *c;
                self.advance();
                Ok(AstNode::CharLiteral(c))
            }
            Some(Token::FString(content)) => {
                let content = content.clone();
                self.advance();
                Ok(AstNode::FString { content })
            }
            Some(Token::True) => {
                self.advance();
                Ok(AstNode::BooleanLiteral(true))
            }
            Some(Token::False) => {
                self.advance();
                Ok(AstNode::BooleanLiteral(false))
            }
            Some(Token::Identifier(id)) => {
                let id = id.clone();
                self.advance();

                // Check for macro call (e.g., vec![...])
                if id == "vec" && self.check(&Token::Not) {
                    self.advance(); // consume !
                    self.consume(&Token::LeftBracket)?; // consume [

                    // Empty vec![]
                    if self.check(&Token::RightBracket) {
                        self.advance();
                        return Ok(AstNode::VecMacro {
                            elements: Vec::new(),
                            repeat_count: None,
                        });
                    }

                    // Parse first expression
                    let first_expr = self.parse_expression()?;

                    // Check for repeat form: vec![expr; count]
                    if self.check(&Token::Semicolon) {
                        self.advance(); // consume ;
                        let count = Box::new(self.parse_expression()?);
                        self.consume(&Token::RightBracket)?;
                        return Ok(AstNode::VecMacro {
                            elements: vec![first_expr],
                            repeat_count: Some(count),
                        });
                    }

                    // Elements form: vec![expr, expr, ...]
                    let mut elements = vec![first_expr];
                    while self.check(&Token::Comma) {
                        self.advance(); // consume ,
                        if self.check(&Token::RightBracket) {
                            break; // trailing comma
                        }
                        elements.push(self.parse_expression()?);
                    }
                    self.consume(&Token::RightBracket)?;
                    return Ok(AstNode::VecMacro {
                        elements,
                        repeat_count: None,
                    });
                }

                // Check for path expression (e.g., thread::spawn, Arc::new)
                if self.check(&Token::ColonColon) {
                    let mut segments = vec![id];
                    while self.check(&Token::ColonColon) {
                        self.advance(); // consume ::
                        segments.push(self.expect_identifier());
                    }

                    // Check for function call on path (e.g., thread::spawn(...))
                    if self.check(&Token::LeftParen) {
                        self.advance();
                        let mut args = Vec::new();
                        while !self.check(&Token::RightParen) && !self.is_at_end() {
                            args.push(self.parse_expression()?);
                            if self.check(&Token::Comma) {
                                self.advance();
                            }
                        }
                        self.consume(&Token::RightParen)?;
                        // Convert path to function name (e.g., "thread::spawn")
                        let name = segments.join("::");
                        return Ok(AstNode::FunctionCall { name, args });
                    } else {
                        // Just a path expression without call
                        return Ok(AstNode::PathExpr { segments });
                    }
                }

                // Check for function call
                if self.check(&Token::LeftParen) {
                    self.advance();
                    let mut args = Vec::new();
                    while !self.check(&Token::RightParen) && !self.is_at_end() {
                        args.push(self.parse_expression()?);
                        if self.check(&Token::Comma) {
                            self.advance();
                        }
                    }
                    self.consume(&Token::RightParen)?;
                    Ok(AstNode::FunctionCall { name: id, args })
                }
                // Check for struct literal (but only if it looks like one)
                // Struct literals have the form: Name { field: value, ... }
                // We need to check if there's a colon after the first identifier
                else if self.check(&Token::LeftBrace) {
                    // Look ahead to see if this is a struct literal
                    // by checking if we have: { identifier : ...
                    let is_struct_literal =
                        if let Some(Token::Identifier(_)) = self.tokens.get(self.pos + 1) {
                            self.tokens.get(self.pos + 2) == Some(&Token::Colon)
                        } else {
                            false
                        };

                    if is_struct_literal {
                        self.advance();
                        let mut fields = Vec::new();
                        while !self.check(&Token::RightBrace) && !self.is_at_end() {
                            if let Some(Token::Identifier(field_name)) = self.current() {
                                let field_name = field_name.clone();
                                self.advance();
                                self.consume(&Token::Colon)?;
                                let value = self.parse_expression()?;
                                fields.push((field_name, value));
                            }
                            if self.check(&Token::Comma) {
                                self.advance();
                            }
                        }
                        self.consume(&Token::RightBrace)?;
                        Ok(AstNode::StructLiteral { name: id, fields })
                    } else {
                        // Not a struct literal, just return the identifier
                        Ok(AstNode::Identifier(id))
                    }
                } else {
                    // Plain identifier - postfix operators handled by parse_primary
                    Ok(AstNode::Identifier(id))
                }
            }
            Some(Token::LeftBracket) => {
                // Vector literal
                self.advance();
                let mut elements = Vec::new();
                while !self.check(&Token::RightBracket) && !self.is_at_end() {
                    elements.push(self.parse_expression()?);
                    if self.check(&Token::Comma) {
                        self.advance();
                    }
                }
                self.consume(&Token::RightBracket)?;
                Ok(AstNode::VectorLiteral { elements })
            }
            Some(Token::LeftBrace) => {
                // Block expression or HashMap literal
                self.advance();

                // Disambiguate: block vs HashMap
                // HashMap: { key: value, ... }
                // Block: { statement; statement; ... }

                // Empty block/hashmap
                if self.check(&Token::RightBrace) {
                    self.advance();
                    return Ok(AstNode::HashMapLiteral { pairs: Vec::new() });
                }

                // Look ahead to determine if it's a HashMap or block
                // HashMap has pattern: expr : expr
                // Block has pattern: statement (often starts with keyword or ends with ;)

                // Check if first token suggests a statement (let, if, while, etc.)
                let is_block = matches!(
                    self.current(),
                    Some(Token::Let)
                        | Some(Token::If)
                        | Some(Token::While)
                        | Some(Token::For)
                        | Some(Token::Match)
                        | Some(Token::Return)
                );

                if is_block {
                    // Parse as block expression
                    let mut body = Vec::new();
                    while !self.check(&Token::RightBrace) && !self.is_at_end() {
                        body.push(self.parse_statement()?);
                    }
                    self.consume(&Token::RightBrace)?;

                    // INTERP-043: Return Block node with all statements
                    // Block creates a new scope and returns the last expression
                    Ok(AstNode::Block { statements: body })
                } else {
                    // Try parsing as HashMap
                    let mut pairs = Vec::new();
                    while !self.check(&Token::RightBrace) && !self.is_at_end() {
                        let key = self.parse_expression()?;
                        self.consume(&Token::Colon)?;
                        let value = self.parse_expression()?;
                        pairs.push((key, value));
                        if self.check(&Token::Comma) {
                            self.advance();
                        }
                    }
                    self.consume(&Token::RightBrace)?;
                    Ok(AstNode::HashMapLiteral { pairs })
                }
            }
            Some(Token::LeftParen) => {
                // Grouped expression or tuple literal
                self.advance();

                // Check for empty tuple: ()
                if self.check(&Token::RightParen) {
                    self.advance();
                    return Ok(AstNode::TupleLiteral {
                        elements: Vec::new(),
                    });
                }

                // Parse first expression
                let first_expr = self.parse_expression()?;

                // Check if this is a tuple (has comma) or grouped expression
                if self.check(&Token::Comma) {
                    // It's a tuple: (expr, expr, ...)
                    let mut elements = vec![first_expr];
                    while self.check(&Token::Comma) {
                        self.advance(); // consume comma

                        // Allow trailing comma: (1, 2,)
                        if self.check(&Token::RightParen) {
                            break;
                        }

                        elements.push(self.parse_expression()?);
                    }
                    self.consume(&Token::RightParen)?;
                    Ok(AstNode::TupleLiteral { elements })
                } else {
                    // It's a grouped expression: (expr)
                    self.consume(&Token::RightParen)?;
                    Ok(first_expr)
                }
            }
            Some(Token::Minus) => {
                // Unary minus: -expr (negative numbers, negation)
                self.advance();
                let operand = Box::new(self.parse_primary()?);
                Ok(AstNode::UnaryOp {
                    op: UnaryOperator::Negate,
                    operand,
                })
            }
            Some(Token::Not) => {
                // Unary not: !expr (boolean negation)
                self.advance();
                let operand = Box::new(self.parse_primary()?);
                Ok(AstNode::UnaryOp {
                    op: UnaryOperator::Not,
                    operand,
                })
            }
            Some(Token::Star) => {
                // Dereference operator: *expr
                // This handles cases like: *num, *counter.lock().unwrap()
                self.advance();
                let operand = Box::new(self.parse_primary()?);
                Ok(AstNode::UnaryOp {
                    op: UnaryOperator::Dereference,
                    operand,
                })
            }
            Some(Token::OrOr) => {
                // Closure with no parameters: || { body }
                self.advance(); // consume ||

                self.consume(&Token::LeftBrace)?;
                let mut body = Vec::new();
                while !self.check(&Token::RightBrace) && !self.is_at_end() {
                    body.push(self.parse_statement()?);
                }
                self.consume(&Token::RightBrace)?;

                Ok(AstNode::Closure {
                    is_move: false,
                    params: Vec::new(),
                    body,
                })
            }
            Some(Token::Pipe) | Some(Token::Move) => {
                // Closure: |params| { body } or move |params| { body }
                let is_move = if self.check(&Token::Move) {
                    self.advance();
                    // Check for || after move
                    if self.check(&Token::OrOr) {
                        self.advance();
                        self.consume(&Token::LeftBrace)?;
                        let mut body = Vec::new();
                        while !self.check(&Token::RightBrace) && !self.is_at_end() {
                            body.push(self.parse_statement()?);
                        }
                        self.consume(&Token::RightBrace)?;
                        return Ok(AstNode::Closure {
                            is_move: true,
                            params: Vec::new(),
                            body,
                        });
                    }
                    true
                } else {
                    false
                };

                self.consume(&Token::Pipe)?;
                let mut params = Vec::new();
                while !self.check(&Token::Pipe) && !self.is_at_end() {
                    params.push(self.expect_identifier());
                    if self.check(&Token::Comma) {
                        self.advance();
                    }
                }
                self.consume(&Token::Pipe)?;

                self.consume(&Token::LeftBrace)?;
                let mut body = Vec::new();
                while !self.check(&Token::RightBrace) && !self.is_at_end() {
                    body.push(self.parse_statement()?);
                }
                self.consume(&Token::RightBrace)?;

                Ok(AstNode::Closure {
                    is_move,
                    params,
                    body,
                })
            }
            _ => {
                let found = format!("{:?}", self.current());
                Err(ParseError::UnexpectedToken {
                    expected: "expression".to_string(),
                    found,
                    line: 1,
                    column: 1,
                })
            }
        }
    }

    // Helper methods

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.pos += 1;
        }
    }

    fn check(&self, token: &Token) -> bool {
        if let Some(current) = self.current() {
            std::mem::discriminant(current) == std::mem::discriminant(token)
        } else {
            false
        }
    }

    fn consume(&mut self, token: &Token) -> Result<(), ParseError> {
        if self.check(token) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", token),
                found: format!("{:?}", self.current()),
                line: 0,
                column: 0,
            })
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(self.current(), Some(Token::Eof) | None) || self.pos >= self.tokens.len()
    }

    /// Extract an identifier token and advance, or return empty string
    fn expect_identifier(&mut self) -> String {
        if let Some(Token::Identifier(name)) = self.current().cloned() {
            self.advance();
            name
        } else {
            String::new()
        }
    }

    // DEBUGGER-050: Token debugging methods (GREEN Phase Priority 1)

    /// Get the internal token list for debugging
    ///
    /// DEBUGGER-050: Exposes token stream for inspection
    pub fn debug_get_tokens(&mut self) -> Result<Vec<String>, ParseError> {
        if self.tokens.is_empty() {
            self.tokenize()?;
        }
        Ok(self.tokens.iter().map(|t| format!("{:?}", t)).collect())
    }

    /// Check if a token at the given position looks like an error recovery token
    ///
    /// DEBUGGER-050: Error detection (no Bang token in this parser, return false)
    pub fn debug_is_error_token(&self, _index: usize) -> bool {
        // This interpreter parser doesn't have Bang/error recovery tokens
        false
    }

    /// Get source code that's being parsed
    ///
    /// DEBUGGER-050: Access to source for comparison
    pub fn debug_get_source(&self) -> &str {
        &self.source
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_exists() {
        // Verify Parser type exists
        let source = "fun main() {}";
        let _parser = Parser::new(source);
    }

    #[test]
    fn test_ast_empty() {
        // Verify empty AST works
        let ast = Ast::new();
        assert_eq!(ast.nodes().len(), 0);
    }

    #[test]
    fn test_parse_implemented() {
        // REFACTOR: Verify parse() works (was RED phase test expecting panic)
        let mut parser = Parser::new("fun main() {}");
        let result = parser.parse();
        assert!(
            result.is_ok(),
            "Parser should successfully parse simple function"
        );
    }
}
