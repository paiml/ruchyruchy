# Ruchy to WASM Type Mapping System

**Date**: October 23, 2025  
**Status**: Draft  
**Purpose**: Define the type mapping system for Ruchy to WASM compilation  

## Overview

This document defines the comprehensive type mapping system for compiling Ruchy code to WebAssembly. The mapping system addresses the challenge of bridging Ruchy's rich type system with WASM's limited primitive types.

## Core Types

### Primitive Types

| Ruchy Type | WASM Type | Implementation | Notes |
|------------|-----------|----------------|-------|
| `i32` | `i32` | Direct | One-to-one mapping |
| `i64` | `i64` | Direct | One-to-one mapping |
| `f32` | `f32` | Direct | One-to-one mapping |
| `f64` | `f64` | Direct | One-to-one mapping |
| `bool` | `i32` | `0` = false, `1` = true | Boolean values encoded as integers |
| `unit` | - | No value | Functions returning unit omit return value |

### Composite Types

| Ruchy Type | WASM Type | Implementation | Notes |
|------------|-----------|----------------|-------|
| `String` | `i32` | Linear memory address | Length-prefixed UTF-8 with header |
| `Array<T>` | `i32` | Linear memory address | Length-prefixed with element type tag |
| `Tuple(T1, T2, ...)` | Multiple values or memory | Stack values or heap structure | Small tuples use multi-value return |
| `Option<T>` | `i32` or multi-value | Tagged union in memory | None = 0, Some(x) = address |
| `Result<T, E>` | `i32` or multi-value | Tagged union in memory | Ok/Err tag + value |
| `Struct` | `i32` | Linear memory address | Field offsets computed at compile time |
| `Enum` | `i32` + memory | Tag + payload | Tag in lower bits, payload in memory |

### Function Types

| Ruchy Type | WASM Type | Implementation | Notes |
|------------|-----------|----------------|-------|
| `Function` | `funcref` or `i32` | Table index or closure record | Based on closure spike findings |
| `Closure` | `i32` | Memory address | Closure record containing func index + captures |
| `Method` | `i32` | Memory address | Closure record with implicit `self` |
| `Lambda` | `i32` | Memory address | Closure record with bound variables |

## Memory Layout

### Strings

Strings are represented as length-prefixed UTF-8 sequences in linear memory:

```
┌───────────┬─────────────┬─────────────────┐
│ Length(4B) │ Flags(4B)   │ UTF-8 Data...   │
└───────────┴─────────────┴─────────────────┘
```

- **Length**: Number of bytes (not characters)
- **Flags**: Metadata (hash caching, interning status)
- **Data**: UTF-8 encoded characters

### Arrays

Arrays are represented as length-prefixed sequences with runtime type information:

```
┌───────────┬─────────────┬─────────────────────────┐
│ Length(4B) │ Type Tag(4B) │ Elements (size * count) │
└───────────┴─────────────┴─────────────────────────┘
```

- **Length**: Number of elements
- **Type Tag**: Runtime type information
- **Elements**: Inline for primitives, pointers for complex types

### Structs

Structs are represented as fixed-layout blocks in memory:

```
┌─────────────┬─────────────┬─────────────┬─────┐
│ Type Tag(4B) │ Field 1     │ Field 2     │ ... │
└─────────────┴─────────────┴─────────────┴─────┘
```

- **Type Tag**: Identifies struct type for runtime type checking
- **Fields**: Inline for primitives, pointers for complex types
- **Layout**: Computed at compile time for optimal field alignment

### Closure Records

Based on the closure spike, closures use memory records:

```
┌─────────────┬────────────────┬─────────────┬─────┐
│ Func Idx(4B) │ Capture 1 Ptr  │ Capture 2   │ ... │
└─────────────┴────────────────┴─────────────┴─────┘
```

- **Func Idx**: Index in function table
- **Captures**: Pointers to captured variables
- **Function Call**: Indirect call with closure pointer as first parameter

## Type Conversions

### Between Ruchy Types and WASM Types

| Conversion | Implementation |
|------------|----------------|
| `i32 -> bool` | `i32.ne 0` |
| `bool -> i32` | Direct (already 0 or 1) |
| `i32 -> f32` | `f32.convert_i32_s` |
| `f32 -> i32` | `i32.trunc_f32_s` |
| `Option<T> -> T` | Check tag, load value or throw |
| `Result<T, E> -> T` | Check tag, load value or throw |

### Type Checking

Runtime type checking is implemented for operations that require it:

```wat
;; Example: Check if value is string
(func $is_string (param $ptr i32) (result i32)
  (i32.eq
    (i32.load offset=4 (local.get $ptr))  ;; Type tag
    (i32.const 1))                        ;; String tag = 1
)
```

## Function Calling Conventions

### Regular Functions

Regular functions follow standard WASM calling conventions:

```wat
(func $add (param $a i32) (param $b i32) (result i32)
  (i32.add (local.get $a) (local.get $b))
)
```

### Closures

Closures receive the closure record as an additional first parameter:

```wat
(func $counter_impl (param $closure_ptr i32) (param $x i32) (result i32)
  ;; Load captured variable from closure record
  (local $count_addr i32)
  (local.set $count_addr
    (i32.load offset=4 (local.get $closure_ptr)))
  
  ;; Rest of function implementation
  ...
)
```

### Methods

Methods also receive `self` as the first parameter:

```wat
(func $point_distance (param $self_ptr i32) (param $other_ptr i32) (result f32)
  ;; Implementation
)
```

## Memory Management

### Allocation

Memory is allocated using a simple bump allocator during development:

```wat
(func $malloc (param $size i32) (result i32)
  (local $addr i32)
  (local.set $addr (global.get $heap_ptr))
  (global.set $heap_ptr
    (i32.add (global.get $heap_ptr) (local.get $size)))
  (local.get $addr))
```

Future implementations will use more sophisticated memory management based on the Memory Management Optimizations (OPT-INTERP-004).

### Garbage Collection

Initial implementation will use JavaScript host GC integration, while planning for eventual migration to native WASM GC when fully supported:

```wat
;; Import JavaScript GC functions
(import "js" "collect" (func $js_collect))
(import "js" "alloc" (func $js_alloc (param i32) (result i32)))
```

## Special Cases

### String Operations

String operations are implemented as built-in functions:

| Operation | Implementation |
|-----------|----------------|
| `str1 + str2` | `$string_concat(str1_ptr, str2_ptr)` |
| `str.length()` | `i32.load(str_ptr)` (load length field) |
| `str[i]` | `$string_char_at(str_ptr, i)` |
| `str.substr(i, len)` | `$string_substr(str_ptr, i, len)` |

### Optional and Result Types

Option and Result types are implemented using tagged unions:

```wat
;; Create Some(42)
(func $create_some (param $value i32) (result i32)
  (local $ptr i32)
  (local.set $ptr (call $malloc (i32.const 8)))
  (i32.store (local.get $ptr) (i32.const 1))      ;; Some tag = 1
  (i32.store offset=4 (local.get $ptr) (local.get $value))
  (local.get $ptr))

;; Unwrap Option
(func $option_unwrap (param $opt_ptr i32) (result i32)
  (if (result i32)
    (i32.eq (i32.load (local.get $opt_ptr)) (i32.const 0))
    (then
      (call $panic_none_unwrap))  ;; None case - panic
    (else
      (i32.load offset=4 (local.get $opt_ptr))))  ;; Some case - return value
)
```

## Implementation Strategy

### Phase 1: Core Type Mapping

Implement direct mappings for primitive types and basic composite types:
- Primitive types (i32, i64, f32, f64, bool)
- Strings with basic operations
- Arrays with single element type
- Simple functions without closures

### Phase 2: Advanced Type Mapping

Add support for more complex types:
- Structs with multiple fields
- Closures with captured variables
- Enums with variant payloads
- Option and Result types

### Phase 3: Optimization

Optimize type representations for performance:
- Small string optimization (inline short strings)
- Value type optimization (avoid heap for small structs)
- Specialized type operations
- SIMD acceleration where applicable

## Test Cases

### Primitive Types

```ruchy
// Test primitive type operations
fun test_primitive_ops() {
    let a: i32 = 42
    let b: i32 = 10
    let sum: i32 = a + b
    let product: i32 = a * b
    let quotient: f32 = a as f32 / b as f32
    let is_greater: bool = a > b
    
    assert_eq(sum, 52)
    assert_eq(product, 420)
    assert_eq(quotient, 4.2)
    assert(is_greater)
}
```

### Strings

```ruchy
// Test string operations
fun test_string_ops() {
    let s1: String = "Hello"
    let s2: String = "World"
    let concat: String = s1 + " " + s2
    let len: i32 = concat.length()
    let first_char: char = concat[0]
    
    assert_eq(concat, "Hello World")
    assert_eq(len, 11)
    assert_eq(first_char, 'H')
}
```

### Arrays

```ruchy
// Test array operations
fun test_array_ops() {
    let arr: [i32; 3] = [1, 2, 3]
    let first: i32 = arr[0]
    let mut sum: i32 = 0
    
    for i in 0..3 {
        sum = sum + arr[i]
    }
    
    assert_eq(first, 1)
    assert_eq(sum, 6)
}
```

### Closures

```ruchy
// Test closures
fun test_closures() {
    fun make_counter() {
        let mut count = 0
        (x) => {
            count = count + x
            count
        }
    }
    
    let counter = make_counter()
    let a = counter(5)
    let b = counter(3)
    
    assert_eq(a, 5)
    assert_eq(b, 8)
}
```

## Benchmarking

Type mapping performance will be assessed using:

1. **Memory Usage**: Size of type representations
2. **Allocation Time**: Time to create complex types
3. **Operation Time**: Time for type-specific operations
4. **GC Pressure**: Memory churn for different representations

## Conclusion

This type mapping system provides a comprehensive strategy for translating Ruchy types to WASM representations. By using linear memory for complex types and WASM's native types for primitives, we achieve a balance of performance and expressiveness.

The implementation follows a phased approach, starting with core types and progressively adding support for more complex types. The final system will enable efficient execution of Ruchy programs in WebAssembly environments.