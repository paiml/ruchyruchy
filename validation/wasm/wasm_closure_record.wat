;; WASM Closure Compilation Spike - Approach 1: Closure Records
;; Date: October 23, 2025
;; Purpose: Implement counter example using closure records in linear memory

(module
  ;; Memory for closure records and captured variables
  (memory (export "memory") 1)
  
  ;; Global heap pointer for memory allocation
  (global $heap_ptr (mut i32) (i32.const 0))
  
  ;; Function type for closures
  (type $closure_type (func (param i32) (param i32) (result i32)))
  
  ;; Function table for indirect calls
  (table 1 funcref)
  (elem (i32.const 0) $counter_impl)
  
  ;; Simple memory allocator
  (func $malloc (export "malloc") (param $size i32) (result i32)
    (local $addr i32)
    ;; Get current heap pointer
    (local.set $addr (global.get $heap_ptr))
    ;; Advance heap pointer
    (global.set $heap_ptr 
      (i32.add (global.get $heap_ptr) (local.get $size)))
    ;; Return allocated address
    (local.get $addr))
  
  ;; Create a counter - returns closure record pointer
  (func $make_counter (export "make_counter") (result i32)
    (local $closure_addr i32)
    (local $count_addr i32)
    
    ;; Allocate closure record: [func_idx, count_addr]
    (local.set $closure_addr (call $malloc (i32.const 8)))
    
    ;; Store function index (0 = counter_impl in table)
    (i32.store (local.get $closure_addr) (i32.const 0))
    
    ;; Allocate memory for count variable
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
  
  ;; Invoke a closure
  (func $call_closure (export "call_closure") (param $closure_ptr i32) (param $x i32) (result i32)
    (local $func_idx i32)
    
    ;; Load function index from closure record
    (local.set $func_idx (i32.load (local.get $closure_ptr)))
    
    ;; Call function with closure pointer and argument
    (call_indirect (type $closure_type)
      (local.get $closure_ptr)
      (local.get $x)
      (local.get $func_idx)))
  
  ;; Test function - creates two counters and invokes them
  (func $test (export "test") (result i32)
    (local $counter1 i32)
    (local $counter2 i32)
    (local $result1 i32)
    (local $result2 i32)
    (local $result3 i32)
    
    ;; Create two counters
    (local.set $counter1 (call $make_counter))
    (local.set $counter2 (call $make_counter))
    
    ;; Test counter1
    (local.set $result1 (call $call_closure (local.get $counter1) (i32.const 5)))
    (local.set $result2 (call $call_closure (local.get $counter1) (i32.const 3)))
    
    ;; Test counter2
    (local.set $result3 (call $call_closure (local.get $counter2) (i32.const 10)))
    
    ;; Verify results
    ;; result1 should be 5
    ;; result2 should be 8
    ;; result3 should be 10
    
    ;; Return 1 if all tests pass
    (i32.and
      (i32.and
        (i32.eq (local.get $result1) (i32.const 5))
        (i32.eq (local.get $result2) (i32.const 8)))
      (i32.eq (local.get $result3) (i32.const 10))))
  
  ;; Main entry point
  (func (export "_start") (result i32)
    (call $test))
)