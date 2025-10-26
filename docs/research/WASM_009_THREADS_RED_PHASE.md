# WASM-009: Thread Support - RED Phase Plan

## Overview

The RED phase for WASM-009 focuses on creating comprehensive test specifications for WebAssembly thread support, including shared memory, atomic operations, thread creation, and synchronization primitives. Following Extreme TDD methodology, all tests are designed to fail initially.

## Objectives

1. **Comprehensive Test Specifications** - 30+ tests across 4 major categories
2. **Thread Safety Requirements** - Define correctness guarantees
3. **Performance Targets** - Parallel speedup and overhead limits
4. **WebAssembly Threads Spec** - Follow official specification

## Thread Support Categories

### Category 1: Shared Memory (8 tests)
Tests for WebAssembly shared memory implementation:
- Shared memory creation and initialization
- Memory sharing between threads
- Memory access patterns (read/write)
- Memory bounds checking
- Memory growth in shared context
- Memory isolation verification
- Memory fence operations
- Memory visibility guarantees

### Category 2: Atomic Operations (10 tests)
Tests for atomic memory operations:
- Atomic load/store (i32, i64)
- Atomic RMW operations (add, sub, and, or, xor, exchange)
- Compare-and-swap (CAS) operations
- Atomic wait/notify primitives
- Memory ordering guarantees
- ABA problem prevention
- Atomic operation atomicity verification
- Atomic operation performance
- Atomic fence instructions
- Weak vs strong atomics

### Category 3: Thread Management (8 tests)
Tests for thread creation and lifecycle:
- Thread spawn/creation
- Thread join/termination
- Thread local storage
- Thread pooling
- Maximum thread limits
- Thread creation overhead
- Thread cleanup on error
- Thread resource management

### Category 4: Synchronization Primitives (6 tests)
Tests for thread synchronization:
- Mutex lock/unlock
- Condition variables (wait/notify)
- Barriers
- Reader-writer locks
- Deadlock detection
- Priority inversion prevention

### Category 5: Integration & Performance (3 tests)
Tests for overall thread system:
- Parallel execution correctness
- Scalability (1-16 threads)
- Performance benchmarks

**Total RED Phase Tests**: 35 tests

## Performance Targets

### Parallel Speedup
- **Target**: 3-4x speedup on 4 cores for embarrassingly parallel workloads
- **Measurement**: Compare single-threaded vs multi-threaded execution
- **Baseline**: Single-threaded execution time = 100%

### Thread Creation Overhead
- **Target**: <10ms per thread creation
- **Measurement**: Time to spawn new thread
- **Baseline**: Sequential execution overhead

### Atomic Operation Performance
- **Target**: <10ns per atomic operation (load/store)
- **Measurement**: Throughput of atomic operations
- **Baseline**: Non-atomic operation performance

### Memory Overhead
- **Target**: <1MB per thread
- **Measurement**: Memory usage per thread
- **Baseline**: Single-threaded memory usage

## WebAssembly Threads Specification Compliance

Following the [WebAssembly Threads Proposal](https://github.com/WebAssembly/threads):

### Shared Memory
- Shared linear memory via `SharedArrayBuffer`
- Memory declared with `shared` flag
- Maximum memory size: 4GB (32-bit addressing)

### Atomic Operations
- Support all atomic operations from spec:
  - `i32.atomic.load`, `i64.atomic.load`
  - `i32.atomic.store`, `i64.atomic.store`
  - `i32.atomic.rmw.add`, `i64.atomic.rmw.add`
  - `i32.atomic.rmw.sub`, `i64.atomic.rmw.sub`
  - `i32.atomic.rmw.and`, `i64.atomic.rmw.and`
  - `i32.atomic.rmw.or`, `i64.atomic.rmw.or`
  - `i32.atomic.rmw.xor`, `i64.atomic.rmw.xor`
  - `i32.atomic.rmw.xchg`, `i64.atomic.rmw.xchg`
  - `i32.atomic.rmw.cmpxchg`, `i64.atomic.rmw.cmpxchg`

### Thread Management
- Worker-based execution model (Web Workers in browser)
- Shared module instance across workers
- Thread-local storage via imported functions

### Synchronization
- `memory.atomic.wait32`, `memory.atomic.wait64`
- `memory.atomic.notify`
- Timeout support for wait operations

## Test File Structure

### Test File 1: Shared Memory Tests
**File**: `/validation/wasm/threads/test_shared_memory_red.ruchy`
**Estimated LOC**: ~250 lines
**Tests**: 8 tests

### Test File 2: Atomic Operations Tests
**File**: `/validation/wasm/threads/test_atomic_operations_red.ruchy`
**Estimated LOC**: ~350 lines
**Tests**: 10 tests

### Test File 3: Thread Management Tests
**File**: `/validation/wasm/threads/test_thread_management_red.ruchy`
**Estimated LOC**: ~300 lines
**Tests**: 8 tests

### Test File 4: Synchronization Tests
**File**: `/validation/wasm/threads/test_synchronization_red.ruchy`
**Estimated LOC**: ~250 lines
**Tests**: 6 tests

### Test File 5: Integration Tests
**File**: `/validation/wasm/threads/test_thread_integration_red.ruchy`
**Estimated LOC**: ~150 lines
**Tests**: 3 tests

**Total**: ~1,300 LOC, 35 tests

## Thread Safety Guarantees

### Correctness Properties
1. **Sequential Consistency**: Operations appear in program order
2. **Atomicity**: Atomic operations are indivisible
3. **Visibility**: Writes are eventually visible to all threads
4. **No Data Races**: Proper synchronization prevents races
5. **Deadlock Freedom**: No circular wait conditions
6. **Liveness**: Threads make progress

### Thread Safety Invariants
- Shared memory access must use atomic operations or locks
- Non-atomic operations on shared memory are undefined behavior
- Thread creation/destruction is thread-safe
- Synchronization primitives guarantee happens-before relationships

## Test Patterns

### Pattern 1: Basic Functionality Test
```ruchy
fun test_basic_atomic_load() -> bool {
    let shared_mem = create_shared_memory(1024);

    // Store value
    atomic_store_i32(shared_mem, 0, 42);

    // Load value
    let value = atomic_load_i32(shared_mem, 0);

    if value == 42 {
        println("  PASS: Atomic load/store works");
        true
    } else {
        println("  FAIL: Atomic load/store failed");
        false
    }
}
```

### Pattern 2: Concurrency Correctness Test
```ruchy
fun test_atomic_increment_concurrent() -> bool {
    let shared_mem = create_shared_memory(1024);
    atomic_store_i32(shared_mem, 0, 0);

    let num_threads = 4;
    let increments_per_thread = 1000;

    // Spawn threads that increment counter
    let threads = spawn_threads(num_threads, || {
        for _ in 0..increments_per_thread {
            atomic_add_i32(shared_mem, 0, 1);
        }
    });

    // Wait for all threads
    join_all(threads);

    let final_value = atomic_load_i32(shared_mem, 0);
    let expected = num_threads * increments_per_thread;

    if final_value == expected {
        println("  PASS: Concurrent increments correct");
        true
    } else {
        println("  FAIL: Lost updates (expected {}, got {})", expected, final_value);
        false
    }
}
```

### Pattern 3: Performance Test
```ruchy
fun test_thread_creation_overhead() -> bool {
    let num_threads = 100;
    let start_time = current_time_ms();

    let threads = spawn_threads(num_threads, || {
        // Empty thread body
    });

    join_all(threads);

    let elapsed = current_time_ms() - start_time;
    let per_thread_ms = elapsed / num_threads;

    if per_thread_ms < 10 {
        println("  PASS: Thread creation <10ms per thread");
        true
    } else {
        println("  FAIL: Thread creation too slow ({}ms per thread)", per_thread_ms);
        false
    }
}
```

## Success Criteria - RED Phase

✅ **All Tests Created**: 35 tests across 5 test files
✅ **All Tests Failing**: Tests demonstrate missing implementation
✅ **Clear Requirements**: Each test documents expected behavior
✅ **Performance Targets**: Speedup, overhead, and latency targets defined
✅ **WebAssembly Spec Compliance**: Follow official threads proposal
✅ **Documentation Complete**: RED phase plan and test documentation

## Known Challenges

### Challenge 1: Browser Compatibility
**Issue**: Not all browsers support WebAssembly threads
**Mitigation**: Feature detection, polyfills for testing, graceful degradation

### Challenge 2: Shared Memory Restrictions
**Issue**: SharedArrayBuffer disabled in some contexts due to Spectre
**Mitigation**: Require COOP/COEP headers, document deployment requirements

### Challenge 3: Debugging Difficulty
**Issue**: Thread bugs are non-deterministic and hard to reproduce
**Mitigation**: Comprehensive logging, thread sanitizers, stress testing

### Challenge 4: Performance Portability
**Issue**: Performance varies across platforms and browsers
**Mitigation**: Multiple benchmarks, platform-specific optimizations

## Timeline

- **Test Specification**: 1 day (5 test files, ~1,300 LOC)
- **Documentation**: 0.5 days (RED phase plan and completion report)
- **Total**: 1.5 days

## Comparison with Previous Features

| Metric | WASM-006 RED | WASM-007 RED | WASM-008 RED | WASM-009 RED (Planned) |
|--------|--------------|--------------|--------------|------------------------|
| Test Files | 3 | 3 | 2 | 5 |
| Unit Tests | 30 | 30 | 20 | 35 |
| Test LOC | ~1,630 | ~1,630 | ~500 | ~1,300 |
| Timeline | 1-2 days | 1-2 days | 1-2 days | 1.5 days |

WASM-009 has more test files and tests due to complexity of thread support.

## Next Steps (After RED)

1. **Document RED Completion**
   - Create WASM_009_THREADS_RED_COMPLETE.md
   - Record all failing tests
   - List limitations for GREEN phase

2. **Update INTEGRATION.md**
   - Mark RED phase complete
   - Document test results

3. **Begin GREEN Phase**
   - Minimal thread support implementation
   - Basic shared memory and atomics
   - Simple thread creation

## Conclusion

The RED phase for WASM-009 (Thread Support) establishes comprehensive requirements for WebAssembly threading support through 35 failing tests across shared memory, atomic operations, thread management, synchronization, and integration testing.

---

**Phase**: RED
**Status**: PLANNED
**Tests**: 35 tests across 5 files (~1,300 LOC)
**Timeline**: 1.5 days
**Next**: Begin test file creation
