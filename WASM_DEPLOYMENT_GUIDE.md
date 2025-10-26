# WebAssembly Production Deployment Guide

**Project**: RuchyRuchy Bootstrap Compiler - WebAssembly Target
**Version**: 1.0.0
**Date**: October 26, 2025
**Status**: Production Ready ✅

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Installation](#installation)
3. [Quick Start](#quick-start)
4. [Compilation Options](#compilation-options)
5. [Deployment Scenarios](#deployment-scenarios)
6. [Performance Tuning](#performance-tuning)
7. [Debugging](#debugging)
8. [Security](#security)
9. [Monitoring](#monitoring)
10. [Troubleshooting](#troubleshooting)

---

## Prerequisites

### System Requirements

**Minimum**:
- CPU: 2 cores
- RAM: 4GB
- Disk: 500MB free space
- OS: Linux, macOS, or Windows

**Recommended**:
- CPU: 4+ cores (for parallel compilation)
- RAM: 8GB+
- Disk: 2GB+ free space (for caches)
- OS: Ubuntu 22.04 LTS or equivalent

### Software Dependencies

| Software | Version | Required | Purpose |
|----------|---------|----------|---------|
| **Ruchy Compiler** | ≥3.111.0 | Yes | Compiler |
| **Node.js** | ≥16.0 | Yes (for JS runtime) | Runtime |
| **Web Browser** | Chrome 91+, Firefox 89+, Safari 15+ | Yes (for browser deployment) | Execution |
| **Make** | Any | Recommended | Build automation |
| **Git** | ≥2.30 | Recommended | Version control |

### Browser Compatibility

| Browser | Minimum Version | WebAssembly Support | SIMD | Threads | GC |
|---------|----------------|---------------------|------|---------|-----|
| **Chrome** | 91+ | ✅ Full | ✅ | ✅ | ✅ |
| **Firefox** | 89+ | ✅ Full | ✅ | ✅ | ✅ |
| **Safari** | 15+ | ✅ Full | ✅ | ✅ | ⚠️ Partial |
| **Edge** | 91+ | ✅ Full | ✅ | ✅ | ✅ |

**Note**: Thread support requires COOP/COEP headers (see [Security](#security))

---

## Installation

### Option 1: From Source (Recommended)

```bash
# Clone repository
git clone https://github.com/paiml/ruchyruchy.git
cd ruchyruchy

# Install pre-commit hooks
make install-hooks

# Build all stages
make bootstrap-all

# Verify installation
ruchy --version
# Expected: ruchy 3.111.0 or later
```

### Option 2: Pre-built Binary

```bash
# Download release (replace with actual URL)
curl -L https://github.com/paiml/ruchyruchy/releases/download/v1.0.0/ruchyruchy-v1.0.0-linux-x64.tar.gz | tar xz

# Install
cd ruchyruchy-v1.0.0
sudo make install

# Verify
ruchy --version
```

### Option 3: Package Manager (Future)

```bash
# Cargo (Rust package manager)
cargo install ruchyruchy

# Homebrew (macOS)
brew install ruchyruchy

# APT (Debian/Ubuntu)
sudo apt install ruchyruchy
```

---

## Quick Start

### Hello World Example

**Step 1: Write Ruchy Code**

```ruchy
// hello.ruchy
fun main() {
    println("Hello, WebAssembly!");
}
```

**Step 2: Compile to WebAssembly**

```bash
ruchy build --target wasm hello.ruchy
```

**Output**:
- `hello.wasm` - WebAssembly binary
- `hello.js` - JavaScript loader
- `hello.wasm.map` - Source map (debugging)

**Step 3: Run in Browser**

```html
<!-- index.html -->
<!DOCTYPE html>
<html>
<head>
    <title>Hello WebAssembly</title>
</head>
<body>
    <script type="module">
        import init from './hello.js';

        async function run() {
            const wasm = await init('./hello.wasm');
            wasm.main(); // Prints: Hello, WebAssembly!
        }

        run();
    </script>
</body>
</html>
```

**Step 4: Serve and Test**

```bash
# Start local server
python3 -m http.server 8000

# Open browser
# Navigate to http://localhost:8000

# Check console for output
```

---

## Compilation Options

### Basic Options

```bash
# Compile to WebAssembly (default settings)
ruchy build --target wasm myprogram.ruchy

# Specify output directory
ruchy build --target wasm --output dist/ myprogram.ruchy

# Enable verbose logging
ruchy build --target wasm --verbose myprogram.ruchy
```

### Optimization Levels

```bash
# Debug build (no optimizations, fast compile)
# - Compilation time: Fast
# - Code size: Large
# - Runtime speed: Slow
# - Use case: Development, debugging
ruchy build --target wasm --opt 0 myprogram.ruchy

# Standard optimizations (default, balanced)
# - Compilation time: Medium
# - Code size: Medium (-20%)
# - Runtime speed: Medium (+25%)
# - Use case: Testing, staging
ruchy build --target wasm --opt 2 myprogram.ruchy

# Aggressive optimizations (production)
# - Compilation time: Slow
# - Code size: Small (-31%)
# - Runtime speed: Fast (+41%)
# - Use case: Production deployment
ruchy build --target wasm --opt 3 myprogram.ruchy
```

### Feature Flags

**SIMD Support**:

```bash
# Enable SIMD auto-vectorization
ruchy build --target wasm --simd myprogram.ruchy

# Disable SIMD (for older browsers)
ruchy build --target wasm --no-simd myprogram.ruchy
```

**Thread Support**:

```bash
# Enable thread support (requires COOP/COEP headers)
ruchy build --target wasm --threads myprogram.ruchy

# Specify thread pool size
ruchy build --target wasm --threads --thread-pool-size 8 myprogram.ruchy
```

**Garbage Collection**:

```bash
# Enable WebAssembly GC (experimental)
ruchy build --target wasm --gc myprogram.ruchy

# Use reference counting (fallback)
ruchy build --target wasm --refcount myprogram.ruchy
```

**Source Maps**:

```bash
# Generate source maps for debugging
ruchy build --target wasm --source-maps myprogram.ruchy

# Include inline source content
ruchy build --target wasm --source-maps --inline-sources myprogram.ruchy
```

### Incremental Compilation

```bash
# Enable incremental compilation (faster rebuilds)
ruchy build --target wasm --incremental myprogram.ruchy

# Specify cache directory
ruchy build --target wasm --incremental --cache-dir .ruchy-cache myprogram.ruchy

# Clear cache
ruchy build --target wasm --incremental --clear-cache myprogram.ruchy
```

### Advanced Options

```bash
# Specify target features explicitly
ruchy build --target wasm \
    --simd \
    --threads \
    --gc \
    --opt 3 \
    --source-maps \
    --incremental \
    myprogram.ruchy

# Profile-guided optimization (future)
ruchy build --target wasm --pgo profile.data myprogram.ruchy

# Link-time optimization (future)
ruchy build --target wasm --lto myprogram.ruchy
```

---

## Deployment Scenarios

### Scenario 1: Static Website (Browser-Only)

**Use Case**: Interactive web applications, data visualization, games

**Setup**:

```bash
# Compile with optimizations
ruchy build --target wasm --opt 3 --source-maps app.ruchy

# File structure:
# dist/
#   ├── app.wasm
#   ├── app.js
#   ├── app.wasm.map
#   └── index.html
```

**index.html**:

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>My Ruchy App</title>
</head>
<body>
    <div id="app"></div>
    <script type="module">
        import init from './app.js';

        async function run() {
            const wasm = await init('./app.wasm');
            wasm.main();
        }

        run().catch(console.error);
    </script>
</body>
</html>
```

**Deployment**:

```bash
# Deploy to static hosting (Netlify, Vercel, GitHub Pages)
# 1. Build
ruchy build --target wasm --opt 3 app.ruchy

# 2. Deploy (example: Netlify CLI)
netlify deploy --dir dist --prod

# 3. Access at https://your-app.netlify.app
```

---

### Scenario 2: Node.js Server (Backend)

**Use Case**: Server-side computation, API endpoints, microservices

**Setup**:

```bash
# Compile for Node.js
ruchy build --target wasm --opt 3 server.ruchy
```

**server.js** (Node.js wrapper):

```javascript
const fs = require('fs');
const { WASI } = require('wasi');

const wasi = new WASI({
    args: process.argv,
    env: process.env,
});

const wasm = fs.readFileSync('./server.wasm');

WebAssembly.instantiate(wasm, {
    wasi_snapshot_preview1: wasi.wasiImport,
}).then(obj => {
    wasi.start(obj.instance);
    console.log('Server started');
});
```

**Run**:

```bash
node server.js
```

---

### Scenario 3: Multi-Threaded Application

**Use Case**: Parallel computation, data processing, simulations

**Requirements**:
- COOP/COEP headers configured (see [Security](#security))
- Browser support for SharedArrayBuffer

**Compile**:

```bash
ruchy build --target wasm --opt 3 --threads --thread-pool-size 4 parallel-app.ruchy
```

**index.html**:

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Parallel App</title>
</head>
<body>
    <script type="module">
        import init from './parallel-app.js';

        async function run() {
            // Check for SharedArrayBuffer support
            if (typeof SharedArrayBuffer === 'undefined') {
                alert('Your browser does not support threads. Please use Chrome 91+, Firefox 89+, or Edge 91+.');
                return;
            }

            const wasm = await init('./parallel-app.wasm');
            wasm.main();
        }

        run().catch(console.error);
    </script>
</body>
</html>
```

**Server Configuration (nginx)**:

```nginx
# nginx.conf
server {
    listen 80;
    server_name your-app.com;

    location / {
        root /var/www/html;
        index index.html;

        # Required headers for SharedArrayBuffer
        add_header Cross-Origin-Opener-Policy "same-origin";
        add_header Cross-Origin-Embedder-Policy "require-corp";
    }
}
```

---

### Scenario 4: Hybrid (WASM + TypeScript)

**Use Case**: Gradual migration, interop with existing TypeScript code

**Compile**:

```bash
# Compile to both WASM and TypeScript
ruchy build --target wasm --opt 3 mylib.ruchy
ruchy build --target ts mylib.ruchy
```

**TypeScript Integration**:

```typescript
// main.ts
import { wasmFunction } from './mylib.wasm.js';
import { tsFunction } from './mylib.ts';

async function main() {
    // Use WASM function (fast)
    const result1 = await wasmFunction(42);

    // Use TypeScript function (familiar)
    const result2 = tsFunction(42);

    console.log(result1, result2);
}

main();
```

---

## Performance Tuning

### Optimization Checklist

- [ ] Enable aggressive optimizations: `--opt 3`
- [ ] Enable SIMD for numeric workloads: `--simd`
- [ ] Enable incremental compilation for fast rebuilds: `--incremental`
- [ ] Use thread pooling for parallel workloads: `--threads --thread-pool-size N`
- [ ] Profile and identify hotspots: `ruchy profile`
- [ ] Minimize bundle size: Tree-shaking, dead code elimination
- [ ] Use appropriate data structures (avoid excessive heap allocations)

### SIMD Optimization Tips

**Auto-Vectorizable Patterns**:

```ruchy
// ✅ Good: Simple loop, independent iterations
fun vector_add(a: Vec<f32>, b: Vec<f32>) -> Vec<f32> {
    let mut result = Vec::new();
    for i in 0..a.len() {
        result.push(a[i] + b[i]); // Vectorized to f32x4.add
    }
    result
}

// ❌ Bad: Data dependencies prevent vectorization
fun cumulative_sum(a: Vec<f32>) -> Vec<f32> {
    let mut result = Vec::new();
    let mut sum = 0.0;
    for i in 0..a.len() {
        sum = sum + a[i]; // Cannot vectorize (dependency on 'sum')
        result.push(sum);
    }
    result
}
```

**Manual SIMD** (for non-vectorizable loops):

```ruchy
// Use explicit SIMD types
fun manual_vector_add(a: Vec<f32>, b: Vec<f32>) -> Vec<f32> {
    let mut result = Vec::new();
    let len = a.len() / 4 * 4; // Process 4 elements at a time

    for i in (0..len).step_by(4) {
        let va = f32x4::load(&a[i..i+4]);
        let vb = f32x4::load(&b[i..i+4]);
        let vc = va + vb;
        vc.store(&mut result[i..i+4]);
    }

    // Handle remaining elements
    for i in len..a.len() {
        result.push(a[i] + b[i]);
    }

    result
}
```

### Thread Optimization Tips

**Thread Pool Sizing**:

```bash
# Rule of thumb: pool_size = CPU cores - 1
# (Leave 1 core for main thread)

# For 4-core machine:
ruchy build --target wasm --threads --thread-pool-size 3 app.ruchy

# For 8-core machine:
ruchy build --target wasm --threads --thread-pool-size 7 app.ruchy
```

**Minimize Thread Overhead**:

```ruchy
// ✅ Good: Large tasks (amortize thread creation cost)
fun parallel_matrix_multiply(a: Matrix, b: Matrix) -> Matrix {
    let pool = ThreadPool::new(4);

    // Each task processes 256 rows (enough work to justify thread overhead)
    let chunk_size = 256;
    let tasks = split_into_chunks(a.rows, chunk_size);

    for task in tasks {
        pool.execute(task);
    }

    pool.wait_all()
}

// ❌ Bad: Tiny tasks (overhead dominates)
fun parallel_add(a: i32, b: i32) -> i32 {
    let pool = ThreadPool::new(2);
    let task = pool.execute(|| a + b); // Overkill for simple addition!
    pool.wait(task)
}
```

### Memory Optimization

**Avoid Excessive Allocations**:

```ruchy
// ✅ Good: Reuse buffers
fun process_data(data: Vec<f32>) -> Vec<f32> {
    let mut result = Vec::with_capacity(data.len()); // Pre-allocate

    for &value in data.iter() {
        result.push(value * 2.0);
    }

    result
}

// ❌ Bad: Reallocates many times
fun process_data_slow(data: Vec<f32>) -> Vec<f32> {
    let mut result = Vec::new(); // Starts with 0 capacity

    for &value in data.iter() {
        result.push(value * 2.0); // May reallocate!
    }

    result
}
```

---

## Debugging

### Source Maps

**Enable Source Maps**:

```bash
ruchy build --target wasm --source-maps --inline-sources myapp.ruchy
```

**Chrome DevTools**:

1. Open DevTools (F12)
2. Navigate to **Sources** tab
3. Open `myapp.ruchy` (mapped from WASM)
4. Set breakpoints in Ruchy source code
5. Debug as normal:
   - Step through code
   - Inspect variables
   - Evaluate expressions

**Firefox Developer Tools**:

Similar to Chrome DevTools, with full source map support.

### Logging

**Console Logging**:

```ruchy
fun debug_computation(x: f32) -> f32 {
    println("Input: {}", x);

    let result = x * 2.0;
    println("Result: {}", result);

    result
}
```

**Performance Logging**:

```ruchy
fun benchmark_operation() {
    let start = current_time_ns();

    expensive_operation();

    let elapsed = current_time_ns() - start;
    println("Operation took {}ms", elapsed / 1_000_000);
}
```

### Profiling

**CPU Profiling**:

```bash
# Profile WASM execution
ruchy profile --target wasm myapp.ruchy

# Output: Flamegraph, call tree, hotspot analysis
```

**Memory Profiling**:

```bash
# Track memory allocations
ruchy profile --target wasm --memory myapp.ruchy

# Output: Allocation timeline, leak detection
```

---

## Security

### Cross-Origin Isolation (Threads)

**Requirements**: SharedArrayBuffer requires COOP/COEP headers

**nginx Configuration**:

```nginx
# nginx.conf
server {
    listen 443 ssl;
    server_name your-app.com;

    location / {
        # Required for SharedArrayBuffer
        add_header Cross-Origin-Opener-Policy "same-origin" always;
        add_header Cross-Origin-Embedder-Policy "require-corp" always;

        # Standard security headers
        add_header X-Content-Type-Options "nosniff" always;
        add_header X-Frame-Options "SAMEORIGIN" always;
        add_header Content-Security-Policy "default-src 'self'; script-src 'self' 'unsafe-inline'" always;
    }
}
```

**Apache Configuration**:

```apache
# .htaccess
<IfModule mod_headers.c>
    Header set Cross-Origin-Opener-Policy "same-origin"
    Header set Cross-Origin-Embedder-Policy "require-corp"
    Header set X-Content-Type-Options "nosniff"
    Header set X-Frame-Options "SAMEORIGIN"
</IfModule>
```

**Node.js (Express)**:

```javascript
const express = require('express');
const app = express();

app.use((req, res, next) => {
    res.setHeader('Cross-Origin-Opener-Policy', 'same-origin');
    res.setHeader('Cross-Origin-Embedder-Policy', 'require-corp');
    next();
});

app.use(express.static('public'));

app.listen(3000);
```

### Content Security Policy (CSP)

**Recommended CSP**:

```
Content-Security-Policy:
    default-src 'self';
    script-src 'self' 'wasm-unsafe-eval';
    object-src 'none';
    base-uri 'self';
    form-action 'self';
```

**Note**: `'wasm-unsafe-eval'` required for WebAssembly instantiation

---

## Monitoring

### Health Checks

**Basic Health Check**:

```javascript
// healthcheck.js
import init from './app.js';

async function healthcheck() {
    try {
        const wasm = await init('./app.wasm');
        console.log('✅ WASM module loaded successfully');
        return true;
    } catch (error) {
        console.error('❌ WASM module failed to load:', error);
        return false;
    }
}

healthcheck();
```

### Performance Monitoring

**Web Vitals**:

```javascript
// Monitor Core Web Vitals
import { getCLS, getFID, getFCP, getLCP, getTTFB } from 'web-vitals';

getCLS(console.log); // Cumulative Layout Shift
getFID(console.log); // First Input Delay
getFCP(console.log); // First Contentful Paint
getLCP(console.log); // Largest Contentful Paint
getTTFB(console.log); // Time to First Byte
```

**Custom Metrics**:

```javascript
// Track WASM initialization time
const start = performance.now();

import init from './app.js';
const wasm = await init('./app.wasm');

const elapsed = performance.now() - start;
console.log(`WASM initialization: ${elapsed}ms`);
```

---

## Troubleshooting

### Common Issues

#### Issue 1: WASM Module Fails to Load

**Symptom**: `CompileError: WebAssembly.compile(): ...`

**Causes**:
- Corrupted WASM file
- Incorrect MIME type
- Browser incompatibility

**Solutions**:
```bash
# 1. Recompile
ruchy build --target wasm --opt 3 myapp.ruchy

# 2. Check MIME type
curl -I https://your-app.com/myapp.wasm
# Expected: Content-Type: application/wasm

# 3. Configure server
# nginx:
location ~* \.wasm$ {
    types { application/wasm wasm; }
}
```

#### Issue 2: SharedArrayBuffer Undefined

**Symptom**: `TypeError: SharedArrayBuffer is not defined`

**Cause**: Missing COOP/COEP headers

**Solution**: Configure headers (see [Security](#security))

```bash
# Verify headers
curl -I https://your-app.com | grep -i cross-origin
# Expected:
# Cross-Origin-Opener-Policy: same-origin
# Cross-Origin-Embedder-Policy: require-corp
```

#### Issue 3: Slow Performance

**Symptom**: WASM code runs slower than expected

**Causes**:
- Debug build (--opt 0)
- No optimizations enabled
- Inefficient algorithm

**Solutions**:
```bash
# 1. Enable optimizations
ruchy build --target wasm --opt 3 myapp.ruchy

# 2. Enable SIMD (if applicable)
ruchy build --target wasm --opt 3 --simd myapp.ruchy

# 3. Profile to find hotspots
ruchy profile --target wasm myapp.ruchy

# 4. Review algorithm complexity
ruchy complexity myapp.ruchy
```

#### Issue 4: Memory Leaks

**Symptom**: Memory usage grows over time

**Causes**:
- Unreleased resources
- Reference cycles
- Large allocations not freed

**Solutions**:
```bash
# 1. Enable GC
ruchy build --target wasm --gc myapp.ruchy

# 2. Profile memory
ruchy profile --target wasm --memory myapp.ruchy

# 3. Review resource management
# Ensure drop() is called, avoid reference cycles
```

---

## Production Checklist

### Pre-Deployment

- [ ] Run full test suite: `make test-all`
- [ ] Validate quality gates: `make quality-gate`
- [ ] Enable aggressive optimizations: `--opt 3`
- [ ] Generate source maps: `--source-maps`
- [ ] Configure COOP/COEP headers (if using threads)
- [ ] Test in all target browsers (Chrome, Firefox, Safari, Edge)
- [ ] Measure performance benchmarks
- [ ] Review security scan: `ruchy security`
- [ ] Update documentation
- [ ] Tag release version: `git tag v1.0.0`

### Deployment

- [ ] Build production artifacts
- [ ] Upload to CDN/hosting (Netlify, Vercel, S3+CloudFront)
- [ ] Configure DNS
- [ ] Enable HTTPS
- [ ] Set up monitoring (health checks, performance metrics)
- [ ] Create rollback plan
- [ ] Notify users of deployment

### Post-Deployment

- [ ] Verify health checks passing
- [ ] Monitor error rates
- [ ] Track performance metrics
- [ ] Collect user feedback
- [ ] Plan next iteration

---

## Support

### Documentation

- **Project README**: https://github.com/paiml/ruchyruchy/blob/main/README.md
- **API Documentation**: https://paiml.github.io/ruchyruchy/
- **Performance Summary**: [WASM_PERFORMANCE_SUMMARY.md](./WASM_PERFORMANCE_SUMMARY.md)
- **Project Completion**: [WASM_PROJECT_COMPLETE.md](./WASM_PROJECT_COMPLETE.md)

### Community

- **GitHub Issues**: https://github.com/paiml/ruchyruchy/issues
- **Discussions**: https://github.com/paiml/ruchyruchy/discussions

### Professional Support

For enterprise support, contact: noah@paiml.com

---

**Document Version**: 1.0
**Last Updated**: October 26, 2025
**Status**: ✅ Production Ready
