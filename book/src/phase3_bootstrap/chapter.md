# Bootstrap Stage 0: Lexer

Stage 0 of the bootstrap compiler implements lexical analysis - converting source code text into tokens.

## Goal

Build a self-tokenizing lexer in pure Ruchy that can:
- Tokenize its own source code
- Handle 82 different token types
- Track position information (line, column, offset)
- Achieve >10K LOC/s throughput
- Pass 100% of validation tests

## Components

1. **Token Type Definitions** (BOOTSTRAP-001)
   - 82 token types covering keywords, operators, literals, delimiters
   - Keyword lookup functionality
   - Position tracking structures

2. **Character Stream Processing** (BOOTSTRAP-002)
   - Character-by-character input abstraction
   - Lookahead support for multi-character tokens
   - Position tracking integration
   - O(1) character access performance

3. **Core Lexer Implementation** (BOOTSTRAP-003)
   - Main tokenization loop
   - Operator and keyword recognition
   - Literal parsing (numbers, strings, chars)
   - Comment handling
   - Error recovery

## TDD Approach

Each component follows strict TDD:
1. Write tests first (RED)
2. Implement minimal code (GREEN)
3. Refactor for quality (REFACTOR)
4. Validate with `ruchy test`, `ruchy lint`, `ruchy run`

## Ruchy Features Utilized

- **Enum Runtime**: Token types and Position tracking
- **Pattern Matching**: Keyword and token classification
- **String Methods**: Character access and manipulation
- **Control Flow**: Tokenization loop and state machine

## Discoveries

Through dogfooding, we discovered:
- Enum tuple variants work (v3.93.0+)
- String iterator `.nth()` works (v3.94.0+)
- Parser supports more than runtime (documented in BOUNDARIES.md)

## Performance Targets

- Lexer throughput: >10K LOC/s
- Character access: O(1)
- Memory usage: <100MB for 10K LOC input
- Test coverage: 80%+ via `ruchy score`

Read on to see how each component was built using TDD!
