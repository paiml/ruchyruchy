# WASM-006: Incremental Compilation - RED Phase Plan

## Overview

The RED phase for WASM-006 focuses on implementing incremental compilation for the WebAssembly compilation target. This feature will dramatically improve build times by caching compiled modules and only recompiling what has changed, targeting a 5x speedup for incremental builds.

## Motivation

**Current Problem**:
- Every build recompiles all modules from scratch
- Large codebases experience slow iteration cycles
- Developers wait unnecessarily for unchanged code to recompile
- Build times scale linearly with codebase size

**Solution**:
- Cache compiled WebAssembly modules
- Detect file changes and dependencies
- Recompile only affected modules
- Enable function-level granularity for maximum caching

**Expected Benefits**:
- 5x faster incremental builds (target)
- Near-instant rebuilds for single-file changes
- Better developer experience and productivity
- Scalable build times for large codebases

## Technical Requirements

### 1. Module Caching System

**Objectives**:
- Cache compiled WebAssembly modules to disk
- Invalidate cache when source files change
- Support multiple cache storage backends
- Handle cache corruption gracefully

**Key Features**:
- Content-based cache keys (hash of source + dependencies)
- Configurable cache location and size limits
- Cache statistics and monitoring
- Automatic cache cleanup

**Cache Storage Format**:
```ruchy
struct CachedModule {
    source_hash: String,           // SHA-256 of source file
    dependency_hashes: Vec<String>, // Hashes of dependencies
    compiled_wasm: Vec<u8>,        // Compiled WebAssembly binary
    metadata: ModuleMetadata,       // Type info, exports, imports
    timestamp: u64,                 // Cache creation time
}
```

### 2. Incremental Rebuild Detection

**Objectives**:
- Detect which files have changed since last build
- Build dependency graph for modules
- Determine minimal rebuild set
- Handle transitive dependencies correctly

**Change Detection Strategies**:
- File modification timestamps
- Content hashing (more reliable)
- Explicit dependency declarations
- Automatic dependency inference

**Dependency Graph**:
```ruchy
struct DependencyGraph {
    modules: HashMap<ModuleId, ModuleNode>,
    edges: Vec<(ModuleId, ModuleId)>, // (dependee, dependent)
}

struct ModuleNode {
    id: ModuleId,
    source_path: String,
    source_hash: String,
    dependencies: Vec<ModuleId>,
    dependents: Vec<ModuleId>,
}
```

### 3. Function-Level Compilation Units

**Objectives**:
- Enable caching at function granularity
- Recompile only changed functions within a module
- Merge cached and newly compiled functions
- Preserve module-level coherence

**Function Fingerprinting**:
```ruchy
struct FunctionFingerprint {
    name: String,
    signature: FunctionSignature,
    body_hash: String,              // Hash of function AST
    dependency_hashes: Vec<String>, // Hashes of called functions
}
```

**Incremental Module Linking**:
- Combine cached functions with new functions
- Resolve cross-function references
- Maintain consistent function indices
- Update module metadata

### 4. Performance Optimizations

**Objectives**:
- Minimize cache lookup overhead
- Parallelize independent compilations
- Optimize cache serialization/deserialization
- Reduce memory usage during builds

**Optimization Strategies**:
- Lazy loading of cached modules
- Parallel compilation of independent modules
- Streaming cache reads/writes
- In-memory cache for frequently accessed modules

## Testing Strategy

### 1. Cache Correctness Tests

**Test Scenarios**:
- Cache hit returns identical output to fresh compilation
- Cache invalidation on source file change
- Cache invalidation on dependency change
- Cache handles missing files gracefully
- Cache corruption detected and recovered

**Property Tests**:
```ruchy
// Property: Cached compilation equals fresh compilation
property cache_correctness(source: SourceFile) {
    let fresh = compile(source);
    let cached = compile_with_cache(source);
    assert(fresh == cached);
}

// Property: Cache invalidates on change
property cache_invalidation(source: SourceFile, change: Edit) {
    compile_with_cache(source);  // Populate cache
    let modified = apply_edit(source, change);
    let result = compile_with_cache(modified);
    assert(cache_was_invalidated());
    assert(result == compile(modified));
}
```

### 2. Incremental Build Tests

**Test Scenarios**:
- Single file change triggers minimal rebuild
- Dependency change triggers dependent rebuilds
- No change results in zero compilation
- Circular dependencies handled correctly
- Multiple changes batched efficiently

**Example Tests**:
```ruchy
test incremental_single_file() {
    // Initial build
    let project = create_test_project(10_modules);
    let build1 = build(project);
    assert(build1.compiled_modules == 10);

    // Change one file
    modify_file(project, "module5.ruchy");
    let build2 = build(project);
    assert(build2.compiled_modules == 1);  // Only module5
    assert(build2.cache_hits == 9);
}

test incremental_with_dependencies() {
    // Module A depends on Module B
    let project = create_project_with_deps([
        ("moduleA.ruchy", ["moduleB.ruchy"]),
        ("moduleB.ruchy", []),
    ]);

    build(project);  // Initial build

    // Change moduleB - should rebuild both
    modify_file(project, "moduleB.ruchy");
    let build = build(project);
    assert(build.compiled_modules == 2);  // A and B
}
```

### 3. Performance Tests

**Benchmarks**:
- Full build vs incremental build (no changes)
- Single file change rebuild time
- Multi-file change rebuild time
- Cache overhead measurement
- Scaling with project size

**Target Metrics**:
- Incremental build (no changes): <100ms
- Single file change: <500ms (vs ~2.5s full build for 10 modules)
- Cache hit overhead: <10ms per module
- Memory overhead: <50MB for cache structures

### 4. Function-Level Granularity Tests

**Test Scenarios**:
- Function change recompiles only that function
- Function deletion handled correctly
- Function addition handled correctly
- Cross-function dependencies tracked
- Module-level coherence maintained

## Failing Test Implementation Plan

### Test File 1: Module Caching (`test_module_caching_red.ruchy`)

**Tests**:
1. `test_cache_basic_functionality` - Store and retrieve compiled modules
2. `test_cache_invalidation_on_source_change` - Cache updates when source changes
3. `test_cache_invalidation_on_dependency_change` - Transitive invalidation
4. `test_cache_corruption_recovery` - Handle corrupt cache gracefully
5. `test_cache_size_limits` - Enforce cache size constraints
6. `test_cache_statistics` - Track cache hits/misses

### Test File 2: Incremental Rebuild (`test_incremental_rebuild_red.ruchy`)

**Tests**:
1. `test_no_change_no_rebuild` - Zero compilation when nothing changed
2. `test_single_file_minimal_rebuild` - Only changed file recompiled
3. `test_dependency_triggered_rebuild` - Dependencies trigger rebuilds
4. `test_transitive_dependencies` - Multi-level dependency chains
5. `test_circular_dependencies` - Cycles handled correctly
6. `test_parallel_compilation` - Independent modules compiled in parallel

### Test File 3: Function-Level Compilation (`test_function_level_red.ruchy`)

**Tests**:
1. `test_function_change_isolation` - Only changed function recompiled
2. `test_function_deletion` - Removed functions handled
3. `test_function_addition` - New functions integrated
4. `test_function_dependency_tracking` - Cross-function calls tracked
5. `test_module_coherence` - Module integrity maintained
6. `test_function_fingerprinting` - Accurate change detection

### Test File 4: Performance Benchmarks (`test_incremental_performance_red.ruchy`)

**Tests**:
1. `bench_full_build_baseline` - Measure full build time
2. `bench_incremental_no_change` - Measure cache-only build
3. `bench_incremental_single_file` - Measure single file rebuild
4. `bench_cache_overhead` - Measure caching overhead
5. `bench_scaling_with_size` - Test with various project sizes
6. `bench_parallel_compilation` - Measure parallelization speedup

## Expected Test Results (RED Phase)

All tests should **FAIL** with clear error messages indicating missing functionality:

```
❌ test_cache_basic_functionality
   Error: Module caching not implemented
   Expected: Cached module retrieved
   Actual: Cache system does not exist

❌ test_no_change_no_rebuild
   Error: Incremental compilation not implemented
   Expected: 0 modules compiled
   Actual: All 10 modules recompiled

❌ test_function_change_isolation
   Error: Function-level caching not implemented
   Expected: 1 function recompiled
   Actual: Entire module recompiled

❌ bench_incremental_no_change
   Error: Build time >2000ms (expected <100ms)
   Reason: Full rebuild performed instead of using cache
```

## Implementation Complexity Analysis

### Module Caching (Medium Complexity)
- **Core Logic**: Straightforward - hash files, store/retrieve binaries
- **Challenges**: Cache invalidation, corruption handling
- **Estimated Effort**: 2-3 days

### Incremental Rebuild (High Complexity)
- **Core Logic**: Dependency graph construction and traversal
- **Challenges**: Transitive dependencies, circular dependencies, correctness
- **Estimated Effort**: 3-4 days

### Function-Level Granularity (Very High Complexity)
- **Core Logic**: Function fingerprinting, partial module compilation
- **Challenges**: Cross-function dependencies, module linking, correctness
- **Estimated Effort**: 4-5 days

## Success Criteria for RED Phase

The RED phase is complete when:

✅ **All test files created**:
- `test_module_caching_red.ruchy`
- `test_incremental_rebuild_red.ruchy`
- `test_function_level_red.ruchy`
- `test_incremental_performance_red.ruchy`

✅ **All tests failing with clear messages**:
- Each test specifies what needs to be implemented
- Error messages guide GREEN phase development
- No false positives or unexpected failures

✅ **Documentation complete**:
- RED phase plan documented
- Test scenarios explained
- Implementation requirements clear

✅ **Integration updated**:
- INTEGRATION.md reflects WASM-006 RED phase started
- Roadmap updated with progress

## Next Steps (GREEN Phase)

After RED phase completion, the GREEN phase will implement:

1. **Basic Module Caching**:
   - Simple file-based cache
   - Content hashing
   - Cache lookup and storage

2. **Dependency Tracking**:
   - Build dependency graph
   - Detect changes
   - Compute minimal rebuild set

3. **Incremental Compilation**:
   - Integrate cache with compiler
   - Skip compilation for cached modules
   - Verify correctness

4. **Function-Level Granularity** (if time permits):
   - Function fingerprinting
   - Partial module compilation
   - Function-level cache

## Estimated Timeline

- **RED Phase**: 1-2 days (test writing and documentation)
- **GREEN Phase**: 5-7 days (implementation)
- **REFACTOR Phase**: 2-3 days (optimization)
- **TOOL Phase**: 2-3 days (validation)
- **Total**: ~2-3 weeks

## Conclusion

The RED phase for WASM-006 (Incremental Compilation) establishes comprehensive requirements and failing tests for implementing build time optimizations. By following Extreme TDD methodology, we ensure that the implementation will be correct, well-tested, and meet all performance targets.

The 5x speedup target is ambitious but achievable through careful implementation of caching, dependency tracking, and incremental rebuild logic. This feature will significantly improve developer productivity and make Ruchy more competitive with modern build systems.

---

**Phase**: RED
**Status**: PLANNED
**Next**: Implement failing tests
**Target**: 5x faster incremental builds
