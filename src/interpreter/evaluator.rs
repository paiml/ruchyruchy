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
    /// Call stack for error reporting (tracks function call chain)
    call_stack: Vec<String>,
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
        error: Box<EvalError>,
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
                    write!(f, "  {}. {}\n", i + 1, func_name)?;
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
            AstNode::FloatLiteral(f) => Ok(ControlFlow::Value(Value::float(*f))),
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

        // 2. Check for built-in functions first (read_file, write_file, println, etc.)
        //
        // Built-ins have priority over user-defined functions. This ensures that core
        // functionality like I/O is always available and cannot be shadowed by users.
        // If try_call_builtin returns Some(value), we found and executed a built-in.
        // If it returns None, we fall through to check user-defined functions below.
        if let Some(result) = self.try_call_builtin(name, args)? {
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
        let saved_scope = std::mem::replace(&mut self.scope, Scope::new());
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
                // String length: "hello".len() => 5
                if arg_values.len() != 0 {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: format!("{}.len()", receiver.type_name()),
                        expected: 0,
                        actual: arg_values.len(),
                    });
                }

                if let Ok(s) = receiver.as_string() {
                    Ok(Value::integer(s.len() as i64))
                } else {
                    Err(EvalError::UnsupportedOperation {
                        operation: format!("method 'len' not supported on type {}", receiver.type_name()),
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
            _ => Err(EvalError::UnsupportedOperation {
                operation: format!("unknown method '{}' on type {}", method, receiver.type_name()),
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
                // println(msg: String) -> nil
                if args.len() != 1 {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "println".to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }

                let msg_val = self.eval(&args[0])?;
                let msg = msg_val.as_string()?;

                println!("{}", msg);
                Ok(Some(Value::nil()))
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
