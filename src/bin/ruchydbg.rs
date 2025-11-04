//! RuchyRuchy Debugging Tools CLI
//!
//! This binary provides a command-line interface for the RuchyRuchy debugging
//! validation tools. It wraps the Ruchy-based validation scripts and makes them
//! easily accessible via the `ruchydbg` command.

use std::env;
use std::path::PathBuf;
use std::process::{exit, Command};
use std::time::Instant;

#[cfg(feature = "ebpf")]
use std::fs;

#[cfg(feature = "ebpf")]
use ruchyruchy::tracing::ebpf::SyscallTracer;

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
        "debug" => run_debug(&args),
        "tokenize" => run_tokenize(&args),
        "compare" => run_compare(&args),
        "trace" => run_trace(&args),
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
    // Parse arguments: ruchydbg run <file> [--timeout <ms>] [--trace] [--trace-syscalls] [--trace-output <path>]

    // Check for help first
    if args.len() >= 3 && (args[2] == "--help" || args[2] == "-h") {
        print_run_help();
        exit(EXIT_SUCCESS);
    }

    if args.len() < 3 {
        eprintln!("Error: Missing file argument");
        eprintln!("Usage: ruchydbg run <file> [--timeout <ms>] [--trace] [--trace-syscalls] [--trace-output <path>]");
        exit(EXIT_ERROR);
    }

    let file_path = &args[2];

    // Parse timeout and trace flags
    let mut timeout_ms = DEFAULT_TIMEOUT_MS;
    let mut enable_trace = false;
    let mut enable_syscall_trace = false;
    let mut trace_output_path: Option<String> = None;

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
            "--trace-syscalls" => {
                enable_syscall_trace = true;
                i += 1;
            }
            "--trace-output" => {
                if i + 1 < args.len() {
                    trace_output_path = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --trace-output requires a value");
                    exit(EXIT_ERROR);
                }
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
    if enable_syscall_trace {
        println!("🔍 eBPF syscall tracing: enabled");
        if let Some(ref output_path) = trace_output_path {
            println!("📄 Trace output: {}", output_path);
        }
    }
    println!();

    // Start eBPF syscall tracer if requested
    #[cfg(feature = "ebpf")]
    let mut tracer = if enable_syscall_trace {
        match SyscallTracer::new() {
            Ok(t) => {
                println!("✅ eBPF tracer started");
                Some(t)
            }
            Err(e) => {
                eprintln!("⚠️  Warning: Failed to start eBPF tracer: {}", e);
                eprintln!("    Try running with sudo or ensure CAP_BPF capability");
                None
            }
        }
    } else {
        None
    };

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

    // Collect syscall trace if eBPF tracer was running
    #[cfg(feature = "ebpf")]
    let syscall_events = if let Some(ref mut tracer) = tracer {
        // Give tracer a moment to collect final events
        std::thread::sleep(std::time::Duration::from_millis(100));

        match tracer.read_events() {
            Ok(events) => {
                println!("📊 Captured {} syscall events", events.len());
                Some(events)
            }
            Err(e) => {
                eprintln!("⚠️  Warning: Failed to read syscall events: {}", e);
                None
            }
        }
    } else {
        None
    };

    // Write syscall trace to JSON if output path specified
    #[cfg(feature = "ebpf")]
    if let Some(events) = syscall_events {
        if let Some(ref output_path) = trace_output_path {
            // Convert events to JSON
            let json_events: Vec<_> = events
                .iter()
                .map(|e| {
                    serde_json::json!({
                        "pid": e.pid,
                        "syscall_nr": e.syscall_nr,
                        "timestamp_ns": e.timestamp_ns,
                        "is_enter": e.is_enter == 1,
                    })
                })
                .collect();

            match fs::write(
                output_path,
                serde_json::to_string_pretty(&json_events).unwrap(),
            ) {
                Ok(_) => println!("✅ Syscall trace written to: {}", output_path),
                Err(e) => eprintln!("⚠️  Warning: Failed to write trace: {}", e),
            }
        }
    }

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
        "--perf" => run_perf_profiler(&args[3..]),
        "--types" => run_types_profiler(&args[3..]),
        _ => {
            eprintln!("Error: Unknown profile type: {}", profile_type);
            eprintln!("Available types: --stack, --perf, --types");
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

fn run_perf_profiler(args: &[String]) {
    // PERF-001B: Performance profiler
    // Usage: ruchydbg profile --perf <file> [--iterations N]

    if args.is_empty() {
        eprintln!("Error: Missing file argument");
        eprintln!("Usage: ruchydbg profile --perf <file> [--iterations N]");
        exit(EXIT_ERROR);
    }

    let file_path = &args[0];

    // Parse optional iterations flag
    let iterations = if args.len() >= 3 && args[1] == "--iterations" {
        args[2].parse::<usize>().unwrap_or(1000)
    } else {
        1000 // Default: 1000 iterations for statistical rigor
    };

    // Check if file exists
    let path = PathBuf::from(file_path);
    if !path.exists() {
        eprintln!("Error: File not found: {}", file_path);
        exit(EXIT_ERROR);
    }

    // Read source file
    let source = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("Error reading file: {}", e);
        exit(EXIT_ERROR);
    });

    println!("🔍 Performance Profiling: {}", file_path);
    println!("📊 Iterations: {}", iterations);
    println!("=============================================================");
    println!();

    // Profile: Parse phase
    use ruchyruchy::interpreter::parser::Parser;
    use std::time::Instant;

    let mut parse_times_us = Vec::with_capacity(iterations);
    let mut asts = Vec::with_capacity(iterations);

    for _ in 0..iterations {
        let start = Instant::now();
        let mut parser = Parser::new(&source);
        let ast = parser.parse();
        let duration = start.elapsed();

        parse_times_us.push(duration.as_micros() as f64);
        if let Ok(ast) = ast {
            asts.push(ast);
        }
    }

    // Profile: Eval phase
    use ruchyruchy::interpreter::evaluator::Evaluator;

    let mut eval_times_us = Vec::with_capacity(iterations);

    for ast in &asts {
        let start = Instant::now();
        let mut eval = Evaluator::new();
        for statement in ast.nodes() {
            let _ = eval.eval(statement);
        }
        let duration = start.elapsed();
        eval_times_us.push(duration.as_micros() as f64);
    }

    // Calculate statistics
    let parse_mean = parse_times_us.iter().sum::<f64>() / parse_times_us.len() as f64;
    let eval_mean = eval_times_us.iter().sum::<f64>() / eval_times_us.len() as f64;
    let total_mean = parse_mean + eval_mean;

    let parse_pct = (parse_mean / total_mean) * 100.0;
    let eval_pct = (eval_mean / total_mean) * 100.0;

    // Output results
    println!("Phase Breakdown:");
    println!("  Parse:    {:>8.2} µs ({:>5.1}%)", parse_mean, parse_pct);
    println!("  Eval:     {:>8.2} µs ({:>5.1}%)", eval_mean, eval_pct);
    println!("  Total:    {:>8.2} µs", total_mean);
    println!();

    // Amdahl's Law Analysis
    let bottleneck_threshold = 30.0; // Phase taking >30% is considered dominant
    if parse_pct > bottleneck_threshold && parse_pct > eval_pct {
        println!("🎯 BOTTLENECK: Parse ({:.1}%)", parse_pct);
        println!("   Recommendation: Optimize parser (tokenization, AST construction)");
        println!(
            "   Amdahl's Law: 50% speedup in parse → {:.1}% overall speedup",
            (parse_pct / 2.0) * 0.5
        );
    } else if eval_pct > bottleneck_threshold && eval_pct > parse_pct {
        println!("🎯 BOTTLENECK: Eval ({:.1}%)", eval_pct);
        println!("   Recommendation: Optimize evaluator (cloning, lookups, operations)");
        println!(
            "   Amdahl's Law: 50% speedup in eval → {:.1}% overall speedup",
            (eval_pct / 2.0) * 0.5
        );
    } else {
        println!("✓ Balanced performance (no single dominant bottleneck)");
    }

    println!();
    println!("=============================================================");
    println!("✅ Profiling complete");
}

fn run_types_profiler(args: &[String]) {
    // INTERP-050: Type stability tracking for JIT
    // Usage: ruchydbg profile --types <file>

    if args.is_empty() {
        eprintln!("Error: Missing file argument");
        eprintln!("Usage: ruchydbg profile --types <file>");
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

    // Execute with compiler profiler enabled
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::profiler::{CompilerProfiler, Stability};

    let profiler = CompilerProfiler::new();
    let mut eval = Evaluator::new().with_type_observation(&profiler);

    for statement in ast.nodes() {
        if let Err(e) = eval.eval(statement) {
            eprintln!("Evaluation error: {:?}", e);
            exit(EXIT_ERROR);
        }
    }

    // Display type stability report
    println!("\n🔍 Type Stability Analysis: {}", file_path);
    println!("=============================================================");
    println!();

    // Get all function profiles sorted by total time (hottest first)
    let all_profiles = profiler.all_function_profiles_sorted();

    if all_profiles.is_empty() {
        println!("⚠️  No function calls observed");
        exit(EXIT_SUCCESS);
    }

    // Get all functions for later hot function analysis
    let _hot_functions = profiler.hot_functions(0.0); // Get all functions with timing

    for (func_name, profile) in &all_profiles {
        let stability = profiler.type_stability(func_name);
        let observations = profiler.type_observations(func_name);
        let unique_sigs: std::collections::HashSet<_> = observations.iter().collect();
        let percentage = profiler.function_percentage(func_name);

        println!("Function: {}", func_name);
        println!("  Calls: {}", profile.call_count);
        println!(
            "  Time: {:.2} µs ({:.1}%)",
            profile.total_time_us, percentage
        );

        // Show type stability
        match stability {
            Stability::Monomorphic => {
                println!("  Type Stability: ✅ Monomorphic (1 type signature)");
                println!("    → EXCELLENT JIT candidate (type-stable)");
            }
            Stability::Polymorphic => {
                println!(
                    "  Type Stability: ⚠️  Polymorphic ({} type signatures)",
                    unique_sigs.len()
                );
                println!("    → MODERATE JIT candidate (use inline cache)");
            }
            Stability::Megamorphic => {
                println!(
                    "  Type Stability: ❌ Megamorphic ({}+ type signatures)",
                    unique_sigs.len()
                );
                println!("    → POOR JIT candidate (too unstable)");
            }
        }

        // Show type signatures
        if !observations.is_empty() {
            println!("  Type Signatures:");
            for sig in unique_sigs {
                let params = sig.param_types().join(", ");
                let ret = sig.return_type();
                println!("    ({}) → {}", params, ret);
            }
        }

        println!();
    }

    // Identify excellent JIT candidates (hot + type-stable)
    println!("=============================================================");
    println!("🎯 JIT Compilation Recommendations:");
    println!();

    let hot_threshold = 30.0; // >30% of total time
    let hot_funcs = profiler.hot_functions(hot_threshold);

    let excellent_candidates: Vec<_> = hot_funcs
        .iter()
        .filter(|(name, _)| profiler.type_stability(name) == Stability::Monomorphic)
        .collect();

    if excellent_candidates.is_empty() {
        println!("⚠️  No excellent JIT candidates found");
        println!(
            "   (Looking for: hot functions >{}% AND type-stable)",
            hot_threshold
        );
    } else {
        println!("✅ Excellent JIT Candidates (hot + type-stable):");
        for (name, pct) in excellent_candidates {
            println!("   • {} ({:.1}% of time, monomorphic)", name, pct);
        }
        println!();
        println!("💡 These functions should be JIT compiled for maximum speedup!");
    }

    println!();
    exit(EXIT_SUCCESS);
}

fn print_profile_help() {
    println!("USAGE:");
    println!("    ruchydbg profile <TYPE> <file> [OPTIONS]");
    println!();
    println!("TYPES:");
    println!("    --perf             Performance profiling (parse vs eval breakdown)");
    println!("    --stack            Stack depth profiler (max depth, call counts, call tree)");
    println!("    --types            Type stability tracking (JIT candidate identification)");
    println!();
    println!("OPTIONS (for --perf):");
    println!("    --iterations N     Number of iterations for statistical rigor (default: 1000)");
    println!();
    println!("EXAMPLES:");
    println!("    ruchydbg profile --perf fibonacci.ruchy");
    println!("    ruchydbg profile --perf test.ruchy --iterations 10000");
    println!("    ruchydbg profile --stack factorial.ruchy");
    println!("    ruchydbg profile --types hot_loop.ruchy");
    println!();
    println!("OUTPUT (--perf):");
    println!("    - Phase breakdown (Parse, Eval percentages)");
    println!("    - Bottleneck identification (>30% threshold)");
    println!("    - Amdahl's Law analysis (optimization potential)");
    println!("    - Optimization recommendations");
    println!();
    println!("OUTPUT (--stack):");
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

fn run_debug(args: &[String]) {
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

fn run_debug_interactive(args: &[String]) {
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

fn run_debug_analyze(args: &[String]) {
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

fn find_ruchy_binary() -> PathBuf {
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

fn print_debug_help() {
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

fn run_tokenize(args: &[String]) {
    // Parse arguments: ruchydbg tokenize <file> [--errors] [--analyze]

    if args.len() >= 3 && (args[2] == "--help" || args[2] == "-h") {
        print_tokenize_help();
        exit(EXIT_SUCCESS);
    }

    if args.len() < 3 {
        eprintln!("Error: Missing file argument");
        eprintln!("Usage: ruchydbg tokenize <file> [--errors] [--analyze]");
        exit(EXIT_ERROR);
    }

    let file_path = &args[2];

    // Read source file
    let source = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Error reading file '{}': {}", file_path, e);
            exit(EXIT_ERROR);
        }
    };

    // Check for flags
    let show_errors = args.iter().any(|a| a == "--errors");
    let analyze = args.iter().any(|a| a == "--analyze");

    // Run appropriate tokenization function
    if analyze {
        let analysis = ruchyruchy::debugger::tokenize_analyze(&source);
        println!("Token Pattern Analysis");
        println!("======================\n");

        if analysis.warnings.is_empty() {
            println!("✅ No pattern conflicts detected");
        } else {
            println!("⚠️  {} warning(s) detected:\n", analysis.warnings.len());
            for (i, warning) in analysis.warnings.iter().enumerate() {
                println!("{}. {}", i + 1, warning);
            }
        }
    } else if show_errors {
        let output = ruchyruchy::debugger::tokenize_with_errors(&source);
        print!("{}", output);
    } else {
        let output = ruchyruchy::debugger::tokenize(&source);
        print!("{}", output);
    }
}

fn run_compare(args: &[String]) {
    // Parse arguments: ruchydbg compare <file1> <file2> [--hints]

    if args.len() >= 3 && (args[2] == "--help" || args[2] == "-h") {
        print_compare_help();
        exit(EXIT_SUCCESS);
    }

    if args.len() < 4 {
        eprintln!("Error: Missing file arguments");
        eprintln!("Usage: ruchydbg compare <file1> <file2> [--hints]");
        exit(EXIT_ERROR);
    }

    let file1_path = &args[2];
    let file2_path = &args[3];

    // Read source files
    let source1 = match std::fs::read_to_string(file1_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Error reading file '{}': {}", file1_path, e);
            exit(EXIT_ERROR);
        }
    };

    let source2 = match std::fs::read_to_string(file2_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Error reading file '{}': {}", file2_path, e);
            exit(EXIT_ERROR);
        }
    };

    // Check for hints flag
    let show_hints = args.iter().any(|a| a == "--hints");

    // Run comparison
    let output = if show_hints {
        ruchyruchy::debugger::compare_tokens_with_hints(&source1, &source2)
    } else {
        ruchyruchy::debugger::compare_tokens(&source1, &source2)
    };

    print!("{}", output);
}

fn run_trace(args: &[String]) {
    // Parse arguments: ruchydbg trace <file> [--analyze] [--errors-only]

    if args.len() >= 3 && (args[2] == "--help" || args[2] == "-h") {
        print_trace_help();
        exit(EXIT_SUCCESS);
    }

    if args.len() < 3 {
        eprintln!("Error: Missing file argument");
        eprintln!("Usage: ruchydbg trace <file> [--analyze] [--errors-only]");
        exit(EXIT_ERROR);
    }

    let file_path = &args[2];

    // Read source file
    let source = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Error reading file '{}': {}", file_path, e);
            exit(EXIT_ERROR);
        }
    };

    // Check for flags
    let analyze = args.iter().any(|a| a == "--analyze");
    let errors_only = args.iter().any(|a| a == "--errors-only");

    // Run appropriate trace function
    let output = if analyze {
        ruchyruchy::debugger::parser_trace_with_analysis(&source)
    } else if errors_only {
        ruchyruchy::debugger::parser_trace_errors_only(&source)
    } else {
        ruchyruchy::debugger::parser_trace(&source)
    };

    print!("{}", output);
}

fn print_tokenize_help() {
    println!("DEBUGGER-050: Token stream inspection");
    println!();
    println!("USAGE:");
    println!("    ruchydbg tokenize <file> [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    --errors     Highlight error recovery tokens");
    println!("    --analyze    Detect pattern conflicts (String vs Lifetime priority)");
    println!("    -h, --help   Show this help message");
    println!();
    println!("DESCRIPTION:");
    println!("    Show the token stream for a Ruchy source file.");
    println!("    Addresses PARSER-079 pain: 'Couldn't see raw token stream'");
    println!();
    println!("EXAMPLES:");
    println!("    ruchydbg tokenize test.ruchy");
    println!("    ruchydbg tokenize test.ruchy --errors");
    println!("    ruchydbg tokenize test.ruchy --analyze");
    println!();
}

fn print_compare_help() {
    println!("DEBUGGER-050: Token comparison");
    println!();
    println!("USAGE:");
    println!("    ruchydbg compare <file1> <file2> [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    --hints      Show root cause hints for mismatches");
    println!("    -h, --help   Show this help message");
    println!();
    println!("DESCRIPTION:");
    println!("    Compare token streams between two Ruchy source files.");
    println!("    Addresses PARSER-079 pain: 'Manual comparison took hours'");
    println!();
    println!("EXAMPLES:");
    println!("    ruchydbg compare working.ruchy broken.ruchy");
    println!("    ruchydbg compare v1.ruchy v2.ruchy --hints");
    println!();
}

fn print_trace_help() {
    println!("DEBUGGER-050: Parser trace");
    println!();
    println!("USAGE:");
    println!("    ruchydbg trace <file> [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    --analyze       Show root cause analysis");
    println!("    --errors-only   Show only error context");
    println!("    -h, --help      Show this help message");
    println!();
    println!("DESCRIPTION:");
    println!("    Show parser state at failure points.");
    println!("    Addresses PARSER-079 pain: 'Root cause hidden from parser error'");
    println!();
    println!("EXAMPLES:");
    println!("    ruchydbg trace test.ruchy");
    println!("    ruchydbg trace test.ruchy --analyze");
    println!("    ruchydbg trace test.ruchy --errors-only");
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
    println!("    debug <mode>         Interactive rust-gdb wrapper (run, analyze) ⭐ NEW!");
    println!("    tokenize <file>      Show token stream with pattern conflict detection ⭐ NEW!");
    println!("    compare <f1> <f2>    Compare token streams between two files ⭐ NEW!");
    println!("    trace <file>         Show parser trace with root cause analysis ⭐ NEW!");
    println!("    profile <type>       Profile code execution (--stack for call depth analysis)");
    println!("    detect <file>        Detect pathological inputs causing performance cliffs");
    println!("    regression <type>    Check for regressions (snapshot, determinism, state, perf)");
    println!("    validate, test       Run debugging tools validation (default)");
    println!("    version, -v          Print version information");
    println!("    help, -h             Print this help message");
    println!();
    println!("DEBUGGING FEATURES:");
    println!("    - Interactive REPL debugger with time-travel (DEBUGGER-046) ⭐ NEW!");
    println!("    - Token stream inspection & pattern conflict detection (DEBUGGER-050) ⭐ NEW!");
    println!("    - Token comparison with root cause hints (DEBUGGER-050) ⭐ NEW!");
    println!("    - Parser trace with lexer issue detection (DEBUGGER-050) ⭐ NEW!");
    println!("    - Property-based testing: 14,000+ test cases (DEBUGGER-044)");
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
    println!("    ruchydbg debug run test.ruchy              # Interactive debugger ⭐");
    println!("    ruchydbg debug analyze test.ruchy          # Automated trace capture ⭐");
    println!("    ruchydbg tokenize test.ruchy               # Show token stream ⭐");
    println!("    ruchydbg tokenize test.ruchy --analyze     # Detect pattern conflicts ⭐");
    println!("    ruchydbg compare working.ruchy broken.ruchy --hints  # Compare tokens ⭐");
    println!("    ruchydbg trace test.ruchy --analyze        # Parser trace with root cause ⭐");
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
