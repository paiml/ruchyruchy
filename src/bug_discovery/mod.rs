// Bug Discovery Module
// Implements DISC-001 through DISC-004 from specification v1.0.0
//
// References:
// - docs/specifications/BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md
// - DISC-001: Differential Testing with Statistical Analysis

pub mod differential;
pub mod confidence;
pub mod statistics;

pub use differential::DifferentialTester;
pub use confidence::{ConfidenceScore, ConfidenceScorer};
pub use statistics::{welchs_t_test, cohens_d, PerformanceRegression};
