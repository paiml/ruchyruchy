// Debug, detect, and regression command handlers for ruchydbg.
//
// Extracted from main.rs for file-health compliance (<2000 lines).

use std::path::PathBuf;
use std::process::{exit, Command};

use super::{EXIT_ERROR, EXIT_SUCCESS};

pub(crate) fn run_detect(args: &[String]) {
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

pub(crate) fn run_regression(args: &[String]) {
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

pub(crate) fn run_regression_snapshot(args: &[String]) {
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

pub(crate) fn run_regression_determinism(args: &[String]) {
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
        println!(
            "✅ DETERMINISTIC: All {} runs produced identical results",
            runs
        );
        exit(EXIT_SUCCESS);
    } else {
        println!("⚠️  NON-DETERMINISM DETECTED: Runs produced different results!");
        println!("   This indicates a bug in the interpreter or code.");
        exit(EXIT_ERROR);
    }
}

pub(crate) fn run_regression_state(args: &[String]) {
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

pub(crate) fn run_regression_perf(args: &[String]) {
    // Performance regression check: compare execution times
    if args.len() < 2 {
        eprintln!("Error: perf check requires baseline and current files");
        eprintln!(
            "Usage: ruchydbg regression perf <baseline.ruchy> <current.ruchy> [--threshold <N>]"
        );
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
        println!(
            "⚠️  PERFORMANCE REGRESSION: {:.2}x slowdown exceeds {:.2}x threshold!",
            slowdown, threshold
        );
        exit(EXIT_ERROR);
    } else {
        println!("✅ NO PERFORMANCE REGRESSION: Within acceptable bounds");
        exit(EXIT_SUCCESS);
    }
}

pub(crate) fn print_regression_help() {
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

pub(crate) fn print_detect_help() {
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

pub(crate) fn run_debug(args: &[String]) {
    // DEBUGGER-055: Interactive rust-gdb wrapper for Ruchy debugging
    // Usage: ruchydbg debug <run|analyze> <file> [--break <function>]

    // Check for help
    if args.len() >= 3 && (args[2] == "--help" || args[2] == "-h") {
        print_debug_help();
        exit(EXIT_SUCCESS);
    }

    if args.len() < 3 {
        eprintln!("Error: Missing debug mode");
        eprintln!("Usage: ruchydbg debug <run|analyze> <file> [OPTIONS]");
        print_debug_help();
        exit(EXIT_ERROR);
    }

    let mode = &args[2];

    match mode.as_str() {
        "run" => run_debug_interactive(&args[3..]),
        "analyze" => run_debug_analyze(&args[3..]),
        _ => {
            eprintln!("Error: Unknown debug mode: {}", mode);
            eprintln!("Valid modes: run, analyze");
            print_debug_help();
            exit(EXIT_ERROR);
        }
    }
}

pub(crate) fn run_debug_interactive(args: &[String]) {
    // Interactive rust-gdb session
    if args.is_empty() {
        eprintln!("Error: Missing file argument");
        eprintln!("Usage: ruchydbg debug run <file> [--break <function>]");
        exit(EXIT_ERROR);
    }

    let file_path = &args[0];

    // Check if file exists
    if !PathBuf::from(file_path).exists() {
        eprintln!("Error: File not found: {}", file_path);
        exit(EXIT_ERROR);
    }

    // Parse breakpoint flag
    let breakpoint = if args.len() >= 3 && args[1] == "--break" {
        &args[2]
    } else {
        "dispatch_method_call" // Default breakpoint
    };

    // Find ruchy binary
    let ruchy_bin = find_ruchy_binary();

    println!("🔍 Launching interactive rust-gdb session...");
    println!("📄 File: {}", file_path);
    println!("🎯 Breakpoint: {}", breakpoint);
    println!();
    println!("Commands:");
    println!("  (gdb) run          - Start execution");
    println!("  (gdb) bt           - Show backtrace");
    println!("  (gdb) info locals  - Show local variables");
    println!("  (gdb) print <var>  - Print variable value");
    println!("  (gdb) continue     - Continue execution");
    println!("  (gdb) quit         - Exit debugger");
    println!();

    // Create temporary gdb commands file
    let gdb_commands = format!(
        r#"set print pretty on
set print array on
break {}
run run "{}"
"#,
        breakpoint, file_path
    );

    let temp_file = std::env::temp_dir().join("ruchydbg-gdb-commands.txt");
    std::fs::write(&temp_file, gdb_commands).expect("Failed to write gdb commands");

    // Launch rust-gdb
    let status = Command::new("rust-gdb")
        .arg("-x")
        .arg(&temp_file)
        .arg(&ruchy_bin)
        .status();

    // Cleanup
    let _ = std::fs::remove_file(temp_file);

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                exit(EXIT_SUCCESS);
            } else {
                exit(exit_status.code().unwrap_or(EXIT_ERROR));
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to launch rust-gdb: {}", e);
            eprintln!("Make sure rust-gdb is installed: rustup component add rust-src");
            exit(EXIT_ERROR);
        }
    }
}

pub(crate) fn run_debug_analyze(args: &[String]) {
    // Automated debug session with trace capture
    if args.is_empty() {
        eprintln!("Error: Missing file argument");
        eprintln!("Usage: ruchydbg debug analyze <file> [--break <function>]");
        exit(EXIT_ERROR);
    }

    let file_path = &args[0];

    // Check if file exists
    if !PathBuf::from(file_path).exists() {
        eprintln!("Error: File not found: {}", file_path);
        exit(EXIT_ERROR);
    }

    // Parse breakpoint flag
    let breakpoint = if args.len() >= 3 && args[1] == "--break" {
        &args[2]
    } else {
        "dispatch_method_call" // Default breakpoint
    };

    // Find ruchy binary
    let ruchy_bin = find_ruchy_binary();

    println!("🔍 Running automated rust-gdb analysis...");
    println!("📄 File: {}", file_path);
    println!("🎯 Breakpoint: {}", breakpoint);
    println!();

    // Create GDB batch commands
    let gdb_commands = format!(
        r#"set pagination off
set print pretty on
set print array on

break {}

commands 1
  echo ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n
  echo 🔍 BREAKPOINT HIT: {}\n
  echo ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n
  echo \n📋 Backtrace:\n
  bt 10
  echo \n📊 Local variables:\n
  info locals
  echo \n📍 Arguments:\n
  info args
  echo \n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n
  continue
end

run run "{}"
quit
"#,
        breakpoint, breakpoint, file_path
    );

    let temp_file = std::env::temp_dir().join("ruchydbg-analyze-commands.txt");
    std::fs::write(&temp_file, gdb_commands).expect("Failed to write gdb commands");

    // Run rust-gdb in batch mode
    let output = Command::new("rust-gdb")
        .arg("-batch")
        .arg("-x")
        .arg(&temp_file)
        .arg(&ruchy_bin)
        .output();

    // Cleanup
    let _ = std::fs::remove_file(temp_file);

    match output {
        Ok(output) => {
            // Print stdout and stderr
            print!("{}", String::from_utf8_lossy(&output.stdout));
            eprint!("{}", String::from_utf8_lossy(&output.stderr));

            println!();
            println!("✅ Analysis complete");
            exit(EXIT_SUCCESS);
        }
        Err(e) => {
            eprintln!("❌ Failed to run rust-gdb: {}", e);
            eprintln!("Make sure rust-gdb is installed: rustup component add rust-src");
            exit(EXIT_ERROR);
        }
    }
}

pub(crate) fn find_ruchy_binary() -> PathBuf {
    // Try to find ruchy binary in common locations
    let candidates = [
        "../ruchy/target/debug/ruchy",
        "../ruchy/target/release/ruchy",
        "../../ruchy/target/debug/ruchy",
    ];

    for candidate in &candidates {
        let path = PathBuf::from(candidate);
        if path.exists() {
            return path;
        }
    }

    // Check if ruchy is in PATH
    if Command::new("which")
        .arg("ruchy")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        return PathBuf::from("ruchy");
    }

    eprintln!("❌ Error: Cannot find ruchy binary");
    eprintln!("Expected locations:");
    for candidate in &candidates {
        eprintln!("  - {}", candidate);
    }
    eprintln!("  - ruchy in PATH");
    eprintln!("\nPlease build ruchy first:");
    eprintln!("  cd ../ruchy && cargo build --bin ruchy");
    exit(EXIT_ERROR);
}

pub(crate) fn print_debug_help() {
    println!("ruchydbg debug - Interactive rust-gdb wrapper for Ruchy debugging");
    println!();
    println!("USAGE:");
    println!("    ruchydbg debug <MODE> <file> [OPTIONS]");
    println!();
    println!("MODES:");
    println!("    run          Launch interactive rust-gdb session");
    println!("    analyze      Run automated debug analysis (batch mode)");
    println!();
    println!("OPTIONS:");
    println!(
        "    --break <function>    Set breakpoint at function (default: dispatch_method_call)"
    );
    println!();
    println!("EXAMPLES:");
    println!("    # Interactive debugging session");
    println!("    ruchydbg debug run test.ruchy");
    println!();
    println!("    # Interactive with custom breakpoint");
    println!("    ruchydbg debug run test.ruchy --break eval_method_dispatch");
    println!();
    println!("    # Automated trace capture");
    println!("    ruchydbg debug analyze test.ruchy");
    println!();
    println!("    # Automated analysis with custom breakpoint");
    println!("    ruchydbg debug analyze test.ruchy --break parse_function");
    println!();
    println!("COMMON BREAKPOINTS:");
    println!("    dispatch_method_call     - Method dispatch entry point");
    println!("    eval_method_dispatch     - Method evaluation");
    println!("    parse_function           - Function parsing");
    println!("    eval_expression          - Expression evaluation");
    println!();
    println!("GDB COMMANDS (interactive mode):");
    println!("    run          - Start execution");
    println!("    bt           - Show backtrace");
    println!("    info locals  - Show local variables");
    println!("    print <var>  - Print variable value");
    println!("    continue     - Continue execution");
    println!("    quit         - Exit debugger");
    println!();
}

// DEBUGGER-050: Tokenization debugging commands

