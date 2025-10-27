# WASM Compilation Plan

**Date**: October 23, 2025  
**Status**: Ready for Implementation  
**Scope**: Ruchy to WASM Compilation Target  

## Executive Summary

This document outlines the implementation plan for adding WebAssembly (WASM) as a compilation target for the RuchyRuchy project. The plan builds on the research documented in `docs/research/WASM_COMPILATION_TARGET.md` and the detailed specification in `docs/specifications/ruchyruchy-wasm.md`.

The WASM compilation target will be implemented following the same multi-target architecture already established for TypeScript and Rust in the bootstrap compiler. This adds a third target that enables browser-based execution, improved performance, and greater portability.

## Implementation Phases

### Phase 0: Validation Spike (1-2 days)

**Objective**: Validate feasibility of the hardest problems before full implementation

**Key Tasks**:
1. Implement closure compilation for WASM
2. Validate memory management approach
3. Test integration with JavaScript host
4. Document findings and adjust implementation plan

**Success Criteria**:
- Functional implementation of closures with captured variables
- Clear strategy for memory management
- Validated approach for JavaScript integration
- Confidence in full implementation timeline

### Phase 1: Foundation (7-11 days)

#### WASM-001: Basic Expression Emitter (3-5 days)

**Tasks**:
- Implement basic WASM emission for literals, variables, operations
- Create function and module wrappers
- Support basic control flow (if/else)
- Add WAT formatting and validation

**Files**:
- `bootstrap/stage3/emit_wasm.ruchy`
- Testing infrastructure in `validation/wasm/`

#### WASM-002: Closure Compilation (2-3 days)

**Tasks**:
- Implement closure compilation strategy from spike
- Support variable capture and environment handling
- Create runtime support for closures

#### WASM-003: Multi-Target Integration (2-3 days)

**Tasks**:
- Integrate WASM into existing multi-target validation
- Add differential testing across all three targets
- Ensure consistent behavior across TypeScript, Rust, and WASM

### Phase 2: Advanced Features (7-8 days)

#### WASM-004: Memory Management (2 days)

**Tasks**:
- Implement linear memory allocation
- Add string and composite type handling
- Create array and object representation

#### WASM-005: Imports & Exports (2 days)

**Tasks**:
- Add module import/export handling
- Implement JavaScript host integration
- Create runtime support libraries

#### WASM-006: Function Tables (1 day)

**Tasks**:
- Implement indirect function calls via tables
- Add support for first-class functions
- Create dynamic dispatch mechanisms

#### WASM-007: Optimization Research (2-3 days)

**Tasks**:
- Research WASM-specific optimization opportunities
- Benchmark different approaches
- Document optimization guidelines

### Phase 3: Self-Compilation (13 days)

**Tasks**:
- Compile Stage 0 (Lexer) to WASM (3 days)
- Compile Stage 1 (Parser) to WASM (3 days)
- Compile Stage 2 (Type Checker) to WASM (3 days)
- Compile Stage 3 (Code Generator) to WASM (4 days)
- Create browser-based demo of the compiler

**Success Criteria**:
- Complete RuchyRuchy bootstrap compiler running in browser as `ruchyruchy.wasm`
- Self-compilation tests passing (compiler compiles itself)
- Browser demo showcasing compilation in real-time

### Phase 4: Performance Optimization (2 weeks)

**Tasks**:
- Optimize WASM emission speed
- Improve runtime performance of generated WASM
- Implement WASM-specific optimizations
- Create comprehensive benchmarking suite
- Document optimization techniques

## Technical Strategy

### Type Mapping (Ruchy to WASM)

| Ruchy Type | WASM Type | Implementation Strategy |
|------------|-----------|-------------------------|
| `i32` | `i32` | Direct mapping |
| `i64` | `i64` | Direct mapping |
| `f32` | `f32` | Direct mapping |
| `f64` | `f64` | Direct mapping |
| `bool` | `i32` | 0 = false, 1 = true |
| `String` | `i32` | Linear memory address (length-prefixed) |
| `Array` | `i32` | Linear memory address (length-prefixed) |
| `Function` | `i32` | Function table index |
| `Object` | `i32` | Linear memory address (struct) |

### Closure Compilation

**Option A: Closure Records** (preferred approach)
- Store function pointer and captured variables in heap
- Pass closure record pointer to function
- Load captured variables from closure record

**Option B: Global Variables** (fallback approach)
- Use global variables for capture
- Simpler but less flexible
- May be useful for initial implementation

### Memory Management Strategy

**Phase 1-2: Host GC Integration**
- Import JavaScript garbage collection
- Use reference counting where appropriate
- Explicit free for determinate lifetimes

**Future: WasmGC**
- Transition to WasmGC when browser support is universal
- Native GC with better performance
- Cleaner implementation

## Validation Strategy

### Testing Approach

**Unit Tests**
- Test each component of the WASM emitter
- Verify WAT generation is correct
- Ensure all language features covered

**Property Testing**
- Verify semantic equivalence across all targets
- Test type preservation properties
- Validate memory safety properties

**Fuzz Testing**
- Generate random valid and invalid programs
- Verify robustness of WASM generation
- Aim for 1M+ fuzz test inputs

**Differential Testing**
- Run same program on TypeScript, Rust, and WASM
- Compare outputs to ensure consistency
- Verify behavior preservation across targets

### Quality Gates

All WASM code must pass:
- `ruchy check` - Syntax validation
- `ruchy lint` - A+ lint grade required
- `ruchy test` - All tests must pass
- `ruchy prove` - Property verification
- `ruchy score` - Quality score >0.8
- `ruchy runtime` - Performance validation

## Integration with Main Ruchy Project

The WASM compilation target will provide valuable feedback to the upstream Ruchy project by:

1. Stress-testing the WASM backend through self-compilation
2. Finding edge cases through differential and property testing
3. Providing performance benchmarks and optimization insights
4. Documenting discovered limitations and workarounds

Any bugs or issues discovered will be filed as GitHub issues following the Bug Discovery Protocol outlined in the specification.

## Success Criteria

The implementation will be considered successful when:

1. **Foundation Phase Complete**
   - All basic WASM emission working (expressions, functions, closures)
   - 20+ tests passing
   - WAT compiles to valid WASM binary
   - WASM executes correctly in Node.js and browsers

2. **Self-Compilation Complete**
   - Full bootstrap compiler running in browser
   - Self-compilation tests passing
   - Browser demo functional

3. **Performance Goals Met**
   - WASM emission speed: ~30 LOC/ms
   - Execution speed: Within 2x of native code
   - Binary size: Reasonable for web deployment

4. **Quality Standards Met**
   - 100% test coverage
   - A+ lint grade
   - Quality score >0.8
   - Zero crashes from fuzz testing

## Timeline

**Estimated Total Duration**: 4-6 weeks

- **Phase 0** (Validation Spike): 1-2 days
- **Phase 1** (Foundation): 7-11 days
- **Phase 2** (Advanced Features): 7-8 days
- **Phase 3** (Self-Compilation): 13 days
- **Phase 4** (Performance): 2 weeks (can overlap with Phase 3)

This timeline includes buffer for unexpected challenges and follows a sustainable development pace.

## Next Steps

1. **Immediate**
   - Implement validation spike for closure compilation
   - Validate memory management approach
   - Document findings and adjust plan if needed

2. **Short-term**
   - Begin implementation of basic WASM emitter
   - Set up testing infrastructure
   - Implement core expression emission

3. **Medium-term**
   - Complete all four implementation phases
   - Create browser demo of the compiler
   - Optimize performance

## Conclusion

Adding WASM as a compilation target is a natural extension of the RuchyRuchy bootstrap compiler's multi-target architecture. The implementation will leverage existing patterns from the TypeScript and Rust emitters while addressing WASM-specific challenges.

The WASM target will significantly enhance the RuchyRuchy project by enabling browser execution, improving performance, and increasing portability. It will also provide valuable feedback to the upstream Ruchy project through stress-testing and differential analysis.

The implementation will follow the Toyota Way principles of careful validation, incremental implementation, built-in quality, and sustainable pace to ensure a successful outcome.

**Decision**: Proceed with implementation starting with the validation spike.