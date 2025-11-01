// DEBUGGER-047: Performance Profiler with Flame Graphs
//
// GREEN Phase: Minimal implementation to make tests pass
//
// Mission: Create performance profiler to identify bottlenecks
// Design Pattern: Inspired by paiml-mcp-agent-toolkit PerformanceProfiler
//
// Architecture:
// - PerformanceProfiler: Main profiler tracking timing and memory
// - ProfileReport: Aggregated profiling data with bottleneck analysis
// - FlameGraphNode: Hierarchical call tree for flame graph visualization

use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;

/// Performance profiler for tracking parse/eval operations
#[derive(Debug, Clone)]
pub struct PerformanceProfiler {
    data: Rc<RefCell<ProfileData>>,
}

/// Internal profiling data
#[derive(Debug, Clone)]
struct ProfileData {
    enabled: bool,
    parse_start: Option<Instant>,
    parse_time_ns: u128,
    parse_operations: Vec<Operation>,
    eval_start: Option<Instant>,
    eval_time_ns: u128,
    eval_operations: Vec<Operation>,
    function_calls: HashMap<String, usize>,
    memory_allocated_bytes: usize,
    call_stack: Vec<StackFrame>,
}

/// Single operation timing
#[derive(Debug, Clone)]
struct Operation {
    name: String,
    duration_ns: u128,
    depth: usize,
}

/// Call stack frame for flame graph
#[derive(Debug, Clone)]
struct StackFrame {
    function_name: String,
    start_time: Instant,
    #[allow(dead_code)] // Used in flame graph generation
    depth: usize,
}

/// Profiling report with bottleneck analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileReport {
    /// Total time spent parsing (nanoseconds)
    pub parse_time_ns: u128,
    /// Total time spent evaluating (nanoseconds)
    pub eval_time_ns: u128,
    /// Total time (parse + eval) in nanoseconds
    pub total_time_ns: u128,
    /// List of timed parse operations
    pub parse_operations: Vec<OperationReport>,
    /// List of timed eval operations
    pub eval_operations: Vec<OperationReport>,
    /// Function call counts
    pub function_calls: HashMap<String, usize>,
    /// Total memory allocated (bytes)
    pub memory_allocated_bytes: usize,
    /// Identified performance bottlenecks
    pub bottlenecks: Vec<Bottleneck>,
}

/// Serializable operation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationReport {
    /// Operation name
    pub name: String,
    /// Operation duration (nanoseconds)
    pub duration_ns: u128,
    /// Call depth (0 = top level)
    pub depth: usize,
}

/// Identified performance bottleneck
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bottleneck {
    /// Operation name (e.g., function name)
    pub operation: String,
    /// Total duration across all calls (nanoseconds)
    pub duration_ns: u128,
    /// Percentage of total execution time
    pub percentage: f64,
    /// Number of times this operation was called
    pub call_count: usize,
}

impl PerformanceProfiler {
    /// Create a new enabled performance profiler
    pub fn new() -> Self {
        Self {
            data: Rc::new(RefCell::new(ProfileData {
                enabled: true,
                parse_start: None,
                parse_time_ns: 0,
                parse_operations: Vec::new(),
                eval_start: None,
                eval_time_ns: 0,
                eval_operations: Vec::new(),
                function_calls: HashMap::new(),
                memory_allocated_bytes: 0,
                call_stack: Vec::new(),
            })),
        }
    }

    /// Check if profiling is enabled
    pub fn is_enabled(&self) -> bool {
        self.data.borrow().enabled
    }

    /// Start timing parse operation
    pub fn start_parse(&self) {
        let mut data = self.data.borrow_mut();
        data.parse_start = Some(Instant::now());
    }

    /// End timing parse operation
    pub fn end_parse(&self) {
        let mut data = self.data.borrow_mut();
        if let Some(start) = data.parse_start.take() {
            data.parse_time_ns = start.elapsed().as_nanos();
        }
    }

    /// Record a parse operation
    pub fn record_parse_operation(&self, name: String, duration_ns: u128) {
        let mut data = self.data.borrow_mut();
        data.parse_operations.push(Operation {
            name,
            duration_ns,
            depth: 0,
        });
    }

    /// Start timing eval operation
    pub fn start_eval(&self) {
        let mut data = self.data.borrow_mut();
        data.eval_start = Some(Instant::now());
    }

    /// End timing eval operation
    pub fn end_eval(&self) {
        let mut data = self.data.borrow_mut();
        if let Some(start) = data.eval_start.take() {
            data.eval_time_ns += start.elapsed().as_nanos();
        }
    }

    /// Record an eval operation
    pub fn record_eval_operation(&self, name: String, duration_ns: u128) {
        let mut data = self.data.borrow_mut();
        let depth = data.call_stack.len();
        data.eval_operations.push(Operation {
            name,
            duration_ns,
            depth,
        });
    }

    /// Record a function call
    pub fn record_function_call(&self, function_name: &str) {
        let mut data = self.data.borrow_mut();
        *data
            .function_calls
            .entry(function_name.to_string())
            .or_insert(0) += 1;
    }

    /// Push function onto call stack
    pub fn push_call_stack(&self, function_name: String) {
        let mut data = self.data.borrow_mut();
        let depth = data.call_stack.len();
        data.call_stack.push(StackFrame {
            function_name,
            start_time: Instant::now(),
            depth,
        });
    }

    /// Pop function from call stack and record duration
    pub fn pop_call_stack(&self) -> Option<(String, u128)> {
        let mut data = self.data.borrow_mut();
        if let Some(frame) = data.call_stack.pop() {
            let duration = frame.start_time.elapsed().as_nanos();
            Some((frame.function_name, duration))
        } else {
            None
        }
    }

    /// Record memory allocation
    pub fn record_memory_allocation(&self, bytes: usize) {
        let mut data = self.data.borrow_mut();
        data.memory_allocated_bytes += bytes;
    }

    /// Generate profiling report
    pub fn report(&self) -> ProfileReport {
        let data = self.data.borrow();

        let parse_ops: Vec<OperationReport> = data
            .parse_operations
            .iter()
            .map(|op| OperationReport {
                name: op.name.clone(),
                duration_ns: op.duration_ns,
                depth: op.depth,
            })
            .collect();

        let eval_ops: Vec<OperationReport> = data
            .eval_operations
            .iter()
            .map(|op| OperationReport {
                name: op.name.clone(),
                duration_ns: op.duration_ns,
                depth: op.depth,
            })
            .collect();

        let total_time_ns = data.parse_time_ns + data.eval_time_ns;

        // Identify bottlenecks
        let bottlenecks =
            identify_bottlenecks(&data.function_calls, &data.eval_operations, total_time_ns);

        ProfileReport {
            parse_time_ns: data.parse_time_ns,
            eval_time_ns: data.eval_time_ns,
            total_time_ns,
            parse_operations: parse_ops,
            eval_operations: eval_ops,
            function_calls: data.function_calls.clone(),
            memory_allocated_bytes: data.memory_allocated_bytes,
            bottlenecks,
        }
    }
}

impl Default for PerformanceProfiler {
    fn default() -> Self {
        Self::new()
    }
}

impl ProfileReport {
    /// Get identified bottlenecks
    pub fn bottlenecks(&self) -> &[Bottleneck] {
        &self.bottlenecks
    }

    /// Export as JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }

    /// Export as flame graph JSON format
    pub fn to_flame_graph_json(&self) -> String {
        // Create root node
        let mut root = serde_json::json!({
            "name": "root",
            "value": self.total_time_ns,
            "children": []
        });

        // Add parse operations
        if self.parse_time_ns > 0 {
            root["children"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!({
                    "name": "parse",
                    "value": self.parse_time_ns,
                    "children": []
                }));
        }

        // Add eval operations by function
        let mut eval_node = serde_json::json!({
            "name": "eval",
            "value": self.eval_time_ns,
            "children": []
        });

        for (func_name, call_count) in &self.function_calls {
            // Calculate total time for this function
            let total_time: u128 = self
                .eval_operations
                .iter()
                .filter(|op| op.name.contains(func_name))
                .map(|op| op.duration_ns)
                .sum();

            if total_time > 0 {
                eval_node["children"]
                    .as_array_mut()
                    .unwrap()
                    .push(serde_json::json!({
                        "name": format!("{} ({}x)", func_name, call_count),
                        "value": total_time,
                    }));
            }
        }

        root["children"].as_array_mut().unwrap().push(eval_node);

        serde_json::to_string_pretty(&root).unwrap_or_else(|_| "{}".to_string())
    }
}

/// Identify performance bottlenecks
fn identify_bottlenecks(
    function_calls: &HashMap<String, usize>,
    eval_operations: &[Operation],
    total_time_ns: u128,
) -> Vec<Bottleneck> {
    let mut bottlenecks = Vec::new();

    if total_time_ns == 0 {
        return bottlenecks;
    }

    // Aggregate time by function
    let mut function_times: HashMap<String, u128> = HashMap::new();

    for op in eval_operations {
        // Extract function name from operation name
        for func_name in function_calls.keys() {
            if op.name.contains(func_name) {
                *function_times.entry(func_name.clone()).or_insert(0) += op.duration_ns;
            }
        }
    }

    // Create bottleneck entries
    for (func_name, total_func_time) in function_times {
        let percentage = (total_func_time as f64 / total_time_ns as f64) * 100.0;
        let call_count = *function_calls.get(&func_name).unwrap_or(&0);

        bottlenecks.push(Bottleneck {
            operation: func_name,
            duration_ns: total_func_time,
            percentage,
            call_count,
        });
    }

    // Sort by duration (highest first)
    bottlenecks.sort_by(|a, b| b.duration_ns.cmp(&a.duration_ns));

    bottlenecks
}
