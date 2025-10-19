## Bug Report: String.chars().nth() Method Not Implemented in Runtime

**Ruchy Version**: ruchy 3.93.0
**Project**: RuchyRuchy Bootstrap Compiler
**Ticket**: BOOTSTRAP-002 (Character Stream Processing)

### Reproduction Steps
1. Create a String variable
2. Call `.chars()` method to get character iterator
3. Call `.nth(index)` method on the iterator
4. Run with `ruchy run`

### Minimal Reproduction Code
```ruchy
// Minimal reproduction for String.chars().nth() method not implemented
// Ruchy v3.93.0

fn main() {
    let input = "hello";
    let c = input.chars().nth(0);

    match c {
        Some(ch) => println("Char: {}", ch.to_string()),
        None => println("No char")
    }
}

main();
```

### Expected Behavior
The `.nth()` method should return `Some(char)` for valid indices and `None` for out-of-bounds indices. This is a standard iterator method in Rust that Ruchy appears to model after.

For the test case:
- `input.chars().nth(0)` should return `Some('h')`
- Should print: `"Char: h"`

### Actual Behavior
Runtime error occurs:
```
Error: Evaluation error: Runtime error: Unknown array method: nth
```

### Full Error Output
```bash
$ ruchy check bug_reproduction_string_nth.ruchy
✓ Syntax is valid

$ ruchy run bug_reproduction_string_nth.ruchy
Error: Evaluation error: Runtime error: Unknown array method: nth
```

### Context
I was implementing BOOTSTRAP-002 (Character Stream Processing) for the RuchyRuchy bootstrap compiler. The character stream requires character-by-character access to input strings.

My implementation uses `input.chars().nth(index)` to access characters by index, which is the idiomatic Rust approach. The code passes `ruchy check` (syntax validation) successfully, confirming that the parser supports this method call syntax. However, the runtime does not implement the `.nth()` method on string character iterators.

### Impact
**BLOCKS BOOTSTRAP-002**: Character Stream Processing implementation

The character stream component needs random access to characters by index for:
- Current character retrieval
- Lookahead (peek) functionality
- Position-based character access

Without `.nth()` method, character access by index is not possible using the standard iterator API.

### Workaround Options

**Option 1: Use substring + first char** (inefficient):
```ruchy
fn char_at_index(input: String, index: i32) -> String {
    if index >= input.len() {
        "\0"
    } else {
        input.substring(index, index + 1)
    }
}
```
This creates a new substring for each character access - O(n) instead of O(1).

**Option 2: Convert to character array** (if supported):
```ruchy
let chars = input.chars().collect();
let c = chars[index];
```
Need to verify if array indexing on collected chars is supported.

**Option 3: Iterate and count** (very inefficient):
```ruchy
fn char_at_index(input: String, index: i32) -> Option<char> {
    let mut count = 0;
    for c in input.chars() {
        if count == index {
            return Some(c);
        }
        count += 1;
    }
    None
}
```
This is O(n) for each access, making lexer performance O(n²) overall.

### Environment
- OS: Linux (6.8.0-85-generic)
- Ruchy install: ruchy 3.93.0
- Working directory: /home/noah/src/ruchyruchy

### Additional Notes

**Runtime Method Gap**: The error message "Unknown array method: nth" suggests the runtime is treating the character iterator as an array but doesn't recognize `.nth()` as a valid method.

**Related Documentation**: BOUNDARIES.md documents other string method limitations discovered during this project:
- `String.clone()` - not implemented (use `.to_string()`)
- `String.push_str()` - not implemented (use `+` operator)

This appears to be another case where the parser accepts method syntax but the runtime hasn't implemented the method.

**Files Affected**:
- `bootstrap/stage0/char_stream_v3.ruchy` (implementation blocked)
- `bug_reproduction_string_nth.ruchy` (minimal repro case)

**Request**: Please implement `.nth(index)` method for string character iterators, or document which character access methods are available in the runtime. Alternative suggestions for efficient character-by-index access would be helpful.

**Severity**: HIGH - Blocks lexer implementation which is foundational for bootstrap compiler
