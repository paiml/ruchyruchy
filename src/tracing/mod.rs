//! Zero-Cost Compiler Instrumentation Infrastructure
//!
//! This module provides the tracing infrastructure for DEBUGGER-014 and DEBUGGER-015.
//!
//! # Modules
//!
//! ## DEBUGGER-014: Compiler Instrumentation (Complete ✅)
//! - `events` - Trace event structures (function entry/exit, syscalls)
//! - `buffer` - Per-thread lock-free buffers
//! - `output` - JSON and strace-style formatters
//!
//! ## DEBUGGER-015: eBPF Syscall Tracing (GREEN Phase ⏳)
//! - `ebpf_placeholder` - eBPF syscall tracing (requires setup)
//!
//! See `docs/setup/EBPF_DEVELOPMENT_SETUP.md` for eBPF setup instructions.

pub mod events;
pub mod buffer;
pub mod output;

/// eBPF syscall tracing (DEBUGGER-015)
///
/// **Note**: Requires eBPF development setup and Linux 5.10+. Enable with `--features ebpf`.
#[cfg(feature = "ebpf")]
pub mod ebpf;

/// eBPF placeholder (when feature is disabled)
///
/// **Note**: Requires eBPF development setup. See `docs/setup/EBPF_DEVELOPMENT_SETUP.md`
#[cfg(not(feature = "ebpf"))]
pub mod ebpf_placeholder;

pub use events::{TraceEvent, FunctionEntry, FunctionExit, SourceLocation, TypedValue, TypeInfo};
pub use buffer::TraceBuffer;
pub use output::{JsonFormatter, TraceFile, TraceMetadata, TraceStats};
