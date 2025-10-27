# WASM-001: GREEN Phase Implementation Plan

## Overview

This document outlines the GREEN phase implementation plan for WASM-001: WebAssembly Type Mapping. After successfully completing the RED phase, we now need to implement the minimum viable functionality required to make our tests pass.

## Implementation Components

### 1. Core Classes

#### `WasmEmitter` Class
```ruchy
class WasmEmitter {
    // Internal state
    module: Module
    function_table: Map<String, Int>
    type_registry: Map<String, RuchyWasmType>
    
    // Constructor
    fun new() -> WasmEmitter
    
    // Module generation
    fun generate_module() -> Module
    fun emit_wat() -> String
    
    // Function handling
    fun add_function(name: String, params: [[String, String]], 
                     result_type: String, body: String) -> ()
}
```

#### `RuchyWasmType` Class
```ruchy
class RuchyWasmType {
    // Type information
    type_name: String
    memory_layout: MemoryLayout
    
    // Constructors
    fun new(type_name: String) -> RuchyWasmType
    fun new_struct(name: String, fields: [[String, RuchyWasmType]]) -> RuchyWasmType
    fun new_function(params: [RuchyWasmType], result: RuchyWasmType) -> RuchyWasmType
    fun new_closure(params: [RuchyWasmType], result: RuchyWasmType, 
                    captured: [RuchyWasmType]) -> RuchyWasmType
    
    // Methods
    fun wasm_type() -> Type
    fun memory_layout() -> MemoryLayout
    fun element_type() -> RuchyWasmType  // For arrays
    fun field_offset(field_name: String) -> Int  // For structs
    fun to_wasm_function() -> WasmFunction  // For functions
}
```

#### `MemoryLayout` Class
```ruchy
class MemoryLayout {
    header_size: Int
    body_size: Int
    fields: Map<String, Int>  // Field name to offset
    
    fun new(header_size: Int, body_size: Int) -> MemoryLayout
    fun add_field(name: String, offset: Int) -> ()
    fun size() -> Int
}
```

#### `WasmFunction` Class
```ruchy
class WasmFunction {
    params: [Type]
    result: Type
    
    fun new(params: [Type], result: Type) -> WasmFunction
    fun param_types() -> [Type]
    fun result_type() -> Type
}
```

### 2. Utility Functions

```ruchy
// Map Ruchy type string to WASM type
fun wasmify_type(type_string: String) -> Type

// Convert Ruchy literal to WASM instruction
fun wasmify_literal(type_string: String, literal_value: String) -> Instruction
```

## Implementation Plan

### Phase 1: Type Mapping (Day 1)

1. Implement `RuchyWasmType` class
   - Basic type mapping for primitives
   - Memory layout calculation
   - Special handling for complex types

2. Implement `wasmify_type` utility
   - Map Ruchy type strings to WASM types
   - Handle edge cases (unit type, boolean)

3. Implement `MemoryLayout` class
   - Manage header and body sizes
   - Track field offsets for structs

### Phase 2: Basic Emitter (Day 2)

1. Implement `WasmEmitter` class
   - Module creation
   - Function handling
   - Type registry

2. Implement `WasmFunction` class
   - Parameter and result type mapping
   - Function signature generation

3. Implement `wasmify_literal` utility
   - Convert integer/float literals
   - Handle boolean literals
   - Special handling for string literals

### Phase 3: Module Generation (Day 3)

1. Implement `generate_module()` method
   - Create WASM module structure
   - Add memory section
   - Add function section
   - Add export section

2. Implement `emit_wat()` method
   - Generate WebAssembly Text Format
   - Validate generated module

3. Implement additional utilities
   - Memory allocation helpers
   - String handling functions
   - Array manipulation functions

## Test-Driven Implementation

For each component, we'll follow this approach:

1. Start with the simplest tests (primitive type mapping)
2. Implement minimum code to pass the test
3. Move to more complex tests (strings, arrays)
4. Gradually implement all required functionality
5. Ensure all tests pass with minimal implementation

## Leveraging Ruchy v3.125.0

We'll make use of the Ruchy v3.125.0 WASM APIs:

```ruchy
import ruchy::wasm::Module
import ruchy::wasm::Type
import ruchy::wasm::Function
import ruchy::wasm::Instruction
import ruchy::wasm::emit
import ruchy::wasm::validate
```

This will simplify our implementation by leveraging the built-in WASM support.

## Memory Management Strategy

We'll implement a memory management approach that:

1. Uses a single linear memory section
2. Provides allocation functions via JavaScript host
3. Uses standard memory layouts for complex types:
   - Strings: [length(4), capacity(4), data...]
   - Arrays: [length(4), capacity(4), elements...]
   - Structs: [field1, field2, ...]
   - Closures: [function_idx(4), captured_vars...]

## Success Criteria

The GREEN phase is considered successful when:

1. All tests in `test_wasm_emitter_red.ruchy` pass
2. The implementation is minimal but complete
3. The generated WASM modules are valid and can be executed
4. All type mapping functionality works correctly
5. Complex types (strings, arrays, structs, functions) are handled properly

## Timeline

- Day 1: Implement type mapping and memory layout
- Day 2: Implement basic emitter and function handling
- Day 3: Implement module generation and WAT emission
- Day 4: Fix remaining issues and ensure all tests pass

Total estimated time: 3-4 days for the GREEN phase