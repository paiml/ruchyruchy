# WASM-003: Multi-Target Integration - RED Phase Plan

## Overview

This document outlines the plan for the RED phase of the WASM-003: Multi-Target Integration ticket. This phase focuses on creating a unified interface for compiling Ruchy code to multiple targets (WebAssembly, TypeScript, and Rust) and defining comprehensive tests that will initially fail due to the missing implementation.

## Objectives

The main objectives for the RED phase of WASM-003 are:

1. Define a unified interface for multi-target compilation
2. Create a common abstraction layer for code generation
3. Design mechanisms for target-specific optimizations
4. Develop tests that verify consistent behavior across targets
5. Establish infrastructure for cross-target validation

## Design Approach

### 1. Unified Compilation Interface

```rust
/// Multi-target compiler interface
struct MultiTargetCompiler {
    /// Available compilation targets
    targets: Vec<CompilationTarget>,
    
    /// Current active target
    active_target: Option<CompilationTarget>,
    
    /// Shared AST representation
    ast: Option<Box<Ast>>,
    
    /// Shared type environment
    type_env: TypeEnvironment,
}

/// Supported compilation targets
enum CompilationTarget {
    WebAssembly,
    TypeScript,
    Rust,
}

impl MultiTargetCompiler {
    /// Create a new multi-target compiler
    fn new() -> MultiTargetCompiler;
    
    /// Set the active compilation target
    fn set_target(&mut self, target: CompilationTarget) -> Result<(), CompilationError>;
    
    /// Parse source code to shared AST
    fn parse(&mut self, source: String) -> Result<Box<Ast>, CompilationError>;
    
    /// Type check the AST
    fn type_check(&mut self) -> Result<(), CompilationError>;
    
    /// Compile to the active target
    fn compile(&mut self) -> Result<CompiledOutput, CompilationError>;
    
    /// Compile to all targets
    fn compile_all(&mut self) -> HashMap<CompilationTarget, Result<CompiledOutput, CompilationError>>;
}
```

### 2. Target-Specific Emitters

```rust
/// Common interface for all target emitters
trait TargetEmitter {
    /// Initialize the emitter
    fn initialize(&mut self) -> Result<(), CompilationError>;
    
    /// Emit code for a module
    fn emit_module(&mut self, module: &Module) -> Result<String, CompilationError>;
    
    /// Emit code for a function
    fn emit_function(&mut self, function: &Function) -> Result<String, CompilationError>;
    
    /// Emit code for a type definition
    fn emit_type(&mut self, type_def: &TypeDefinition) -> Result<String, CompilationError>;
    
    /// Emit code for an expression
    fn emit_expression(&mut self, expr: &Expression) -> Result<String, CompilationError>;
    
    /// Get target-specific options
    fn get_options(&self) -> EmitterOptions;
    
    /// Set target-specific options
    fn set_options(&mut self, options: EmitterOptions) -> Result<(), CompilationError>;
}

/// WebAssembly emitter
struct WasmEmitter {
    // Reuse existing implementation from WASM-001 and WASM-002
}

/// TypeScript emitter
struct TypeScriptEmitter {
    // Reuse existing implementation
}

/// Rust emitter
struct RustEmitter {
    // Reuse existing implementation
}
```

### 3. Shared Code Generation Abstractions

```rust
/// Abstract representation of a compilation unit
struct CompilationUnit {
    /// Module name
    name: String,
    
    /// AST representation
    ast: Box<Ast>,
    
    /// Type information
    type_info: TypeEnvironment,
    
    /// Dependencies
    dependencies: Vec<Dependency>,
    
    /// Compilation options
    options: CompilationOptions,
}

/// Code generation pipeline
struct CodeGenerationPipeline {
    /// Transformation passes
    passes: Vec<Box<dyn TransformationPass>>,
    
    /// Target emitter
    emitter: Box<dyn TargetEmitter>,
    
    /// Pipeline options
    options: PipelineOptions,
}

/// Transformation pass interface
trait TransformationPass {
    /// Apply transformation to a compilation unit
    fn apply(&self, unit: &mut CompilationUnit) -> Result<(), CompilationError>;
    
    /// Check if the pass is applicable to the given target
    fn is_applicable(&self, target: CompilationTarget) -> bool;
    
    /// Get the pass priority (lower runs earlier)
    fn priority(&self) -> i32;
}
```

### 4. Cross-Target Testing Framework

```rust
/// Cross-target test case
struct CrossTargetTest {
    /// Test name
    name: String,
    
    /// Source code
    source: String,
    
    /// Expected results for each target
    expected_results: HashMap<CompilationTarget, ExpectedResult>,
    
    /// Test options
    options: TestOptions,
}

/// Expected result for a target
struct ExpectedResult {
    /// Expected output
    output: String,
    
    /// Expected error (if failure is expected)
    error: Option<String>,
    
    /// Expected warnings
    warnings: Vec<String>,
    
    /// Target-specific assertions
    assertions: Vec<TestAssertion>,
}

/// Test framework
struct CrossTargetTestFramework {
    /// Test cases
    tests: Vec<CrossTargetTest>,
    
    /// Compiler instance
    compiler: MultiTargetCompiler,
    
    /// Test options
    options: TestFrameworkOptions,
    
    /// Run tests for specific targets
    fn run(&mut self, targets: Vec<CompilationTarget>) -> TestResults;
    
    /// Run all tests for all targets
    fn run_all(&mut self) -> TestResults;
}
```

## RED Phase Test Cases

### 1. Basic Compilation Tests

Tests for compiling simple Ruchy code to all targets:

```rust
fn test_basic_compilation() {
    let source = "fun main() { println(\"Hello, World!\"); }";
    
    // Create compiler
    let mut compiler = MultiTargetCompiler::new();
    
    // Parse source
    let ast = compiler.parse(source)?;
    
    // Type check
    compiler.type_check()?;
    
    // Compile to all targets
    let results = compiler.compile_all();
    
    // Check WebAssembly result
    let wasm_result = results.get(&CompilationTarget::WebAssembly)?;
    assert(wasm_result.contains("(module"), "WebAssembly output should contain module declaration");
    
    // Check TypeScript result
    let ts_result = results.get(&CompilationTarget::TypeScript)?;
    assert(ts_result.contains("function main()"), "TypeScript output should contain main function");
    
    // Check Rust result
    let rust_result = results.get(&CompilationTarget::Rust)?;
    assert(rust_result.contains("fn main()"), "Rust output should contain main function");
}
```

### 2. Closure Tests

Tests for compiling closures to all targets:

```rust
fn test_closure_compilation() {
    let source = "fun make_counter() { 
        let mut count = 0;
        return () => {
            count = count + 1;
            count
        };
    }";
    
    // Create compiler
    let mut compiler = MultiTargetCompiler::new();
    
    // Parse source
    let ast = compiler.parse(source)?;
    
    // Type check
    compiler.type_check()?;
    
    // Compile to all targets
    let results = compiler.compile_all();
    
    // Check WebAssembly result
    let wasm_result = results.get(&CompilationTarget::WebAssembly)?;
    assert(wasm_result.contains("call $alloc"), 
           "WebAssembly output should allocate closure");
    
    // Check TypeScript result
    let ts_result = results.get(&CompilationTarget::TypeScript)?;
    assert(ts_result.contains("function make_counter()"), 
           "TypeScript output should contain make_counter function");
    assert(ts_result.contains("return () =>"), 
           "TypeScript output should contain closure");
    
    // Check Rust result
    let rust_result = results.get(&CompilationTarget::Rust)?;
    assert(rust_result.contains("move ||"), 
           "Rust output should use move closure");
}
```

### 3. Type System Tests

Tests for handling types consistently across targets:

```rust
fn test_type_consistency() {
    let source = "struct Point { x: f64, y: f64 }
                 fun calculate_distance(p1: Point, p2: Point) -> f64 {
                     let dx = p2.x - p1.x;
                     let dy = p2.y - p1.y;
                     return (dx * dx + dy * dy).sqrt();
                 }";
    
    // Create compiler
    let mut compiler = MultiTargetCompiler::new();
    
    // Parse source
    let ast = compiler.parse(source)?;
    
    // Type check
    compiler.type_check()?;
    
    // Compile to all targets
    let results = compiler.compile_all();
    
    // Check WebAssembly result
    let wasm_result = results.get(&CompilationTarget::WebAssembly)?;
    assert(wasm_result.contains("f64"), 
           "WebAssembly output should use f64 type");
    
    // Check TypeScript result
    let ts_result = results.get(&CompilationTarget::TypeScript)?;
    assert(ts_result.contains("interface Point"), 
           "TypeScript output should define Point interface");
    
    // Check Rust result
    let rust_result = results.get(&CompilationTarget::Rust)?;
    assert(rust_result.contains("struct Point"), 
           "Rust output should define Point struct");
}
```

### 4. Feature Compatibility Tests

Tests for features that work across all targets:

```rust
fn test_feature_compatibility() {
    // Test various language features that should work on all targets
    let features = [
        // Closures
        "fun test_closure() { return (x) => x * 2; }",
        
        // Pattern matching
        "fun test_pattern(opt: Option<i32>) -> i32 { 
            match opt {
                Some(x) => x,
                None => 0,
            }
        }",
        
        // Error handling
        "fun test_error() -> Result<i32, String> {
            do {
                return Ok(42);
            } catch e {
                return Err(e.to_string());
            }
        }"
    ];
    
    // Create compiler
    let mut compiler = MultiTargetCompiler::new();
    
    for feature in features {
        // Parse source
        let ast = compiler.parse(feature)?;
        
        // Type check
        compiler.type_check()?;
        
        // Compile to all targets
        let results = compiler.compile_all();
        
        // All targets should produce valid output
        for (target, result) in results {
            assert(result.is_ok(), format!("Feature should compile for {}", target));
        }
    }
}
```

### 5. Integration Tests

Tests for integrating with target-specific ecosystems:

```rust
fn test_target_integration() {
    // Test code with target-specific imports/integrations
    let source = "
        #[target(wasm)]
        import wasm_bindgen::prelude::*;
        
        #[target(typescript)]
        import { useState } from 'react';
        
        #[target(rust)]
        use std::collections::HashMap;
        
        fun main() {
            #[target(wasm)]
            js_log(\"Hello from WebAssembly\");
            
            #[target(typescript)]
            console.log(\"Hello from TypeScript\");
            
            #[target(rust)]
            println!(\"Hello from Rust\");
        }
    ";
    
    // Create compiler
    let mut compiler = MultiTargetCompiler::new();
    
    // Parse source
    let ast = compiler.parse(source)?;
    
    // Type check
    compiler.type_check()?;
    
    // Compile to all targets
    let results = compiler.compile_all();
    
    // Check target-specific imports
    let wasm_result = results.get(&CompilationTarget::WebAssembly)?;
    assert(wasm_result.contains("wasm_bindgen"), 
           "WebAssembly output should include wasm_bindgen");
    
    let ts_result = results.get(&CompilationTarget::TypeScript)?;
    assert(ts_result.contains("import { useState } from 'react'"), 
           "TypeScript output should include React import");
    
    let rust_result = results.get(&CompilationTarget::Rust)?;
    assert(rust_result.contains("use std::collections::HashMap"), 
           "Rust output should include HashMap import");
}
```

## Implementation Plan

### 1. Interface Definition

Create the core interfaces for multi-target compilation:

1. Define `MultiTargetCompiler` interface
2. Define `TargetEmitter` trait and implementations
3. Define shared code generation abstractions
4. Create cross-target testing framework

### 2. Test Implementation

Implement comprehensive RED phase tests:

1. Basic compilation tests
2. Closure compilation tests
3. Type system tests
4. Feature compatibility tests
5. Integration tests

### 3. Documentation

Document the design and test cases:

1. Create interface documentation
2. Document test cases with expected behavior
3. Create RED phase completion report

## Success Criteria

The RED phase will be considered successful when:

1. All interfaces are clearly defined
2. All test cases are implemented and properly failing
3. The failure modes are well-documented and understood
4. The plan for the GREEN phase is clear

## Next Steps

After completing the RED phase, the next steps will be:

1. **GREEN Phase**: Implement the multi-target integration
2. **REFACTOR Phase**: Improve the implementation for better organization and efficiency
3. **TOOL Phase**: Validate the implementation with Ruchy tools

## Conclusion

The RED phase for WASM-003: Multi-Target Integration will establish a solid foundation for integrating WebAssembly compilation with existing TypeScript and Rust targets. By defining clear interfaces and comprehensive tests upfront, we ensure that the subsequent implementation will meet all requirements and maintain consistency across targets.