// Bug Replication Harness
// REPLIC-002: Replication Harness Implementation
//
// References:
// - Chen & Kim (2015): "Crash reproduction via test case mutation"
// - Jin et al. (2012): "Automated behavioral regression testing"
// - Section 7.2 of BUG_DISCOVERY_REPORTER_REPLICATOR_SPEC.md

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Environment information for bug reproduction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Environment {
    /// Compiler version (e.g., "ruchy 0.1.0")
    pub compiler_version: String,
    /// Operating system (e.g., "Linux 6.8.0")
    pub os: String,
    /// Architecture (e.g., "x86_64")
    pub arch: String,
    /// Environment variables
    pub env_vars: HashMap<String, String>,
    /// Dependencies with versions
    pub dependencies: HashMap<String, String>,
}

impl Environment {
    /// Create a new environment
    pub fn new(compiler_version: String, os: String, arch: String) -> Self {
        Environment {
            compiler_version,
            os,
            arch,
            env_vars: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }

    /// Add an environment variable
    pub fn with_env_var(mut self, key: String, value: String) -> Self {
        self.env_vars.insert(key, value);
        self
    }

    /// Add a dependency
    pub fn with_dependency(mut self, name: String, version: String) -> Self {
        self.dependencies.insert(name, version);
        self
    }

    /// Capture current system environment
    pub fn capture_current() -> Self {
        let os = std::env::consts::OS.to_string();
        let arch = std::env::consts::ARCH.to_string();

        // In a real implementation, would capture:
        // - ruchy --version output
        // - All environment variables
        // - All dependencies from Cargo.toml/package.json

        Environment {
            compiler_version: "ruchy 0.1.0".to_string(),
            os,
            arch,
            env_vars: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }
}

/// Result of test execution
#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionResult {
    /// Test passed
    Success { output: String, duration: Duration },
    /// Test failed with error
    Failure {
        error: String,
        output: String,
        duration: Duration,
    },
    /// Test hung (exceeded timeout)
    Timeout {
        timeout_ms: u64,
        partial_output: String,
    },
    /// Test crashed
    Crash {
        signal: String,
        output: String,
        duration: Duration,
    },
}

impl ExecutionResult {
    /// Check if result indicates success
    pub fn is_success(&self) -> bool {
        matches!(self, ExecutionResult::Success { .. })
    }

    /// Check if result indicates failure
    pub fn is_failure(&self) -> bool {
        matches!(self, ExecutionResult::Failure { .. })
    }

    /// Check if result indicates timeout
    pub fn is_timeout(&self) -> bool {
        matches!(self, ExecutionResult::Timeout { .. })
    }

    /// Check if result indicates crash
    pub fn is_crash(&self) -> bool {
        matches!(self, ExecutionResult::Crash { .. })
    }

    /// Get duration if available
    pub fn duration(&self) -> Option<Duration> {
        match self {
            ExecutionResult::Success { duration, .. } => Some(*duration),
            ExecutionResult::Failure { duration, .. } => Some(*duration),
            ExecutionResult::Crash { duration, .. } => Some(*duration),
            ExecutionResult::Timeout { .. } => None,
        }
    }
}

/// Reproducible test case
#[derive(Debug, Clone)]
pub struct ReproducibleTest {
    /// Test source code
    pub source: String,
    /// Expected result
    pub expected: ExecutionResult,
    /// Environment requirements
    pub environment: Environment,
    /// Reproduction steps (human-readable)
    pub steps: Vec<String>,
}

impl ReproducibleTest {
    /// Create a new reproducible test
    pub fn new(source: String, expected: ExecutionResult, environment: Environment) -> Self {
        ReproducibleTest {
            source,
            expected,
            environment,
            steps: Vec::new(),
        }
    }

    /// Add a reproduction step
    pub fn add_step(mut self, step: String) -> Self {
        self.steps.push(step);
        self
    }

    /// Generate markdown documentation
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();

        md.push_str("# Reproducible Test Case\n\n");

        // Environment section
        md.push_str("## Environment\n\n");
        md.push_str(&format!(
            "- **Compiler**: {}\n",
            self.environment.compiler_version
        ));
        md.push_str(&format!("- **OS**: {}\n", self.environment.os));
        md.push_str(&format!("- **Arch**: {}\n", self.environment.arch));

        if !self.environment.dependencies.is_empty() {
            md.push_str("\n### Dependencies\n\n");
            for (name, version) in &self.environment.dependencies {
                md.push_str(&format!("- `{}`: {}\n", name, version));
            }
        }

        // Reproduction steps
        if !self.steps.is_empty() {
            md.push_str("\n## Reproduction Steps\n\n");
            for (i, step) in self.steps.iter().enumerate() {
                md.push_str(&format!("{}. {}\n", i + 1, step));
            }
        }

        // Source code
        md.push_str("\n## Source Code\n\n```ruchy\n");
        md.push_str(&self.source);
        md.push_str("\n```\n");

        // Expected result
        md.push_str("\n## Expected Result\n\n");
        match &self.expected {
            ExecutionResult::Success { output, duration } => {
                md.push_str(&format!("**Status**: Success ({:?})\n\n", duration));
                md.push_str("**Output**:\n```\n");
                md.push_str(output);
                md.push_str("\n```\n");
            }
            ExecutionResult::Failure {
                error,
                output,
                duration,
            } => {
                md.push_str(&format!("**Status**: Failure ({:?})\n\n", duration));
                md.push_str(&format!("**Error**: {}\n\n", error));
                md.push_str("**Output**:\n```\n");
                md.push_str(output);
                md.push_str("\n```\n");
            }
            ExecutionResult::Timeout {
                timeout_ms,
                partial_output,
            } => {
                md.push_str(&format!("**Status**: Timeout (>{}ms)\n\n", timeout_ms));
                md.push_str("**Partial Output**:\n```\n");
                md.push_str(partial_output);
                md.push_str("\n```\n");
            }
            ExecutionResult::Crash {
                signal,
                output,
                duration,
            } => {
                md.push_str(&format!("**Status**: Crash ({:?})\n\n", duration));
                md.push_str(&format!("**Signal**: {}\n\n", signal));
                md.push_str("**Output**:\n```\n");
                md.push_str(output);
                md.push_str("\n```\n");
            }
        }

        md
    }
}

/// Replication harness for automated bug reproduction
pub struct ReplicationHarness {
    /// Timeout for test execution (milliseconds)
    pub timeout_ms: u64,
    /// Maximum number of reproduction attempts
    pub max_attempts: usize,
    /// Captured environment
    pub environment: Environment,
}

impl ReplicationHarness {
    /// Create a new replication harness
    pub fn new() -> Self {
        ReplicationHarness {
            timeout_ms: 5000, // Default: 5 seconds
            max_attempts: 3,
            environment: Environment::capture_current(),
        }
    }

    /// Set timeout
    pub fn with_timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    /// Set max attempts
    pub fn with_max_attempts(mut self, max_attempts: usize) -> Self {
        self.max_attempts = max_attempts;
        self
    }

    /// Execute a test case
    pub fn execute(&self, source: &str) -> ExecutionResult {
        let start = Instant::now();

        // Simulate test execution
        // In a real implementation, this would:
        // 1. Write source to temporary file
        // 2. Run: ruchy run <file>
        // 3. Capture stdout/stderr
        // 4. Detect timeout using process::Command with timeout
        // 5. Detect crashes via exit code

        // For now, we simulate based on source content
        if source.contains("Vec::new()") && source.contains("// KNOWN BUG #76") {
            // Simulate timeout
            ExecutionResult::Timeout {
                timeout_ms: self.timeout_ms,
                partial_output: "Started Vec::new()...".to_string(),
            }
        } else if source.contains("panic!") {
            // Simulate crash
            ExecutionResult::Crash {
                signal: "SIGABRT".to_string(),
                output: "thread 'main' panicked at 'explicit panic'".to_string(),
                duration: start.elapsed(),
            }
        } else if source.contains("// ERROR:") {
            // Simulate failure
            let error_line = source
                .lines()
                .find(|line| line.contains("// ERROR:"))
                .unwrap_or("// ERROR: unknown");
            let error = error_line
                .trim_start_matches("// ERROR:")
                .trim()
                .to_string();

            ExecutionResult::Failure {
                error,
                output: String::new(),
                duration: start.elapsed(),
            }
        } else {
            // Simulate success
            ExecutionResult::Success {
                output: "Test passed".to_string(),
                duration: start.elapsed(),
            }
        }
    }

    /// Reproduce a bug with multiple attempts
    pub fn reproduce(&self, source: &str, attempts: usize) -> Vec<ExecutionResult> {
        let mut results = Vec::new();

        for _ in 0..attempts.min(self.max_attempts) {
            let result = self.execute(source);
            results.push(result);
        }

        results
    }

    /// Check if bug is reproducible (consistent across attempts)
    pub fn is_reproducible(&self, source: &str) -> bool {
        let results = self.reproduce(source, self.max_attempts);

        if results.is_empty() {
            return false;
        }

        // Check if all results have the same outcome type
        let first_type = std::mem::discriminant(&results[0]);
        results
            .iter()
            .all(|r| std::mem::discriminant(r) == first_type)
    }

    /// Create a reproducible test case from source
    pub fn create_reproducible_test(&self, source: &str) -> Option<ReproducibleTest> {
        // Execute multiple times to verify reproducibility
        let results = self.reproduce(source, self.max_attempts);

        if results.is_empty() {
            return None;
        }

        // Check consistency
        let first_type = std::mem::discriminant(&results[0]);
        let is_consistent = results
            .iter()
            .all(|r| std::mem::discriminant(r) == first_type);

        if !is_consistent {
            // Non-deterministic bug - harder to reproduce
            return None;
        }

        // Create reproducible test with first result as expected
        let mut test = ReproducibleTest::new(
            source.to_string(),
            results[0].clone(),
            self.environment.clone(),
        );

        // Add reproduction steps
        test = test.add_step("Save source code to file (e.g., test.ruchy)".to_string());
        test = test.add_step("Run: ruchy run test.ruchy".to_string());
        test = test.add_step(format!("Expected: {:?}", results[0]));

        Some(test)
    }
}

impl Default for ReplicationHarness {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_creation() {
        let env = Environment::new(
            "ruchy 0.1.0".to_string(),
            "Linux".to_string(),
            "x86_64".to_string(),
        );

        assert_eq!(env.compiler_version, "ruchy 0.1.0");
        assert_eq!(env.os, "Linux");
        assert_eq!(env.arch, "x86_64");
        assert!(env.env_vars.is_empty());
        assert!(env.dependencies.is_empty());
    }

    #[test]
    fn test_environment_with_vars() {
        let env = Environment::new(
            "ruchy 0.1.0".to_string(),
            "Linux".to_string(),
            "x86_64".to_string(),
        )
        .with_env_var("PATH".to_string(), "/usr/bin".to_string())
        .with_dependency("std".to_string(), "0.1.0".to_string());

        assert_eq!(env.env_vars.get("PATH"), Some(&"/usr/bin".to_string()));
        assert_eq!(env.dependencies.get("std"), Some(&"0.1.0".to_string()));
    }

    #[test]
    fn test_environment_capture() {
        let env = Environment::capture_current();
        assert!(!env.os.is_empty());
        assert!(!env.arch.is_empty());
    }

    #[test]
    fn test_execution_result_success() {
        let result = ExecutionResult::Success {
            output: "OK".to_string(),
            duration: Duration::from_millis(100),
        };

        assert!(result.is_success());
        assert!(!result.is_failure());
        assert!(!result.is_timeout());
        assert!(!result.is_crash());
        assert_eq!(result.duration(), Some(Duration::from_millis(100)));
    }

    #[test]
    fn test_execution_result_timeout() {
        let result = ExecutionResult::Timeout {
            timeout_ms: 5000,
            partial_output: "Partial".to_string(),
        };

        assert!(!result.is_success());
        assert!(result.is_timeout());
        assert_eq!(result.duration(), None);
    }

    #[test]
    fn test_reproducible_test_creation() {
        let env = Environment::new(
            "ruchy 0.1.0".to_string(),
            "Linux".to_string(),
            "x86_64".to_string(),
        );

        let expected = ExecutionResult::Success {
            output: "OK".to_string(),
            duration: Duration::from_millis(100),
        };

        let test = ReproducibleTest::new("fun main() {}".to_string(), expected, env);

        assert_eq!(test.source, "fun main() {}");
        assert!(test.steps.is_empty());
    }

    #[test]
    fn test_reproducible_test_with_steps() {
        let env = Environment::new(
            "ruchy 0.1.0".to_string(),
            "Linux".to_string(),
            "x86_64".to_string(),
        );

        let expected = ExecutionResult::Success {
            output: "OK".to_string(),
            duration: Duration::from_millis(100),
        };

        let test = ReproducibleTest::new("fun main() {}".to_string(), expected, env)
            .add_step("Step 1".to_string())
            .add_step("Step 2".to_string());

        assert_eq!(test.steps.len(), 2);
    }

    #[test]
    fn test_reproducible_test_markdown() {
        let env = Environment::new(
            "ruchy 0.1.0".to_string(),
            "Linux".to_string(),
            "x86_64".to_string(),
        );

        let expected = ExecutionResult::Success {
            output: "OK".to_string(),
            duration: Duration::from_millis(100),
        };

        let test = ReproducibleTest::new("fun main() {}".to_string(), expected, env);

        let markdown = test.to_markdown();
        assert!(markdown.contains("# Reproducible Test Case"));
        assert!(markdown.contains("## Environment"));
        assert!(markdown.contains("## Source Code"));
        assert!(markdown.contains("```ruchy"));
    }

    #[test]
    fn test_replication_harness_creation() {
        let harness = ReplicationHarness::new();
        assert_eq!(harness.timeout_ms, 5000);
        assert_eq!(harness.max_attempts, 3);
    }

    #[test]
    fn test_replication_harness_configuration() {
        let harness = ReplicationHarness::new()
            .with_timeout_ms(10000)
            .with_max_attempts(5);

        assert_eq!(harness.timeout_ms, 10000);
        assert_eq!(harness.max_attempts, 5);
    }

    #[test]
    fn test_execute_success() {
        let harness = ReplicationHarness::new();
        let result = harness.execute("fun main() { println(\"Hello\"); }");

        assert!(result.is_success());
    }

    #[test]
    fn test_execute_timeout() {
        let harness = ReplicationHarness::new();
        let result = harness.execute("// KNOWN BUG #76\nlet v = Vec::new();");

        assert!(result.is_timeout());
    }

    #[test]
    fn test_execute_crash() {
        let harness = ReplicationHarness::new();
        let result = harness.execute("fun main() { panic!(\"test\"); }");

        assert!(result.is_crash());
    }

    #[test]
    fn test_execute_failure() {
        let harness = ReplicationHarness::new();
        let result = harness.execute("// ERROR: type mismatch\nlet x: i32 = \"hello\";");

        assert!(result.is_failure());
    }

    #[test]
    fn test_reproduce_multiple_attempts() {
        let harness = ReplicationHarness::new().with_max_attempts(3);
        let results = harness.reproduce("fun main() {}", 3);

        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.is_success()));
    }

    #[test]
    fn test_is_reproducible_consistent() {
        let harness = ReplicationHarness::new();
        let is_repro = harness.is_reproducible("fun main() {}");

        assert!(is_repro);
    }

    #[test]
    fn test_create_reproducible_test() {
        let harness = ReplicationHarness::new();
        let test = harness.create_reproducible_test("fun main() {}");

        assert!(test.is_some());
        let test = test.unwrap();
        assert_eq!(test.source, "fun main() {}");
        assert!(!test.steps.is_empty());
    }
}
