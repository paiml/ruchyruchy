// Evaluator dispatch helpers: per-node-type evaluators extracted from eval_internal.
//
// These methods handle individual AST node evaluation: f-strings, ranges,
// method calls, blocks, compound assignment, index access, vec! macros,
// tuple destructuring, HashMap/struct literals, and field access.
//
// Extracted from evaluator.rs for file-health compliance (<500 lines per file).

use crate::interpreter::value::Value;
use super::evaluator::{ControlFlow, EvalError, Evaluator};

impl Evaluator {
    /// Evaluate f-string interpolation: f"text {expr} more"
    ///
    /// Scans content for `{expr}` blocks, parses and evaluates each expression,
    /// and concatenates the results into a single string.
    pub(crate) fn eval_fstring(&mut self, content: &str) -> Result<ControlFlow, EvalError> {
        use crate::interpreter::parser::Parser;

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

    /// Evaluate a range expression: start..end
    ///
    /// Creates a vector of integers from start (inclusive) to end (exclusive).
    pub(crate) fn eval_range(
        &mut self,
        start: &crate::interpreter::parser::AstNode,
        end: &crate::interpreter::parser::AstNode,
    ) -> Result<ControlFlow, EvalError> {
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

    /// Evaluate a method call on a receiver expression.
    ///
    /// Handles mutating methods (push, push_str, pop, insert) that modify
    /// the receiver in-place, then falls back to immutable method dispatch.
    pub(crate) fn eval_method_call(
        &mut self,
        receiver: &crate::interpreter::parser::AstNode,
        method: &str,
        args: &[crate::interpreter::parser::AstNode],
    ) -> Result<ControlFlow, EvalError> {
        use crate::interpreter::parser::AstNode;

        // Special handling for push() - it mutates the array
        if method == "push" {
            if let AstNode::Identifier(var_name) = receiver {
                let mut current_val = self.scope.get_cloned(var_name).map_err(|_| {
                    EvalError::UndefinedVariable {
                        name: var_name.clone(),
                    }
                })?;

                if args.len() != 1 {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "push".to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }
                let arg_val = self.eval(&args[0])?;

                match current_val {
                    Value::Vector(ref mut arr) => {
                        arr.push(arg_val);

                        // DEBUGGER-047: Track memory allocation for push
                        if let Some(ref profiler) = self.performance_profiler {
                            let bytes = std::mem::size_of::<Value>();
                            profiler.record_memory_allocation(bytes);
                        }

                        self.scope.assign(var_name, current_val).map_err(|_| {
                            EvalError::UndefinedVariable {
                                name: var_name.clone(),
                            }
                        })?;
                        return Ok(ControlFlow::Value(Value::nil()));
                    }
                    Value::String(ref mut s) => {
                        let char_str = arg_val.as_string().map_err(|_| {
                            EvalError::UnsupportedOperation {
                                operation: "push() on String requires char argument"
                                    .to_string(),
                            }
                        })?;
                        s.push_str(char_str);

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
            if let AstNode::Identifier(var_name) = receiver {
                let mut current_val = self.scope.get_cloned(var_name).map_err(|_| {
                    EvalError::UndefinedVariable {
                        name: var_name.clone(),
                    }
                })?;

                if args.len() != 1 {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "push_str".to_string(),
                        expected: 1,
                        actual: args.len(),
                    });
                }
                let arg_val = self.eval(&args[0])?;

                if let Value::String(ref mut s) = current_val {
                    let str_arg = arg_val.as_string().map_err(|_| {
                        EvalError::UnsupportedOperation {
                            operation: "push_str() requires String argument".to_string(),
                        }
                    })?;
                    s.push_str(str_arg);

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
            if let AstNode::Identifier(var_name) = receiver {
                let mut current_val = self.scope.get_cloned(var_name).map_err(|_| {
                    EvalError::UndefinedVariable {
                        name: var_name.clone(),
                    }
                })?;

                if !args.is_empty() {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "pop".to_string(),
                        expected: 0,
                        actual: args.len(),
                    });
                }

                if let Value::Vector(ref mut arr) = current_val {
                    let popped = arr.pop().unwrap_or(Value::nil());

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

        // Special handling for insert() - it mutates the HashMap
        if method == "insert" {
            if let AstNode::Identifier(var_name) = receiver {
                let mut current_val = self.scope.get_cloned(var_name).map_err(|_| {
                    EvalError::UndefinedVariable {
                        name: var_name.clone(),
                    }
                })?;

                if args.len() != 2 {
                    return Err(EvalError::ArgumentCountMismatch {
                        function: "insert".to_string(),
                        expected: 2,
                        actual: args.len(),
                    });
                }

                let key_val = self.eval(&args[0])?;
                let key = key_val.as_string()?;
                let value_val = self.eval(&args[1])?;

                if let Value::HashMap(ref mut map) = current_val {
                    map.insert(key.to_string(), value_val);

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

    /// Evaluate a block expression, creating a child scope.
    ///
    /// Evaluates all statements in a new scope, propagating early returns.
    /// The scope is restored after the block completes.
    pub(crate) fn eval_block(
        &mut self,
        statements: &[crate::interpreter::parser::AstNode],
    ) -> Result<ControlFlow, EvalError> {
        // Create child scope for block
        let child_scope = self.scope.create_child();
        let parent_scope = std::mem::replace(&mut self.scope, child_scope);

        // Evaluate all statements in block scope
        let mut last_value = Value::nil();
        let mut early_exit = None;

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

    /// Evaluate a compound assignment: x += 5, *num -= 1
    ///
    /// Desugars to `lhs = lhs op rhs`, handling both simple identifiers
    /// and dereference targets (arc_store / _inner model).
    pub(crate) fn eval_compound_assignment(
        &mut self,
        lhs: &crate::interpreter::parser::AstNode,
        op: crate::interpreter::parser::BinaryOperator,
        rhs: &crate::interpreter::parser::AstNode,
    ) -> Result<ControlFlow, EvalError> {
        use crate::interpreter::parser::{AstNode, UnaryOperator};

        // Evaluate current value of LHS
        let current_val = self.eval(lhs)?;

        // Evaluate RHS
        let rhs_val = self.eval(rhs)?;

        // Apply operation
        // INTERP-OPT-002: Move current_val instead of cloning (not used after binary op)
        let new_val = self.eval_binary_op(op, current_val, rhs_val)?;

        // Update the variable
        // For simple identifiers: x += 1
        if let AstNode::Identifier(name) = lhs {
            self.scope
                .assign(name, new_val)
                .map_err(|_| EvalError::UndefinedVariable { name: name.clone() })?;
            Ok(ControlFlow::Value(Value::nil()))
        } else if let AstNode::UnaryOp {
            op: UnaryOperator::Dereference,
            operand,
        } = lhs
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

    /// Evaluate index access: vec[i] or map[key]
    ///
    /// For vectors, the index must be a non-negative integer.
    /// For hashmaps, any value is accepted as key (converted to string).
    pub(crate) fn eval_index_access(
        &mut self,
        expr: &crate::interpreter::parser::AstNode,
        index: &crate::interpreter::parser::AstNode,
    ) -> Result<ControlFlow, EvalError> {
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

    /// Evaluate a vec! macro invocation.
    ///
    /// Supports two forms:
    /// - Elements: `vec![1, 2, 3]` or `vec![]`
    /// - Repeat: `vec![0; 10]`
    pub(crate) fn eval_vec_macro(
        &mut self,
        elements: &[crate::interpreter::parser::AstNode],
        repeat_count: Option<&crate::interpreter::parser::AstNode>,
    ) -> Result<ControlFlow, EvalError> {
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
                let bytes = array.len() * std::mem::size_of::<Value>();
                profiler.record_memory_allocation(bytes);
            }

            Ok(ControlFlow::Value(Value::Vector(array)))
        }
    }

    /// Evaluate tuple destructuring: let (a, b, c) = expr
    ///
    /// Evaluates the RHS, verifies it is a tuple with matching arity,
    /// and binds each element to the corresponding name.
    pub(crate) fn eval_tuple_destruct(
        &mut self,
        names: &[String],
        value: &crate::interpreter::parser::AstNode,
    ) -> Result<ControlFlow, EvalError> {
        let tuple_val = self.eval(value)?;

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

        if names.len() != elements.len() {
            return Err(EvalError::UnsupportedOperation {
                operation: format!(
                    "tuple destructuring: expected {} elements, got {}",
                    names.len(),
                    elements.len()
                ),
            });
        }

        for (name, elem) in names.iter().zip(elements.iter()) {
            self.scope.define(name.clone(), elem.clone()).map_err(|e| {
                EvalError::UnsupportedOperation {
                    operation: format!("define variable in tuple destructuring: {}", e),
                }
            })?;
        }

        Ok(ControlFlow::Value(Value::nil()))
    }

    /// Evaluate a HashMap literal: {key1: val1, key2: val2, ...}
    pub(crate) fn eval_hashmap_literal(
        &mut self,
        pairs: &[(crate::interpreter::parser::AstNode, crate::interpreter::parser::AstNode)],
    ) -> Result<ControlFlow, EvalError> {
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

    /// Evaluate a struct literal, creating it as a HashMap.
    pub(crate) fn eval_struct_literal(
        &mut self,
        fields: &[(String, crate::interpreter::parser::AstNode)],
    ) -> Result<ControlFlow, EvalError> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (field_name, field_val_node) in fields {
            let field_val = self.eval(field_val_node)?;
            map.insert(field_name.clone(), field_val);
        }
        Ok(ControlFlow::Value(Value::HashMap(map)))
    }

    /// Evaluate field access on a struct/object value.
    pub(crate) fn eval_field_access(
        &mut self,
        expr: &crate::interpreter::parser::AstNode,
        field: &str,
    ) -> Result<ControlFlow, EvalError> {
        let value = self.eval(expr)?;
        match &value {
            Value::HashMap(_) => {
                let key = Value::string(field.to_string());
                let result = value.get(&key)?.clone();
                Ok(ControlFlow::Value(result))
            }
            _ => Err(EvalError::UnsupportedOperation {
                operation: format!("field access on {}", value.type_name()),
            }),
        }
    }
}
