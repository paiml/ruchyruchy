// TDD Integration and Cycle Tracking
// REPORT-003: TDD Integration Implementation
//
// References:
// - Beck (2002): "Test-Driven Development: By Example"
// - Freeman & Pryce (2009): "Growing Object-Oriented Software, Guided by Tests"
// - Section 8.3 of BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md

use std::time::{Duration, SystemTime};

/// TDD phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TddPhase {
    /// RED: Write failing test
    Red,
    /// GREEN: Make test pass
    Green,
    /// REFACTOR: Improve code while keeping tests green
    Refactor,
}

impl TddPhase {
    /// Get string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            TddPhase::Red => "RED",
            TddPhase::Green => "GREEN",
            TddPhase::Refactor => "REFACTOR",
        }
    }

    /// Get color code for display
    pub fn color(&self) -> &'static str {
        match self {
            TddPhase::Red => "🔴",
            TddPhase::Green => "🟢",
            TddPhase::Refactor => "🔵",
        }
    }

    /// Get next phase
    pub fn next(&self) -> TddPhase {
        match self {
            TddPhase::Red => TddPhase::Green,
            TddPhase::Green => TddPhase::Refactor,
            TddPhase::Refactor => TddPhase::Red,
        }
    }
}

/// Test result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestResult {
    /// All tests passing
    Pass,
    /// Some tests failing
    Fail,
    /// Tests not run
    NotRun,
}

impl TestResult {
    /// Check if passing
    pub fn is_pass(&self) -> bool {
        matches!(self, TestResult::Pass)
    }

    /// Check if failing
    pub fn is_fail(&self) -> bool {
        matches!(self, TestResult::Fail)
    }
}

/// TDD cycle entry
#[derive(Debug, Clone)]
pub struct TddCycle {
    /// Cycle number
    pub cycle: usize,
    /// Current phase
    pub phase: TddPhase,
    /// Test result
    pub test_result: TestResult,
    /// Number of tests
    pub test_count: usize,
    /// Number of passing tests
    pub passing: usize,
    /// Number of failing tests
    pub failing: usize,
    /// Coverage percentage (0.0-100.0)
    pub coverage: f64,
    /// Timestamp
    pub timestamp: SystemTime,
    /// Duration in this phase
    pub duration: Duration,
    /// Description/notes
    pub description: String,
}

impl TddCycle {
    /// Create a new TDD cycle
    pub fn new(cycle: usize, phase: TddPhase, description: String) -> Self {
        TddCycle {
            cycle,
            phase,
            test_result: TestResult::NotRun,
            test_count: 0,
            passing: 0,
            failing: 0,
            coverage: 0.0,
            timestamp: SystemTime::now(),
            duration: Duration::from_secs(0),
            description,
        }
    }

    /// Update test results
    pub fn update_tests(
        &mut self,
        test_count: usize,
        passing: usize,
        failing: usize,
        coverage: f64,
    ) {
        self.test_count = test_count;
        self.passing = passing;
        self.failing = failing;
        self.coverage = coverage.max(0.0).min(100.0);

        self.test_result = if failing == 0 && test_count > 0 {
            TestResult::Pass
        } else if failing > 0 {
            TestResult::Fail
        } else {
            TestResult::NotRun
        };
    }

    /// Check if cycle is valid for phase
    pub fn is_valid_for_phase(&self) -> bool {
        match self.phase {
            TddPhase::Red => {
                // RED phase should have failing tests
                self.test_result.is_fail()
            }
            TddPhase::Green => {
                // GREEN phase should have passing tests
                self.test_result.is_pass()
            }
            TddPhase::Refactor => {
                // REFACTOR phase should have passing tests
                self.test_result.is_pass()
            }
        }
    }

    /// Check if coverage improved
    pub fn coverage_improved(&self, previous: &TddCycle) -> bool {
        self.coverage > previous.coverage
    }
}

/// TDD history tracker
#[derive(Debug, Clone)]
pub struct TddHistory {
    /// All cycles
    cycles: Vec<TddCycle>,
    /// Current cycle number
    current_cycle: usize,
    /// Start time
    start_time: SystemTime,
}

impl TddHistory {
    /// Create a new TDD history
    pub fn new() -> Self {
        TddHistory {
            cycles: Vec::new(),
            current_cycle: 0,
            start_time: SystemTime::now(),
        }
    }

    /// Add a cycle
    pub fn add_cycle(&mut self, mut cycle: TddCycle) {
        self.current_cycle += 1;
        cycle.cycle = self.current_cycle;
        self.cycles.push(cycle);
    }

    /// Get all cycles
    pub fn cycles(&self) -> &[TddCycle] {
        &self.cycles
    }

    /// Get current cycle
    pub fn current(&self) -> Option<&TddCycle> {
        self.cycles.last()
    }

    /// Get cycles for a specific phase
    pub fn cycles_for_phase(&self, phase: TddPhase) -> Vec<&TddCycle> {
        self.cycles.iter().filter(|c| c.phase == phase).collect()
    }

    /// Get total duration
    pub fn total_duration(&self) -> Duration {
        self.cycles.iter().map(|c| c.duration).sum()
    }

    /// Get average cycle duration
    pub fn average_cycle_duration(&self) -> Duration {
        if self.cycles.is_empty() {
            return Duration::from_secs(0);
        }

        let total = self.total_duration();
        total / self.cycles.len() as u32
    }

    /// Get current coverage
    pub fn current_coverage(&self) -> f64 {
        self.current().map(|c| c.coverage).unwrap_or(0.0)
    }

    /// Check if currently in valid state
    pub fn is_valid_state(&self) -> bool {
        self.current()
            .map(|c| c.is_valid_for_phase())
            .unwrap_or(true)
    }

    /// Count complete RED-GREEN-REFACTOR cycles
    pub fn complete_cycles(&self) -> usize {
        let mut complete = 0;
        let mut in_cycle = false;
        let mut seen_red = false;
        let mut seen_green = false;

        for cycle in &self.cycles {
            match cycle.phase {
                TddPhase::Red if cycle.test_result.is_fail() => {
                    seen_red = true;
                    seen_green = false;
                    in_cycle = true;
                }
                TddPhase::Green if cycle.test_result.is_pass() && seen_red => {
                    seen_green = true;
                }
                TddPhase::Refactor if cycle.test_result.is_pass() && seen_green => {
                    if in_cycle {
                        complete += 1;
                        in_cycle = false;
                        seen_red = false;
                        seen_green = false;
                    }
                }
                _ => {}
            }
        }

        complete
    }

    /// Generate markdown report
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# TDD Cycle History\n\n");
        md.push_str(&format!("**Total Cycles**: {}\n", self.cycles.len()));
        md.push_str(&format!(
            "**Complete RED-GREEN-REFACTOR Cycles**: {}\n",
            self.complete_cycles()
        ));
        md.push_str(&format!(
            "**Current Coverage**: {:.1}%\n",
            self.current_coverage()
        ));
        md.push_str(&format!(
            "**Total Duration**: {:?}\n",
            self.total_duration()
        ));
        md.push_str(&format!(
            "**Average Cycle Duration**: {:?}\n\n",
            self.average_cycle_duration()
        ));

        md.push_str("## Cycle History\n\n");
        md.push_str("| Cycle | Phase | Tests | Passing | Failing | Coverage | Duration | Valid |\n");
        md.push_str("|-------|-------|-------|---------|---------|----------|----------|-------|\n");

        for cycle in &self.cycles {
            md.push_str(&format!(
                "| {} | {} {} | {} | {} | {} | {:.1}% | {:?} | {} |\n",
                cycle.cycle,
                cycle.phase.color(),
                cycle.phase.as_str(),
                cycle.test_count,
                cycle.passing,
                cycle.failing,
                cycle.coverage,
                cycle.duration,
                if cycle.is_valid_for_phase() {
                    "✅"
                } else {
                    "❌"
                }
            ));
        }

        md.push_str("\n## Phase Breakdown\n\n");
        md.push_str(&format!(
            "- **RED phases**: {}\n",
            self.cycles_for_phase(TddPhase::Red).len()
        ));
        md.push_str(&format!(
            "- **GREEN phases**: {}\n",
            self.cycles_for_phase(TddPhase::Green).len()
        ));
        md.push_str(&format!(
            "- **REFACTOR phases**: {}\n\n",
            self.cycles_for_phase(TddPhase::Refactor).len()
        ));

        md.push_str("---\n\n");
        md.push_str("**TDD Adherence**: ");
        if self.is_valid_state() {
            md.push_str("✅ Currently in valid state\n");
        } else {
            md.push_str("⚠️  Invalid state detected (phase doesn't match test results)\n");
        }

        md
    }
}

impl Default for TddHistory {
    fn default() -> Self {
        Self::new()
    }
}

/// Quality gate result
#[derive(Debug, Clone)]
pub struct QualityGate {
    /// Gate name
    pub name: String,
    /// Passed or failed
    pub passed: bool,
    /// Message
    pub message: String,
    /// Metric value (if applicable)
    pub value: Option<f64>,
    /// Threshold (if applicable)
    pub threshold: Option<f64>,
}

impl QualityGate {
    /// Create a new quality gate
    pub fn new(name: String, passed: bool, message: String) -> Self {
        QualityGate {
            name,
            passed,
            message,
            value: None,
            threshold: None,
        }
    }

    /// Create with metric
    pub fn with_metric(
        name: String,
        passed: bool,
        message: String,
        value: f64,
        threshold: f64,
    ) -> Self {
        QualityGate {
            name,
            passed,
            message,
            value: Some(value),
            threshold: Some(threshold),
        }
    }
}

/// Quality gates checker
#[derive(Debug, Clone)]
pub struct QualityGates {
    /// Gates to check
    gates: Vec<QualityGate>,
}

impl QualityGates {
    /// Create new quality gates
    pub fn new() -> Self {
        QualityGates { gates: Vec::new() }
    }

    /// Add a gate
    pub fn add_gate(&mut self, gate: QualityGate) {
        self.gates.push(gate);
    }

    /// Check all gates
    pub fn all_passed(&self) -> bool {
        self.gates.iter().all(|g| g.passed)
    }

    /// Get failed gates
    pub fn failed_gates(&self) -> Vec<&QualityGate> {
        self.gates.iter().filter(|g| !g.passed).collect()
    }

    /// Get all gates
    pub fn gates(&self) -> &[QualityGate] {
        &self.gates
    }

    /// Generate markdown report
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# Quality Gates\n\n");
        md.push_str(&format!(
            "**Status**: {}\n\n",
            if self.all_passed() {
                "✅ All gates passed"
            } else {
                "❌ Some gates failed"
            }
        ));

        md.push_str("| Gate | Status | Message | Metric |\n");
        md.push_str("|------|--------|---------|--------|\n");

        for gate in &self.gates {
            let status = if gate.passed { "✅" } else { "❌" };
            let metric = if let (Some(value), Some(threshold)) = (gate.value, gate.threshold) {
                format!("{:.2} (threshold: {:.2})", value, threshold)
            } else {
                "N/A".to_string()
            };

            md.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                gate.name, status, gate.message, metric
            ));
        }

        md
    }
}

impl Default for QualityGates {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tdd_phase_strings() {
        assert_eq!(TddPhase::Red.as_str(), "RED");
        assert_eq!(TddPhase::Green.as_str(), "GREEN");
        assert_eq!(TddPhase::Refactor.as_str(), "REFACTOR");
    }

    #[test]
    fn test_tdd_phase_next() {
        assert_eq!(TddPhase::Red.next(), TddPhase::Green);
        assert_eq!(TddPhase::Green.next(), TddPhase::Refactor);
        assert_eq!(TddPhase::Refactor.next(), TddPhase::Red);
    }

    #[test]
    fn test_test_result_checks() {
        assert!(TestResult::Pass.is_pass());
        assert!(!TestResult::Pass.is_fail());

        assert!(TestResult::Fail.is_fail());
        assert!(!TestResult::Fail.is_pass());
    }

    #[test]
    fn test_tdd_cycle_creation() {
        let cycle = TddCycle::new(1, TddPhase::Red, "Write failing test".to_string());

        assert_eq!(cycle.cycle, 1);
        assert_eq!(cycle.phase, TddPhase::Red);
        assert_eq!(cycle.description, "Write failing test");
        assert_eq!(cycle.test_result, TestResult::NotRun);
    }

    #[test]
    fn test_tdd_cycle_update_tests() {
        let mut cycle = TddCycle::new(1, TddPhase::Red, "Test".to_string());
        cycle.update_tests(10, 8, 2, 75.5);

        assert_eq!(cycle.test_count, 10);
        assert_eq!(cycle.passing, 8);
        assert_eq!(cycle.failing, 2);
        assert!((cycle.coverage - 75.5).abs() < 0.01);
        assert_eq!(cycle.test_result, TestResult::Fail);
    }

    #[test]
    fn test_tdd_cycle_valid_for_phase() {
        let mut red_cycle = TddCycle::new(1, TddPhase::Red, "RED".to_string());
        red_cycle.update_tests(1, 0, 1, 0.0);
        assert!(red_cycle.is_valid_for_phase());

        let mut green_cycle = TddCycle::new(2, TddPhase::Green, "GREEN".to_string());
        green_cycle.update_tests(1, 1, 0, 50.0);
        assert!(green_cycle.is_valid_for_phase());

        let mut invalid_red = TddCycle::new(3, TddPhase::Red, "Invalid".to_string());
        invalid_red.update_tests(1, 1, 0, 50.0); // Should be failing for RED
        assert!(!invalid_red.is_valid_for_phase());
    }

    #[test]
    fn test_tdd_history_creation() {
        let history = TddHistory::new();
        assert_eq!(history.cycles().len(), 0);
        assert_eq!(history.current_cycle, 0);
    }

    #[test]
    fn test_tdd_history_add_cycle() {
        let mut history = TddHistory::new();
        let cycle = TddCycle::new(0, TddPhase::Red, "Test".to_string());

        history.add_cycle(cycle);
        assert_eq!(history.cycles().len(), 1);
        assert_eq!(history.current_cycle, 1);
    }

    #[test]
    fn test_tdd_history_current() {
        let mut history = TddHistory::new();
        assert!(history.current().is_none());

        let cycle = TddCycle::new(0, TddPhase::Red, "Test".to_string());
        history.add_cycle(cycle);
        assert!(history.current().is_some());
        assert_eq!(history.current().unwrap().phase, TddPhase::Red);
    }

    #[test]
    fn test_tdd_history_cycles_for_phase() {
        let mut history = TddHistory::new();

        history.add_cycle(TddCycle::new(0, TddPhase::Red, "Red1".to_string()));
        history.add_cycle(TddCycle::new(0, TddPhase::Green, "Green1".to_string()));
        history.add_cycle(TddCycle::new(0, TddPhase::Red, "Red2".to_string()));

        let red_cycles = history.cycles_for_phase(TddPhase::Red);
        assert_eq!(red_cycles.len(), 2);
    }

    #[test]
    fn test_tdd_history_complete_cycles() {
        let mut history = TddHistory::new();

        // Complete cycle
        let mut red = TddCycle::new(0, TddPhase::Red, "RED".to_string());
        red.update_tests(1, 0, 1, 0.0);
        history.add_cycle(red);

        let mut green = TddCycle::new(0, TddPhase::Green, "GREEN".to_string());
        green.update_tests(1, 1, 0, 50.0);
        history.add_cycle(green);

        let mut refactor = TddCycle::new(0, TddPhase::Refactor, "REFACTOR".to_string());
        refactor.update_tests(1, 1, 0, 50.0);
        history.add_cycle(refactor);

        assert_eq!(history.complete_cycles(), 1);
    }

    #[test]
    fn test_quality_gate_creation() {
        let gate = QualityGate::new("Coverage".to_string(), true, "Met threshold".to_string());

        assert_eq!(gate.name, "Coverage");
        assert!(gate.passed);
        assert_eq!(gate.message, "Met threshold");
    }

    #[test]
    fn test_quality_gate_with_metric() {
        let gate = QualityGate::with_metric(
            "Coverage".to_string(),
            true,
            "Good".to_string(),
            85.5,
            80.0,
        );

        assert!(gate.value.is_some());
        assert!((gate.value.unwrap() - 85.5).abs() < 0.01);
        assert_eq!(gate.threshold, Some(80.0));
    }

    #[test]
    fn test_quality_gates_all_passed() {
        let mut gates = QualityGates::new();
        assert!(gates.all_passed());

        gates.add_gate(QualityGate::new("Test1".to_string(), true, "OK".to_string()));
        gates.add_gate(QualityGate::new("Test2".to_string(), true, "OK".to_string()));
        assert!(gates.all_passed());

        gates.add_gate(QualityGate::new(
            "Test3".to_string(),
            false,
            "Failed".to_string(),
        ));
        assert!(!gates.all_passed());
    }

    #[test]
    fn test_quality_gates_failed_gates() {
        let mut gates = QualityGates::new();
        gates.add_gate(QualityGate::new("Pass".to_string(), true, "OK".to_string()));
        gates.add_gate(QualityGate::new(
            "Fail".to_string(),
            false,
            "Failed".to_string(),
        ));

        let failed = gates.failed_gates();
        assert_eq!(failed.len(), 1);
        assert_eq!(failed[0].name, "Fail");
    }

    #[test]
    fn test_tdd_history_markdown() {
        let mut history = TddHistory::new();
        let mut cycle = TddCycle::new(0, TddPhase::Red, "Test".to_string());
        cycle.update_tests(1, 0, 1, 0.0);
        history.add_cycle(cycle);

        let markdown = history.to_markdown();
        assert!(markdown.contains("# TDD Cycle History"));
        assert!(markdown.contains("RED"));
    }

    #[test]
    fn test_quality_gates_markdown() {
        let mut gates = QualityGates::new();
        gates.add_gate(QualityGate::new("Test".to_string(), true, "OK".to_string()));

        let markdown = gates.to_markdown();
        assert!(markdown.contains("# Quality Gates"));
        assert!(markdown.contains("✅"));
    }
}
