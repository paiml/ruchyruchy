# WASM-002: Closure Compilation - RED Phase Implementation Plan

## Overview

This document outlines the RED phase implementation plan for WASM-002: Closure Compilation. This feature builds on the WebAssembly type mapping system implemented in WASM-001, focusing specifically on compiling Ruchy closures to WebAssembly.

## Background

Closures in Ruchy allow functions to capture variables from their enclosing scope. WebAssembly does not have native support for closures, so we need to implement a mechanism to represent closures in WASM.

Based on our validation spike, we determined that the most effective approach is to use closure records in linear memory. This approach:

1. Represents closures as records in linear memory
2. Stores the function index and captured variables in the record
3. Passes the closure record pointer to the function when called
4. Accesses captured variables through the closure record

## RED Phase Implementation Plan

The RED phase will focus on creating failing tests that define the expected behavior of the closure compilation system.

### 1. Test Cases to Create

#### Basic Closure Tests

```ruchy
// Test a simple counter closure
fun test_counter_closure() {
    // Test that we can create a closure that captures a variable
    let counter = make_counter();
    assert_eq(counter(), 1);
    assert_eq(counter(), 2);
    assert_eq(counter(), 3);
}

// Helper function to create a counter
fun make_counter() {
    let mut count = 0;
    return () => {
        count = count + 1;
        count
    };
}
```

#### Nested Closure Tests

```ruchy
// Test nested closures
fun test_nested_closures() {
    // Test that we can create nested closures that capture variables
    let make_adder = make_adder_factory();
    let add5 = make_adder(5);
    let add10 = make_adder(10);
    
    assert_eq(add5(2), 7);
    assert_eq(add10(2), 12);
}

// Helper function to create a factory for adder closures
fun make_adder_factory() {
    return (n) => {
        return (x) => x + n;
    };
}
```

#### Multiple Capture Tests

```ruchy
// Test closures that capture multiple variables
fun test_multiple_captures() {
    // Test that we can create a closure that captures multiple variables
    let (increment, get_values) = create_counter_pair(10, 5);
    
    increment();
    increment();
    let (count, step) = get_values();
    
    assert_eq(count, 12);
    assert_eq(step, 5);
}

// Helper function to create a pair of related closures
fun create_counter_pair(initial, step) {
    let mut count = initial;
    
    let increment = () => {
        count = count + step;
    };
    
    let get_values = () => {
        (count, step)
    };
    
    (increment, get_values)
}
```

#### Closure as Argument Tests

```ruchy
// Test passing closures as arguments
fun test_closure_as_argument() {
    // Test that we can pass closures as arguments
    let double = (x) => x * 2;
    let triple = (x) => x * 3;
    
    let result1 = apply_twice(double, 3);
    let result2 = apply_twice(triple, 3);
    
    assert_eq(result1, 12);  // double(double(3)) = double(6) = 12
    assert_eq(result2, 27);  // triple(triple(3)) = triple(9) = 27
}

// Helper function that applies a function twice
fun apply_twice(f, x) {
    return f(f(x));
}
```

#### Closure in Data Structure Tests

```ruchy
// Test storing closures in data structures
fun test_closure_in_data_structure() {
    // Test that we can store closures in data structures
    let callbacks = [
        (x) => x * 2,
        (x) => x + 10,
        (x) => x * x
    ];
    
    assert_eq(callbacks[0](5), 10);
    assert_eq(callbacks[1](5), 15);
    assert_eq(callbacks[2](5), 25);
}
```

### 2. WASM Emitter Extensions to Test

We will extend the WASM emitter with the following functionality:

#### Closure Compilation Functions

```ruchy
// Test the closure compilation functions
fun test_closure_compilation_functions() {
    let emitter = WasmEmitter::new();
    
    // Test creating closure environment
    let env = emitter.create_closure_environment();
    assert(env != null, "Closure environment should be created");
    
    // Test adding captured variables to environment
    emitter.add_capture_to_environment(env, "count", "i32");
    assert_eq(emitter.get_environment_size(env), 4, "Environment should have one i32 variable (4 bytes)");
    
    // Test generating closure call
    let call_instr = emitter.generate_closure_call(env, ["i32"], "i32");
    assert(call_instr != null, "Should generate closure call instruction");
}
```

#### Closure Type Mapping

```ruchy
// Test the closure type mapping
fun test_closure_type_mapping() {
    // Test mapping closure type to WASM
    let closure_type = RuchyWasmType::new_closure(
        [RuchyWasmType::new("i32")],
        RuchyWasmType::new("i32"),
        [RuchyWasmType::new("i32"), RuchyWasmType::new("f64")]
    );
    
    // Check memory layout
    let layout = closure_type.memory_layout();
    assert_eq(layout.size(), 16, "Closure size should be 4 (func idx) + 4 (i32) + 8 (f64)");
    
    // Check field offsets
    assert_eq(layout.field_offset("function_index"), 0, "Function index should be at offset 0");
    assert_eq(layout.field_offset("capture_0"), 4, "First capture should be at offset 4");
    assert_eq(layout.field_offset("capture_1"), 8, "Second capture should be at offset 8");
}
```

#### Closure Code Generation

```ruchy
// Test generating WASM code for closures
fun test_closure_code_generation() {
    let emitter = WasmEmitter::new();
    
    // Add a function that creates and returns a closure
    emitter.add_function("make_counter", [], "closure", `
        let mut count = 0;
        return () => {
            count = count + 1;
            count
        };
    `);
    
    // Generate the module
    let module = emitter.generate_module();
    
    // Verify the module contains:
    // 1. A function for creating closures
    assert(module.has_function("make_counter"), "Module should have make_counter function");
    
    // 2. A function for the closure implementation
    assert(module.has_function("closure_impl_0"), "Module should have closure implementation function");
    
    // 3. Code for allocating and initializing the closure record
    let wat = emitter.emit_wat();
    assert(wat.contains("call $malloc"), "WAT should include memory allocation for closure");
    assert(wat.contains("i32.store"), "WAT should include storing function index");
}
```

### 3. File Structure

We will create the following files for the RED phase:

- `/validation/wasm/test_closure_compilation_red.ruchy`: Tests for closure compilation
- `/bootstrap/stage3/wasm_closure.ruchy`: Stub file for closure compilation implementation
- `/docs/research/WASM_002_CLOSURE_RED_IMPLEMENTATION.md`: Documentation of RED phase implementation

### 4. Implementation Approach

The RED phase will:

1. Define the expected interface for closure compilation
2. Create comprehensive tests for various closure scenarios
3. Verify that the tests fail as expected (since implementation doesn't exist yet)
4. Document the requirements and expected behavior

### 5. Validation Strategy

We will create a test runner that:

1. Attempts to run the closure compilation tests
2. Verifies that they fail with the expected error messages
3. Confirms that the failures are due to missing implementation, not test errors

## Success Criteria

The RED phase is successful when:

1. All test cases are defined and properly test closure functionality
2. The tests fail in the expected way (missing implementation, not test errors)
3. The interface for closure compilation is clearly defined
4. The requirements and expected behavior are well-documented

## Timeline

- Day 1: Define test cases and interface for closure compilation
- Day 2: Implement test runner and verify test failures
- Day 3: Document requirements and expected behavior

Total estimated time: 2-3 days for the RED phase