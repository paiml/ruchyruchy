# WASM-006: Incremental Compilation - REFACTOR Phase Plan

## Overview

The REFACTOR phase for WASM-006 focuses on optimizing the GREEN phase implementation for production use. The goal is to improve performance, reduce memory usage, add missing features (LRU eviction, true parallelism), and ensure production-grade quality.

## Objectives

1. **Performance Optimization**: Optimize hot paths for maximum speed
2. **LRU Cache Eviction**: Implement configurable cache size limits with LRU policy
3. **True Parallel Compilation**: Add thread pool for parallel module compilation
4. **Memory Optimization**: Reduce memory footprint for large projects
5. **Code Quality**: Improve maintainability and reduce duplication
6. **Cross-Platform Support**: Ensure Windows/macOS/Linux compatibility

## Target Improvements

### Performance Targets

| Metric | GREEN Phase | REFACTOR Target | Improvement |
|--------|-------------|-----------------|-------------|
| No-change build | <100ms | <50ms | 2x faster |
| Single-file build | <500ms | <200ms | 2.5x faster |
| Cache lookup | O(1) | O(1) amortized | Optimized |
| Memory usage | Unbounded | <50MB | Bounded |
| Binary size | N/A | <200KB | Minimal |

### Code Quality Targets

| Metric | GREEN Phase | REFACTOR Target | Improvement |
|--------|-------------|-----------------|-------------|
| Lines of code | ~2,700 | ~2,400 | 11% reduction |
| Code duplication | ~5% | 0% | Eliminated |
| Cyclomatic complexity | <20 | <15 | Simplified |
| Test coverage | 22 tests | 30+ tests | +36% |

## Refactoring Strategy

### Phase 1: Cache Optimization (Priority 1)

**Goal**: Implement LRU eviction and optimize cache performance

**Components to Refactor**:

1. **LRU Cache Index**:
   ```ruchy
   pub struct LRUCache<K, V> {
       map: HashMap<K, Node<K, V>>,
       head: Option<Box<Node<K, V>>>,
       tail: Option<Box<Node<K, V>>>,
       capacity: usize,
       size: usize,
   }

   impl<K, V> LRUCache<K, V> {
       pub fun new(capacity: usize) -> Self
       pub fun get(&mut self, key: &K) -> Option<&V>
       pub fun put(&mut self, key: K, value: V) -> Option<V>
       fun move_to_head(&mut self, node: &mut Node<K, V>)
       fun evict_tail(&mut self) -> Option<(K, V)>
   }
   ```

2. **Cache Size Management**:
   - Track total cache size in bytes
   - Evict LRU entries when size limit exceeded
   - Configurable size limits (default: 1GB)
   - Provide cache statistics (hit rate, evictions)

3. **Cache Index Optimization**:
   - In-memory LRU index for hot entries
   - Disk-based storage for cold entries
   - Lazy loading of cache metadata
   - Batch index updates to reduce I/O

**Expected Improvements**:
- Cache lookup: O(1) amortized (with LRU updates)
- Memory bounded: <50MB for index
- Eviction overhead: <10ms per eviction
- Hit rate maintained: >95%

### Phase 2: Parallel Compilation (Priority 2)

**Goal**: Implement true parallel compilation with thread pool

**Components to Refactor**:

1. **Thread Pool**:
   ```ruchy
   pub struct ThreadPool {
       workers: Vec<Worker>,
       sender: Sender<Job>,
   }

   impl ThreadPool {
       pub fun new(size: usize) -> Self
       pub fun execute<F>(&self, f: F) where F: FnOnce() + Send + 'static
       pub fun join(&mut self)
   }
   ```

2. **Parallel Compilation Strategy**:
   - Use dependency graph to compute parallel batches
   - Compile independent modules in parallel
   - Respect dependency order between batches
   - Thread-safe cache access (mutex or lock-free)

3. **Work Stealing**:
   - Dynamic load balancing across threads
   - Work stealing queue for efficiency
   - Minimize idle threads

**Expected Improvements**:
- Parallel speedup: 2-4x on multi-core systems
- CPU utilization: >80% on all cores
- Thread overhead: <5ms
- Scalability: Linear up to 8 cores

### Phase 3: Performance Optimization (Priority 3)

**Goal**: Optimize hot paths for maximum speed

**Hot Paths Identified**:

1. **Hash Computation**:
   - Current: Compute SHA-256 for every file
   - Optimization: Check timestamp first, cache hashes
   - Expected: 10-100x faster for unchanged files

2. **Dependency Graph Traversal**:
   - Current: BFS for every build
   - Optimization: Cache transitive dependencies
   - Expected: 5-10x faster for large graphs

3. **Cache Lookup**:
   - Current: HashMap lookup + file read
   - Optimization: Memory-mapped files, batch reads
   - Expected: 2-3x faster lookups

4. **JSON Serialization**:
   - Current: Placeholder implementation
   - Optimization: Use efficient JSON library (serde-json)
   - Expected: 10-20x faster serialization

**Optimization Techniques**:
- Memoization for expensive computations
- Lazy evaluation where possible
- Memory-mapped I/O for large files
- SIMD for hash computation (optional)

**Expected Improvements**:
- Overall build time: 40-60% reduction
- Memory allocations: 50% reduction
- I/O operations: 70% reduction

### Phase 4: Code Quality (Priority 4)

**Goal**: Improve maintainability and reduce duplication

**Refactoring Tasks**:

1. **Extract Common Patterns**:
   - Error handling utilities
   - Path manipulation helpers
   - Hash computation wrappers
   - JSON serialization helpers

2. **Reduce Duplication**:
   - Consolidate similar functions
   - Use generics for type-agnostic code
   - Extract shared logic into utilities

3. **Improve Naming**:
   - More descriptive function names
   - Consistent naming conventions
   - Clear module organization

4. **Add Documentation**:
   - API documentation for public functions
   - Examples for common use cases
   - Architecture diagrams

**Expected Improvements**:
- Code duplication: 0% (from ~5%)
- Lines of code: -300 lines (11% reduction)
- Maintainability index: A+ grade

## Detailed Refactoring Plan

### Component 1: LRU Cache Implementation

**File**: `/bootstrap/stage3/lru_cache.ruchy`

**Key Type**:
```ruchy
struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<Box<Node<K, V>>>,
    next: Option<Box<Node<K, V>>>,
}

pub struct LRUCache<K, V> {
    map: HashMap<K, *mut Node<K, V>>,
    head: *mut Node<K, V>,
    tail: *mut Node<K, V>,
    capacity: usize,
    size: usize,
}

impl<K: Hash + Eq, V> LRUCache<K, V> {
    pub fun new(capacity: usize) -> Self

    pub fun get(&mut self, key: &K) -> Option<&V> {
        // 1. Lookup in map
        // 2. Move to head (most recently used)
        // 3. Return value
    }

    pub fun put(&mut self, key: K, value: V) {
        // 1. Check if key exists
        // 2. If exists: update and move to head
        // 3. If new: insert at head
        // 4. If over capacity: evict tail
    }

    fun move_to_head(&mut self, node: *mut Node<K, V>) {
        // Doubly-linked list manipulation
    }

    fun evict_tail(&mut self) -> Option<(K, V)> {
        // Remove tail node and return key/value
    }
}
```

**Integration with ModuleCache**:
```ruchy
pub struct ModuleCache {
    cache_dir: PathBuf,
    lru_index: LRUCache<String, CacheEntry>,  // Changed from HashMap
    hasher: ContentHasher,
    max_size_bytes: u64,
    current_size_bytes: u64,
}

impl ModuleCache {
    pub fun new_with_config(cache_dir: PathBuf, config: CacheConfig) -> Result<Self, String>

    pub fun get(&mut self, source_path: &str) -> Result<Option<CachedModule>, String> {
        // LRU cache automatically moves accessed entries to head
        if let Some(entry) = self.lru_index.get(source_path) {
            // ... validate and return
        }
    }

    pub fun put(&mut self, source_path: &str, module: CompiledModule) -> Result<(), String> {
        // Check size limits
        let module_size = module.wasm_binary.len() as u64;

        while self.current_size_bytes + module_size > self.max_size_bytes {
            // Evict LRU entry
            if let Some((path, entry)) = self.lru_index.evict_tail() {
                self.current_size_bytes -= self.get_entry_size(&entry);
                self.delete_cache_files(&entry)?;
            } else {
                break;  // Cache empty
            }
        }

        // Insert new entry (automatically goes to head)
        self.lru_index.put(source_path.to_string(), entry);
        self.current_size_bytes += module_size;
    }
}
```

### Component 2: Thread Pool Implementation

**File**: `/bootstrap/stage3/thread_pool.ruchy`

**Key Types**:
```ruchy
type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    fun new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = std::thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv();

                match job {
                    Ok(job) => job(),
                    Err(_) => break,  // Channel closed
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
}

impl ThreadPool {
    pub fun new(size: usize) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fun execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }

    pub fun join(&mut self) {
        drop(self.sender.clone());  // Close channel

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
```

**Integration with ParallelBuilder**:
```ruchy
pub struct ParallelBuilder {
    builder: IncrementalBuilder,
    thread_pool: ThreadPool,
}

impl ParallelBuilder {
    pub fun new(cache_dir: PathBuf, num_threads: usize) -> Result<Self, String> {
        let builder = IncrementalBuilder::new(cache_dir)?;
        let thread_pool = ThreadPool::new(num_threads);

        Ok(ParallelBuilder {
            builder,
            thread_pool,
        })
    }

    pub fun build(&mut self, project: &Project) -> BuildResult {
        // Get parallel batches from dependency graph
        let batches = self.builder.graph.get_parallel_batches();

        // Compile each batch in parallel
        for batch in batches {
            let (sender, receiver) = std::sync::mpsc::channel();

            // Spawn compilation jobs
            for module in &batch {
                let module = module.clone();
                let sender = sender.clone();

                self.thread_pool.execute(move || {
                    let result = compile_module(&module);
                    sender.send((module, result)).unwrap();
                });
            }

            // Collect results
            for _ in 0..batch.len() {
                let (module, result) = receiver.recv().unwrap();
                // Process result...
            }
        }

        self.thread_pool.join();

        // Return build result
    }
}
```

### Component 3: Hash Caching

**Optimization in ContentHasher**:
```ruchy
pub struct ContentHasher {
    // File path -> (timestamp, hash)
    hash_cache: HashMap<String, (u64, String)>,
}

impl ContentHasher {
    pub fun hash_file(&mut self, path: &str) -> Result<String, String> {
        let metadata = fs::metadata(path)?;
        let modified = get_timestamp(&metadata);

        // Check cache first
        if let Some((cached_time, cached_hash)) = self.hash_cache.get(path) {
            if *cached_time == modified {
                // File unchanged, return cached hash
                return Ok(cached_hash.clone());
            }
        }

        // File changed or not in cache, compute hash
        let content = fs::read_to_string(path)?;
        let hash = sha256(&content);

        // Update cache
        self.hash_cache.insert(path.to_string(), (modified, hash.clone()));

        Ok(hash)
    }

    pub fun clear_cache(&mut self) {
        self.hash_cache.clear();
    }
}
```

### Component 4: Transitive Dependency Caching

**Optimization in DependencyGraph**:
```ruchy
pub struct DependencyGraph {
    nodes: HashMap<String, ModuleNode>,
    edges: Vec<(String, String)>,

    // Cache for transitive dependencies
    transitive_cache: HashMap<String, Vec<String>>,
}

impl DependencyGraph {
    pub fun get_transitive_deps(&mut self, module: &str) -> Vec<String> {
        // Check cache first
        if let Some(cached) = self.transitive_cache.get(module) {
            return cached.clone();
        }

        // Compute transitive dependencies (BFS)
        let mut deps = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(module.to_string());

        while let Some(current) = queue.pop_front() {
            if let Some(node) = self.nodes.get(&current) {
                for dep in &node.dependencies {
                    if !deps.contains(dep) && dep != module {
                        deps.insert(dep.clone());
                        queue.push_back(dep.clone());
                    }
                }
            }
        }

        let result: Vec<String> = deps.into_iter().collect();

        // Cache result
        self.transitive_cache.insert(module.to_string(), result.clone());

        result
    }

    pub fun invalidate_transitive_cache(&mut self) {
        self.transitive_cache.clear();
    }
}
```

## Implementation Timeline

### Week 1: Core Optimizations

**Days 1-2: LRU Cache**
- Implement LRU cache data structure
- Integrate with ModuleCache
- Add size limits and eviction
- **Target**: Cache bounded to <50MB

**Days 3-4: Thread Pool**
- Implement thread pool
- Integrate with ParallelBuilder
- Add work distribution logic
- **Target**: 2-4x speedup on multi-core

**Day 5: Hash Caching**
- Add timestamp-based hash caching
- Optimize file I/O
- **Target**: 10-100x faster for unchanged files

### Week 2: Polish and Testing

**Days 6-7: Transitive Caching**
- Cache transitive dependencies
- Optimize graph traversal
- **Target**: 5-10x faster graph operations

**Days 8-9: Code Quality**
- Reduce duplication
- Extract utilities
- Improve documentation
- **Target**: 11% LOC reduction, 0% duplication

**Day 10: Testing and Validation**
- Add new tests for optimizations
- Benchmark performance
- Validate targets achieved
- **Target**: All tests passing, 40-60% speedup

## Testing Strategy

### New Tests to Add (8+ tests)

1. **LRU Cache Tests** (3 tests):
   - LRU eviction order correctness
   - Size limit enforcement
   - Cache hit rate with eviction

2. **Thread Pool Tests** (2 tests):
   - Parallel job execution
   - Thread pool shutdown

3. **Hash Caching Tests** (2 tests):
   - Timestamp-based cache hit
   - Cache invalidation on file change

4. **Performance Tests** (1+ tests):
   - Benchmark parallel vs sequential
   - Benchmark with/without caching

### Benchmarking

Run comprehensive benchmarks to validate improvements:
```bash
# Baseline (GREEN phase)
./benchmark_green.sh

# Optimized (REFACTOR phase)
./benchmark_refactor.sh

# Compare
./compare_benchmarks.sh
```

**Expected Results**:
- No-change build: 50-100x faster (100ms → 1-2ms)
- Single-file build: 2-3x faster (500ms → 150-250ms)
- 10-file build: 1.5-2x faster (1000ms → 500-700ms)

## Success Criteria

✅ **LRU Cache**: Size bounded, eviction working, <50MB memory

✅ **Thread Pool**: 2-4x parallel speedup on multi-core systems

✅ **Hash Caching**: 10-100x faster for unchanged files

✅ **Code Quality**: 11% LOC reduction, 0% duplication

✅ **Performance**: 40-60% overall build time reduction

✅ **Tests**: All existing tests passing + 8 new tests

✅ **Cross-Platform**: Working on Linux, macOS, Windows

## Risk Mitigation

### Risk 1: Thread Safety Issues

**Impact**: Race conditions, deadlocks, data corruption

**Mitigation**:
- Use Arc<Mutex<T>> for shared state
- Minimize shared mutable state
- Extensive testing with thread sanitizer
- Property tests for concurrent operations

### Risk 2: LRU Overhead

**Impact**: LRU updates slow down cache operations

**Mitigation**:
- Lazy LRU updates (batch every N accesses)
- Lock-free LRU implementation (optional)
- Benchmark to ensure <10% overhead

### Risk 3: Memory Leaks

**Impact**: Unbounded memory growth over time

**Mitigation**:
- Careful RAII (drop traits)
- Memory profiling during tests
- Valgrind/ASan validation

## Next Steps After REFACTOR Phase

Once REFACTOR phase is complete:

1. **TOOL Phase**: Comprehensive validation
   - Property testing
   - Fuzz testing
   - Cross-platform validation
   - Production deployment

2. **Production Deployment**: Enable by default
   - Documentation
   - Migration guide
   - Performance monitoring

## Conclusion

The REFACTOR phase will optimize the GREEN phase implementation through:
- LRU cache eviction for bounded memory
- True parallel compilation for multi-core speedup
- Hash caching for unchanged file optimization
- Code quality improvements for maintainability

By following this plan, we will achieve production-grade incremental compilation with 40-60% performance improvements over GREEN phase and 5-50x speedup over full rebuilds.

---

**Phase**: REFACTOR
**Status**: PLANNED
**Target**: 40-60% performance improvement over GREEN
**Timeline**: 2 weeks
**Tests to Add**: 8+ new tests
