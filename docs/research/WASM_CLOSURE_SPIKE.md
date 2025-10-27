# WASM Closure Compilation Spike

**Date**: October 23, 2025  
**Status**: In Progress  
**Purpose**: Validate feasibility of closure compilation for WASM target  
**Timeline**: 1-2 days (time-boxed investigation)  

## Objectives

This spike aims to validate the most challenging aspect of WASM compilation: implementing closures with captured variables. The findings will inform the full WASM implementation strategy and timeline.

Key questions to answer:
1. Can we effectively implement closures with captured variables in WASM?
2. Which approach is most effective: closure records or global variables?
3. What is the memory management strategy for closures?
4. How complex is the implementation, and what's the realistic timeline?

## Test Case

We'll implement compilation for the following Ruchy program:

```ruchy
fun make_counter() {
    let mut count = 0
    (x) => {
        count = count + x
        count
    }
}

let counter1 = make_counter()
let counter2 = make_counter()
let result1 = counter1(5)  // Should return 5
let result2 = counter1(3)  // Should return 8
let result3 = counter2(10) // Should return 10 (independent counter)
```

This test case verifies:
- Closure creation
- Variable capture
- Mutable variable updates
- Multiple independent closures from same factory

## Implementation Approaches

### Approach 1: Closure Records in Linear Memory

This approach stores closure data (function pointer + captured variables) in WASM linear memory:

```wat
;; Memory allocation
(memory 1)
(global $heap_ptr (mut i32) (i32.const 0))

;; Allocate memory helper
(func $malloc (param $size i32) (result i32)
  (local $addr i32)
  (local.set $addr (global.get $heap_ptr))
  (global.set $heap_ptr
    (i32.add (global.get $heap_ptr) (local.get $size)))
  (local.get $addr))

;; Create closure record
(func $make_counter (result i32)
  (local $closure_addr i32)
  (local $count_addr i32)
  
  ;; Allocate closure record: [func_idx, count_addr]
  (local.set $closure_addr (call $malloc (i32.const 8)))
  
  ;; Store function index (0 = closure implementation)
  (i32.store (local.get $closure_addr) (i32.const 0))
  
  ;; Allocate memory for count
  (local.set $count_addr (call $malloc (i32.const 4)))
  
  ;; Initialize count to 0
  (i32.store (local.get $count_addr) (i32.const 0))
  
  ;; Store count address in closure record
  (i32.store
    (i32.add (local.get $closure_addr) (i32.const 4))
    (local.get $count_addr))
  
  ;; Return closure record address
  (local.get $closure_addr))

;; Counter implementation
(func $counter_impl (param $closure_ptr i32) (param $x i32) (result i32)
  (local $count_addr i32)
  (local $current i32)
  
  ;; Load count address from closure record
  (local.set $count_addr
    (i32.load (i32.add (local.get $closure_ptr) (i32.const 4))))
  
  ;; Load current count value
  (local.set $current (i32.load (local.get $count_addr)))
  
  ;; Add x to count
  (local.set $current (i32.add (local.get $current) (local.get $x)))
  
  ;; Store updated count
  (i32.store (local.get $count_addr) (local.get $current))
  
  ;; Return new count
  (local.get $current))

;; Function table for indirect calls
(table 1 funcref)
(elem (i32.const 0) $counter_impl)

;; Invoke closure
(func $call_closure (param $closure_ptr i32) (param $x i32) (result i32)
  (local $func_idx i32)
  
  ;; Load function index from closure record
  (local.set $func_idx (i32.load (local.get $closure_ptr)))
  
  ;; Call function with closure pointer and argument
  (call_indirect (type $closure_type)
    (local.get $closure_ptr)
    (local.get $x)
    (local.get $func_idx)))

;; Type definition for closures
(type $closure_type (func (param i32) (param i32) (result i32)))
```

### Approach 2: Global Variables (Simplified)

This approach uses global variables for captured state:

```wat
;; Global counter states
(global $counter1_val (mut i32) (i32.const 0))
(global $counter2_val (mut i32) (i32.const 0))
(global $next_counter_id (mut i32) (i32.const 0))

;; Create counter (returns counter ID)
(func $make_counter (result i32)
  (local $id i32)
  (local.set $id (global.get $next_counter_id))
  (global.set $next_counter_id 
    (i32.add (global.get $next_counter_id) (i32.const 1)))
  (local.get $id))

;; Call counter with ID
(func $call_counter (param $id i32) (param $x i32) (result i32)
  (local $current i32)
  
  (if (result i32)
    (i32.eq (local.get $id) (i32.const 0))
    (then
      ;; Counter 1
      (local.set $current (global.get $counter1_val))
      (local.set $current (i32.add (local.get $current) (local.get $x)))
      (global.set $counter1_val (local.get $current))
      (local.get $current))
    (else
      ;; Counter 2
      (local.set $current (global.get $counter2_val))
      (local.set $current (i32.add (local.get $current) (local.get $x)))
      (global.set $counter2_val (local.get $current))
      (local.get $current))))
```

## Implementation Plan

1. **Setup** (1 hour)
   - Create test harness
   - Set up WAT compilation environment

2. **Implement Approach 1** (4-6 hours)
   - Implement closure records in linear memory
   - Implement function table
   - Test with counter example

3. **Implement Approach 2** (2-3 hours)
   - Implement global variable approach
   - Test with counter example

4. **Compare Approaches** (2-3 hours)
   - Measure performance
   - Evaluate complexity
   - Assess maintainability and scalability

5. **Document Findings** (2-3 hours)
   - Record implementation details
   - Document challenges and solutions
   - Recommend approach for full implementation
   - Update timeline estimates

## Success Criteria

The spike is successful if we can:
1. Implement working closures with captured variables in WASM
2. Verify correct behavior with multiple independent closures
3. Document a clear approach for the full implementation
4. Provide accurate timeline estimates for WASM-002 (Closure Compilation)

## Next Steps

After completing the spike:
1. Update `WASM_COMPILATION_PLAN.md` with findings
2. Begin implementation of basic WASM emitter (WASM-001)
3. Set up testing infrastructure for WASM compilation
4. Implement closure compilation based on spike findings (WASM-002)

## Timeline

- Start: October 23, 2025
- Complete: October 24-25, 2025 (1-2 days)
- Documentation update: October 25, 2025