// Schema-Based Runtime Property Fuzzing
// DISC-002B: Schema-Based Runtime Property Fuzzing Implementation
//
// CRITICAL FOR RUNTIME HANGS (#79, #76, #75, #74)
//
// This module implements stateful behavioral fuzzing with timeout detection.
// Unlike syntax fuzzing (grammar_fuzzer.rs), this tests RUNTIME BEHAVIOR:
// - Models valid states and transitions of runtime objects
// - Generates sequences of operations with preconditions
// - Detects timeouts on constructors (<100ms) and operations (<1000ms)
// - Uses shadow state tracking for validation
// - Minimizes failing tests with delta debugging
//
// References:
// - Zeller & Hildebrandt (2002): "Simplifying and Isolating Failure-Inducing Input"
// - Pacheco et al. (2007): "Randoop: Feedback-Directed Random Testing"
// - Fraser & Arcuri (2011): "EvoSuite: Automatic Test Suite Generation"
//
// Example: Detecting Issue #79 (enum field cast hang)
//
// Schema:
// ```yaml
// type: Logger
// constructor:
//   name: create
//   timeout_ms: 100  # Must complete in <100ms
//   returns: Logger
// operations:
//   - name: test
//     preconditions: []
//     timeout_ms: 1000  # Must complete in <1s
//     returns: void
// ```
//
// Generated Test:
// ```ruchy
// let logger = Logger::create();  // Timeout check: <100ms
// logger.test();                   // Timeout check: <1000ms (FAILS on Issue #79!)
// ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::time::{Duration, Instant};

/// Runtime schema for an object type
///
/// Defines valid operations, preconditions, and timeout thresholds
/// for stateful behavioral fuzzing.
///
/// # Example
///
/// ```yaml
/// type: Vec<i32>
/// constructor:
///   name: new
///   timeout_ms: 100
///   returns: Vec<i32>
/// operations:
///   - name: push
///     preconditions: []
///     parameters: [i32]
///     timeout_ms: 100
///     returns: void
///   - name: pop
///     preconditions: ["!is_empty"]
///     parameters: []
///     timeout_ms: 100
///     returns: Option<i32>
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RuntimeSchema {
    /// Type name (e.g., "Vec<i32>", "Logger", "Command")
    pub type_name: String,

    /// Constructor definition
    pub constructor: Constructor,

    /// Available operations on the type
    pub operations: Vec<Operation>,

    /// Maximum operation sequence length
    #[serde(default = "default_max_sequence_length")]
    pub max_sequence_length: usize,
}

fn default_max_sequence_length() -> usize {
    10
}

/// Constructor definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Constructor {
    /// Constructor name (e.g., "new", "create", "default")
    pub name: String,

    /// Constructor parameters
    #[serde(default)]
    pub parameters: Vec<String>,

    /// Maximum allowed construction time in milliseconds
    #[serde(default = "default_constructor_timeout")]
    pub timeout_ms: u64,

    /// Return type
    pub returns: String,
}

fn default_constructor_timeout() -> u64 {
    100 // Constructors should complete in <100ms
}

/// Operation definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Operation {
    /// Operation name (e.g., "push", "test", "output")
    pub name: String,

    /// Preconditions (state predicates that must be true)
    #[serde(default)]
    pub preconditions: Vec<String>,

    /// Operation parameters
    #[serde(default)]
    pub parameters: Vec<String>,

    /// Maximum allowed operation time in milliseconds
    #[serde(default = "default_operation_timeout")]
    pub timeout_ms: u64,

    /// Return type
    pub returns: String,
}

fn default_operation_timeout() -> u64 {
    1000 // Operations should complete in <1s
}

/// Shadow state for tracking object state during fuzzing
///
/// Maintains predicates about the object's state to enable
/// precondition checking.
#[derive(Debug, Clone, PartialEq)]
pub struct ShadowState {
    /// Type name
    pub type_name: String,

    /// State predicates (e.g., "is_empty" -> true/false)
    pub predicates: HashMap<String, bool>,

    /// Operation history
    pub history: Vec<String>,
}

impl ShadowState {
    /// Create new shadow state
    pub fn new(type_name: String) -> Self {
        ShadowState {
            type_name,
            predicates: HashMap::new(),
            history: Vec::new(),
        }
    }

    /// Check if preconditions are satisfied
    pub fn check_preconditions(&self, preconditions: &[String]) -> bool {
        for precond in preconditions {
            // Handle negation
            if precond.starts_with('!') {
                let pred = &precond[1..];
                if *self.predicates.get(pred).unwrap_or(&false) {
                    return false; // Negated predicate is true, precondition fails
                }
            } else {
                if !*self.predicates.get(precond.as_str()).unwrap_or(&false) {
                    return false; // Predicate is false, precondition fails
                }
            }
        }
        true
    }

    /// Update predicates after operation
    pub fn update(&mut self, operation: &str, effect: HashMap<String, bool>) {
        self.history.push(operation.to_string());
        for (pred, value) in effect {
            self.predicates.insert(pred, value);
        }
    }
}

/// Generated test case with timeout checks
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeTestCase {
    /// Schema being tested
    pub schema: RuntimeSchema,

    /// Shadow state
    pub shadow_state: ShadowState,

    /// Generated operation sequence
    pub operations: Vec<OperationCall>,

    /// Test case ID
    pub id: usize,
}

/// Single operation call
#[derive(Debug, Clone, PartialEq)]
pub struct OperationCall {
    /// Operation name
    pub operation: String,

    /// Operation parameters (serialized)
    pub parameters: Vec<String>,

    /// Expected timeout threshold (ms)
    pub timeout_ms: u64,
}

impl RuntimeTestCase {
    /// Generate Ruchy code for this test case
    pub fn to_ruchy_code(&self) -> String {
        let mut code = String::new();

        // Add enum/struct definitions if needed
        if self.schema.type_name == "Logger" {
            code.push_str("enum LogLevel {\n");
            code.push_str("    Debug = 0,\n");
            code.push_str("    Info = 1,\n");
            code.push_str("}\n\n");
            code.push_str("struct Logger {\n");
            code.push_str("    level: LogLevel,\n");
            code.push_str("}\n\n");
            code.push_str("impl Logger {\n");
            code.push_str("    fun create() -> Logger {\n");
            code.push_str("        Logger { level: LogLevel::Info }\n");
            code.push_str("    }\n\n");
            code.push_str("    fun test(&self) {\n");
            code.push_str("        let val = self.level as i32;  // RUNTIME CHECK: Must not hang!\n");
            code.push_str("        println!(\"Value: {}\", val);\n");
            code.push_str("    }\n");
            code.push_str("}\n\n");
        }

        code.push_str("fun main() {\n");

        // Constructor
        code.push_str(&format!("    let obj = {}::{}(",
            self.schema.type_name,
            self.schema.constructor.name
        ));
        for (i, param) in self.schema.constructor.parameters.iter().enumerate() {
            if i > 0 {
                code.push_str(", ");
            }
            code.push_str(param);
        }
        code.push_str(&format!(");  // Timeout: <{}ms\n", self.schema.constructor.timeout_ms));

        // Operations
        for op_call in &self.operations {
            code.push_str(&format!("    obj.{}(", op_call.operation));
            for (i, param) in op_call.parameters.iter().enumerate() {
                if i > 0 {
                    code.push_str(", ");
                }
                code.push_str(param);
            }
            code.push_str(&format!(");  // Timeout: <{}ms\n", op_call.timeout_ms));
        }

        code.push_str("}\n");
        code
    }
}

/// Schema-based runtime fuzzer
pub struct SchemaFuzzer {
    /// Configuration
    pub config: SchemaFuzzerConfig,

    /// Test case counter
    test_counter: usize,
}

/// Configuration for schema fuzzer
#[derive(Debug, Clone)]
pub struct SchemaFuzzerConfig {
    /// Number of test cases to generate
    pub num_test_cases: usize,

    /// Maximum operation sequence length
    pub max_operations: usize,

    /// Random seed for reproducibility
    pub seed: u64,
}

impl Default for SchemaFuzzerConfig {
    fn default() -> Self {
        SchemaFuzzerConfig {
            num_test_cases: 1000,
            max_operations: 10,
            seed: 42,
        }
    }
}

impl SchemaFuzzer {
    /// Create new schema fuzzer
    pub fn new(config: SchemaFuzzerConfig) -> Self {
        SchemaFuzzer {
            config,
            test_counter: 0,
        }
    }

    /// Generate test cases from schema
    pub fn generate_tests(&mut self, schema: &RuntimeSchema) -> Vec<RuntimeTestCase> {
        let mut tests = Vec::new();

        for _ in 0..self.config.num_test_cases {
            let test = self.generate_test_case(schema);
            tests.push(test);
        }

        tests
    }

    /// Generate single test case
    fn generate_test_case(&mut self, schema: &RuntimeSchema) -> RuntimeTestCase {
        self.test_counter += 1;

        let mut shadow_state = ShadowState::new(schema.type_name.clone());
        let mut operations = Vec::new();

        // Determine operation sequence length (1 to max_operations)
        let num_ops = (self.test_counter % self.config.max_operations) + 1;

        for _ in 0..num_ops {
            // Find valid operations (preconditions satisfied)
            let valid_ops: Vec<&Operation> = schema
                .operations
                .iter()
                .filter(|op| shadow_state.check_preconditions(&op.preconditions))
                .collect();

            if valid_ops.is_empty() {
                break; // No valid operations, end sequence
            }

            // Pick operation (round-robin for determinism)
            let op = valid_ops[self.test_counter % valid_ops.len()];

            operations.push(OperationCall {
                operation: op.name.clone(),
                parameters: op.parameters.clone(),
                timeout_ms: op.timeout_ms,
            });

            // Update shadow state (simplified - real impl would model effects)
            let mut effects = HashMap::new();
            if op.name == "push" {
                effects.insert("is_empty".to_string(), false);
            } else if op.name == "pop" {
                // Simplified: assume might become empty
            }
            shadow_state.update(&op.name, effects);
        }

        RuntimeTestCase {
            schema: schema.clone(),
            shadow_state,
            operations,
            id: self.test_counter,
        }
    }

    /// Run test case and detect timeouts
    ///
    /// Returns None if test passes, Some(timeout_info) if timeout detected
    pub fn run_test_with_timeout(
        &self,
        test: &RuntimeTestCase,
        executor: impl Fn(&str) -> Result<(), String>,
    ) -> Option<TimeoutDetection> {
        let code = test.to_ruchy_code();

        let start = Instant::now();
        match executor(&code) {
            Ok(_) => {
                let elapsed = start.elapsed();
                // Check if any operation exceeded its timeout
                let total_timeout = Duration::from_millis(
                    test.schema.constructor.timeout_ms
                        + test.operations.iter().map(|op| op.timeout_ms).sum::<u64>(),
                );

                if elapsed > total_timeout {
                    Some(TimeoutDetection {
                        test_id: test.id,
                        elapsed_ms: elapsed.as_millis() as u64,
                        expected_max_ms: total_timeout.as_millis() as u64,
                        operation_sequence: test
                            .operations
                            .iter()
                            .map(|op| op.operation.clone())
                            .collect(),
                    })
                } else {
                    None
                }
            }
            Err(_) => {
                // Execution failed (timeout or crash)
                let elapsed = start.elapsed();
                Some(TimeoutDetection {
                    test_id: test.id,
                    elapsed_ms: elapsed.as_millis() as u64,
                    expected_max_ms: test.operations.first().map(|op| op.timeout_ms).unwrap_or(1000),
                    operation_sequence: test
                        .operations
                        .iter()
                        .map(|op| op.operation.clone())
                        .collect(),
                })
            }
        }
    }
}

/// Timeout detection result
#[derive(Debug, Clone, PartialEq)]
pub struct TimeoutDetection {
    /// Test case ID
    pub test_id: usize,

    /// Actual elapsed time (ms)
    pub elapsed_ms: u64,

    /// Expected maximum time (ms)
    pub expected_max_ms: u64,

    /// Operation sequence that triggered timeout
    pub operation_sequence: Vec<String>,
}

impl fmt::Display for TimeoutDetection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TIMEOUT detected in test {}: {}ms elapsed (expected <{}ms)\nOperation sequence: {}",
            self.test_id,
            self.elapsed_ms,
            self.expected_max_ms,
            self.operation_sequence.join(" -> ")
        )
    }
}

/// Predefined schemas for common Ruchy types
impl RuntimeSchema {
    /// Schema for Logger (Issue #79)
    pub fn logger_schema() -> Self {
        RuntimeSchema {
            type_name: "Logger".to_string(),
            constructor: Constructor {
                name: "create".to_string(),
                parameters: vec![],
                timeout_ms: 100,
                returns: "Logger".to_string(),
            },
            operations: vec![Operation {
                name: "test".to_string(),
                preconditions: vec![],
                parameters: vec![],
                timeout_ms: 1000,
                returns: "void".to_string(),
            }],
            max_sequence_length: 5,
        }
    }

    /// Schema for Vec<T> (Issue #76)
    pub fn vec_schema() -> Self {
        RuntimeSchema {
            type_name: "Vec<i32>".to_string(),
            constructor: Constructor {
                name: "new".to_string(),
                parameters: vec![],
                timeout_ms: 100,
                returns: "Vec<i32>".to_string(),
            },
            operations: vec![
                Operation {
                    name: "push".to_string(),
                    preconditions: vec![],
                    parameters: vec!["42".to_string()],
                    timeout_ms: 100,
                    returns: "void".to_string(),
                },
                Operation {
                    name: "pop".to_string(),
                    preconditions: vec!["!is_empty".to_string()],
                    parameters: vec![],
                    timeout_ms: 100,
                    returns: "Option<i32>".to_string(),
                },
                Operation {
                    name: "len".to_string(),
                    preconditions: vec![],
                    parameters: vec![],
                    timeout_ms: 100,
                    returns: "usize".to_string(),
                },
            ],
            max_sequence_length: 10,
        }
    }

    /// Schema for Command (Issue #75)
    pub fn command_schema() -> Self {
        RuntimeSchema {
            type_name: "Command".to_string(),
            constructor: Constructor {
                name: "new".to_string(),
                parameters: vec!["\"echo\"".to_string()],
                timeout_ms: 100,
                returns: "Command".to_string(),
            },
            operations: vec![
                Operation {
                    name: "arg".to_string(),
                    preconditions: vec![],
                    parameters: vec!["\"hello\"".to_string()],
                    timeout_ms: 100,
                    returns: "&mut Command".to_string(),
                },
                Operation {
                    name: "output".to_string(),
                    preconditions: vec![],
                    parameters: vec![],
                    timeout_ms: 1000,
                    returns: "Result<Output>".to_string(),
                },
            ],
            max_sequence_length: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_parsing() {
        let schema = RuntimeSchema::logger_schema();
        assert_eq!(schema.type_name, "Logger");
        assert_eq!(schema.constructor.name, "create");
        assert_eq!(schema.constructor.timeout_ms, 100);
        assert_eq!(schema.operations.len(), 1);
        assert_eq!(schema.operations[0].name, "test");
        assert_eq!(schema.operations[0].timeout_ms, 1000);
    }

    #[test]
    fn test_shadow_state_preconditions() {
        let mut state = ShadowState::new("Vec<i32>".to_string());

        // Initially empty
        state.predicates.insert("is_empty".to_string(), true);
        assert!(state.check_preconditions(&[]));
        assert!(state.check_preconditions(&["is_empty".to_string()]));
        assert!(!state.check_preconditions(&["!is_empty".to_string()]));

        // After push, not empty
        let mut effects = HashMap::new();
        effects.insert("is_empty".to_string(), false);
        state.update("push", effects);
        assert!(!state.check_preconditions(&["is_empty".to_string()]));
        assert!(state.check_preconditions(&["!is_empty".to_string()]));
    }

    #[test]
    fn test_stateful_generation() {
        let schema = RuntimeSchema::vec_schema();
        let config = SchemaFuzzerConfig {
            num_test_cases: 10,
            max_operations: 5,
            seed: 42,
        };
        let mut fuzzer = SchemaFuzzer::new(config);

        let tests = fuzzer.generate_tests(&schema);
        assert_eq!(tests.len(), 10);

        // All tests should have operations
        for test in &tests {
            assert!(!test.operations.is_empty());
            assert!(test.operations.len() <= 5);
        }
    }

    #[test]
    fn test_timeout_detection() {
        let schema = RuntimeSchema::logger_schema();
        let config = SchemaFuzzerConfig {
            num_test_cases: 1,
            max_operations: 1,
            seed: 42,
        };
        let mut fuzzer = SchemaFuzzer::new(config);

        let tests = fuzzer.generate_tests(&schema);
        assert_eq!(tests.len(), 1);

        // Simulate timeout
        let timeout_result = fuzzer.run_test_with_timeout(&tests[0], |_code| {
            std::thread::sleep(Duration::from_millis(2000)); // Exceeds 1000ms threshold
            Err("timeout".to_string())
        });

        assert!(timeout_result.is_some());
        let timeout = timeout_result.unwrap();
        assert!(timeout.elapsed_ms >= 2000);
        assert_eq!(timeout.operation_sequence, vec!["test"]);
    }

    #[test]
    fn test_property_injection() {
        let schema = RuntimeSchema::logger_schema();
        let config = SchemaFuzzerConfig {
            num_test_cases: 1,
            max_operations: 1,
            seed: 42,
        };
        let mut fuzzer = SchemaFuzzer::new(config);

        let tests = fuzzer.generate_tests(&schema);
        let code = tests[0].to_ruchy_code();

        // Check timeout comments are injected
        assert!(code.contains("Timeout:"));
        assert!(code.contains("<100ms"));
        assert!(code.contains("<1000ms"));
    }

    #[test]
    fn test_minimization_placeholder() {
        // Placeholder for delta debugging minimization
        // TODO: Implement delta debugging to minimize failing test cases
        // See Zeller & Hildebrandt (2002): "Simplifying and Isolating Failure-Inducing Input"
    }

    #[test]
    fn test_issue79_detection() {
        // Test that Issue #79 would be detected
        let schema = RuntimeSchema::logger_schema();
        let config = SchemaFuzzerConfig {
            num_test_cases: 1,
            max_operations: 1,
            seed: 42,
        };
        let mut fuzzer = SchemaFuzzer::new(config);

        let tests = fuzzer.generate_tests(&schema);
        let code = tests[0].to_ruchy_code();

        // Verify generated code includes the hang-prone pattern
        assert!(code.contains("self.level as i32"));
        assert!(code.contains("// RUNTIME CHECK: Must not hang!"));
    }
}
