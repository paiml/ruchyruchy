//! Trace Output Formatters
//!
//! Formats trace events for various output formats (JSON, text, Chrome Trace).

use super::events::TraceEvent;
use serde::{Serialize, Deserialize};
use std::io::Write;

/// Trace file metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct TraceMetadata {
    /// Program name
    pub program: String,
    /// Start time (ISO 8601)
    pub start_time: String,
    /// Ruchy version
    pub ruchy_version: String,
    /// RuchyRuchy version
    pub ruchyruchy_version: String,
}

/// Complete trace file format
#[derive(Debug, Serialize, Deserialize)]
pub struct TraceFile {
    /// Metadata
    pub metadata: TraceMetadata,
    /// All trace events
    pub events: Vec<TraceEvent>,
    /// Statistics
    pub stats: TraceStats,
}

/// Trace statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct TraceStats {
    /// Total events captured
    pub total_events: usize,
    /// Events dropped (buffer overflow)
    pub dropped_events: usize,
    /// Total duration (nanoseconds)
    pub duration_ns: u64,
}

/// JSON formatter
pub struct JsonFormatter {
    /// Pretty print (vs compact)
    pretty: bool,
}

impl JsonFormatter {
    /// Create new JSON formatter
    pub fn new(pretty: bool) -> Self {
        Self { pretty }
    }

    /// Format trace file to JSON
    pub fn format(&self, trace: &TraceFile) -> Result<String, serde_json::Error> {
        if self.pretty {
            serde_json::to_string_pretty(trace)
        } else {
            serde_json::to_string(trace)
        }
    }

    /// Write trace file to writer
    pub fn write<W: Write>(&self, trace: &TraceFile, writer: &mut W) -> std::io::Result<()> {
        let json = self.format(trace).map_err(std::io::Error::other)?;
        writer.write_all(json.as_bytes())?;
        writer.write_all(b"\n")?;
        Ok(())
    }
}

/// Text formatter (strace-style)
pub struct TextFormatter;

impl TextFormatter {
    /// Format single event as text
    pub fn format_event(event: &TraceEvent) -> String {
        match event {
            TraceEvent::FunctionEnter(entry) => {
                let args_str = entry.args.iter()
                    .map(|arg| format!("{}={}", arg.type_info.name, arg.value))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "[{:.6}] -> {}({}) <{}:{}:{}>",
                    entry.timestamp_ns as f64 / 1_000_000_000.0,
                    entry.name,
                    args_str,
                    entry.location.file,
                    entry.location.line,
                    entry.location.column
                )
            }
            TraceEvent::FunctionExit(exit) => {
                let ret_str = if let Some(ref ret) = exit.return_value {
                    format!(" = {}", ret.value)
                } else {
                    String::new()
                };
                format!(
                    "[{:.6}] <- {}(){} [{:.3}ms]",
                    exit.timestamp_ns as f64 / 1_000_000_000.0,
                    exit.name,
                    ret_str,
                    exit.duration_ns as f64 / 1_000_000.0
                )
            }
            TraceEvent::Syscall(syscall) => {
                let args_str = syscall.args.iter()
                    .map(|arg| format!("{}", arg))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "[{:.6}] {}({}) = {} [{:.3}ms]",
                    syscall.timestamp_ns as f64 / 1_000_000_000.0,
                    syscall.name,
                    args_str,
                    syscall.return_value,
                    syscall.duration_ns as f64 / 1_000_000.0
                )
            }
        }
    }

    /// Format all events as text
    pub fn format(events: &[TraceEvent]) -> String {
        events.iter()
            .map(Self::format_event)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tracing::events::{function_enter, function_exit, SourceLocation, TypedValue, TypeInfo};

    #[test]
    fn test_json_formatter() {
        let formatter = JsonFormatter::new(true);

        let trace = TraceFile {
            metadata: TraceMetadata {
                program: "test.ruchy".to_string(),
                start_time: "2025-10-29T00:00:00Z".to_string(),
                ruchy_version: "3.147.7".to_string(),
                ruchyruchy_version: "1.6.1".to_string(),
            },
            events: vec![
                function_enter(
                    "test_fn",
                    vec![],
                    SourceLocation {
                        file: "test.ruchy".to_string(),
                        line: 10,
                        column: 1,
                    },
                ),
            ],
            stats: TraceStats {
                total_events: 1,
                dropped_events: 0,
                duration_ns: 1000000,
            },
        };

        let json = formatter.format(&trace).unwrap();
        assert!(json.contains("\"program\": \"test.ruchy\""));
        assert!(json.contains("\"type\": \"function_enter\""));
        assert!(json.contains("\"total_events\": 1"));
    }

    #[test]
    fn test_text_formatter_function_enter() {
        let event = function_enter(
            "compute",
            vec![
                TypedValue {
                    type_info: TypeInfo {
                        name: "i64".to_string(),
                        fields: None,
                    },
                    value: serde_json::json!(42),
                },
            ],
            SourceLocation {
                file: "test.ruchy".to_string(),
                line: 10,
                column: 5,
            },
        );

        let text = TextFormatter::format_event(&event);
        assert!(text.contains("-> compute(i64=42)"));
        assert!(text.contains("<test.ruchy:10:5>"));
    }

    #[test]
    fn test_text_formatter_function_exit() {
        let event = function_exit(
            "compute",
            Some(TypedValue {
                type_info: TypeInfo {
                    name: "i64".to_string(),
                    fields: None,
                },
                value: serde_json::json!(100),
            }),
            1500000, // 1.5ms
        );

        let text = TextFormatter::format_event(&event);
        assert!(text.contains("<- compute() = 100"));
        assert!(text.contains("[1.500ms]"));
    }
}
