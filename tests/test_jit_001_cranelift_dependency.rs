// JIT-001: Cranelift Dependency Integration
//
// EXTREME TDD - RED Phase
//
// Mission: Add Cranelift JIT compiler dependencies to enable JIT compilation
//
// Cranelift:
// - Rust-native code generation library
// - Fast compilation (vs LLVM's slow compilation)
// - Designed for JIT use cases (vs LLVM's AOT focus)
// - Safe Rust API (vs LLVM C API bindings)
//
// Why Cranelift over LLVM:
// 1. Faster compilation (10-100x vs LLVM) - critical for JIT
// 2. Pure Rust (no C dependencies, easier build)
// 3. Simpler API (designed for embedders)
// 4. Predictable compile times (no complex optimizations)
//
// Dependencies needed:
// - cranelift: Core code generation
// - cranelift-jit: JIT execution support
// - cranelift-module: Module management
// - cranelift-frontend: Frontend utilities (function builder)
//
// Method: Test-driven development with Cranelift smoke test

use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};

/// Test: Cranelift dependencies available
///
/// Validates that Cranelift crates can be imported and used
#[test]
fn test_cranelift_dependencies_available() {
    // Should be able to create a JIT builder
    let builder = JITBuilder::new(cranelift_module::default_libcall_names())
        .expect("Failed to create JIT builder");

    // Should be able to create a JIT module
    let module = JITModule::new(builder);

    // Should be able to get settings
    let isa = module.isa();
    assert!(isa.triple().architecture != target_lexicon::Architecture::Unknown);
}

/// Test: Create basic function with Cranelift
///
/// Validates that we can create a simple function using Cranelift IR
#[test]
fn test_create_basic_cranelift_function() {
    let builder = JITBuilder::new(cranelift_module::default_libcall_names())
        .expect("Failed to create JIT builder");
    let mut module = JITModule::new(builder);

    // Create a function signature: () -> i64
    let mut sig = module.make_signature();
    sig.returns.push(AbiParam::new(types::I64));

    // Declare the function
    let func_id = module
        .declare_function("test_func", Linkage::Export, &sig)
        .expect("Failed to declare function");

    // Define the function
    let mut ctx = module.make_context();
    ctx.func.signature = sig;

    {
        let mut builder_context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut ctx.func, &mut builder_context);

        // Create entry block
        let entry_block = builder.create_block();
        builder.append_block_params_for_function_params(entry_block);
        builder.switch_to_block(entry_block);
        builder.seal_block(entry_block);

        // Return constant 42
        let val = builder.ins().iconst(types::I64, 42);
        builder.ins().return_(&[val]);

        // Finalize function
        builder.finalize();
    }

    // Define the function in the module
    module
        .define_function(func_id, &mut ctx)
        .expect("Failed to define function");

    // Finalize definitions
    module.finalize_definitions().expect("Failed to finalize");

    // Get function pointer
    let code_ptr = module.get_finalized_function(func_id);

    // Call the function
    let func: fn() -> i64 = unsafe { std::mem::transmute(code_ptr) };
    let result = func();

    assert_eq!(result, 42, "Cranelift-compiled function should return 42");
}

/// Test: Cranelift version compatibility
///
/// Validates that we're using a compatible version of Cranelift
#[test]
fn test_cranelift_version() {
    // Create a simple module to verify API compatibility
    let builder = JITBuilder::new(cranelift_module::default_libcall_names())
        .expect("JIT builder creation should work with installed Cranelift version");

    let module = JITModule::new(builder);

    // If we can create a module, the API is compatible
    assert!(
        module.isa().triple().architecture != target_lexicon::Architecture::Unknown,
        "Cranelift should provide valid ISA information"
    );
}
