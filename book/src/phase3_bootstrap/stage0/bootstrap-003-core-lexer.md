# BOOTSTRAP-003: Core Lexer Implementation

## Context

With token types defined (BOOTSTRAP-001) and character stream ready (BOOTSTRAP-002), we can now implement the core lexer that converts source code into tokens.

## Status

🚧 **IN PROGRESS** - This chapter will document the TDD process as we implement the core lexer.

## Requirements

- Main tokenization loop
- Operator recognition (single and multi-character)
- Literal parsing (numbers, strings, chars)
- Comment handling (`//` line comments)
- String literal support
- Error recovery
- Performance target: >10K LOC/s

## TDD Process

This chapter will be written following RED-GREEN-REFACTOR as we implement the lexer.

Stay tuned!
