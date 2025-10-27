# Ruchy WebAssembly Counter Example

This example demonstrates how to use the Ruchy WebAssembly compilation target to create a simple counter application with browser integration.

## Overview

The example showcases:
- Compiling Ruchy code to WebAssembly
- JavaScript interop for DOM manipulation
- State management in WebAssembly
- Closures in WebAssembly

## Files

- `counter.ruchy`: The Ruchy source code for the counter implementation
- `index.html`: HTML file with the user interface and JavaScript integration code

## How It Works

1. The `counter.ruchy` file contains the counter implementation in Ruchy, including:
   - Functions for incrementing, decrementing, and resetting the counter
   - JavaScript interop functions for DOM manipulation
   - A closure-based counter implementation for demonstration

2. The Ruchy code is compiled to WebAssembly:
   ```bash
   ruchy compile counter.ruchy --target wasm
   ```

3. This generates:
   - `counter.wasm`: The WebAssembly binary
   - `counter.js`: JavaScript glue code for easy integration

4. The `index.html` file loads the WebAssembly module and sets up the UI:
   - It provides a user interface with increment, decrement, and reset buttons
   - It connects UI events to the WebAssembly functions
   - It demonstrates calling the closure-based counter

## Running the Example

1. Compile the Ruchy code to WebAssembly:
   ```bash
   ruchy compile counter.ruchy --target wasm
   ```

2. Serve the files with a web server:
   ```bash
   python -m http.server
   ```

3. Open `http://localhost:8000` in your browser.

4. Interact with the counter:
   - Click "+" to increment
   - Click "-" to decrement
   - Click "Reset" to set it back to zero
   - Click "Run Demo" to see the closure-based counter in action

## Technical Notes

### JavaScript Interop

The example uses JavaScript interop annotations to interface with the browser DOM:

```ruchy
@js_import("document.getElementById")
external fun js_get_element_by_id(id: string) -> i32;

@js_import("Element.prototype.innerText")
external fun js_set_inner_text(element: i32, text: string);
```

### State Management

The counter state is managed in WebAssembly memory:

```ruchy
static mut COUNTER_VALUE: i32 = 0;
```

### Closures

The example demonstrates closures in WebAssembly:

```ruchy
fun create_counter(start: i32) -> fun() -> i32 {
    let mut count = start;
    
    return || {
        let current = count;
        count += 1;
        return current;
    };
}
```

Internally, this creates a function table entry and an environment record in linear memory.

## Next Steps

1. Try modifying the counter to add more features:
   - Add a multiplier for the increment/decrement
   - Add persistence using localStorage
   - Add animation effects

2. Explore more complex WebAssembly applications:
   - Check out the Todo app example for a more complete application
   - Try creating a game using WebAssembly for the logic

3. Learn about optimizing WebAssembly:
   - Try compiling with different optimization levels
   - Profile the performance
   - Experiment with SIMD instructions

## Resources

For more information, see:
- [WASM Integration Guide](/docs/guides/WASM_INTEGRATION_GUIDE.md)
- [Ruchy WebAssembly Documentation](/docs/targets/wasm.md)
- [WebAssembly Official Documentation](https://webassembly.org/docs/)