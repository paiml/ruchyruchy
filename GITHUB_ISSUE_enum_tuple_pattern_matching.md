## Bug Report: Enum Tuple Variant Pattern Matching Fails at Runtime

**Ruchy Version**: ruchy 3.92.0
**Project**: RuchyRuchy Bootstrap Compiler
**Ticket**: BOOTSTRAP-002 (Character Stream Processing)

### Reproduction Steps
1. Create an enum with a tuple variant containing 3 i32 fields
2. Create an instance of the enum using the tuple variant constructor
3. Attempt to pattern match on the enum to extract fields
4. Run with `ruchy run`

### Minimal Reproduction Code
```ruchy
// Minimal reproduction for enum tuple variant pattern matching bug
// Ruchy v3.92.0

enum Position {
    Pos(i32, i32, i32)
}

fn get_line(pos: Position) -> i32 {
    match pos {
        Position::Pos(line, _, _) => line
    }
}

fn main() {
    let pos = Position::Pos(1, 2, 3);
    let line = get_line(pos);
    println("Line: {}", line);
}

main();
```

### Expected Behavior
The pattern match should successfully match the `Position::Pos(line, _, _)` pattern and extract the first field (line=1), printing "Line: 1".

According to the Ruchy v3.92.0 release notes, tuple variants are supported:
- "Tuple variants: `enum Response { Ok, Error(String) }` with `Response::Error("msg")`"

Pattern matching on enum variants is also listed as supported:
- "Pattern matching on enum variants"

### Actual Behavior
Runtime error occurs:
```
Error: Evaluation error: Runtime error: No match arm matched the value
```

### Full Error Output
```bash
$ ruchy check bug_reproduction_enum_tuple.ruchy
✓ Syntax is valid

$ ruchy run bug_reproduction_enum_tuple.ruchy
Error: Evaluation error: Runtime error: No match arm matched the value
```

### Context
I was implementing BOOTSTRAP-002 (Character Stream Processing) for the RuchyRuchy bootstrap compiler. The character stream requires position tracking with line, column, and offset fields. I attempted to use an enum tuple variant `Position::Pos(i32, i32, i32)` to represent position data, following the v3.92.0 release notes which state tuple variants are supported.

The code passes `ruchy check` (syntax validation) successfully, confirming that the parser supports tuple variant declarations and pattern matching syntax. However, the runtime fails with "No match arm matched the value" when attempting to pattern match on the tuple variant.

### Impact
**BLOCKS BOOTSTRAP-002**: Character Stream Processing implementation

The character stream component is critical for the Stage 0 lexer. Position tracking is essential for:
- Error reporting with line/column information
- Source location preservation
- Debugging information in generated code

Without tuple variant pattern matching, I must either:
1. Use separate i32 variables and tuples (loses type safety and semantic meaning)
2. Use only unit variants (loses data carrying capability)
3. Wait for struct runtime support (also not yet implemented per BOUNDARIES.md)

### Workaround
Currently implementing workaround using unit variants only:
```ruchy
enum PositionState {
    Start,      // Represents (1, 1, 0)
    Middle,     // Represents any intermediate position
    End         // Represents EOF
}
```

This loses the ability to track exact line/column/offset values but allows testing the character stream abstraction concept.

Alternative workaround: Use plain i32 tuples `(i32, i32, i32)` but this loses:
- Type safety (can't distinguish Position from other 3-tuples)
- Pattern matching on Position-specific logic
- Semantic clarity in code

### Environment
- OS: Linux (6.8.0-85-generic)
- Ruchy install: ruchy 3.92.0
- Installation method: [from release/cargo/etc - need to verify]
- Working directory: /home/noah/src/ruchyruchy

### Additional Notes

**Discrepancy Between Parser and Runtime**:
The parser fully supports tuple variant syntax (passes `ruchy check`), but the runtime does not support pattern matching on tuple variants. This suggests the v3.92.0 enum runtime implementation may be incomplete.

**Related**: BOUNDARIES.md documents that struct runtime is not yet implemented. It appears tuple variants may have similar runtime limitations despite parser support.

**Testing**: The RuchyRuchy project is dogfooding Ruchy v3.92.0 for the bootstrap compiler implementation. We maintain comprehensive boundary documentation in BOUNDARIES.md and will update it with this discovery.

**Files Affected**:
- `bootstrap/stage0/char_stream_v2.ruchy` (implementation blocked)
- `bug_reproduction_enum_tuple.ruchy` (minimal repro case)

**Request**: Please clarify the current status of tuple variant pattern matching in the runtime. If this is a known limitation, updating the v3.92.0 release notes to specify "unit variants only" for runtime support would help users avoid this issue.
