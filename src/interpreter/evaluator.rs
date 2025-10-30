// INTERP-005: Function Calls & Recursion
// GREEN Phase: Complete implementation with all tests passing
//
// Research:
// - Aho et al. (2006) Chapter 8: Expression Evaluation
// - Aho et al. (2006) Chapter 7: Runtime Environments
//
// This module implements expression evaluation and function calls for the Ruchy interpreter.
// Supports function definition, function calls with recursion, control flow, and variable scoping.
//
// Design:
// - Tree-walking evaluator that recursively evaluates AST nodes
// - Function registry for storing function definitions
// - Call stack depth tracking for stack overflow detection
// - Scope management for function parameters and local variables
// - Control flow mechanism for early returns (ControlFlow enum)
// - Operator precedence handled by parser (AST structure)
// - Type safety enforced at runtime through Value operations
// - Error propagation via Result types

use crate::interpreter::parser::{AstNode, BinaryOperator, UnaryOperator};
use crate::interpreter::scope::Scope;
use crate::interpreter::value::{Value, ValueError};
use std::collections::HashMap;
use std::fmt;

/// Maximum recursion depth before stack overflow
/// This is set conservatively to prevent actual Rust stack overflow
const MAX_CALL_DEPTH: usize = 150;

/// Evaluator executes AST nodes and produces values
pub struct Evaluator {
    /// Scope for variable lookups
    scope: Scope,
    /// Function registry: name -> (params, body)
    functions: HashMap<String, (Vec<String>, Vec<AstNode>)>,
    /// Current call depth for stack overflow detection
    call_depth: usize,
}

/// Internal control flow for handling early returns
///
/// When evaluating function bodies, we need to distinguish between:
/// - Normal evaluation (last expression value)
/// - Early return (explicit return statement)
///
/// This enum allows return statements to propagate up through nested
/// control structures (if/else, loops) without executing remaining statements.
enum ControlFlow {
    /// Normal value - continues evaluating subsequent statements
    Value(Value),
    /// Early return from function - stops evaluation and returns immediately
    Return(Value),
}

/// Evaluation errors
#[derive(Debug, Clone)]
pub enum EvalError {
    /// Value error (type mismatch, division by zero, etc.)
    ValueError(ValueError),
    /// Undefined variable
    UndefinedVariable { name: String },
    /// Undefined function
    UndefinedFunction { name: String },
    /// Argument count mismatch in function call
    ArgumentCountMismatch {
        function: String,
        expected: usize,
        actual: usize,
    },
    /// Stack overflow from excessive recursion
    StackOverflow,
    /// No match arm matched in match expression
    NoMatchArm,
    /// Unsupported operation
    UnsupportedOperation { operation: String },
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalError::ValueError(err) => write!(f, "{}", err),
            EvalError::UndefinedVariable { name } => {
                write!(f, "Undefined variable: {}", name)
            }
            EvalError::UndefinedFunction { name } => {
                write!(f, "Undefined function: {}", name)
            }
            EvalError::ArgumentCountMismatch {
                function,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Function '{}' expects {} arguments, but {} were provided",
                    function, expected, actual
                )
            }
            EvalError::StackOverflow => {
                write!(f, "Stack overflow: recursion depth exceeded")
            }
            EvalError::NoMatchArm => {
                write!(f, "No match arm matched")
            }
            EvalError::UnsupportedOperation { operation } => {
                write!(f, "Unsupported operation: {}", operation)
            }
        }
    }
}

impl std::error::Error for EvalError {}

impl From<ValueError> for EvalError {
    fn from(err: ValueError) -> Self {
        EvalError::ValueError(err)
    }
}

impl Evaluator {
    /// Create a new evaluator
    pub fn new() -> Self {
        Evaluator {
            scope: Scope::new(),
            functions: HashMap::new(),
            call_depth: 0,
        }
    }

    /// Evaluate an AST node and return a value
    ///
    /// Recursively evaluates the AST by pattern matching on node type.
    /// Literals are converted directly to values. Operations are evaluated
    /// by recursively evaluating operands first, then applying the operation.
    ///
    /// # Example
    /// ```ignore
    /// let mut eval = Evaluator::new();
    /// let node = AstNode::BinaryOp {
    ///     op: BinaryOperator::Add,
    ///     left: Box::new(AstNode::IntegerLiteral(2)),
    ///     right: Box::new(AstNode::IntegerLiteral(3)),
    /// };
    /// let result = eval.eval(&node).unwrap();
    /// assert_eq!(result.as_integer().unwrap(), 5);
    /// ```
    pub fn eval(&mut self, node: &AstNode) -> Result<Value, EvalError> {
        match self.eval_internal(node)? {
            ControlFlow::Value(v) => Ok(v),
            ControlFlow::Return(v) => Ok(v),
        }
    }

    /// Internal evaluation with control flow support
    fn eval_internal(&mut self, node: &AstNode) -> Result<ControlFlow, EvalError> {
        match node {
            // Literals - direct conversion to values
            AstNode::IntegerLiteral(n) => Ok(ControlFlow::Value(Value::integer(*n))),
            AstNode::StringLiteral(s) => Ok(ControlFlow::Value(Value::string(s.clone()))),
            AstNode::BooleanLiteral(b) => Ok(ControlFlow::Value(Value::boolean(*b))),

            // Binary operations - evaluate operands then apply operator
            AstNode::BinaryOp { op, left, right } => {
                let left_val = self.eval(left)?;
                let right_val = self.eval(right)?;
                let result = self.eval_binary_op(*op, left_val, right_val)?;
                Ok(ControlFlow::Value(result))
            }

            // Unary operations - evaluate operand then apply operator
            AstNode::UnaryOp { op, operand } => {
                let operand_val = self.eval(operand)?;
                let result = self.eval_unary_op(*op, operand_val)?;
                Ok(ControlFlow::Value(result))
            }

            // Function definition - register function
            AstNode::FunctionDef { name, params, body } => {
                self.functions.insert(name.clone(), (params.clone(), body.clone()));
                Ok(ControlFlow::Value(Value::nil()))
            }

            // Function call - evaluate arguments and call function
            AstNode::FunctionCall { name, args } => {
                let result = self.call_function(name, args)?;
                Ok(ControlFlow::Value(result))
            }

            // Identifier - lookup variable in scope
            AstNode::Identifier(name) => {
                let value = self.scope.get_cloned(name)
                    .map_err(|_| EvalError::UndefinedVariable { name: name.clone() })?;
                Ok(ControlFlow::Value(value))
            }

            // Return statement - early return from function
            AstNode::Return { value } => {
                let return_val = if let Some(expr) = value {
                    self.eval(expr)?
                } else {
                    Value::nil()
                };
                Ok(ControlFlow::Return(return_val))
            }

            // If expression - conditional branching
            AstNode::IfExpr { condition, then_branch, else_branch } => {
                self.eval_if(condition, then_branch, else_branch)
            }

            // Let declaration - variable binding
            AstNode::LetDecl { name, value } => {
                let val = self.eval(value)?;
                self.scope.define(name.clone(), val)
                    .map_err(|e| EvalError::UnsupportedOperation {
                        operation: format!("define variable: {}", e)
                    })?;
                Ok(ControlFlow::Value(Value::nil()))
            }

            // While loop (STUB - INTERP-006 RED PHASE)
            AstNode::WhileLoop { .. } => {
                unimplemented!("INTERP-006: While loop not yet implemented")
            }

            // For loop (STUB - INTERP-006 RED PHASE)
            AstNode::ForLoop { .. } => {
                unimplemented!("INTERP-006: For loop not yet implemented")
            }

            // Match expression (STUB - INTERP-006 RED PHASE)
            AstNode::MatchExpr { .. } => {
                unimplemented!("INTERP-006: Match expression not yet implemented")
            }

            // Assignment (STUB - INTERP-006 RED PHASE)
            AstNode::Assignment { .. } => {
                unimplemented!("INTERP-006: Assignment not yet implemented")
            }

            // Vector literal (STUB - INTERP-006 RED PHASE)
            AstNode::VectorLiteral { .. } => {
                unimplemented!("INTERP-006: Vector literal not yet implemented")
            }

            // Unsupported nodes
            _ => Err(EvalError::UnsupportedOperation {
                operation: format!("{:?}", node),
            }),
        }
    }

    /// Evaluate a binary operation
    ///
    /// Applies a binary operator to two values. Operations are grouped into:
    /// - Arithmetic: +, -, *, /, %
    /// - Comparison: <, >, ==, !=, <=, >=
    /// - Logical: &&, ||
    ///
    /// Type checking is performed by Value methods.
    fn eval_binary_op(
        &self,
        op: BinaryOperator,
        left: Value,
        right: Value,
    ) -> Result<Value, EvalError> {
        match op {
            // Arithmetic operators
            BinaryOperator::Add => Ok(left.add(&right)?),
            BinaryOperator::Subtract => Ok(left.subtract(&right)?),
            BinaryOperator::Multiply => Ok(left.multiply(&right)?),
            BinaryOperator::Divide => Ok(left.divide(&right)?),
            BinaryOperator::Modulo => self.eval_modulo(left, right),

            // Comparison operators
            BinaryOperator::LessThan => Ok(left.less_than(&right)?),
            BinaryOperator::GreaterThan => Ok(left.greater_than(&right)?),
            BinaryOperator::Equal => Ok(left.equals(&right)?),
            BinaryOperator::NotEqual => {
                // NotEqual is the logical NOT of Equal
                let equals = left.equals(&right)?;
                Ok(Value::boolean(!equals.as_boolean()?))
            }
            BinaryOperator::LessEqual => {
                // LessEqual is LessThan OR Equal
                let less = left.less_than(&right)?;
                let equal = left.equals(&right)?;
                Ok(Value::boolean(
                    less.as_boolean()? || equal.as_boolean()?,
                ))
            }
            BinaryOperator::GreaterEqual => {
                // GreaterEqual is GreaterThan OR Equal
                let greater = left.greater_than(&right)?;
                let equal = left.equals(&right)?;
                Ok(Value::boolean(
                    greater.as_boolean()? || equal.as_boolean()?,
                ))
            }

            // Logical operators
            BinaryOperator::And => Ok(left.logical_and(&right)?),
            BinaryOperator::Or => Ok(left.logical_or(&right)?),
        }
    }

    /// Evaluate modulo operation
    fn eval_modulo(&self, left: Value, right: Value) -> Result<Value, EvalError> {
        let left_int = left.as_integer()?;
        let right_int = right.as_integer()?;

        if right_int == 0 {
            return Err(EvalError::ValueError(ValueError::DivisionByZero));
        }

        Ok(Value::integer(left_int % right_int))
    }

    /// Evaluate a unary operation
    fn eval_unary_op(&self, op: UnaryOperator, operand: Value) -> Result<Value, EvalError> {
        match op {
            UnaryOperator::Negate => {
                let n = operand.as_integer()?;
                Ok(Value::integer(-n))
            }
            UnaryOperator::Plus => {
                // Unary plus is identity for integers
                let n = operand.as_integer()?;
                Ok(Value::integer(n))
            }
            UnaryOperator::Not => {
                // Logical NOT for booleans
                Ok(operand.logical_not()?)
            }
        }
    }

    /// Call a function with arguments
    ///
    /// Implements function call semantics:
    /// 1. Stack overflow protection
    /// 2. Function lookup in registry
    /// 3. Argument count validation
    /// 4. Eager argument evaluation (call-by-value)
    /// 5. New scope creation with parameter binding
    /// 6. Function body execution with early return support
    /// 7. Scope restoration after function exit
    ///
    /// Returns the last expression value or explicit return value.
    fn call_function(&mut self, name: &str, args: &[AstNode]) -> Result<Value, EvalError> {
        // 1. Check stack depth before recursing (prevent stack overflow)
        if self.call_depth >= MAX_CALL_DEPTH {
            return Err(EvalError::StackOverflow);
        }

        // 2. Look up function in registry
        let (params, body) = self.functions.get(name)
            .cloned()
            .ok_or_else(|| EvalError::UndefinedFunction { name: name.to_string() })?;

        // 3. Check argument count matches parameter count (arity check)
        if args.len() != params.len() {
            return Err(EvalError::ArgumentCountMismatch {
                function: name.to_string(),
                expected: params.len(),
                actual: args.len(),
            });
        }

        // 4. Evaluate all arguments eagerly (call-by-value semantics)
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.eval(arg)?);
        }

        // 5. Create new scope and bind parameters to argument values
        let saved_scope = std::mem::replace(&mut self.scope, Scope::new());
        for (param, value) in params.iter().zip(arg_values.iter()) {
            self.scope.define(param.clone(), value.clone())
                .map_err(|e| EvalError::UnsupportedOperation {
                    operation: format!("define parameter: {}", e)
                })?;
        }

        // Increment call depth for recursion tracking
        self.call_depth += 1;

        // 6. Execute function body, handling early returns
        let mut result = Value::nil();
        for stmt in &body {
            match self.eval_internal(stmt)? {
                ControlFlow::Value(v) => {
                    // Normal evaluation - update result and continue
                    result = v;
                }
                ControlFlow::Return(v) => {
                    // Early return - stop executing and return immediately
                    result = v;
                    break;
                }
            }
        }

        // 7. Restore previous scope and call depth
        self.call_depth -= 1;
        self.scope = saved_scope;

        Ok(result)
    }

    /// Evaluate if expression with conditional branching
    ///
    /// Evaluates the condition, then executes either the then_branch or else_branch
    /// based on the result. Returns the last expression value from the executed branch.
    ///
    /// Early returns are propagated up to allow:
    ///   if (condition) { return value; }
    fn eval_if(
        &mut self,
        condition: &AstNode,
        then_branch: &[AstNode],
        else_branch: &Option<Vec<AstNode>>,
    ) -> Result<ControlFlow, EvalError> {
        // Evaluate condition and check it's a boolean
        let cond_val = self.eval(condition)?;
        let cond_bool = cond_val.as_boolean()?;

        if cond_bool {
            // Execute then branch
            let mut result = Value::nil();
            for stmt in then_branch {
                match self.eval_internal(stmt)? {
                    ControlFlow::Value(v) => {
                        // Normal evaluation - continue with next statement
                        result = v;
                    }
                    ControlFlow::Return(v) => {
                        // Early return - propagate immediately
                        return Ok(ControlFlow::Return(v));
                    }
                }
            }
            Ok(ControlFlow::Value(result))
        } else if let Some(else_stmts) = else_branch {
            // Execute else branch
            let mut result = Value::nil();
            for stmt in else_stmts {
                match self.eval_internal(stmt)? {
                    ControlFlow::Value(v) => {
                        // Normal evaluation - continue with next statement
                        result = v;
                    }
                    ControlFlow::Return(v) => {
                        // Early return - propagate immediately
                        return Ok(ControlFlow::Return(v));
                    }
                }
            }
            Ok(ControlFlow::Value(result))
        } else {
            // No else branch - return nil
            Ok(ControlFlow::Value(Value::nil()))
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluator_implemented() {
        // GREEN: Verify basic implementation works
        let mut eval = Evaluator::new();
        let node = AstNode::IntegerLiteral(42);
        let result = eval.eval(&node).unwrap();
        assert_eq!(result.as_integer().unwrap(), 42);
    }
}
