# Stage 3: Code Generator

This stage implements multi-target code generation (TypeScript and Rust) with self-compilation validation.

## Overview

Stage 3 takes typed ASTs from Stage 2 and generates executable code in multiple target languages:
- TypeScript code emission
- Rust code emission
- Self-compilation validation
- Semantic preservation verification

## Tickets

- **BOOTSTRAP-014**: TypeScript Code Emitter
- **BOOTSTRAP-015**: Rust Code Emitter
- **BOOTSTRAP-016**: Self-Compilation Test

## Success Criteria

✅ Valid TypeScript generated (passes tsc)
✅ Valid Rust generated (passes rustc)
✅ Self-compilation: compiler can compile itself
✅ Semantic preservation: behavior matches source
✅ >10K LOC/s throughput

## Technical Highlights

- **Multi-target**: Single compiler → multiple output languages
- **Idiomatic code**: Generated code follows target language conventions
- **Type preservation**: TypeScript/Rust types match inferred types
- **Bit-identical**: Self-compilation produces identical output
