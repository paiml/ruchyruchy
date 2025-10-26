# WASM-007: Browser Debugging Integration - RED Phase Plan

## Overview

The RED phase for WASM-007 focuses on implementing browser debugging integration for the WebAssembly compilation target. This feature will enable developers to debug Ruchy code directly in browser DevTools by generating source maps, debug symbols, and integrating with Chrome/Firefox DevTools.

## Objectives

1. **Source Map Generation**: Generate source maps that map WebAssembly instructions back to Ruchy source code
2. **Debug Symbol Generation**: Generate debug information for variables, functions, and types
3. **DevTools Integration**: Enable debugging in Chrome DevTools and Firefox Developer Tools
4. **Breakpoint Support**: Support setting breakpoints in Ruchy source code
5. **Variable Inspection**: Enable inspecting Ruchy variables during debugging

## Technical Background

### Source Maps for WebAssembly

Source maps are JSON files that map compiled code back to original source code. For WebAssembly, we need:

**Source Map Format** (Source Map v3):
```json
{
  "version": 3,
  "sources": ["module.ruchy"],
  "names": ["main", "x", "y", "add"],
  "mappings": "AAAA,CAAC...",
  "sourceRoot": "",
  "sourcesContent": ["fun main() { ... }"]
}
```

**DWARF Debug Information**:
WebAssembly supports DWARF debugging format embedded in custom sections:
- `.debug_info` - Debug information entries
- `.debug_line` - Line number information
- `.debug_abbrev` - Abbreviation tables
- `.debug_str` - String table

### Browser DevTools Integration

**Chrome DevTools**:
- Supports WebAssembly debugging via source maps
- DWARF support in progress
- Requires `.wasm.map` file alongside `.wasm`

**Firefox Developer Tools**:
- Full DWARF support for WebAssembly
- Source map support
- Better WebAssembly debugging experience

## RED Phase Test Strategy

### Test Category 1: Source Map Generation (10 tests)

**Goal**: Verify source maps correctly map WASM to Ruchy source

**Tests**:

1. **test_source_map_basic_function**
   - Input: Simple Ruchy function
   - Expected: Source map with correct mappings
   - Verify: Line numbers map correctly

2. **test_source_map_multiple_functions**
   - Input: Multiple functions in one file
   - Expected: Source map with entries for each function
   - Verify: Function boundaries preserved

3. **test_source_map_expressions**
   - Input: Complex expressions (binary, unary, calls)
   - Expected: Fine-grained expression mappings
   - Verify: Each expression maps to source

4. **test_source_map_control_flow**
   - Input: If/else, loops, match statements
   - Expected: Mappings for each branch
   - Verify: Control flow preserved

5. **test_source_map_multi_file**
   - Input: Multiple source files (modules)
   - Expected: Combined source map
   - Verify: All files represented

6. **test_source_map_format_compliance**
   - Input: Any Ruchy code
   - Expected: Valid Source Map v3 format
   - Verify: JSON schema compliance

7. **test_source_map_names_section**
   - Input: Functions with parameters and locals
   - Expected: Names section populated
   - Verify: All names preserved

8. **test_source_map_inline_content**
   - Input: Source code
   - Expected: sourcesContent includes original
   - Verify: Content matches input

9. **test_source_map_accuracy**
   - Input: Known positions (line 5, column 10)
   - Expected: Exact mapping to WASM offset
   - Verify: Mapping accuracy <5 bytes

10. **test_source_map_optimization_resilience**
    - Input: Optimized WASM
    - Expected: Source map still valid
    - Verify: Mappings not broken by opts

### Test Category 2: Debug Symbol Generation (10 tests)

**Goal**: Verify DWARF debug information is correctly generated

**Tests**:

1. **test_debug_info_functions**
   - Input: Functions with parameters
   - Expected: DW_TAG_subprogram entries
   - Verify: Function metadata complete

2. **test_debug_info_variables**
   - Input: Local variables
   - Expected: DW_TAG_variable entries
   - Verify: Variable names and types

3. **test_debug_info_types**
   - Input: Ruchy types (i32, String, struct)
   - Expected: DW_TAG_base_type entries
   - Verify: Type system mapped to DWARF

4. **test_debug_info_line_numbers**
   - Input: Multi-line function
   - Expected: .debug_line section
   - Verify: Line table correctness

5. **test_debug_info_scopes**
   - Input: Nested scopes (blocks)
   - Expected: DW_TAG_lexical_block entries
   - Verify: Scope hierarchy preserved

6. **test_debug_info_inlining**
   - Input: Inlined function
   - Expected: DW_TAG_inlined_subroutine
   - Verify: Inlining tracked

7. **test_debug_info_compilation_unit**
   - Input: Source file
   - Expected: DW_TAG_compile_unit
   - Verify: File metadata complete

8. **test_debug_info_string_table**
   - Input: Various names (functions, variables)
   - Expected: .debug_str section
   - Verify: All strings deduplicated

9. **test_debug_info_abbreviation_table**
   - Input: Any code
   - Expected: .debug_abbrev section
   - Verify: Abbreviations valid

10. **test_debug_info_custom_section**
    - Input: Debug-enabled compilation
    - Expected: Custom DWARF sections in WASM
    - Verify: Sections parseable

### Test Category 3: DevTools Integration (10 tests)

**Goal**: Verify debugging works in actual browser DevTools

**Tests**:

1. **test_devtools_load_source_map**
   - Action: Load WASM in Chrome DevTools
   - Expected: Source map loaded
   - Verify: Ruchy source displayed

2. **test_devtools_set_breakpoint**
   - Action: Set breakpoint in Ruchy source
   - Expected: Breakpoint hits
   - Verify: Execution pauses correctly

3. **test_devtools_step_through**
   - Action: Step through Ruchy code
   - Expected: Steps follow Ruchy lines
   - Verify: No jumps to WASM

4. **test_devtools_inspect_variables**
   - Action: Inspect local variable
   - Expected: Variable value shown
   - Verify: Correct value and type

5. **test_devtools_call_stack**
   - Action: Pause in nested call
   - Expected: Call stack shows Ruchy functions
   - Verify: Stack trace accurate

6. **test_devtools_watch_expressions**
   - Action: Add watch expression
   - Expected: Expression evaluated
   - Verify: Value updates correctly

7. **test_devtools_exception_handling**
   - Action: Trigger runtime error
   - Expected: Exception caught
   - Verify: Source location accurate

8. **test_devtools_async_debugging**
   - Action: Debug async code (promises)
   - Expected: Async boundaries preserved
   - Verify: Can step through async

9. **test_devtools_hot_reload**
   - Action: Modify source and reload
   - Expected: New source map loaded
   - Verify: Breakpoints preserved

10. **test_devtools_performance_profiling**
    - Action: Profile WASM execution
    - Expected: Profile shows Ruchy functions
    - Verify: Function names correct

## Implementation Requirements

### Component 1: Source Map Generator

**File**: `/bootstrap/stage3/source_map_generator.ruchy`

**Key Types**:
```ruchy
pub struct SourceMapGenerator {
    version: u32,
    sources: Vec<String>,
    names: Vec<String>,
    mappings: String,
    sources_content: Vec<String>,
}

pub struct Mapping {
    generated_line: u32,
    generated_column: u32,
    source_index: u32,
    source_line: u32,
    source_column: u32,
    name_index: Option<u32>,
}
```

**Key Functions**:
```ruchy
impl SourceMapGenerator {
    pub fun new() -> Self
    pub fun add_source(&mut self, path: String, content: String) -> usize
    pub fun add_name(&mut self, name: String) -> usize
    pub fun add_mapping(&mut self, mapping: Mapping)
    pub fun generate(&self) -> String  // Generate JSON
}
```

### Component 2: DWARF Generator

**File**: `/bootstrap/stage3/dwarf_generator.ruchy`

**Key Types**:
```ruchy
pub struct DwarfGenerator {
    info_section: Vec<u8>,
    line_section: Vec<u8>,
    abbrev_section: Vec<u8>,
    str_section: Vec<u8>,
}

pub enum DwarfTag {
    CompileUnit,
    Subprogram,
    Variable,
    BaseType,
    LexicalBlock,
}
```

**Key Functions**:
```ruchy
impl DwarfGenerator {
    pub fun new() -> Self
    pub fun add_compile_unit(&mut self, name: String)
    pub fun add_function(&mut self, name: String, line: u32)
    pub fun add_variable(&mut self, name: String, type_id: u32)
    pub fun add_type(&mut self, name: String, size: u32)
    pub fun generate(&self) -> Vec<u8>  // Binary DWARF
}
```

### Component 3: Debug Info Emitter

**File**: `/bootstrap/stage3/debug_info_emitter.ruchy`

**Integration with WASM Compiler**:
```ruchy
pub struct DebugInfoEmitter {
    source_map: SourceMapGenerator,
    dwarf: DwarfGenerator,
    current_source: String,
    current_line: u32,
}

impl DebugInfoEmitter {
    pub fun new() -> Self

    pub fun emit_function_start(&mut self, name: String, line: u32, wasm_offset: u32)
    pub fun emit_expression(&mut self, line: u32, column: u32, wasm_offset: u32)
    pub fun emit_variable(&mut self, name: String, type_name: String)

    pub fun finalize(&self) -> (String, Vec<u8>)  // (source map JSON, DWARF binary)
}
```

### Component 4: Browser Integration

**File**: `/bootstrap/stage3/browser_debug_integration.ruchy`

**DevTools Protocol Support**:
```ruchy
pub struct DevToolsIntegration {
    wasm_module: Vec<u8>,
    source_map: String,
    debug_info: Vec<u8>,
}

impl DevToolsIntegration {
    pub fun new(wasm: Vec<u8>, source_map: String, debug_info: Vec<u8>) -> Self

    pub fun embed_debug_info(&mut self) -> Vec<u8>  // Embed in WASM custom sections
    pub fun write_source_map_file(&self, path: String) -> Result<(), String>
    pub fun generate_html_harness(&self) -> String  // HTML for testing
}
```

## Test Implementation Structure

```
validation/wasm/debugging/
├── test_source_map_red.ruchy           # 10 source map tests
├── test_debug_symbols_red.ruchy        # 10 debug symbol tests
├── test_devtools_integration_red.ruchy # 10 DevTools tests
└── fixtures/
    ├── simple_function.ruchy           # Test input
    ├── multi_function.ruchy
    ├── control_flow.ruchy
    └── expected_source_map.json        # Expected output
```

## Performance Targets

| Metric | Target | Rationale |
|--------|--------|-----------|
| Source map generation | <100ms | Should not slow builds |
| Debug info size | <30% of WASM | Reasonable overhead |
| Mapping accuracy | ±5 bytes | Acceptable for debugging |
| DevTools load time | <1s | Fast debugging startup |
| Breakpoint hit time | <50ms | Responsive debugging |

## Success Criteria for RED Phase

✅ **30 Failing Tests Created**: Comprehensive test coverage

✅ **Source Map Tests**: 10 tests covering all mapping scenarios

✅ **Debug Symbol Tests**: 10 tests covering DWARF generation

✅ **DevTools Tests**: 10 tests covering browser integration

✅ **Requirements Clear**: Each test specifies implementation needs

✅ **Targets Established**: Performance and accuracy targets documented

## Known Challenges

### Challenge 1: DWARF Complexity

**Problem**: DWARF format is complex with many interdependencies

**Mitigation**:
- Start with minimal DWARF subset
- Use existing libraries where possible
- Incremental implementation
- Extensive testing

### Challenge 2: Browser Compatibility

**Problem**: Chrome and Firefox have different debugging capabilities

**Mitigation**:
- Test on both browsers
- Implement common subset
- Document browser-specific features
- Provide fallbacks

### Challenge 3: Optimization vs Debug Info

**Problem**: Optimizations can break source mappings

**Mitigation**:
- Maintain debug info through optimizations
- Provide debug builds (unoptimized)
- Track transformations
- Conservative mappings

### Challenge 4: Source Map Accuracy

**Problem**: Mapping WASM instructions to source lines

**Mitigation**:
- Track source positions during compilation
- Fine-grained position tracking
- Test with known positions
- Accept ±5 byte tolerance

## Next Steps (GREEN Phase)

Once RED phase is complete:

1. **GREEN Phase**: Implement minimal debugging support
   - Basic source map generation
   - Simple DWARF info
   - Chrome DevTools integration

2. **REFACTOR Phase**: Optimize and enhance
   - Full DWARF support
   - Firefox DevTools
   - Performance optimization

3. **TOOL Phase**: Comprehensive validation
   - Cross-browser testing
   - Real-world debugging scenarios
   - Performance benchmarking

## Conclusion

The RED phase will establish comprehensive requirements for browser debugging integration through 30 failing tests covering source maps, debug symbols, and DevTools integration. This will enable developers to debug Ruchy code directly in browser DevTools with full source mapping and variable inspection.

---

**Phase**: RED
**Status**: PLANNED
**Target**: Enable browser debugging with source maps and DWARF
**Timeline**: 1 week
**Tests to Create**: 30 failing tests
