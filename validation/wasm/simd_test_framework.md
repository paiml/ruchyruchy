# SIMD Test Framework for WASM-004

## Overview

This document provides comprehensive documentation for the SIMD test framework developed as part of the WASM-004 TOOL phase. The framework enables thorough testing of SIMD (Single Instruction, Multiple Data) operations in the context of WebAssembly compilation targets.

## Framework Components

The SIMD test framework consists of the following components:

### 1. Core Framework (`simd_test_framework.ruchy`)

The core framework provides:

- **SIMD Vector Types**: Type definitions for common SIMD vector formats (V128, F32x4, I32x4, etc.)
- **Vector Creation Utilities**: Functions to create and manipulate SIMD vectors
- **Property Testing Utilities**: Functions to verify mathematical properties of SIMD operations
- **Assertion Utilities**: Specialized functions for comparing SIMD vectors
- **Fuzz Testing Generators**: Random data generators for SIMD vector types
- **Performance Measurement**: Tools for comparing scalar vs SIMD implementations
- **Cross-Platform Testing**: Utilities for testing across different compilation targets

### 2. Example Tests (`test_simd_framework_example.ruchy`)

Demonstrates basic usage of the framework with:

- Property-based tests for fundamental operations (add, multiply)
- Unit tests for specific vector operations
- Performance benchmarks comparing scalar vs SIMD implementations

### 3. Cryptography Tests (`test_simd_cryptography_green.ruchy`)

Specialized tests for cryptographic applications:

- Simplified implementations of common crypto operations (SHA-256, ChaCha20)
- Property tests for XOR operations and rotations
- Performance benchmarks for crypto primitives

### 4. Image Processing Tests (`test_simd_image_processing_green.ruchy`)

Specialized tests for image processing:

- RGBA to grayscale conversion using SIMD
- Box blur implementation
- Comparison of scalar vs SIMD image processing
- Performance benchmarks for common operations

### 5. Test Runner (`test_simd_tool_runner.ruchy`)

Orchestrates all tests and provides:

- Categorized test execution
- Performance measurement for each test category
- Detailed reporting of test results
- Summary statistics for all test suites

## Usage Guide

### Setting Up SIMD Tests

1. **Import the Framework**:

```ruchy
import { 
    F32x4, I32x4, 
    property_simd_commutativity,
    simd_equals,
    gen_f32x4,
    benchmark_scalar_vs_simd,
    register_simd_test_suite,
    run_simd_tests
} from "./simd_test_framework.ruchy";
```

2. **Define SIMD Operations to Test**:

```ruchy
// SIMD vector addition
fun f32x4_add(a: F32x4, b: F32x4) -> F32x4 {
    return [
        a[0] + b[0],
        a[1] + b[1],
        a[2] + b[2],
        a[3] + b[3]
    ];
}
```

3. **Write Property Tests**:

```ruchy
// Test that addition is commutative
fun test_addition_commutative() -> bool {
    for _ in 0..100 {
        let a = gen_f32x4();
        let b = gen_f32x4();
        if !property_simd_commutativity(a, b, f32x4_add) {
            return false;
        }
    }
    return true;
}
```

4. **Write Unit Tests with Specific Values**:

```ruchy
fun test_specific_values() -> bool {
    let a = f32x4(1.0, 2.0, 3.0, 4.0);
    let b = f32x4(5.0, 6.0, 7.0, 8.0);
    let expected = f32x4(6.0, 8.0, 10.0, 12.0);
    let result = f32x4_add(a, b);
    assert_simd_equals(result, expected, "Addition test failed");
    return true;
}
```

5. **Benchmark Performance**:

```ruchy
fun benchmark_performance() -> bool {
    let a = f32x4(1.0, 2.0, 3.0, 4.0);
    let b = f32x4(5.0, 6.0, 7.0, 8.0);
    let pair = [a, b];
    
    let iterations = 1000000;
    let [scalar_time, simd_time, speedup] = benchmark_scalar_vs_simd(
        pair,
        scalar_add_vectors,
        simd_add_vectors,
        iterations
    );
    
    println("Speedup: " + speedup.to_string() + "x");
    return speedup >= 1.5;  // Expect at least 1.5x improvement
}
```

6. **Register and Run Tests**:

```ruchy
fun main() {
    let tests = [
        test_addition_commutative,
        test_specific_values,
        benchmark_performance
    ];
    
    register_simd_test_suite("My SIMD Tests", tests);
    let all_passed = run_simd_tests();
    return if all_passed { 0 } else { 1 };
}
```

### Running All Tests

To run the complete SIMD test suite:

```bash
ruchy test validation/wasm/test_simd_tool_runner.ruchy
```

This will execute all test categories and generate a detailed report.

## Key Testing Approaches

The framework supports several complementary testing approaches:

### 1. Property-Based Testing

Tests mathematical properties that should hold for SIMD operations:

- **Commutativity**: f(a, b) = f(b, a)
- **Associativity**: f(f(a, b), c) = f(a, f(b, c))
- **Identity**: f(a, identity) = a
- **Inverse**: f(f(a, b), inverse(b)) = a
- **Scalar Equivalence**: SIMD operations should match scalar operations applied to each lane

### 2. Unit Testing

Tests specific input-output pairs with known correct values:

- Basic cases (zeros, ones, identity values)
- Edge cases (NaN, infinity, maximum/minimum values)
- Special patterns (alternating bits, single-lane values)

### 3. Fuzz Testing

Generates random inputs to discover edge cases:

- Random vector generation (`gen_f32x4()`, `gen_i32x4()`, etc.)
- Special pattern generation (`gen_special_patterns()`)
- Combined with property testing for thorough verification

### 4. Performance Testing

Benchmarks SIMD operations against scalar equivalents:

- Measures execution time of both implementations
- Calculates speedup factor
- Verifies that SIMD provides expected performance benefits
- Can enforce minimum speedup thresholds

### 5. Cross-Platform Testing

Verifies consistent behavior across compilation targets:

- WebAssembly (with and without SIMD extensions)
- JavaScript fallbacks
- Native compilation targets

## Example Applications

The framework includes examples for several SIMD application domains:

### Cryptography

- SHA-256 mixing functions
- ChaCha20 quarter rounds
- XOR operations
- Bit rotations and shifts

### Image Processing

- RGB to grayscale conversion
- Box blur filter
- Color space transformations
- Pixel manipulation

### Machine Learning (Extensible)

The framework can be extended for ML operations:

- Vector dot products
- Matrix multiplications
- Activation functions
- Normalization

## Contributing New Tests

To add new SIMD tests:

1. Create a new test file (e.g., `test_simd_your_domain.ruchy`)
2. Import the framework components you need
3. Define the SIMD operations you want to test
4. Write property tests, unit tests, and benchmarks
5. Add your test file to the `test_simd_tool_runner.ruchy`

## Performance Guidelines

When writing SIMD tests, consider:

1. **Thresholds**: Define minimum expected speedup factors
2. **Iteration Counts**: Use enough iterations for stable measurements
3. **Data Size**: Test with realistic data sizes
4. **Compilation Flags**: Document any required WebAssembly SIMD flags
5. **Browser Compatibility**: Note which browsers support each feature

## Conclusion

The SIMD test framework provides a comprehensive solution for validating SIMD operations in WebAssembly compilation targets. By combining property testing, unit testing, fuzz testing, and performance benchmarking, it ensures correct and efficient implementation of SIMD operations across a variety of application domains.