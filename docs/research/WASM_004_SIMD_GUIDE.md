# WASM-004: SIMD Guide for WebAssembly

## Introduction to SIMD

Single Instruction Multiple Data (SIMD) is a class of parallel computing techniques that allows a single instruction to operate on multiple data points simultaneously. SIMD instructions are particularly valuable for performance-critical applications that process large amounts of data with the same operations.

WebAssembly SIMD (introduced in the SIMD proposal) extends the WebAssembly instruction set with 128-bit packed vectors and operations, enabling efficient parallel data processing. This capability is particularly valuable for computationally intensive tasks such as:

- Image and video processing
- Audio processing
- Physics simulations
- Machine learning
- Cryptography
- Scientific computing

## SIMD Types in WebAssembly

WebAssembly SIMD introduces a new value type:

- `v128` - A 128-bit vector that can be interpreted as:
  - 16 × 8-bit lanes (i8x16, u8x16)
  - 8 × 16-bit lanes (i16x8, u16x8)
  - 4 × 32-bit lanes (i32x4, u32x4, f32x4)
  - 2 × 64-bit lanes (i64x2, u64x2, f64x2)

## Common SIMD Operations

### Vector Creation and Lane Access

```ruchy
// Create vectors
fun create_vectors() {
  // Create a vector with all lanes set to the same value
  let v1 = i32x4.splat(42);
  
  // Create a vector with specific values for each lane
  let v2 = i32x4(1, 2, 3, 4);
  
  // Extract a lane
  let x = i32x4.extract_lane(v2, 0); // x = 1
  
  // Replace a lane
  let v3 = i32x4.replace_lane(v2, 0, 99); // v3 = [99, 2, 3, 4]
}
```

### Arithmetic Operations

```ruchy
fun vector_arithmetic() {
  let a = f32x4(1.0, 2.0, 3.0, 4.0);
  let b = f32x4(5.0, 6.0, 7.0, 8.0);
  
  // Addition (element-wise)
  let sum = f32x4.add(a, b); // [6.0, 8.0, 10.0, 12.0]
  
  // Subtraction (element-wise)
  let diff = f32x4.sub(b, a); // [4.0, 4.0, 4.0, 4.0]
  
  // Multiplication (element-wise)
  let prod = f32x4.mul(a, b); // [5.0, 12.0, 21.0, 32.0]
  
  // Division (element-wise)
  let quot = f32x4.div(b, a); // [5.0, 3.0, 2.33..., 2.0]
}
```

### Bitwise Operations

```ruchy
fun bitwise_operations() {
  let a = i32x4(0xFF, 0x00, 0xF0, 0x0F);
  let b = i32x4(0x0F, 0xF0, 0xFF, 0x00);
  
  // Bitwise AND
  let and_result = v128.and(a, b);
  
  // Bitwise OR
  let or_result = v128.or(a, b);
  
  // Bitwise XOR
  let xor_result = v128.xor(a, b);
  
  // Bitwise NOT (complement)
  let not_result = v128.not(a);
}
```

### Comparison Operations

```ruchy
fun comparison_operations() {
  let a = f32x4(1.0, 2.0, 3.0, 4.0);
  let b = f32x4(4.0, 3.0, 2.0, 1.0);
  
  // Element-wise equality comparison
  // Results in -1 (all bits set) for true, 0 for false
  let eq = f32x4.eq(a, b); // [0, 0, 0, 0]
  
  // Element-wise greater-than comparison
  let gt = f32x4.gt(a, b); // [0, 0, -1, -1]
  
  // Element-wise less-than comparison
  let lt = f32x4.lt(a, b); // [-1, -1, 0, 0]
}
```

### Data Movement and Shuffling

```ruchy
fun shuffling_operations() {
  let a = i8x16(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
  let b = i8x16(17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32);
  
  // Shuffle bytes from two vectors based on indices
  // Select bytes alternating from a and b
  let indices = i8x16(0, 16, 1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23);
  let shuffled = i8x16.shuffle(a, b, indices);
  // Result: [1, 17, 2, 18, 3, 19, 4, 20, 5, 21, 6, 22, 7, 23, 8, 24]
}
```

## Best Practices for SIMD Usage

### 1. Align Your Data

Memory alignment is crucial for optimal SIMD performance. Misaligned memory accesses can significantly degrade performance.

```ruchy
// Ensure memory alignment for optimal performance
fun aligned_data() {
  // Allocate aligned memory
  let data = new Array(1024);
  
  // Process with SIMD in chunks of 4 (for f32x4)
  for (let i = 0; i < data.length; i += 4) {
    // Load aligned data
    let v = v128.load(data.buffer, i);
    
    // Process data with SIMD operations
    let processed = f32x4.mul(v, f32x4.splat(2.0));
    
    // Store processed data back
    v128.store(data.buffer, i, processed);
  }
}
```

### 2. Vectorize Inner Loops

Focus on vectorizing the innermost loops of your application for maximum performance gain.

```ruchy
// Scalar implementation
fun scalar_dot_product(a, b, len) {
  let sum = 0.0;
  for (let i = 0; i < len; i++) {
    sum += a[i] * b[i];
  }
  return sum;
}

// SIMD implementation
fun simd_dot_product(a, b, len) {
  let sum_vec = f32x4.splat(0.0);
  
  // Process 4 elements at a time
  for (let i = 0; i < len; i += 4) {
    let va = v128.load(a.buffer, i);
    let vb = v128.load(b.buffer, i);
    sum_vec = f32x4.add(sum_vec, f32x4.mul(va, vb));
  }
  
  // Horizontal sum of the vector elements
  let sum = f32x4.extract_lane(sum_vec, 0) +
            f32x4.extract_lane(sum_vec, 1) +
            f32x4.extract_lane(sum_vec, 2) +
            f32x4.extract_lane(sum_vec, 3);
  
  // Handle remaining elements
  let rem = len % 4;
  for (let i = len - rem; i < len; i++) {
    sum += a[i] * b[i];
  }
  
  return sum;
}
```

### 3. Consider Data Layout

Sometimes restructuring your data from Array of Structures (AoS) to Structure of Arrays (SoA) can significantly improve SIMD performance.

```ruchy
// Array of Structures (less efficient for SIMD)
struct Particle {
  x: f32,
  y: f32,
  z: f32,
  mass: f32
}

// Structure of Arrays (more efficient for SIMD)
struct ParticleSystem {
  x: Array<f32>,
  y: Array<f32>,
  z: Array<f32>,
  mass: Array<f32>
}

fun update_particles(particles: ParticleSystem, count: i32, dt: f32) {
  let dt_vec = f32x4.splat(dt);
  let gravity = f32x4.splat(9.81);
  
  for (let i = 0; i < count; i += 4) {
    // Load 4 particles' data
    let x = v128.load(particles.x.buffer, i);
    let y = v128.load(particles.y.buffer, i);
    let velocities = v128.load(particles.velocities.buffer, i);
    
    // Update positions: position += velocity * dt
    let dx = f32x4.mul(velocities, dt_vec);
    let new_x = f32x4.add(x, dx);
    
    // Apply gravity: velocity += gravity * dt
    let dv = f32x4.mul(gravity, dt_vec);
    let new_velocities = f32x4.add(velocities, dv);
    
    // Store updated data
    v128.store(particles.x.buffer, i, new_x);
    v128.store(particles.velocities.buffer, i, new_velocities);
  }
}
```

### 4. Provide Scalar Fallbacks

Always implement a scalar fallback for environments where SIMD might not be available.

```ruchy
fun process_image(pixels, width, height) {
  // Check if SIMD is available
  if (wasm_simd_supported()) {
    process_image_simd(pixels, width, height);
  } else {
    process_image_scalar(pixels, width, height);
  }
}
```

## Browser and Runtime Compatibility

WASM SIMD support varies across browsers and JavaScript runtimes:

| Browser/Runtime | SIMD Support | Notes |
|-----------------|--------------|-------|
| Chrome | Yes (since v91) | Full support for SIMD instruction set |
| Firefox | Yes (since v89) | Full support |
| Safari | Yes (since v16.4) | Full support |
| Edge | Yes (since v91) | Based on Chromium |
| Node.js | Yes (since v16.4) | Full support with V8 engine |
| Deno | Yes | Full support |

Always check for SIMD support at runtime before using SIMD instructions:

```ruchy
fun wasm_simd_supported() {
  try {
    // Try to instantiate a module with SIMD instructions
    const bytes = new Uint8Array([
      0x00, 0x61, 0x73, 0x6d, // wasm magic
      0x01, 0x00, 0x00, 0x00, // wasm version
      // Module with a single SIMD instruction
    ]);
    WebAssembly.validate(bytes);
    return true;
  } catch (e) {
    return false;
  }
}
```

## Performance Considerations

### 1. Measuring SIMD Performance

Always measure performance with and without SIMD to ensure you're actually gaining speed:

```ruchy
fun benchmark_function(f, args, iterations) {
  let start = performance.now();
  for (let i = 0; i < iterations; i++) {
    f(...args);
  }
  let end = performance.now();
  return (end - start) / iterations;
}

// Compare scalar vs SIMD performance
let scalar_time = benchmark_function(scalar_dot_product, [a, b, len], 1000);
let simd_time = benchmark_function(simd_dot_product, [a, b, len], 1000);
console.log(`Speedup: ${scalar_time / simd_time}x`);
```

### 2. Common Optimization Patterns

#### Loop Unrolling with SIMD

```ruchy
fun optimized_array_sum(array, length) {
  let sum0 = f32x4.splat(0.0);
  let sum1 = f32x4.splat(0.0);
  let sum2 = f32x4.splat(0.0);
  let sum3 = f32x4.splat(0.0);
  
  // Process 16 elements per iteration
  for (let i = 0; i < length; i += 16) {
    sum0 = f32x4.add(sum0, v128.load(array.buffer, i));
    sum1 = f32x4.add(sum1, v128.load(array.buffer, i + 4));
    sum2 = f32x4.add(sum2, v128.load(array.buffer, i + 8));
    sum3 = f32x4.add(sum3, v128.load(array.buffer, i + 12));
  }
  
  // Combine the partial sums
  let sum = f32x4.add(f32x4.add(sum0, sum1), f32x4.add(sum2, sum3));
  
  // Horizontal sum
  return f32x4.extract_lane(sum, 0) +
         f32x4.extract_lane(sum, 1) +
         f32x4.extract_lane(sum, 2) +
         f32x4.extract_lane(sum, 3);
}
```

#### Avoiding Horizontal Operations

Horizontal operations (operations across lanes) are often less efficient than vertical operations (operations within a lane):

```ruchy
// Less efficient - horizontal operations
fun horizontal_max(v) {
  return Math.max(
    f32x4.extract_lane(v, 0),
    Math.max(
      f32x4.extract_lane(v, 1),
      Math.max(
        f32x4.extract_lane(v, 2),
        f32x4.extract_lane(v, 3)
      )
    )
  );
}

// More efficient - keep operations vertical as long as possible
fun process_max_vertical(arrays, len) {
  let max_vec = f32x4.splat(-Infinity);
  
  for (let i = 0; i < len; i += 4) {
    let v = v128.load(arrays.buffer, i);
    max_vec = f32x4.max(max_vec, v);
  }
  
  // Only do horizontal operations at the end
  return horizontal_max(max_vec);
}
```

### 3. Memory Usage and Cache Considerations

SIMD operations can make more efficient use of CPU caches, but they also have specific memory access patterns that can affect performance.

```ruchy
// Ensure data is in cache for SIMD operations
fun prefetch_data(data, length) {
  const CACHE_LINE_SIZE = 64; // Typical cache line size in bytes
  
  // Prefetch data in cache line increments
  for (let i = 0; i < length; i += CACHE_LINE_SIZE / 4) {
    // Touch the data to bring it into cache
    let temp = data[i];
  }
}
```

## Common Application Domains

### Image Processing

```ruchy
fun grayscale_filter(rgba_data, width, height) {
  // Weights for converting RGB to grayscale
  let weights = f32x4(0.299, 0.587, 0.114, 0.0);
  
  for (let i = 0; i < width * height; i++) {
    // Load 4 bytes (R, G, B, A)
    let pixel = v128.load(rgba_data.buffer, i * 4);
    
    // Convert to floating point
    let fpixel = f32x4.convert_i32x4(pixel);
    
    // Calculate grayscale value (dot product with weights)
    let gray_value = f32x4.dot(fpixel, weights);
    
    // Create grayscale pixel (R=G=B=gray, A unchanged)
    let gray_pixel = f32x4(
      gray_value, gray_value, gray_value,
      f32x4.extract_lane(fpixel, 3)
    );
    
    // Convert back to bytes and store
    v128.store(rgba_data.buffer, i * 4, f32x4.convert_f32x4(gray_pixel));
  }
}
```

### Matrix Operations

```ruchy
// Matrix multiplication with SIMD
fun matrix_multiply_simd(a, b, c, n) {
  // Assumes square matrices of size n×n
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < n; j += 4) {
      let row_sum = f32x4.splat(0.0);
      
      for (let k = 0; k < n; k++) {
        // Load 4 elements from matrix B's row
        let b_elements = v128.load(b.buffer, (k * n + j) * 4);
        
        // Broadcast the element from matrix A
        let a_element = f32x4.splat(a[i * n + k]);
        
        // Multiply and accumulate
        row_sum = f32x4.add(row_sum, f32x4.mul(a_element, b_elements));
      }
      
      // Store 4 elements of the result
      v128.store(c.buffer, (i * n + j) * 4, row_sum);
    }
  }
}
```

### Physics Simulations

```ruchy
fun particle_physics_step(positions, velocities, forces, masses, count, dt) {
  let dt_vec = f32x4.splat(dt);
  
  for (let i = 0; i < count; i += 4) {
    // Load data
    let pos_x = v128.load(positions.x.buffer, i);
    let pos_y = v128.load(positions.y.buffer, i);
    let pos_z = v128.load(positions.z.buffer, i);
    
    let vel_x = v128.load(velocities.x.buffer, i);
    let vel_y = v128.load(velocities.y.buffer, i);
    let vel_z = v128.load(velocities.z.buffer, i);
    
    let force_x = v128.load(forces.x.buffer, i);
    let force_y = v128.load(forces.y.buffer, i);
    let force_z = v128.load(forces.z.buffer, i);
    
    let mass = v128.load(masses.buffer, i);
    
    // Calculate acceleration: a = F/m
    let acc_x = f32x4.div(force_x, mass);
    let acc_y = f32x4.div(force_y, mass);
    let acc_z = f32x4.div(force_z, mass);
    
    // Update velocity: v += a * dt
    let new_vel_x = f32x4.add(vel_x, f32x4.mul(acc_x, dt_vec));
    let new_vel_y = f32x4.add(vel_y, f32x4.mul(acc_y, dt_vec));
    let new_vel_z = f32x4.add(vel_z, f32x4.mul(acc_z, dt_vec));
    
    // Update position: p += v * dt
    let new_pos_x = f32x4.add(pos_x, f32x4.mul(new_vel_x, dt_vec));
    let new_pos_y = f32x4.add(pos_y, f32x4.mul(new_vel_y, dt_vec));
    let new_pos_z = f32x4.add(pos_z, f32x4.mul(new_vel_z, dt_vec));
    
    // Store updated data
    v128.store(positions.x.buffer, i, new_pos_x);
    v128.store(positions.y.buffer, i, new_pos_y);
    v128.store(positions.z.buffer, i, new_pos_z);
    
    v128.store(velocities.x.buffer, i, new_vel_x);
    v128.store(velocities.y.buffer, i, new_vel_y);
    v128.store(velocities.z.buffer, i, new_vel_z);
  }
}
```

## Conclusion

WebAssembly SIMD provides significant performance benefits for computationally intensive applications. By understanding the available SIMD operations and following best practices, you can optimize your Ruchy code to take full advantage of modern hardware capabilities.

When implementing SIMD optimizations:

1. Start with a correct scalar implementation
2. Measure performance to identify hotspots
3. Apply SIMD optimizations to critical sections
4. Validate correctness against scalar version
5. Benchmark to verify performance improvements
6. Provide scalar fallbacks for compatibility

The examples in this guide demonstrate how to leverage SIMD for various application domains. By adapting these patterns to your specific use cases, you can achieve substantial performance improvements for your WebAssembly applications.