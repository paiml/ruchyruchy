// INTERP-053 (JIT-002): JIT Compiler Infrastructure
//
// JIT (Just-In-Time) compilation using Cranelift code generator
//
// Architecture:
// 1. Input: RuchyRuchy AST (from parser)
// 2. Translation: AST → Cranelift IR
// 3. Compilation: Cranelift IR → Machine code
// 4. Execution: Call compiled code via function pointer
//
// References:
// - Cranelift documentation: https://cranelift.dev/
// - JIT compilation techniques: Aycock (2003) "A Brief History of Just-In-Time"
// - SSA form: Cytron et al. (1991) "Efficiently computing static single assignment form"

use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};
use std::collections::HashMap;

use crate::interpreter::parser::{AstNode, BinaryOperator, UnaryOperator};

/// JIT compilation error
#[derive(Debug)]
pub enum JitError {
    /// Cranelift module error
    ModuleError(String),
    /// Compilation failed
    CompilationFailed(String),
    /// Unsupported AST node
    UnsupportedNode(String),
}

impl std::fmt::Display for JitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JitError::ModuleError(msg) => write!(f, "Module error: {}", msg),
            JitError::CompilationFailed(msg) => write!(f, "Compilation failed: {}", msg),
            JitError::UnsupportedNode(msg) => write!(f, "Unsupported AST node: {}", msg),
        }
    }
}

impl std::error::Error for JitError {}

/// String compilation context (passed through compilation)
struct StringContext<'a> {
    /// String literals storage (kept alive for JIT lifetime)
    literals: &'a mut Vec<Box<[u8]>>,
    /// String interning map (content → pointer)
    intern: &'a mut HashMap<String, i64>,
}

impl<'a> StringContext<'a> {
    /// Intern a string literal (reuse existing pointer if available)
    fn intern_string(&mut self, s: &str) -> i64 {
        // Check if already interned
        if let Some(&ptr) = self.intern.get(s) {
            return ptr;
        }

        // Create new string buffer: [i64 length][data bytes]
        let str_bytes = s.as_bytes();
        let total_size = 8 + str_bytes.len();
        let mut buffer = vec![0u8; total_size];

        // Write length as i64 (little-endian)
        let len = str_bytes.len() as i64;
        buffer[0..8].copy_from_slice(&len.to_le_bytes());

        // Write string data
        buffer[8..].copy_from_slice(str_bytes);

        // Store and get pointer
        let boxed = buffer.into_boxed_slice();
        let ptr = boxed.as_ptr() as i64;
        self.literals.push(boxed);

        // Cache for future lookups
        self.intern.insert(s.to_string(), ptr);

        ptr
    }
}

/// JIT Compiler
///
/// Compiles RuchyRuchy AST to machine code using Cranelift
pub struct JitCompiler {
    /// Cranelift JIT module
    module: JITModule,
    /// Function builder context (reused across compilations)
    builder_context: FunctionBuilderContext,
    /// Compiled function counter (for unique names)
    function_counter: usize,
    /// Compiled functions cache (name → pointer)
    compiled_functions: HashMap<String, *const u8>,
    /// String literals storage (kept alive for JIT lifetime)
    /// Format: Vec<Box<[u8]>> where each entry is [i64 length][data bytes]
    string_literals: Vec<Box<[u8]>>,
    /// String interning map (content → pointer) for string literal deduplication
    string_intern: HashMap<String, i64>,
}

impl JitCompiler {
    /// Create a new JIT compiler
    pub fn new() -> Result<Self, JitError> {
        let builder = JITBuilder::new(cranelift_module::default_libcall_names())
            .map_err(|e| JitError::ModuleError(e.to_string()))?;

        let module = JITModule::new(builder);

        Ok(Self {
            module,
            builder_context: FunctionBuilderContext::new(),
            function_counter: 0,
            compiled_functions: HashMap::new(),
            string_literals: Vec::new(),
            string_intern: HashMap::new(),
        })
    }

    /// Register a compiled function for use in function calls
    ///
    /// This allows JIT-compiled code to call other JIT-compiled functions
    pub fn register_function(&mut self, name: String, func_ptr: *const u8) {
        self.compiled_functions.insert(name, func_ptr);
    }

    /// Compile an expression to machine code
    ///
    /// Returns a function pointer that evaluates the expression and returns i64
    pub fn compile_expression(&mut self, ast: &AstNode) -> Result<fn() -> i64, JitError> {
        // Create unique function name
        let func_name = format!("__jit_expr_{}", self.function_counter);
        self.function_counter += 1;

        // Create function signature: () -> i64
        let mut sig = self.module.make_signature();
        sig.returns.push(AbiParam::new(types::I64));

        // Declare the function
        let func_id = self
            .module
            .declare_function(&func_name, Linkage::Export, &sig)
            .map_err(|e| JitError::ModuleError(e.to_string()))?;

        // Create compilation context
        let mut ctx = self.module.make_context();
        ctx.func.signature = sig;

        // Build function body
        {
            let mut builder = FunctionBuilder::new(&mut ctx.func, &mut self.builder_context);

            // Create entry block
            let entry_block = builder.create_block();
            builder.append_block_params_for_function_params(entry_block);
            builder.switch_to_block(entry_block);
            builder.seal_block(entry_block);

            // Compile expression to Cranelift IR
            let mut string_ctx = StringContext {
                literals: &mut self.string_literals,
                intern: &mut self.string_intern,
            };
            let result =
                Self::compile_expr(ast, &mut builder, &self.compiled_functions, &mut string_ctx)?;

            // Return the result
            builder.ins().return_(&[result]);

            // Finalize function
            builder.finalize();
        }

        // Define the function in the module
        self.module
            .define_function(func_id, &mut ctx)
            .map_err(|e| JitError::CompilationFailed(e.to_string()))?;

        // Finalize definitions (compile to machine code)
        self.module
            .finalize_definitions()
            .map_err(|e| JitError::CompilationFailed(e.to_string()))?;

        // Get function pointer
        let code_ptr = self.module.get_finalized_function(func_id);

        // Cache the compiled function
        self.compiled_functions.insert(func_name.clone(), code_ptr);

        // Return as function pointer
        let func: fn() -> i64 = unsafe { std::mem::transmute(code_ptr) };
        Ok(func)
    }

    /// Compile function with parameters
    ///
    /// Returns a raw function pointer that must be cast to the appropriate function type:
    /// - 1 param: fn(i64) -> i64
    /// - 2 params: fn(i64, i64) -> i64
    /// - 3 params: fn(i64, i64, i64) -> i64
    /// - etc.
    pub fn compile_function_with_params<T>(
        &mut self,
        param_names: &[String],
        body: &AstNode,
    ) -> Result<T, JitError> {
        // Create unique function name
        let func_name = format!("__jit_func_{}", self.function_counter);
        self.function_counter += 1;

        // Create function signature: (i64, i64, ...) -> i64
        let mut sig = self.module.make_signature();
        for _ in param_names {
            sig.params.push(AbiParam::new(types::I64));
        }
        sig.returns.push(AbiParam::new(types::I64));

        // Declare the function
        let func_id = self
            .module
            .declare_function(&func_name, Linkage::Export, &sig)
            .map_err(|e| JitError::ModuleError(e.to_string()))?;

        // Create compilation context
        let mut ctx = self.module.make_context();
        ctx.func.signature = sig;

        // Build function body
        {
            let mut builder = FunctionBuilder::new(&mut ctx.func, &mut self.builder_context);

            // Create entry block
            let entry_block = builder.create_block();

            // Add block parameters for function parameters
            builder.append_block_params_for_function_params(entry_block);

            builder.switch_to_block(entry_block);
            builder.seal_block(entry_block);

            // Get parameter values from the entry block
            let param_values: Vec<Value> = (0..param_names.len())
                .map(|i| builder.block_params(entry_block)[i])
                .collect();

            // Create variable map (parameter name → Cranelift value)
            let mut variables = HashMap::new();
            for (name, &value) in param_names.iter().zip(param_values.iter()) {
                variables.insert(name.clone(), value);
            }

            // Compile body expression with variable context
            let mut string_ctx = StringContext {
                literals: &mut self.string_literals,
                intern: &mut self.string_intern,
            };
            let result = Self::compile_expr_with_vars(
                body,
                &mut builder,
                &variables,
                &self.compiled_functions,
                &mut string_ctx,
            )?;

            // Check if body ends with explicit return (to avoid double-return error)
            let has_explicit_return = match body {
                AstNode::Return { .. } => true,
                AstNode::Block { statements } => statements
                    .last()
                    .is_some_and(|stmt| matches!(stmt, AstNode::Return { .. })),
                _ => false,
            };

            // Return the result (only if no explicit return in body)
            if !has_explicit_return {
                builder.ins().return_(&[result]);
            }

            // Finalize function
            builder.finalize();
        }

        // Define the function in the module
        self.module
            .define_function(func_id, &mut ctx)
            .map_err(|e| JitError::CompilationFailed(e.to_string()))?;

        // Finalize definitions (compile to machine code)
        self.module
            .finalize_definitions()
            .map_err(|e| JitError::CompilationFailed(e.to_string()))?;

        // Get function pointer
        let code_ptr = self.module.get_finalized_function(func_id);

        // Cache the compiled function
        self.compiled_functions.insert(func_name.clone(), code_ptr);

        // Return as generic function pointer (caller must cast to correct type)
        let func: T = unsafe { std::mem::transmute_copy(&code_ptr) };
        Ok(func)
    }

    /// Compile AST expression to Cranelift IR value (no variables)
    fn compile_expr(
        ast: &AstNode,
        builder: &mut FunctionBuilder,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
    ) -> Result<Value, JitError> {
        let mut var_counter = 0;

        Self::compile_expr_with_context(
            ast,
            builder,
            &HashMap::new(),
            &mut HashMap::new(),
            &mut var_counter,
            compiled_functions,
            string_ctx,
        )
    }

    /// Compile AST expression to Cranelift IR value (with variable context)
    fn compile_expr_with_vars(
        ast: &AstNode,
        builder: &mut FunctionBuilder,
        variables: &HashMap<String, Value>,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
    ) -> Result<Value, JitError> {
        let mut var_counter = 0;

        Self::compile_expr_with_context(
            ast,
            builder,
            variables,
            &mut HashMap::new(),
            &mut var_counter,
            compiled_functions,
            string_ctx,
        )
    }

    /// Compile AST expression to Cranelift IR value (with full context)
    fn compile_expr_with_context(
        ast: &AstNode,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
    ) -> Result<Value, JitError> {
        match ast {
            // Return statement: early function exit
            AstNode::Return { value } => {
                // Create dummy value first (before terminating block)
                // This is needed because we must return a Value, but after return_
                // the block is terminated and we can't create new instructions
                let dummy = builder.ins().iconst(types::I64, 0);

                if let Some(expr) = value {
                    // Compile return value
                    let return_value = Self::compile_expr_with_context(
                        expr,
                        builder,
                        parameters,
                        local_vars,
                        var_counter,
                        compiled_functions,
                        string_ctx,
                    )?;

                    // Convert F64 to I64 bits if needed (for float returns)
                    let return_value = if builder.func.dfg.value_type(return_value) == types::F64 {
                        builder
                            .ins()
                            .bitcast(types::I64, MemFlags::new(), return_value)
                    } else {
                        return_value
                    };

                    // Return from function (terminates block)
                    builder.ins().return_(&[return_value]);
                } else {
                    // Return with no value (return 0 by default)
                    let zero = builder.ins().iconst(types::I64, 0);
                    builder.ins().return_(&[zero]);
                }
                // Return the dummy value (created before block termination)
                Ok(dummy)
            }

            // Block: sequence of statements, returns last expression value
            AstNode::Block { statements } => {
                let mut result = builder.ins().iconst(types::I64, 0);
                for stmt in statements {
                    result = Self::compile_expr_with_context(
                        stmt,
                        builder,
                        parameters,
                        local_vars,
                        var_counter,
                        compiled_functions,
                        string_ctx,
                    )?;
                }
                Ok(result)
            }

            // Let declaration: create and initialize a variable
            AstNode::LetDecl { name, value } => {
                // Compile the initial value first to determine type
                let init_value = Self::compile_expr_with_context(
                    value,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                    compiled_functions,
                    string_ctx,
                )?;

                // Create a new Cranelift variable with the correct type
                let var = Variable::new(*var_counter);
                *var_counter += 1;

                // Declare it with the type matching the initial value
                let value_type = builder.func.dfg.value_type(init_value);
                builder.declare_var(var, value_type);

                // Define the variable with the initial value
                builder.def_var(var, init_value);

                // Store in local variables map
                local_vars.insert(name.clone(), var);

                // Let statements return 0 (they're statements, not expressions)
                Ok(builder.ins().iconst(types::I64, 0))
            }

            // Assignment: update an existing variable
            AstNode::Assignment { name, value } => {
                // Look up the variable (copy it to avoid borrow checker issues)
                let var = *local_vars.get(name).ok_or_else(|| {
                    JitError::UnsupportedNode(format!("Undefined variable: {}", name))
                })?;

                // Compile the new value
                let new_value = Self::compile_expr_with_context(
                    value,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                    compiled_functions,
                    string_ctx,
                )?;

                // Update the variable
                builder.def_var(var, new_value);

                // Assignments return 0 (they're statements, not expressions)
                Ok(builder.ins().iconst(types::I64, 0))
            }

            // Integer literal: load constant
            AstNode::IntegerLiteral(n) => {
                let val = builder.ins().iconst(types::I64, *n);
                Ok(val)
            }

            // Boolean literal: convert to 0 (false) or 1 (true)
            AstNode::BooleanLiteral(b) => {
                let val = builder.ins().iconst(types::I64, if *b { 1 } else { 0 });
                Ok(val)
            }

            // String literal: intern and return pointer
            // Format: [i64 length][data bytes]
            AstNode::StringLiteral(s) => {
                // Intern the string (reuse pointer if already seen)
                let ptr = string_ctx.intern_string(s);

                // Return pointer as i64
                let val = builder.ins().iconst(types::I64, ptr);
                Ok(val)
            }

            // Float literal: load as F64 constant
            // Note: F64 values are converted to I64 bits only at return statements
            AstNode::FloatLiteral(f) => {
                let f64_val = builder.ins().f64const(*f);
                Ok(f64_val)
            }

            // Identifier: lookup variable (local or parameter)
            AstNode::Identifier(name) => {
                // Check local variables first
                if let Some(&var) = local_vars.get(name) {
                    let value = builder.use_var(var);
                    Ok(value)
                // Then check parameters
                } else if let Some(&value) = parameters.get(name) {
                    Ok(value)
                } else {
                    Err(JitError::UnsupportedNode(format!(
                        "Undefined variable: {}",
                        name
                    )))
                }
            }

            // Binary operation: compile left, compile right, apply operator
            AstNode::BinaryOp { left, op, right } => {
                let lhs = Self::compile_expr_with_context(
                    left,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                    compiled_functions,
                    string_ctx,
                )?;
                let rhs = Self::compile_expr_with_context(
                    right,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                    compiled_functions,
                    string_ctx,
                )?;

                // Check if operands are floats
                let lhs_type = builder.func.dfg.value_type(lhs);
                let rhs_type = builder.func.dfg.value_type(rhs);
                let is_float = lhs_type == types::F64 || rhs_type == types::F64;

                let result = if is_float {
                    // Float arithmetic operations
                    match op {
                        BinaryOperator::Add => builder.ins().fadd(lhs, rhs),
                        BinaryOperator::Subtract => builder.ins().fsub(lhs, rhs),
                        BinaryOperator::Multiply => builder.ins().fmul(lhs, rhs),
                        BinaryOperator::Divide => builder.ins().fdiv(lhs, rhs),

                        // Float comparisons (return i64: 0 or 1)
                        BinaryOperator::Equal => {
                            let cmp = builder.ins().fcmp(FloatCC::Equal, lhs, rhs);
                            builder.ins().uextend(types::I64, cmp)
                        }
                        BinaryOperator::NotEqual => {
                            let cmp = builder.ins().fcmp(FloatCC::NotEqual, lhs, rhs);
                            builder.ins().uextend(types::I64, cmp)
                        }
                        BinaryOperator::LessThan => {
                            let cmp = builder.ins().fcmp(FloatCC::LessThan, lhs, rhs);
                            builder.ins().uextend(types::I64, cmp)
                        }
                        BinaryOperator::GreaterThan => {
                            let cmp = builder.ins().fcmp(FloatCC::GreaterThan, lhs, rhs);
                            builder.ins().uextend(types::I64, cmp)
                        }
                        BinaryOperator::LessEqual => {
                            let cmp = builder.ins().fcmp(FloatCC::LessThanOrEqual, lhs, rhs);
                            builder.ins().uextend(types::I64, cmp)
                        }
                        BinaryOperator::GreaterEqual => {
                            let cmp = builder.ins().fcmp(FloatCC::GreaterThanOrEqual, lhs, rhs);
                            builder.ins().uextend(types::I64, cmp)
                        }

                        BinaryOperator::Modulo => {
                            return Err(JitError::UnsupportedNode(
                                "Modulo operator not supported for floats".to_string(),
                            ))
                        }
                        BinaryOperator::And | BinaryOperator::Or => {
                            return Err(JitError::UnsupportedNode(
                                "Logical operators not supported for floats".to_string(),
                            ))
                        }
                    }
                } else {
                    // Integer arithmetic operations
                    match op {
                        BinaryOperator::Add => builder.ins().iadd(lhs, rhs),
                        BinaryOperator::Subtract => builder.ins().isub(lhs, rhs),
                        BinaryOperator::Multiply => builder.ins().imul(lhs, rhs),
                        BinaryOperator::Divide => builder.ins().sdiv(lhs, rhs),
                        BinaryOperator::Modulo => builder.ins().srem(lhs, rhs),

                        // Comparison operators (return 0 or 1 as i64)
                        BinaryOperator::Equal => {
                            let cmp = builder.ins().icmp(IntCC::Equal, lhs, rhs);
                            builder.ins().uextend(types::I64, cmp)
                        }
                        BinaryOperator::NotEqual => {
                            let cmp = builder.ins().icmp(IntCC::NotEqual, lhs, rhs);
                            builder.ins().uextend(types::I64, cmp)
                        }
                        BinaryOperator::LessThan => {
                            let cmp = builder.ins().icmp(IntCC::SignedLessThan, lhs, rhs);
                            builder.ins().uextend(types::I64, cmp)
                        }
                        BinaryOperator::GreaterThan => {
                            let cmp = builder.ins().icmp(IntCC::SignedGreaterThan, lhs, rhs);
                            builder.ins().uextend(types::I64, cmp)
                        }
                        BinaryOperator::LessEqual => {
                            let cmp = builder.ins().icmp(IntCC::SignedLessThanOrEqual, lhs, rhs);
                            builder.ins().uextend(types::I64, cmp)
                        }
                        BinaryOperator::GreaterEqual => {
                            let cmp = builder
                                .ins()
                                .icmp(IntCC::SignedGreaterThanOrEqual, lhs, rhs);
                            builder.ins().uextend(types::I64, cmp)
                        }

                        // Boolean operators (treat non-zero as true)
                        BinaryOperator::And => {
                            // lhs && rhs: both must be non-zero
                            let zero = builder.ins().iconst(types::I64, 0);
                            let lhs_bool = builder.ins().icmp(IntCC::NotEqual, lhs, zero);
                            let rhs_bool = builder.ins().icmp(IntCC::NotEqual, rhs, zero);
                            let and = builder.ins().band(lhs_bool, rhs_bool);
                            builder.ins().uextend(types::I64, and)
                        }
                        BinaryOperator::Or => {
                            // lhs || rhs: at least one must be non-zero
                            let zero = builder.ins().iconst(types::I64, 0);
                            let lhs_bool = builder.ins().icmp(IntCC::NotEqual, lhs, zero);
                            let rhs_bool = builder.ins().icmp(IntCC::NotEqual, rhs, zero);
                            let or = builder.ins().bor(lhs_bool, rhs_bool);
                            builder.ins().uextend(types::I64, or)
                        }
                    }
                };

                Ok(result)
            }

            // Unary operation: op operand (e.g., -x, !x, +x)
            AstNode::UnaryOp { op, operand } => {
                // Compile operand
                let value = Self::compile_expr_with_context(
                    operand,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                    compiled_functions,
                    string_ctx,
                )?;

                let result =
                    match op {
                        // Negation: -x
                        UnaryOperator::Negate => {
                            let value_type = builder.func.dfg.value_type(value);
                            if value_type == types::F64 {
                                builder.ins().fneg(value)
                            } else {
                                builder.ins().ineg(value)
                            }
                        }

                        // Logical NOT: !x
                        // Convert to boolean: x == 0 ? 1 : 0
                        UnaryOperator::Not => {
                            let zero = builder.ins().iconst(types::I64, 0);
                            let is_zero = builder.ins().icmp(IntCC::Equal, value, zero);
                            builder.ins().uextend(types::I64, is_zero)
                        }

                        // Unary plus: +x (identity operation)
                        UnaryOperator::Plus => value,

                        // Dereference not supported in MVP JIT
                        UnaryOperator::Dereference => return Err(JitError::UnsupportedNode(
                            "Dereference operator not supported in JIT (requires pointer types)"
                                .to_string(),
                        )),
                    };

                Ok(result)
            }

            // While loop: while (condition) { body }
            AstNode::WhileLoop { condition, body } => {
                // Create basic blocks for loop structure
                let loop_header = builder.create_block();
                let loop_body = builder.create_block();
                let loop_exit = builder.create_block();

                // Jump to loop header to start
                builder.ins().jump(loop_header, &[]);

                // Loop header: Evaluate condition and branch
                builder.switch_to_block(loop_header);
                // Don't seal yet - has back edge from loop_body

                let cond_value = Self::compile_expr_with_context(
                    condition,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                    compiled_functions,
                    string_ctx,
                )?;

                // Convert condition to boolean: condition != 0
                let zero = builder.ins().iconst(types::I64, 0);
                let is_true = builder.ins().icmp(IntCC::NotEqual, cond_value, zero);

                // Branch: if true, execute body; if false, exit loop
                builder.ins().brif(is_true, loop_body, &[], loop_exit, &[]);

                // Loop body: Execute statements and jump back to header
                builder.switch_to_block(loop_body);
                builder.seal_block(loop_body);

                // Execute loop body statements (if any)
                for stmt in body {
                    Self::compile_expr_with_context(
                        stmt,
                        builder,
                        parameters,
                        local_vars,
                        var_counter,
                        compiled_functions,
                        string_ctx,
                    )?;
                }

                // Back edge: Jump back to loop header
                builder.ins().jump(loop_header, &[]);

                // Now seal loop header (all predecessors known)
                builder.seal_block(loop_header);

                // Loop exit: Continue after loop, return 0
                builder.switch_to_block(loop_exit);
                builder.seal_block(loop_exit);

                let result = builder.ins().iconst(types::I64, 0);
                Ok(result)
            }

            // For loop: for var in iterable { body }
            // Desugars to: let var = start; while (var < end) { body; var = var + 1; }
            AstNode::ForLoop {
                var,
                iterable,
                body,
            } => {
                // Extract range start and end from iterable
                let (start_expr, end_expr) = match &**iterable {
                    AstNode::Range { start, end } => (start, end),
                    _ => {
                        return Err(JitError::UnsupportedNode(
                            "For loop iterable must be a Range".to_string(),
                        ))
                    }
                };

                // Compile start value
                let start_value = Self::compile_expr_with_context(
                    start_expr,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                    compiled_functions,
                    string_ctx,
                )?;

                // Create loop variable and initialize it
                let loop_var = Variable::new(*var_counter);
                *var_counter += 1;
                builder.declare_var(loop_var, types::I64);
                builder.def_var(loop_var, start_value);
                local_vars.insert(var.clone(), loop_var);

                // Create basic blocks for loop structure
                let loop_header = builder.create_block();
                let loop_body = builder.create_block();
                let loop_exit = builder.create_block();

                // Jump to loop header to start
                builder.ins().jump(loop_header, &[]);

                // Loop header: Check condition (var < end)
                builder.switch_to_block(loop_header);
                // Don't seal yet - has back edge from loop_body

                // Get current loop variable value
                let current_var = builder.use_var(loop_var);

                // Compile end value (may depend on variables)
                let end_value = Self::compile_expr_with_context(
                    end_expr,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                    compiled_functions,
                    string_ctx,
                )?;

                // Check condition: var < end
                let condition = builder
                    .ins()
                    .icmp(IntCC::SignedLessThan, current_var, end_value);

                // Branch: if true, execute body; if false, exit loop
                builder
                    .ins()
                    .brif(condition, loop_body, &[], loop_exit, &[]);

                // Loop body: Execute statements
                builder.switch_to_block(loop_body);
                builder.seal_block(loop_body);

                // Execute loop body statements
                for stmt in body {
                    Self::compile_expr_with_context(
                        stmt,
                        builder,
                        parameters,
                        local_vars,
                        var_counter,
                        compiled_functions,
                        string_ctx,
                    )?;
                }

                // Increment loop variable: var = var + 1
                let current_var = builder.use_var(loop_var);
                let one = builder.ins().iconst(types::I64, 1);
                let incremented = builder.ins().iadd(current_var, one);
                builder.def_var(loop_var, incremented);

                // Back edge: Jump back to loop header
                builder.ins().jump(loop_header, &[]);

                // Now seal loop header (all predecessors known)
                builder.seal_block(loop_header);

                // Loop exit: Continue after loop, return 0
                builder.switch_to_block(loop_exit);
                builder.seal_block(loop_exit);

                let result = builder.ins().iconst(types::I64, 0);
                Ok(result)
            }

            // If expression: if (condition) { then } else { else }
            AstNode::IfExpr {
                condition,
                then_branch,
                else_branch,
            } => {
                // Create basic blocks
                let then_block = builder.create_block();
                let else_block = builder.create_block();
                let merge_block = builder.create_block();

                // Both branches must return the same type (i64)
                builder.append_block_param(merge_block, types::I64);

                // Compile condition
                let cond_value = Self::compile_expr_with_context(
                    condition,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                    compiled_functions,
                    string_ctx,
                )?;

                // Convert to boolean: condition != 0
                let zero = builder.ins().iconst(types::I64, 0);
                let is_true = builder.ins().icmp(IntCC::NotEqual, cond_value, zero);

                // Branch: if true, go to then, else go to else
                builder
                    .ins()
                    .brif(is_true, then_block, &[], else_block, &[]);

                // Compile then branch
                builder.switch_to_block(then_block);
                builder.seal_block(then_block);

                let then_result = if then_branch.is_empty() {
                    builder.ins().iconst(types::I64, 0)
                } else {
                    let mut result = builder.ins().iconst(types::I64, 0);
                    for stmt in then_branch {
                        result = Self::compile_expr_with_context(
                            stmt,
                            builder,
                            parameters,
                            local_vars,
                            var_counter,
                            compiled_functions,
                            string_ctx,
                        )?;
                    }
                    result
                };

                // Only jump to merge if then_branch doesn't end with return
                let then_has_return = !then_branch.is_empty()
                    && matches!(then_branch.last().unwrap(), AstNode::Return { .. });
                if !then_has_return {
                    builder.ins().jump(merge_block, &[then_result]);
                }

                // Compile else branch
                builder.switch_to_block(else_block);
                builder.seal_block(else_block);

                let else_result = if let Some(else_stmts) = else_branch {
                    if else_stmts.is_empty() {
                        builder.ins().iconst(types::I64, 0)
                    } else {
                        let mut result = builder.ins().iconst(types::I64, 0);
                        for stmt in else_stmts {
                            result = Self::compile_expr_with_context(
                                stmt,
                                builder,
                                parameters,
                                local_vars,
                                var_counter,
                                compiled_functions,
                                string_ctx,
                            )?;
                        }
                        result
                    }
                } else {
                    builder.ins().iconst(types::I64, 0)
                };

                // Only jump to merge if else_branch doesn't end with return
                let else_has_return = if let Some(else_stmts) = else_branch {
                    !else_stmts.is_empty()
                        && matches!(else_stmts.last().unwrap(), AstNode::Return { .. })
                } else {
                    false
                };
                if !else_has_return {
                    builder.ins().jump(merge_block, &[else_result]);
                }

                // Continue at merge block
                builder.switch_to_block(merge_block);
                builder.seal_block(merge_block);

                // Get the result from the merge block parameter
                let result = builder.block_params(merge_block)[0];

                Ok(result)
            }

            // Function call: name(args)
            AstNode::FunctionCall { name, args } => {
                // Look up function in registry
                let func_ptr = compiled_functions.get(name).ok_or_else(|| {
                    JitError::UnsupportedNode(format!("Function '{}' not registered", name))
                })?;

                // Compile arguments
                let mut arg_values = Vec::new();
                for arg in args {
                    let arg_value = Self::compile_expr_with_context(
                        arg,
                        builder,
                        parameters,
                        local_vars,
                        var_counter,
                        compiled_functions,
                        string_ctx,
                    )?;
                    arg_values.push(arg_value);
                }

                // Create function signature based on argument count
                // Use the function builder's calling convention
                let mut sig = Signature::new(builder.func.signature.call_conv);
                for _ in 0..args.len() {
                    sig.params.push(AbiParam::new(types::I64));
                }
                sig.returns.push(AbiParam::new(types::I64));

                // Load function pointer as constant
                let ptr_value = *func_ptr as i64;
                let func_addr = builder.ins().iconst(types::I64, ptr_value);

                // Generate indirect call instruction
                let sig_ref = builder.import_signature(sig);
                let call = builder.ins().call_indirect(sig_ref, func_addr, &arg_values);
                let result = builder.inst_results(call)[0];

                Ok(result)
            }

            // Vector literal: [elem1, elem2, ...]
            AstNode::VectorLiteral { elements } => {
                if elements.is_empty() {
                    // Empty array - just return 0 for now (future work)
                    return Ok(builder.ins().iconst(types::I64, 0));
                }

                // Create stack slot for array (8 bytes per i64 element)
                let array_size = elements.len() * 8;
                let stack_slot = builder.create_sized_stack_slot(StackSlotData::new(
                    StackSlotKind::ExplicitSlot,
                    array_size as u32,
                    3, // 8-byte alignment (2^3 = 8)
                ));

                // Get address of stack slot
                let array_addr = builder.ins().stack_addr(types::I64, stack_slot, 0);

                // Store each element at appropriate offset
                for (i, elem) in elements.iter().enumerate() {
                    // Compile element value
                    let elem_value = Self::compile_expr_with_context(
                        elem,
                        builder,
                        parameters,
                        local_vars,
                        var_counter,
                        compiled_functions,
                        string_ctx,
                    )?;

                    // Calculate offset (i * 8 bytes)
                    let offset = (i * 8) as i32;

                    // Store value at array[i]
                    builder
                        .ins()
                        .store(MemFlags::trusted(), elem_value, array_addr, offset);
                }

                // Return the array address (as i64)
                Ok(array_addr)
            }

            // Tuple literal: (elem1, elem2, ...)
            // Implementation: stack-allocated like arrays, return pointer
            AstNode::TupleLiteral { elements } => {
                if elements.is_empty() {
                    // Empty tuple - just return 0 for now (future work)
                    return Ok(builder.ins().iconst(types::I64, 0));
                }

                // Create stack slot for tuple (8 bytes per element)
                let tuple_size = elements.len() * 8;
                let stack_slot = builder.create_sized_stack_slot(StackSlotData::new(
                    StackSlotKind::ExplicitSlot,
                    tuple_size as u32,
                    3, // 8-byte alignment (2^3 = 8)
                ));

                // Get address of stack slot
                let tuple_addr = builder.ins().stack_addr(types::I64, stack_slot, 0);

                // Store each element at appropriate offset
                for (i, elem) in elements.iter().enumerate() {
                    // Compile element value
                    let elem_value = Self::compile_expr_with_context(
                        elem,
                        builder,
                        parameters,
                        local_vars,
                        var_counter,
                        compiled_functions,
                        string_ctx,
                    )?;

                    // Calculate offset (i * 8 bytes)
                    let offset = (i * 8) as i32;

                    // Store value at tuple[i]
                    // Note: If elem_value is F64, it's stored as F64 bits
                    builder
                        .ins()
                        .store(MemFlags::trusted(), elem_value, tuple_addr, offset);
                }

                // Return the tuple address (as i64)
                Ok(tuple_addr)
            }

            // Index access: expr[index]
            AstNode::IndexAccess { expr, index } => {
                // Compile array expression (should return address)
                let array_addr = Self::compile_expr_with_context(
                    expr,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                    compiled_functions,
                    string_ctx,
                )?;

                // Compile index expression (should return integer)
                let index_value = Self::compile_expr_with_context(
                    index,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                    compiled_functions,
                    string_ctx,
                )?;

                // Calculate byte offset: index * 8 (8 bytes per i64)
                let eight = builder.ins().iconst(types::I64, 8);
                let byte_offset = builder.ins().imul(index_value, eight);

                // Calculate element address: array_addr + byte_offset
                let elem_addr = builder.ins().iadd(array_addr, byte_offset);

                // Load value from element address
                let value = builder
                    .ins()
                    .load(types::I64, MemFlags::trusted(), elem_addr, 0);

                Ok(value)
            }

            // Compound assignment: lhs op= rhs (e.g., x += 5, arr[i] *= 2)
            AstNode::CompoundAssignment { lhs, op, rhs } => {
                match lhs.as_ref() {
                    // Array element compound assignment: arr[index] += value
                    AstNode::IndexAccess { expr, index } => {
                        // Compile array address
                        let array_addr = Self::compile_expr_with_context(
                            expr,
                            builder,
                            parameters,
                            local_vars,
                            var_counter,
                            compiled_functions,
                            string_ctx,
                        )?;

                        // Compile index
                        let index_value = Self::compile_expr_with_context(
                            index,
                            builder,
                            parameters,
                            local_vars,
                            var_counter,
                            compiled_functions,
                            string_ctx,
                        )?;

                        // Calculate element address: array_addr + (index * 8)
                        let eight = builder.ins().iconst(types::I64, 8);
                        let byte_offset = builder.ins().imul(index_value, eight);
                        let elem_addr = builder.ins().iadd(array_addr, byte_offset);

                        // Load current value
                        let current_value =
                            builder
                                .ins()
                                .load(types::I64, MemFlags::trusted(), elem_addr, 0);

                        // Compile RHS
                        let rhs_value = Self::compile_expr_with_context(
                            rhs,
                            builder,
                            parameters,
                            local_vars,
                            var_counter,
                            compiled_functions,
                            string_ctx,
                        )?;

                        // Apply operation
                        let new_value = match op {
                            BinaryOperator::Add => builder.ins().iadd(current_value, rhs_value),
                            BinaryOperator::Subtract => {
                                builder.ins().isub(current_value, rhs_value)
                            }
                            BinaryOperator::Multiply => {
                                builder.ins().imul(current_value, rhs_value)
                            }
                            BinaryOperator::Divide => builder.ins().sdiv(current_value, rhs_value),
                            BinaryOperator::Modulo => builder.ins().srem(current_value, rhs_value),
                            _ => {
                                return Err(JitError::UnsupportedNode(format!(
                                    "Unsupported compound assignment operator: {:?}",
                                    op
                                )))
                            }
                        };

                        // Store new value back to array element
                        builder
                            .ins()
                            .store(MemFlags::trusted(), new_value, elem_addr, 0);

                        // Compound assignment returns 0 (it's a statement)
                        Ok(builder.ins().iconst(types::I64, 0))
                    }

                    // Variable compound assignment: x += 5
                    AstNode::Identifier(name) => {
                        // Look up variable
                        let var = *local_vars.get(name).ok_or_else(|| {
                            JitError::UnsupportedNode(format!("Undefined variable: {}", name))
                        })?;

                        // Get current value
                        let current_value = builder.use_var(var);

                        // Compile RHS
                        let rhs_value = Self::compile_expr_with_context(
                            rhs,
                            builder,
                            parameters,
                            local_vars,
                            var_counter,
                            compiled_functions,
                            string_ctx,
                        )?;

                        // Apply operation
                        let new_value = match op {
                            BinaryOperator::Add => builder.ins().iadd(current_value, rhs_value),
                            BinaryOperator::Subtract => {
                                builder.ins().isub(current_value, rhs_value)
                            }
                            BinaryOperator::Multiply => {
                                builder.ins().imul(current_value, rhs_value)
                            }
                            BinaryOperator::Divide => builder.ins().sdiv(current_value, rhs_value),
                            BinaryOperator::Modulo => builder.ins().srem(current_value, rhs_value),
                            _ => {
                                return Err(JitError::UnsupportedNode(format!(
                                    "Unsupported compound assignment operator: {:?}",
                                    op
                                )))
                            }
                        };

                        // Update variable
                        builder.def_var(var, new_value);

                        // Compound assignment returns 0 (it's a statement)
                        Ok(builder.ins().iconst(types::I64, 0))
                    }

                    _ => Err(JitError::UnsupportedNode(format!(
                        "Unsupported LHS in compound assignment: {:?}",
                        lhs
                    ))),
                }
            }

            // Field access: expr.field
            // Used for tuple indexing: tuple.0, tuple.1, etc.
            AstNode::FieldAccess { expr, field } => {
                // Compile the expression (should be a tuple/struct pointer)
                let tuple_addr = Self::compile_expr_with_context(
                    expr,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                    compiled_functions,
                    string_ctx,
                )?;

                // Try to parse field as integer index (for tuples: "0", "1", "2", ...)
                if let Ok(field_index) = field.parse::<usize>() {
                    // Calculate byte offset: field_index * 8
                    let offset = (field_index * 8) as i32;

                    // Load value from tuple[field_index]
                    let value =
                        builder
                            .ins()
                            .load(types::I64, MemFlags::trusted(), tuple_addr, offset);

                    Ok(value)
                } else {
                    // Named field access (for structs - not yet implemented)
                    Err(JitError::UnsupportedNode(format!(
                        "Named field access not yet supported: .{}",
                        field
                    )))
                }
            }

            // Unsupported AST node
            _ => Err(JitError::UnsupportedNode(format!(
                "Cannot compile AST node: {:?}",
                ast
            ))),
        }
    }
}
