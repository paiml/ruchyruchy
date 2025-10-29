//! eBPF Syscall Tracing (DEBUGGER-015)
//!
//! **Status**: GREEN Phase - Minimal implementation
//!
//! This module provides low-overhead syscall tracing using eBPF via the Aya framework.
//!
//! # Architecture
//!
//! 1. **eBPF Programs** (kernel space) - In `ruchyruchy-ebpf` crate
//!    - Attach to `raw_syscalls:sys_enter/exit` tracepoints
//!    - Write events to ring buffer
//!
//! 2. **Userspace Reader** (this module)
//!    - Load eBPF programs
//!    - Read events from ring buffer
//!    - Decode syscall arguments
//!    - Correlate with function traces (DEBUGGER-014)
//!
//! # Usage
//!
//! ```no_run
//! use ruchyruchy::tracing::ebpf::SyscallTracer;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Load and attach eBPF programs
//! let mut tracer = SyscallTracer::new().await?;
//!
//! // Read syscall events
//! let events = tracer.read_events().await?;
//! for event in events {
//!     println!("PID {} syscall {} at {}ns",
//!         event.pid, event.syscall_nr, event.timestamp_ns);
//! }
//! # Ok(())
//! # }
//! ```

pub mod syscall_reader;

pub use syscall_reader::{SyscallTracer, SyscallEvent, EbpfError};
