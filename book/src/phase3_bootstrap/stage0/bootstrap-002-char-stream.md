# BOOTSTRAP-002: Character Stream Processing

## Context

The lexer needs to process source code character-by-character with the ability to look ahead for multi-character tokens (like `==`, `->`, `//`). We need a character stream abstraction that:
- Provides O(1) character access by index
- Tracks position (line, column, offset) for error reporting
- Supports lookahead for token recognition
- Handles newlines correctly (increment line, reset column)

## RED: Write Failing Test

First, we wrote a comprehensive test suite for the character stream:

```ruchy
fn test_position_creation() -> bool {
    let pos = position_new(1, 1, 0);
    let line = position_line(pos);
    let col = position_column(pos);
    let offset = position_offset(pos);

    if line == 1 && col == 1 && offset == 0 {
        println("    ✅ Position: (line=1, col=1, offset=0)");
        true
    } else {
        println("    ❌ Position creation failed");
        false
    }
}
```

**Expected**: Position tracking with line, column, and offset fields
**Actual**: No implementation yet - test would fail to compile

## GREEN: Minimal Implementation

### Attempt 1: Enum Tuple Variants (v3.92.0)

We attempted to use enum tuple variants for Position:

```ruchy
enum Position {
    Pos(i32, i32, i32)  // (line, column, offset)
}

fn position_line(pos: Position) -> i32 {
    match pos {
        Position::Pos(line, _, _) => line
    }
}
```

**Result**: ❌ Runtime error: "No match arm matched the value"
**Discovery**: Enum tuple variant pattern matching not yet implemented in v3.92.0 runtime

### Bug Discovery Protocol Applied

Following CLAUDE.md Bug Discovery Protocol:
1. 🚨 **STOPPED THE LINE** - Halted all work
2. 📋 **Filed Bug Report**: Created `GITHUB_ISSUE_enum_tuple_pattern_matching.md`
3. 🔬 **Minimal Reproduction**: Created `bug_reproduction_enum_tuple.ruchy`
4. ⏸️ **Waited for Fix**: No workarounds, waited for runtime fix

**Fix**: Deployed in Ruchy v3.93.0

### Attempt 2: Character Access (v3.93.0)

With enum tuple variants fixed, we implemented character access:

```ruchy
fn char_at_index(input: String, index: i32) -> String {
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
```

**Result**: ❌ Runtime error: "Unknown array method: nth"
**Discovery**: String iterator `.nth()` method not yet implemented in v3.93.0 runtime

### Bug Discovery Protocol Applied Again

1. 🚨 **STOPPED THE LINE** - Halted all work again
2. 📋 **Filed Bug Report**: Created `GITHUB_ISSUE_string_nth_method.md`
3. 🔬 **Minimal Reproduction**: Created `bug_reproduction_string_nth.ruchy`
4. ⏸️ **Waited for Fix**: No workarounds, waited for runtime fix

**Fix**: Deployed in Ruchy v3.94.0

### Attempt 3: Complete Implementation (v3.94.0)

With both fixes in place, full implementation succeeded:

```ruchy
enum Position {
    Pos(i32, i32, i32)
}

fn position_new(line: i32, column: i32, offset: i32) -> Position {
    Position::Pos(line, column, offset)
}

fn position_line(pos: Position) -> i32 {
    match pos {
        Position::Pos(line, _, _) => line
    }
}

fn position_advance_line(pos: Position) -> Position {
    match pos {
        Position::Pos(line, _, offset) => {
            Position::Pos(line + 1, 1, offset + 1)
        }
    }
}

fn char_at_index(input: String, index: i32) -> String {
    if index >= input.len() || index < 0 {
        "\0"
    } else {
        let c = input.chars().nth(index);
        match c {
            Some(ch) => ch.to_string(),
            None => "\0"
        }
    }
}
```

**Result**: ✅ All 8 tests pass (100% success rate)

## REFACTOR: Improvements

No refactoring needed - implementation is clean and focused:
- Clear function names
- Pattern matching makes intent obvious
- Bounds checking prevents panics
- O(1) character access via `.nth()`

## Validation

```bash
$ ruchy --version
ruchy 3.94.0

$ ruchy check bootstrap/stage0/char_stream_v3.ruchy
✓ Syntax is valid

$ ruchy run bootstrap/stage0/char_stream_v3.ruchy
Total Tests: 8
Passed: 8
Failed: 0
Success Rate: 100%
```

### Test Coverage
- ✅ Position creation and field access
- ✅ Position advancement (column and line)
- ✅ Character access with bounds checking
- ✅ Lookahead capability
- ✅ Newline position tracking
- ✅ EOF detection
- ✅ Unicode (ASCII) support
- ✅ O(1) performance validation

## Discoveries

### Runtime Enhancement: Enum Tuple Variants (v3.93.0)

**Issue**: Pattern matching on enum tuple variants failed with "No match arm matched"
**Resolution**: Fixed in Ruchy v3.93.0
**Impact**: Enabled type-safe position tracking with `Position::Pos(i32, i32, i32)`

**Evidence**:
```bash
$ ruchy run bug_reproduction_enum_tuple.ruchy
Line: 1  # ✅ Works in v3.93.0
```

### Runtime Enhancement: String Iterator .nth() (v3.94.0)

**Issue**: `.chars().nth()` failed with "Unknown array method: nth"
**Resolution**: Fixed in Ruchy v3.94.0
**Impact**: Enabled O(1) character-by-index access for lexer

**Evidence**:
```bash
$ ruchy run bug_reproduction_string_nth.ruchy
Char: "h"  # ✅ Works in v3.94.0
```

### Documentation Updates

- **BOUNDARIES.md**: Added BOOTSTRAP-002 discovery section
- **INTEGRATION.md**: Added Character Stream Implementation section
- **CLAUDE.md**: Added Bug Discovery Protocol (STOP THE LINE procedure)

## Next Steps

Character stream is complete and ready for use in BOOTSTRAP-003 (Core Lexer Implementation).

The lexer will use these API functions:
- `position_new(line, col, off)` - Initialize position
- `position_advance_line/column(pos)` - Update position
- `char_at_index(input, idx)` - Get character with lookahead
- Position tracking for error messages

## Code

Full implementation: [bootstrap/stage0/char_stream_v3.ruchy](https://github.com/paiml/ruchyruchy/blob/main/bootstrap/stage0/char_stream_v3.ruchy)

**Lines of Code**: 287
**Test Pass Rate**: 100% (8/8)
**Ruchy Features Used**: Enum tuple variants, pattern matching, string iterator methods
