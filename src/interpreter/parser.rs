// INTERP-001: AST Parser Integration
// RED Phase: Minimal type definitions for test compilation
//
// Research: Aho et al. (2006) Chapter 4: Syntax Analysis
//
// This is the RED phase - types exist but implementation is stubbed
// All tests should FAIL because parse() returns unimplemented!()

/// Parser for Ruchy source code
pub struct Parser {
    source: String,
}

impl Parser {
    /// Create a new parser for the given source code
    pub fn new(source: &str) -> Self {
        Parser {
            source: source.to_string(),
        }
    }

    /// Parse the source code into an AST
    ///
    /// RED Phase: This returns unimplemented!() so all tests fail
    pub fn parse(&mut self) -> Result<Ast, ParseError> {
        // RED: Not implemented yet - all tests should fail here
        unimplemented!("INTERP-001 RED: Parser not yet implemented")
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
    LetDecl {
        name: String,
        value: Box<AstNode>,
    },

    /// Function call: name(args)
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },

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
    FieldAccess {
        expr: Box<AstNode>,
        field: String,
    },

    /// Vector literal: [elem1, elem2, ...]
    VectorLiteral {
        elements: Vec<AstNode>,
    },

    /// HashMap literal: {key1: val1, key2: val2, ...}
    HashMapLiteral {
        pairs: Vec<(AstNode, AstNode)>,
    },

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

    /// Return statement: return expr
    Return {
        value: Option<Box<AstNode>>,
    },

    /// Identifier reference
    Identifier(String),

    /// Integer literal
    IntegerLiteral(i64),

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
    Add,       // +
    Subtract,  // -
    Multiply,  // *
    Divide,    // /
    Modulo,    // %
    Equal,     // ==
    NotEqual,  // !=
    LessThan,  // <
    GreaterThan, // >
    LessEqual, // <=
    GreaterEqual, // >=
    And,       // &&
    Or,        // ||
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
    Wildcard,              // _
    Literal(AstNode),      // 0, "hello", true
    Identifier(String),    // x (binds variable)
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
    #[should_panic(expected = "not yet implemented")]
    fn test_parse_unimplemented() {
        // RED: parse() should panic with unimplemented
        let mut parser = Parser::new("fun main() {}");
        let _ = parser.parse();
    }
}
