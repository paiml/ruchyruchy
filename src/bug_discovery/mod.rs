// Bug Discovery Module
// Implements DISC-001 through DISC-004 from specification v1.0.0
//
// References:
// - docs/specifications/BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md
// - DISC-001: Differential Testing with Statistical Analysis
// - DISC-002: Grammar-Based Fuzzing
// - DISC-002B: Schema-Based Runtime Property Fuzzing (CRITICAL for Issue #79, #76, #75)
// - DISC-003: Property-Based Testing Integration
// - DISC-004: Code Churn Analysis

pub mod code_churn;
pub mod confidence;
pub mod differential;
pub mod grammar_fuzzer;
pub mod property_testing;
pub mod schema_fuzzer;
pub mod statistics;

pub use code_churn::{ChurnAnalyzer, ChurnHotspot, ChurnMetrics, FileChange, RiskLevel};
pub use confidence::{ConfidenceScore, ConfidenceScorer};
pub use differential::DifferentialTester;
pub use grammar_fuzzer::{
    FuzzBug, FuzzCorpus, FuzzResult, Grammar, GrammarFuzzer, GrammarRule, TestMinimizer,
};
pub use property_testing::{
    AstGenerator, Generator, Property, PropertyBug, PropertyChecker, PropertyResult,
};
pub use schema_fuzzer::{
    RuntimeSchema, RuntimeTestCase, SchemaFuzzer, SchemaFuzzerConfig, ShadowState, TimeoutDetection,
};
pub use statistics::{cohens_d, welchs_t_test, PerformanceRegression};
