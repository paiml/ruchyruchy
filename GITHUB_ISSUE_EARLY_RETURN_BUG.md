# Ruchy Bug Report: Early Return Statement Not Working

**Ruchy Version**: v3.106.0
**Project**: RuchyRuchy Bootstrap Compiler
**Ticket**: DEBUGGER-001 PROPERTY Phase
**Discovered By**: Property-based testing

## Summary

Early `return` statements inside `if` blocks are not working correctly. The function continues executing after the `return` statement, leading to incorrect return values.

## Reproduction Steps

1. Create a function with an early return in an if block
2. Call the function with a condition that should trigger the early return
3. Observe that the function continues executing after the return statement

## Minimal Reproduction Code

```ruchy
struct DAPServer { port: i32, is_running: bool, is_initialized: bool }

fn dap_server_new(port: i32) -> DAPServer {
    DAPServer { port: port, is_running: false, is_initialized: false }
}

fn dap_server_accept_connection(server: DAPServer) -> bool {
    println("  Condition !is_running = {}", !server.is_running)

    if !server.is_running {
        println("  Branch: Taking FALSE path")
        return false  // ← This return is NOT working!
    }
    println("  Branch: Taking TRUE path")
    true
}

fn main() {
    let server = dap_server_new(4711)
    let result = dap_server_accept_connection(server)
    println("Result: {} (expected: false)", result)
}

main()
```

## Expected Behavior

When `server.is_running` is `false`:
1. Condition `!server.is_running` evaluates to `true`
2. Enter the if block
3. Print "Branch: Taking FALSE path"
4. Execute `return false`
5. **Function should return false and stop executing**
6. "Branch: Taking TRUE path" should NOT print
7. Final result should be `false`

## Actual Behavior

```
  Condition !is_running = true
  Branch: Taking FALSE path
  Branch: Taking TRUE path   ← BOTH branches execute!
Result: true (expected: false)
```

**Both branches execute!** The `return false` statement does not stop function execution. The function continues, executes the TRUE branch, and returns `true` instead of `false`.

## Full Error Output

```
Testing accept_connection logic:

Created server with is_running = false

  Input: is_running = false
  Condition !is_running = true
  Branch: Taking FALSE path     ← Returns false here
  Branch: Taking TRUE path       ← But continues execution!

Result: true                    ← Returns true (WRONG!)
Expected: false

❌ BUG CONFIRMED: Function returned true when should return false!
```

## Context

Discovered while implementing property-based tests for DAP server skeleton (DEBUGGER-001). Property test expected `accept_connection()` to return `false` when server is not running, but it returns `true` due to this bug.

This is a **critical correctness bug** that affects control flow in Ruchy programs.

## Impact

- **Blocks**: DEBUGGER-001 PROPERTY phase
- **Severity**: CRITICAL - incorrect return values, control flow broken
- **Scope**: Any function using early returns in if blocks

## Workaround

Use if-else instead of early return:

**Broken Code** (early return):
```ruchy
fn dap_server_accept_connection(server: DAPServer) -> bool {
    if !server.is_running {
        return false  // ← Doesn't work
    }
    true
}
```

**Working Code** (if-else):
```ruchy
fn dap_server_accept_connection(server: DAPServer) -> bool {
    if !server.is_running {
        false  // ← Use expression, not early return
    } else {
        true
    }
}
```

## Environment

- **OS**: Linux
- **Ruchy Version**: v3.106.0
- **Install**: Cargo
- **Discovery Method**: Property-based testing with 600+ test cases

## Additional Notes

This bug was discovered through systematic property-based testing, which tested the postcondition: `accept_connection(s) = true → is_running(s)`. The property test revealed that `accept_connection` was returning `true` even when `is_running` was `false`, leading to this investigation.

This demonstrates the value of property-based testing for discovering compiler bugs!

## Related

- Project: https://github.com/paiml/ruchyruchy
- Issue #1: Parser Debugging Tools
- Similar to Issue #40 (control flow bugs)
