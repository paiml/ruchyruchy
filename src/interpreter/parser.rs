// INTERP-001: AST Parser Integration
// REFACTOR Phase: Clean up implementation while keeping tests green
//
// Research: Aho et al. (2006) Chapter 4: Syntax Analysis
//
// This is a recursive descent parser with operator precedence
// for Ruchy language syntax. Supports functions, structs, control
// flow, expressions, and data structures.

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

    // Identifiers and literals
    Identifier(String),
    Integer(i64),
    Float(f64),
    StringLit(String),
    True,
    False,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    EqualEqual,
    NotEqual,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    AndAnd,
    OrOr,

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
    Arrow,
    FatArrow,
    Dot,
    Equal,
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
        let mut nodes = Vec::new();

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

    /// Tokenize the source code into a vector of tokens
    ///
    /// Handles whitespace, comments, string literals, numbers, identifiers,
    /// keywords, and operators/delimiters
    fn tokenize(&mut self) -> Result<(), ParseError> {
        let mut chars = self.source.chars().peekable();
        let mut tokens = Vec::new();

        while let Some(&ch) = chars.peek() {
            match ch {
                // Whitespace
                ' ' | '\t' | '\n' | '\r' => {
                    chars.next();
                }

                // Comments
                '/' if chars.clone().nth(1) == Some('/') => {
                    // Skip until end of line
                    chars.next(); // /
                    chars.next(); // /
                    while let Some(&ch) = chars.peek() {
                        chars.next();
                        if ch == '\n' {
                            break;
                        }
                    }
                }

                // String literals
                '"' => {
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

                // Numbers (integers and floats)
                '0'..='9' => {
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
                        let chars_clone = chars.clone();
                        if let Some(next_ch) = chars_clone.skip(1).next() {
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
                    } else {
                        if let Ok(n) = num.parse::<i64>() {
                            tokens.push(Token::Integer(n));
                        }
                    }
                }

                // Identifiers and keywords
                'a'..='z' | 'A'..='Z' | '_' => {
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
                        "if" => Token::If,
                        "else" => Token::Else,
                        "while" => Token::While,
                        "for" => Token::For,
                        "in" => Token::In,
                        "match" => Token::Match,
                        "return" => Token::Return,
                        "struct" => Token::Struct,
                        "true" => Token::True,
                        "false" => Token::False,
                        _ => Token::Identifier(ident),
                    };
                    tokens.push(token);
                }

                // Operators and delimiters
                '+' => {
                    chars.next();
                    tokens.push(Token::Plus);
                }
                '-' if chars.clone().nth(1) == Some('>') => {
                    chars.next();
                    chars.next();
                    tokens.push(Token::Arrow);
                }
                '-' => {
                    chars.next();
                    tokens.push(Token::Minus);
                }
                '*' => {
                    chars.next();
                    tokens.push(Token::Star);
                }
                '/' => {
                    chars.next();
                    tokens.push(Token::Slash);
                }
                '%' => {
                    chars.next();
                    tokens.push(Token::Percent);
                }
                '(' => {
                    chars.next();
                    tokens.push(Token::LeftParen);
                }
                ')' => {
                    chars.next();
                    tokens.push(Token::RightParen);
                }
                '{' => {
                    chars.next();
                    tokens.push(Token::LeftBrace);
                }
                '}' => {
                    chars.next();
                    tokens.push(Token::RightBrace);
                }
                '[' => {
                    chars.next();
                    tokens.push(Token::LeftBracket);
                }
                ']' => {
                    chars.next();
                    tokens.push(Token::RightBracket);
                }
                ',' => {
                    chars.next();
                    tokens.push(Token::Comma);
                }
                ';' => {
                    chars.next();
                    tokens.push(Token::Semicolon);
                }
                ':' => {
                    chars.next();
                    tokens.push(Token::Colon);
                }
                '.' => {
                    chars.next();
                    tokens.push(Token::Dot);
                }

                '=' if chars.clone().nth(1) == Some('=') => {
                    chars.next();
                    chars.next();
                    tokens.push(Token::EqualEqual);
                }
                '=' if chars.clone().nth(1) == Some('>') => {
                    chars.next();
                    chars.next();
                    tokens.push(Token::FatArrow);
                }
                '=' => {
                    chars.next();
                    tokens.push(Token::Equal);
                }

                '!' if chars.clone().nth(1) == Some('=') => {
                    chars.next();
                    chars.next();
                    tokens.push(Token::NotEqual);
                }

                '<' if chars.clone().nth(1) == Some('=') => {
                    chars.next();
                    chars.next();
                    tokens.push(Token::LessEqual);
                }
                '<' => {
                    chars.next();
                    tokens.push(Token::LessThan);
                }

                '>' if chars.clone().nth(1) == Some('=') => {
                    chars.next();
                    chars.next();
                    tokens.push(Token::GreaterEqual);
                }
                '>' => {
                    chars.next();
                    tokens.push(Token::GreaterThan);
                }

                '&' if chars.clone().nth(1) == Some('&') => {
                    chars.next();
                    chars.next();
                    tokens.push(Token::AndAnd);
                }

                '|' if chars.clone().nth(1) == Some('|') => {
                    chars.next();
                    chars.next();
                    tokens.push(Token::OrOr);
                }

                _ => {
                    // Unknown character - skip
                    chars.next();
                }
            }
        }

        tokens.push(Token::Eof);
        self.tokens = tokens;
        Ok(())
    }

    /// Parse a top-level item (function, struct, or statement)
    ///
    /// Supports REPL-style programming by allowing top-level statements
    /// like `println("Hello")` or `let x = 42` in addition to function/struct declarations.
    fn parse_top_level(&mut self) -> Result<AstNode, ParseError> {
        if self.check(&Token::Fun) {
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
                if self.check(&Token::Colon) {
                    self.advance(); // consume ':'
                    self.advance(); // skip type (for now, just skip one token)
                }
            }

            if self.check(&Token::Comma) {
                self.advance();
            }
        }

        self.consume(&Token::RightParen)?;

        // Skip optional return type annotation
        if self.check(&Token::Arrow) {
            self.advance(); // consume '->'
            self.advance(); // skip return type (for now, just skip one token)
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
            // Check for assignment: identifier = expr
            if let Some(Token::Identifier(name)) = self.current().cloned() {
                // Look ahead to see if next token is =
                if self.tokens.get(self.pos + 1) == Some(&Token::Equal) {
                    self.advance(); // consume identifier
                    self.advance(); // consume =
                    let value = Box::new(self.parse_expression()?);
                    if self.check(&Token::Semicolon) {
                        self.advance();
                    }
                    return Ok(AstNode::Assignment { name, value });
                }
            }

            // Expression statement
            let expr = self.parse_expression()?;
            if self.check(&Token::Semicolon) {
                self.advance();
            }
            Ok(expr)
        }
    }

    /// Parse let declaration
    fn parse_let(&mut self) -> Result<AstNode, ParseError> {
        self.consume(&Token::Let)?;

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
        let mut then_branch = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            then_branch.push(self.parse_statement()?);
        }
        self.consume(&Token::RightBrace)?;

        let else_branch = if self.check(&Token::Else) {
            self.advance();
            self.consume(&Token::LeftBrace)?;
            let mut else_body = Vec::new();
            while !self.check(&Token::RightBrace) && !self.is_at_end() {
                else_body.push(self.parse_statement()?);
            }
            self.consume(&Token::RightBrace)?;
            Some(else_body)
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
        let mut body = Vec::new();
        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            body.push(self.parse_statement()?);
        }
        self.consume(&Token::RightBrace)?;

        Ok(AstNode::WhileLoop { condition, body })
    }

    /// Parse for loop
    fn parse_for(&mut self) -> Result<AstNode, ParseError> {
        self.consume(&Token::For)?;

        let var = self.expect_identifier();

        self.consume(&Token::In)?;

        let iterable = Box::new(self.parse_expression()?);

        self.consume(&Token::LeftBrace)?;
        let mut body = Vec::new();
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
        let mut left = self.parse_primary()?;

        while let Some(token) = self.current() {
            let op = match token {
                Token::Star => BinaryOperator::Multiply,
                Token::Slash => BinaryOperator::Divide,
                Token::Percent => BinaryOperator::Modulo,
                _ => break,
            };

            self.advance();
            let right = self.parse_primary()?;

            left = AstNode::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    /// Parse primary expression
    fn parse_primary(&mut self) -> Result<AstNode, ParseError> {
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
                }
                // Check for field access
                else if self.check(&Token::Dot) {
                    self.advance();
                    let field = self.expect_identifier();
                    Ok(AstNode::FieldAccess {
                        expr: Box::new(AstNode::Identifier(id)),
                        field,
                    })
                }
                // Check for index access
                else if self.check(&Token::LeftBracket) {
                    self.advance();
                    let index = Box::new(self.parse_expression()?);
                    self.consume(&Token::RightBracket)?;
                    Ok(AstNode::IndexAccess {
                        expr: Box::new(AstNode::Identifier(id)),
                        index,
                    })
                } else {
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
                // HashMap literal
                self.advance();
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
            Some(Token::LeftParen) => {
                // Grouped expression
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(&Token::RightParen)?;
                Ok(expr)
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
}

/// Abstract Syntax Tree
#[derive(Debug, Clone)]
pub struct Ast {
    nodes: Vec<AstNode>,
}

impl Ast {
    /// Create a new empty AST
    pub fn new() -> Self {
        Ast { nodes: Vec::new() }
    }

    /// Get nodes in the AST
    pub fn nodes(&self) -> &[AstNode] {
        &self.nodes
    }

    /// Visit all nodes in the AST with a callback
    pub fn visit<F>(&self, mut callback: F)
    where
        F: FnMut(&AstNode),
    {
        for node in &self.nodes {
            callback(node);
            node.visit_children(&mut callback);
        }
    }
}

impl Default for Ast {
    fn default() -> Self {
        Self::new()
    }
}

/// AST Node types
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    /// Empty node (for comparisons)
    Empty,

    /// Function definition: fun name(params) { body }
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Vec<AstNode>,
    },

    /// Variable declaration: let name = expr
    LetDecl { name: String, value: Box<AstNode> },

    /// Assignment: name = expr
    Assignment { name: String, value: Box<AstNode> },

    /// Function call: name(args)
    FunctionCall { name: String, args: Vec<AstNode> },

    /// If expression: if condition { then_branch } else { else_branch }
    IfExpr {
        condition: Box<AstNode>,
        then_branch: Vec<AstNode>,
        else_branch: Option<Vec<AstNode>>,
    },

    /// While loop: while condition { body }
    WhileLoop {
        condition: Box<AstNode>,
        body: Vec<AstNode>,
    },

    /// For loop: for var in expr { body }
    ForLoop {
        var: String,
        iterable: Box<AstNode>,
        body: Vec<AstNode>,
    },

    /// Match expression: match expr { arms }
    MatchExpr {
        expr: Box<AstNode>,
        arms: Vec<MatchArm>,
    },

    /// Struct definition
    StructDef {
        name: String,
        fields: Vec<StructField>,
    },

    /// Struct instantiation: Name { field: value, ... }
    StructLiteral {
        name: String,
        fields: Vec<(String, AstNode)>,
    },

    /// Field access: expr.field
    FieldAccess { expr: Box<AstNode>, field: String },

    /// Vector literal: [elem1, elem2, ...]
    VectorLiteral { elements: Vec<AstNode> },

    /// HashMap literal: {key1: val1, key2: val2, ...}
    HashMapLiteral { pairs: Vec<(AstNode, AstNode)> },

    /// Index access: expr[index]
    IndexAccess {
        expr: Box<AstNode>,
        index: Box<AstNode>,
    },

    /// Binary operation: left op right
    BinaryOp {
        op: BinaryOperator,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },

    /// Unary operation: op operand
    UnaryOp {
        op: UnaryOperator,
        operand: Box<AstNode>,
    },

    /// Return statement: return expr
    Return { value: Option<Box<AstNode>> },

    /// Identifier reference
    Identifier(String),

    /// Integer literal
    IntegerLiteral(i64),

    /// Float literal
    FloatLiteral(f64),

    /// String literal
    StringLiteral(String),

    /// Boolean literal
    BooleanLiteral(bool),
}

impl AstNode {
    /// Visit all children of this node
    fn visit_children<F>(&self, callback: &mut F)
    where
        F: FnMut(&AstNode),
    {
        match self {
            AstNode::FunctionDef { body, .. } => {
                for node in body {
                    callback(node);
                    node.visit_children(callback);
                }
            }
            AstNode::LetDecl { value, .. } => {
                callback(value);
                value.visit_children(callback);
            }
            AstNode::Assignment { value, .. } => {
                callback(value);
                value.visit_children(callback);
            }
            AstNode::FunctionCall { args, .. } => {
                for arg in args {
                    callback(arg);
                    arg.visit_children(callback);
                }
            }
            AstNode::IfExpr {
                condition,
                then_branch,
                else_branch,
            } => {
                callback(condition);
                condition.visit_children(callback);
                for node in then_branch {
                    callback(node);
                    node.visit_children(callback);
                }
                if let Some(else_branch) = else_branch {
                    for node in else_branch {
                        callback(node);
                        node.visit_children(callback);
                    }
                }
            }
            AstNode::BinaryOp { left, right, .. } => {
                callback(left);
                left.visit_children(callback);
                callback(right);
                right.visit_children(callback);
            }
            AstNode::UnaryOp { operand, .. } => {
                callback(operand);
                operand.visit_children(callback);
            }
            _ => {
                // Other node types: visit children as needed
                // This is minimal for RED phase
            }
        }
    }
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,          // +
    Subtract,     // -
    Multiply,     // *
    Divide,       // /
    Modulo,       // %
    Equal,        // ==
    NotEqual,     // !=
    LessThan,     // <
    GreaterThan,  // >
    LessEqual,    // <=
    GreaterEqual, // >=
    And,          // &&
    Or,           // ||
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Negate, // - (unary minus)
    Not,    // ! (logical not)
    Plus,   // + (unary plus, identity)
}

/// Match arm in match expression
#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Vec<AstNode>,
}

/// Pattern in match arm
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Wildcard,           // _
    Literal(AstNode),   // 0, "hello", true
    Identifier(String), // x (binds variable)
}

/// Struct field definition
#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    pub name: String,
    pub type_annotation: Option<String>,
}

/// Parse errors
#[derive(Debug, Clone)]
pub enum ParseError {
    /// Unexpected token encountered
    UnexpectedToken {
        expected: String,
        found: String,
        line: usize,
        column: usize,
    },

    /// Unexpected end of file
    UnexpectedEof,

    /// Invalid syntax
    InvalidSyntax {
        message: String,
        line: usize,
        column: usize,
    },

    /// Not yet implemented
    Unimplemented(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken {
                expected,
                found,
                line,
                column,
            } => {
                write!(
                    f,
                    "Unexpected token at {}:{}: expected {}, found {}",
                    line, column, expected, found
                )
            }
            ParseError::UnexpectedEof => {
                write!(f, "Unexpected end of file")
            }
            ParseError::InvalidSyntax {
                message,
                line,
                column,
            } => {
                write!(f, "Invalid syntax at {}:{}: {}", line, column, message)
            }
            ParseError::Unimplemented(msg) => {
                write!(f, "Not yet implemented: {}", msg)
            }
        }
    }
}

impl std::error::Error for ParseError {}

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
