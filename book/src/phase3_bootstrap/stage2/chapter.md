# Stage 2: Type Checker

This stage implements type inference using Algorithm W (Hindley-Milner type system) for the bootstrap compiler.

## Overview

Stage 2 builds upon the parsed AST from Stage 1 and adds type inference capabilities:
- Type environment management
- Unification algorithm with occurs check
- Algorithm W implementation
- Self-typing validation

## Tickets

- **BOOTSTRAP-010**: Type Environment
- **BOOTSTRAP-011**: Unification Algorithm
- **BOOTSTRAP-012**: Algorithm W Implementation
- **BOOTSTRAP-013**: Self-Typing Test

## Success Criteria

✅ Type inference working on all bootstrap stages
✅ Soundness property: well-typed programs don't crash
✅ Self-typing: type checker can type its own code
✅ O(n log n) complexity achieved

## Technical Highlights

- **Hindley-Milner type system**: Automatic type inference
- **Let-polymorphism**: Generalization at let bindings
- **Occurs check**: Prevents infinite types
- **Constraint solving**: Unification-based inference
