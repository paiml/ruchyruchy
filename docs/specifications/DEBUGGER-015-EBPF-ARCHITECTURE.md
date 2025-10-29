# DEBUGGER-015: eBPF Syscall Tracing Architecture

**Date**: 2025-10-29
**Status**: RED Phase - Architecture Design
**Dependencies**: DEBUGGER-014 (Zero-Cost Compiler Instrumentation)

## Executive Summary

DEBUGGER-015 adds low-overhead syscall tracing using **eBPF** (not ptrace) to complement the function-level tracing from DEBUGGER-014. This provides complete visibility into Ruchy program execution from userspace functions down to kernel syscalls.

**Key Decision**: Use **Aya** (pure Rust eBPF library) instead of BCC.

## Goals

1. **<1% overhead** - eBPF provides kernel-level tracing with minimal overhead
2. **50+ common syscalls** - Decode arguments for file, network, process syscalls
3. **Correlation with functions** - Link syscalls to calling functions
4. **strace compatibility** - Output format compatible with strace
5. **JSON output** - Machine-readable format for analysis

## Why eBPF (Not ptrace)

| Approach | Overhead | Production-Safe | Pros | Cons |
|----------|----------|-----------------|------|------|
| **ptrace** | 2-5x | ❌ No | Simple API | Process stops on every syscall |
| **eBPF** | <1% | ✅ Yes | No stops, kernel aggregation | Complex setup |

**Decision**: Use eBPF via Aya framework.

**Reference**: Gregg, B. (2019). "BPF Performance Tools" - eBPF has orders of magnitude less overhead than ptrace.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                        Ruchy Program                             │
│  (Instrumented with DEBUGGER-014 function tracing)              │
└────────────────┬────────────────────────────────────────────────┘
                 │
                 │ Function calls
                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Syscall Boundary                            │
└─────────────────┬──────────────┬────────────────────────────────┘
                  │              │
                  │              │ eBPF Tracepoints
                  ▼              ▼
         ┌────────────────────────────────┐
         │   Kernel: raw_syscalls         │
         │   - sys_enter (before syscall) │
         │   - sys_exit (after syscall)   │
         └────────────┬───────────────────┘
                      │
                      │ eBPF Program (Rust + Aya)
                      ▼
         ┌────────────────────────────────┐
         │   eBPF Maps (Ring Buffer)      │
         │   - Syscall events             │
         │   - Arguments (decoded)        │
         │   - Return values              │
         │   - Timestamps, PID/TID        │
         └────────────┬───────────────────┘
                      │
                      │ User-space read
                      ▼
         ┌────────────────────────────────┐
         │   RuchyRuchy Userspace         │
         │   - Read eBPF events           │
         │   - Decode syscall arguments   │
         │   - Correlate with functions   │
         │   - Format output (JSON/text)  │
         └────────────────────────────────┘
```

## Technology Stack

### Aya (Pure Rust eBPF)

**Why Aya over BCC**:
- ✅ Pure Rust (no C/clang dependency)
- ✅ Type-safe (kernel and userspace share Rust types)
- ✅ Modern (actively maintained, 2025 best practice)
- ✅ BTF support (compile once, run everywhere)
- ✅ Developer experience (Rust ecosystem)

**BCC drawbacks**:
- ❌ Requires C/clang toolchain
- ❌ Python bindings for Rust (awkward)
- ❌ Older approach (pre-2020 era)

### Components

#### 1. eBPF Programs (Kernel Space)

**File**: `src/tracing/ebpf/syscall_tracer.bpf.rs`

```rust
// eBPF program that runs in kernel
#![no_std]
#![no_main]

use aya_bpf::{
    macros::{map, tracepoint},
    maps::RingBuf,
    programs::TracePointContext,
};

// Event structure (shared between kernel and userspace)
#[repr(C)]
pub struct SyscallEvent {
    pub syscall_nr: u64,
    pub pid: u32,
    pub tid: u32,
    pub timestamp_ns: u64,
    pub args: [u64; 6],  // Raw syscall arguments
    pub ret: i64,        // Return value (only in sys_exit)
}

// Ring buffer to send events to userspace
#[map]
static EVENTS: RingBuf = RingBuf::with_byte_size(256 * 1024, 0);

// Attach to syscall entry tracepoint
#[tracepoint]
pub fn sys_enter(ctx: TracePointContext) -> i32 {
    // Read syscall number, arguments, PID/TID
    // Write event to ring buffer
    0
}

// Attach to syscall exit tracepoint
#[tracepoint]
pub fn sys_exit(ctx: TracePointContext) -> i32 {
    // Read syscall return value
    // Write event to ring buffer
    0
}
```

#### 2. Userspace Event Reader

**File**: `src/tracing/ebpf/syscall_reader.rs`

```rust
use aya::{Bpf, programs::TracePoint, maps::RingBuf};

pub struct SyscallTracer {
    bpf: Bpf,
    ring_buf: RingBuf<SyscallEvent>,
}

impl SyscallTracer {
    pub fn new() -> Result<Self, Error> {
        // Load eBPF program
        let mut bpf = Bpf::load(include_bytes_aligned!("syscall_tracer.bpf.o"))?;

        // Attach to tracepoints
        let enter: &mut TracePoint = bpf
            .program_mut("sys_enter")?
            .try_into()?;
        enter.load()?;
        enter.attach("raw_syscalls", "sys_enter")?;

        let exit: &mut TracePoint = bpf
            .program_mut("sys_exit")?
            .try_into()?;
        exit.load()?;
        exit.attach("raw_syscalls", "sys_exit")?;

        // Get ring buffer
        let ring_buf = RingBuf::try_from(bpf.map_mut("EVENTS")?)?;

        Ok(Self { bpf, ring_buf })
    }

    pub fn read_events(&mut self) -> impl Iterator<Item = SyscallEvent> {
        self.ring_buf.read()
    }
}
```

#### 3. Syscall Decoder

**File**: `src/tracing/ebpf/syscall_decoder.rs`

```rust
use crate::tracing::events::SyscallEvent as TraceEvent;

pub struct SyscallDecoder;

impl SyscallDecoder {
    pub fn decode(raw: &RawSyscallEvent) -> TraceEvent {
        match raw.syscall_nr {
            // x86_64 syscall numbers
            0 => decode_read(raw),     // read(fd, buf, count)
            1 => decode_write(raw),    // write(fd, buf, count)
            2 => decode_open(raw),     // open(pathname, flags)
            3 => decode_close(raw),    // close(fd)
            4 => decode_stat(raw),     // stat(pathname, statbuf)
            // ... 50+ more syscalls
            _ => decode_unknown(raw),
        }
    }
}

fn decode_open(raw: &RawSyscallEvent) -> TraceEvent {
    // arg[0] = pathname (char* in kernel)
    // arg[1] = flags (int)
    // arg[2] = mode (mode_t, optional)

    let pathname = read_string_from_kernel(raw.args[0])?;
    let flags = decode_open_flags(raw.args[1]);

    TraceEvent::Syscall(SyscallEvent {
        number: raw.syscall_nr,
        name: "open".to_string(),
        args: vec![
            json!(pathname),
            json!(flags),
        ],
        return_value: raw.ret,
        duration_ns: /* calculate from enter/exit */,
        timestamp_ns: raw.timestamp_ns,
        thread_id: raw.tid,
    })
}
```

**Challenge**: Reading strings from kernel memory is restricted in eBPF.

**Solution**: Use `bpf_probe_read_kernel_str()` helper or read at syscall entry.

#### 4. Correlation Engine

**File**: `src/tracing/ebpf/correlator.rs`

```rust
use crate::tracing::events::TraceEvent;

pub struct Correlator {
    function_events: Vec<TraceEvent>,  // From DEBUGGER-014
    syscall_events: Vec<TraceEvent>,   // From DEBUGGER-015
}

impl Correlator {
    pub fn merge(&self) -> Vec<TraceEvent> {
        // Merge function and syscall events by timestamp
        // Add parent_function field to syscalls
        // Ensure chronological ordering

        let mut merged = Vec::new();

        // Algorithm:
        // 1. Sort both lists by timestamp
        // 2. Track current function stack
        // 3. When syscall occurs, link to top-of-stack function
        // 4. Interleave events in chronological order

        merged
    }
}
```

**Output Example**:
```json
{
  "type": "function_enter",
  "name": "write_file",
  "timestamp_ns": 1000000
},
{
  "type": "syscall",
  "name": "open",
  "args": ["/tmp/file.txt", "O_WRONLY|O_CREAT"],
  "parent_function": "write_file",
  "timestamp_ns": 1000100
},
{
  "type": "syscall",
  "name": "write",
  "args": [3, "Hello", 5],
  "parent_function": "write_file",
  "timestamp_ns": 1000200
},
{
  "type": "function_exit",
  "name": "write_file",
  "timestamp_ns": 1000300
}
```

## Syscall Coverage

### Phase 1: File Operations (20 syscalls)
- `open`, `openat`, `close`
- `read`, `write`, `pread`, `pwrite`
- `stat`, `fstat`, `lstat`
- `access`, `chmod`, `chown`
- `mkdir`, `rmdir`, `unlink`
- `rename`, `link`, `symlink`

### Phase 2: Process Operations (15 syscalls)
- `fork`, `vfork`, `clone`
- `execve`, `exit`, `wait4`
- `getpid`, `getppid`, `gettid`
- `kill`, `signal`, `sigaction`

### Phase 3: Network Operations (15 syscalls)
- `socket`, `bind`, `listen`, `accept`
- `connect`, `send`, `recv`
- `sendto`, `recvfrom`
- `setsockopt`, `getsockopt`

**Total**: 50+ common syscalls

## Performance Targets

### Overhead (Honest Estimates)

| Workload | Without eBPF | With eBPF | Overhead |
|----------|--------------|-----------|----------|
| Syscall-heavy (10K syscalls/sec) | 100ms | 100.9ms | <1% |
| Syscall-light (100 syscalls/sec) | 100ms | 100.01ms | <0.01% |
| CPU-bound (no syscalls) | 100ms | 100ms | 0% |

**Reference**: Gregg (2019) - eBPF overhead is typically <1% even for syscall-heavy workloads.

### Event Rate

- **Capture**: 100K+ syscalls/second
- **Processing**: 50K+ events/second in userspace
- **Output**: 10K+ events/second to JSON

**Bottleneck**: Userspace processing, not eBPF capture.

## Output Formats

### strace-Compatible Text

```
open("/tmp/file.txt", O_WRONLY|O_CREAT|O_TRUNC, 0666) = 3
write(3, "Hello, eBPF!", 12) = 12
close(3) = 0
```

### JSON (Machine-Readable)

```json
{
  "type": "syscall",
  "name": "open",
  "args": [
    "/tmp/file.txt",
    "O_WRONLY|O_CREAT|O_TRUNC",
    "0666"
  ],
  "return_value": 3,
  "duration_ns": 15000,
  "timestamp_ns": 1761761183405413218,
  "pid": 12345,
  "tid": 12345
}
```

## Implementation Roadmap

### Phase 1: Proof of Concept (2 weeks)
- [ ] Setup Aya development environment
- [ ] Write minimal eBPF program (capture syscall number only)
- [ ] Attach to raw_syscalls:sys_enter/exit
- [ ] Read events in userspace
- [ ] Verify <1% overhead

### Phase 2: Syscall Decoding (3 weeks)
- [ ] Implement decoder for 20 file syscalls
- [ ] Read arguments from kernel memory (strings, buffers)
- [ ] Decode flags/modes (O_RDONLY, etc.)
- [ ] Handle 64-bit/32-bit architecture differences

### Phase 3: Correlation (2 weeks)
- [ ] Merge function events (DEBUGGER-014) with syscall events
- [ ] Link syscalls to parent functions
- [ ] Ensure chronological ordering
- [ ] Handle multi-threading

### Phase 4: Output & Polish (1 week)
- [ ] strace-compatible text formatter
- [ ] JSON output with full metadata
- [ ] Filtering by syscall pattern
- [ ] Documentation and examples

**Total**: 8 weeks (2 months)

## Challenges & Solutions

### Challenge 1: Reading Kernel Strings

**Problem**: eBPF cannot directly dereference kernel pointers.

**Solution**: Use `bpf_probe_read_kernel_str()` helper function.

**Example**:
```rust
let mut pathname = [0u8; 256];
unsafe {
    bpf_probe_read_kernel_str(
        pathname.as_mut_ptr(),
        pathname.len() as u32,
        raw.args[0] as *const u8,
    )?;
}
```

### Challenge 2: BTF (BPF Type Format)

**Problem**: Different kernel versions have different syscall ABIs.

**Solution**: Use BTF for portable access to kernel structures.

**Reference**: Aya has built-in BTF support for "compile once, run everywhere".

### Challenge 3: Ring Buffer Overflow

**Problem**: High syscall rate can overflow ring buffer.

**Solution**:
1. Use 256KB ring buffer (default)
2. Drop events if buffer full (track with counter)
3. Add filtering to reduce event rate

### Challenge 4: Syscall Arguments are Arch-Specific

**Problem**: x86_64 vs ARM64 have different calling conventions.

**Solution**: Use Aya's architecture-agnostic tracepoint API (it handles this).

## Testing Strategy

### RED Phase Tests (8 tests created)

1. `test_ebpf_syscall_capture` - Capture basic syscalls
2. `test_syscall_decoding` - Decode 50+ syscall arguments
3. `test_correlation_with_functions` - Link to functions
4. `test_overhead_under_1_percent` - Performance requirement
5. `test_strace_compatible_output` - Format compatibility
6. `test_json_output_format` - Machine-readable output
7. `test_filtering_by_syscall_pattern` - Selective tracing

### Integration Tests

- Run against real Ruchy programs
- Compare with strace output (differential testing)
- Benchmark overhead with syscall-heavy workloads

### Compatibility Tests

- Test on multiple kernel versions (5.10+)
- Test on x86_64 and ARM64
- Test with different Ruchy versions

## Dependencies

### Rust Crates

```toml
[dependencies]
aya = "0.12"                # Pure Rust eBPF
aya-log = "0.2"            # eBPF logging
tokio = { version = "1", features = ["full"] }  # Async runtime

[build-dependencies]
aya-bpf-build = "0.1"      # Build eBPF programs
```

### System Requirements

- Linux kernel 5.10+ (for BTF support)
- `bpftool` (for debugging)
- `llvm` (for eBPF compilation)

**Note**: Aya handles most of this automatically.

## References

1. **Aya Documentation**: https://aya-rs.dev/
2. **Gregg, B. (2019)**. "BPF Performance Tools: Linux System and Application Observability". Addison-Wesley.
3. **Tutorial (2025)**: "Track Linux Syscalls with Rust and eBPF" - https://diobr4nd0.github.io/2025/06/21/Track-Linux-Syscalls-with-Rust-and-eBPF/
4. **Nakamura, Y. (2024)**. "Writing eBPF Tracepoint Program with Rust Aya" - https://yuki-nakamura.com/2024/07/06/writing-ebpf-tracepoint-program-with-rust-aya-tips-and-example/

## Next Steps

1. ✅ RED Phase: Architecture documented
2. ⏳ Update main specification with eBPF details
3. ⏳ Setup Aya development environment
4. ⏳ Create minimal eBPF program (GREEN phase)

## Appendix: Aya vs BCC Comparison

| Feature | Aya (2025) | BCC (Legacy) |
|---------|-----------|--------------|
| Language | Pure Rust | C + Python bindings |
| Type Safety | ✅ Full | ❌ Limited |
| BTF Support | ✅ Native | ⚠️ Partial |
| Compile Once | ✅ Yes | ❌ No (needs kernel headers) |
| Developer UX | ✅ Excellent (Rust ecosystem) | ⚠️ Mixed (C macros) |
| Maintenance | ✅ Active (2025) | ⚠️ Stable but older |
| Documentation | ✅ Modern tutorials | ✅ Extensive but dated |

**Decision**: Use Aya for pure Rust development and better developer experience.
