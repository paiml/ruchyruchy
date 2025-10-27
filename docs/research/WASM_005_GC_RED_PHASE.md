# WASM-005: WebAssembly GC Integration - RED Phase Plan

## Introduction

WebAssembly Garbage Collection (GC) is a critical extension to the WebAssembly specification that introduces support for managed objects and garbage collection directly within the WebAssembly environment. This feature enables languages with managed memory models (like Ruchy) to compile to WebAssembly with better performance characteristics and reduced complexity compared to implementing custom garbage collection in JavaScript or through other workarounds.

The GC proposal adds reference types, structs, arrays, and built-in garbage collection to WebAssembly, allowing for more efficient representation of high-level language constructs while maintaining memory safety. For Ruchy, this integration represents a significant advancement in our WASM compilation target capabilities.

## Scope of GC Integration for Ruchy

This ticket (WASM-005) covers the initial integration of WebAssembly GC features into the Ruchy compiler's WebAssembly backend. The scope includes:

1. Support for GC reference types
2. Implementation of struct and array type definitions
3. Operations on reference types (get, set, new, etc.)
4. Memory management integration for reference types
5. Type mapping from Ruchy types to appropriate WASM GC types

## Technical Requirements and Specifications

### Reference Types

- Implement `ref` and `ref null` type representations
- Support for type references (e.g., `ref $type`)
- Implementation of reference type checking operations
- Support for reference casting and subtyping

### Struct Types

- Define struct type declarations in the WASM module section
- Support for field access (get/set)
- Implementation of struct instantiation
- Field mutability controls

### Array Types

- Define array type declarations
- Support for element access (get/set)
- Implementation of array instantiation with initial size
- Support for array length queries

### Reference Operations

- `struct.new` / `array.new` operations
- `struct.get` / `array.get` operations
- `struct.set` / `array.set` operations
- Reference equality comparison
- Reference type casting

### Memory Management

- Integration with WASM GC's built-in garbage collection
- Proper handling of reference lifetimes
- Support for weak references when needed
- Strategies for managing cyclic references

## Failing Tests (RED Phase)

For the RED phase, we will implement a suite of failing tests that validate our WebAssembly GC integration. Each test will target a specific aspect of the GC features we need to support:

### Test 1: Basic Reference Type Definition and Usage

```ruchy
// File: validation/wasm_gc/test_reference_types_red.ruchy

fun test_reference_types() {
    // Test that we can define and use reference types
    let struct_type = wasm_struct_type([
        {name: "field1", type: "i32", mutable: true},
        {name: "field2", type: "f64", mutable: false}
    ]);
    
    // This should fail because the compiler doesn't support struct types yet
    let instance = wasm_struct_new(struct_type, [42, 3.14]);
    
    // This should also fail because we can't access struct fields yet
    let field1 = wasm_struct_get(instance, "field1");
    assert_eq(field1, 42);
}

// Expected: Test fails because WASM GC struct operations are not implemented
```

### Test 2: Array Type and Operations

```ruchy
// File: validation/wasm_gc/test_array_types_red.ruchy

fun test_array_types() {
    // Test that we can define and use array types
    let array_type = wasm_array_type("i32", true);  // type, mutable
    
    // This should fail because the compiler doesn't support array types yet
    let array = wasm_array_new(array_type, 10, 0);  // type, length, init
    
    // These operations should also fail
    wasm_array_set(array, 5, 42);
    let value = wasm_array_get(array, 5);
    assert_eq(value, 42);
    
    let length = wasm_array_len(array);
    assert_eq(length, 10);
}

// Expected: Test fails because WASM GC array operations are not implemented
```

### Test 3: Reference Type Casting and Checking

```ruchy
// File: validation/wasm_gc/test_reference_operations_red.ruchy

fun test_reference_operations() {
    // Define a base type and a subtype
    let base_type = wasm_struct_type([
        {name: "base_field", type: "i32", mutable: true}
    ]);
    
    let sub_type = wasm_struct_type([
        {name: "base_field", type: "i32", mutable: true},
        {name: "sub_field", type: "i32", mutable: true}
    ]);
    
    // This should fail because the compiler doesn't support struct creation yet
    let base_instance = wasm_struct_new(base_type, [10]);
    let sub_instance = wasm_struct_new(sub_type, [10, 20]);
    
    // These reference operations should also fail
    let is_ref = wasm_ref_is_null(base_instance);
    assert_eq(is_ref, false);
    
    let can_cast = wasm_ref_test(sub_instance, base_type);
    assert_eq(can_cast, true);
    
    let cast_result = wasm_ref_cast(sub_instance, base_type);
    assert(cast_result != null);
}

// Expected: Test fails because WASM GC reference operations are not implemented
```

### Test 4: Garbage Collection and Memory Management

```ruchy
// File: validation/wasm_gc/test_gc_memory_management_red.ruchy

fun test_gc_memory_management() {
    // Create a function that generates a lot of objects and returns one
    fun generate_and_return_one() {
        let result = null;
        
        // Generate many objects that should be garbage collected
        for (let i = 0; i < 1000; i++) {
            let struct_type = wasm_struct_type([
                {name: "value", type: "i32", mutable: false}
            ]);
            
            // This should fail because struct creation is not implemented
            let temp = wasm_struct_new(struct_type, [i]);
            
            if (i == 500) {
                result = temp;
            }
        }
        
        return result;
    }
    
    // This should fail because the compiler can't generate the WASM code
    let obj = generate_and_return_one();
    assert(obj != null);
    
    // This should also fail
    let value = wasm_struct_get(obj, "value");
    assert_eq(value, 500);
}

// Expected: Test fails because WASM GC memory management is not implemented
```

### Test 5: Complex Type Hierarchies and Polymorphism

```ruchy
// File: validation/wasm_gc/test_type_hierarchies_red.ruchy

fun test_type_hierarchies() {
    // Define a set of types forming a hierarchy
    let animal_type = wasm_struct_type([
        {name: "kind", type: "i32", mutable: false}
    ]);
    
    let dog_type = wasm_struct_type([
        {name: "kind", type: "i32", mutable: false},
        {name: "bark_volume", type: "i32", mutable: true}
    ]);
    
    let cat_type = wasm_struct_type([
        {name: "kind", type: "i32", mutable: false},
        {name: "lives_left", type: "i32", mutable: true}
    ]);
    
    // This function should take an animal and process it polymorphically
    fun process_animal(animal) {
        // This should fail because the compiler can't handle reference type checking
        if (wasm_ref_test(animal, dog_type)) {
            let dog = wasm_ref_cast(animal, dog_type);
            let volume = wasm_struct_get(dog, "bark_volume");
            return volume * 2;
        } else if (wasm_ref_test(animal, cat_type)) {
            let cat = wasm_ref_cast(animal, cat_type);
            let lives = wasm_struct_get(cat, "lives_left");
            return lives;
        }
        
        return 0;
    }
    
    // These should fail because struct creation isn't implemented
    let dog = wasm_struct_new(dog_type, [1, 10]);
    let cat = wasm_struct_new(cat_type, [2, 9]);
    
    // These should also fail
    let dog_result = process_animal(dog);
    let cat_result = process_animal(cat);
    
    assert_eq(dog_result, 20);
    assert_eq(cat_result, 9);
}

// Expected: Test fails because WASM GC type hierarchies are not implemented
```

## Expected Challenges and Considerations

1. **WebAssembly GC Proposal Status**: The WebAssembly GC proposal is still evolving. We need to target a specific version of the proposal and be prepared to adapt as the specification changes.

2. **Runtime Support**: Different WebAssembly runtimes have varying levels of support for the GC proposal. We need to ensure compatibility with major runtimes like V8, SpiderMonkey, and Wasmtime.

3. **Type System Mapping**: Mapping Ruchy's type system to WebAssembly GC types while preserving semantics, especially for generics and higher-level constructs, will be challenging.

4. **Performance Considerations**: While WebAssembly GC provides garbage collection, we need to design our memory usage patterns to minimize GC overhead and pauses.

5. **Debugging Support**: Debugging tools for WebAssembly GC are still maturing. We need to implement proper error messages and diagnostics.

6. **Edge Cases**: Handling of cyclic references, finalization, and weak references requires special consideration.

7. **Binary Size**: WebAssembly GC features may increase the binary size. We need to measure and optimize accordingly.

## Implementation Approach for the RED Phase

For the RED phase, our focus is on creating comprehensive failing tests that cover all aspects of WebAssembly GC integration. Our approach includes:

1. **Test-First Development**: Write all failing tests before implementing any functionality.

2. **Comprehensive Coverage**: Ensure tests cover all WebAssembly GC features we intend to support.

3. **Clear Failure Conditions**: Each test should clearly document expected behavior and current failure modes.

4. **Integration with Existing Pipeline**: Tests should fit into our existing validation framework and tooling.

5. **Documentation**: Thoroughly document the test suite to serve as a specification for the GREEN phase.

6. **Runtime Detection**: Include tests that detect runtime support for WebAssembly GC features.

The RED phase will involve the following steps:

1. Create the test suite structure under `validation/wasm_gc/`
2. Implement failing tests for each GC feature
3. Document expected behaviors and failure modes
4. Set up test runners and integration with our build system
5. Create skeleton implementations that will be filled in during the GREEN phase
6. Document the specifications and requirements for the GREEN phase implementation

## Success Criteria for the RED Phase

The RED phase will be considered complete when:

1. All specified tests are implemented and consistently fail for the expected reasons.

2. Test failures provide clear error messages that indicate the specific WebAssembly GC features that need to be implemented.

3. Tests are integrated with our existing testing infrastructure and can be run via `ruchy test`.

4. The test suite covers all WebAssembly GC features within scope.

5. Documentation clearly outlines the requirements and specifications for the GREEN phase implementation.

6. Tests are verified to pass when run against a reference implementation (even if manually constructed) to ensure the tests themselves are valid.

7. A plan for implementing the GREEN phase is documented, including the order of feature implementation.

## Next Steps

After completing the RED phase:

1. Review the test suite with the team to ensure comprehensive coverage.

2. Prioritize features for implementation in the GREEN phase.

3. Begin implementing WebAssembly GC features according to the test requirements.

4. Set up continuous integration to track progress as tests begin to pass.

5. Document best practices and patterns for using WebAssembly GC in Ruchy programs.

## Conclusion

The WebAssembly GC integration represents a significant enhancement to Ruchy's WebAssembly compilation target. By following a rigorous RED phase implementation, we ensure that our subsequent GREEN phase will be guided by comprehensive test coverage and clear requirements. The successful integration of WebAssembly GC will enable Ruchy to compile to WebAssembly with efficient and safe memory management, opening new deployment possibilities and performance improvements for Ruchy applications.