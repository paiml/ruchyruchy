// Bug Replication Module
// Implements REPLIC-001 through REPLIC-003 from specification v1.0.0
//
// References:
// - docs/specifications/BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md
// - REPLIC-001: Minimization System (Delta Debugging)
// - REPLIC-002: Replication Harness
// - REPLIC-003: Bisection Tool

pub mod minimizer;
pub mod harness;
pub mod bisect;

pub use minimizer::{DeltaDebugger, MinimizationStrategy, MinimizationResult, TestOutcome};
pub use harness::{
    Environment, ExecutionResult, ReproducibleTest, ReplicationHarness,
};
pub use bisect::{
    CommitId, Commit, TestResult, BisectionState, BisectionResult, GitBisector,
};
