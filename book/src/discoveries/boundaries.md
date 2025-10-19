# Language Boundaries

This page links to the comprehensive boundary documentation maintained in the repository.

## Full Documentation

See [BOUNDARIES.md](https://github.com/paiml/ruchyruchy/blob/main/BOUNDARIES.md) for complete boundary analysis.

## Quick Summary

Through comprehensive dogfooding, we've discovered:

### ✅ Working Features (v3.94.0)

- **Enum Unit Variants**: `enum Status { Success, Pending }`
- **Enum Tuple Variants**: `enum Position { Pos(i32, i32, i32) }`
- **Pattern Matching**: Full support on enums
- **String Methods**: `.len()`, `.to_string()`, `.chars()`, `.nth()`
- **Control Flow**: `for`, `while`, `if`, `match`
- **Functions**: Nested functions, closures
- **Collections**: Basic string operations

### ⚠️ Known Limitations (v3.94.0)

- **Struct Runtime**: Parser supports, runtime does not (yet)
- **vec! Macro**: Parser supports, runtime does not (yet)
- **Some String Methods**: `.clone()`, `.push_str()` not implemented
- **Inline Comments**: Not supported in enum/struct blocks
- **Trailing Comments**: After closing `}` cause parser errors

### 📊 Boundary Testing

- **VALID-005**: 10/10 boundary tests passing (100%)
- **Performance**: Identifier length 1-10K chars, nesting depth 1000+ levels
- **Complexity**: 200+ LOC files, 15+ functions per file

## Discovery Method

Boundaries discovered through:
1. Pure Ruchy dogfooding (`ruchy check`, `ruchy lint`, `ruchy run`)
2. Property-based testing (40,000+ test cases)
3. Fuzz testing (250,000+ test cases)
4. Systematic boundary analysis framework

## Bug Discovery Protocol

When bugs are found:
1. 🚨 STOP THE LINE immediately
2. 📋 File detailed GitHub issue
3. 🔬 Create minimal reproduction
4. ⏸️ Wait for fix (no workarounds)
5. ✅ Verify fix before resuming

See [Bug Discovery Protocol](https://github.com/paiml/ruchyruchy/blob/main/CLAUDE.md#-critical-bug-discovery-protocol) for details.

## Updates

Boundary documentation is continuously updated as new discoveries are made through dogfooding.

**Last Major Update**: October 19, 2025 (BOOTSTRAP-002 discoveries)
**Ruchy Version**: v3.94.0
