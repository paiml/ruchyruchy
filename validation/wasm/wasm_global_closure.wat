;; WASM Closure Compilation Spike - Approach 2: Global Variables
;; Date: October 23, 2025
;; Purpose: Implement counter example using global variables for state

(module
  ;; Global counter states
  (global $counter1_val (mut i32) (i32.const 0))
  (global $counter2_val (mut i32) (i32.const 0))
  (global $next_counter_id (mut i32) (i32.const 0))
  
  ;; Create counter - returns counter ID
  (func $make_counter (export "make_counter") (result i32)
    (local $id i32)
    
    ;; Get next counter ID
    (local.set $id (global.get $next_counter_id))
    
    ;; Increment counter ID for next call
    (global.set $next_counter_id 
      (i32.add (global.get $next_counter_id) (i32.const 1)))
    
    ;; Return counter ID
    (local.get $id))
  
  ;; Call counter with ID
  (func $call_counter (export "call_counter") (param $id i32) (param $x i32) (result i32)
    (local $current i32)
    
    ;; Choose counter based on ID
    (if (result i32)
      (i32.eq (local.get $id) (i32.const 0))
      (then
        ;; Counter 1
        ;; Get current value
        (local.set $current (global.get $counter1_val))
        
        ;; Add x to value
        (local.set $current (i32.add (local.get $current) (local.get $x)))
        
        ;; Update global
        (global.set $counter1_val (local.get $current))
        
        ;; Return new value
        (local.get $current))
      (else
        ;; Counter 2
        ;; Get current value
        (local.set $current (global.get $counter2_val))
        
        ;; Add x to value
        (local.set $current (i32.add (local.get $current) (local.get $x)))
        
        ;; Update global
        (global.set $counter2_val (local.get $current))
        
        ;; Return new value
        (local.get $current))))
  
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
    (local.set $result1 (call $call_counter (local.get $counter1) (i32.const 5)))
    (local.set $result2 (call $call_counter (local.get $counter1) (i32.const 3)))
    
    ;; Test counter2
    (local.set $result3 (call $call_counter (local.get $counter2) (i32.const 10)))
    
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