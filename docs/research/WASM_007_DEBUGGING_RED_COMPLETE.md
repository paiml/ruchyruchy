# WASM-007: Browser Debugging Integration - RED Phase Complete

## Overview

The RED phase for WASM-007 (Browser Debugging Integration) has been successfully completed with 30 comprehensive failing tests that define the requirements for enabling browser-based debugging of Ruchy code compiled to WebAssembly.

## Accomplishments

### 1. RED Phase Plan Created ✅

**File**: `/docs/research/WASM_007_DEBUGGING_RED_PHASE.md` (447 lines)

Comprehensive RED phase plan covering:
- Source map generation requirements (10 tests)
- Debug symbol generation requirements (10 tests)
- DevTools integration requirements (10 tests)
- Technical specifications for Source Map v3 format
- DWARF debug information requirements
- Browser DevTools integration approach
- Performance targets and success criteria

### 2. Source Map Test Suite ✅

**File**: `/validation/wasm/debugging/test_source_map_red.ruchy` (420 lines)

**10 Failing Tests Created**:

1. **test_source_map_basic_function** - Basic function source mapping
   - Input: Simple Ruchy function
   - Expected: Source map with correct version, sources, names, mappings
   - Verifies: Line numbers map correctly from WASM to Ruchy

2. **test_source_map_multiple_functions** - Multiple function mapping
   - Input: Multiple functions in one file
   - Expected: Source map entries for each function
   - Verifies: Function boundaries preserved in source map

3. **test_source_map_expressions** - Expression-level mapping
   - Input: Complex expressions (binary, unary, calls)
   - Expected: Fine-grained expression mappings
   - Verifies: Each expression maps back to source

4. **test_source_map_control_flow** - Control flow mapping
   - Input: If/else, loops, match statements
   - Expected: Mappings for each branch
   - Verifies: Control flow structure preserved

5. **test_source_map_multi_file** - Multi-file project mapping
   - Input: Multiple source files (modules)
   - Expected: Combined source map with all files
   - Verifies: All source files represented

6. **test_source_map_format_compliance** - Source Map v3 compliance
   - Input: Any Ruchy code
   - Expected: Valid Source Map v3 JSON
   - Verifies: JSON schema compliance, required fields

7. **test_source_map_names_section** - Names section population
   - Input: Functions with parameters and locals
   - Expected: Names section includes all identifiers
   - Verifies: All function, parameter, and variable names preserved

8. **test_source_map_inline_content** - Inline source content
   - Input: Source code
   - Expected: sourcesContent field includes original source
   - Verifies: Content matches input exactly

9. **test_source_map_accuracy** - Mapping accuracy verification
   - Input: Known source positions
   - Expected: Exact mapping to WASM offset
   - Verifies: Mapping accuracy within ±5 bytes

10. **test_source_map_optimization_resilience** - Optimization compatibility
    - Input: Optimized WASM code
    - Expected: Source map still valid after optimization
    - Verifies: Mappings not broken by optimizations

**Key Types Defined**:
```ruchy
struct CompileResult {
    wasm_binary: Vec<u8>,
    source_map: Option<SourceMap>,
    function_offsets: HashMap<String, u32>,
}

struct SourceMap {
    version: u32,
    sources: Vec<String>,
    names: Vec<String>,
    mappings: String,
    sources_content: Vec<String>,
}

struct Mapping {
    generated_line: u32,
    generated_column: u32,
    source_index: u32,
    source_line: u32,
    source_column: u32,
    name_index: Option<u32>,
}
```

### 3. Debug Symbol Test Suite ✅

**File**: `/validation/wasm/debugging/test_debug_symbols_red.ruchy` (560 lines)

**10 Failing Tests Created**:

1. **test_debug_info_functions** - Function debug information
   - Input: Functions with parameters
   - Expected: DW_TAG_subprogram entries in DWARF
   - Verifies: Function metadata (name, line, parameters) complete

2. **test_debug_info_variables** - Variable debug information
   - Input: Local variables in functions
   - Expected: DW_TAG_variable entries
   - Verifies: Variable names, types, and scopes tracked

3. **test_debug_info_types** - Type system debug information
   - Input: Ruchy types (primitives, structs, enums)
   - Expected: DW_TAG_base_type and DW_TAG_structure_type entries
   - Verifies: Type system correctly mapped to DWARF

4. **test_debug_info_line_numbers** - Line number information
   - Input: Multi-line functions
   - Expected: .debug_line section with line table
   - Verifies: Line table correctness and completeness

5. **test_debug_info_scopes** - Lexical scope tracking
   - Input: Nested scopes (blocks)
   - Expected: DW_TAG_lexical_block entries
   - Verifies: Scope hierarchy preserved, variable visibility

6. **test_debug_info_inlining** - Inlined function tracking
   - Input: Inlined functions
   - Expected: DW_TAG_inlined_subroutine entries
   - Verifies: Inlining tracked with call sites

7. **test_debug_info_compilation_unit** - Compilation unit metadata
   - Input: Source file
   - Expected: DW_TAG_compile_unit entry
   - Verifies: File metadata (name, language, producer) complete

8. **test_debug_info_string_table** - DWARF string table
   - Input: Various names (functions, variables)
   - Expected: .debug_str section with deduplicated strings
   - Verifies: All strings present, properly deduplicated

9. **test_debug_info_abbreviation_table** - DWARF abbreviations
   - Input: Any code
   - Expected: .debug_abbrev section with abbreviation table
   - Verifies: Abbreviations valid and complete

10. **test_debug_info_custom_section** - WASM custom section embedding
    - Input: Debug-enabled compilation
    - Expected: DWARF sections embedded in WASM custom sections
    - Verifies: Sections parseable and valid

**Key Types Defined**:
```ruchy
struct DebugInfo {
    functions: HashMap<String, FunctionDebugInfo>,
    types: HashMap<String, TypeDebugInfo>,
    compilation_unit: Option<CompilationUnit>,
    line_table: Option<LineTable>,
    string_table: Option<StringTable>,
    abbrev_table: Option<AbbreviationTable>,
}

struct FunctionDebugInfo {
    name: String,
    line_number: u32,
    parameters: Vec<ParameterInfo>,
    local_variables: Vec<VariableInfo>,
    lexical_blocks: Vec<LexicalBlock>,
    inlined_calls: Vec<InlinedCall>,
}

enum DwarfTag {
    CompileUnit,
    Subprogram,
    Variable,
    BaseType,
    StructType,
    LexicalBlock,
}
```

### 4. DevTools Integration Test Suite ✅

**File**: `/validation/wasm/debugging/test_devtools_integration_red.ruchy` (650 lines)

**10 Failing Tests Created**:

1. **test_devtools_load_source_map** - Source map loading
   - Action: Load WASM with source map in Chrome DevTools
   - Expected: Source map detected and loaded
   - Verifies: Ruchy source displayed in DevTools

2. **test_devtools_set_breakpoint** - Breakpoint setting
   - Action: Set breakpoint in Ruchy source line
   - Expected: Breakpoint resolves and hits
   - Verifies: Execution pauses at correct Ruchy line

3. **test_devtools_step_through** - Step-through debugging
   - Action: Step over Ruchy statements
   - Expected: Steps follow Ruchy source lines
   - Verifies: No jumps into WASM instructions

4. **test_devtools_inspect_variables** - Variable inspection
   - Action: Inspect local variables during pause
   - Expected: Variable values displayed correctly
   - Verifies: Correct values and types shown

5. **test_devtools_call_stack** - Call stack display
   - Action: Pause in nested function call
   - Expected: Call stack shows Ruchy function names
   - Verifies: Stack trace accurate with source locations

6. **test_devtools_watch_expressions** - Watch expressions
   - Action: Add watch expressions for variables
   - Expected: Expressions evaluated and updated
   - Verifies: Values update correctly during execution

7. **test_devtools_exception_handling** - Exception debugging
   - Action: Trigger runtime error
   - Expected: DevTools pauses on exception
   - Verifies: Source location accurate for error

8. **test_devtools_async_debugging** - Async code debugging
   - Action: Debug async/await code
   - Expected: Async boundaries preserved in debugging
   - Verifies: Can step through async code seamlessly

9. **test_devtools_hot_reload** - Hot reload support
   - Action: Modify source and reload
   - Expected: New source map loaded, breakpoints preserved
   - Verifies: Debugging session continuity maintained

10. **test_devtools_performance_profiling** - Performance profiling
    - Action: Profile WASM execution
    - Expected: Profile shows Ruchy function names
    - Verifies: Performance data mapped to source

**Key Types Defined**:
```ruchy
struct DevTools {
    loaded_modules: HashMap<String, LoadedModule>,
    watches: HashMap<u32, String>,
    profiler: Option<Profiler>,
}

struct LoadedModule {
    path: String,
    source_map: Option<SourceMap>,
    breakpoints: Vec<Breakpoint>,
}

struct ExecutionContext {
    paused: bool,
    pause_location: Option<Location>,
    exception: Option<Exception>,
}

struct CallStack {
    frames: Vec<StackFrame>,
}

struct Profile {
    function_stats: HashMap<String, FunctionStats>,
}
```

## Test Summary

**Total Tests Created**: 30 failing tests

| Category | Tests | File | Lines |
|----------|-------|------|-------|
| Source Maps | 10 | test_source_map_red.ruchy | 420 |
| Debug Symbols | 10 | test_debug_symbols_red.ruchy | 560 |
| DevTools Integration | 10 | test_devtools_integration_red.ruchy | 650 |
| **Total** | **30** | **3 files** | **~1,630** |

**Test Execution Pattern**:
All tests follow the RED phase pattern where:
- Each test defines expected behavior
- Helper functions/types panic with "not implemented"
- Test runner expects all tests to fail
- Tests demonstrate requirements through expected assertions

## Implementation Requirements Defined

### Component 1: Source Map Generator

**Requirements**:
- Generate Source Map v3 format JSON
- Support multiple source files
- Include sourcesContent for inline source viewing
- VLQ encoding for mappings
- Track line and column positions
- Support function and variable name mapping
- Maintain accuracy within ±5 bytes

**Target File**: `/bootstrap/stage3/source_map_generator.ruchy`

### Component 2: DWARF Generator

**Requirements**:
- Generate .debug_info section (DIEs)
- Generate .debug_line section (line table)
- Generate .debug_abbrev section (abbreviation table)
- Generate .debug_str section (string table with deduplication)
- Support DW_TAG_compile_unit, DW_TAG_subprogram, DW_TAG_variable
- Support DW_TAG_base_type, DW_TAG_structure_type
- Support DW_TAG_lexical_block for scopes
- Track inlining with DW_TAG_inlined_subroutine
- Embed in WASM custom sections

**Target File**: `/bootstrap/stage3/dwarf_generator.ruchy`

### Component 3: Debug Info Emitter

**Requirements**:
- Integrate with WASM compiler pipeline
- Track source positions during compilation
- Emit both source maps and DWARF simultaneously
- Handle optimizations gracefully
- Maintain debug info through compiler passes

**Target File**: `/bootstrap/stage3/debug_info_emitter.ruchy`

### Component 4: Browser Integration

**Requirements**:
- Write .wasm.map file alongside .wasm
- Embed DWARF in WASM custom sections
- Generate HTML test harness for DevTools
- Support Chrome DevTools debugging
- Support Firefox Developer Tools debugging
- Enable breakpoint resolution
- Enable variable inspection
- Enable call stack display
- Support async debugging
- Support hot reload

**Target File**: `/bootstrap/stage3/browser_debug_integration.ruchy`

## Performance Targets

| Metric | Target | Rationale |
|--------|--------|-----------|
| Source map generation | <100ms | Should not slow builds |
| Debug info size | <30% of WASM | Reasonable overhead |
| Mapping accuracy | ±5 bytes | Acceptable for debugging |
| DevTools load time | <1s | Fast debugging startup |
| Breakpoint hit time | <50ms | Responsive debugging |

## Success Criteria for RED Phase

✅ **30 Failing Tests Created**: Comprehensive test coverage across all categories

✅ **Source Map Tests**: 10 tests covering all mapping scenarios

✅ **Debug Symbol Tests**: 10 tests covering DWARF generation

✅ **DevTools Tests**: 10 tests covering browser integration

✅ **Requirements Clear**: Each test specifies implementation needs

✅ **Types Defined**: All necessary types and interfaces defined

✅ **Targets Established**: Performance and accuracy targets documented

## Technical Specifications

### Source Map Format (Source Map v3)

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

**Mappings Format**: VLQ-encoded segments
- Each segment: `[generated_column, source_index, source_line, source_column, name_index?]`
- Base64 VLQ encoding
- Semicolon separates lines
- Comma separates segments

### DWARF Sections

**Required Custom Sections**:
1. `.debug_info` - Debug Information Entries (DIEs)
2. `.debug_line` - Line number program
3. `.debug_abbrev` - Abbreviation declarations
4. `.debug_str` - String table (deduplicated)

**Optional Sections** (future):
- `.debug_loc` - Location lists
- `.debug_ranges` - Address ranges
- `.debug_frame` - Call frame information

### Browser Compatibility

**Chrome DevTools**:
- Source Map v3 support: ✅ Full
- DWARF support: ⚠️ Partial (in progress)
- Breakpoints: ✅ Full
- Variable inspection: ✅ Full
- Call stack: ✅ Full

**Firefox Developer Tools**:
- Source Map v3 support: ✅ Full
- DWARF support: ✅ Full
- Breakpoints: ✅ Full
- Variable inspection: ✅ Full
- Call stack: ✅ Full
- Better WASM debugging: ✅ Yes

## Known Challenges

### Challenge 1: DWARF Complexity

**Problem**: DWARF format is complex with many interdependencies between sections

**Mitigation**:
- Start with minimal DWARF subset (compile unit, functions, variables)
- Use existing DWARF libraries where appropriate
- Incremental implementation (add tags as needed)
- Extensive testing at each step

### Challenge 2: Browser Compatibility

**Problem**: Chrome and Firefox have different debugging capabilities

**Mitigation**:
- Test on both browsers during GREEN phase
- Implement common subset first
- Document browser-specific features
- Provide fallbacks where needed

### Challenge 3: Optimization vs Debug Info

**Problem**: Compiler optimizations can break source mappings

**Mitigation**:
- Maintain debug info through optimization passes
- Provide debug builds (unoptimized) option
- Track transformations carefully
- Use conservative mappings when uncertain

### Challenge 4: Source Map Accuracy

**Problem**: Mapping WASM instructions precisely to source lines

**Mitigation**:
- Track source positions throughout compilation
- Fine-grained position tracking in AST
- Test with known positions
- Accept ±5 byte tolerance as reasonable

## Files Created

**RED Phase Files** (3 test files + 1 plan):

1. `/docs/research/WASM_007_DEBUGGING_RED_PHASE.md` (447 lines)
   - Comprehensive RED phase plan
   - Technical specifications
   - Test strategy

2. `/validation/wasm/debugging/test_source_map_red.ruchy` (420 lines)
   - 10 source map tests
   - Source Map v3 format testing
   - Mapping accuracy verification

3. `/validation/wasm/debugging/test_debug_symbols_red.ruchy` (560 lines)
   - 10 debug symbol tests
   - DWARF format testing
   - Section validation

4. `/validation/wasm/debugging/test_devtools_integration_red.ruchy` (650 lines)
   - 10 DevTools integration tests
   - Browser debugging testing
   - Real-world debugging scenarios

**Total**: 4 files, ~2,077 lines of specification and test code

## Next Steps (GREEN Phase)

Once RED phase is validated:

1. **GREEN Phase**: Implement minimal debugging support
   - Basic source map generation (mappings only)
   - Simple DWARF info (functions and lines)
   - Chrome DevTools integration
   - Target: Make tests pass with minimal implementation

2. **REFACTOR Phase**: Optimize and enhance
   - Full DWARF support (all tags)
   - Firefox DevTools optimization
   - Performance optimization (source map generation)
   - Enhanced mapping accuracy

3. **TOOL Phase**: Comprehensive validation
   - Cross-browser testing (Chrome, Firefox, Safari)
   - Real-world debugging scenarios
   - Performance benchmarking
   - Property testing for mapping correctness

## Comparison with Similar Features

### WASM-006 (Incremental Compilation)
- **RED Phase**: 20 tests across 3 files
- **Complexity**: Build system integration
- **Performance Focus**: Yes (5-50x speedup target)

### WASM-007 (Browser Debugging)
- **RED Phase**: 30 tests across 3 files
- **Complexity**: Browser integration, dual format (source maps + DWARF)
- **Performance Focus**: Secondary (debug experience priority)

WASM-007 is more complex due to:
- Two debugging formats (Source Map v3 + DWARF)
- Browser compatibility requirements
- Real-time debugging scenarios
- Integration with external tools (DevTools)

## Validation Summary

✅ **RED Phase Complete**: All 30 failing tests created

✅ **Requirements Defined**: Clear implementation requirements

✅ **Types Specified**: Complete type definitions for all components

✅ **Performance Targets**: Established and documented

✅ **Browser Compatibility**: Strategy defined for Chrome and Firefox

✅ **Known Challenges**: Identified with mitigation strategies

✅ **Next Steps**: GREEN phase plan ready

**Status**: 🔴 RED Phase COMPLETE - Ready for GREEN Phase

---

**Phase**: RED
**Status**: ✅ COMPLETE
**Tests Created**: 30 failing tests (100% expected failures)
**Files**: 4 files, ~2,077 lines
**Coverage**: Source Maps (10) + Debug Symbols (10) + DevTools (10)
**Next**: GREEN Phase - Minimal implementation to make tests pass
