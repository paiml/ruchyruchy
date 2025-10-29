//! Trace Event Structures
//!
//! Defines the event types that can be traced during program execution.

use serde::{Serialize, Deserialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Source code location for an event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    /// Source file path
    pub file: String,
    /// Line number (1-indexed)
    pub line: u32,
    /// Column number (1-indexed)
    pub column: u32,
}

/// Type information for traced values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeInfo {
    /// Type name (e.g., "i64", "String", "User")
    pub name: String,
    /// Optional field information for structs
    pub fields: Option<Vec<(String, String)>>,
}

/// Traced value with type information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedValue {
    /// Type information
    pub type_info: TypeInfo,
    /// JSON-serialized value
    pub value: serde_json::Value,
}

/// Function entry event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionEntry {
    /// Function name
    pub name: String,
    /// Function arguments (with type information)
    pub args: Vec<TypedValue>,
    /// Source location
    pub location: SourceLocation,
    /// Timestamp (nanoseconds since UNIX epoch)
    pub timestamp_ns: u64,
    /// Thread ID
    pub thread_id: u64,
}

/// Function exit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionExit {
    /// Function name
    pub name: String,
    /// Return value (with type information)
    pub return_value: Option<TypedValue>,
    /// Duration (nanoseconds)
    pub duration_ns: u64,
    /// Timestamp (nanoseconds since UNIX epoch)
    pub timestamp_ns: u64,
    /// Thread ID
    pub thread_id: u64,
}

/// System call event (for future eBPF integration)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyscallEvent {
    /// Syscall number
    pub number: u64,
    /// Syscall name (e.g., "open", "read")
    pub name: String,
    /// Arguments (decoded)
    pub args: Vec<serde_json::Value>,
    /// Return value
    pub return_value: i64,
    /// Duration (nanoseconds)
    pub duration_ns: u64,
    /// Timestamp (nanoseconds since UNIX epoch)
    pub timestamp_ns: u64,
    /// Thread ID
    pub thread_id: u64,
}

/// Trace event (union of all event types)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TraceEvent {
    /// Function entry
    #[serde(rename = "function_enter")]
    FunctionEnter(FunctionEntry),

    /// Function exit
    #[serde(rename = "function_exit")]
    FunctionExit(FunctionExit),

    /// System call (future)
    #[serde(rename = "syscall")]
    Syscall(SyscallEvent),
}

impl TraceEvent {
    /// Get timestamp for this event
    pub fn timestamp_ns(&self) -> u64 {
        match self {
            TraceEvent::FunctionEnter(e) => e.timestamp_ns,
            TraceEvent::FunctionExit(e) => e.timestamp_ns,
            TraceEvent::Syscall(e) => e.timestamp_ns,
        }
    }

    /// Get thread ID for this event
    pub fn thread_id(&self) -> u64 {
        match self {
            TraceEvent::FunctionEnter(e) => e.thread_id,
            TraceEvent::FunctionExit(e) => e.thread_id,
            TraceEvent::Syscall(e) => e.thread_id,
        }
    }
}

/// Get current timestamp in nanoseconds since UNIX epoch
pub fn current_timestamp_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_nanos() as u64
}

/// Get current thread ID (using debug format as stable workaround)
pub fn current_thread_id() -> u64 {
    // Use debug format of ThreadId and hash it to get a u64
    // This is stable but may have collisions (acceptable for tracing)
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let thread_id = std::thread::current().id();
    let mut hasher = DefaultHasher::new();
    format!("{:?}", thread_id).hash(&mut hasher);
    hasher.finish()
}

/// Helper to create function entry event
pub fn function_enter(
    name: &str,
    args: Vec<TypedValue>,
    location: SourceLocation,
) -> TraceEvent {
    TraceEvent::FunctionEnter(FunctionEntry {
        name: name.to_string(),
        args,
        location,
        timestamp_ns: current_timestamp_ns(),
        thread_id: current_thread_id(),
    })
}

/// Helper to create function exit event
pub fn function_exit(
    name: &str,
    return_value: Option<TypedValue>,
    duration_ns: u64,
) -> TraceEvent {
    TraceEvent::FunctionExit(FunctionExit {
        name: name.to_string(),
        return_value,
        duration_ns,
        timestamp_ns: current_timestamp_ns(),
        thread_id: current_thread_id(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_function_enter_event() {
        let loc = SourceLocation {
            file: "test.ruchy".to_string(),
            line: 10,
            column: 5,
        };

        let arg = TypedValue {
            type_info: TypeInfo {
                name: "i64".to_string(),
                fields: None,
            },
            value: serde_json::json!(42),
        };

        let event = function_enter("test_fn", vec![arg], loc);

        match event {
            TraceEvent::FunctionEnter(entry) => {
                assert_eq!(entry.name, "test_fn");
                assert_eq!(entry.args.len(), 1);
                assert_eq!(entry.location.file, "test.ruchy");
                assert_eq!(entry.location.line, 10);
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_create_function_exit_event() {
        let ret = TypedValue {
            type_info: TypeInfo {
                name: "i64".to_string(),
                fields: None,
            },
            value: serde_json::json!(100),
        };

        let event = function_exit("test_fn", Some(ret), 1000);

        match event {
            TraceEvent::FunctionExit(exit) => {
                assert_eq!(exit.name, "test_fn");
                assert!(exit.return_value.is_some());
                assert_eq!(exit.duration_ns, 1000);
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_timestamp_and_thread_id() {
        let event = function_enter(
            "test",
            vec![],
            SourceLocation {
                file: "test.ruchy".to_string(),
                line: 1,
                column: 1,
            },
        );

        assert!(event.timestamp_ns() > 0);
        assert!(event.thread_id() > 0);
    }
}
