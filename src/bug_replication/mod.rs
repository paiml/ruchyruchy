// Bug Replication Module
// Implements REPLIC-001 through REPLIC-003 from specification v1.0.0
//
// References:
// - docs/specifications/BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md
// - REPLIC-001: Minimization System (Delta Debugging)
// - REPLIC-002: Replication Harness
// - REPLIC-003: Bisection Tool

/// Git bisection for finding bug-introducing commits
pub mod bisect;
/// Test execution harness for bug replication
pub mod harness;
/// Delta debugging for test case minimization
pub mod minimizer;

pub use bisect::{BisectionResult, BisectionState, Commit, CommitId, GitBisector, TestResult};
pub use harness::{Environment, ExecutionResult, ReplicationHarness, ReproducibleTest};
pub use minimizer::{DeltaDebugger, MinimizationResult, MinimizationStrategy, TestOutcome};
