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

use crate::interpreter::parser::{AstNode, BinaryOperator};

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
        })
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
            let result = Self::compile_expr(ast, &mut builder)?;

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
            let result = Self::compile_expr_with_vars(body, &mut builder, &variables)?;

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

        // Return as generic function pointer (caller must cast to correct type)
        let func: T = unsafe { std::mem::transmute_copy(&code_ptr) };
        Ok(func)
    }

    /// Compile AST expression to Cranelift IR value (no variables)
    fn compile_expr(ast: &AstNode, builder: &mut FunctionBuilder) -> Result<Value, JitError> {
        let mut var_counter = 0;
        Self::compile_expr_with_context(
            ast,
            builder,
            &HashMap::new(),
            &mut HashMap::new(),
            &mut var_counter,
        )
    }

    /// Compile AST expression to Cranelift IR value (with variable context)
    fn compile_expr_with_vars(
        ast: &AstNode,
        builder: &mut FunctionBuilder,
        variables: &HashMap<String, Value>,
    ) -> Result<Value, JitError> {
        let mut var_counter = 0;
        Self::compile_expr_with_context(ast, builder, variables, &mut HashMap::new(), &mut var_counter)
    }

    /// Compile AST expression to Cranelift IR value (with full context)
    fn compile_expr_with_context(
        ast: &AstNode,
        builder: &mut FunctionBuilder,
        parameters: &HashMap<String, Value>,
        local_vars: &mut HashMap<String, Variable>,
        var_counter: &mut usize,
    ) -> Result<Value, JitError> {
        match ast {
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
                    )?;
                }
                Ok(result)
            }

            // Let declaration: create and initialize a variable
            AstNode::LetDecl { name, value } => {
                // Create a new Cranelift variable
                let var = Variable::new(*var_counter);
                *var_counter += 1;

                // Declare it in the builder
                builder.declare_var(var, types::I64);

                // Compile the initial value
                let init_value = Self::compile_expr_with_context(
                    value,
                    builder,
                    parameters,
                    local_vars,
                    var_counter,
                )?;

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
                let lhs = Self::compile_expr_with_context(left, builder, parameters, local_vars, var_counter)?;
                let rhs = Self::compile_expr_with_context(right, builder, parameters, local_vars, var_counter)?;

                let result = match op {
                    // Arithmetic operators
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

                let cond_value = Self::compile_expr_with_context(condition, builder, parameters, local_vars, var_counter)?;

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
                    Self::compile_expr_with_context(stmt, builder, parameters, local_vars, var_counter)?;
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
                let cond_value = Self::compile_expr_with_context(condition, builder, parameters, local_vars, var_counter)?;

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
                        result = Self::compile_expr_with_context(stmt, builder, parameters, local_vars, var_counter)?;
                    }
                    result
                };

                builder.ins().jump(merge_block, &[then_result]);

                // Compile else branch
                builder.switch_to_block(else_block);
                builder.seal_block(else_block);

                let else_result = if let Some(else_stmts) = else_branch {
                    if else_stmts.is_empty() {
                        builder.ins().iconst(types::I64, 0)
                    } else {
                        let mut result = builder.ins().iconst(types::I64, 0);
                        for stmt in else_stmts {
                            result = Self::compile_expr_with_context(stmt, builder, parameters, local_vars, var_counter)?;
                        }
                        result
                    }
                } else {
                    builder.ins().iconst(types::I64, 0)
                };

                builder.ins().jump(merge_block, &[else_result]);

                // Continue at merge block
                builder.switch_to_block(merge_block);
                builder.seal_block(merge_block);

                // Get the result from the merge block parameter
                let result = builder.block_params(merge_block)[0];

                Ok(result)
            }

            // Unsupported AST node
            _ => Err(JitError::UnsupportedNode(format!(
                "Cannot compile AST node: {:?}",
                ast
            ))),
        }
    }
}
