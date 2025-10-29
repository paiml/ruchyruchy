// Bug Reporting Module
// Implements REPORT-001 through REPORT-004 from specification v1.0.0
//
// References:
// - docs/specifications/BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md
// - REPORT-001: Quantitative Analysis Engine
// - REPORT-002: Assisted Five-Whys Analysis
// - REPORT-003: TDD Integration
// - REPORT-004: Markdown Report Generator

pub mod metrics;
pub mod five_whys;
pub mod tdd;

pub use metrics::{
    ComplexityMetrics, ChurnCorrelation, SatdDetector, SatdType, DependencyAnalyzer,
    DependencyNode, QuantitativeAnalysis,
};
pub use five_whys::{
    ConfidenceLevel, DataPoint, Hypothesis, WhyLayer, FiveWhysAnalysis, FiveWhysAnalyzer,
};
pub use tdd::{
    TddPhase, TestResult, TddCycle, TddHistory, QualityGate, QualityGates,
};
