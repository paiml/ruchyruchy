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

use crate::interpreter::parser::{AstNode, Parser, UnaryOperator};
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

                        // Mutate array or string
                        match current_val {
                            Value::Vector(ref mut arr) => {
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
                            }
                            Value::String(ref mut s) => {
                                // String::push(char) - append character to string
                                let char_str = arg_val.as_string().map_err(|_| {
                                    EvalError::UnsupportedOperation {
                                        operation: "push() on String requires char argument"
                                            .to_string(),
                                    }
                                })?;
                                s.push_str(char_str);

                                // Update scope with mutated string
                                self.scope.assign(var_name, current_val).map_err(|_| {
                                    EvalError::UndefinedVariable {
                                        name: var_name.clone(),
                                    }
                                })?;
                                return Ok(ControlFlow::Value(Value::nil()));
                            }
                            _ => {
                                return Err(EvalError::UnsupportedOperation {
                                    operation: format!(
                                        "push() requires array or String, got {}",
                                        current_val.type_name()
                                    ),
                                });
                            }
                        }
                    }
                }

                // Special handling for push_str() - appends string to string
                if method == "push_str" {
                    if let AstNode::Identifier(var_name) = receiver.as_ref() {
                        // Get current string
                        let mut current_val = self.scope.get_cloned(var_name).map_err(|_| {
                            EvalError::UndefinedVariable {
                                name: var_name.clone(),
                            }
                        })?;

                        // Evaluate argument
                        if args.len() != 1 {
                            return Err(EvalError::ArgumentCountMismatch {
                                function: "push_str".to_string(),
                                expected: 1,
                                actual: args.len(),
                            });
                        }
                        let arg_val = self.eval(&args[0])?;

                        // Mutate string
                        if let Value::String(ref mut s) = current_val {
                            let str_arg = arg_val.as_string().map_err(|_| {
                                EvalError::UnsupportedOperation {
                                    operation: "push_str() requires String argument".to_string(),
                                }
                            })?;
                            s.push_str(str_arg);

                            // Update scope with mutated string
                            self.scope.assign(var_name, current_val).map_err(|_| {
                                EvalError::UndefinedVariable {
                                    name: var_name.clone(),
                                }
                            })?;
                            return Ok(ControlFlow::Value(Value::nil()));
                        } else {
                            return Err(EvalError::UnsupportedOperation {
                                operation: format!(
                                    "push_str() requires String, got {}",
                                    current_val.type_name()
                                ),
                            });
                        }
                    }
                }

                // Special handling for pop() - it mutates the array
                if method == "pop" {
                    if let AstNode::Identifier(var_name) = receiver.as_ref() {
                        // Get current array
                        let mut current_val = self.scope.get_cloned(var_name).map_err(|_| {
                            EvalError::UndefinedVariable {
                                name: var_name.clone(),
                            }
                        })?;

                        // pop() takes no arguments
                        if !args.is_empty() {
                            return Err(EvalError::ArgumentCountMismatch {
                                function: "pop".to_string(),
                                expected: 0,
                                actual: args.len(),
                            });
                        }

                        // Mutate array
                        if let Value::Vector(ref mut arr) = current_val {
                            let popped = arr.pop().unwrap_or(Value::nil());

                            // Update scope with mutated array
                            self.scope.assign(var_name, current_val).map_err(|_| {
                                EvalError::UndefinedVariable {
                                    name: var_name.clone(),
                                }
                            })?;
                            return Ok(ControlFlow::Value(popped));
                        } else {
                            return Err(EvalError::UnsupportedOperation {
                                operation: format!(
                                    "pop() requires array, got {}",
                                    current_val.type_name()
                                ),
                            });
                        }
                    }
                }

                // Special handling for push_str() - it mutates the string
                if method == "push_str" {
                    if let AstNode::Identifier(var_name) = receiver.as_ref() {
                        // Get current string
                        let mut current_val = self.scope.get_cloned(var_name).map_err(|_| {
                            EvalError::UndefinedVariable {
                                name: var_name.clone(),
                            }
                        })?;

                        // push_str() takes exactly one argument (string to append)
                        if args.len() != 1 {
                            return Err(EvalError::ArgumentCountMismatch {
                                function: "push_str".to_string(),
                                expected: 1,
                                actual: args.len(),
                            });
                        }

                        // Evaluate the argument
                        let arg_val = self.eval(&args[0])?;
                        let to_append = arg_val.as_string()?;

                        // Mutate string
                        if let Value::String(ref mut s) = current_val {
                            s.push_str(to_append);

                            // Update scope with mutated string
                            self.scope.assign(var_name, current_val).map_err(|_| {
                                EvalError::UndefinedVariable {
                                    name: var_name.clone(),
                                }
                            })?;
                            return Ok(ControlFlow::Value(Value::nil()));
                        } else {
                            return Err(EvalError::UnsupportedOperation {
                                operation: format!(
                                    "push_str() requires string, got {}",
                                    current_val.type_name()
                                ),
                            });
                        }
                    }
                }

                // Special handling for insert() - it mutates the HashMap
                if method == "insert" {
                    if let AstNode::Identifier(var_name) = receiver.as_ref() {
                        // Get current HashMap
                        let mut current_val = self.scope.get_cloned(var_name).map_err(|_| {
                            EvalError::UndefinedVariable {
                                name: var_name.clone(),
                            }
                        })?;

                        // insert() takes exactly two arguments (key, value)
                        if args.len() != 2 {
                            return Err(EvalError::ArgumentCountMismatch {
                                function: "insert".to_string(),
                                expected: 2,
                                actual: args.len(),
                            });
                        }

                        // Evaluate the arguments
                        let key_val = self.eval(&args[0])?;
                        let key = key_val.as_string()?;
                        let value_val = self.eval(&args[1])?;

                        // Mutate HashMap
                        if let Value::HashMap(ref mut map) = current_val {
                            map.insert(key.to_string(), value_val);

                            // Update scope with mutated HashMap
                            self.scope.assign(var_name, current_val).map_err(|_| {
                                EvalError::UndefinedVariable {
                                    name: var_name.clone(),
                                }
                            })?;
                            return Ok(ControlFlow::Value(Value::nil()));
                        } else {
                            return Err(EvalError::UnsupportedOperation {
                                operation: format!(
                                    "insert() requires HashMap, got {}",
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

            // Closure - create closure value with captured environment
            AstNode::Closure {
                is_move: _, // TODO: Implement move semantics in future
                params,
                body,
            } => {
                // Capture current environment for closure
                let captured_env = self.scope.capture();

                // Return closure value with captured environment
                Ok(ControlFlow::Value(Value::Closure {
                    params: params.clone(),
                    body: body.clone(),
                    captured_env,
                }))
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
