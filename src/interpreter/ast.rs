// AST type definitions for the Ruchy parser.
//
// Contains: Ast, AstNode, BinaryOperator, UnaryOperator, MatchArm,
// Pattern, StructField, ParseError, and their trait implementations.
//
// Extracted from parser.rs for file-health compliance (<2000 lines).

/// Abstract Syntax Tree
#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    /// AST nodes
    pub(crate) nodes: Vec<AstNode>,
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

    /// Emit AST back to source code (DEBUGGER-044: Property-based testing)
    ///
    /// Converts the AST back into Ruchy source code. This is used for
    /// property testing (roundtrip: parse(emit(ast)) = ast).
    ///
    /// This is a minimal implementation for property testing. It doesn't
    /// preserve exact formatting (whitespace, comments), but preserves
    /// semantic structure.
    pub fn emit(&self) -> String {
        self.nodes
            .iter()
            .map(|node| self.emit_node(node))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Emit a single AST node to source code
    fn emit_node(&self, node: &AstNode) -> String {
        match node {
            AstNode::Empty => String::new(),

            AstNode::IntegerLiteral(n) => n.to_string(),
            AstNode::FloatLiteral(f) => f.to_string(),
            AstNode::StringLiteral(s) => format!("\"{}\"", s),
            AstNode::BooleanLiteral(b) => b.to_string(),
            AstNode::Identifier(name) => name.clone(),

            AstNode::BinaryOp { left, op, right } => {
                format!(
                    "{} {} {}",
                    self.emit_node(left),
                    self.emit_binop(op),
                    self.emit_node(right)
                )
            }

            AstNode::UnaryOp { op, operand } => {
                format!("{}{}", self.emit_unaryop(op), self.emit_node(operand))
            }

            AstNode::LetDecl { name, value } => {
                format!("let {} = {};", name, self.emit_node(value))
            }

            AstNode::Assignment { name, value } => {
                format!("{} = {};", name, self.emit_node(value))
            }

            AstNode::FunctionCall { name, args } => {
                let args_str = args
                    .iter()
                    .map(|arg| self.emit_node(arg))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}({})", name, args_str)
            }

            // For complex nodes, emit minimal representation
            _ => format!("/* {:?} */", node),
        }
    }

    /// Emit binary operator
    fn emit_binop(&self, op: &BinaryOperator) -> &'static str {
        match op {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Modulo => "%",
            BinaryOperator::Equal => "==",
            BinaryOperator::NotEqual => "!=",
            BinaryOperator::LessThan => "<",
            BinaryOperator::LessEqual => "<=",
            BinaryOperator::GreaterThan => ">",
            BinaryOperator::GreaterEqual => ">=",
            BinaryOperator::And => "&&",
            BinaryOperator::Or => "||",
        }
    }

    /// Emit unary operator
    fn emit_unaryop(&self, op: &UnaryOperator) -> &'static str {
        match op {
            UnaryOperator::Negate => "-",
            UnaryOperator::Not => "!",
            UnaryOperator::Plus => "+",
            UnaryOperator::Dereference => "*",
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
        /// Function name
        name: String,
        /// Parameter names
        params: Vec<String>,
        /// Function body statements
        body: Vec<AstNode>,
    },

    /// Variable declaration: let name = expr
    LetDecl {
        /// Variable name
        name: String,
        /// Initial value expression
        value: Box<AstNode>,
    },

    /// Tuple destructuring: let (a, b, c) = expr
    TupleDestruct {
        /// Pattern variables (list of names to bind)
        names: Vec<String>,
        /// Value expression (must evaluate to tuple)
        value: Box<AstNode>,
    },

    /// Assignment: name = expr
    Assignment {
        /// Variable name
        name: String,
        /// New value expression
        value: Box<AstNode>,
    },

    /// Compound assignment: x += 5, *num -= 1
    CompoundAssignment {
        /// Left-hand side (identifier or expression like *num)
        lhs: Box<AstNode>,
        /// Operator (+, -, *, /, %)
        op: BinaryOperator,
        /// Right-hand side value
        rhs: Box<AstNode>,
    },

    /// Function call: name(args)
    FunctionCall {
        /// Function name
        name: String,
        /// Argument expressions
        args: Vec<AstNode>,
    },

    /// If expression: if condition { then_branch } else { else_branch }
    IfExpr {
        /// Condition expression
        condition: Box<AstNode>,
        /// Then branch statements
        then_branch: Vec<AstNode>,
        /// Else branch statements (optional)
        else_branch: Option<Vec<AstNode>>,
    },

    /// While loop: while condition { body }
    WhileLoop {
        /// Loop condition expression
        condition: Box<AstNode>,
        /// Loop body statements
        body: Vec<AstNode>,
    },

    /// For loop: for var in expr { body }
    ForLoop {
        /// Loop variable name
        var: String,
        /// Iterable expression
        iterable: Box<AstNode>,
        /// Loop body statements
        body: Vec<AstNode>,
    },

    /// Match expression: match expr { arms }
    MatchExpr {
        /// Expression to match against
        expr: Box<AstNode>,
        /// Match arms (patterns and bodies)
        arms: Vec<MatchArm>,
    },

    /// Struct definition
    StructDef {
        /// Struct name
        name: String,
        /// Struct fields
        fields: Vec<StructField>,
    },

    /// Struct instantiation: Name { field: value, ... }
    StructLiteral {
        /// Struct name
        name: String,
        /// Field name-value pairs
        fields: Vec<(String, AstNode)>,
    },

    /// Field access: expr.field
    FieldAccess {
        /// Expression to access field from
        expr: Box<AstNode>,
        /// Field name
        field: String,
    },

    /// Method call: receiver.method(args)
    MethodCall {
        /// Receiver expression
        receiver: Box<AstNode>,
        /// Method name
        method: String,
        /// Argument expressions
        args: Vec<AstNode>,
    },

    /// Vector literal: [elem1, elem2, ...]
    VectorLiteral {
        /// Vector elements
        elements: Vec<AstNode>,
    },

    /// HashMap literal: {key1: val1, key2: val2, ...}
    HashMapLiteral {
        /// Key-value pairs
        pairs: Vec<(AstNode, AstNode)>,
    },

    /// Tuple literal: (elem1, elem2, ...)
    TupleLiteral {
        /// Tuple elements
        elements: Vec<AstNode>,
    },

    /// Index access: expr[index]
    IndexAccess {
        /// Expression to index into
        expr: Box<AstNode>,
        /// Index expression
        index: Box<AstNode>,
    },

    /// Binary operation: left op right
    BinaryOp {
        /// Binary operator
        op: BinaryOperator,
        /// Left operand
        left: Box<AstNode>,
        /// Right operand
        right: Box<AstNode>,
    },

    /// Unary operation: op operand
    UnaryOp {
        /// Unary operator
        op: UnaryOperator,
        /// Operand expression
        operand: Box<AstNode>,
    },

    /// Type cast: expr as type
    TypeCast {
        /// Expression to cast
        expr: Box<AstNode>,
        /// Target type name
        target_type: String,
    },

    /// Range expression: start..end
    Range {
        /// Range start expression
        start: Box<AstNode>,
        /// Range end expression
        end: Box<AstNode>,
    },

    /// Return statement: return expr
    Return {
        /// Return value (optional)
        value: Option<Box<AstNode>>,
    },

    /// Identifier reference
    Identifier(String),

    /// Integer literal
    IntegerLiteral(i64),

    /// Float literal
    FloatLiteral(f64),

    /// String literal
    StringLiteral(String),

    /// Character literal: 'a', '!', etc.
    CharLiteral(char),

    /// F-string with interpolation: f"text {expr} more"
    FString {
        /// F-string content with embedded expressions
        content: String,
    },

    /// Boolean literal
    BooleanLiteral(bool),

    /// Use statement: use std::sync::Mutex;
    UseDecl {
        /// Import path (e.g., ["std", "sync", "Mutex"])
        path: Vec<String>,
    },

    /// Grouped use declaration: use std::sync::{Arc, Mutex};
    GroupedUseDecl {
        /// Base path (e.g., ["std", "sync"])
        base_path: Vec<String>,
        /// Items to import (e.g., ["Arc", "Mutex"])
        items: Vec<String>,
    },

    /// Path expression: std::thread::spawn
    PathExpr {
        /// Path segments (e.g., ["std", "thread", "spawn"])
        segments: Vec<String>,
    },

    /// Closure/lambda expression: |params| body or move |params| body
    Closure {
        /// Whether this is a move closure
        is_move: bool,
        /// Parameter names
        params: Vec<String>,
        /// Closure body statements
        body: Vec<AstNode>,
    },

    /// vec! macro: vec![], vec![1, 2, 3], vec![0; 10]
    VecMacro {
        /// Elements (for vec![1, 2, 3] form) or repeat expression (for vec![x; n] form)
        elements: Vec<AstNode>,
        /// Repeat count (for vec![x; n] form, otherwise None)
        repeat_count: Option<Box<AstNode>>,
    },

    /// Block expression: { stmt1; stmt2; expr }
    /// INTERP-043: Blocks create new scopes for variables
    Block {
        /// Statements in the block
        statements: Vec<AstNode>,
    },
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
    /// Addition operator (+)
    Add,
    /// Subtraction operator (-)
    Subtract,
    /// Multiplication operator (*)
    Multiply,
    /// Division operator (/)
    Divide,
    /// Modulo operator (%)
    Modulo,
    /// Equality operator (==)
    Equal,
    /// Inequality operator (!=)
    NotEqual,
    /// Less than operator (<)
    LessThan,
    /// Greater than operator (>)
    GreaterThan,
    /// Less than or equal operator (<=)
    LessEqual,
    /// Greater than or equal operator (>=)
    GreaterEqual,
    /// Logical AND operator (&&)
    And,
    /// Logical OR operator (||)
    Or,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    /// Unary negation operator (-)
    Negate,
    /// Logical NOT operator (!)
    Not,
    /// Unary plus operator (+, identity)
    Plus,
    /// Dereference operator (*) - extract value from pointer/wrapper
    Dereference,
}

/// Match arm in match expression
#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    /// Pattern to match
    pub pattern: Pattern,
    /// Statements to execute if pattern matches
    pub body: Vec<AstNode>,
}

/// Pattern in match arm
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// Wildcard pattern (_) - matches anything
    Wildcard,
    /// Literal pattern (0, "hello", true) - matches specific value
    Literal(AstNode),
    /// Identifier pattern (x) - binds variable
    Identifier(String),
}

/// Struct field definition
#[derive(Debug, Clone, PartialEq)]
pub struct StructField {
    /// Field name
    pub name: String,
    /// Optional type annotation
    pub type_annotation: Option<String>,
}

/// Parse errors
#[derive(Debug, Clone)]
pub enum ParseError {
    /// Unexpected token encountered
    UnexpectedToken {
        /// Expected token description
        expected: String,
        /// Found token description
        found: String,
        /// Line number (1-based)
        line: usize,
        /// Column number (1-based)
        column: usize,
    },

    /// Unexpected end of file
    UnexpectedEof,

    /// Invalid syntax
    InvalidSyntax {
        /// Error message
        message: String,
        /// Line number (1-based)
        line: usize,
        /// Column number (1-based)
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
