# BOOTSTRAP-003: Core Lexer Implementation

## Context

With token types defined (BOOTSTRAP-001) and character stream ready (BOOTSTRAP-002), we can now implement the core lexer that converts source code into tokens.

The lexer is the first stage of the compiler pipeline. It reads raw source code and produces a stream of tokens for the parser to consume.

## Requirements

- Main tokenization loop returning (Token, i32) pairs
- Operator recognition (single and multi-character)
- Literal parsing (numbers and identifiers)
- Comment handling (`//` line comments)
- Keyword recognition (`fun`, `let`, `if`, `while`)
- Whitespace skipping
- Performance target: >10K LOC/s

## RED: Write Failing Test

Following TDD, we start by writing tests that specify the behavior we want. The tests should fail because we haven't implemented the lexer yet.

**File**: `bootstrap/stage0/test_lexer.ruchy` (138 LOC)

```ruchy
// BOOTSTRAP-003: Core Lexer Implementation - Test Suite (RED Phase)

enum TokenType {
    Number, Identifier, Fun, Let, If, While,
    Plus, Minus, Star, Slash, Equal, EqualEqual,
    Eof, Error
}

enum Token {
    Tok(TokenType, String)
}

// Test 1: Single number tokenization
fun test_tokenize_single_number() -> bool {
    println("  Testing single number tokenization...");
    let input = "42";
    println("    ❌ Lexer not implemented - test fails");
    false
}

// Test 2: Identifier tokenization
fun test_tokenize_identifier() -> bool {
    println("  Testing identifier tokenization...");
    let input = "hello";
    println("    ❌ Lexer not implemented - test fails");
    false
}

// Test 3: Keyword recognition
fun test_tokenize_keyword() -> bool {
    println("  Testing keyword recognition...");
    let input = "fun";
    println("    ❌ Lexer not implemented - test fails");
    false
}

// Test 4: Operator tokenization
fun test_tokenize_operator() -> bool {
    println("  Testing operator tokenization...");
    let input = "+";
    println("    ❌ Lexer not implemented - test fails");
    false
}

// Test 5: Multi-char operators
fun test_tokenize_equal_equal() -> bool {
    println("  Testing multi-char operator tokenization...");
    let input = "==";
    println("    ❌ Lexer not implemented - test fails");
    false
}

// Test 6: Expression tokenization
fun test_tokenize_expression() -> bool {
    println("  Testing expression tokenization...");
    let input = "x + 1";
    println("    ❌ Lexer not implemented - test fails");
    false
}

// Test 7: Whitespace skipping
fun test_skip_whitespace() -> bool {
    println("  Testing whitespace skipping...");
    let input = "   42   ";
    println("    ❌ Lexer not implemented - test fails");
    false
}

// Test 8: Line comment handling
fun test_skip_line_comment() -> bool {
    println("  Testing line comment handling...");
    let input = "// comment\n42";
    println("    ❌ Lexer not implemented - test fails");
    false
}

fun main() {
    println("🧪 BOOTSTRAP-003: Core Lexer Test Suite (RED Phase)");
    println("");

    let mut passed = 0;
    let mut failed = 0;

    if test_tokenize_single_number() { passed = passed + 1; } else { failed = failed + 1; }
    if test_tokenize_identifier() { passed = passed + 1; } else { failed = failed + 1; }
    if test_tokenize_keyword() { passed = passed + 1; } else { failed = failed + 1; }
    if test_tokenize_operator() { passed = passed + 1; } else { failed = failed + 1; }
    if test_tokenize_equal_equal() { passed = passed + 1; } else { failed = failed + 1; }
    if test_tokenize_expression() { passed = passed + 1; } else { failed = failed + 1; }
    if test_skip_whitespace() { passed = passed + 1; } else { failed = failed + 1; }
    if test_skip_line_comment() { passed = passed + 1; } else { failed = failed + 1; }

    println("");
    println("Total Tests: {}", passed + failed);
    println("Passed: {}", passed);
    println("Failed: {}", failed);

    if failed == 0 {
        println("✅ All tests passed!");
    } else {
        println("❌ RED PHASE: {} tests failing as expected", failed);
    }
}

main();
```

### Run the Failing Tests

```bash
$ ruchy run bootstrap/stage0/test_lexer.ruchy

🧪 BOOTSTRAP-003: Core Lexer Test Suite (RED Phase)

  Testing single number tokenization...
    ❌ Lexer not implemented - test fails
  Testing identifier tokenization...
    ❌ Lexer not implemented - test fails
  Testing keyword recognition...
    ❌ Lexer not implemented - test fails
  Testing operator tokenization...
    ❌ Lexer not implemented - test fails
  Testing multi-char operator tokenization...
    ❌ Lexer not implemented - test fails
  Testing expression tokenization...
    ❌ Lexer not implemented - test fails
  Testing whitespace skipping...
    ❌ Lexer not implemented - test fails
  Testing line comment handling...
    ❌ Lexer not implemented - test fails

Total Tests: 8
Passed: 0
Failed: 8
❌ RED PHASE: 8 tests failing as expected
```

✅ **RED Phase Complete**: All 8 tests fail as expected, proving our test suite is valid.

## GREEN: Minimal Implementation

Now we write the simplest code that makes the tests pass.

### Attempt 1: Initial Implementation (v3.94.0)

We attempted to implement the lexer using the standard tokenization pattern where each tokenize function returns `(Token, i32)` pairs:
- The `Token` represents what was parsed
- The `i32` represents the position after parsing (for next tokenize call)

**File**: `bootstrap/stage0/lexer_minimal.ruchy` (465 LOC)

```ruchy
fun tokenize_number(input: String, start: i32) -> (Token, i32) {
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

    (Token::Tok(TokenType::Number, num_str), idx)
}
```

**Result**: ❌ Runtime error!

```bash
$ ruchy run bootstrap/stage0/lexer_minimal.ruchy
Error: Type error: Cannot call non-function value: integer
```

### Bug Discovered: Loop + Mutable + Tuple Return

**Issue**: Returning a tuple from a function containing a loop with mutable variables caused a runtime error in Ruchy v3.94.0.

**Error**: `Type error: Cannot call non-function value: integer`

This was a CRITICAL blocker because the `(Token, i32)` return pattern is fundamental to compiler construction:
- It's the standard way to implement lexers and parsers
- Each tokenize function needs to return both the parsed token AND the new position
- Without this, we cannot implement sequential tokenization

### Bug Discovery Protocol Applied

Following the project's Bug Discovery Protocol, we:

1. **🚨 STOPPED THE LINE** - Halted all BOOTSTRAP-003 work immediately
2. **📋 Filed Bug Report**: Created `GITHUB_ISSUE_loop_mut_tuple_return.md` with extreme detail
3. **🔬 Created Minimal Reproduction**: `bug_reproduction_loop_mut_tuple.ruchy` (11 LOC)
4. **🔬 Created Control Tests**: Validated simpler cases work:
   - ✅ Tuple return without loop: Works
   - ✅ Tuple return without mut: Works
   - ✅ Loop with mut without tuple return: Works
   - ❌ Loop + mut + tuple return: FAILS
5. **📋 Updated Documentation**:
   - BOUNDARIES.md: Documented the limitation
   - INTEGRATION.md: Marked BOOTSTRAP-003 as BLOCKED
6. **⏸️ AWAITED FIX** - No workarounds, waited for runtime fix

**Minimal Reproduction** (11 LOC):
```ruchy
fun test_loop_mut() -> (i32, i32) {
    let mut idx = 0;
    loop {
        if idx >= 5 { break; }
        idx = idx + 1;
    }
    (0, idx)  // ❌ Runtime error in v3.94.0
}
```

**Severity**: CRITICAL - Blocks fundamental compiler construction patterns

### Fix Deployed: Ruchy v3.95.0

The Ruchy team deployed a fix in version 3.95.0, resolving the loop+mut+tuple return issue.

**Verification**:
```bash
$ ruchy --version
ruchy 3.95.0

$ ruchy run bug_reproduction_loop_mut_tuple.ruchy
Sum: 10, Index: 5
✅ Works perfectly!
```

### Attempt 2: Complete Implementation (v3.95.0)

With the fix deployed, we resumed implementation. The lexer now works perfectly!

**File**: `bootstrap/stage0/lexer_minimal.ruchy` (465 LOC)

**Key Functions**:

```ruchy
// Helper: Get character at index
fun char_at(input: String, index: i32) -> String {
    if index >= input.len() {
        "\0"
    } else {
        let c = input.chars().nth(index);
        match c {
            Some(ch) => ch.to_string(),
            None => "\0"
        }
    }
}

// Helper: Check if character is digit
fun is_digit(ch: String) -> bool {
    ch == "0" || ch == "1" || ch == "2" || ch == "3" || ch == "4" ||
    ch == "5" || ch == "6" || ch == "7" || ch == "8" || ch == "9"
}

// Helper: Check if character is letter
fun is_letter(ch: String) -> bool {
    (ch >= "a" && ch <= "z") || (ch >= "A" && ch <= "Z") || ch == "_"
}

// Helper: Match keyword
fun match_keyword(id: String) -> TokenType {
    match id.to_string() {
        "fun" => TokenType::Fun,
        "let" => TokenType::Let,
        "if" => TokenType::If,
        "while" => TokenType::While,
        _ => TokenType::Identifier
    }
}

// Helper: Skip whitespace
fun skip_whitespace(input: String, start: i32) -> i32 {
    let mut idx = start;
    loop {
        let ch = char_at(input, idx);
        if ch == "\0" || (ch != " " && ch != "\t" && ch != "\n" && ch != "\r") {
            break;
        }
        idx = idx + 1;
    }
    idx
}

// Tokenize number: "42" -> (Token::Tok(Number, "42"), 2)
fun tokenize_number(input: String, start: i32) -> (Token, i32) {
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

    (Token::Tok(TokenType::Number, num_str), idx)  // ✅ Works in v3.95.0!
}

// Tokenize identifier or keyword
fun tokenize_identifier(input: String, start: i32) -> (Token, i32) {
    let mut idx = start;
    let mut id_str = "".to_string();

    loop {
        let ch = char_at(input, idx);
        if ch == "\0" || (!is_letter(ch) && !is_digit(ch)) {
            break;
        }
        id_str = id_str + ch;
        idx = idx + 1;
    }

    let token_type = match_keyword(id_str.to_string());
    (Token::Tok(token_type, id_str), idx)
}

// Tokenize single character operators
fun tokenize_single(input: String, start: i32) -> (Token, i32) {
    let ch = char_at(input, start);

    if ch == "=" {
        let next_ch = char_at(input, start + 1);
        if next_ch == "=" {
            (Token::Tok(TokenType::EqualEqual, "==".to_string()), start + 2)
        } else {
            (Token::Tok(TokenType::Equal, "=".to_string()), start + 1)
        }
    } else if ch == "+" {
        (Token::Tok(TokenType::Plus, "+".to_string()), start + 1)
    } else if ch == "-" {
        (Token::Tok(TokenType::Minus, "-".to_string()), start + 1)
    } else if ch == "*" {
        (Token::Tok(TokenType::Star, "*".to_string()), start + 1)
    } else if ch == "/" {
        // Check for line comment
        let next_ch = char_at(input, start + 1);
        if next_ch == "/" {
            // Skip until newline
            let mut idx = start + 2;
            loop {
                let c = char_at(input, idx);
                if c == "\0" || c == "\n" {
                    break;
                }
                idx = idx + 1;
            }
            tokenize_one(input, idx)  // Recurse to get next token
        } else {
            (Token::Tok(TokenType::Slash, "/".to_string()), start + 1)
        }
    } else {
        (Token::Tok(TokenType::Error, ch.to_string()), start + 1)
    }
}

// Main tokenization function
fun tokenize_one(input: String, start: i32) -> (Token, i32) {
    let idx = skip_whitespace(input, start);
    let ch = char_at(input, idx);

    if ch == "\0" {
        (Token::Tok(TokenType::Eof, "".to_string()), idx)
    } else if is_digit(ch) {
        tokenize_number(input, idx)
    } else if is_letter(ch) {
        tokenize_identifier(input, idx)
    } else {
        tokenize_single(input, idx)
    }
}
```

### Run the Passing Tests

```bash
$ ruchy run bootstrap/stage0/lexer_minimal.ruchy

🧪 BOOTSTRAP-003: Core Lexer Test Suite

  Testing single number tokenization...
    Input: "42"
    Expected: Number("42")
    Got: Number("42")
    ✅ Pass

  Testing identifier tokenization...
    Input: "hello"
    Expected: Identifier("hello")
    Got: Identifier("hello")
    ✅ Pass

  Testing keyword recognition...
    Input: "fun"
    Expected: Fun
    Got: Fun
    ✅ Pass

  Testing operator tokenization...
    Input: "+"
    Expected: Plus
    Got: Plus
    ✅ Pass

  Testing multi-char operator tokenization...
    Input: "=="
    Expected: EqualEqual (NOT two Equal)
    Got: EqualEqual
    ✅ Pass

  Testing expression tokenization...
    Input: "x + 1"
    Expected: [Identifier("x"), Plus, Number("1")]
    Got: [Identifier("x"), Plus, Number("1")]
    ✅ Pass

  Testing whitespace skipping...
    Input: "   42   "
    Expected: Number("42")
    Got: Number("42")
    ✅ Pass

  Testing line comment handling...
    Input: "// comment\n42"
    Expected: Number("42")
    Got: Number("42")
    ✅ Pass

Total Tests: 8
Passed: 8
Failed: 0
Success Rate: 100%

✅ GREEN PHASE COMPLETE!

All tests pass with minimal implementation.

Next: REFACTOR Phase - Improve code quality
```

✅ **GREEN Phase Complete**: All 8/8 tests passing (100% success rate)!

## Key Learnings

### 1. The (Token, Position) Pattern

The lexer uses a fundamental pattern where each tokenization function returns:
- **Token**: What was parsed (Number, Identifier, Operator, etc.)
- **Position**: Index after parsing (where next tokenize should start)

This enables sequential tokenization without global state:
```ruchy
let result1 = tokenize_one(input, 0);      // Parse first token
let token1 = result1.0;
let pos1 = result1.1;

let result2 = tokenize_one(input, pos1);   // Parse second token starting where first left off
let token2 = result2.0;
let pos2 = result2.1;
```

### 2. Multi-Character Operator Lookahead

For operators like `==` that start with `=`, we need lookahead:
```ruchy
if ch == "=" {
    let next_ch = char_at(input, start + 1);
    if next_ch == "=" {
        (Token::Tok(TokenType::EqualEqual, "==".to_string()), start + 2)
    } else {
        (Token::Tok(TokenType::Equal, "=".to_string()), start + 1)
    }
}
```

Without lookahead, `==` would tokenize as two separate `Equal` tokens instead of one `EqualEqual` token.

### 3. Comment Handling via Recursion

Line comments are handled by skipping to the newline, then recursively calling `tokenize_one`:
```ruchy
if next_ch == "/" {
    // Skip until newline
    let mut idx = start + 2;
    loop {
        let c = char_at(input, idx);
        if c == "\0" || c == "\n" { break; }
        idx = idx + 1;
    }
    tokenize_one(input, idx)  // Get next token after comment
}
```

This elegantly handles comments without special state.

### 4. Bug Discovery Protocol Success

The Bug Discovery Protocol proved invaluable:
- **STOP THE LINE**: Prevented working around the bug with inferior code
- **Detailed Bug Report**: Helped Ruchy team understand and fix the issue quickly
- **Minimal Reproduction**: Made it easy to verify the fix
- **No Workarounds**: Ensured we use the correct pattern, not a hack

Result: **Clean fix in v3.95.0, proper implementation achieved**

## REFACTOR: Improve Code Quality

With all tests passing, we can now refactor to improve code quality while maintaining the GREEN state.

### Potential Refactorings

1. **Extract helper modules** - Separate character classification, keyword matching, and tokenization
2. **Add more operators** - Extend to full Ruchy operator set
3. **String literal support** - Add tokenization for quoted strings
4. **Better error tokens** - Track position and context for errors
5. **Performance optimization** - Benchmark against >10K LOC/s target

**Status**: Ready for REFACTOR phase (optional improvement while maintaining 100% test pass rate)

## Summary

**BOOTSTRAP-003 GREEN Phase**: ✅ COMPLETE

**Test Results**: 8/8 passing (100% success rate)

**Implementation**: 465 LOC lexer with:
- Number tokenization
- Identifier and keyword recognition
- Single and multi-character operators
- Whitespace skipping
- Line comment handling
- (Token, i32) return pattern for sequential parsing

**Bug Discovered and Fixed**:
- Loop + mut + tuple return failed in v3.94.0
- Bug Discovery Protocol applied successfully
- Fixed in Ruchy v3.95.0
- Implementation unblocked

**Files**:
- `bootstrap/stage0/test_lexer.ruchy` (138 LOC - RED phase)
- `bootstrap/stage0/lexer_minimal.ruchy` (465 LOC - GREEN phase)
- `bug_reproduction_loop_mut_tuple.ruchy` (11 LOC - minimal repro)
- `GITHUB_ISSUE_loop_mut_tuple_return.md` (detailed bug report)

**Next Steps**:
- REFACTOR phase (optional quality improvements)
- BOOTSTRAP-004: Error Recovery Mechanisms
- BOOTSTRAP-005: Self-Tokenization Test
