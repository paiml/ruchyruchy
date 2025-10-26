# WASM-007: Browser Debugging Integration - REFACTOR Phase Plan

## Overview

The REFACTOR phase for WASM-007 focuses on optimizing the GREEN phase implementation for production use. The goal is to improve performance, code quality, and feature completeness while maintaining all 30 passing tests.

## Objectives

1. **Optimize Performance** - Target: <100ms total debug info generation
2. **Improve Code Quality** - Remove duplication, improve abstractions
3. **Enhance Features** - Complete DWARF implementation, Firefox support
4. **Reduce Memory** - Minimize allocations and copies
5. **Maintain Tests** - All 30 tests must continue passing

## Current Baseline (GREEN Phase)

### Performance Metrics
- Source Map Generation: ~10-50ms (unoptimized)
- DWARF Generation: ~20-100ms (unoptimized)
- Total Time: ~50-200ms per file
- Memory: ~5-15MB per file

### Code Quality Issues
- Bubble sort for mappings (O(n²))
- Excessive string concatenation
- Linear string searches in DWARF
- Code duplication in test helpers
- No input validation
- Panic on all errors

### Missing Features
- Complete DWARF tag support (only 5/50+ tags)
- Firefox-specific optimizations
- Advanced source map features (sourceRoot, file)
- Robust JSON parsing
- VLQ decoding
- Location lists, range lists

## Optimization Strategy

### Phase 1: Algorithm Improvements

**Target**: 50% performance improvement from algorithm changes alone

1. **Replace Bubble Sort with Quicksort**
   - Current: O(n²) bubble sort for mappings
   - Target: O(n log n) quicksort
   - Expected: 10-100x speedup for large files

2. **Optimize String Building**
   - Current: Repeated string concatenation (O(n²))
   - Target: StringBuilder/Vec<u8> with capacity pre-allocation
   - Expected: 2-5x speedup

3. **Hash-Based String Deduplication**
   - Current: Linear search in DWARF string table
   - Target: HashMap for O(1) lookups
   - Expected: 10-100x speedup for string-heavy code

### Phase 2: Memory Optimization

**Target**: Reduce memory usage by 50%

1. **Remove Intermediate Copies**
   - Avoid cloning large data structures
   - Use references where possible
   - Stream writing instead of buffering

2. **Pre-allocate Buffers**
   - Calculate sizes before allocation
   - Reduce reallocation overhead

3. **String Interning**
   - Share common strings (type names, keywords)
   - Reduce duplicate storage

### Phase 3: Code Quality

**Target**: <1% code duplication, <15 cyclomatic complexity

1. **Extract Common Abstractions**
   - Binary writer trait/struct
   - JSON builder
   - Source parser

2. **Remove Duplication**
   - Consolidate test helper functions
   - Share encoding logic

3. **Improve Error Handling**
   - Return Result instead of panic
   - Meaningful error messages
   - Error recovery

### Phase 4: Feature Completeness

**Target**: Complete DWARF implementation, Firefox support

1. **Additional DWARF Tags**
   - DW_TAG_pointer_type
   - DW_TAG_array_type
   - DW_TAG_enumeration_type
   - DW_TAG_member (for struct fields)

2. **Advanced Features**
   - Location lists (DW_AT_location)
   - Range lists (DW_AT_ranges)
   - Source map sourceRoot field
   - VLQ decoding

3. **Browser Compatibility**
   - Test with Firefox DevTools
   - Handle browser-specific quirks
   - Fallbacks for unsupported features

## Implementation Plan

### Component 1: Optimized Source Map Generator

**File**: `/bootstrap/stage3/source_map_generator_refactored.ruchy`

**Optimizations**:

1. **Quicksort for Mappings** (~20 lines)
```ruchy
fun quicksort_mappings(mappings: &mut [Mapping], low: usize, high: usize) {
    if low < high {
        let pivot = partition(mappings, low, high);
        quicksort_mappings(mappings, low, pivot);
        quicksort_mappings(mappings, pivot + 1, high);
    }
}

fun partition(mappings: &mut [Mapping], low: usize, high: usize) -> usize {
    // Partition logic
}
```

2. **StringBuilder for JSON** (~50 lines)
```ruchy
struct JsonBuilder {
    buffer: Vec<u8>,
}

impl JsonBuilder {
    pub fun new(capacity: usize) -> Self {
        JsonBuilder { buffer: Vec::with_capacity(capacity) }
    }

    pub fun add_field(&mut self, key: &str, value: &str) {
        self.buffer.extend(b"\"");
        self.buffer.extend(key.bytes());
        self.buffer.extend(b"\":\"");
        self.buffer.extend(value.bytes());
        self.buffer.extend(b"\",");
    }

    pub fun build(&self) -> String {
        String::from_utf8(self.buffer.clone()).unwrap()
    }
}
```

3. **VLQ Decoding** (~40 lines)
```ruchy
fun decode_vlq(s: &str) -> Result<Vec<i32>, String> {
    let mut result = Vec::new();
    let mut value = 0;
    let mut shift = 0;

    for ch in s.chars() {
        let digit = base64_decode(ch)?;
        value |= (digit & 0b11111) << shift;

        if (digit & 0b100000) == 0 {
            // No continuation
            let signed_value = if (value & 1) != 0 {
                -(value >> 1)
            } else {
                value >> 1
            };
            result.push(signed_value);
            value = 0;
            shift = 0;
        } else {
            shift += 5;
        }
    }

    Ok(result)
}
```

**Estimated LOC**: ~700-800 lines (vs 655 in GREEN)
**Expected Performance**: 2-3x faster

### Component 2: Optimized DWARF Generator

**File**: `/bootstrap/stage3/dwarf_generator_refactored.ruchy`

**Optimizations**:

1. **Binary Writer Abstraction** (~80 lines)
```ruchy
struct BinaryWriter {
    buffer: Vec<u8>,
}

impl BinaryWriter {
    pub fun new() -> Self {
        BinaryWriter { buffer: Vec::with_capacity(4096) }
    }

    pub fun write_u8(&mut self, value: u8) {
        self.buffer.push(value);
    }

    pub fun write_u32_le(&mut self, value: u32) {
        self.buffer.extend(&[
            (value & 0xff) as u8,
            ((value >> 8) & 0xff) as u8,
            ((value >> 16) & 0xff) as u8,
            ((value >> 24) & 0xff) as u8,
        ]);
    }

    pub fun write_uleb128(&mut self, value: u32) {
        let bytes = encode_uleb128(value);
        self.buffer.extend(&bytes);
    }

    pub fun write_string(&mut self, s: &str) {
        self.buffer.extend(s.bytes());
        self.buffer.push(0); // Null terminator
    }

    pub fun as_bytes(&self) -> &[u8] {
        &self.buffer
    }
}
```

2. **Additional DWARF Tags** (~150 lines)
```ruchy
// Abbrev 6: DW_TAG_pointer_type
self.abbrev_section.extend(encode_uleb128(6));
self.abbrev_section.extend(encode_uleb128(DW_TAG_POINTER_TYPE));
self.abbrev_section.push(DW_CHILDREN_NO);
self.abbrev_section.extend(encode_uleb128(DW_AT_TYPE));
self.abbrev_section.extend(encode_uleb128(DW_FORM_REF4));
self.abbrev_section.extend(encode_uleb128(0));
self.abbrev_section.extend(encode_uleb128(0));

// Abbrev 7: DW_TAG_array_type
// ... similar for other tags
```

3. **Optimized String Table** (~30 lines)
```ruchy
// Already using HashMap in GREEN - keep but optimize usage
impl DwarfGenerator {
    fun intern_string(&mut self, s: String) -> u32 {
        if let Some(&offset) = self.strings.get(&s) {
            return offset;
        }
        let offset = self.next_string_offset;
        self.str_section.extend(s.bytes());
        self.str_section.push(0);
        self.strings.insert(s, offset);
        self.next_string_offset += (s.len() + 1) as u32;
        offset
    }
}
```

**Estimated LOC**: ~950-1050 lines (vs 850 in GREEN)
**Expected Performance**: 1.5-2x faster

### Component 3: Enhanced Browser Integration

**File**: `/bootstrap/stage3/browser_debug_integration_refactored.ruchy`

**Improvements**:

1. **Error Handling** (~50 lines)
```ruchy
pub enum DebugError {
    FileNotFound(String),
    InvalidSourceMap(String),
    InvalidWasm(String),
    DevToolsError(String),
}

impl DevTools {
    pub fun load_wasm(&mut self, path: &str) -> Result<LoadedModule, DebugError> {
        let wasm_binary = fs::read(path)
            .map_err(|e| DebugError::FileNotFound(format!("WASM: {}", e)))?;

        let source_map_path = format!("{}.map", path);
        let source_map = load_source_map_file(&source_map_path)
            .ok();

        Ok(LoadedModule { /* ... */ })
    }
}
```

2. **Firefox DevTools Support** (~100 lines)
```ruchy
pub enum BrowserType {
    Chrome,
    Firefox,
    Safari,
}

impl DevTools {
    pub fun new_for_browser(browser: BrowserType) -> Self {
        let config = match browser {
            BrowserType::Chrome => ChromeConfig::default(),
            BrowserType::Firefox => FirefoxConfig::default(),
            BrowserType::Safari => SafariConfig::default(),
        };

        DevTools {
            browser,
            config,
            // ... rest
        }
    }
}
```

3. **Real Browser Automation** (optional - may defer to TOOL phase)

**Estimated LOC**: ~550-650 lines (vs 470 in GREEN)
**Expected Performance**: Similar (I/O bound)

## Performance Targets

| Metric | GREEN Baseline | REFACTOR Target | Improvement |
|--------|----------------|-----------------|-------------|
| Source Map Generation | 10-50ms | <30ms | 2-3x faster |
| DWARF Generation | 20-100ms | <50ms | 1.5-2x faster |
| Total Time | 50-200ms | <100ms | 2-4x faster |
| Memory Usage | 5-15MB | <5MB | 2-3x reduction |
| Code Duplication | ~10% | <1% | 10x improvement |
| Cyclomatic Complexity | ~20 | <15 | 1.3x improvement |

## Code Quality Targets

| Metric | GREEN | REFACTOR | Improvement |
|--------|-------|----------|-------------|
| Duplication | ~200 lines | <20 lines | 10x reduction |
| Max Complexity | ~25 | <15 | Simpler functions |
| Error Handling | 0% (panics) | 100% (Result) | Full coverage |
| Input Validation | 0% | 100% | Full validation |
| Documentation | Minimal | Complete | Full docs |

## Testing Strategy

### Regression Testing
- Run all 30 RED phase tests after each optimization
- Ensure all tests continue passing
- No behavioral changes allowed

### Performance Testing
- Benchmark before/after each optimization
- Measure time and memory for each component
- Track improvement metrics

### Quality Validation
- `ruchy lint` - A+ grade required
- `ruchy score` - >0.8 required
- `ruchy complexity` - All functions <15

## Implementation Order

1. **Source Map Optimizations** (highest impact)
   - Replace bubble sort with quicksort
   - Optimize JSON generation
   - Add VLQ decoding

2. **DWARF Optimizations** (medium impact)
   - Binary writer abstraction
   - Additional tags
   - Memory optimization

3. **Code Quality** (medium impact)
   - Extract abstractions
   - Remove duplication
   - Error handling

4. **Browser Integration** (lower impact - already functional)
   - Error handling
   - Firefox support
   - Enhanced features

## Success Criteria for REFACTOR Phase

✅ **All 30 Tests Passing**: No regressions

✅ **Performance Target**: <100ms total generation time

✅ **Memory Target**: <5MB memory usage

✅ **Code Quality**: <1% duplication, <15 complexity

✅ **Error Handling**: 100% Result-based (no panics)

✅ **Feature Complete**: All core DWARF tags, Firefox support

✅ **Documentation**: Complete inline docs and examples

## Non-Goals for REFACTOR Phase

❌ **New Features**: No new functionality (only optimizations)

❌ **Breaking Changes**: All tests must pass without modification

❌ **Perfect DWARF**: Don't implement all 50+ tags (focus on core 10-15)

❌ **Browser Automation**: Defer to TOOL phase

## Estimated Effort

**Total Lines of Code**: ~2,200-2,500 lines (vs 1,975 in GREEN)

| Component | GREEN LOC | REFACTOR LOC | Increase |
|-----------|-----------|--------------|----------|
| Source Map Generator | 655 | 700-800 | +10-20% |
| DWARF Generator | 850 | 950-1050 | +12-24% |
| Browser Integration | 470 | 550-650 | +17-38% |
| **Total** | **1,975** | **2,200-2,500** | **+11-27%** |

**Timeline**: 1-2 days of focused optimization

## Risk Mitigation

### Risk 1: Breaking Tests
**Mitigation**: Run tests after each optimization, commit frequently

### Risk 2: Over-Optimization
**Mitigation**: Measure before/after, stop when targets met

### Risk 3: Complexity Increase
**Mitigation**: Monitor complexity metrics, refactor if needed

### Risk 4: Time Overrun
**Mitigation**: Prioritize high-impact optimizations first

## Next Steps (TOOL Phase)

After REFACTOR phase completion:

1. **Property Testing** - Verify correctness properties
2. **Fuzz Testing** - Test with random/invalid inputs
3. **Cross-Browser Testing** - Chrome, Firefox, Safari
4. **Performance Benchmarking** - Real-world scenarios
5. **Production Validation** - Ready for deployment

## Conclusion

The REFACTOR phase will optimize WASM-007 for production use by:
- Improving performance by 2-4x
- Reducing memory usage by 2-3x
- Achieving <1% code duplication
- Adding complete error handling
- Supporting Firefox DevTools

All optimizations will maintain the 30 passing tests from GREEN phase, ensuring no regressions while significantly improving quality and performance.

---

**Phase**: REFACTOR
**Status**: PLANNED
**Target**: 2-4x performance improvement, <1% duplication, 100% error handling
**Timeline**: 1-2 days
**Files to Modify**: 3 implementation files (~2,200-2,500 lines)
