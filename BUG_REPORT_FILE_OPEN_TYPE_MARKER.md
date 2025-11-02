# Bug Report: File.open() Returns Object Without __type Marker

**DRAFT - Ready to file at https://github.com/paiml/ruchy/issues**

## Bug Report: File.open() returns object without __type marker, breaks method dispatch

**Ruchy Version**: [output of `ruchy --version` - please fill in]
**Project**: RuchyRuchy Bootstrap Compiler / File I/O Feature Development
**Ticket**: N/A (discovered during File I/O implementation)

### Reproduction Steps
1. Create a test file with File.open() and subsequent method call:
```bash
cat > /tmp/test_file_open.ruchy <<'EOF'
fun main() {
    let file = File.open("/tmp/test.txt");
    let line = file.read_line();
    println(line);
}
EOF
```

2. Run with production Ruchy compiler:
```bash
ruchy run /tmp/test_file_open.ruchy
```

3. Observe error

### Minimal Reproduction Code
```ruchy
fun main() {
    let file = File.open("/tmp/test.txt");
    let line = file.read_line();  // Fails here
    println(line);
}
```

### Expected Behavior
- `File.open()` should return a File object with proper `__type` marker
- `file.read_line()` should successfully dispatch to the File type's read_line method
- Program should execute without errors

### Actual Behavior
```
Error: Evaluation error: Runtime error: Object is missing __type marker
```

### Full Error Output
```
Error: Evaluation error: Runtime error: Object is missing __type marker
```

**Error occurs during method dispatch on the returned File object.**

### Root Cause Analysis (via rust-gdb)

Using automated rust-gdb debugging (script: `/home/noah/src/ruchyruchy/scripts/debug-ruchy-auto.sh`):

**Findings**:
1. `File.open()` method dispatch **succeeds** (breakpoint at `dispatch_method_call` shows successful call)
2. The returned File object is **missing the `__type` marker** field
3. When `file.read_line()` attempts method dispatch, it fails due to missing `__type`

**Debug session output** (excerpt):
```
Breakpoint 1, ruchy::runtime::interpreter::Interpreter::dispatch_method_call
method = "open"
receiver type = "object"
[File.open() completes successfully]

...later...

Error: Evaluation error: Runtime error: Object is missing __type marker
[Inferior 1 (process 2220585) exited with code 01]
```

### Context
**What I was trying to accomplish**:
Implement File I/O operations for the Ruchy language, starting with basic file reading capabilities. Added `File` global object with `open()` method in `src/stdlib/builtin_functions.rs`.

### Impact
**Blocks**: File I/O feature development, any code using `File.open()` and subsequent method calls

**Severity**: HIGH - File I/O is a critical feature for real-world programs

**Workaround**: None currently - File objects cannot be used for method calls until `__type` marker is properly set

### Technical Details

**Expected Object Structure**:
```rust
// File object should contain:
{
    "__type": "File",  // ← Missing!
    "handle": <file_handle>,
    // ... other fields
}
```

**Actual Object Structure** (inferred):
```rust
// File object contains:
{
    // "__type": "File",  ← MISSING!
    "handle": <file_handle>,
    // ... other fields
}
```

**Method Dispatch Logic** (src/runtime/eval_method_dispatch.rs):
- Line ~4327: Checks for `__type` marker in object
- If missing: Returns error "Object is missing __type marker"
- This check prevents method dispatch from proceeding

### Suggested Fix

In the `File_open()` implementation (or wherever File objects are constructed):

**Before** (hypothetical):
```rust
// Construct File object without __type
let file_obj = create_object(fields);
```

**After** (suggested):
```rust
// Construct File object WITH __type marker
let mut fields = HashMap::new();
fields.insert("__type".to_string(), Value::String("File".into()));
fields.insert("handle".to_string(), Value::from(file_handle));
let file_obj = create_object(fields);
```

### Environment
- OS: Linux
- Ruchy install: [Cargo/Binary/etc - please specify]
- Rust version (for debugging): rustc 1.XX.X

### Debug Tooling Used

Created automated rust-gdb wrapper script to debug this issue:
- Script: `/home/noah/src/ruchyruchy/scripts/debug-ruchy-auto.sh`
- Breakpoints at `dispatch_method_call`
- Captured full backtrace, local variables, and method arguments
- Identified exact point of failure

This will be formalized as **DEBUGGER-048** ticket in RuchyRuchy project.

### Related Issues
- N/A (first File I/O bug discovered)

### Reproduction Success Rate
- 100% reproducible with the test case provided

---

**Note to Ruchy Team**: The `__type` marker appears to be critical for method dispatch on objects. All object types that support methods need this field. Consider:
1. Adding `__type` marker to all stdlib object constructors
2. Adding validation that objects have `__type` before returning from native functions
3. Documenting `__type` requirement in stdlib development guide
