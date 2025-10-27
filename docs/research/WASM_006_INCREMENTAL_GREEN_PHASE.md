# WASM-006: Incremental Compilation - GREEN Phase Plan

## Overview

The GREEN phase for WASM-006 focuses on implementing the minimal functionality required to make all 20 RED phase tests pass. The goal is to achieve a working incremental compilation system that provides a 5x speedup for incremental builds.

## Objectives

1. **Module Caching System**: Store and retrieve compiled WebAssembly modules
2. **Dependency Tracking**: Build and maintain dependency graphs
3. **Incremental Rebuild Logic**: Compute minimal rebuild sets
4. **Compiler Integration**: Integrate caching with the WebAssembly compiler
5. **Performance Target**: Achieve 5x faster incremental builds

## Implementation Strategy

### Phase 1: Module Caching (Priority 1)

**Goal**: Implement basic cache that stores compiled modules and retrieves them

**Components**:
1. Cache storage backend (file-based)
2. Content hashing (SHA-256)
3. Cache lookup and storage
4. Cache invalidation

**Implementation Order**:
1. Simple file-based cache structure
2. Hash computation for source files
3. Cache read/write operations
4. Basic invalidation on source change

**Tests to Pass**: 6/10 module caching tests

### Phase 2: Dependency Tracking (Priority 2)

**Goal**: Build dependency graph to detect which modules need recompilation

**Components**:
1. Dependency graph data structure
2. Import/use declaration parsing
3. Transitive dependency computation
4. Change propagation logic

**Implementation Order**:
1. Graph data structure (adjacency list)
2. Parse imports from source files
3. Build complete dependency graph
4. Compute affected modules on change

**Tests to Pass**: 7/10 incremental rebuild tests

### Phase 3: Incremental Build Integration (Priority 3)

**Goal**: Integrate caching with compiler to skip unchanged modules

**Components**:
1. Compiler pipeline integration
2. Cache hit/miss logic
3. Rebuild set computation
4. Result aggregation

**Implementation Order**:
1. Hook into compiler entry point
2. Check cache before compilation
3. Compile only on cache miss
4. Aggregate compiled and cached modules

**Tests to Pass**: Remaining module caching and rebuild tests

### Phase 4: Parallel Compilation (Priority 4, Optional)

**Goal**: Compile independent modules in parallel

**Components**:
1. Identify independent modules
2. Thread pool for parallel compilation
3. Dependency-aware scheduling

**Implementation Order**:
1. Topological sort for build order
2. Identify parallelizable work
3. Thread pool executor
4. Parallel compilation orchestration

**Tests to Pass**: Parallel compilation tests

## Detailed Implementation Plan

### Component 1: Cache Storage

**File**: `/bootstrap/stage3/incremental_cache.ruchy`

**Key Types**:
```ruchy
pub struct ModuleCache {
    cache_dir: PathBuf,
    index: HashMap<String, CacheEntry>,
}

pub struct CacheEntry {
    source_hash: String,
    dependency_hashes: Vec<String>,
    compiled_path: PathBuf,
    metadata: ModuleMetadata,
    timestamp: u64,
}

pub struct ModuleMetadata {
    exports: Vec<String>,
    imports: Vec<String>,
    source_path: String,
}
```

**Key Functions**:
```ruchy
impl ModuleCache {
    pub fun new(cache_dir: PathBuf) -> Self;
    pub fun get(&self, source_path: &str) -> Option<CachedModule>;
    pub fun put(&mut self, source_path: &str, module: CompiledModule);
    pub fun invalidate(&mut self, source_path: &str);
    pub fun clear(&mut self);
}
```

**Implementation Notes**:
- Cache directory: `.ruchy-cache/`
- Cache entries stored as: `<hash>.wasm` and `<hash>.meta.json`
- Index stored in memory for fast lookups
- Persist index to disk for cross-session persistence

### Component 2: Content Hashing

**File**: `/bootstrap/stage3/content_hasher.ruchy`

**Key Functions**:
```ruchy
pub fun hash_source_file(path: &str) -> String;
pub fun hash_dependencies(deps: Vec<String>) -> String;
pub fun compute_cache_key(source_hash: String, dep_hashes: Vec<String>) -> String;
```

**Implementation**:
- Use SHA-256 for content hashing
- Normalize line endings before hashing
- Include dependency hashes in cache key
- Fast path: check timestamp first, hash only if changed

### Component 3: Dependency Graph

**File**: `/bootstrap/stage3/dependency_graph.ruchy`

**Key Types**:
```ruchy
pub struct DependencyGraph {
    nodes: HashMap<String, ModuleNode>,
    edges: Vec<(String, String)>, // (from, to)
}

pub struct ModuleNode {
    path: String,
    source_hash: String,
    dependencies: Vec<String>,
    dependents: Vec<String>,
}
```

**Key Functions**:
```ruchy
impl DependencyGraph {
    pub fun new() -> Self;
    pub fun add_module(&mut self, path: String, dependencies: Vec<String>);
    pub fun get_affected_modules(&self, changed: Vec<String>) -> Vec<String>;
    pub fun get_transitive_deps(&self, module: &str) -> Vec<String>;
    pub fun topological_sort(&self) -> Vec<String>;
}
```

**Implementation**:
- Adjacency list representation
- DFS for transitive dependencies
- Kahn's algorithm for topological sort
- Detect and handle circular dependencies

### Component 4: Incremental Builder

**File**: `/bootstrap/stage3/incremental_builder.ruchy`

**Key Types**:
```ruchy
pub struct IncrementalBuilder {
    cache: ModuleCache,
    graph: DependencyGraph,
    compiler: WasmCompiler,
}

pub struct BuildResult {
    success: bool,
    compiled_modules: Vec<String>,
    cached_modules: Vec<String>,
    failed_modules: Vec<String>,
    duration_ms: u64,
}
```

**Key Functions**:
```ruchy
impl IncrementalBuilder {
    pub fun new(cache_dir: PathBuf) -> Self;
    pub fun build(&mut self, project: &Project) -> BuildResult;
    fun detect_changes(&self, project: &Project) -> Vec<String>;
    fun compute_rebuild_set(&self, changed: Vec<String>) -> Vec<String>;
    fun compile_modules(&mut self, modules: Vec<String>) -> Result<(), Error>;
}
```

**Build Algorithm**:
```
1. Scan all source files, compute hashes
2. Compare hashes with cached versions
3. Identify changed files
4. Compute affected modules (using dependency graph)
5. For each module in rebuild set:
   a. Check if cached version is valid
   b. If valid: use cached version
   c. If invalid: compile and update cache
6. Aggregate results and return
```

### Component 5: Compiler Integration

**Modifications**: `/bootstrap/stage3/wasm_compiler.ruchy`

**Changes**:
- Add `--incremental` flag support
- Hook `IncrementalBuilder` into compiler entry point
- Bypass compilation for cache hits
- Maintain compilation statistics

**Integration Point**:
```ruchy
pub fun compile_project(project: &Project, incremental: bool) -> BuildResult {
    if incremental {
        let mut builder = IncrementalBuilder::new(".ruchy-cache");
        builder.build(project)
    } else {
        // Existing full rebuild logic
        compile_all_modules(project)
    }
}
```

## Implementation Timeline

### Week 1: Core Infrastructure

**Days 1-2: Module Caching**
- Implement cache storage (file-based)
- Add content hashing (SHA-256)
- Implement cache get/put operations
- **Target**: Pass 6/10 module caching tests

**Days 3-4: Dependency Tracking**
- Implement dependency graph structure
- Parse import declarations
- Build complete dependency graph
- Compute transitive dependencies
- **Target**: Pass 5/10 incremental rebuild tests

**Day 5: Incremental Build Logic**
- Detect changed files
- Compute minimal rebuild set
- Integrate cache with compiler
- **Target**: Pass 10/10 module caching tests

### Week 2: Integration and Optimization

**Days 6-7: Full Integration**
- Complete compiler integration
- Handle edge cases (circular deps, etc.)
- Add error handling and recovery
- **Target**: Pass 15/20 total tests

**Days 8-9: Parallel Compilation**
- Implement topological sort
- Add parallel task execution
- Thread pool for compilation
- **Target**: Pass all 20 tests

**Day 10: Polish and Testing**
- Fix remaining test failures
- Optimize performance
- Validate 5x speedup target
- **Target**: All tests passing, 5x speedup achieved

## Performance Optimization Strategy

### Phase 1: Correctness (Days 1-7)
Focus on making tests pass, not on performance

### Phase 2: Performance (Days 8-10)
Optimize for speed once correctness is established

**Optimization Targets**:
1. **Cache Lookup**: Use in-memory index, avoid disk I/O
2. **Hashing**: Check timestamps first, hash only if changed
3. **Graph Operations**: Cache transitive dependencies
4. **Parallel Compilation**: Use all CPU cores effectively

## Testing Strategy

### Unit Testing
- Test each component in isolation
- Mock dependencies where needed
- Focus on edge cases

### Integration Testing
- Run RED phase tests frequently
- Track progress (X/20 tests passing)
- Fix failures immediately

### Performance Testing
- Benchmark after each major milestone
- Compare against full rebuild baseline
- Ensure 5x target is achievable

## Success Criteria

The GREEN phase is complete when:

✅ **All 20 Tests Pass**: Every RED phase test passes

✅ **Performance Target Met**: ≥5x speedup for incremental builds

✅ **No-Change Builds**: <100ms for unchanged projects

✅ **Single-File Changes**: <500ms for single file changes

✅ **Cache Hit Rate**: >95% for typical workflows

✅ **Correctness**: Incremental builds produce identical output to full builds

## Risk Mitigation

### Risk 1: Cache Invalidation Bugs
**Impact**: Stale cache causes incorrect builds
**Mitigation**:
- Property test: `incremental_build(project) == full_build(project)`
- Add `--force-rebuild` flag for debugging
- Verbose logging of cache decisions

### Risk 2: Performance Target Not Met
**Impact**: <5x speedup achieved
**Mitigation**:
- Profile early and often
- Optimize hot paths
- Consider function-level caching if needed
- Accept 3-4x as minimum viable

### Risk 3: Dependency Tracking Errors
**Impact**: Missing dependencies = incorrect rebuilds
**Mitigation**:
- Conservative dependency tracking (over-invalidate vs under-invalidate)
- Validation mode that checks all dependencies
- Comprehensive test coverage for dependency patterns

## Next Steps After GREEN Phase

Once GREEN phase is complete:

1. **REFACTOR Phase**: Optimize and clean up implementation
2. **TOOL Phase**: Comprehensive validation and documentation
3. **Production Deployment**: Enable incremental compilation by default

## Conclusion

The GREEN phase will implement a working incremental compilation system through:
- File-based module caching
- Dependency graph tracking
- Minimal rebuild computation
- Compiler pipeline integration

By following this plan, we will achieve a 5x speedup for incremental builds while maintaining correctness and making all 20 RED phase tests pass.

---

**Phase**: GREEN
**Status**: PLANNED
**Target**: 5x faster incremental builds
**Timeline**: 2 weeks
**Tests to Pass**: 20/20
