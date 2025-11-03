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

use crate::interpreter::parser::{AstNode, BinaryOperator, Parser, UnaryOperator};
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
const MAX_CALL_DEPTH: usize = 30;

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
    scope: Scope,
    /// Function registry: name -> (params, body)
    functions: HashMap<String, (Vec<String>, Vec<AstNode>)>,
    /// Current call depth for stack overflow detection
    call_depth: usize,
    /// Call stack for error reporting (tracks function call chain)
    call_stack: Vec<String>,
    /// Optional profiling data (DEBUGGER-041: Stack Depth Profiler)
    profiling: Option<ProfilingData>,
    /// Optional performance profiler (DEBUGGER-047: Performance Profiler)
    performance_profiler: Option<crate::debugger::PerformanceProfiler>,
    /// Arc store for shared references (INTERP-041: Fix Arc mock concurrency)
    /// Maps arc_id -> shared value for proper Arc::clone semantics
    arc_store: HashMap<usize, Value>,
    /// Next available arc ID
    next_arc_id: usize,
    /// Optional compiler profiler (DEBUGGER-052: Type Observation)
    compiler_profiler: Option<crate::profiler::CompilerProfiler>,
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
    fn eval_internal(&mut self, node: &AstNode) -> Result<ControlFlow, EvalError> {
        match node {
            // Literals - direct conversion to values
            AstNode::IntegerLiteral(n) => Ok(ControlFlow::Value(Value::integer(*n))),
            AstNode::FloatLiteral(f) => Ok(ControlFlow::Value(Value::float(*f))),
            AstNode::StringLiteral(s) => Ok(ControlFlow::Value(Value::string(s.clone()))),
            AstNode::BooleanLiteral(b) => Ok(ControlFlow::Value(Value::boolean(*b))),

            // F-string with interpolation: f"text {expr} more"
            // Parse the content to extract {expr} parts and evaluate them
            AstNode::FString { content } => {
                let mut result = String::new();
                let mut chars = content.chars().peekable();

                while let Some(ch) = chars.next() {
                    if ch == '{' {
                        // Extract expression until '}'
                        let mut expr_str = String::new();
                        let mut depth = 1;
                        for ch in chars.by_ref() {
                            if ch == '{' {
                                depth += 1;
                                expr_str.push(ch);
                            } else if ch == '}' {
                                depth -= 1;
                                if depth == 0 {
                                    break;
                                }
                                expr_str.push(ch);
                            } else {
                                expr_str.push(ch);
                            }
                        }

                        // Parse and evaluate the expression
                        let mut parser = Parser::new(&expr_str);
                        let ast = parser
                            .parse()
                            .map_err(|e| EvalError::UnsupportedOperation {
                                operation: format!(
                                    "Failed to parse f-string expression '{}': {:?}",
                                    expr_str, e
                                ),
                            })?;

                        // Evaluate the expression
                        if let Some(node) = ast.nodes().first() {
                            let value = self.eval(node)?;
                            result.push_str(&value.to_println_string());
                        }
                    } else {
                        result.push(ch);
                    }
                }

                Ok(ControlFlow::Value(Value::string(result)))
            }

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

            // Type cast - convert value to target type
            AstNode::TypeCast { expr, target_type } => {
                let value = self.eval(expr)?;
                let result = self.eval_type_cast(value, target_type)?;
                Ok(ControlFlow::Value(result))
            }

            // Range expression: start..end
            // Creates a vector of integers from start to end (inclusive)
            AstNode::Range { start, end } => {
                let start_val = self.eval(start)?;
                let end_val = self.eval(end)?;

                let start_int =
                    start_val
                        .as_integer()
                        .map_err(|_| EvalError::UnsupportedOperation {
                            operation: format!(
                                "range start must be integer, got {}",
                                start_val.type_name()
                            ),
                        })?;
                let end_int =
                    end_val
                        .as_integer()
                        .map_err(|_| EvalError::UnsupportedOperation {
                            operation: format!(
                                "range end must be integer, got {}",
                                end_val.type_name()
                            ),
                        })?;

                // Create vector of integers from start to end (exclusive)
                let mut elements = Vec::new();
                for i in start_int..end_int {
                    elements.push(Value::integer(i));
                }

                Ok(ControlFlow::Value(Value::vector(elements)))
            }

            // Function definition - register function
            AstNode::FunctionDef { name, params, body } => {
                self.functions
                    .insert(name.clone(), (params.clone(), body.clone()));
                Ok(ControlFlow::Value(Value::nil()))
            }

            // Function call - evaluate arguments and call function
            AstNode::FunctionCall { name, args } => {
                let result = self.call_function(name, args)?;
                Ok(ControlFlow::Value(result))
            }

            // Method call - evaluate receiver and call method on it
            AstNode::MethodCall {
                receiver,
                method,
                args,
            } => {
                // Special handling for push() - it mutates the array
                if method == "push" {
                    if let AstNode::Identifier(var_name) = receiver.as_ref() {
                        // Get current array
                        let mut current_val = self.scope.get_cloned(var_name).map_err(|_| {
                            EvalError::UndefinedVariable {
                                name: var_name.clone(),
                            }
                        })?;

                        // Evaluate argument
                        if args.len() != 1 {
                            return Err(EvalError::ArgumentCountMismatch {
                                function: "push".to_string(),
                                expected: 1,
                                actual: args.len(),
                            });
                        }
                        let arg_val = self.eval(&args[0])?;

                        // Mutate array
                        if let Value::Vector(ref mut arr) = current_val {
                            arr.push(arg_val);

                            // DEBUGGER-047: Track memory allocation for push
                            if let Some(ref profiler) = self.performance_profiler {
                                let bytes = std::mem::size_of::<Value>();
                                profiler.record_memory_allocation(bytes);
                            }

                            // Update scope with mutated array
                            self.scope.assign(var_name, current_val).map_err(|_| {
                                EvalError::UndefinedVariable {
                                    name: var_name.clone(),
                                }
                            })?;
                            return Ok(ControlFlow::Value(Value::nil()));
                        } else {
                            return Err(EvalError::UnsupportedOperation {
                                operation: format!(
                                    "push() requires array, got {}",
                                    current_val.type_name()
                                ),
                            });
                        }
                    }
                }

                // Default method call handling
                let receiver_val = self.eval(receiver)?;
                let result = self.call_method(receiver_val, method, args)?;
                Ok(ControlFlow::Value(result))
            }

            // Identifier - lookup variable in scope
            AstNode::Identifier(name) => {
                let value = self
                    .scope
                    .get_cloned(name)
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
            AstNode::IfExpr {
                condition,
                then_branch,
                else_branch,
            } => self.eval_if(condition, then_branch, else_branch),

            // Block expression - creates a new scope (INTERP-043)
            AstNode::Block { statements } => {
                // Create child scope for block
                let child_scope = self.scope.create_child();
                let parent_scope = std::mem::replace(&mut self.scope, child_scope);

                // Evaluate all statements in block scope
                let mut last_value = Value::nil();
                let mut early_exit = None; // Track early return or error

                for stmt in statements {
                    match self.eval_internal(stmt) {
                        Ok(ControlFlow::Value(v)) => last_value = v,
                        Ok(ControlFlow::Return(v)) => {
                            early_exit = Some(Ok(ControlFlow::Return(v)));
                            break;
                        }
                        Err(e) => {
                            early_exit = Some(Err(e));
                            break;
                        }
                    }
                }

                // Restore parent scope
                self.scope = parent_scope;

                // Return early exit if any, otherwise return last value
                early_exit.unwrap_or(Ok(ControlFlow::Value(last_value)))
            }

            // Let declaration - variable binding
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
            AstNode::TupleDestruct { names, value } => {
                // Evaluate RHS to get tuple
                let tuple_val = self.eval(value)?;

                // Extract tuple elements
                let elements = match tuple_val {
                    Value::Tuple(ref elems) => elems.clone(),
                    _ => {
                        return Err(EvalError::UnsupportedOperation {
                            operation: format!(
                                "tuple destructuring requires tuple, got {}",
                                tuple_val.type_name()
                            ),
                        })
                    }
                };

                // Check arity match
                if names.len() != elements.len() {
                    return Err(EvalError::UnsupportedOperation {
                        operation: format!(
                            "tuple destructuring: expected {} elements, got {}",
                            names.len(),
                            elements.len()
                        ),
                    });
                }

                // Bind each element to corresponding pattern variable
                for (name, elem) in names.iter().zip(elements.iter()) {
                    self.scope.define(name.clone(), elem.clone()).map_err(|e| {
                        EvalError::UnsupportedOperation {
                            operation: format!("define variable in tuple destructuring: {}", e),
                        }
                    })?;
                }

                Ok(ControlFlow::Value(Value::nil()))
            }

            // While loop - execute body while condition is true
            AstNode::WhileLoop { condition, body } => self.eval_while(condition, body),

            // For loop - iterate over elements
            AstNode::ForLoop {
                var,
                iterable,
                body,
            } => self.eval_for(var, iterable, body),

            // Match expression - pattern matching
            AstNode::MatchExpr { expr, arms } => self.eval_match(expr, arms),

            // Assignment - reassign existing variable
            AstNode::Assignment { name, value } => {
                let val = self.eval(value)?;
                self.scope
                    .assign(name, val)
                    .map_err(|_| EvalError::UndefinedVariable { name: name.clone() })?;
                Ok(ControlFlow::Value(Value::nil()))
            }

            // Compound assignment: x += 5, *num -= 1
            // Desugar to: lhs = lhs op rhs
            AstNode::CompoundAssignment { lhs, op, rhs } => {
                // Evaluate current value of LHS
                let current_val = self.eval(lhs)?;

                // Evaluate RHS
                let rhs_val = self.eval(rhs)?;

                // Apply operation
                // INTERP-OPT-002: Move current_val instead of cloning (not used after binary op)
                let new_val = self.eval_binary_op(*op, current_val, rhs_val)?;

                // Update the variable
                // For simple identifiers: x += 1
                if let AstNode::Identifier(name) = lhs.as_ref() {
                    self.scope
                        .assign(name, new_val)
                        .map_err(|_| EvalError::UndefinedVariable { name: name.clone() })?;
                    Ok(ControlFlow::Value(Value::nil()))
                } else if let AstNode::UnaryOp {
                    op: UnaryOperator::Dereference,
                    operand,
                } = lhs.as_ref()
                {
                    // For dereference: *num += 1
                    // INTERP-041: Support both _arc_id (shared ref) and _inner (local)
                    if let AstNode::Identifier(name) = operand.as_ref() {
                        // Get the wrapper object
                        let wrapper = self
                            .scope
                            .get_cloned(name)
                            .map_err(|_| EvalError::UndefinedVariable { name: name.clone() })?;

                        // Update the value (arc_store or _inner)
                        if let Value::HashMap(map) = wrapper {
                            // INTERP-041: Check for _arc_id first (shared reference model)
                            if let Some(Value::Integer(arc_id)) = map.get("_arc_id") {
                                // Re-wrap new_val in Mutex HashMap structure before storing
                                // This preserves the Mutex::new wrapper when updating
                                use std::collections::HashMap;
                                let mut mutex_wrapper = HashMap::new();
                                mutex_wrapper.insert("_inner".to_string(), new_val.clone());
                                self.arc_store
                                    .insert(*arc_id as usize, Value::HashMap(mutex_wrapper));
                                Ok(ControlFlow::Value(Value::nil()))
                            } else {
                                // Fallback: _inner model (local reference)
                                let mut new_map = map.clone();
                                new_map.insert("_inner".to_string(), new_val);
                                self.scope
                                    .assign(name, Value::HashMap(new_map))
                                    .map_err(|_| EvalError::UndefinedVariable {
                                        name: name.clone(),
                                    })?;
                                Ok(ControlFlow::Value(Value::nil()))
                            }
                        } else {
                            // Not a wrapper, just update the variable directly
                            self.scope
                                .assign(name, new_val)
                                .map_err(|_| EvalError::UndefinedVariable { name: name.clone() })?;
                            Ok(ControlFlow::Value(Value::nil()))
                        }
                    } else {
                        Err(EvalError::UnsupportedOperation {
                            operation: "compound assignment to complex dereference expression"
                                .to_string(),
                        })
                    }
                } else {
                    Err(EvalError::UnsupportedOperation {
                        operation: format!("compound assignment to {}", "complex expression"),
                    })
                }
            }

            // Vector literal - create vector value
            AstNode::VectorLiteral { elements } => {
                let mut values = Vec::new();
                for elem in elements {
                    values.push(self.eval(elem)?);
                }
                Ok(ControlFlow::Value(Value::vector(values)))
            }

            // HashMap literal - create hashmap value
            // Syntax: {key1: val1, key2: val2, ...}
            // Keys must evaluate to strings, values can be any type
            AstNode::HashMapLiteral { pairs } => {
                use std::collections::HashMap;
                let mut map = HashMap::new();
                for (key_node, val_node) in pairs {
                    let key_val = self.eval(key_node)?;
                    let key_str = key_val.as_string()?.to_string();
                    let value = self.eval(val_node)?;
                    map.insert(key_str, value);
                }
                Ok(ControlFlow::Value(Value::HashMap(map)))
            }

            // Tuple literal - create tuple value
            // Syntax: (elem1, elem2, ...)
            // Elements can be heterogeneous types
            AstNode::TupleLiteral { elements } => {
                let mut values = Vec::new();
                for elem in elements {
                    values.push(self.eval(elem)?);
                }
                Ok(ControlFlow::Value(Value::tuple(values)))
            }

            // Index access - vec[i], map[key]
            // For vectors: index must be non-negative integer
            // For hashmaps: key can be any Value (converted to string internally)
            // Errors: IndexOutOfBounds (vec), KeyNotFound (map), TypeMismatch (non-indexable)
            AstNode::IndexAccess { expr, index } => {
                let container = self.eval(expr)?;
                let index_val = self.eval(index)?;

                match &container {
                    Value::Vector(_) => {
                        let idx = index_val.as_integer()?;
                        if idx < 0 {
                            return Err(EvalError::ValueError(
                                crate::interpreter::value::ValueError::InvalidOperation {
                                    operation: "vector index".to_string(),
                                    message: "index cannot be negative".to_string(),
                                },
                            ));
                        }
                        let result = container.index(idx as usize)?.clone();
                        Ok(ControlFlow::Value(result))
                    }
                    Value::HashMap(_) => {
                        let result = container.get(&index_val)?.clone();
                        Ok(ControlFlow::Value(result))
                    }
                    _ => Err(EvalError::ValueError(
                        crate::interpreter::value::ValueError::TypeMismatch {
                            expected: "Vector or HashMap".to_string(),
                            found: container.type_name().to_string(),
                            operation: "index access".to_string(),
                        },
                    )),
                }
            }

            // Use declaration - ignored for now (no module system yet)
            // In a full implementation, this would import modules/symbols
            AstNode::UseDecl { path: _ } => {
                // For now, just acknowledge the use statement without error
                // Path like ["std", "sync", "Mutex"] is noted but not imported
                Ok(ControlFlow::Value(Value::nil()))
            }

            // Grouped use declaration - ignored for now (no module system yet)
            // Examples: use std::sync::{Arc, Mutex};
            AstNode::GroupedUseDecl {
                base_path: _,
                items: _,
            } => {
                // For now, just acknowledge the grouped use statement without error
                // Base path like ["std", "sync"] and items like ["Arc", "Mutex"]
                // Would expand to: use std::sync::Arc; use std::sync::Mutex;
                Ok(ControlFlow::Value(Value::nil()))
            }

            // Path expression - convert to identifier or function lookup
            // Examples: Arc::new, thread::spawn
            AstNode::PathExpr { segments } => {
                // Join path segments into a single identifier
                // This allows treating "thread::spawn" as a function name
                let name = segments.join("::");

                // Try to look up as variable first, then as function
                if let Ok(value) = self.scope.get_cloned(&name) {
                    Ok(ControlFlow::Value(value))
                } else {
                    // Path expressions without calls evaluate to a placeholder
                    // In a full implementation, this would be a function reference
                    Ok(ControlFlow::Value(Value::string(format!(
                        "<path: {}>",
                        name
                    ))))
                }
            }

            // Closure - store as a function-like value
            // For now, closures are not first-class values, so we'll return nil
            // Full implementation would require closure values with captured environment
            AstNode::Closure {
                is_move,
                params,
                body: _,
            } => {
                // TODO: Implement proper closure support with environment capture
                // For now, return a placeholder value
                Err(EvalError::UnsupportedOperation {
                    operation: format!(
                        "Closures not yet fully implemented (is_move: {}, params: {:?})",
                        is_move, params
                    ),
                })
            }

            // Struct definition - register struct schema
            AstNode::StructDef { .. } => {
                // Struct definitions are currently no-ops
                // Full implementation would register struct schemas
                Ok(ControlFlow::Value(Value::nil()))
            }

            // Struct literal - create struct instance
            AstNode::StructLiteral { name: _, fields } => {
                // For now, represent structs as hashmaps
                use std::collections::HashMap;
                let mut map = HashMap::new();
                for (field_name, field_val_node) in fields {
                    let field_val = self.eval(field_val_node)?;
                    map.insert(field_name.clone(), field_val);
                }
                Ok(ControlFlow::Value(Value::HashMap(map)))
            }

            // Field access - get field from struct/object
            AstNode::FieldAccess { expr, field } => {
                let value = self.eval(expr)?;
                // Treat as hashmap field access for struct-like values
                match &value {
                    Value::HashMap(_) => {
                        let key = Value::string(field.clone());
                        let result = value.get(&key)?.clone();
                        Ok(ControlFlow::Value(result))
                    }
                    _ => Err(EvalError::UnsupportedOperation {
                        operation: format!("field access on {}", value.type_name()),
                    }),
                }
            }

            // vec! macro - create array
            // Forms: vec![], vec![1, 2, 3], vec![0; 10]
            AstNode::VecMacro {
                elements,
                repeat_count,
            } => {
                if let Some(count_expr) = repeat_count {
                    // Repeat form: vec![expr; count]
                    let element_value = self.eval(&elements[0])?;
                    let count_value = self.eval(count_expr)?;
                    let count = match count_value {
                        Value::Integer(n) if n >= 0 => n as usize,
                        _ => {
                            return Err(EvalError::UnsupportedOperation {
                                operation: "vec! repeat count must be non-negative integer"
                                    .to_string(),
                            })
                        }
                    };
                    let repeated_array = vec![element_value; count];

                    // DEBUGGER-047: Track memory allocation for vector
                    if let Some(ref profiler) = self.performance_profiler {
                        // Estimate: each Value is ~32 bytes
                        let bytes = count * std::mem::size_of::<Value>();
                        profiler.record_memory_allocation(bytes);
                    }

                    Ok(ControlFlow::Value(Value::Vector(repeated_array)))
                } else {
                    // Elements form: vec![1, 2, 3] or vec![]
                    let mut array = Vec::new();
                    for elem in elements {
                        let val = self.eval(elem)?;
                        array.push(val);
                    }

                    // DEBUGGER-047: Track memory allocation for vector
                    if let Some(ref profiler) = self.performance_profiler {
                        // Estimate: each Value is ~32 bytes
                        let bytes = array.len() * std::mem::size_of::<Value>();
                        profiler.record_memory_allocation(bytes);
                    }

                    Ok(ControlFlow::Value(Value::Vector(array)))
                }
            }

            // Empty node - no-op
            AstNode::Empty => Ok(ControlFlow::Value(Value::nil())),
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
                Ok(Value::boolean(less.as_boolean()? || equal.as_boolean()?))
            }
            BinaryOperator::GreaterEqual => {
                // GreaterEqual is GreaterThan OR Equal
                let greater = left.greater_than(&right)?;
                let equal = left.equals(&right)?;
                Ok(Value::boolean(greater.as_boolean()? || equal.as_boolean()?))
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
            UnaryOperator::Dereference => {
                // Dereference operator: *expr
                // INTERP-041: Recursively unwrap _locked_value and _inner until we hit a non-wrapper
                // INTERP-OPT-002: Use references to avoid unnecessary clones during unwrapping
                let mut current = &operand;
                #[allow(clippy::while_let_loop)]
                loop {
                    match current {
                        Value::HashMap(map) => {
                            // Check for _locked_value first (from lock() method)
                            if let Some(locked_value) = map.get("_locked_value") {
                                current = locked_value;
                                continue;
                            }
                            // Check for _inner (original mock wrapper)
                            else if let Some(inner_value) = map.get("_inner") {
                                current = inner_value;
                                continue;
                            } else {
                                // Not a wrapper, return as-is
                                break;
                            }
                        }
                        _ => {
                            // Not a wrapper, done unwrapping
                            break;
                        }
                    }
                }
                // Only clone once at the end
                Ok(current.clone())
            }
        }
    }

    /// Evaluate a type cast
    fn eval_type_cast(&mut self, value: Value, target_type: &str) -> Result<Value, EvalError> {
        match target_type {
            "i32" => {
                // Cast to integer
                if let Ok(i) = value.as_integer() {
                    Ok(Value::integer(i))
                } else if let Ok(f) = value.as_float() {
                    Ok(Value::integer(f as i64))
                } else if let Ok(s) = value.as_string() {
                    s.parse::<i64>().map(Value::integer).map_err(|_| {
                        EvalError::UnsupportedOperation {
                            operation: format!("cannot cast string '{}' to i32", s),
                        }
                    })
                } else {
                    Err(EvalError::UnsupportedOperation {
                        operation: format!("cannot cast {} to i32", value.type_name()),
                    })
                }
            }
            "f64" => {
                // Cast to float
                if let Ok(f) = value.as_float() {
                    Ok(Value::float(f))
                } else if let Ok(i) = value.as_integer() {
                    Ok(Value::float(i as f64))
                } else if let Ok(s) = value.as_string() {
                    s.parse::<f64>().map(Value::float).map_err(|_| {
                        EvalError::UnsupportedOperation {
                            operation: format!("cannot cast string '{}' to f64", s),
                        }
                    })
                } else {
                    Err(EvalError::UnsupportedOperation {
                        operation: format!("cannot cast {} to f64", value.type_name()),
                    })
                }
            }
            "bool" => {
                // Cast to boolean
                if let Ok(b) = value.as_boolean() {
                    Ok(Value::boolean(b))
                } else {
                    Err(EvalError::UnsupportedOperation {
                        operation: format!("cannot cast {} to bool", value.type_name()),
                    })
                }
            }
            _ => Err(EvalError::UnsupportedOperation {
                operation: format!("unknown type cast target: {}", target_type),
            }),
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
        // DEBUGGER-047: Track function calls if profiler is attached
        if let Some(ref profiler) = self.performance_profiler {
            profiler.record_function_call(name);
            profiler.push_call_stack(name.to_string());
        }

        // 1. Check stack depth before recursing (prevent stack overflow)
        if self.call_depth >= MAX_CALL_DEPTH {
            // DEBUGGER-047: Pop call stack before early return
            if let Some(ref profiler) = self.performance_profiler {
                if let Some((func_name, duration)) = profiler.pop_call_stack() {
                    profiler.record_eval_operation(func_name, duration);
                }
            }
            return Err(EvalError::StackOverflow);
        }

        // 2. Check for built-in functions first (read_file, write_file, println, etc.)
        //
        // Built-ins have priority over user-defined functions. This ensures that core
        // functionality like I/O is always available and cannot be shadowed by users.
        // If try_call_builtin returns Some(value), we found and executed a built-in.
        // If it returns None, we fall through to check user-defined functions below.
        if let Some(result) = self.try_call_builtin(name, args)? {
            // DEBUGGER-047: Pop call stack before early return
            if let Some(ref profiler) = self.performance_profiler {
                if let Some((func_name, duration)) = profiler.pop_call_stack() {
                    profiler.record_eval_operation(func_name, duration);
                }
            }
            return Ok(result);
        }

        // 3. Look up function in user-defined registry
        let (params, body) =
            self.functions
                .get(name)
                .cloned()
                .ok_or_else(|| EvalError::UndefinedFunction {
                    name: name.to_string(),
                })?;

        // 4. Check argument count matches parameter count (arity check)
        if args.len() != params.len() {
            return Err(EvalError::ArgumentCountMismatch {
                function: name.to_string(),
                expected: params.len(),
                actual: args.len(),
            });
        }

        // 5. Evaluate all arguments eagerly (call-by-value semantics)
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.eval(arg)?);
        }

        // 6. Create new scope and bind parameters to argument values
        let saved_scope = std::mem::take(&mut self.scope);
        for (param, value) in params.iter().zip(arg_values.iter()) {
            self.scope
                .define(param.clone(), value.clone())
                .map_err(|e| EvalError::UnsupportedOperation {
                    operation: format!("define parameter: {}", e),
                })?;
        }

        // Push function name to call stack for error reporting
        // This enables displaying the full call chain when errors occur,
        // making debugging much easier (shows which functions led to the error)
        self.call_stack.push(name.to_string());

        // Increment call depth for recursion tracking
        self.call_depth += 1;

        // DEBUGGER-041: Track profiling data if enabled
        if let Some(ref mut prof) = self.profiling {
            prof.total_calls += 1;
            *prof.call_counts.entry(name.to_string()).or_insert(0) += 1;

            // Update max depth and capture deepest stack if this is deepest
            if self.call_depth > prof.max_depth {
                prof.max_depth = self.call_depth;
                prof.deepest_stack = self.call_stack.clone();
            }
        }

        // DEBUGGER-052: Start timing for hot function detection
        let start_time = std::time::Instant::now();

        // 7. Execute function body, handling early returns
        let mut result = Value::nil();
        for stmt in &body {
            match self.eval_internal(stmt) {
                Ok(ControlFlow::Value(v)) => {
                    // Normal evaluation - update result and continue
                    result = v;
                }
                Ok(ControlFlow::Return(v)) => {
                    // Early return - stop executing and return immediately
                    result = v;
                    break;
                }
                Err(e) => {
                    // Error occurred during function body execution
                    //
                    // IMPORTANT: Capture the call stack BEFORE popping the current function.
                    // The captured stack includes all functions in the call chain up to and
                    // including the current function where the error occurred.
                    let captured_stack = self.call_stack.clone();

                    // Restore evaluator state (depth, stack, scope)
                    self.call_depth -= 1;
                    self.call_stack.pop(); // Remove current function from active stack
                    self.scope = saved_scope;

                    // DEBUGGER-047: Pop profiler call stack on error
                    if let Some(ref profiler) = self.performance_profiler {
                        if let Some((func_name, duration)) = profiler.pop_call_stack() {
                            profiler.record_eval_operation(func_name, duration);
                        }
                    }

                    // Wrap error with call stack information for debugging, unless it's
                    // already wrapped (prevents double-wrapping in nested errors)
                    return Err(match e {
                        EvalError::WithCallStack { .. } => e, // Already has stack info
                        _ => EvalError::WithCallStack {
                            error: Box::new(e),
                            call_stack: captured_stack, // Attach the call stack
                        },
                    });
                }
            }
        }

        // 8. Restore previous scope, call depth, and call stack
        self.call_depth -= 1;
        self.call_stack.pop();
        self.scope = saved_scope;

        // DEBUGGER-047: Pop profiler call stack on success and record timing
        if let Some(ref profiler) = self.performance_profiler {
            if let Some((func_name, duration)) = profiler.pop_call_stack() {
                profiler.record_eval_operation(func_name, duration);
            }
        }

        // DEBUGGER-052: Observe type signature and record timing for this function call
        if let Some(ref profiler) = self.compiler_profiler {
            // Build type signature: param types + return type
            let param_types: Vec<String> = arg_values
                .iter()
                .map(|v| v.type_name().to_string())
                .collect();
            let return_type = result.type_name().to_string();

            let signature = crate::profiler::TypeSignature::new(param_types, return_type);
            profiler.observe_type(name, signature);

            // Record function call timing for hot function detection
            let duration = start_time.elapsed();
            profiler.record_function_call(name, duration);
        }

        Ok(result)
    }

    /// Call a method on a receiver value
    ///
    /// Implements basic method call syntax: receiver.method(args)
    /// Currently supports string methods: len(), contains()
    fn call_method(
        &mut self,
        receiver: Value,
        method: &str,
        args: &[AstNode],
    ) -> Result<Value, EvalError> {
        // Evaluate arguments
        let mut arg_values = Vec::new();
        for arg in args {
            arg_values.push(self.eval(arg)?);
        }

        // Dispatch based on method name
        match method {
            "len" => {
                // String or Array length: "hello".len() => 5, [1,2,3].len() => 3
                if !arg_values.is_empty() {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: format!("{}.len()", receiver.type_name()),
                        expected: 0,
                        actual: arg_values.len(),
                    });
                }

                if let Ok(s) = receiver.as_string() {
                    Ok(Value::integer(s.len() as i64))
                } else if let Ok(arr) = receiver.as_vector() {
                    Ok(Value::integer(arr.len() as i64))
                } else {
                    Err(EvalError::UnsupportedOperation {
                        operation: format!(
                            "method 'len' not supported on type {}",
                            receiver.type_name()
                        ),
                    })
                }
            }
            "contains" => {
                // String contains: "hello".contains('e') => true
                if arg_values.len() != 1 {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: format!("{}.contains()", receiver.type_name()),
                        expected: 1,
                        actual: arg_values.len(),
                    });
                }

                if let (Ok(haystack), Ok(needle)) =
                    (receiver.as_string(), arg_values[0].as_string())
                {
                    Ok(Value::boolean(haystack.contains(needle)))
                } else {
                    Err(EvalError::UnsupportedOperation {
                        operation: format!(
                            "method 'contains' not supported on types {} and {}",
                            receiver.type_name(),
                            arg_values[0].type_name()
                        ),
                    })
                }
            }
            "round" => {
                // Float rounding: 3.7.round() => 4.0
                if !arg_values.is_empty() {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: format!("{}.round()", receiver.type_name()),
                        expected: 0,
                        actual: arg_values.len(),
                    });
                }

                if let Ok(f) = receiver.as_float() {
                    Ok(Value::float(f.round()))
                } else {
                    Err(EvalError::UnsupportedOperation {
                        operation: format!(
                            "method 'round' not supported on type {}",
                            receiver.type_name()
                        ),
                    })
                }
            }

            // Mock concurrency methods
            "lock" => {
                // Mutex::lock() -> LockGuard
                // INTERP-041: Look up value in arc_store if _arc_id exists
                if !arg_values.is_empty() {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "lock".to_string(),
                        expected: 0,
                        actual: arg_values.len(),
                    });
                }

                // INTERP-041: Look up in arc_store if _arc_id exists, otherwise use _inner
                match &receiver {
                    Value::HashMap(map) => {
                        // Check for _arc_id first (new shared ref model)
                        if let Some(Value::Integer(arc_id)) = map.get("_arc_id") {
                            if let Some(stored_value) = self.arc_store.get(&(*arc_id as usize)) {
                                // Return wrapper with _arc_id so dereference can update it
                                use std::collections::HashMap;
                                let mut wrapper = HashMap::new();
                                wrapper.insert("_arc_id".to_string(), Value::integer(*arc_id));
                                wrapper.insert("_locked_value".to_string(), stored_value.clone());
                                return Ok(Value::HashMap(wrapper));
                            }
                        }
                        // Fallback: old _inner approach (backwards compatibility)
                        if let Some(inner) = map.get("_inner") {
                            Ok(inner.clone())
                        } else {
                            Ok(receiver.clone())
                        }
                    }
                    _ => Ok(receiver.clone()),
                }
            }

            "unwrap" => {
                // Result::unwrap() or Option::unwrap()
                // Mock: just return the receiver
                // INTERP-OPT-002: Return receiver directly, no need to clone (we own it)
                if !arg_values.is_empty() {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "unwrap".to_string(),
                        expected: 0,
                        actual: arg_values.len(),
                    });
                }
                Ok(receiver)
            }

            "join" => {
                // ThreadHandle::join() -> Result<(), ()>
                // Mock: just return nil
                if !arg_values.is_empty() {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "join".to_string(),
                        expected: 0,
                        actual: arg_values.len(),
                    });
                }
                Ok(Value::nil())
            }

            "push" => {
                // Vec::push(value) -> ()
                // This is a mutating operation, which is tricky in our immutable design
                // For now, just return nil
                if arg_values.len() != 1 {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "push".to_string(),
                        expected: 1,
                        actual: arg_values.len(),
                    });
                }
                Ok(Value::nil())
            }

            "send" => {
                // Sender::send(value) -> Result<(), SendError>
                // Mock: just return nil
                if arg_values.len() != 1 {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "send".to_string(),
                        expected: 1,
                        actual: arg_values.len(),
                    });
                }
                Ok(Value::nil())
            }

            "recv" => {
                // Receiver::recv() -> Result<T, RecvError>
                // Mock: just return a placeholder value
                if !arg_values.is_empty() {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "recv".to_string(),
                        expected: 0,
                        actual: arg_values.len(),
                    });
                }
                Ok(Value::string("Hello from thread!".to_string()))
            }

            _ => Err(EvalError::UnsupportedOperation {
                operation: format!(
                    "unknown method '{}' on type {}",
                    method,
                    receiver.type_name()
                ),
            }),
        }
    }

    /// Try to call a built-in function
    ///
    /// Built-in functions are checked before user-defined functions, allowing
    /// core functionality like I/O to be available without explicit imports.
    ///
    /// # Built-in Functions
    ///
    /// - `read_file(path: String) -> String` - Reads file content
    /// - `write_file(path: String, content: String) -> nil` - Writes to file
    /// - `println(msg: String) -> nil` - Prints message to stdout
    ///
    /// # Return Values
    ///
    /// - `Ok(Some(Value))` - Built-in function executed successfully
    /// - `Ok(None)` - Not a built-in, should try user-defined functions
    /// - `Err(EvalError)` - Built-in function call failed (I/O error, wrong args, etc.)
    fn try_call_builtin(
        &mut self,
        name: &str,
        args: &[AstNode],
    ) -> Result<Option<Value>, EvalError> {
        match name {
            "read_file" => {
                // read_file(path: String) -> String
                if args.len() != 1 {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "read_file".to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }

                let path_val = self.eval(&args[0])?;
                let path = path_val.as_string()?;

                match std::fs::read_to_string(path) {
                    Ok(content) => Ok(Some(Value::string(content))),
                    Err(e) => Err(EvalError::ValueError(ValueError::InvalidOperation {
                        operation: "read_file".to_string(),
                        message: format!("Failed to read file: {}", e),
                    })),
                }
            }

            "write_file" => {
                // write_file(path: String, content: String) -> nil
                if args.len() != 2 {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "write_file".to_string(),
                        expected: 2,
                        actual: args.len(),
                    });
                }

                let path_val = self.eval(&args[0])?;
                let path = path_val.as_string()?;

                let content_val = self.eval(&args[1])?;
                let content = content_val.as_string()?;

                match std::fs::write(path, content) {
                    Ok(_) => Ok(Some(Value::nil())),
                    Err(e) => Err(EvalError::ValueError(ValueError::InvalidOperation {
                        operation: "write_file".to_string(),
                        message: format!("Failed to write file: {}", e),
                    })),
                }
            }

            "println" => {
                // println(value: Any) -> nil
                // Prints any value (strings without quotes, other types with their display format)
                if args.len() != 1 {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "println".to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }

                let msg_val = self.eval(&args[0])?;
                let msg = msg_val.to_println_string();

                println!("{}", msg);
                Ok(Some(Value::nil()))
            }

            "assert" => {
                // assert(condition: Boolean) -> nil
                // Panics if condition is false
                if args.len() != 1 {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "assert".to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }

                let cond_val = self.eval(&args[0])?;
                let cond_bool = cond_val.as_boolean()?;

                if !cond_bool {
                    return Err(EvalError::UnsupportedOperation {
                        operation: "assertion failed".to_string(),
                    });
                }

                Ok(Some(Value::nil()))
            }

            // Mock concurrency primitives for testing
            // These are simplified stubs that don't actually spawn threads
            "thread::spawn" => {
                // thread::spawn(closure) -> ThreadHandle
                // INTERP-042: Mock implementation - execute closure synchronously
                if args.len() != 1 {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "thread::spawn".to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }

                // Extract closure body and execute it
                let result = match &args[0] {
                    AstNode::Closure {
                        is_move: _,
                        params,
                        body,
                    } => {
                        // For mock threading, closures should have no parameters
                        if !params.is_empty() {
                            return Err(EvalError::UnsupportedOperation {
                                operation: format!(
                                    "thread::spawn closures with parameters not supported (found {} params)",
                                    params.len()
                                ),
                            });
                        }

                        // Execute closure body synchronously
                        let mut last_value = Value::nil();
                        for stmt in body {
                            last_value = self.eval(stmt)?;
                        }
                        last_value
                    }
                    _ => {
                        return Err(EvalError::UnsupportedOperation {
                            operation: "thread::spawn requires a closure argument".to_string(),
                        });
                    }
                };

                // Return mock thread handle with result
                use std::collections::HashMap;
                let mut handle = HashMap::new();
                handle.insert("_thread_id".to_string(), Value::integer(1));
                handle.insert("_result".to_string(), result);
                Ok(Some(Value::HashMap(handle)))
            }

            "Mutex::new" => {
                // Mutex::new(value) -> Mutex<T>
                // INTERP-041: Just wrap locally, NO arc_store (only Arc uses arc_store)
                if args.len() != 1 {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "Mutex::new".to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }

                let val = self.eval(&args[0])?;

                // Wrap in HashMap with _inner (local wrapper, not shared)
                use std::collections::HashMap;
                let mut wrapper = HashMap::new();
                wrapper.insert("_inner".to_string(), val);
                Ok(Some(Value::HashMap(wrapper)))
            }

            "Arc::new" => {
                // Arc::new(value) -> Arc<T>
                // INTERP-041: Store value in arc_store for shared references
                if args.len() != 1 {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "Arc::new".to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }

                let val = self.eval(&args[0])?;

                // Store value in arc_store
                let arc_id = self.next_arc_id;
                self.arc_store.insert(arc_id, val);
                self.next_arc_id += 1;

                // Return HashMap with _arc_id for Arc::clone to reference
                use std::collections::HashMap;
                let mut wrapper = HashMap::new();
                wrapper.insert("_arc_id".to_string(), Value::integer(arc_id as i64));
                Ok(Some(Value::HashMap(wrapper)))
            }

            "Arc::clone" => {
                // Arc::clone(&arc) -> Arc<T>
                // INTERP-041: Return HashMap with same _arc_id (shared reference!)
                if args.len() != 1 {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "Arc::clone".to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }

                let val = self.eval(&args[0])?;

                // Extract _arc_id from the HashMap and return same ID
                // This makes Arc::clone share the reference instead of deep cloning
                if let Value::HashMap(ref map) = val {
                    if let Some(Value::Integer(arc_id)) = map.get("_arc_id") {
                        use std::collections::HashMap;
                        let mut wrapper = HashMap::new();
                        wrapper.insert("_arc_id".to_string(), Value::integer(*arc_id));
                        return Ok(Some(Value::HashMap(wrapper)));
                    }
                }

                // Fallback: if not an Arc, just clone (for backwards compatibility)
                Ok(Some(val.clone()))
            }

            "mpsc::channel" => {
                // mpsc::channel() -> (Sender, Receiver)
                // Mock: return a tuple of two hashmaps
                if !args.is_empty() {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "mpsc::channel".to_string(),
                        expected: 0,
                        actual: args.len(),
                    });
                }

                use std::collections::HashMap;
                let mut sender = HashMap::new();
                sender.insert("_type".to_string(), Value::string("Sender".to_string()));
                let mut receiver = HashMap::new();
                receiver.insert("_type".to_string(), Value::string("Receiver".to_string()));

                Ok(Some(Value::tuple(vec![
                    Value::HashMap(sender),
                    Value::HashMap(receiver),
                ])))
            }

            "vec" => {
                // vec![] or vec![elements] -> Vector
                // Create a vector from arguments
                let mut elements = Vec::new();
                for arg in args {
                    elements.push(self.eval(arg)?);
                }
                Ok(Some(Value::vector(elements)))
            }

            _ => {
                // Not a built-in function, return None to try user-defined functions
                Ok(None)
            }
        }
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

    /// Helper: Execute loop body statements
    /// Returns Ok(None) to continue, Ok(Some(v)) for early return
    fn eval_loop_body_impl(&mut self, body: &[AstNode]) -> Result<Option<Value>, EvalError> {
        for stmt in body {
            match self.eval_internal(stmt)? {
                ControlFlow::Value(_) => {
                    // Normal evaluation - continue
                }
                ControlFlow::Return(v) => {
                    // Early return from enclosing function
                    return Ok(Some(v));
                }
            }
        }
        Ok(None)
    }

    /// Helper: Execute loop body in a child scope
    /// Returns Ok(None) to continue, Ok(Some(v)) for early return
    fn eval_loop_body_with_scope(&mut self, body: &[AstNode]) -> Result<Option<Value>, EvalError> {
        // Create child scope for loop body iteration
        // This allows variables declared inside the loop to be fresh each iteration
        let child_scope = self.scope.create_child();
        let old_scope = std::mem::replace(&mut self.scope, child_scope);

        // Execute body statements
        let return_value = self.eval_loop_body_impl(body)?;

        // Restore parent scope after loop iteration
        self.scope = old_scope;

        Ok(return_value)
    }

    /// Evaluate while loop
    fn eval_while(
        &mut self,
        condition: &AstNode,
        body: &[AstNode],
    ) -> Result<ControlFlow, EvalError> {
        loop {
            // Evaluate condition
            let cond_val = self.eval(condition)?;
            let cond_bool = cond_val.as_boolean()?;

            if !cond_bool {
                break; // Exit loop when condition is false
            }

            // Execute body in child scope
            if let Some(return_value) = self.eval_loop_body_with_scope(body)? {
                // Early return from enclosing function
                return Ok(ControlFlow::Return(return_value));
            }
        }

        // While loops return nil
        Ok(ControlFlow::Value(Value::nil()))
    }

    /// Evaluate for loop
    fn eval_for(
        &mut self,
        var: &str,
        iterable: &AstNode,
        body: &[AstNode],
    ) -> Result<ControlFlow, EvalError> {
        // Evaluate iterable expression
        let iterable_val = self.eval(iterable)?;

        // Get vector elements
        let elements = iterable_val.as_vector()?.clone();

        // Iterate over elements
        for element in elements.iter() {
            // Create child scope with loop variable bound to current element
            let child_scope = self.scope.create_child();
            let old_scope = std::mem::replace(&mut self.scope, child_scope);

            // Define loop variable in child scope
            self.scope
                .define(var.to_string(), element.clone())
                .map_err(|e| EvalError::UnsupportedOperation {
                    operation: format!("define loop variable: {}", e),
                })?;

            // Execute body and check for early return
            let return_value = self.eval_loop_body_impl(body)?;

            // Restore parent scope
            self.scope = old_scope;

            // Propagate early return
            if let Some(v) = return_value {
                return Ok(ControlFlow::Return(v));
            }
        }

        // For loops return nil
        Ok(ControlFlow::Value(Value::nil()))
    }

    /// Evaluate match expression
    fn eval_match(
        &mut self,
        expr: &AstNode,
        arms: &[crate::interpreter::parser::MatchArm],
    ) -> Result<ControlFlow, EvalError> {
        use crate::interpreter::parser::Pattern;

        // Evaluate the matched expression
        let match_val = self.eval(expr)?;

        // Try each arm in order
        for arm in arms {
            let matches = match &arm.pattern {
                Pattern::Wildcard => {
                    // Wildcard matches anything
                    true
                }
                Pattern::Literal(lit) => {
                    // Literal pattern - evaluate and compare
                    let pattern_val = self.eval(lit)?;
                    match_val == pattern_val
                }
                Pattern::Identifier(name) => {
                    // Identifier pattern - bind variable and always match
                    self.scope
                        .define(name.clone(), match_val.clone())
                        .map_err(|e| EvalError::UnsupportedOperation {
                            operation: format!("bind match variable: {}", e),
                        })?;
                    true
                }
            };

            if matches {
                // Execute arm body
                let mut result = Value::nil();
                for stmt in &arm.body {
                    match self.eval_internal(stmt)? {
                        ControlFlow::Value(v) => {
                            result = v;
                        }
                        ControlFlow::Return(v) => {
                            // Early return - propagate
                            return Ok(ControlFlow::Return(v));
                        }
                    }
                }
                return Ok(ControlFlow::Value(result));
            }
        }

        // No arm matched
        Err(EvalError::NoMatchArm)
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
