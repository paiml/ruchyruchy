# WASM-002: Closure Compilation - RED Phase Implementation

## Overview

This document outlines the RED phase implementation for WASM-002: Closure Compilation. In accordance with our Extreme TDD methodology, this phase focuses on creating failing tests that define the expected behavior of the closure compilation system before implementing the actual solution.

## Files Created

### Test Files
- `validation/wasm/test_closure_compilation_red.ruchy`: Contains comprehensive tests for closure compilation
- `validation/wasm/test_closure_red_runner.ruchy`: Verifies that our tests fail as expected before implementation

### Implementation Skeleton
- `bootstrap/stage3/wasm_closure.ruchy`: Empty skeleton file that will be implemented in the GREEN phase

## Test Coverage

The RED phase tests cover the following key functionality:

1. **Basic Closures**: Simple closures that capture a single variable
   - Example: Counter closure that increments a captured variable

2. **Nested Closures**: Closures that create other closures
   - Example: Function factory that creates adder functions

3. **Multiple Captures**: Closures that capture multiple variables
   - Example: Counter with configurable step size

4. **Closures as Arguments**: Functions that take closures as parameters
   - Example: Apply-twice function that applies a function twice

5. **Closures in Data Structures**: Storing closures in arrays or other structures
   - Example: Array of different calculation functions

6. **Closure Environment**: Management of captured variables
   - Creating environments
   - Adding captures
   - Calculating offsets and sizes

7. **Closure Type Mapping**: Mapping Ruchy closure types to WASM
   - Memory layout for closure records
   - Type mapping for parameters and return values

8. **Code Generation**: Generating WASM code for closures
   - Allocation code for closure records
   - Call code for invoking closures

## Implementation Interface

The tests define the following interface for closure compilation:

### ClosureCompiler

```rust
struct ClosureCompiler {
    // Implementation details
}

impl ClosureCompiler {
    // Creates a new closure compiler
    fn new() -> ClosureCompiler;
    
    // Creates a new closure environment
    fn create_environment(&self) -> ClosureEnvironment;
    
    // Adds a captured variable to the environment
    fn add_capture(&self, env: &mut ClosureEnvironment, 
                   name: String, type_name: String, offset: i32);
    
    // Gets the total size of the environment in bytes
    fn environment_size(&self, env: &ClosureEnvironment) -> i32;
    
    // Gets the number of captured variables
    fn capture_count(&self, env: &ClosureEnvironment) -> i32;
    
    // Gets the offset of a captured variable
    fn capture_offset(&self, env: &ClosureEnvironment, name: String) -> i32;
    
    // Generates code for allocating a closure
    fn generate_allocation(&self, env: &ClosureEnvironment, 
                          function_index: i32) -> String;
    
    // Generates code for calling a closure
    fn generate_call(&self, env: &ClosureEnvironment, 
                     param_types: Vec<Type>, result_type: Type) -> String;
}
```

### ClosureEnvironment

```rust
struct ClosureEnvironment {
    // Stores information about captured variables
}
```

### WasmEmitter Extensions

The WASM emitter will need to be extended with functionality for:

1. Detecting closures in Ruchy code
2. Creating closure implementation functions
3. Allocating closure records
4. Setting up function tables for indirect calls

## RED Phase Verification

The `test_closure_red_runner.ruchy` file verifies that our tests fail as expected in the RED phase, since we haven't implemented the actual functionality yet. This ensures our tests are properly set up and will detect when the implementation is completed.

Expected failure reasons:
- Missing `ClosureCompiler` implementation
- Missing `ClosureEnvironment` implementation
- Missing extensions to `WasmEmitter`

## Closure Compilation Strategy

Based on our validation spike, we will implement closure compilation using the closure record approach:

1. **Closure Records**: Each closure is represented by a record in linear memory
   - Function index (pointer to implementation function)
   - Captured variables

2. **Implementation Functions**: Each unique closure gets a dedicated implementation function
   - Takes closure record pointer as first parameter
   - Accesses captured variables through the closure record

3. **Function Tables**: Use WASM function tables for indirect calls
   - Store function implementations in a table
   - Call functions indirectly through the table

4. **Allocation**: Generate code to allocate and initialize closure records
   - Allocate memory for the record
   - Store function index
   - Store captured variables

## Next Steps

In the GREEN phase, we will implement the minimum viable functionality required to make all the tests pass. This will include:

1. Implementing the `ClosureCompiler` and `ClosureEnvironment` classes
2. Extending the `WasmEmitter` to handle closures
3. Implementing code generation for closure allocation and calls
4. Implementing memory layout calculations for closure records

## Success Criteria

The RED phase is considered successful when:
1. All tests in `test_closure_compilation_red.ruchy` are properly defined
2. `test_closure_red_runner.ruchy` passes, confirming the expected failures
3. The test suite provides comprehensive coverage of closure compilation functionality

This RED phase implementation establishes a clear target for the GREEN phase implementation to follow.