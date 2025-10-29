//! Zero-Cost Compiler Instrumentation Infrastructure
//!
//! This module provides the tracing infrastructure for DEBUGGER-014.
//! It implements per-thread trace buffers, event structures, and JSON output.

pub mod events;
pub mod buffer;
pub mod output;

pub use events::{TraceEvent, FunctionEntry, FunctionExit, SourceLocation, TypedValue, TypeInfo};
pub use buffer::TraceBuffer;
pub use output::{JsonFormatter, TraceFile, TraceMetadata, TraceStats};
