//! RuchyRuchy Debugging Tools CLI
//!
//! This binary provides a command-line interface for the RuchyRuchy debugging
//! validation tools. It wraps the Ruchy-based validation scripts and makes them
//! easily accessible via the `ruchydbg` command.

use std::env;
use std::path::PathBuf;
use std::process::{exit, Command};
use std::time::Instant;

const VERSION: &str = env!("CARGO_PKG_VERSION");

// Exit codes
const EXIT_SUCCESS: i32 = 0;
const EXIT_TIMEOUT: i32 = 124;
const EXIT_ERROR: i32 = 1;

// Default timeout in milliseconds (5 seconds)
const DEFAULT_TIMEOUT_MS: u64 = 5000;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Parse command
    let command = if args.len() > 1 {
        args[1].as_str()
    } else {
        "validate"
    };

    match command {
        "run" => run_ruchy_file(&args),
        "profile" => run_profile(&args),
        "detect" => run_detect(&args),
        "regression" => run_regression(&args),
        "validate" | "test" => run_validation(),
        "version" | "--version" | "-v" => {
            println!("ruchydbg {VERSION}");
            exit(0);
        }
        "help" | "--help" | "-h" => {
            print_help();
            exit(0);
        }
        _ => {
            eprintln!("Unknown command: {command}");
            print_help();
            exit(1);
        }
    }
}

fn run_ruchy_file(args: &[String]) {
    // Parse arguments: ruchydbg run <file> [--timeout <ms>] [--trace]

    // Check for help first
    if args.len() >= 3 && (args[2] == "--help" || args[2] == "-h") {
        print_run_help();
        exit(EXIT_SUCCESS);
    }

    if args.len() < 3 {
        eprintln!("Error: Missing file argument");
        eprintln!("Usage: ruchydbg run <file> [--timeout <ms>] [--trace]");
        exit(EXIT_ERROR);
    }

    let file_path = &args[2];

    // Parse timeout and trace flags
    let mut timeout_ms = DEFAULT_TIMEOUT_MS;
    let mut enable_trace = false;

    let mut i = 3;
    while i < args.len() {
        match args[i].as_str() {
            "--timeout" => {
                if i + 1 < args.len() {
                    timeout_ms = args[i + 1].parse::<u64>().unwrap_or(DEFAULT_TIMEOUT_MS);
                    i += 2;
                } else {
                    eprintln!("Error: --timeout requires a value");
                    exit(EXIT_ERROR);
                }
            }
            "--trace" => {
                enable_trace = true;
                i += 1;
            }
            _ => {
                eprintln!("Error: Unknown flag: {}", args[i]);
                print_run_help();
                exit(EXIT_ERROR);
            }
        }
    }

    // Check if file exists
    if !PathBuf::from(file_path).exists() {
        eprintln!("Error: File not found: {}", file_path);
        exit(EXIT_ERROR);
    }

    // Check if ruchy is available
    let ruchy_check = Command::new("ruchy").arg("--version").output();

    if ruchy_check.is_err() {
        eprintln!("❌ Error: 'ruchy' command not found in PATH");
        eprintln!("Please install Ruchy: https://github.com/paiml/ruchy");
        exit(EXIT_ERROR);
    }

    // Run with timeout
    println!("🔍 Running: {}", file_path);
    println!("⏱️  Timeout: {}ms", timeout_ms);
    if enable_trace {
        println!("🔍 Type-aware tracing: enabled");
    }
    println!();

    let start = Instant::now();

    // Use timeout command if available (Unix-like systems)
    #[cfg(unix)]
    let mut cmd = Command::new("timeout");
    #[cfg(unix)]
    {
        // timeout command uses seconds, convert ms to seconds
        let timeout_secs = (timeout_ms as f64) / 1000.0;
        cmd.arg(format!("{}", timeout_secs));
        cmd.arg("ruchy");
        if enable_trace {
            cmd.arg("--trace");
        }
        cmd.arg("run");
        cmd.arg(file_path);
    }

    // On Windows, just run ruchy (timeout not available)
    #[cfg(not(unix))]
    let mut cmd = Command::new("ruchy");
    #[cfg(not(unix))]
    {
        if enable_trace {
            cmd.arg("--trace");
        }
        cmd.arg("run");
        cmd.arg(file_path);
    }

    // Set RUCHY_TRACE environment variable if tracing is enabled
    if enable_trace {
        cmd.env("RUCHY_TRACE", "1");
    }

    let status = cmd.status();

    let elapsed = start.elapsed();

    match status {
        Ok(exit_status) => {
            println!();
            println!("⏱️  Execution time: {}ms", elapsed.as_millis());

            if let Some(code) = exit_status.code() {
                if code == EXIT_TIMEOUT {
                    eprintln!("❌ TIMEOUT after {}ms", timeout_ms);
                    eprintln!("The program exceeded the timeout threshold.");
                    exit(EXIT_TIMEOUT);
                } else if code == EXIT_SUCCESS {
                    println!("✅ SUCCESS");
                    exit(EXIT_SUCCESS);
                } else {
                    eprintln!("❌ FAILED with exit code: {}", code);
                    exit(code);
                }
            } else {
                eprintln!("❌ FAILED (killed by signal)");
                exit(EXIT_ERROR);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to execute: {}", e);
            exit(EXIT_ERROR);
        }
    }
}

fn print_run_help() {
    println!("ruchydbg run - Execute Ruchy code with timeout detection and type-aware tracing");
    println!();
    println!("USAGE:");
    println!("    ruchydbg run <file> [OPTIONS]");
    println!();
    println!("ARGUMENTS:");
    println!("    <file>           Path to .ruchy file to execute");
    println!();
    println!("OPTIONS:");
    println!("    --timeout <ms>   Timeout in milliseconds (default: 5000ms)");
    println!("    --trace          Enable type-aware tracing (shows argument/return types)");
    println!();
    println!("EXIT CODES:");
    println!("    0    Success");
    println!("    124  Timeout");
    println!("    1+   Other error");
    println!();
    println!("EXAMPLES:");
    println!("    # Basic execution with timeout detection");
    println!("    ruchydbg run test.ruchy");
    println!();
    println!("    # With custom timeout");
    println!("    ruchydbg run test.ruchy --timeout 1000");
    println!();
    println!("    # With type-aware tracing (Ruchy v3.149.0+)");
    println!("    ruchydbg run test.ruchy --trace");
    println!();
    println!("    # Combined: timeout + tracing");
    println!("    ruchydbg run test.ruchy --timeout 5000 --trace");
    println!();
    println!("TYPE-AWARE TRACING (Ruchy v3.149.0+):");
    println!("    Shows types of function arguments and return values:");
    println!("      TRACE: → square(5: integer)");
    println!("      TRACE: ← square = 25: integer");
}

fn run_profile(args: &[String]) {
    // DEBUGGER-041: Stack depth profiler CLI integration
    // Usage: ruchydbg profile --stack <file>

    // Check for help
    if args.len() >= 3 && (args[2] == "--help" || args[2] == "-h") {
        print_profile_help();
        exit(EXIT_SUCCESS);
    }

    // Parse subcommand
    if args.len() < 3 {
        eprintln!("Error: Missing profile type");
        eprintln!("Usage: ruchydbg profile <--stack> <file>");
        print_profile_help();
        exit(EXIT_ERROR);
    }

    let profile_type = &args[2];

    match profile_type.as_str() {
        "--stack" => run_stack_profiler(&args[3..]),
        _ => {
            eprintln!("Error: Unknown profile type: {}", profile_type);
            eprintln!("Available types: --stack");
            print_profile_help();
            exit(EXIT_ERROR);
        }
    }
}

fn run_stack_profiler(args: &[String]) {
    // DEBUGGER-041: Stack depth profiler
    // Usage: ruchydbg profile --stack <file>

    if args.is_empty() {
        eprintln!("Error: Missing file argument");
        eprintln!("Usage: ruchydbg profile --stack <file>");
        exit(EXIT_ERROR);
    }

    let file_path = &args[0];

    // Check if file exists
    let path = PathBuf::from(file_path);
    if !path.exists() {
        eprintln!("Error: File not found: {}", file_path);
        exit(EXIT_ERROR);
    }

    // Read file
    let code = match std::fs::read_to_string(&path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            exit(EXIT_ERROR);
        }
    };

    // Parse
    use ruchyruchy::interpreter::parser::Parser;
    let mut parser = Parser::new(&code);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
            exit(EXIT_ERROR);
        }
    };

    // Execute with profiling enabled
    use ruchyruchy::interpreter::evaluator::Evaluator;
    let mut eval = Evaluator::new().with_profiling();

    for statement in ast.nodes() {
        if let Err(e) = eval.eval(statement) {
            eprintln!("Evaluation error: {:?}", e);
            exit(EXIT_ERROR);
        }
    }

    // Get profiling data
    match eval.take_profiling_data() {
        Some(profile) => {
            // Display report
            println!("\n=== Stack Depth Profile ===\n");
            println!("File: {}", file_path);
            println!("Max depth: {}", profile.max_depth);
            println!("Total calls: {}\n", profile.total_calls);

            if !profile.call_counts.is_empty() {
                println!("Call counts:");
                let mut counts: Vec<_> = profile.call_counts.iter().collect();
                counts.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count descending
                for (func, count) in counts {
                    println!("  {}: {} calls", func, count);
                }
            }

            if !profile.deepest_stack.is_empty() {
                println!("\nDeepest call stack:");
                for (i, func) in profile.deepest_stack.iter().enumerate() {
                    println!("  {}. {}", i + 1, func);
                }
            }

            println!();
            exit(EXIT_SUCCESS);
        }
        None => {
            eprintln!("Error: Profiling was not enabled");
            exit(EXIT_ERROR);
        }
    }
}

fn print_profile_help() {
    println!("USAGE:");
    println!("    ruchydbg profile <TYPE> <file>");
    println!();
    println!("TYPES:");
    println!("    --stack       Stack depth profiler (max depth, call counts, call tree)");
    println!();
    println!("EXAMPLES:");
    println!("    ruchydbg profile --stack factorial.ruchy");
    println!();
    println!("OUTPUT:");
    println!("    - Maximum call depth reached");
    println!("    - Total function calls executed");
    println!("    - Per-function call counts (sorted)");
    println!("    - Call stack at maximum depth");
}

fn run_detect(args: &[String]) {
    // DEBUGGER-042: Pathological input detector
    // Usage: ruchydbg detect <file> [--threshold <N>]

    // Check for help
    if args.len() > 2 && (args[2] == "--help" || args[2] == "-h") {
        print_detect_help();
        exit(EXIT_SUCCESS);
    }

    if args.len() < 3 {
        eprintln!("Error: Missing file argument");
        eprintln!("Usage: ruchydbg detect <file> [--threshold <N>]");
        print_detect_help();
        exit(EXIT_ERROR);
    }

    let file_path = &args[2];

    // Parse threshold flag (optional)
    let threshold = if args.len() >= 5 && args[3] == "--threshold" {
        args[4].parse::<f64>().unwrap_or(10.0)
    } else {
        10.0 // Default threshold
    };

    // Check if file exists
    let path = PathBuf::from(file_path);
    if !path.exists() {
        eprintln!("Error: File not found: {}", file_path);
        exit(EXIT_ERROR);
    }

    // Read file
    let code = match std::fs::read_to_string(&path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            exit(EXIT_ERROR);
        }
    };

    // Create detector
    use ruchyruchy::interpreter::pathological_detector::{
        PathologicalCategory, PathologicalDetector,
    };
    let detector = PathologicalDetector::with_threshold(threshold);

    // Auto-detect category based on code patterns
    let category = if code.contains("((") || code.contains("))") {
        PathologicalCategory::ParserStress
    } else if code.contains("let ") && code.lines().filter(|l| l.contains("let ")).count() > 10 {
        PathologicalCategory::EvaluatorStress
    } else {
        PathologicalCategory::ParserStress // Default
    };

    // Run detection
    let result = detector.detect(&code, category);

    // Display report
    println!("\n=== Pathological Input Detection ===\n");
    println!("File: {}", file_path);
    println!("Category: {:?}", result.category);
    println!("Threshold: {:.1}x", threshold);
    println!();
    println!("Performance:");
    println!("  Baseline: {:.2} µs", result.baseline_time_us);
    println!("  Actual: {:.2} µs", result.actual_time_us);
    println!("  Slowdown: {:.2}x", result.slowdown_factor);
    println!();

    if result.is_pathological {
        println!("⚠️  PATHOLOGICAL INPUT DETECTED!");
        println!(
            "    This input causes {:.1}x slowdown vs expected baseline.",
            result.slowdown_factor
        );
        println!("    Consider optimizing or limiting input complexity.");
        exit(EXIT_ERROR); // Non-zero exit for pathological inputs
    } else {
        println!("✅ Performance within acceptable bounds");
        println!(
            "    Slowdown {:.1}x is below {:.1}x threshold.",
            result.slowdown_factor, threshold
        );
        exit(EXIT_SUCCESS);
    }
}

fn run_regression(args: &[String]) {
    // DEBUGGER-043: Regression and hang detector
    // Usage: ruchydbg regression <baseline.ruchy> <current.ruchy> [--runs <N>]

    // Check for help
    if args.len() > 2 && (args[2] == "--help" || args[2] == "-h") {
        print_regression_help();
        exit(EXIT_SUCCESS);
    }

    // Parse command: regression <check-type> <file> [options]
    //   check-type: snapshot, determinism, state, or perf
    if args.len() < 3 {
        eprintln!("Error: Missing check type");
        eprintln!("Usage: ruchydbg regression <check-type> <file> [OPTIONS]");
        print_regression_help();
        exit(EXIT_ERROR);
    }

    let check_type = &args[2];

    match check_type.as_str() {
        "snapshot" => run_regression_snapshot(&args[3..]),
        "determinism" => run_regression_determinism(&args[3..]),
        "state" => run_regression_state(&args[3..]),
        "perf" => run_regression_perf(&args[3..]),
        _ => {
            eprintln!("Error: Unknown check type: {}", check_type);
            eprintln!("Valid types: snapshot, determinism, state, perf");
            print_regression_help();
            exit(EXIT_ERROR);
        }
    }
}

fn run_regression_snapshot(args: &[String]) {
    // Snapshot comparison: compare baseline vs current behavior
    if args.len() < 2 {
        eprintln!("Error: snapshot check requires baseline and current files");
        eprintln!("Usage: ruchydbg regression snapshot <baseline.ruchy> <current.ruchy>");
        exit(EXIT_ERROR);
    }

    let baseline_path = &args[0];
    let current_path = &args[1];

    // Read files
    let baseline_code = match std::fs::read_to_string(baseline_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading baseline file {}: {}", baseline_path, e);
            exit(EXIT_ERROR);
        }
    };

    let current_code = match std::fs::read_to_string(current_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading current file {}: {}", current_path, e);
            exit(EXIT_ERROR);
        }
    };

    // Create detector and snapshots
    use ruchyruchy::interpreter::regression_hang_detector::RegressionHangDetector;
    let detector = RegressionHangDetector::new();

    println!("🔍 Regression Check: Snapshot Comparison");
    println!("Baseline: {}", baseline_path);
    println!("Current:  {}", current_path);
    println!();

    let baseline_snap = detector.create_snapshot(&baseline_code);
    let current_snap = detector.create_snapshot(&current_code);

    let matches = detector.snapshots_match(&baseline_snap, &current_snap);

    println!("Baseline execution: {}ms", baseline_snap.execution_time_ms);
    println!("Current execution:  {}ms", current_snap.execution_time_ms);
    println!();

    if matches {
        println!("✅ NO REGRESSION: Behavior matches baseline");
        println!("   Output: {}", baseline_snap.output);
        exit(EXIT_SUCCESS);
    } else {
        println!("⚠️  REGRESSION DETECTED: Behavior changed!");
        println!("   Baseline output: {}", baseline_snap.output);
        println!("   Current output:  {}", current_snap.output);
        exit(EXIT_ERROR);
    }
}

fn run_regression_determinism(args: &[String]) {
    // Determinism check: run N times and check consistency
    if args.is_empty() {
        eprintln!("Error: determinism check requires a file");
        eprintln!("Usage: ruchydbg regression determinism <file.ruchy> [--runs <N>]");
        exit(EXIT_ERROR);
    }

    let file_path = &args[0];
    let runs = if args.len() >= 3 && args[1] == "--runs" {
        args[2].parse::<usize>().unwrap_or(10)
    } else {
        10
    };

    let code = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            exit(EXIT_ERROR);
        }
    };

    use ruchyruchy::interpreter::regression_hang_detector::RegressionHangDetector;
    let detector = RegressionHangDetector::new();

    println!("🔍 Regression Check: Determinism");
    println!("File: {}", file_path);
    println!("Runs: {}", runs);
    println!();

    let is_deterministic = detector.check_determinism(&code, runs);

    if is_deterministic {
        println!("✅ DETERMINISTIC: All {} runs produced identical results", runs);
        exit(EXIT_SUCCESS);
    } else {
        println!("⚠️  NON-DETERMINISM DETECTED: Runs produced different results!");
        println!("   This indicates a bug in the interpreter or code.");
        exit(EXIT_ERROR);
    }
}

fn run_regression_state(args: &[String]) {
    // State pollution check: ensure variables don't leak between runs
    if args.len() < 2 {
        eprintln!("Error: state check requires two code snippets");
        eprintln!("Usage: ruchydbg regression state <file1.ruchy> <file2.ruchy>");
        exit(EXIT_ERROR);
    }

    let file1_path = &args[0];
    let file2_path = &args[1];

    let code1 = match std::fs::read_to_string(file1_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file1_path, e);
            exit(EXIT_ERROR);
        }
    };

    let code2 = match std::fs::read_to_string(file2_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file2_path, e);
            exit(EXIT_ERROR);
        }
    };

    use ruchyruchy::interpreter::regression_hang_detector::RegressionHangDetector;
    let detector = RegressionHangDetector::new();

    println!("🔍 Regression Check: State Pollution");
    println!("First:  {}", file1_path);
    println!("Second: {}", file2_path);
    println!();

    // Run first code
    let _ = detector.run_isolated(&code1);

    // Run second code - should not see variables from first
    let result = detector.run_isolated(&code2);

    if result.is_err() {
        println!("✅ NO STATE POLLUTION: Second run isolated (expected error)");
        exit(EXIT_SUCCESS);
    } else {
        println!("⚠️  STATE POLLUTION DETECTED: Variables leaked between runs!");
        exit(EXIT_ERROR);
    }
}

fn run_regression_perf(args: &[String]) {
    // Performance regression check: compare execution times
    if args.len() < 2 {
        eprintln!("Error: perf check requires baseline and current files");
        eprintln!("Usage: ruchydbg regression perf <baseline.ruchy> <current.ruchy> [--threshold <N>]");
        exit(EXIT_ERROR);
    }

    let baseline_path = &args[0];
    let current_path = &args[1];

    let threshold = if args.len() >= 4 && args[2] == "--threshold" {
        args[3].parse::<f64>().unwrap_or(2.0)
    } else {
        2.0 // Default 2x slowdown
    };

    let baseline_code = match std::fs::read_to_string(baseline_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading baseline file {}: {}", baseline_path, e);
            exit(EXIT_ERROR);
        }
    };

    let current_code = match std::fs::read_to_string(current_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading current file {}: {}", current_path, e);
            exit(EXIT_ERROR);
        }
    };

    use ruchyruchy::interpreter::regression_hang_detector::RegressionHangDetector;
    let detector = RegressionHangDetector::new();

    println!("🔍 Regression Check: Performance");
    println!("Baseline: {}", baseline_path);
    println!("Current:  {}", current_path);
    println!("Threshold: {}x", threshold);
    println!();

    let baseline_ms = detector.measure_execution_time(&baseline_code);
    let current_ms = detector.measure_execution_time(&current_code);
    let slowdown = detector.detect_performance_regression(baseline_ms, current_ms);

    println!("Baseline: {}ms", baseline_ms);
    println!("Current:  {}ms", current_ms);
    println!("Slowdown: {:.2}x", slowdown);
    println!();

    if slowdown > threshold {
        println!("⚠️  PERFORMANCE REGRESSION: {:.2}x slowdown exceeds {:.2}x threshold!", slowdown, threshold);
        exit(EXIT_ERROR);
    } else {
        println!("✅ NO PERFORMANCE REGRESSION: Within acceptable bounds");
        exit(EXIT_SUCCESS);
    }
}

fn print_regression_help() {
    println!("USAGE:");
    println!("    ruchydbg regression <check-type> <file> [OPTIONS]");
    println!();
    println!("CHECK TYPES:");
    println!("    snapshot       Compare baseline vs current behavior");
    println!("    determinism    Check if code produces consistent results");
    println!("    state          Check for variable leakage between runs");
    println!("    perf           Check for performance regressions");
    println!();
    println!("COMMANDS:");
    println!("    ruchydbg regression snapshot <baseline.ruchy> <current.ruchy>");
    println!("        Compare two versions and detect behavior changes");
    println!();
    println!("    ruchydbg regression determinism <file.ruchy> [--runs <N>]");
    println!("        Run code N times and check for consistency (default: 10)");
    println!();
    println!("    ruchydbg regression state <file1.ruchy> <file2.ruchy>");
    println!("        Check if variables leak between isolated runs");
    println!();
    println!("    ruchydbg regression perf <baseline.ruchy> <current.ruchy> [--threshold <N>]");
    println!("        Detect performance regressions (default threshold: 2.0x)");
    println!();
    println!("EXAMPLES:");
    println!("    # Check for behavior regression");
    println!("    ruchydbg regression snapshot v1.ruchy v2.ruchy");
    println!();
    println!("    # Check determinism with 20 runs");
    println!("    ruchydbg regression determinism test.ruchy --runs 20");
    println!();
    println!("    # Check for state pollution");
    println!("    ruchydbg regression state defines.ruchy uses.ruchy");
    println!();
    println!("    # Check performance with 3x threshold");
    println!("    ruchydbg regression perf old.ruchy new.ruchy --threshold 3");
    println!();
    println!("EXIT CODES:");
    println!("    0    No regression detected");
    println!("    1    Regression detected");
    println!();
}

fn print_detect_help() {
    println!("USAGE:");
    println!("    ruchydbg detect <file> [OPTIONS]");
    println!();
    println!("DESCRIPTION:");
    println!("    Detect pathological inputs that cause extreme performance degradation.");
    println!("    Complements fuzzing (crashes) and benchmarking (average performance)");
    println!("    by finding specific inputs causing 10x-1000x slowdowns.");
    println!();
    println!("OPTIONS:");
    println!("    --threshold <N>   Slowdown threshold for detection (default: 10.0x)");
    println!();
    println!("CATEGORIES:");
    println!("    Parser Stress     Deeply nested expressions, complex syntax");
    println!("    Evaluator Stress  Quadratic variable lookup, deep call stacks");
    println!("    Memory Stress     Allocation bombs, large data structures");
    println!();
    println!("EXAMPLES:");
    println!("    # Detect with default 10x threshold");
    println!("    ruchydbg detect test.ruchy");
    println!();
    println!("    # Detect with custom 15x threshold");
    println!("    ruchydbg detect test.ruchy --threshold 15");
    println!();
    println!("EXIT CODES:");
    println!("    0    Performance within bounds");
    println!("    1    Pathological input detected (slowdown > threshold)");
    println!();
}

fn run_validation() {
    // Find the validation script relative to the package
    let script_path = find_validation_script();

    // Check if ruchy is available
    let ruchy_check = Command::new("ruchy").arg("--version").output();

    if ruchy_check.is_err() {
        eprintln!("❌ Error: 'ruchy' command not found in PATH");
        eprintln!("Please install Ruchy: https://github.com/paiml/ruchy");
        exit(1);
    }

    // Run the validation script
    println!("🔍 Running RuchyRuchy debugging tools validation...");

    let status = Command::new("ruchy").arg("run").arg(&script_path).status();

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("✅ All debugging tools validation passed!");
                exit(0);
            } else {
                eprintln!("❌ Debugging tools validation failed");
                exit(1);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to execute validation: {e}");
            exit(1);
        }
    }
}

fn find_validation_script() -> PathBuf {
    // Try to find the script in common locations
    let candidates = [
        // Development: relative to package root
        "validation/debugging/ruchydbg.ruchy",
        // Installed: in share directory
        "../share/ruchyruchy/validation/debugging/ruchydbg.ruchy",
        // Alternative: next to binary
        "./validation/debugging/ruchydbg.ruchy",
    ];

    for candidate in &candidates {
        let path = PathBuf::from(candidate);
        if path.exists() {
            return path;
        }
    }

    // If not found, provide helpful error
    eprintln!("❌ Error: Cannot find validation script");
    eprintln!("Expected locations:");
    for candidate in &candidates {
        eprintln!("  - {candidate}");
    }
    eprintln!("\nPlease ensure RuchyRuchy is properly installed.");
    exit(1);
}

fn print_help() {
    println!("RuchyRuchy Debugging Tools CLI v{VERSION}");
    println!();
    println!("USAGE:");
    println!("    ruchydbg [COMMAND]");
    println!();
    println!("COMMANDS:");
    println!(
        "    run <file>           Execute Ruchy code with timeout detection and type-aware tracing"
    );
    println!("    profile <type>       Profile code execution (--stack for call depth analysis)");
    println!("    detect <file>        Detect pathological inputs causing performance cliffs");
    println!("    regression <type>    Check for regressions (snapshot, determinism, state, perf)");
    println!("    validate, test       Run debugging tools validation (default)");
    println!("    version, -v          Print version information");
    println!("    help, -h             Print this help message");
    println!();
    println!("DEBUGGING FEATURES:");
    println!("    - Stack depth profiling (DEBUGGER-041)");
    println!("    - Pathological input detection (DEBUGGER-042)");
    println!("    - Regression & hang detection (DEBUGGER-043)");
    println!("    - Timeout detection for infinite loops and hangs");
    println!("    - Type-aware tracing (Ruchy v3.149.0+)");
    println!("    - Source map generation and mapping");
    println!("    - Record-replay engine for time-travel debugging");
    println!("    - Performance benchmarking");
    println!();
    println!("EXAMPLES:");
    println!("    ruchydbg run test.ruchy --timeout 1000 --trace");
    println!("    ruchydbg profile --stack factorial.ruchy");
    println!("    ruchydbg detect test.ruchy --threshold 15");
    println!("    ruchydbg regression snapshot v1.ruchy v2.ruchy");
    println!("    ruchydbg regression determinism test.ruchy --runs 20");
    println!("    ruchydbg validate     # Run all validations");
    println!("    ruchydbg --version    # Show version");
    println!();
    println!("For more information, visit:");
    println!("    https://github.com/paiml/ruchyruchy");
}
