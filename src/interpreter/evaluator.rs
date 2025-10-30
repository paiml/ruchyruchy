// INTERP-004: Expression Evaluator
// GREEN Phase: Minimal implementation to pass all tests
//
// Research: Aho et al. (2006) Chapter 8: Expression Evaluation
//
// This module implements expression evaluation for the Ruchy interpreter.
// Handles arithmetic, comparison, and logical operations with proper precedence.

use crate::interpreter::parser::{AstNode, BinaryOperator, UnaryOperator};
use crate::interpreter::scope::Scope;
use crate::interpreter::value::{Value, ValueError};
use std::fmt;

/// Evaluator executes AST nodes and produces values
pub struct Evaluator {
    /// Scope for variable lookups (will be used in later phases)
    _scope: Scope,
}

/// Evaluation errors
#[derive(Debug, Clone)]
pub enum EvalError {
    /// Value error (type mismatch, division by zero, etc.)
    ValueError(ValueError),
    /// Undefined variable
    UndefinedVariable { name: String },
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
            _scope: Scope::new(),
        }
    }

    /// Evaluate an AST node and return a value
    pub fn eval(&mut self, node: &AstNode) -> Result<Value, EvalError> {
        match node {
            // Literals
            AstNode::IntegerLiteral(n) => Ok(Value::integer(*n)),
            AstNode::StringLiteral(s) => Ok(Value::string(s.clone())),
            AstNode::BooleanLiteral(b) => Ok(Value::boolean(*b)),

            // Binary operations
            AstNode::BinaryOp { op, left, right } => {
                let left_val = self.eval(left)?;
                let right_val = self.eval(right)?;
                self.eval_binary_op(*op, left_val, right_val)
            }

            // Unary operations
            AstNode::UnaryOp { op, operand } => {
                let operand_val = self.eval(operand)?;
                self.eval_unary_op(*op, operand_val)
            }

            _ => Err(EvalError::UnsupportedOperation {
                operation: format!("{:?}", node),
            }),
        }
    }

    /// Evaluate a binary operation
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
