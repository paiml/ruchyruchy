// Bug Replication Module
// Implements REPLIC-001 through REPLIC-003 from specification v1.0.0
//
// References:
// - docs/specifications/BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md
// - REPLIC-001: Minimization System (Delta Debugging)
// - REPLIC-002: Replication Harness
// - REPLIC-003: Bisection Tool

pub mod bisect;
pub mod harness;
pub mod minimizer;

pub use bisect::{BisectionResult, BisectionState, Commit, CommitId, GitBisector, TestResult};
pub use harness::{Environment, ExecutionResult, ReplicationHarness, ReproducibleTest};
pub use minimizer::{DeltaDebugger, MinimizationResult, MinimizationStrategy, TestOutcome};
