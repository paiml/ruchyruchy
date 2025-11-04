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

use crate::interpreter::parser::{AstNode, BinaryOperator, Pattern, UnaryOperator};

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
    /// Struct definitions (name → ordered field names)
    struct_defs: HashMap<String, Vec<String>>,
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
            struct_defs: HashMap::new(),
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
            let result = Self::compile_expr(
                ast,
                &mut builder,
                &self.compiled_functions,
                &mut string_ctx,
                &mut self.struct_defs,
            )?;

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
                &mut self.struct_defs,
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
        struct_defs: &mut HashMap<String, Vec<String>>,
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
            struct_defs,
        )
    }

    /// Compile AST expression to Cranelift IR value (with variable context)
    fn compile_expr_with_vars(
        ast: &AstNode,
        builder: &mut FunctionBuilder,
        variables: &HashMap<String, Value>,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
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
            struct_defs,
        )
    }

    /// Compile AST expression to Cranelift IR value (with full context)
    #[allow(clippy::too_many_arguments)]
    fn compile_expr_with_context(
        ast: &AstNode,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
        compiled_functions: &HashMap<String, *const u8>,
        string_ctx: &mut StringContext,
        struct_defs: &mut HashMap<String, Vec<String>>,
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
                        struct_defs,
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
                        struct_defs,
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
                    struct_defs,
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
                    struct_defs,
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
                    struct_defs,
                )?;
                let rhs = Self::compile_expr_with_context(
                    right,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                    compiled_functions,
                    string_ctx,
                    struct_defs,
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
                    struct_defs,
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
                    struct_defs,
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
                        struct_defs,
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
                    struct_defs,
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
                    struct_defs,
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
                        struct_defs,
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
                    struct_defs,
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
                            struct_defs,
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
                                struct_defs,
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
                        struct_defs,
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
                        struct_defs,
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
                        struct_defs,
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
            // Supports both array indexing and HashMap lookup
            AstNode::IndexAccess { expr, index } => {
                // Compile collection expression (array or hashmap address)
                let coll_addr = Self::compile_expr_with_context(
                    expr,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                    compiled_functions,
                    string_ctx,
                    struct_defs,
                )?;

                // Compile index/key expression
                let key_value = Self::compile_expr_with_context(
                    index,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                    compiled_functions,
                    string_ctx,
                    struct_defs,
                )?;

                // Check if this is a HashMap by looking for magic number -9999 at offset 0
                // HashMap format: [MAGIC=-9999, count, k1, v1, k2, v2, ...]
                // Array format: [elem0, elem1, elem2, ...]

                // Load potential magic from offset 0
                let magic_value = builder
                    .ins()
                    .load(types::I64, MemFlags::trusted(), coll_addr, 0);

                // Check if magic == -9999
                let magic_constant = builder.ins().iconst(types::I64, -9999);
                let is_hashmap = builder
                    .ins()
                    .icmp(IntCC::Equal, magic_value, magic_constant);

                // Create blocks for conditional
                let hashmap_block = builder.create_block();
                let array_block = builder.create_block();
                let merge_block = builder.create_block();

                // Add block parameter for result
                builder.append_block_param(merge_block, types::I64);

                // Branch based on magic number check
                builder
                    .ins()
                    .brif(is_hashmap, hashmap_block, &[], array_block, &[]);

                // HASHMAP LOOKUP PATH: Linear search through key-value pairs
                builder.switch_to_block(hashmap_block);
                builder.seal_block(hashmap_block);
                {
                    // Load count from offset 8 (after magic at offset 0)
                    let count = builder
                        .ins()
                        .load(types::I64, MemFlags::trusted(), coll_addr, 8);

                    // Create loop blocks
                    let loop_header = builder.create_block();
                    let loop_body = builder.create_block();
                    let loop_exit = builder.create_block();

                    // Loop variable: index i (0 to count-1)
                    builder.append_block_param(loop_header, types::I64);

                    // Jump to loop with i=0
                    let i_init = builder.ins().iconst(types::I64, 0);
                    builder.ins().jump(loop_header, &[i_init]);

                    // Loop header: check if i < count
                    builder.switch_to_block(loop_header);
                    let i = builder.block_params(loop_header)[0];
                    let i_lt_count = builder.ins().icmp(IntCC::SignedLessThan, i, count);
                    builder
                        .ins()
                        .brif(i_lt_count, loop_body, &[], loop_exit, &[]);

                    // Loop body: check if keys match
                    builder.switch_to_block(loop_body);
                    builder.seal_block(loop_body);

                    // Define constants used across blocks in loop body
                    let one = builder.ins().iconst(types::I64, 1);
                    let two = builder.ins().iconst(types::I64, 2);
                    let eight = builder.ins().iconst(types::I64, 8);

                    // Calculate key offset: (2 + i*2) * 8 (account for magic + count)
                    let i_times_2 = builder.ins().imul(i, two);
                    let key_index = builder.ins().iadd(two, i_times_2);
                    let key_offset = builder.ins().imul(key_index, eight);
                    let key_addr = builder.ins().iadd(coll_addr, key_offset);
                    let stored_key =
                        builder
                            .ins()
                            .load(types::I64, MemFlags::trusted(), key_addr, 0);

                    // Check if stored_key == search_key
                    let keys_match = builder.ins().icmp(IntCC::Equal, stored_key, key_value);

                    let found_block = builder.create_block();
                    let continue_block = builder.create_block();
                    builder
                        .ins()
                        .brif(keys_match, found_block, &[], continue_block, &[]);

                    // Found: load value and jump to merge
                    builder.switch_to_block(found_block);
                    builder.seal_block(found_block);
                    let val_index = builder.ins().iadd(key_index, one);
                    let val_offset = builder.ins().imul(val_index, eight);
                    let val_addr = builder.ins().iadd(coll_addr, val_offset);
                    let found_value =
                        builder
                            .ins()
                            .load(types::I64, MemFlags::trusted(), val_addr, 0);
                    builder.ins().jump(merge_block, &[found_value]);

                    // Continue: increment i and loop
                    builder.switch_to_block(continue_block);
                    builder.seal_block(continue_block);
                    let i_next = builder.ins().iadd(i, one);
                    builder.ins().jump(loop_header, &[i_next]);

                    // Seal loop header now that all predecessors are known
                    builder.seal_block(loop_header);

                    // Loop exit: key not found, return 0
                    builder.switch_to_block(loop_exit);
                    builder.seal_block(loop_exit);
                    let not_found = builder.ins().iconst(types::I64, 0);
                    builder.ins().jump(merge_block, &[not_found]);
                }

                // ARRAY PATH: Simple index-based access
                builder.switch_to_block(array_block);
                builder.seal_block(array_block);
                {
                    let eight = builder.ins().iconst(types::I64, 8);
                    let byte_offset = builder.ins().imul(key_value, eight);
                    let elem_addr = builder.ins().iadd(coll_addr, byte_offset);
                    let value = builder
                        .ins()
                        .load(types::I64, MemFlags::trusted(), elem_addr, 0);
                    builder.ins().jump(merge_block, &[value]);
                }

                // Merge block: get result from either path
                builder.switch_to_block(merge_block);
                builder.seal_block(merge_block);
                let result = builder.block_params(merge_block)[0];

                Ok(result)
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
                            struct_defs,
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
                            struct_defs,
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
                            struct_defs,
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
                            struct_defs,
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
                    struct_defs,
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
                    // Named field access (for structs)
                    // MVP approach: search all registered structs to find field position
                    // This is not type-safe but works when field names don't collide

                    let mut field_offset: Option<i32> = None;

                    // Search through all struct definitions
                    for (_struct_name, field_names) in struct_defs.iter() {
                        if let Some(index) = field_names.iter().position(|f| f == field) {
                            field_offset = Some((index * 8) as i32);
                            break;
                        }
                    }

                    match field_offset {
                        Some(offset) => {
                            // Load value from struct.field
                            let value = builder.ins().load(
                                types::I64,
                                MemFlags::trusted(),
                                tuple_addr,
                                offset,
                            );
                            Ok(value)
                        }
                        None => Err(JitError::UnsupportedNode(format!(
                            "Field '{}' not found in any registered struct",
                            field
                        ))),
                    }
                }
            }

            // Struct definition: struct Name { field1, field2, ... }
            // Registers the struct type with field names for later use
            AstNode::StructDef { name, fields } => {
                // Extract field names from StructField list
                let field_names: Vec<String> = fields.iter().map(|f| f.name.clone()).collect();

                // Register struct definition (name -> ordered field names)
                struct_defs.insert(name.clone(), field_names);

                // StructDef doesn't produce a runtime value - return 0
                Ok(builder.ins().iconst(types::I64, 0))
            }

            // Struct literal: StructName { field1: value1, field2: value2, ... }
            // Implementation: stack-allocated like tuples, return pointer
            AstNode::StructLiteral { name, fields } => {
                // Look up struct definition to get field order (clone to avoid borrow issues)
                let field_order = struct_defs
                    .get(name)
                    .ok_or_else(|| {
                        JitError::UnsupportedNode(format!("Undefined struct type: {}", name))
                    })?
                    .clone();

                if fields.is_empty() {
                    // Empty struct - just return 0 for now
                    return Ok(builder.ins().iconst(types::I64, 0));
                }

                // Create stack slot for struct (8 bytes per field)
                let struct_size = field_order.len() * 8;
                let stack_slot = builder.create_sized_stack_slot(StackSlotData::new(
                    StackSlotKind::ExplicitSlot,
                    struct_size as u32,
                    3, // 8-byte alignment (2^3 = 8)
                ));

                // Get address of stack slot
                let struct_addr = builder.ins().stack_addr(types::I64, stack_slot, 0);

                // Store each field at appropriate offset based on field order
                for (field_name, field_value_ast) in fields {
                    // Find field position in struct definition
                    let field_index = field_order
                        .iter()
                        .position(|f| f == field_name)
                        .ok_or_else(|| {
                            JitError::UnsupportedNode(format!(
                                "Field '{}' not found in struct '{}'",
                                field_name, name
                            ))
                        })?;

                    // Compile field value
                    let field_value = Self::compile_expr_with_context(
                        field_value_ast,
                        builder,
                        parameters,
                        local_vars,
                        var_counter,
                        compiled_functions,
                        string_ctx,
                        struct_defs,
                    )?;

                    // Calculate byte offset: field_index * 8
                    let offset = (field_index * 8) as i32;

                    // Store value at struct.field
                    builder
                        .ins()
                        .store(MemFlags::trusted(), field_value, struct_addr, offset);
                }

                // Return the struct address (as i64)
                Ok(struct_addr)
            }

            // HashMap literal: {key1: value1, key2: value2, ...}
            // Implementation: stack-allocated with format [MAGIC, count, k1, v1, k2, v2, ...]
            // MAGIC = -9999 distinguishes from arrays
            // Linear search for lookup (MVP - not efficient but correct)
            AstNode::HashMapLiteral { pairs } => {
                if pairs.is_empty() {
                    // Empty hashmap - allocate minimal structure: [MAGIC, 0]
                    let stack_slot = builder.create_sized_stack_slot(StackSlotData::new(
                        StackSlotKind::ExplicitSlot,
                        16, // 8 bytes for magic + 8 bytes for count=0
                        3,
                    ));
                    let hashmap_addr = builder.ins().stack_addr(types::I64, stack_slot, 0);
                    let magic = builder.ins().iconst(types::I64, -9999);
                    let zero = builder.ins().iconst(types::I64, 0);
                    builder
                        .ins()
                        .store(MemFlags::trusted(), magic, hashmap_addr, 0);
                    builder
                        .ins()
                        .store(MemFlags::trusted(), zero, hashmap_addr, 8);
                    return Ok(hashmap_addr);
                }

                // Calculate size: magic + count + (key, value) pairs
                // [MAGIC, count, k1, v1, k2, v2, ...]
                let hashmap_size = (2 + pairs.len() * 2) * 8; // magic + count + pairs

                // Create stack slot for hashmap
                let stack_slot = builder.create_sized_stack_slot(StackSlotData::new(
                    StackSlotKind::ExplicitSlot,
                    hashmap_size as u32,
                    3, // 8-byte alignment
                ));

                // Get address of stack slot
                let hashmap_addr = builder.ins().stack_addr(types::I64, stack_slot, 0);

                // Store magic at offset 0
                let magic = builder.ins().iconst(types::I64, -9999);
                builder
                    .ins()
                    .store(MemFlags::trusted(), magic, hashmap_addr, 0);

                // Store count at offset 8
                let count = builder.ins().iconst(types::I64, pairs.len() as i64);
                builder
                    .ins()
                    .store(MemFlags::trusted(), count, hashmap_addr, 8);

                // Store each key-value pair
                for (i, (key_ast, value_ast)) in pairs.iter().enumerate() {
                    // Compile key
                    let key_value = Self::compile_expr_with_context(
                        key_ast,
                        builder,
                        parameters,
                        local_vars,
                        var_counter,
                        compiled_functions,
                        string_ctx,
                        struct_defs,
                    )?;

                    // Compile value
                    let val_value = Self::compile_expr_with_context(
                        value_ast,
                        builder,
                        parameters,
                        local_vars,
                        var_counter,
                        compiled_functions,
                        string_ctx,
                        struct_defs,
                    )?;

                    // Calculate offsets (account for magic + count = 2 words)
                    // Pair i: key at offset (2 + i*2) * 8, value at offset (2 + i*2 + 1) * 8
                    let key_offset = ((2 + i * 2) * 8) as i32;
                    let val_offset = ((2 + i * 2 + 1) * 8) as i32;

                    // Store key and value
                    builder
                        .ins()
                        .store(MemFlags::trusted(), key_value, hashmap_addr, key_offset);
                    builder
                        .ins()
                        .store(MemFlags::trusted(), val_value, hashmap_addr, val_offset);
                }

                // Return the hashmap address (as i64)
                Ok(hashmap_addr)
            }

            // Match expression: match expr { pattern => body, ... }
            // Implementation: Compile to if-else chain checking each pattern in order
            // Patterns: Literal (compare), Identifier (bind), Wildcard (catch-all)
            AstNode::MatchExpr { expr, arms } => {
                // Compile the match expression (value to match against)
                let match_value = Self::compile_expr_with_context(
                    expr,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                    compiled_functions,
                    string_ctx,
                    struct_defs,
                )?;

                // Create merge block (all arms jump here with result)
                let merge_block = builder.create_block();
                builder.append_block_param(merge_block, types::I64);

                // Process each arm in order
                let mut current_block = None;
                for (arm_index, arm) in arms.iter().enumerate() {
                    // Create blocks for this arm
                    let arm_body_block = builder.create_block();
                    let next_arm_block = if arm_index < arms.len() - 1 {
                        Some(builder.create_block())
                    } else {
                        None
                    };

                    // If not first arm, switch to current block
                    if let Some(block) = current_block {
                        builder.switch_to_block(block);
                    }

                    // Check if pattern matches
                    match &arm.pattern {
                        // Literal pattern: compare match_value == pattern_value
                        Pattern::Literal(pattern_ast) => {
                            // Compile the pattern value (constant)
                            let pattern_value = Self::compile_expr_with_context(
                                pattern_ast,
                                builder,
                                parameters,
                                local_vars,
                                var_counter,
                                compiled_functions,
                                string_ctx,
                                struct_defs,
                            )?;

                            // Check if values match
                            let matches =
                                builder.ins().icmp(IntCC::Equal, match_value, pattern_value);

                            // If match, execute body; otherwise try next arm
                            if let Some(next) = next_arm_block {
                                builder.ins().brif(matches, arm_body_block, &[], next, &[]);
                            } else {
                                // Last arm - if pattern doesn't match, still execute body (for exhaustiveness)
                                // In a real compiler, this would be a type error if not exhaustive
                                builder.ins().jump(arm_body_block, &[]);
                            }
                        }

                        // Identifier pattern: bind variable, always matches
                        Pattern::Identifier(_name) => {
                            // Jump to body unconditionally
                            builder.ins().jump(arm_body_block, &[]);
                        }

                        // Wildcard pattern: always matches
                        Pattern::Wildcard => {
                            // Jump to body unconditionally
                            builder.ins().jump(arm_body_block, &[]);
                        }
                    }

                    // Seal current block if it exists
                    if let Some(block) = current_block {
                        builder.seal_block(block);
                    }

                    // Compile arm body
                    builder.switch_to_block(arm_body_block);
                    builder.seal_block(arm_body_block);

                    // For Identifier pattern, bind the variable
                    if let Pattern::Identifier(name) = &arm.pattern {
                        // Create a new Cranelift variable
                        let var = Variable::new(*var_counter);
                        *var_counter += 1;
                        // Declare it with the type matching the match value
                        let value_type = builder.func.dfg.value_type(match_value);
                        builder.declare_var(var, value_type);
                        // Define the variable with the match value
                        builder.def_var(var, match_value);
                        // Store in local variables map
                        local_vars.insert(name.clone(), var);
                    }

                    // Execute arm body (last expression is the result)
                    let arm_result = if arm.body.is_empty() {
                        builder.ins().iconst(types::I64, 0)
                    } else {
                        let mut result = builder.ins().iconst(types::I64, 0);
                        for stmt in &arm.body {
                            result = Self::compile_expr_with_context(
                                stmt,
                                builder,
                                parameters,
                                local_vars,
                                var_counter,
                                compiled_functions,
                                string_ctx,
                                struct_defs,
                            )?;
                        }
                        result
                    };

                    // Jump to merge with result
                    builder.ins().jump(merge_block, &[arm_result]);

                    // Move to next arm block
                    current_block = next_arm_block;
                }

                // Switch to merge block and return result
                builder.switch_to_block(merge_block);
                builder.seal_block(merge_block);
                let result = builder.block_params(merge_block)[0];
                Ok(result)
            }

            // Method call: receiver.method(args)
            // Implementation: Desugar to function call with receiver as first argument
            // receiver.method(arg1, arg2) → method(receiver, arg1, arg2)
            AstNode::MethodCall {
                receiver,
                method,
                args,
            } => {
                // Look up method function in registry
                let func_ptr = compiled_functions.get(method).ok_or_else(|| {
                    JitError::UnsupportedNode(format!("Method '{}' not registered", method))
                })?;

                // Compile the receiver expression (becomes first argument)
                let receiver_value = Self::compile_expr_with_context(
                    receiver,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                    compiled_functions,
                    string_ctx,
                    struct_defs,
                )?;

                // Compile remaining arguments
                let mut all_args = vec![receiver_value]; // receiver is first argument
                for arg in args {
                    let arg_value = Self::compile_expr_with_context(
                        arg,
                        builder,
                        parameters,
                        local_vars,
                        var_counter,
                        compiled_functions,
                        string_ctx,
                        struct_defs,
                    )?;
                    all_args.push(arg_value);
                }

                // Create function signature: receiver + args → return value
                let mut sig = Signature::new(builder.func.signature.call_conv);
                for _ in 0..all_args.len() {
                    sig.params.push(AbiParam::new(types::I64));
                }
                sig.returns.push(AbiParam::new(types::I64));

                // Load function pointer as constant
                let ptr_value = *func_ptr as i64;
                let func_addr = builder.ins().iconst(types::I64, ptr_value);

                // Generate indirect call instruction
                let sig_ref = builder.import_signature(sig);
                let call = builder.ins().call_indirect(sig_ref, func_addr, &all_args);
                let result = builder.inst_results(call)[0];
                Ok(result)
            }

            // Tuple destructuring: let (a, b, c) = tuple_expr;
            // Implementation: Load tuple pointer, then load each field into new variables
            AstNode::TupleDestruct { names, value } => {
                // Compile the tuple value expression (returns tuple pointer)
                let tuple_addr = Self::compile_expr_with_context(
                    value,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                    compiled_functions,
                    string_ctx,
                    struct_defs,
                )?;

                // For each pattern name, load the corresponding tuple field
                for (i, name) in names.iter().enumerate() {
                    // Calculate byte offset: field i is at offset i * 8
                    let offset = (i * 8) as i32;

                    // Load value from tuple[i]
                    let field_value =
                        builder
                            .ins()
                            .load(types::I64, MemFlags::trusted(), tuple_addr, offset);

                    // Create a new Cranelift variable for this pattern name
                    let var = Variable::new(*var_counter);
                    *var_counter += 1;

                    // Declare it with the type matching the field value
                    let value_type = builder.func.dfg.value_type(field_value);
                    builder.declare_var(var, value_type);

                    // Define the variable with the field value
                    builder.def_var(var, field_value);

                    // Store in local variables map
                    local_vars.insert(name.clone(), var);
                }

                // TupleDestruct doesn't produce a runtime value - return 0
                Ok(builder.ins().iconst(types::I64, 0))
            }

            // Type cast: expr as target_type
            // Implementation: Convert between i64 and f64 types
            AstNode::TypeCast { expr, target_type } => {
                // Compile the expression to cast
                let value = Self::compile_expr_with_context(
                    expr,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                    compiled_functions,
                    string_ctx,
                    struct_defs,
                )?;

                // Get the source type from the value
                let source_type = builder.func.dfg.value_type(value);

                // Perform type conversion based on target type
                match target_type.as_str() {
                    "f64" => {
                        // Cast to f64
                        if source_type == types::I64 {
                            // i64 → f64: signed integer to float
                            let float_value = builder.ins().fcvt_from_sint(types::F64, value);
                            Ok(float_value)
                        } else if source_type == types::F64 {
                            // f64 → f64: no-op
                            Ok(value)
                        } else {
                            Err(JitError::UnsupportedNode(format!(
                                "Cannot cast {:?} to f64",
                                source_type
                            )))
                        }
                    }
                    "i64" => {
                        // Cast to i64
                        if source_type == types::F64 {
                            // f64 → i64: float to signed integer (truncates toward zero)
                            let int_value = builder.ins().fcvt_to_sint_sat(types::I64, value);
                            Ok(int_value)
                        } else if source_type == types::I64 {
                            // i64 → i64: no-op
                            Ok(value)
                        } else {
                            Err(JitError::UnsupportedNode(format!(
                                "Cannot cast {:?} to i64",
                                source_type
                            )))
                        }
                    }
                    _ => Err(JitError::UnsupportedNode(format!(
                        "Unknown target type for cast: {}",
                        target_type
                    ))),
                }
            }

            // vec![] macro: vec![elem1, elem2] or vec![value; count]
            // Implementation: Desugar to array allocation and initialization
            AstNode::VecMacro {
                elements,
                repeat_count,
            } => {
                match repeat_count {
                    None => {
                        // List form: vec![elem1, elem2, elem3]
                        // Desugar to VectorLiteral logic
                        if elements.is_empty() {
                            // Empty vec![] - allocate minimal stack slot for consistency
                            let stack_slot = builder.create_sized_stack_slot(StackSlotData::new(
                                StackSlotKind::ExplicitSlot,
                                8, // Minimal 8 bytes
                                3,
                            ));
                            let array_addr = builder.ins().stack_addr(types::I64, stack_slot, 0);
                            return Ok(array_addr);
                        }

                        // Create stack slot for array (8 bytes per element)
                        let array_size = elements.len() * 8;
                        let stack_slot = builder.create_sized_stack_slot(StackSlotData::new(
                            StackSlotKind::ExplicitSlot,
                            array_size as u32,
                            3, // 8-byte alignment
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
                                struct_defs,
                            )?;

                            // Calculate offset (i * 8 bytes)
                            let offset = (i * 8) as i32;

                            // Store value at array[i]
                            builder.ins().store(
                                MemFlags::trusted(),
                                elem_value,
                                array_addr,
                                offset,
                            );
                        }

                        // Return the array address
                        Ok(array_addr)
                    }
                    Some(count_expr) => {
                        // Repeat form: vec![value; count]
                        // Compile the value expression once
                        let value = Self::compile_expr_with_context(
                            &elements[0],
                            builder,
                            parameters,
                            local_vars,
                            var_counter,
                            compiled_functions,
                            string_ctx,
                            struct_defs,
                        )?;

                        // Compile the count expression
                        let count = Self::compile_expr_with_context(
                            count_expr,
                            builder,
                            parameters,
                            local_vars,
                            var_counter,
                            compiled_functions,
                            string_ctx,
                            struct_defs,
                        )?;

                        // For MVP, use a fixed max size since dynamic allocation is complex
                        // Allocate max 1024 elements (8192 bytes)
                        let max_size = 1024 * 8;
                        let stack_slot = builder.create_sized_stack_slot(StackSlotData::new(
                            StackSlotKind::ExplicitSlot,
                            max_size,
                            3, // 8-byte alignment
                        ));

                        // Get address of stack slot
                        let array_addr = builder.ins().stack_addr(types::I64, stack_slot, 0);

                        // Create loop to fill array: for i in 0..count
                        let loop_header = builder.create_block();
                        let loop_body = builder.create_block();
                        let loop_exit = builder.create_block();

                        // Loop variable i (block parameter)
                        builder.append_block_param(loop_header, types::I64);

                        // Initialize i = 0 and jump to loop header
                        let i_init = builder.ins().iconst(types::I64, 0);
                        builder.ins().jump(loop_header, &[i_init]);

                        // Loop header: check if i < count
                        builder.switch_to_block(loop_header);
                        let i = builder.block_params(loop_header)[0];
                        let i_lt_count = builder.ins().icmp(IntCC::SignedLessThan, i, count);
                        builder
                            .ins()
                            .brif(i_lt_count, loop_body, &[], loop_exit, &[]);

                        // Loop body: store value at array[i]
                        builder.switch_to_block(loop_body);
                        builder.seal_block(loop_body);

                        // Calculate byte offset: i * 8
                        let eight = builder.ins().iconst(types::I64, 8);
                        let byte_offset = builder.ins().imul(i, eight);
                        let elem_addr = builder.ins().iadd(array_addr, byte_offset);

                        // Store value at array[i]
                        builder
                            .ins()
                            .store(MemFlags::trusted(), value, elem_addr, 0);

                        // Increment i and jump back to loop header
                        let one = builder.ins().iconst(types::I64, 1);
                        let i_next = builder.ins().iadd(i, one);
                        builder.ins().jump(loop_header, &[i_next]);

                        // Now seal loop_header after all predecessors are known
                        builder.seal_block(loop_header);

                        // Loop exit: return array address
                        builder.switch_to_block(loop_exit);
                        builder.seal_block(loop_exit);

                        Ok(array_addr)
                    }
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
