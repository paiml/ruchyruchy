# WASM-007: Browser Debugging Integration - GREEN Phase Plan

## Overview

The GREEN phase for WASM-007 focuses on implementing the **minimal** functionality needed to make the 30 RED phase tests pass. This phase prioritizes making tests pass over code quality, performance, or completeness.

## Objectives

1. **Make all 30 RED phase tests pass** with minimal implementation
2. **Implement basic source map generation** (Source Map v3 format)
3. **Implement basic DWARF generation** (minimal debug info)
4. **Enable basic DevTools integration** (breakpoints and stepping)
5. **No optimization** - simple, straightforward implementations only

## Implementation Strategy

### Philosophy: Minimal Working Implementation

Following the GREEN phase principle:
- Write the **simplest code** that makes tests pass
- **No premature optimization** - performance comes in REFACTOR
- **Hardcoded solutions acceptable** where appropriate
- **Incomplete implementations OK** if tests pass
- **Code duplication acceptable** - will refactor later

### Component Breakdown

We'll implement 4 core components in order of dependency:

1. **Source Map Generator** (foundation for all debugging)
2. **DWARF Generator** (debug symbols for browsers)
3. **Debug Info Emitter** (integration with compiler)
4. **Browser Integration** (DevTools support)

## Implementation Plan

### Component 1: Source Map Generator (Minimal)

**File**: `/bootstrap/stage3/source_map_generator_green.ruchy`

**Goal**: Generate valid Source Map v3 JSON that makes 10 tests pass

**Minimal Requirements**:
- Struct to hold source map data
- Add sources, names, mappings
- Generate JSON output (can be simple string concatenation)
- VLQ encoding for mappings (minimal implementation)

**What to Skip** (will add in REFACTOR):
- Efficient VLQ encoding (use simple, slow version)
- Memory optimization
- Advanced features (sourceRoot, file)
- Error handling (panic on errors)

**Estimated LOC**: ~300-400 lines

**Key Functions**:
```ruchy
pub struct SourceMapGenerator {
    version: u32,
    sources: Vec<String>,
    names: Vec<String>,
    mappings: Vec<Mapping>,
    sources_content: Vec<String>,
}

impl SourceMapGenerator {
    pub fun new() -> Self
    pub fun add_source(&mut self, path: String, content: String) -> usize
    pub fun add_name(&mut self, name: String) -> usize
    pub fun add_mapping(&mut self, mapping: Mapping)
    pub fun generate(&self) -> String  // Simple JSON string concatenation
}

pub struct Mapping {
    pub generated_line: u32,
    pub generated_column: u32,
    pub source_index: u32,
    pub source_line: u32,
    pub source_column: u32,
    pub name_index: Option<u32>,
}

// Minimal VLQ encoding (simple, slow, but correct)
fun encode_vlq(value: i32) -> String
fun encode_mapping(mapping: &Mapping) -> String
```

**Implementation Notes**:
- Use simple string concatenation for JSON (no JSON library needed)
- VLQ encoding: implement minimal version that works
- No validation - assume inputs are valid
- Hardcode version to 3

### Component 2: DWARF Generator (Minimal)

**File**: `/bootstrap/stage3/dwarf_generator_green.ruchy`

**Goal**: Generate minimal DWARF debug info that makes 10 tests pass

**Minimal Requirements**:
- Generate .debug_info section (DIEs for functions, variables, types)
- Generate .debug_line section (line table)
- Generate .debug_abbrev section (abbreviation table)
- Generate .debug_str section (string table)
- Embed in WASM custom sections

**What to Skip** (will add in REFACTOR):
- Advanced DWARF tags (only implement what tests require)
- Optimization of binary format
- Complete DWARF compliance (minimal subset only)
- Complex type encoding

**Estimated LOC**: ~500-600 lines

**Key Types**:
```ruchy
pub struct DwarfGenerator {
    info_section: Vec<u8>,
    line_section: Vec<u8>,
    abbrev_section: Vec<u8>,
    str_section: Vec<u8>,
    strings: HashMap<String, u32>,  // String deduplication
}

impl DwarfGenerator {
    pub fun new() -> Self

    // Add debug entries
    pub fun add_compile_unit(&mut self, name: String)
    pub fun add_function(&mut self, name: String, line: u32)
    pub fun add_variable(&mut self, name: String, type_id: u32)
    pub fun add_type(&mut self, name: String, size: u32)
    pub fun add_line_entry(&mut self, line: u32, wasm_offset: u32)

    // Generate binary DWARF
    pub fun generate(&self) -> DwarfSections
}

pub struct DwarfSections {
    pub debug_info: Vec<u8>,
    pub debug_line: Vec<u8>,
    pub debug_abbrev: Vec<u8>,
    pub debug_str: Vec<u8>,
}

// Helper functions
fun write_uleb128(value: u32) -> Vec<u8>
fun write_string(s: &str) -> Vec<u8>
fun encode_die(tag: DwarfTag, attributes: Vec<DwarfAttr>) -> Vec<u8>
```

**Implementation Notes**:
- Implement only required DWARF tags (CompileUnit, Subprogram, Variable, BaseType)
- Use simple binary encoding (no optimization)
- Hardcode DWARF version to 4
- Minimal abbreviation table (one entry per tag type)
- Simple line table (no optimizations)

### Component 3: Debug Info Emitter (Integration)

**File**: `/bootstrap/stage3/debug_info_emitter_green.ruchy`

**Goal**: Integrate source maps and DWARF with WASM compiler

**Minimal Requirements**:
- Hook into WASM compilation pipeline
- Track source positions during compilation
- Generate both source map and DWARF
- Coordinate between source map and DWARF generators

**What to Skip** (will add in REFACTOR):
- Optimization tracking (assume unoptimized code)
- Advanced position tracking
- Error recovery

**Estimated LOC**: ~400-500 lines

**Key Types**:
```ruchy
pub struct DebugInfoEmitter {
    source_map: SourceMapGenerator,
    dwarf: DwarfGenerator,
    current_file: String,
    current_line: u32,
    current_column: u32,
    wasm_offset: u32,
}

impl DebugInfoEmitter {
    pub fun new() -> Self

    // Track source positions
    pub fun set_source_file(&mut self, path: String, content: String)
    pub fun set_position(&mut self, line: u32, column: u32)
    pub fun advance_wasm_offset(&mut self, bytes: u32)

    // Emit debug info
    pub fun emit_function(&mut self, name: String)
    pub fun emit_variable(&mut self, name: String, type_name: String)
    pub fun emit_expression(&mut self)

    // Finalize
    pub fun finalize(&self) -> DebugOutput
}

pub struct DebugOutput {
    pub source_map: String,  // JSON
    pub dwarf: DwarfSections,
}
```

**Implementation Notes**:
- Simple position tracking (track current file/line/column)
- Emit debug info at key points (function start, variable declaration, expression)
- No optimization handling (assume debug builds)
- Coordinate source map and DWARF generation

### Component 4: Browser Integration (DevTools Support)

**File**: `/bootstrap/stage3/browser_debug_integration_green.ruchy`

**Goal**: Enable basic debugging in Chrome DevTools

**Minimal Requirements**:
- Embed DWARF in WASM custom sections
- Write .wasm.map file alongside .wasm
- Generate HTML test harness for DevTools testing

**What to Skip** (will add in REFACTOR):
- Firefox-specific optimizations
- Advanced DevTools features
- Performance optimization

**Estimated LOC**: ~300-400 lines

**Key Types**:
```ruchy
pub struct BrowserDebugger {
    wasm_binary: Vec<u8>,
    debug_output: DebugOutput,
}

impl BrowserDebugger {
    pub fun new(wasm: Vec<u8>, debug: DebugOutput) -> Self

    // Embed debug info in WASM
    pub fun embed_dwarf(&mut self) -> Vec<u8>

    // Write files
    pub fun write_wasm_file(&self, path: String) -> Result<(), String>
    pub fun write_source_map(&self, path: String) -> Result<(), String>

    // Generate test harness
    pub fun generate_html_harness(&self) -> String
}

// WASM custom section embedding
fun add_custom_section(wasm: &mut Vec<u8>, name: String, data: Vec<u8>)
fun parse_wasm_sections(wasm: &[u8]) -> Vec<WasmSection>
```

**Implementation Notes**:
- Simple WASM binary manipulation (append custom sections)
- Write .wasm.map as plain JSON file
- Generate minimal HTML harness for testing
- Focus on Chrome DevTools compatibility

## Test Execution Plan

### Making Tests Pass

We'll make tests pass in 3 waves:

**Wave 1: Source Map Tests (10 tests)**
1. Implement SourceMapGenerator
2. Run test_source_map_red.ruchy
3. Fix failures one by one
4. Target: All 10 tests passing

**Wave 2: Debug Symbol Tests (10 tests)**
1. Implement DwarfGenerator
2. Run test_debug_symbols_red.ruchy
3. Fix failures one by one
4. Target: All 10 tests passing

**Wave 3: DevTools Integration Tests (10 tests)**
1. Implement DebugInfoEmitter + BrowserDebugger
2. Run test_devtools_integration_red.ruchy
3. Fix failures one by one (may need browser automation)
4. Target: All 10 tests passing

### Test Modifications

Since RED phase tests use panic!() for unimplemented functions, we'll need to:
1. Replace helper function panics with real implementations
2. Keep test assertions exactly as-is
3. Add minimal implementations to make assertions pass

## Implementation Order

1. **Source Map Generator** (foundation)
   - Implement SourceMapGenerator struct
   - Implement VLQ encoding
   - Implement JSON generation
   - Test: Run test_source_map_red.ruchy

2. **DWARF Generator** (debug symbols)
   - Implement DwarfGenerator struct
   - Implement binary encoding (ULEB128, DIEs)
   - Implement section generation
   - Test: Run test_debug_symbols_red.ruchy

3. **Debug Info Emitter** (integration)
   - Implement DebugInfoEmitter struct
   - Coordinate source map + DWARF
   - Test: Integration tests

4. **Browser Integration** (final piece)
   - Implement WASM custom section embedding
   - Implement file writing
   - Test: Run test_devtools_integration_red.ruchy

## Success Criteria for GREEN Phase

✅ **All 30 Tests Passing**: Every RED phase test must pass

✅ **Minimal Implementation**: No unnecessary features

✅ **Source Map Generation**: Valid Source Map v3 JSON

✅ **DWARF Generation**: Valid DWARF sections

✅ **DevTools Integration**: Basic breakpoint support in Chrome

✅ **Code Quality**: Not a concern in GREEN (will refactor later)

## Non-Goals for GREEN Phase

❌ **Performance Optimization**: Don't optimize (REFACTOR phase)

❌ **Code Quality**: Duplication is OK (REFACTOR phase)

❌ **Complete DWARF**: Only implement what tests require

❌ **Advanced Features**: Stick to minimal requirements

❌ **Error Handling**: Panic on errors is acceptable

## Estimated Effort

**Total Lines of Code**: ~1,500-1,900 lines

| Component | Est. LOC | Complexity |
|-----------|----------|------------|
| Source Map Generator | 300-400 | Medium |
| DWARF Generator | 500-600 | High |
| Debug Info Emitter | 400-500 | Medium |
| Browser Integration | 300-400 | Low |
| **Total** | **1,500-1,900** | **Medium-High** |

**Timeline**: 1-2 days of focused implementation

## Known Challenges

### Challenge 1: VLQ Encoding

**Problem**: Source Map v3 uses VLQ (Variable Length Quantity) encoding

**Solution**: Implement minimal VLQ encoder
- Use simple algorithm (not optimized)
- Handle positive and negative values
- Base64 encoding for output

### Challenge 2: DWARF Binary Format

**Problem**: DWARF is a complex binary format

**Solution**: Implement minimal subset
- Only required tags (CompileUnit, Subprogram, Variable, BaseType)
- Simple attribute encoding
- No optimization

### Challenge 3: WASM Custom Sections

**Problem**: Need to embed DWARF in WASM binary

**Solution**: Simple binary manipulation
- Parse existing WASM sections
- Append custom sections at end
- Update section count

### Challenge 4: DevTools Testing

**Problem**: Need to verify debugging works in browser

**Solution**: Manual testing approach
- Generate HTML harness
- Load in Chrome DevTools
- Verify breakpoints manually
- Automated testing in TOOL phase

## Next Steps (REFACTOR Phase)

After GREEN phase completion:

1. **Performance Optimization**
   - Optimize VLQ encoding
   - Optimize DWARF generation
   - Reduce memory usage

2. **Code Quality**
   - Remove duplication
   - Improve abstractions
   - Better error handling

3. **Feature Completeness**
   - Complete DWARF implementation
   - Firefox compatibility
   - Advanced DevTools features

## Conclusion

The GREEN phase will implement minimal browser debugging support by:
- Creating a basic source map generator
- Creating a minimal DWARF generator
- Integrating with the WASM compiler
- Enabling basic DevTools debugging

All 30 RED phase tests will pass with simple, straightforward implementations. Performance and code quality will be addressed in the REFACTOR phase.

---

**Phase**: GREEN
**Status**: PLANNED
**Target**: Make all 30 tests pass with minimal implementation
**Timeline**: 1-2 days
**Files to Create**: 4 implementation files (~1,500-1,900 lines)
