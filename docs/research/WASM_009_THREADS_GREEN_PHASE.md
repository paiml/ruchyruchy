# WASM-009: Thread Support - GREEN Phase Plan

## Overview

The GREEN phase for WASM-009 focuses on implementing minimal thread support to make the RED phase tests pass. Following Extreme TDD methodology, this phase prioritizes simplicity and correctness over performance, using straightforward approaches to establish a baseline.

## Objectives

1. **Make Tests Pass** - Implement minimal logic for 35 tests (18 created + 17 specified)
2. **Establish Baseline** - Create performance baseline for REFACTOR
3. **Simple Implementation** - Use Web Workers and JS Atomics API wrappers
4. **No Premature Optimization** - Focus on correctness, not performance

## GREEN Phase Implementation Strategy

### Priority: Correctness > Performance

- Use simple wrappers around browser APIs
- Leverage JavaScript `SharedArrayBuffer` and `Atomics` API
- Implement basic Web Workers for threading
- Defer advanced features to REFACTOR phase

### Performance Expectations (GREEN Phase)

- Parallel speedup: 2-3x on 4 cores (target: 3-4x)
- Thread creation: <50ms per thread (target: <10ms)
- Atomic operations: <200ns per op (target: <100ns)
- Memory overhead: <2MB per thread (target: <1MB)

**GREEN serves as baseline for REFACTOR improvements.**

## Implementation Plan

### Component 1: Shared Memory (Minimal)

**File**: `/bootstrap/stage3/shared_memory_green.ruchy`
**Estimated LOC**: ~300 lines

**Approach**: Simple wrapper around `SharedArrayBuffer`

```ruchy
// Minimal shared memory implementation
pub struct SharedMemory {
    buffer: SharedArrayBuffer,
    size: usize,
}

impl SharedMemory {
    pub fun new(size: usize) -> Result<Self, String> {
        // Check for SharedArrayBuffer support
        if !is_shared_array_buffer_supported() {
            return Err("SharedArrayBuffer not supported".to_string());
        }

        // Create shared buffer
        let buffer = create_shared_array_buffer(size);

        Ok(SharedMemory {
            buffer: buffer,
            size: size,
        })
    }

    pub fun write_i32(&self, offset: usize, value: i32) -> Result<(), String> {
        // Bounds check
        if offset + 4 > self.size {
            return Err("Out of bounds".to_string());
        }

        // Write using DataView (non-atomic)
        let view = create_data_view(self.buffer, offset, 4);
        view.setInt32(0, value, true); // little-endian

        Ok(())
    }

    pub fun read_i32(&self, offset: usize) -> Result<i32, String> {
        // Bounds check
        if offset + 4 > self.size {
            return Err("Out of bounds".to_string());
        }

        // Read using DataView (non-atomic)
        let view = create_data_view(self.buffer, offset, 4);
        let value = view.getInt32(0, true); // little-endian

        Ok(value)
    }

    pub fun grow(&mut self, pages: usize) -> Result<(), String> {
        // SharedArrayBuffer cannot be resized in current browsers
        // Allocate new buffer and copy data
        let new_size = self.size + (pages * 65536); // 64KB per page
        let new_buffer = create_shared_array_buffer(new_size);

        // Copy existing data
        copy_buffer(self.buffer, new_buffer, self.size);

        self.buffer = new_buffer;
        self.size = new_size;

        Ok(())
    }
}

// JavaScript interop helpers (simple wrappers)
extern fun is_shared_array_buffer_supported() -> bool;
extern fun create_shared_array_buffer(size: usize) -> SharedArrayBuffer;
extern fun create_data_view(buffer: SharedArrayBuffer, offset: usize, length: usize) -> DataView;
extern fun copy_buffer(src: SharedArrayBuffer, dst: SharedArrayBuffer, length: usize);
```

**Tests Passing**: 8/8 shared memory tests

---

### Component 2: Atomic Operations (Minimal)

**File**: `/bootstrap/stage3/atomic_operations_green.ruchy`
**Estimated LOC**: ~400 lines

**Approach**: Simple wrappers around JavaScript `Atomics` API

```ruchy
// Minimal atomic operations implementation
pub struct AtomicOperations;

impl AtomicOperations {
    // Atomic load (i32)
    pub fun load_i32(memory: &SharedMemory, offset: usize) -> Result<i32, String> {
        // Bounds check
        if offset + 4 > memory.size {
            return Err("Out of bounds".to_string());
        }

        // Use Atomics.load
        let array = create_int32_array(memory.buffer);
        let index = offset / 4; // i32 index
        let value = atomic_load_i32_js(array, index);

        Ok(value)
    }

    // Atomic store (i32)
    pub fun store_i32(memory: &SharedMemory, offset: usize, value: i32) -> Result<(), String> {
        // Bounds check
        if offset + 4 > memory.size {
            return Err("Out of bounds".to_string());
        }

        // Use Atomics.store
        let array = create_int32_array(memory.buffer);
        let index = offset / 4;
        atomic_store_i32_js(array, index, value);

        Ok(())
    }

    // Atomic add (RMW)
    pub fun add_i32(memory: &SharedMemory, offset: usize, value: i32) -> Result<i32, String> {
        // Bounds check
        if offset + 4 > memory.size {
            return Err("Out of bounds".to_string());
        }

        // Use Atomics.add
        let array = create_int32_array(memory.buffer);
        let index = offset / 4;
        let old_value = atomic_add_i32_js(array, index, value);

        Ok(old_value)
    }

    // Atomic sub (RMW)
    pub fun sub_i32(memory: &SharedMemory, offset: usize, value: i32) -> Result<i32, String> {
        // Use Atomics.sub
        let array = create_int32_array(memory.buffer);
        let index = offset / 4;
        let old_value = atomic_sub_i32_js(array, index, value);

        Ok(old_value)
    }

    // Atomic and/or/xor (RMW)
    pub fun and_i32(memory: &SharedMemory, offset: usize, value: i32) -> Result<i32, String> {
        let array = create_int32_array(memory.buffer);
        let index = offset / 4;
        let old_value = atomic_and_i32_js(array, index, value);
        Ok(old_value)
    }

    pub fun or_i32(memory: &SharedMemory, offset: usize, value: i32) -> Result<i32, String> {
        let array = create_int32_array(memory.buffer);
        let index = offset / 4;
        let old_value = atomic_or_i32_js(array, index, value);
        Ok(old_value)
    }

    pub fun xor_i32(memory: &SharedMemory, offset: usize, value: i32) -> Result<i32, String> {
        let array = create_int32_array(memory.buffer);
        let index = offset / 4;
        let old_value = atomic_xor_i32_js(array, index, value);
        Ok(old_value)
    }

    // Atomic exchange (RMW)
    pub fun exchange_i32(memory: &SharedMemory, offset: usize, value: i32) -> Result<i32, String> {
        let array = create_int32_array(memory.buffer);
        let index = offset / 4;
        let old_value = atomic_exchange_i32_js(array, index, value);
        Ok(old_value)
    }

    // Compare-and-swap (RMW)
    pub fun compare_exchange_i32(
        memory: &SharedMemory,
        offset: usize,
        expected: i32,
        desired: i32
    ) -> Result<i32, String> {
        let array = create_int32_array(memory.buffer);
        let index = offset / 4;
        let old_value = atomic_compare_exchange_i32_js(array, index, expected, desired);
        Ok(old_value)
    }

    // Atomic wait
    pub fun wait_i32(
        memory: &SharedMemory,
        offset: usize,
        expected: i32,
        timeout_ms: i64
    ) -> Result<WaitResult, String> {
        let array = create_int32_array(memory.buffer);
        let index = offset / 4;
        let result = atomic_wait_i32_js(array, index, expected, timeout_ms);
        Ok(result)
    }

    // Atomic notify
    pub fun notify(memory: &SharedMemory, offset: usize, count: i32) -> Result<i32, String> {
        let array = create_int32_array(memory.buffer);
        let index = offset / 4;
        let woken = atomic_notify_js(array, index, count);
        Ok(woken)
    }
}

// JavaScript Atomics API interop
extern fun atomic_load_i32_js(array: Int32Array, index: usize) -> i32;
extern fun atomic_store_i32_js(array: Int32Array, index: usize, value: i32);
extern fun atomic_add_i32_js(array: Int32Array, index: usize, value: i32) -> i32;
extern fun atomic_sub_i32_js(array: Int32Array, index: usize, value: i32) -> i32;
extern fun atomic_and_i32_js(array: Int32Array, index: usize, value: i32) -> i32;
extern fun atomic_or_i32_js(array: Int32Array, index: usize, value: i32) -> i32;
extern fun atomic_xor_i32_js(array: Int32Array, index: usize, value: i32) -> i32;
extern fun atomic_exchange_i32_js(array: Int32Array, index: usize, value: i32) -> i32;
extern fun atomic_compare_exchange_i32_js(array: Int32Array, index: usize, expected: i32, desired: i32) -> i32;
extern fun atomic_wait_i32_js(array: Int32Array, index: usize, expected: i32, timeout_ms: i64) -> WaitResult;
extern fun atomic_notify_js(array: Int32Array, index: usize, count: i32) -> i32;
extern fun create_int32_array(buffer: SharedArrayBuffer) -> Int32Array;
```

**Tests Passing**: 10/10 atomic operations tests

---

### Component 3: Thread Management (Minimal)

**File**: `/bootstrap/stage3/thread_manager_green.ruchy`
**Estimated LOC**: ~350 lines

**Approach**: Simple Web Worker wrapper

```ruchy
// Minimal thread management implementation
pub struct Thread {
    worker: Worker,
    id: usize,
    joined: bool,
}

pub struct ThreadManager {
    next_id: usize,
    threads: Vec<Thread>,
}

impl ThreadManager {
    pub fun new() -> Self {
        ThreadManager {
            next_id: 1,
            threads: Vec::new(),
        }
    }

    pub fun spawn(&mut self, entry_point: String, shared_memory: &SharedMemory) -> Result<ThreadHandle, String> {
        // Create Web Worker
        let worker = create_worker_js()?;

        // Send initialization message with shared memory
        let init_msg = ThreadInitMessage {
            thread_id: self.next_id,
            shared_buffer: shared_memory.buffer.clone(),
            entry_point: entry_point,
        };

        send_to_worker_js(worker, init_msg)?;

        let handle = ThreadHandle {
            id: self.next_id,
        };

        self.threads.push(Thread {
            worker: worker,
            id: self.next_id,
            joined: false,
        });

        self.next_id += 1;

        Ok(handle)
    }

    pub fun join(&mut self, handle: ThreadHandle) -> Result<(), String> {
        // Find thread
        let thread = self.threads.iter_mut()
            .find(|t| t.id == handle.id)
            .ok_or("Thread not found")?;

        if thread.joined {
            return Err("Thread already joined".to_string());
        }

        // Wait for worker to finish (blocking)
        wait_for_worker_js(thread.worker)?;

        thread.joined = true;

        Ok(())
    }

    pub fun spawn_multiple(
        &mut self,
        count: usize,
        entry_point: String,
        shared_memory: &SharedMemory
    ) -> Result<Vec<ThreadHandle>, String> {
        let mut handles = Vec::new();

        for _ in 0..count {
            let handle = self.spawn(entry_point.clone(), shared_memory)?;
            handles.push(handle);
        }

        Ok(handles)
    }

    pub fun join_all(&mut self, handles: Vec<ThreadHandle>) -> Result<(), String> {
        for handle in handles {
            self.join(handle)?;
        }

        Ok(())
    }
}

pub struct ThreadHandle {
    pub id: usize,
}

struct ThreadInitMessage {
    thread_id: usize,
    shared_buffer: SharedArrayBuffer,
    entry_point: String,
}

// JavaScript Web Worker interop
extern fun create_worker_js() -> Result<Worker, String>;
extern fun send_to_worker_js(worker: Worker, message: ThreadInitMessage) -> Result<(), String>;
extern fun wait_for_worker_js(worker: Worker) -> Result<(), String>;
```

**Tests Passing**: 6/8 thread management tests (simple cases only)

---

### Component 4: Synchronization Primitives (Minimal)

**File**: `/bootstrap/stage3/synchronization_green.ruchy`
**Estimated LOC**: ~250 lines

**Approach**: Simple mutex and condition variable using atomics

```ruchy
// Minimal synchronization primitives
pub struct Mutex {
    memory: SharedMemory,
    lock_offset: usize,
}

impl Mutex {
    pub fun new(memory: SharedMemory, offset: usize) -> Self {
        // Initialize lock to 0 (unlocked)
        AtomicOperations::store_i32(&memory, offset, 0).unwrap();

        Mutex {
            memory: memory,
            lock_offset: offset,
        }
    }

    pub fun lock(&self) -> Result<(), String> {
        // Spin-lock implementation (simple)
        loop {
            // Try to acquire lock using CAS
            let old = AtomicOperations::compare_exchange_i32(
                &self.memory,
                self.lock_offset,
                0, // expected: unlocked
                1  // desired: locked
            )?;

            if old == 0 {
                // Lock acquired
                return Ok(());
            }

            // Spin (could use wait here in REFACTOR)
        }
    }

    pub fun unlock(&self) -> Result<(), String> {
        // Release lock
        AtomicOperations::store_i32(&self.memory, self.lock_offset, 0)?;
        Ok(())
    }
}

pub struct CondVar {
    memory: SharedMemory,
    wait_offset: usize,
}

impl CondVar {
    pub fun new(memory: SharedMemory, offset: usize) -> Self {
        // Initialize wait counter to 0
        AtomicOperations::store_i32(&memory, offset, 0).unwrap();

        CondVar {
            memory: memory,
            wait_offset: offset,
        }
    }

    pub fun wait(&self, mutex: &Mutex, timeout_ms: i64) -> Result<(), String> {
        // Increment wait counter
        AtomicOperations::add_i32(&self.memory, self.wait_offset, 1)?;

        // Release mutex
        mutex.unlock()?;

        // Wait for notification
        let current = AtomicOperations::load_i32(&self.memory, self.wait_offset)?;
        AtomicOperations::wait_i32(&self.memory, self.wait_offset, current, timeout_ms)?;

        // Re-acquire mutex
        mutex.lock()?;

        Ok(())
    }

    pub fun notify_one(&self) -> Result<(), String> {
        // Wake one waiter
        AtomicOperations::notify(&self.memory, self.wait_offset, 1)?;
        Ok(())
    }

    pub fun notify_all(&self) -> Result<(), String> {
        // Wake all waiters (use large count)
        AtomicOperations::notify(&self.memory, self.wait_offset, i32::MAX)?;
        Ok(())
    }
}
```

**Tests Passing**: 4/6 synchronization tests (simple cases only)

---

### Component 5: Integration & Orchestration

**File**: `/bootstrap/stage3/thread_runtime_green.ruchy`
**Estimated LOC**: ~200 lines

**Main Thread Runtime**:

```ruchy
pub struct ThreadRuntime {
    memory: SharedMemory,
    thread_manager: ThreadManager,
}

impl ThreadRuntime {
    pub fun new(memory_size: usize) -> Result<Self, String> {
        let memory = SharedMemory::new(memory_size)?;
        let thread_manager = ThreadManager::new();

        Ok(ThreadRuntime {
            memory: memory,
            thread_manager: thread_manager,
        })
    }

    pub fun spawn_thread(&mut self, entry: String) -> Result<ThreadHandle, String> {
        self.thread_manager.spawn(entry, &self.memory)
    }

    pub fun join_thread(&mut self, handle: ThreadHandle) -> Result<(), String> {
        self.thread_manager.join(handle)
    }

    pub fun spawn_threads(&mut self, count: usize, entry: String) -> Result<Vec<ThreadHandle>, String> {
        self.thread_manager.spawn_multiple(count, entry, &self.memory)
    }

    pub fun join_all(&mut self, handles: Vec<ThreadHandle>) -> Result<(), String> {
        self.thread_manager.join_all(handles)
    }

    pub fun get_shared_memory(&self) -> &SharedMemory {
        &self.memory
    }
}
```

**Tests Passing**: 3/3 integration tests

## Total Implementation Size (GREEN Phase)

| Component | GREEN LOC | Tests Passing |
|-----------|-----------|---------------|
| Shared Memory | ~300 | 8/8 (100%) |
| Atomic Operations | ~400 | 10/10 (100%) |
| Thread Management | ~350 | 6/8 (75%) |
| Synchronization | ~250 | 4/6 (67%) |
| Integration | ~200 | 3/3 (100%) |
| **Total** | **~1,500 LOC** | **31/35 (89%)** |

**Note**: Some tests may still fail in GREEN phase due to minimal implementation. REFACTOR will achieve 35/35.

## Performance Baseline (GREEN Phase)

### Expected Metrics

**Parallel Speedup**:
- Baseline: 1x (single-threaded)
- GREEN: 2-3x on 4 cores (simple parallelization)
- Target (REFACTOR): 3-4x (optimized scheduling)

**Thread Creation**:
- GREEN: 20-50ms per thread (Web Worker overhead)
- Target (REFACTOR): <10ms (worker pooling)

**Atomic Operations**:
- GREEN: 100-200ns per op (JS Atomics wrapper overhead)
- Target (REFACTOR): <100ns (optimized wrappers)

**Memory Overhead**:
- GREEN: 1-2MB per thread (Web Worker overhead)
- Target (REFACTOR): <1MB (optimized workers)

## Test Passing Strategy

### Shared Memory (8/8 Expected)
All tests should pass with simple SharedArrayBuffer wrapper:
- ✅ Memory creation and initialization
- ✅ Multi-thread access
- ✅ Bounds checking
- ✅ Memory growth
- ✅ Memory isolation
- ✅ Memory fence operations
- ✅ Memory visibility

### Atomic Operations (10/10 Expected)
All tests should pass with JS Atomics API wrappers:
- ✅ Atomic load/store (i32, i64)
- ✅ Atomic RMW operations
- ✅ Compare-and-swap
- ✅ Concurrent increment
- ✅ Wait/notify primitives
- ✅ Performance (acceptable baseline)

### Thread Management (6/8 Expected)
Simple cases only:
- ✅ Thread spawn/join
- ✅ Multiple threads
- ✅ Thread termination
- ⏳ Advanced pooling (REFACTOR)
- ⏳ Thread limits enforcement (REFACTOR)
- ✅ Basic resource management

### Synchronization (4/6 Expected)
Simple cases only:
- ✅ Mutex lock/unlock
- ✅ Condition variable wait/notify
- ⏳ Deadlock detection (REFACTOR)
- ⏳ Priority inversion (REFACTOR)

### Integration (3/3 Expected)
All integration tests should pass:
- ✅ Parallel execution works
- ✅ Basic scalability
- ✅ Performance baseline

## Known Limitations (GREEN Phase)

### Deferred to REFACTOR Phase

**Shared Memory**:
- ⏳ Efficient memory growth (current: copy on grow)
- ⏳ Memory access optimization
- ⏳ Advanced fence strategies

**Atomic Operations**:
- ⏳ Performance optimization
- ⏳ i64 atomic operations (limited browser support)
- ⏳ Custom atomic operations

**Thread Management**:
- ⏳ Thread pooling (avoid repeated Worker creation)
- ⏳ Thread-local storage
- ⏳ Advanced scheduling
- ⏳ Priority threads

**Synchronization**:
- ⏳ Advanced mutex (non-spinning)
- ⏳ Reader-writer locks
- ⏳ Barriers
- ⏳ Deadlock detection
- ⏳ Condition variable broadcast optimization

**All limitations addressed in REFACTOR phase.**

## Success Criteria - GREEN Phase

✅ **31-35 tests passing** (89-100% success rate)
✅ **Parallel speedup: 2-3x** (baseline established)
✅ **Thread creation: <50ms** (baseline established)
✅ **All code compiles** (no syntax/type errors)
✅ **Documentation complete** (GREEN completion report)

## Timeline

- **Implementation**: 2 days (5 components)
- **Testing**: 0.5 days (make tests pass)
- **Documentation**: 0.5 days (completion report)
- **Total**: 3 days

## Browser Compatibility Notes

### Required Features
- `SharedArrayBuffer` support
- `Atomics` API support
- Web Workers support
- Cross-Origin Isolation (COOP/COEP headers)

### Detection and Fallbacks
```ruchy
fun check_thread_support() -> Result<(), String> {
    if !is_shared_array_buffer_supported() {
        return Err("SharedArrayBuffer not supported. Requires COOP/COEP headers.".to_string());
    }

    if !is_atomics_supported() {
        return Err("Atomics API not supported.".to_string());
    }

    if !is_worker_supported() {
        return Err("Web Workers not supported.".to_string());
    }

    Ok(())
}
```

## Next Steps (After GREEN)

1. **Document GREEN Completion**
   - Create WASM_009_THREADS_GREEN_COMPLETE.md
   - Record baseline performance metrics
   - List limitations for REFACTOR

2. **Update INTEGRATION.md**
   - Mark GREEN phase complete
   - Document test results

3. **Begin REFACTOR Phase**
   - Thread pooling for performance
   - Optimized atomic operations
   - Advanced synchronization primitives
   - Achieve 35/35 tests passing

## Comparison with Previous Features

| Metric | WASM-006 GREEN | WASM-007 GREEN | WASM-008 GREEN | WASM-009 GREEN (Planned) |
|--------|----------------|----------------|----------------|-------------------------|
| Implementation LOC | ~2,700 | ~1,975 | ~1,500 | ~1,500 |
| Tests Passing | 30/30 | 30/30 | 36/40 (90%) | 31/35 (89%) |
| Timeline | 2-3 days | 2-3 days | 2-3 days | 3 days |
| Performance | Baseline | Baseline | 10-20% improvement | 2-3x speedup |

## Conclusion

The GREEN phase for WASM-009 focuses on minimal thread support implementation to make tests pass. Using simple wrappers around browser APIs (SharedArrayBuffer, Atomics, Web Workers), we'll establish a performance baseline and validate correctness. The REFACTOR phase will then optimize for production with advanced techniques and achieve full test passage (35/35).

---

**Phase**: GREEN
**Status**: PLANNED
**Implementation**: ~1,500 LOC across 5 files
**Tests**: 31-35/35 expected to pass
**Timeline**: 3 days
**Next**: Begin implementation of shared memory wrapper
