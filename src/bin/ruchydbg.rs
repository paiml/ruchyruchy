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
        "    run <file>        Execute Ruchy code with timeout detection and type-aware tracing"
    );
    println!("    profile <type>    Profile code execution (--stack for call depth analysis)");
    println!("    validate, test    Run debugging tools validation (default)");
    println!("    version, -v       Print version information");
    println!("    help, -h          Print this help message");
    println!();
    println!("DEBUGGING FEATURES:");
    println!("    - Stack depth profiling (DEBUGGER-041)");
    println!("    - Timeout detection for infinite loops and hangs");
    println!("    - Type-aware tracing (Ruchy v3.149.0+)");
    println!("    - Source map generation and mapping");
    println!("    - Record-replay engine for time-travel debugging");
    println!("    - Performance benchmarking");
    println!();
    println!("EXAMPLES:");
    println!("    ruchydbg run test.ruchy --timeout 1000 --trace");
    println!("    ruchydbg profile --stack factorial.ruchy");
    println!("    ruchydbg validate     # Run all validations");
    println!("    ruchydbg --version    # Show version");
    println!();
    println!("For more information, visit:");
    println!("    https://github.com/paiml/ruchyruchy");
}
