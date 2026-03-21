// INTERP-005: Function Calls & Recursion
// INTERP-032: Mock concurrency primitives (GREEN phase)
// INTERP-036: Grouped import evaluation (GREEN phase)
// INTERP-037: Dereference operator evaluation (GREEN phase)
// INTERP-038: Compound assignment operators (GREEN phase)
// INTERP-039: vec! macro evaluation (GREEN phase)
// INTERP-040: Tuple destructuring evaluation (GREEN phase)
// DEBUGGER-041: Stack Depth Profiler (GREEN phase)
// BUG-041: Fixed stack overflow (MAX_CALL_DEPTH 150→30)
// REFACTOR Phase: Optimize and document
//
// Research:
// - Aho et al. (2006) Chapter 8: Expression Evaluation
// - Aho et al. (2006) Chapter 7: Runtime Environments
// - Savage et al. (1997) Eraser: Data Race Detection
//
// This module implements expression evaluation and function calls for the Ruchy interpreter.
// Supports function definition, function calls with recursion, control flow, variable scoping,
// mock concurrency primitives, import handling, dereference operations, compound assignment,
// and vec! macro.
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
//
// Mock Concurrency Support (INTERP-032):
// - Built-in functions: thread::spawn, Arc::new, Mutex::new, mpsc::channel
// - Mock methods: lock(), unwrap(), join(), push(), send(), recv()
// - Note: These are simplified stubs for testing, not actual threading
//
// Import Handling (INTERP-036):
// - UseDecl: Single imports like `use std::thread;`
// - GroupedUseDecl: Grouped imports like `use std::sync::{Arc, Mutex};`
// - Both are currently no-ops (no module system yet)
//
// Dereference Operator (INTERP-037):
// - Unary dereference (*expr) extracts values from mock wrappers
// - For HashMap with "_inner" key: returns the inner value
// - For other values: returns value unchanged (identity operation)
// - Enables Arc<Mutex<T>> pattern: *counter.lock().unwrap()
//
// Compound Assignment Operators (INTERP-038):
// - Compound assignments: +=, -=, *=, /=, %=
// - Desugared to: lhs = lhs op rhs
// - Simple form: x += 5 becomes x = x + 5
// - With dereference: *num += 1 updates _inner field in wrapper HashMap
// - Unblocks INTERP-032 concurrency tests requiring *num += 1 pattern
//
// vec! Macro (INTERP-039):
// - vec![] creates empty vector
// - vec![1, 2, 3] creates vector with elements
// - vec![0; 10] creates vector with repeated element
// - Evaluates to Value::Vector
// - Array methods: .push() mutates array, .len() returns length
//
// Tuple Destructuring (INTERP-040):
// - let (a, b) = tuple extracts elements and binds to variables
// - Evaluates RHS, checks tuple type and arity
// - Binds each element to corresponding pattern variable
// - Unblocks test_channel_communication: let (tx, rx) = mpsc::channel()
// - Note: Nested patterns not yet supported
//
// Stack Depth Profiler (DEBUGGER-041):
// - Optional profiling for debugging and performance analysis
// - Tracks max call depth, total calls, per-function call counts
// - Records deepest call stack for recursion analysis
// - Enable via with_profiling() builder method
// - Extract data via get_profiling_data() or take_profiling_data()
// - Zero overhead when disabled (Option<ProfilingData>)
// - Use for: detecting recursion patterns, finding hotspots, performance tuning
//
// Bug Fix (BUG-041):
// - Reduced MAX_CALL_DEPTH from 150 to 30
// - Prevents Rust stack overflow in test threads (2MB stack limit)
// - Ensures interpreter catches overflow before Rust runtime crashes

use crate::interpreter::parser::AstNode;
use crate::interpreter::scope::Scope;
use crate::interpreter::value::{Value, ValueError};
use std::collections::HashMap;
use std::fmt;

/// Maximum recursion depth before stack overflow
/// This is set conservatively to prevent actual Rust stack overflow
/// Test threads have 2MB stack vs 8MB main thread, so this must be low enough
/// to prevent Rust stack overflow before interpreter can catch it.
/// Testing shows: depth 50 crashes, depth 40 crashes on infinite recursion,
/// depth 30 works for both finite and infinite recursion.
pub(crate) const MAX_CALL_DEPTH: usize = 30;

/// Profiling data for stack depth analysis (DEBUGGER-041)
///
/// Tracks function call statistics during interpreter execution.
/// Used for debugging recursion, performance analysis, and hotspot identification.
///
/// # Example
/// ```rust
/// use ruchyruchy::interpreter::evaluator::Evaluator;
/// use ruchyruchy::interpreter::parser::Parser;
///
/// let code = r#"
///     fun factorial(n) {
///         if (n <= 1) { return 1; }
///         return n * factorial(n - 1);
///     }
///     factorial(5);
/// "#;
///
/// let mut parser = Parser::new(code);
/// let ast = parser.parse().unwrap();
/// let mut eval = Evaluator::new().with_profiling();
///
/// for statement in ast.nodes() {
///     eval.eval(statement).unwrap();
/// }
///
/// let profile = eval.get_profiling_data().unwrap();
/// assert_eq!(profile.max_depth, 5);
/// assert_eq!(profile.total_calls, 5);
/// assert_eq!(profile.call_counts.get("factorial"), Some(&5));
/// ```
#[derive(Debug, Clone)]
pub struct ProfilingData {
    /// Maximum call depth reached during execution
    pub max_depth: usize,
    /// Total function calls executed
    pub total_calls: usize,
    /// Call counts per function: function name -> count
    pub call_counts: HashMap<String, usize>,
    /// Call stack at maximum depth (innermost call last)
    pub deepest_stack: Vec<String>,
}

impl ProfilingData {
    fn new() -> Self {
        Self {
            max_depth: 0,
            total_calls: 0,
            call_counts: HashMap::new(),
            deepest_stack: Vec::new(),
        }
    }
}

/// Evaluator executes AST nodes and produces values
#[derive(Debug, Clone)]
pub struct Evaluator {
    /// Scope for variable lookups
    pub(crate) scope: Scope,
    /// Function registry: name -> (params, body)
    pub(crate) functions: HashMap<String, (Vec<String>, Vec<AstNode>)>,
    /// Current call depth for stack overflow detection
    pub(crate) call_depth: usize,
    /// Call stack for error reporting (tracks function call chain)
    pub(crate) call_stack: Vec<String>,
    /// Optional profiling data (DEBUGGER-041: Stack Depth Profiler)
    pub(crate) profiling: Option<ProfilingData>,
    /// Optional performance profiler (DEBUGGER-047: Performance Profiler)
    pub(crate) performance_profiler: Option<crate::debugger::PerformanceProfiler>,
    /// Arc store for shared references (INTERP-041: Fix Arc mock concurrency)
    /// Maps arc_id -> shared value for proper Arc::clone semantics
    pub(crate) arc_store: HashMap<usize, Value>,
    /// Next available arc ID
    pub(crate) next_arc_id: usize,
    /// Optional compiler profiler (DEBUGGER-052: Type Observation)
    pub(crate) compiler_profiler: Option<crate::profiler::CompilerProfiler>,
}

/// Internal control flow for handling early returns
///
/// When evaluating function bodies, we need to distinguish between:
/// - Normal evaluation (last expression value)
/// - Early return (explicit return statement)
///
/// This enum allows return statements to propagate up through nested
/// control structures (if/else, loops) without executing remaining statements.
pub(crate) enum ControlFlow {
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
    UndefinedVariable {
        /// Variable name
        name: String,
    },
    /// Undefined function
    UndefinedFunction {
        /// Function name
        name: String,
    },
    /// Argument count mismatch in function call
    ArgumentCountMismatch {
        /// Function name
        function: String,
        /// Expected argument count
        expected: usize,
        /// Actual argument count
        actual: usize,
    },
    /// Stack overflow from excessive recursion
    StackOverflow,
    /// No match arm matched in match expression
    NoMatchArm,
    /// Unsupported operation
    UnsupportedOperation {
        /// Operation description
        operation: String,
    },
    /// Error with call stack information for debugging
    ///
    /// Wraps another error and attaches the function call stack at the point
    /// where the error occurred. The call stack is ordered from outermost to
    /// innermost function (most recent call is last in the vector).
    ///
    /// This variant is automatically added when an error occurs during function
    /// execution, providing context about which functions were active when the
    /// error happened.
    WithCallStack {
        /// Wrapped error
        error: Box<EvalError>,
        /// Function call stack (outermost to innermost)
        call_stack: Vec<String>,
    },
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
            EvalError::WithCallStack { error, call_stack } => {
                write!(f, "{}\nCall stack (most recent call first):\n", error)?;
                // Display stack in reverse order: innermost (most recent) call first
                for (i, func_name) in call_stack.iter().rev().enumerate() {
                    writeln!(f, "  {}. {}", i + 1, func_name)?;
                }
                Ok(())
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
            call_stack: Vec::new(),
            profiling: None,
            performance_profiler: None,
            arc_store: HashMap::new(),
            next_arc_id: 0,
            compiler_profiler: None,
        }
    }

    /// DEBUGGER-046: Deep clone for time-travel debugging
    ///
    /// Creates an independent deep copy of this evaluator including deep-copying
    /// the scope tree. Unlike the derived Clone (which shallow-copies the Scope),
    /// this creates completely independent state for snapshots.
    ///
    /// Used by the REPL debugger to create snapshots that won't be affected by
    /// future modifications during program execution.
    pub fn deep_clone(&self) -> Self {
        Evaluator {
            scope: self.scope.deep_clone(),
            functions: self.functions.clone(),
            call_depth: self.call_depth,
            call_stack: self.call_stack.clone(),
            profiling: self.profiling.clone(),
            performance_profiler: self.performance_profiler.clone(),
            arc_store: self.arc_store.clone(),
            next_arc_id: self.next_arc_id,
            compiler_profiler: self.compiler_profiler.clone(),
        }
    }

    /// DEBUGGER-047: Enable performance profiling
    ///
    /// Attaches a performance profiler to track parse/eval timing and identify bottlenecks
    pub fn with_profiler(mut self, profiler: crate::debugger::PerformanceProfiler) -> Self {
        self.performance_profiler = Some(profiler);
        self
    }

    /// DEBUGGER-052: Enable type observation (Julia-inspired)
    ///
    /// Attaches a compiler profiler to observe type signatures at function calls.
    /// Used for type stability analysis and optimization opportunity detection.
    pub fn with_type_observation(mut self, profiler: &crate::profiler::CompilerProfiler) -> Self {
        self.compiler_profiler = Some(profiler.clone());
        self
    }

    /// Enable profiling for stack depth analysis (DEBUGGER-041)
    ///
    /// Enables collection of function call statistics including:
    /// - Maximum call depth reached
    /// - Total function calls executed
    /// - Per-function call counts
    /// - Call stack at maximum depth
    ///
    /// Profiling has minimal overhead (HashMap operations per function call).
    /// Use for debugging recursion patterns and performance analysis.
    ///
    /// # Example
    /// ```rust
    /// use ruchyruchy::interpreter::evaluator::Evaluator;
    ///
    /// let mut eval = Evaluator::new().with_profiling();
    /// // ... execute code ...
    /// let profile = eval.get_profiling_data().unwrap();
    /// println!("Max depth: {}", profile.max_depth);
    /// ```
    pub fn with_profiling(mut self) -> Self {
        self.profiling = Some(ProfilingData::new());
        self
    }

    /// Get profiling data (if profiling was enabled)
    ///
    /// Returns a reference to the profiling data collected during execution.
    /// Returns `None` if profiling was not enabled via `with_profiling()`.
    ///
    /// Use this method if you need to inspect profiling data multiple times.
    /// For consuming the data, use `take_profiling_data()` instead.
    pub fn get_profiling_data(&self) -> Option<&ProfilingData> {
        self.profiling.as_ref()
    }

    /// Take profiling data (consumes the profiling data)
    ///
    /// Extracts and returns the profiling data, leaving `None` in its place.
    /// Returns `None` if profiling was not enabled via `with_profiling()`.
    ///
    /// Use this method to extract profiling data for processing or reporting.
    /// After calling this method, subsequent calls will return `None`.
    pub fn take_profiling_data(&mut self) -> Option<ProfilingData> {
        self.profiling.take()
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
        // DEBUGGER-047: Track overall eval timing (clone once per statement, not per expression)
        let profiler_opt = self.performance_profiler.clone();
        if let Some(profiler) = profiler_opt {
            profiler.start_eval();
            let result = match self.eval_internal(node)? {
                ControlFlow::Value(v) => Ok(v),
                ControlFlow::Return(v) => Ok(v),
            };
            profiler.end_eval();
            result
        } else {
            match self.eval_internal(node)? {
                ControlFlow::Value(v) => Ok(v),
                ControlFlow::Return(v) => Ok(v),
            }
        }
    }

    /// Evaluate a complete program (all nodes in an AST)
    ///
    /// Convenience method for property testing (DEBUGGER-044).
    /// Evaluates all nodes in the AST and returns the value of the last expression.
    ///
    /// # Example
    /// ```
    /// use ruchyruchy::interpreter::{Parser, Evaluator};
    ///
    /// let mut parser = Parser::new("1 + 2");
    /// let ast = parser.parse().unwrap();
    /// let mut eval = Evaluator::new();
    /// let result = eval.eval_program(&ast).unwrap();
    /// ```
    pub fn eval_program(&mut self, ast: &crate::interpreter::Ast) -> Result<Value, EvalError> {
        let mut last_value = Value::Nil;

        for node in ast.nodes() {
            last_value = self.eval(node)?;
        }

        Ok(last_value)
    }

    /// Get variable value from current scope (DEBUGGER-046: REPL Debugger)
    ///
    /// Used by the interactive debugger to inspect variable values.
    /// Looks up the variable in the current scope (including parent scopes).
    ///
    /// # Example
    /// ```
    /// use ruchyruchy::interpreter::{Parser, Evaluator};
    ///
    /// let code = "let x = 42;";
    /// let mut parser = Parser::new(code);
    /// let ast = parser.parse().unwrap();
    /// let mut eval = Evaluator::new();
    ///
    /// for statement in ast.nodes() {
    ///     eval.eval(statement).unwrap();
    /// }
    ///
    /// let x_value = eval.get_variable("x");
    /// assert!(x_value.is_some());
    /// ```
    pub fn get_variable(&self, name: &str) -> Option<Value> {
        self.scope.get_cloned(name).ok()
    }

    /// Get current call stack (DEBUGGER-046: REPL Debugger)
    ///
    /// Returns the current function call stack for debugging purposes.
    /// The call stack is maintained during function calls and cleared on return.
    ///
    /// # Example
    /// ```
    /// use ruchyruchy::interpreter::{Parser, Evaluator};
    ///
    /// let code = r#"
    ///     fun factorial(n) {
    ///         if (n <= 1) { return 1; }
    ///         return n * factorial(n - 1);
    ///     }
    ///     factorial(3);
    /// "#;
    /// let mut parser = Parser::new(code);
    /// let ast = parser.parse().unwrap();
    /// let mut eval = Evaluator::new();
    ///
    /// for statement in ast.nodes() {
    ///     eval.eval(statement).unwrap();
    /// }
    ///
    /// // Call stack is empty after function returns
    /// let stack = eval.get_call_stack();
    /// assert!(stack.is_empty());
    /// ```
    pub fn get_call_stack(&self) -> &[String] {
        &self.call_stack
    }

    /// Internal evaluation with control flow support
    pub(crate) fn eval_internal(&mut self, node: &AstNode) -> Result<ControlFlow, EvalError> {
        match node {
            // Literals - direct conversion to values
            AstNode::IntegerLiteral(n) => Ok(ControlFlow::Value(Value::integer(*n))),
            AstNode::FloatLiteral(f) => Ok(ControlFlow::Value(Value::float(*f))),
            AstNode::StringLiteral(s) => Ok(ControlFlow::Value(Value::string(s.clone()))),
            AstNode::CharLiteral(c) => Ok(ControlFlow::Value(Value::string(c.to_string()))),
            AstNode::BooleanLiteral(b) => Ok(ControlFlow::Value(Value::boolean(*b))),

            // F-string with interpolation
            AstNode::FString { content } => self.eval_fstring(content),

            // Binary operations
            AstNode::BinaryOp { op, left, right } => {
                let left_val = self.eval(left)?;
                let right_val = self.eval(right)?;
                let result = self.eval_binary_op(*op, left_val, right_val)?;
                Ok(ControlFlow::Value(result))
            }

            // Unary operations
            AstNode::UnaryOp { op, operand } => {
                let operand_val = self.eval(operand)?;
                let result = self.eval_unary_op(*op, operand_val)?;
                Ok(ControlFlow::Value(result))
            }

            // Type cast
            AstNode::TypeCast { expr, target_type } => {
                let value = self.eval(expr)?;
                let result = self.eval_type_cast(value, target_type)?;
                Ok(ControlFlow::Value(result))
            }

            // Range expression: start..end
            AstNode::Range { start, end } => self.eval_range(start, end),

            // Function definition - register function
            AstNode::FunctionDef { name, params, body } => {
                self.functions
                    .insert(name.clone(), (params.clone(), body.clone()));
                Ok(ControlFlow::Value(Value::nil()))
            }

            // Function call
            AstNode::FunctionCall { name, args } => {
                let result = self.call_function(name, args)?;
                Ok(ControlFlow::Value(result))
            }

            // Method call (mutating + immutable dispatch)
            AstNode::MethodCall {
                receiver,
                method,
                args,
            } => self.eval_method_call(receiver, method, args),

            // Identifier - lookup variable in scope
            AstNode::Identifier(name) => {
                let value = self
                    .scope
                    .get_cloned(name)
                    .map_err(|_| EvalError::UndefinedVariable { name: name.clone() })?;
                Ok(ControlFlow::Value(value))
            }

            // Return statement
            AstNode::Return { value } => {
                let return_val = if let Some(expr) = value {
                    self.eval(expr)?
                } else {
                    Value::nil()
                };
                Ok(ControlFlow::Return(return_val))
            }

            // If expression
            AstNode::IfExpr {
                condition,
                then_branch,
                else_branch,
            } => self.eval_if(condition, then_branch, else_branch),

            // Block expression (INTERP-043)
            AstNode::Block { statements } => self.eval_block(statements),

            // Let declaration
            AstNode::LetDecl { name, value } => {
                let val = self.eval(value)?;
                self.scope.define(name.clone(), val).map_err(|e| {
                    EvalError::UnsupportedOperation {
                        operation: format!("define variable: {}", e),
                    }
                })?;
                Ok(ControlFlow::Value(Value::nil()))
            }

            // Tuple destructuring: let (a, b, c) = expr
            AstNode::TupleDestruct { names, value } => self.eval_tuple_destruct(names, value),

            // While loop
            AstNode::WhileLoop { condition, body } => self.eval_while(condition, body),

            // For loop
            AstNode::ForLoop {
                var,
                iterable,
                body,
            } => self.eval_for(var, iterable, body),

            // Match expression
            AstNode::MatchExpr { expr, arms } => self.eval_match(expr, arms),

            // Assignment
            AstNode::Assignment { name, value } => {
                let val = self.eval(value)?;
                self.scope
                    .assign(name, val)
                    .map_err(|_| EvalError::UndefinedVariable { name: name.clone() })?;
                Ok(ControlFlow::Value(Value::nil()))
            }

            // Compound assignment: x += 5, *num -= 1
            AstNode::CompoundAssignment { lhs, op, rhs } => {
                self.eval_compound_assignment(lhs, *op, rhs)
            }

            // Vector literal
            AstNode::VectorLiteral { elements } => {
                let mut values = Vec::new();
                for elem in elements {
                    values.push(self.eval(elem)?);
                }
                Ok(ControlFlow::Value(Value::vector(values)))
            }

            // HashMap literal
            AstNode::HashMapLiteral { pairs } => self.eval_hashmap_literal(pairs),

            // Tuple literal
            AstNode::TupleLiteral { elements } => {
                let mut values = Vec::new();
                for elem in elements {
                    values.push(self.eval(elem)?);
                }
                Ok(ControlFlow::Value(Value::tuple(values)))
            }

            // Index access: vec[i], map[key]
            AstNode::IndexAccess { expr, index } => self.eval_index_access(expr, index),

            // Use declaration - no-op (no module system yet)
            AstNode::UseDecl { path: _ } => Ok(ControlFlow::Value(Value::nil())),

            // Grouped use declaration - no-op (no module system yet)
            AstNode::GroupedUseDecl {
                base_path: _,
                items: _,
            } => Ok(ControlFlow::Value(Value::nil())),

            // Path expression: Arc::new, thread::spawn
            AstNode::PathExpr { segments } => {
                let name = segments.join("::");
                if let Ok(value) = self.scope.get_cloned(&name) {
                    Ok(ControlFlow::Value(value))
                } else {
                    Ok(ControlFlow::Value(Value::string(format!(
                        "<path: {}>",
                        name
                    ))))
                }
            }

            // Closure - capture environment
            AstNode::Closure {
                is_move: _,
                params,
                body,
            } => {
                let captured_env = self.scope.capture();
                Ok(ControlFlow::Value(Value::Closure {
                    params: params.clone(),
                    body: body.clone(),
                    captured_env,
                }))
            }

            // Struct definition - no-op
            AstNode::StructDef { .. } => Ok(ControlFlow::Value(Value::nil())),

            // Struct literal - create as HashMap
            AstNode::StructLiteral { name: _, fields } => self.eval_struct_literal(fields),

            // Field access
            AstNode::FieldAccess { expr, field } => self.eval_field_access(expr, field),

            // vec! macro
            AstNode::VecMacro {
                elements,
                repeat_count,
            } => self.eval_vec_macro(elements, repeat_count.as_deref()),

            // Empty node - no-op
            AstNode::Empty => Ok(ControlFlow::Value(Value::nil())),
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
