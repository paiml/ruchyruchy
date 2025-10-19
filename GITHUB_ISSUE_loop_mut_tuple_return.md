## Bug Report: Returning Tuple from Function with Loop and Mutable Variables Fails

**Ruchy Version**: ruchy 3.94.0
**Project**: RuchyRuchy Bootstrap Compiler
**Ticket**: BOOTSTRAP-003 (Core Lexer Implementation)

### Reproduction Steps
1. Create a function that returns a tuple `(i32, i32)`
2. Inside the function, declare mutable variables with `let mut`
3. Use a `loop` that modifies these mutable variables
4. Return a tuple containing the mutable variables
5. Run with `ruchy run`

### Minimal Reproduction Code
```ruchy
// Minimal reproduction: loop with mut variables in function returning tuple
// Ruchy v3.94.0

fn test_loop_mut() -> (i32, i32) {
    let mut idx = 0;
    let mut sum = 0;

    loop {
        if idx >= 5 {
            break;
        }
        sum = sum + idx;
        idx = idx + 1;
    }

    (sum, idx)  // This line causes the error
}

fn main() {
    let result = test_loop_mut();
    let sum = result.0;
    let idx = result.1;

    println("Sum: {}, Index: {}", sum, idx);
}

main();
```

### Expected Behavior
The function should return a tuple `(10, 5)` where:
- `sum` = 0+1+2+3+4 = 10
- `idx` = 5

The calling code should successfully destruct the tuple and print:
```
Sum: 10, Index: 5
```

### Actual Behavior
Runtime error occurs:
```
Error: Evaluation error: Type error: Cannot call non-function value: integer
```

### Full Error Output
```bash
$ ruchy check bug_reproduction_loop_mut_tuple.ruchy
✓ Syntax is valid

$ ruchy run bug_reproduction_loop_mut_tuple.ruchy
Error: Evaluation error: Type error: Cannot call non-function value: integer
```

### Context
I was implementing BOOTSTRAP-003 (Core Lexer Implementation) for the RuchyRuchy bootstrap compiler. The lexer needs to return tokens along with the current position in the input string. The natural pattern is:

```ruchy
fn tokenize_number(input: String, start: i32) -> (Token, i32) {
    let mut idx = start;
    let mut num_str = "".to_string();

    loop {
        let ch = char_at(input, idx);
        if ch == "\0" || !is_digit(ch) {
            break;
        }
        num_str = num_str + ch;
        idx = idx + 1;
    }

    (Token::Tok(TokenType::Number, num_str), idx)  // Fails here
}
```

The code passes `ruchy check` (syntax validation) successfully. However, the runtime fails when trying to return a tuple from a function that contains a loop with mutable variables.

**Important**: Simpler cases DO work:
- ✅ Returning tuple without loop: WORKS
- ✅ Returning tuple without mut: WORKS
- ✅ Loop with mut but not returning tuple: WORKS
- ❌ Loop + mut + tuple return: FAILS

### Impact
**BLOCKS BOOTSTRAP-003**: Core Lexer Implementation

The lexer is the foundational component of the bootstrap compiler. Without the ability to return `(Token, i32)` pairs from tokenization functions, we cannot implement the lexer using the standard pattern where:
- Token represents what was parsed
- i32 represents position after parsing

This pattern is essential for:
- Sequential tokenization (each token knows where next starts)
- Error position tracking
- Efficient parsing without backtracking

### Workaround Options

**Option 1: Use global/closure to store position** (not clean):
```ruchy
let mut global_idx = 0;

fn tokenize_number(input: String, start: i32) -> Token {
    global_idx = start;
    loop {
        // modify global_idx
    }
    Token::Tok(TokenType::Number, num_str)
}
// Caller reads global_idx
```
Issues: Not thread-safe, pollutes global namespace, error-prone

**Option 2: Return array instead of tuple** (if supported):
```ruchy
fn tokenize_number(...) -> [i32; 2] {
    // ...
    [token_id, idx]
}
```
Issues: Loses type safety, cannot return different types

**Option 3: Use separate functions** (inefficient):
```ruchy
fn tokenize_number_value(...) -> Token { /* ... */ }
fn tokenize_number_position(...) -> i32 { /* ... */ }
```
Issues: Duplicates parsing logic, inefficient

**None of these workarounds are acceptable** - they all compromise code quality, type safety, or performance.

### Environment
- OS: Linux (6.8.0-85-generic)
- Ruchy install: ruchy 3.94.0
- Working directory: /home/noah/src/ruchyruchy

### Additional Notes

**Contrast with Working Code**:
This simple tuple return WORKS:
```ruchy
fn make_tuple() -> (i32, i32) {
    (42, 100)  // ✅ Works
}
```

This with enum tuple variant WORKS:
```ruchy
enum Token { Tok(i32, String) }

fn make_token() -> (Token, i32) {
    let tok = Token::Tok(42, "hello".to_string());
    (tok, 100)  // ✅ Works
}
```

But combining with loop + mut FAILS:
```ruchy
fn test_loop_mut() -> (i32, i32) {
    let mut idx = 0;
    loop {
        if idx >= 5 { break; }
        idx = idx + 1;
    }
    (0, idx)  // ❌ Fails
}
```

**Pattern Used Throughout Compiler Development**:
This pattern (returning parsed value + position) is ubiquitous in:
- Parsers
- Lexers
- Scanners
- Any sequential processing

**Files Affected**:
- `bootstrap/stage0/lexer_minimal.ruchy` (implementation blocked)
- `bug_reproduction_loop_mut_tuple.ruchy` (minimal repro)
- `test_tokenize_minimal.ruchy` (isolated test case)

**Request**: Please fix the runtime to support returning tuples from functions containing loops with mutable variables. This is a critical pattern for compiler implementation and many other use cases.

**Severity**: CRITICAL - Blocks fundamental compiler construction patterns
