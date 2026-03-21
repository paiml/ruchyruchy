// Evaluator helper methods: binary ops, unary ops, type casting,
// function/closure/method calls, built-in functions, and control flow.
//
// Extracted from evaluator.rs for file-health compliance (<2000 lines).

use crate::interpreter::parser::{AstNode, BinaryOperator, UnaryOperator};
use crate::interpreter::value::{Value, ValueError};
use super::evaluator::{ControlFlow, EvalError, Evaluator, MAX_CALL_DEPTH};

impl Evaluator {
    /// Evaluate a binary operation
    ///
    /// Applies a binary operator to two values. Operations are grouped into:
    /// - Arithmetic: +, -, *, /, %
    /// - Comparison: <, >, ==, !=, <=, >=
    /// - Logical: &&, ||
    ///
    /// Type checking is performed by Value methods.
    pub(crate) fn eval_binary_op(
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
    pub(crate) fn eval_modulo(&self, left: Value, right: Value) -> Result<Value, EvalError> {
        let left_int = left.as_integer()?;
        let right_int = right.as_integer()?;

        if right_int == 0 {
            return Err(EvalError::ValueError(ValueError::DivisionByZero));
        }

        Ok(Value::integer(left_int % right_int))
    }

    /// Evaluate a unary operation
    pub(crate) fn eval_unary_op(&self, op: UnaryOperator, operand: Value) -> Result<Value, EvalError> {
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
    pub(crate) fn eval_type_cast(&mut self, value: Value, target_type: &str) -> Result<Value, EvalError> {
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
    pub(crate) fn call_function(&mut self, name: &str, args: &[AstNode]) -> Result<Value, EvalError> {
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

        // 3. Check if name refers to a closure variable (before checking function registry)
        if let Ok(Value::Closure {
            params,
            body,
            captured_env,
        }) = self.scope.get_cloned(name)
        {
            // Call the closure with its captured environment
            return self.call_closure(&params, &body, &captured_env, args);
        }

        // 4. Look up function in user-defined registry
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
        // INTERP-046: Pre-allocate capacity for arguments
        let mut arg_values = Vec::with_capacity(args.len());
        for arg in args {
            arg_values.push(self.eval(arg)?);
        }

        // INTERP-046: Collect param types ONLY if profiler is enabled (avoid overhead)
        let param_types_for_profiling: Option<Vec<String>> = if self.compiler_profiler.is_some() {
            Some(
                arg_values
                    .iter()
                    .map(|v| v.type_name().to_string())
                    .collect(),
            )
        } else {
            None
        };

        // 6. Create new scope and bind parameters to argument values
        let saved_scope = std::mem::take(&mut self.scope);
        // INTERP-046: Use into_iter() to consume arg_values and avoid cloning
        for (param, value) in params.iter().zip(arg_values.into_iter()) {
            self.scope.define(param.clone(), value).map_err(|e| {
                EvalError::UnsupportedOperation {
                    operation: format!("define parameter: {}", e),
                }
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
            // INTERP-046: Use pre-collected param types (arg_values was consumed)
            let param_types = param_types_for_profiling.unwrap_or_default();
            let return_type = result.type_name().to_string();

            let signature = crate::profiler::TypeSignature::new(param_types, return_type);
            profiler.observe_type(name, signature);

            // Record function call timing for hot function detection
            let duration = start_time.elapsed();
            profiler.record_function_call(name, duration);
        }

        Ok(result)
    }

    /// Call a closure with captured environment
    ///
    /// Implements closure call semantics:
    /// 1. Check argument count matches parameter count
    /// 2. Evaluate all arguments eagerly (call-by-value)
    /// 3. Create new scope with captured environment
    /// 4. Bind parameters to argument values
    /// 5. Execute closure body
    /// 6. Restore previous scope
    ///
    /// Returns the last expression value from the closure body.
    pub(crate) fn call_closure(
        &mut self,
        params: &[String],
        body: &[AstNode],
        captured_env: &std::collections::HashMap<String, Value>,
        args: &[AstNode],
    ) -> Result<Value, EvalError> {
        // 1. Check argument count matches parameter count
        if args.len() != params.len() {
            return Err(EvalError::ArgumentCountMismatch {
                function: "<closure>".to_string(),
                expected: params.len(),
                actual: args.len(),
            });
        }

        // 2. Evaluate all arguments eagerly (call-by-value semantics)
        let mut arg_values = Vec::with_capacity(args.len());
        for arg in args {
            arg_values.push(self.eval(arg)?);
        }

        // 3. Save current scope and create new child scope
        let child_scope = self.scope.create_child();
        let saved_scope = std::mem::replace(&mut self.scope, child_scope);

        // 4. Restore captured environment into the new scope
        for (name, value) in captured_env {
            let _ = self.scope.define(name.clone(), value.clone());
        }

        // 5. Bind parameters to argument values
        for (param, arg_val) in params.iter().zip(arg_values.iter()) {
            let _ = self.scope.define(param.clone(), arg_val.clone());
        }

        // 6. Execute closure body
        let mut result = Value::nil();
        for stmt in body {
            match self.eval_internal(stmt) {
                Ok(ControlFlow::Value(v)) => {
                    result = v;
                }
                Ok(ControlFlow::Return(v)) => {
                    // Early return from closure
                    result = v;
                    break;
                }
                Err(e) => {
                    // Error occurred - restore scope before propagating
                    self.scope = saved_scope;
                    return Err(e);
                }
            }
        }

        // 7. Restore previous scope
        self.scope = saved_scope;

        Ok(result)
    }

    /// Call a method on a receiver value
    ///
    /// Implements basic method call syntax: receiver.method(args)
    /// Currently supports string methods: len(), contains()
    pub(crate) fn call_method(
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
            "is_empty" => {
                // String or Array is_empty: "".is_empty() => true, [].is_empty() => true
                if !arg_values.is_empty() {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: format!("{}.is_empty()", receiver.type_name()),
                        expected: 0,
                        actual: arg_values.len(),
                    });
                }

                if let Ok(s) = receiver.as_string() {
                    Ok(Value::boolean(s.is_empty()))
                } else if let Ok(arr) = receiver.as_vector() {
                    Ok(Value::boolean(arr.is_empty()))
                } else {
                    Err(EvalError::UnsupportedOperation {
                        operation: format!(
                            "method 'is_empty' not supported on type {}",
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

            "to_string" => {
                // "value".to_string() -> String
                // Convert any value to a String
                if !arg_values.is_empty() {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "to_string".to_string(),
                        expected: 0,
                        actual: arg_values.len(),
                    });
                }

                // For strings, just return as-is (already a String)
                // For other types, convert to their string representation
                if let Ok(s) = receiver.as_string() {
                    Ok(Value::string(s.to_string()))
                } else {
                    Ok(Value::string(receiver.to_println_string()))
                }
            }

            "get" => {
                // HashMap.get(key) -> Option<Value>
                // Get value from HashMap by key
                if arg_values.len() != 1 {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "get".to_string(),
                        expected: 1,
                        actual: arg_values.len(),
                    });
                }

                match receiver {
                    Value::HashMap(ref map) => {
                        let key = arg_values[0].as_string()?;
                        match map.get(key) {
                            Some(value) => Ok(value.clone()),
                            None => Ok(Value::nil()),
                        }
                    }
                    _ => Err(EvalError::UnsupportedOperation {
                        operation: format!(
                            "method 'get' not supported on type {}",
                            receiver.type_name()
                        ),
                    }),
                }
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
    pub(crate) fn try_call_builtin(
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

            "String::new" => {
                // String::new() -> String
                // Create an empty string
                if !args.is_empty() {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "String::new".to_string(),
                        expected: 0,
                        actual: args.len(),
                    });
                }
                Ok(Some(Value::string(String::new())))
            }

            "String::from" => {
                // String::from(s: &str) -> String
                // Convert string slice to String
                if args.len() != 1 {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "String::from".to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }

                let val = self.eval(&args[0])?;
                let s = val.as_string()?;
                Ok(Some(Value::string(s.to_string())))
            }

            "HashMap::new" => {
                // HashMap::new() -> HashMap<K, V>
                // Create an empty HashMap
                if !args.is_empty() {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "HashMap::new".to_string(),
                        expected: 0,
                        actual: args.len(),
                    });
                }
                use std::collections::HashMap;
                Ok(Some(Value::HashMap(HashMap::new())))
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
    pub(crate) fn eval_if(
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
    pub(crate) fn eval_loop_body_impl(&mut self, body: &[AstNode]) -> Result<Option<Value>, EvalError> {
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
    pub(crate) fn eval_loop_body_with_scope(&mut self, body: &[AstNode]) -> Result<Option<Value>, EvalError> {
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
    pub(crate) fn eval_while(
        &mut self,
        condition: &AstNode,
        body: &[AstNode],
    ) -> Result<ControlFlow, EvalError> {
        // INTERP-051: Track loop profiling for OSR
        let start_time = std::time::Instant::now();
        let mut iteration_count = 0;

        loop {
            // Evaluate condition
            let cond_val = self.eval(condition)?;
            let cond_bool = cond_val.as_boolean()?;

            if !cond_bool {
                break; // Exit loop when condition is false
            }

            // Track iteration
            iteration_count += 1;

            // Execute body in child scope
            if let Some(return_value) = self.eval_loop_body_with_scope(body)? {
                // Early return from enclosing function
                // Record loop data before returning
                if let Some(ref profiler) = self.compiler_profiler {
                    let duration = start_time.elapsed();
                    let default_name = "<main>".to_string();
                    let function_name = self.call_stack.last().unwrap_or(&default_name);
                    // For now, use simple loop indexing (can be improved later)
                    profiler.record_loop(function_name, 0, iteration_count, duration);
                }
                return Ok(ControlFlow::Return(return_value));
            }
        }

        // INTERP-051: Record loop profiling data
        if let Some(ref profiler) = self.compiler_profiler {
            let duration = start_time.elapsed();
            // Get current function name from call stack
            let default_name = "<main>".to_string();
            let function_name = self.call_stack.last().unwrap_or(&default_name);
            // For now, all loops in a function share index 0 (can track multiple loops later)
            profiler.record_loop(function_name, 0, iteration_count, duration);
        }

        // While loops return nil
        Ok(ControlFlow::Value(Value::nil()))
    }

    /// Evaluate for loop
    pub(crate) fn eval_for(
        &mut self,
        var: &str,
        iterable: &AstNode,
        body: &[AstNode],
    ) -> Result<ControlFlow, EvalError> {
        // Evaluate iterable expression
        let iterable_val = self.eval(iterable)?;

        // Get elements - support both Vector and HashMap iteration
        let elements = match &iterable_val {
            Value::Vector(_) => iterable_val.as_vector()?.clone(),
            Value::HashMap(map) => {
                // Convert HashMap to vector of (key, value) tuples
                map.iter()
                    .map(|(k, v)| Value::Vector(vec![Value::string(k.clone()), v.clone()]))
                    .collect()
            }
            _ => {
                return Err(EvalError::ValueError(ValueError::TypeMismatch {
                    expected: "Vector or HashMap".to_string(),
                    found: iterable_val.type_name().to_string(),
                    operation: "for-in iteration".to_string(),
                }))
            }
        };

        // Iterate over elements
        for element in elements.iter() {
            // Create child scope with loop variable bound to current element
            let child_scope = self.scope.create_child();
            let old_scope = std::mem::replace(&mut self.scope, child_scope);

            // Define loop variable in child scope
            // Check for tuple destructuring: (key, value)
            if var.starts_with('(') && var.contains(',') {
                // Parse tuple pattern: "(key, value)"
                let inner = var.trim_start_matches('(').trim_end_matches(')');
                let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();

                if parts.len() == 2 {
                    // Element should be a tuple (Vector with 2 elements)
                    if let Value::Vector(tuple_elements) = element {
                        if tuple_elements.len() >= 2 {
                            self.scope
                                .define(parts[0].to_string(), tuple_elements[0].clone())
                                .map_err(|e| EvalError::UnsupportedOperation {
                                    operation: format!("define loop variable {}: {}", parts[0], e),
                                })?;
                            self.scope
                                .define(parts[1].to_string(), tuple_elements[1].clone())
                                .map_err(|e| EvalError::UnsupportedOperation {
                                    operation: format!("define loop variable {}: {}", parts[1], e),
                                })?;
                        }
                    }
                }
            } else {
                // Simple identifier
                self.scope
                    .define(var.to_string(), element.clone())
                    .map_err(|e| EvalError::UnsupportedOperation {
                        operation: format!("define loop variable: {}", e),
                    })?;
            }

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
    pub(crate) fn eval_match(
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
