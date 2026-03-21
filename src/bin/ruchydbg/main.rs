//! RuchyRuchy Debugging Tools CLI
//!
//! This binary provides a command-line interface for the RuchyRuchy debugging
//! validation tools. It wraps the Ruchy-based validation scripts and makes them
//! easily accessible via the `ruchydbg` command.

use std::env;
use std::path::PathBuf;
use std::process::{exit, Command};
use std::time::Instant;

mod commands_debug;
mod commands_util;

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
        "detect" => commands_debug::run_detect(&args),
        "regression" => commands_debug::run_regression(&args),
        "debug" => commands_debug::run_debug(&args),
        "tokenize" => commands_util::run_tokenize(&args),
        "compare" => commands_util::run_compare(&args),
        "trace" => commands_util::run_trace(&args),
        "five-whys" => commands_util::run_five_whys(&args),
        "validate" | "test" => commands_util::run_validation(),
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
    println!("    five-whys <file>     Five Whys root cause analysis (Toyota Way) ⭐ NEW!");
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
    println!("    - Five Whys root cause analysis with knowledge base (DEBUGGER-056) ⭐ NEW!");
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
    println!("    ruchydbg five-whys bug-report.json         # Five Whys analysis ⭐");
    println!("    ruchydbg five-whys bug-report.json --interactive  # Interactive mode ⭐");
    println!("    ruchydbg five-whys bug1.json bug2.json --knowledge-base  # Pattern detection ⭐");
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
