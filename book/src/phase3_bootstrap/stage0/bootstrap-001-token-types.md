# BOOTSTRAP-001: Token Type Definitions

## Context

A lexer needs to classify input characters into tokens. We need to define all 82 token types that the Ruchy language supports, including:
- Keywords (`fun`, `let`, `if`, `while`, etc.)
- Operators (`+`, `-`, `==`, `->`, etc.)
- Literals (numbers, strings, chars, bools)
- Delimiters (`(`, `)`, `{`, `}`, `;`, etc.)
- Special tokens (comments, whitespace, EOF, errors)

## RED: Write Failing Test

_(Note: This ticket was completed before the book was established. Full TDD documentation will be added retrospectively.)_

## GREEN: Minimal Implementation

The implementation uses Ruchy's enum runtime support:

```ruchy
enum TokenType {
    Number,
    String,
    Char,
    Bool,
    Identifier,
    Fun,
    Let,
    // ... 82 total variants
}
```

## Validation

```bash
$ ruchy check bootstrap/stage0/token_v2.ruchy
✓ Syntax is valid

$ ruchy run bootstrap/stage0/token_enum_demo.ruchy
✅ All 82 token types created successfully
```

## Discoveries

- Enum runtime fully supported in v3.92.0+
- 82 token types defined and validated
- Ready for lexer implementation

## Next Steps

With token types defined, we can implement character stream processing (BOOTSTRAP-002) and then the core lexer (BOOTSTRAP-003).

See [token_enum_demo.ruchy](https://github.com/paiml/ruchyruchy/blob/main/bootstrap/stage0/token_enum_demo.ruchy) for full implementation.
