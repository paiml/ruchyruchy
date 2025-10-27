# WebAssembly Integration Guide

## Introduction

This guide explains how to use the WebAssembly compilation target in your Ruchy projects. The WebAssembly target allows you to compile Ruchy code to WebAssembly, which can run in browsers, Node.js, and other WebAssembly runtimes.

## Prerequisites

- Ruchy compiler v1.20.0 or later
- Understanding of basic Ruchy programming
- Familiarity with web development concepts (for browser integration)

## Getting Started

### Installation

The WebAssembly target is included in the standard Ruchy compiler. To verify that your installation supports WebAssembly:

```bash
ruchy --version
ruchy --list-targets
```

You should see `wasm` listed among the available compilation targets.

### Basic Compilation

To compile a Ruchy file to WebAssembly:

```bash
ruchy compile example.ruchy --target wasm
```

This generates `example.wasm` (the WebAssembly binary) and `example.js` (JavaScript glue code for easy integration).

### Compilation Options

You can customize the WebAssembly compilation with various options:

```bash
ruchy compile example.ruchy --target wasm \
    --opt-level 3 \
    --debug-info true \
    --source-maps true \
    --target-feature wasm:simd=true \
    --target-feature wasm:bulk-memory=true \
    --mode production
```

Common options:
- `--opt-level`: Optimization level (0-3)
- `--debug-info`: Include debug information
- `--source-maps`: Generate source maps
- `--target-feature`: Enable target-specific features
- `--mode`: Compilation mode (development or production)

## Integrating with Web Projects

### Basic HTML Integration

Here's a minimal example of loading and running a WebAssembly module in a browser:

```html
<!DOCTYPE html>
<html>
<head>
    <title>Ruchy WebAssembly Example</title>
</head>
<body>
    <h1>Ruchy WebAssembly Example</h1>
    <div id="output"></div>

    <script>
        // Load the WebAssembly module
        (async () => {
            // Import the JavaScript glue code
            const ruchy = await import('./example.js');
            
            // Initialize the module
            const instance = await ruchy.default();
            
            // Call a function from the module
            const result = instance.exports.add(5, 3);
            document.getElementById('output').textContent = `Result: ${result}`;
        })();
    </script>
</body>
</html>
```

### Using with JavaScript Frameworks

#### React

```jsx
import { useState, useEffect } from 'react';
import initRuchy from './example.js';

function App() {
    const [result, setResult] = useState(null);
    const [loading, setLoading] = useState(true);
    
    useEffect(() => {
        async function loadWasm() {
            try {
                const instance = await initRuchy();
                const sum = instance.exports.add(5, 3);
                setResult(sum);
            } catch (err) {
                console.error('Failed to load WebAssembly module:', err);
            } finally {
                setLoading(false);
            }
        }
        
        loadWasm();
    }, []);
    
    return (
        <div>
            <h1>Ruchy WebAssembly in React</h1>
            {loading ? <p>Loading...</p> : <p>Result: {result}</p>}
        </div>
    );
}

export default App;
```

#### Vue.js

```vue
<template>
  <div>
    <h1>Ruchy WebAssembly in Vue</h1>
    <p v-if="loading">Loading...</p>
    <p v-else>Result: {{ result }}</p>
  </div>
</template>

<script>
import initRuchy from './example.js';

export default {
  data() {
    return {
      result: null,
      loading: true
    };
  },
  async mounted() {
    try {
      const instance = await initRuchy();
      this.result = instance.exports.add(5, 3);
    } catch (err) {
      console.error('Failed to load WebAssembly module:', err);
    } finally {
      this.loading = false;
    }
  }
};
</script>
```

## Using with Node.js

You can use Ruchy WebAssembly modules in Node.js applications:

```javascript
const fs = require('fs');
const path = require('path');

// Load the WebAssembly module
(async () => {
    try {
        // Node.js doesn't use the JavaScript glue code directly
        // We need to load the WebAssembly module manually
        const wasmBuffer = fs.readFileSync(path.join(__dirname, 'example.wasm'));
        const wasmModule = new WebAssembly.Module(wasmBuffer);
        
        // Create imports object (memory, environment functions, etc.)
        const imports = {
            env: {
                memory: new WebAssembly.Memory({ initial: 10 }),
                // Add any other required imports
                print: (value) => console.log(value),
            }
        };
        
        // Instantiate the module
        const instance = new WebAssembly.Instance(wasmModule, imports);
        
        // Call a function from the module
        const result = instance.exports.add(5, 3);
        console.log(`Result: ${result}`);
    } catch (err) {
        console.error('Failed to load WebAssembly module:', err);
    }
})();
```

For convenience, you can use the WebAssembly integration package:

```bash
npm install ruchy-wasm-loader
```

```javascript
const ruchyLoader = require('ruchy-wasm-loader');

(async () => {
    const module = await ruchyLoader.load('./example.wasm');
    const result = module.add(5, 3);
    console.log(`Result: ${result}`);
})();
```

## Advanced Features

### Memory Management

Ruchy's WebAssembly target manages memory automatically for most types. However, you might need to handle memory explicitly for large data structures or manual optimization:

```ruchy
// Memory management example
fun create_array(size: i32) -> *i32 {
    // Allocate memory for the array
    let ptr = alloc(size * sizeof(i32)) as *i32;
    
    // Initialize the array
    for i in 0..size {
        ptr[i] = i;
    }
    
    return ptr;
}

fun free_array(ptr: *i32) {
    // Free the allocated memory
    free(ptr as *void);
}
```

### Closures

Ruchy supports closures in WebAssembly:

```ruchy
// Closure example
fun make_counter(start: i32) -> fun() -> i32 {
    let count = start;
    return || {
        let current = count;
        count = count + 1;
        return current;
    };
}
```

Internally, closures are implemented using function tables and environment records in linear memory.

### Working with JavaScript

You can define JavaScript interop functions:

```ruchy
// JavaScript interop
@js_import("console.log")
external fun js_console_log(message: string);

@js_import("Math.random")
external fun js_random() -> f64;

fun generate_random_number() -> i32 {
    return (js_random() * 100.0) as i32;
}

fun log_message(message: string) {
    js_console_log(message);
}
```

### WebAssembly SIMD

If your target environment supports SIMD, you can use SIMD operations:

```ruchy
// SIMD example (requires --target-feature wasm:simd=true)
@simd
fun vector_add(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    let mut result: [f32; 4];
    for i in 0..4 {
        result[i] = a[i] + b[i];
    }
    return result;
}
```

## Performance Considerations

### Optimization Levels

The WebAssembly target supports multiple optimization levels:

- **O0**: No optimizations (fastest compilation, largest/slowest code)
- **O1**: Basic optimizations
- **O2**: Recommended level for most use cases (good balance)
- **O3**: Aggressive optimizations (slowest compilation, smallest/fastest code)

Example:

```bash
ruchy compile example.ruchy --target wasm --opt-level 2
```

### Memory Layout

For best performance with complex types:
- Align data structures to natural boundaries
- Use compact representations for frequently accessed data
- Consider using arrays instead of linked structures when possible

### Function Calls

WebAssembly function calls have low overhead, but:
- Indirect calls (via function tables) are slightly slower
- Closures use indirect calls
- Consider inlining critical functions with `@inline` attribute

## Debugging

### Source Maps

Generate source maps for better debugging:

```bash
ruchy compile example.ruchy --target wasm --debug-info true --source-maps true
```

This allows browser DevTools to show original Ruchy source when debugging.

### Runtime Checks

In development mode, you can enable runtime checks:

```bash
ruchy compile example.ruchy --target wasm --mode development
```

This adds checks for:
- Bounds checking
- Null pointer dereferences
- Division by zero
- Other common runtime errors

### Memory Debugging

For memory issues, use the memory profiler:

```bash
ruchy analyze example.wasm --memory-profile
```

This generates a report of memory usage patterns.

## Deployment

### Size Optimization

For production deployment, optimize for size:

```bash
ruchy compile example.ruchy --target wasm \
    --opt-level 3 \
    --mode production \
    --debug-info false
```

Additional size reduction tools:
- `wasm-opt` from Binaryen
- `wasm-strip` to remove debug information

### Serving WebAssembly

When serving WebAssembly files:
- Use the correct MIME type: `application/wasm`
- Enable compression (WebAssembly compresses well)
- Consider using `Content-Encoding: br` (Brotli) for better compression

Example server configuration (Nginx):

```nginx
location *.wasm {
    types { application/wasm wasm; }
    gzip on;
    gzip_types application/wasm;
    # Or for Brotli:
    # brotli on;
    # brotli_types application/wasm;
}
```

### Streaming Compilation

For faster startup, use streaming compilation:

```javascript
// Streaming compilation example
async function loadModule(url) {
    const response = await fetch(url);
    const wasm = await WebAssembly.compileStreaming(response);
    const instance = await WebAssembly.instantiate(wasm, imports);
    return instance.exports;
}
```

## Troubleshooting

### Common Issues

1. **Memory Access Errors**:
   - Usually caused by out-of-bounds access
   - Enable runtime checks in development mode
   - Use array bounds checking

2. **Unexpected Type Errors**:
   - WebAssembly has a stricter type system than JavaScript
   - Ensure types match exactly between Ruchy and JS

3. **Missing Imports**:
   - Check that all required JavaScript functions are provided
   - Verify function signatures match

4. **Performance Issues**:
   - Use higher optimization levels
   - Consider SIMD for numeric computations
   - Profile to find bottlenecks

### Debugging Tools

- Browser DevTools (with source maps)
- `ruchy inspect example.wasm` for binary inspection
- Memory profiler: `ruchy analyze example.wasm --memory-profile`
- Performance analyzer: `ruchy analyze example.wasm --performance`

## Examples

### Complete Web Application

See the repository for a complete example:
```
/examples/wasm/todo-app/
```

This example shows a complete Todo application using:
- Ruchy backend logic compiled to WebAssembly
- React frontend
- Persistent storage using IndexedDB

### Node.js Integration

See the repository for a Node.js example:
```
/examples/wasm/node-server/
```

This example shows a simple HTTP server with:
- Ruchy request handler compiled to WebAssembly
- Node.js HTTP server
- WebAssembly memory management

## Resources

- [Ruchy WebAssembly Documentation](/docs/targets/wasm.md)
- [WebAssembly Official Documentation](https://webassembly.org/docs/)
- [MDN WebAssembly Guide](https://developer.mozilla.org/en-US/docs/WebAssembly)
- [WebAssembly Studio](https://webassembly.studio/) (useful for testing)

## Support

If you encounter issues with the WebAssembly target:
- Check the [GitHub issues](https://github.com/paiml/ruchy/issues) for known problems
- Search the [Ruchy forums](https://forums.ruchy.dev) for solutions
- Open a new issue with complete reproduction steps

## Conclusion

The WebAssembly compilation target opens up new possibilities for running Ruchy code on the web and other WebAssembly environments. By leveraging WebAssembly's performance and security benefits, you can create high-performance applications that run on multiple platforms.

As the WebAssembly ecosystem continues to evolve with features like SIMD, threads, exception handling, and garbage collection, the Ruchy WebAssembly target will continue to improve and add support for these features.