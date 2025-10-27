# WASM-005: WebAssembly GC Integration - GREEN Phase Plan

## Introduction

Following the successful completion of the RED phase for WebAssembly GC Integration (WASM-005), this document outlines our GREEN phase implementation strategy. The WebAssembly Garbage Collection (GC) extension is a critical feature that will allow Ruchy to compile directly to WebAssembly with efficient memory management for complex data structures without relying on external JavaScript-based memory management solutions.

This GREEN phase plan addresses all failing tests identified in the RED phase and outlines a comprehensive implementation strategy for integrating WebAssembly GC features into the Ruchy compiler's WebAssembly backend. Our goal is to implement each component methodically, following a test-driven development approach to ensure all requirements are met with high-quality, well-tested code.

## Implementation Strategy Overview

Our GREEN phase implementation will follow a layered approach, building from foundational types to complex operations:

1. **Layer 1: Core Type Representations**
   - Implement reference type representations (`ref`, `ref null`)
   - Implement struct and array type declarations
   - Implement type hierarchy and subtyping relationships

2. **Layer 2: Basic Operations**
   - Implement struct and array instantiation
   - Implement field and element access operations
   - Implement type casting and checking operations

3. **Layer 3: Advanced Features**
   - Implement memory management integration
   - Implement runtime detection and compatibility
   - Implement optimizations for performance and binary size

4. **Layer 4: Integration and Testing**
   - Integrate with the existing Ruchy WebAssembly backend
   - Implement comprehensive testing across WebAssembly runtimes
   - Validate performance and memory usage characteristics

Each layer builds upon the previous one, allowing us to verify functionality incrementally and identify any issues early in the implementation process.

## Detailed Implementation Plan

### 1. GC Type References Implementation

#### Implementation Tasks

1. **Reference Type Representation**
   - Implement binary encoding for `ref` and `ref null` types
   - Support type references to declared types (e.g., `ref $struct_type`)
   - Implement reference type validation and verification
   - Support subtyping relationships for reference types

2. **Type System Integration**
   - Map Ruchy's type system to WebAssembly GC types
   - Implement type conversion between Ruchy and WebAssembly types
   - Support generics and parameterized types where applicable
   - Provide proper error messages for type mismatches

3. **Null Reference Handling**
   - Implement safe null reference checking
   - Provide runtime safety for null dereferencing
   - Optimize null checks where possible

#### Implementation Code Example

```ruchy
// File: backend/wasm/type_references.ruchy

fun implement_reference_types() {
    // Binary encoding for reference types
    fun encode_ref_type(type_info, nullable) {
        if (nullable) {
            return {
                kind: "refnull",
                heap_type: type_info.heap_type
            };
        } else {
            return {
                kind: "ref",
                heap_type: type_info.heap_type
            };
        }
    }
    
    // Type reference validation
    fun validate_ref_type(ref_type, context) {
        // Verify that the referenced type exists
        if (ref_type.heap_type.kind == "typeidx") {
            let idx = ref_type.heap_type.index;
            if (!context.has_type(idx)) {
                throw new Error(`Type index ${idx} not found in module`);
            }
        }
        return true;
    }
    
    // Subtyping check for reference types
    fun is_subtype(sub, super, context) {
        // Implement WebAssembly GC subtyping rules
        if (sub.kind == "ref" && super.kind == "refnull") {
            return is_subtype_heap(sub.heap_type, super.heap_type, context);
        }
        if (sub.kind == "ref" && super.kind == "ref") {
            return is_subtype_heap(sub.heap_type, super.heap_type, context);
        }
        if (sub.kind == "refnull" && super.kind == "refnull") {
            return is_subtype_heap(sub.heap_type, super.heap_type, context);
        }
        return false;
    }
    
    return {
        encode_ref_type,
        validate_ref_type,
        is_subtype
    };
}
```

### 2. Struct and Array Types Implementation

#### Implementation Tasks

1. **Struct Type Declarations**
   - Implement struct type declarations in WebAssembly type section
   - Support field definitions with types and mutability
   - Implement structural subtyping for structs
   - Provide efficient memory layout for structs

2. **Array Type Declarations**
   - Implement array type declarations in WebAssembly type section
   - Support element type definitions with mutability
   - Implement array subtyping rules
   - Provide efficient memory layout for arrays

3. **Field and Element Access**
   - Implement struct field access with proper type checking
   - Implement array element access with bounds checking
   - Support mutable and immutable fields/elements
   - Provide optimizations for common access patterns

#### Implementation Code Example

```ruchy
// File: backend/wasm/struct_types.ruchy

fun implement_struct_types() {
    // Encode struct type for WebAssembly binary format
    fun encode_struct_type(fields) {
        return {
            kind: "struct",
            fields: fields.map(field => ({
                type: field.type,
                mutable: field.mutable
            }))
        };
    }
    
    // Add struct type to module
    fun add_struct_type_to_module(module, struct_type) {
        let type_index = module.types.length;
        module.types.push(struct_type);
        return type_index;
    }
    
    // Create struct instantiation instruction
    fun create_struct_new(type_index, values) {
        return {
            op: "struct.new",
            type_index,
            values
        };
    }
    
    // Create struct field access instruction
    fun create_struct_get(struct_ref, field_index) {
        return {
            op: "struct.get",
            struct_ref,
            field_index
        };
    }
    
    // Create struct field update instruction
    fun create_struct_set(struct_ref, field_index, value) {
        return {
            op: "struct.set",
            struct_ref,
            field_index,
            value
        };
    }
    
    return {
        encode_struct_type,
        add_struct_type_to_module,
        create_struct_new,
        create_struct_get,
        create_struct_set
    };
}

// File: backend/wasm/array_types.ruchy

fun implement_array_types() {
    // Encode array type for WebAssembly binary format
    fun encode_array_type(element_type, mutable) {
        return {
            kind: "array",
            element_type,
            mutable
        };
    }
    
    // Add array type to module
    fun add_array_type_to_module(module, array_type) {
        let type_index = module.types.length;
        module.types.push(array_type);
        return type_index;
    }
    
    // Create array instantiation instruction
    fun create_array_new(type_index, length, init_value) {
        return {
            op: "array.new",
            type_index,
            length,
            init_value
        };
    }
    
    // Create array element access instruction
    fun create_array_get(array_ref, index) {
        return {
            op: "array.get",
            array_ref,
            index
        };
    }
    
    // Create array element update instruction
    fun create_array_set(array_ref, index, value) {
        return {
            op: "array.set",
            array_ref,
            index,
            value
        };
    }
    
    // Create array length query instruction
    fun create_array_len(array_ref) {
        return {
            op: "array.len",
            array_ref
        };
    }
    
    return {
        encode_array_type,
        add_array_type_to_module,
        create_array_new,
        create_array_get,
        create_array_set,
        create_array_len
    };
}
```

### 3. Reference Type Operations Implementation

#### Implementation Tasks

1. **Object Instantiation**
   - Implement `struct.new` for struct instantiation
   - Implement `array.new` for array creation
   - Support initialization with values
   - Validate constructor arguments against type definitions

2. **Reference Operations**
   - Implement `ref.test` for type checking
   - Implement `ref.cast` for safe type casting
   - Implement `ref.eq` for reference equality comparison
   - Support `ref.is_null` for null checking

3. **Type Casting and Checking**
   - Implement safe casting with runtime checks
   - Support downcasting with proper validation
   - Implement polymorphic operations
   - Provide error handling for invalid casts

#### Implementation Code Example

```ruchy
// File: backend/wasm/reference_operations.ruchy

fun implement_reference_operations() {
    // Implement reference test operation
    fun create_ref_test(ref_value, type_index) {
        return {
            op: "ref.test",
            ref_value,
            type_index
        };
    }
    
    // Implement reference cast operation
    fun create_ref_cast(ref_value, type_index) {
        return {
            op: "ref.cast",
            ref_value,
            type_index
        };
    }
    
    // Implement reference equality comparison
    fun create_ref_eq(ref1, ref2) {
        return {
            op: "ref.eq",
            ref1,
            ref2
        };
    }
    
    // Implement null check operation
    fun create_ref_is_null(ref_value) {
        return {
            op: "ref.is_null",
            ref_value
        };
    }
    
    // Safely perform runtime type casting
    fun safe_cast(module, ref_value, from_type, to_type) {
        // First perform a test
        let test = create_ref_test(ref_value, to_type);
        // Then conditionally cast
        let cast = create_ref_cast(ref_value, to_type);
        
        // Generate the if-else structure
        return {
            op: "if",
            condition: test,
            then: cast,
            else: {
                op: "unreachable" // Or a safer fallback
            }
        };
    }
    
    return {
        create_ref_test,
        create_ref_cast,
        create_ref_eq,
        create_ref_is_null,
        safe_cast
    };
}
```

### 4. Memory Management Implementation

#### Implementation Tasks

1. **GC Integration**
   - Integrate with WebAssembly's built-in garbage collector
   - Implement proper reference tracking
   - Support cyclic reference handling
   - Provide fallbacks for runtimes without GC support

2. **Lifecycle Management**
   - Implement proper object lifecycle management
   - Support weak references where needed
   - Handle finalizers and resource cleanup
   - Implement memory pressure detection and response

3. **Optimization Strategies**
   - Implement reference counting optimization where appropriate
   - Reduce GC pressure through object pooling
   - Optimize memory layout for cache efficiency
   - Reduce unnecessary object allocations

#### Implementation Code Example

```ruchy
// File: backend/wasm/memory_management.ruchy

fun implement_memory_management() {
    // Detect GC support in runtime
    fun detect_gc_support(runtime_info) {
        return runtime_info.features.includes("gc");
    }
    
    // Optimize memory layout for garbage collection
    fun optimize_struct_layout(struct_type) {
        // Reorder fields for better alignment and less fragmentation
        // This is highly dependent on the runtime, but we can apply general rules
        
        let fields = struct_type.fields;
        let optimized = [];
        
        // Sort fields by size (largest first) to reduce padding
        let size_groups = {
            "8": [],
            "4": [],
            "2": [],
            "1": []
        };
        
        for (let i = 0; i < fields.length; i++) {
            let field = fields[i];
            let size = get_type_size(field.type);
            if (size == 8) size_groups["8"].push(field);
            else if (size == 4) size_groups["4"].push(field);
            else if (size == 2) size_groups["2"].push(field);
            else size_groups["1"].push(field);
        }
        
        // Reassemble fields by size
        optimized = optimized.concat(size_groups["8"]);
        optimized = optimized.concat(size_groups["4"]);
        optimized = optimized.concat(size_groups["2"]);
        optimized = optimized.concat(size_groups["1"]);
        
        return {
            ...struct_type,
            fields: optimized,
            original_indices: optimized.map(field => fields.indexOf(field))
        };
    }
    
    // Handle cyclic references
    fun detect_cycles(type_graph) {
        // Implement cycle detection in the type graph
        let visited = new Set();
        let recursion_stack = new Set();
        let cycles = [];
        
        fun dfs(node) {
            visited.add(node);
            recursion_stack.add(node);
            
            let neighbors = type_graph[node] || [];
            for (let i = 0; i < neighbors.length; i++) {
                let neighbor = neighbors[i];
                
                if (!visited.has(neighbor)) {
                    if (dfs(neighbor)) {
                        return true;
                    }
                } else if (recursion_stack.has(neighbor)) {
                    cycles.push([node, neighbor]);
                    return true;
                }
            }
            
            recursion_stack.delete(node);
            return false;
        }
        
        // Run DFS on all nodes
        Object.keys(type_graph).forEach(node => {
            if (!visited.has(node)) {
                dfs(node);
            }
        });
        
        return cycles;
    }
    
    return {
        detect_gc_support,
        optimize_struct_layout,
        detect_cycles
    };
}
```

### 5. Integration with Existing Backend

#### Implementation Tasks

1. **WebAssembly Module Integration**
   - Integrate GC types into the WebAssembly module structure
   - Update binary encoding for GC features
   - Support validation of GC-enabled modules
   - Ensure compatibility with existing backend features

2. **Compiler Pipeline Integration**
   - Update the code generation pipeline for GC support
   - Integrate type mapping between Ruchy and WebAssembly GC
   - Support progressive enhancement based on runtime capabilities
   - Provide fallbacks for non-GC environments

3. **Optimization Passes**
   - Implement GC-specific optimization passes
   - Optimize reference usage patterns
   - Reduce unnecessary allocations and reference tracking
   - Balance between memory usage and performance

#### Implementation Code Example

```ruchy
// File: backend/wasm/gc_integration.ruchy

fun integrate_gc_with_backend(existing_backend) {
    // Extend the WebAssembly module format
    let extended_module = {
        ...existing_backend.module,
        gc_enabled: true,
        gc_types: []
    };
    
    // Extend the type section encoding
    let original_encode_type_section = existing_backend.encode_type_section;
    fun extended_encode_type_section(module) {
        if (!module.gc_enabled || module.gc_types.length === 0) {
            return original_encode_type_section(module);
        }
        
        // Add GC types to the type section
        let gc_encoded_types = module.gc_types.map(type => {
            if (type.kind === "struct") {
                return encode_struct_type(type.fields);
            } else if (type.kind === "array") {
                return encode_array_type(type.element_type, type.mutable);
            }
            // Handle other GC types
            return null;
        }).filter(t => t !== null);
        
        // Combine with regular types
        let all_types = [...module.types, ...gc_encoded_types];
        
        // Encode the expanded type section
        return {
            id: 1, // Type section ID
            content: encode_vector(all_types)
        };
    }
    
    // Extend the code generation pipeline
    let original_generate_code = existing_backend.generate_code;
    fun extended_generate_code(ast, options) {
        // Detect if GC features are needed
        let requires_gc = detect_gc_requirements(ast);
        
        // If GC is not required, use original code generator
        if (!requires_gc) {
            return original_generate_code(ast, options);
        }
        
        // Process AST with GC support
        let processed_ast = preprocess_ast_for_gc(ast);
        
        // Apply GC-specific optimizations
        let optimized_ast = optimize_for_gc(processed_ast);
        
        // Generate code with GC support
        return generate_code_with_gc(optimized_ast, options);
    }
    
    return {
        module: extended_module,
        encode_type_section: extended_encode_type_section,
        generate_code: extended_generate_code,
        // Preserve other backend functions
        ...existing_backend
    };
}
```

## Implementation Order and Dependencies

The implementation will follow a sequential order with dependencies between components. The recommended implementation sequence is:

1. **Phase 1: Core Type System Integration**
   - Reference type representations
   - Struct type declarations
   - Array type declarations
   
   *Dependencies: None*

2. **Phase 2: Basic Operations Implementation**
   - Struct instantiation and field access
   - Array instantiation and element access
   - Reference type operations (null checks)
   
   *Dependencies: Phase 1*

3. **Phase 3: Advanced Reference Operations**
   - Type casting and checking
   - Polymorphic operations
   - Reference equality and comparison
   
   *Dependencies: Phase 2*

4. **Phase 4: Memory Management Integration**
   - GC integration and lifecycle management
   - Optimization for memory usage
   - Handling of special cases (cycles, etc.)
   
   *Dependencies: Phase 3*

5. **Phase 5: Backend Integration and Testing**
   - Integration with existing WebAssembly backend
   - Comprehensive testing across runtimes
   - Performance optimization and validation
   
   *Dependencies: Phase 4*

This sequencing ensures that each phase builds on stable, well-tested functionality from previous phases.

## Key Challenges and Mitigation Strategies

Based on our RED phase findings, we anticipate several challenges in the GREEN phase implementation. Here are our strategies for addressing each one:

### 1. WebAssembly GC Proposal Evolution

**Challenge:** The WebAssembly GC proposal is still evolving and may change during our implementation.

**Mitigation:**
- Target a specific version of the proposal for initial implementation
- Design with modularity to adapt to changes
- Maintain a compatibility layer to handle version differences
- Regularly monitor proposal updates and adjust implementation as needed

### 2. Runtime Support Variability

**Challenge:** Different WebAssembly runtimes have varying levels of support for GC features.

**Mitigation:**
- Implement runtime detection for GC support
- Provide fallback mechanisms for runtimes without GC support
- Design a progressive enhancement approach for partial support
- Maintain compatibility with major runtimes (V8, SpiderMonkey, Wasmtime)

### 3. Type System Mapping Complexity

**Challenge:** Mapping Ruchy's rich type system to WebAssembly GC types while preserving semantics is complex.

**Mitigation:**
- Create a comprehensive type mapping layer
- Implement specialized handling for complex Ruchy types
- Use structural typing where appropriate
- Provide clear error messages for unmappable types

### 4. Performance Optimization

**Challenge:** GC-based memory management may introduce overhead and pauses.

**Mitigation:**
- Implement memory usage optimization strategies
- Reduce unnecessary object allocations
- Use object pooling for frequently allocated types
- Optimize memory layout for better GC performance
- Implement reference counting for critical sections

### 5. Debugging Support

**Challenge:** Debugging tools for WebAssembly GC are still maturing.

**Mitigation:**
- Implement enhanced error reporting for GC operations
- Create diagnostic tools for GC-related issues
- Provide runtime logging of GC operations when debugging
- Develop visualization tools for object relationships

### 6. Edge Cases and Special Handling

**Challenge:** Handling cyclic references, finalization, and weak references requires special consideration.

**Mitigation:**
- Implement cycle detection in the type system
- Design explicit handling for weak references
- Create safe finalization mechanisms
- Test extensively with pathological cases

### 7. Binary Size Optimization

**Challenge:** WebAssembly GC features may increase binary size.

**Mitigation:**
- Implement on-demand loading of GC types
- Optimize type encoding to reduce size
- Provide tree-shaking to remove unused GC features
- Implement binary compression techniques for GC data

## Testing Approach

Our testing approach for the GREEN phase will ensure comprehensive validation of all WebAssembly GC features:

### 1. Unit Testing

- Test each GC operation in isolation
- Validate type representations and operations
- Test error handling and edge cases
- Ensure type safety and proper validation

### 2. Integration Testing

- Test combinations of GC operations
- Validate complex data structures and relationships
- Test type hierarchies and polymorphism
- Ensure interoperability with non-GC WebAssembly features

### 3. Compatibility Testing

- Test across different WebAssembly runtimes
- Validate behavior with different GC implementations
- Test with different versions of the GC proposal
- Ensure consistent behavior across environments

### 4. Performance Testing

- Measure GC overhead and pauses
- Benchmark memory usage patterns
- Test with large object graphs
- Validate optimization effectiveness

### 5. Regression Testing

- Ensure existing WebAssembly features still work
- Validate backward compatibility where appropriate
- Test with the full Ruchy test suite
- Verify integration with the existing compilation pipeline

## Success Criteria for the GREEN Phase

The GREEN phase will be considered complete when:

1. All RED phase tests pass successfully with the implementation.

2. The implementation supports all specified WebAssembly GC features:
   - Reference types (`ref`, `ref null`)
   - Struct and array type declarations
   - Reference type operations (instantiation, access, casting)
   - Memory management integration

3. The implementation is well-integrated with the existing WebAssembly backend.

4. The implementation demonstrates acceptable performance characteristics:
   - Binary size overhead is within acceptable limits
   - Memory usage is optimized for GC
   - Compilation time remains reasonable
   - Runtime performance is comparable to non-GC alternatives

5. The implementation works across major WebAssembly runtimes with GC support.

6. The implementation includes comprehensive error handling and diagnostics.

7. The implementation is well-documented with examples and best practices.

## Conclusion

The GREEN phase implementation plan for WebAssembly GC integration provides a comprehensive roadmap for successfully implementing all the features identified in the RED phase. By following this structured approach, with careful attention to dependencies, challenges, and testing, we can ensure a high-quality implementation that enhances Ruchy's WebAssembly compilation capabilities.

The successful integration of WebAssembly GC will enable Ruchy to compile to WebAssembly with efficient and safe memory management for complex data structures, opening new deployment possibilities and performance improvements for Ruchy applications. This represents a significant step forward in Ruchy's compiler technology and will provide valuable capabilities for our users.