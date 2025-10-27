# WASM-002: Closure Compilation - GREEN Phase Implementation Plan

## Overview

This document outlines the GREEN phase implementation plan for WASM-002: Closure Compilation. After successfully completing the RED phase, we now need to implement the minimum viable functionality required to make our tests pass.

## Implementation Components

### 1. Closure Environment

The `ClosureEnvironment` class will manage information about captured variables in a closure:

```rust
struct CapturedVariable {
    name: String,
    type_name: String,
    offset: i32,
    size: i32,
}

struct ClosureEnvironment {
    captured_vars: Vec<CapturedVariable>,
    total_size: i32,
}

impl ClosureEnvironment {
    // Creates a new empty closure environment
    fn new() -> ClosureEnvironment;
    
    // Adds a captured variable to the environment
    fn add_variable(&mut self, name: String, type_name: String, size: i32) -> i32;
    
    // Gets the offset of a captured variable
    fn get_offset(&self, name: &String) -> Option<i32>;
    
    // Gets the total size of the environment in bytes
    fn size(&self) -> i32;
    
    // Gets the number of captured variables
    fn variable_count(&self) -> i32;
}
```

### 2. Closure Compiler

The `ClosureCompiler` class will handle compilation of closures to WebAssembly:

```rust
struct ClosureCompiler {
    next_function_index: i32,
    function_implementations: Vec<ClosureImplementation>,
}

struct ClosureImplementation {
    function_index: i32,
    environment: ClosureEnvironment,
    param_types: Vec<Type>,
    result_type: Type,
    body: String,
}

impl ClosureCompiler {
    // Creates a new closure compiler
    fn new() -> ClosureCompiler;
    
    // Creates a new closure environment
    fn create_environment(&mut self) -> ClosureEnvironment;
    
    // Adds a captured variable to the environment
    fn add_capture(&self, env: &mut ClosureEnvironment, 
                   name: String, type_name: String, offset: i32);
    
    // Gets the total size of the environment in bytes
    fn environment_size(&self, env: &ClosureEnvironment) -> i32;
    
    // Gets the number of captured variables
    fn capture_count(&self, env: &ClosureEnvironment) -> i32;
    
    // Gets the offset of a captured variable
    fn capture_offset(&self, env: &ClosureEnvironment, name: String) -> i32;
    
    // Adds a closure implementation
    fn add_implementation(&mut self, env: ClosureEnvironment, 
                         param_types: Vec<Type>, result_type: Type, body: String) -> i32;
    
    // Generates code for allocating a closure
    fn generate_allocation(&self, env: &ClosureEnvironment, function_index: i32) -> String;
    
    // Generates code for calling a closure
    fn generate_call(&self, env: &ClosureEnvironment, 
                    param_types: Vec<Type>, result_type: Type) -> String;
                    
    // Gets all closure implementations
    fn get_implementations(&self) -> &Vec<ClosureImplementation>;
}
```

### 3. WASM Emitter Extensions

We will extend the `WasmEmitter` class from WASM-001 with functionality for handling closures:

```rust
impl WasmEmitter {
    // Adds closure-related functionality to the WasmEmitter
    
    // Creates a closure compiler
    fn create_closure_compiler(&mut self) -> ClosureCompiler;
    
    // Processes a function body and extracts closures
    fn process_closures(&mut self, body: &String) -> Vec<ClosureImplementation>;
    
    // Adds a closure implementation to the module
    fn add_closure_implementation(&mut self, implementation: &ClosureImplementation);
    
    // Generates function table entries for closures
    fn generate_function_table(&mut self);
}
```

## Implementation Approach

### Phase 1: Closure Environment Implementation (Day 1)

1. Implement the `ClosureEnvironment` class
   - Store information about captured variables
   - Calculate offsets and sizes
   - Manage variable tracking

2. Implement basic `ClosureCompiler` functionality
   - Environment creation and management
   - Captured variable handling

### Phase 2: Code Generation for Closures (Day 1-2)

1. Implement allocation code generation
   - Generate code for allocating closure records
   - Generate code for storing function index
   - Generate code for storing captured variables

2. Implement call code generation
   - Generate code for indirect function calls
   - Generate code for passing closure record and parameters
   - Handle return values

### Phase 3: WasmEmitter Integration (Day 2-3)

1. Extend WasmEmitter with closure support
   - Add function table generation
   - Add closure implementation handling
   - Process function bodies for closures

2. Implement closure detection and extraction
   - Identify closures in function bodies
   - Extract captured variables
   - Generate implementation functions

### Phase 4: Module Integration (Day 3)

1. Integrate all components
   - Connect closure compiler to WasmEmitter
   - Ensure consistent function indexing
   - Connect code generation to module output

2. Implement WebAssembly output
   - Generate complete WAT with closures
   - Ensure proper module structure
   - Validate generated modules

## Test-Driven Implementation

For each component, we'll follow this approach:

1. Start with the simplest tests (basic counter closure)
2. Implement minimum code to pass the test
3. Move to more complex tests (nested closures, multiple captures)
4. Gradually implement all required functionality
5. Ensure all tests pass with minimal implementation

## Success Criteria

The GREEN phase is considered successful when:

1. All tests in `test_closure_compilation_red.ruchy` pass
2. The implementation is minimal but complete
3. The generated WebAssembly correctly handles closures
4. All closure scenarios are properly supported:
   - Basic closures
   - Nested closures
   - Multiple captures
   - Closures as arguments
   - Closures in data structures

## Timeline

- Day 1: Implement closure environment and basic compiler functionality
- Day 2: Implement code generation and WasmEmitter integration
- Day 3: Complete module integration and ensure all tests pass
- Day 4: Fix any remaining issues and finalize the implementation

Total estimated time: 3-4 days for the GREEN phase