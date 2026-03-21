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

mod compiler;

use crate::interpreter::parser::AstNode;

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
pub(crate) struct StringContext<'a> {
    /// String literals storage (kept alive for JIT lifetime)
    pub(crate) literals: &'a mut Vec<Box<[u8]>>,
    /// String interning map (content → pointer)
    pub(crate) intern: &'a mut HashMap<String, i64>,
}

impl<'a> StringContext<'a> {
    /// Intern a string literal (reuse existing pointer if available)
    pub(crate) fn intern_string(&mut self, s: &str) -> i64 {
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
    pub(crate) module: JITModule,
    /// Function builder context (reused across compilations)
    pub(crate) builder_context: FunctionBuilderContext,
    /// Compiled function counter (for unique names)
    pub(crate) function_counter: usize,
    /// Compiled functions cache (name → pointer)
    pub(crate) compiled_functions: HashMap<String, *const u8>,
    /// String literals storage (kept alive for JIT lifetime)
    /// Format: Vec<Box<[u8]>> where each entry is [i64 length][data bytes]
    pub(crate) string_literals: Vec<Box<[u8]>>,
    /// String interning map (content → pointer) for string literal deduplication
    pub(crate) string_intern: HashMap<String, i64>,
    /// Struct definitions (name → ordered field names)
    pub(crate) struct_defs: HashMap<String, Vec<String>>,
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

}
