//! Conformance Test Suite Export Module
//!
//! This module provides functionality to export RuchyRuchy interpreter
//! test cases as a standalone conformance test suite for the Ruchy compiler.
//!
//! # Overview
//!
//! The conformance export module extracts test cases from Rust integration
//! tests and converts them into portable .ruchy files that can be executed
//! by the Ruchy compiler using `ruchy test`.
//!
//! # Architecture
//!
//! - **ConformanceExporter**: Main exporter class
//! - **TestCase**: Represents a single test case
//! - **ExportResult**: Statistics from export operation
//! - **ExportError**: Error types for export failures
//!
//! # Usage
//!
//! ```no_run
//! use ruchyruchy::conformance::ConformanceExporter;
//!
//! let exporter = ConformanceExporter::new();
//! let result = exporter.export_all_chapters().unwrap();
//! println!("Exported {} tests from {} chapters",
//!          result.test_count, result.chapters_exported);
//! ```
//!
//! # Output Format
//!
//! Exported tests are organized by chapter:
//!
//! ```text
//! conformance/ruchy_test_suite/
//! ├── chapter_01_hello_world/
//! │   ├── test_001_hello_world.ruchy
//! │   └── ...
//! ├── chapter_02_variables/
//! │   └── ...
//! └── ...
//! ```
//!
//! Each test file contains:
//! - Metadata (test name, chapter, description)
//! - Expected output
//! - Ruchy source code

/// Conformance exporter implementation
pub mod exporter;

// Re-export main types
pub use exporter::{ConformanceExporter, ExportError, ExportResult, TestCase};
