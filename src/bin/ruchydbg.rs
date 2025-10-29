//! RuchyRuchy Debugging Tools CLI
//!
//! This binary provides a command-line interface for the RuchyRuchy debugging
//! validation tools. It wraps the Ruchy-based validation scripts and makes them
//! easily accessible via the `ruchydbg` command.

use std::env;
use std::path::PathBuf;
use std::process::{Command, exit};
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
    // Parse arguments: ruchydbg run <file> [--timeout <ms>]

    // Check for help first
    if args.len() >= 3 && (args[2] == "--help" || args[2] == "-h") {
        print_run_help();
        exit(EXIT_SUCCESS);
    }

    if args.len() < 3 {
        eprintln!("Error: Missing file argument");
        eprintln!("Usage: ruchydbg run <file> [--timeout <ms>]");
        exit(EXIT_ERROR);
    }

    let file_path = &args[2];

    // Parse timeout (default: 5000ms = 5 seconds)
    let timeout_ms = if args.len() >= 5 && args[3] == "--timeout" {
        args[4].parse::<u64>().unwrap_or(DEFAULT_TIMEOUT_MS)
    } else {
        DEFAULT_TIMEOUT_MS
    };

    // Check if file exists
    if !PathBuf::from(file_path).exists() {
        eprintln!("Error: File not found: {}", file_path);
        exit(EXIT_ERROR);
    }

    // Check if ruchy is available
    let ruchy_check = Command::new("ruchy")
        .arg("--version")
        .output();

    if ruchy_check.is_err() {
        eprintln!("❌ Error: 'ruchy' command not found in PATH");
        eprintln!("Please install Ruchy: https://github.com/paiml/ruchy");
        exit(EXIT_ERROR);
    }

    // Run with timeout
    println!("🔍 Running: {}", file_path);
    println!("⏱️  Timeout: {}ms", timeout_ms);
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
        cmd.arg("run");
        cmd.arg(file_path);
    }

    // On Windows, just run ruchy (timeout not available)
    #[cfg(not(unix))]
    let mut cmd = Command::new("ruchy");
    #[cfg(not(unix))]
    {
        cmd.arg("run");
        cmd.arg(file_path);
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
    println!("ruchydbg run - Execute Ruchy code with timeout detection");
    println!();
    println!("USAGE:");
    println!("    ruchydbg run <file> [--timeout <ms>]");
    println!();
    println!("ARGUMENTS:");
    println!("    <file>           Path to .ruchy file to execute");
    println!("    --timeout <ms>   Timeout in milliseconds (default: 5000ms)");
    println!();
    println!("EXIT CODES:");
    println!("    0    Success");
    println!("    124  Timeout");
    println!("    1+   Other error");
    println!();
    println!("EXAMPLES:");
    println!("    ruchydbg run test.ruchy");
    println!("    ruchydbg run test.ruchy --timeout 1000");
    println!("    ruchydbg run test.ruchy --timeout 5000");
}

fn run_validation() {
    // Find the validation script relative to the package
    let script_path = find_validation_script();

    // Check if ruchy is available
    let ruchy_check = Command::new("ruchy")
        .arg("--version")
        .output();

    if ruchy_check.is_err() {
        eprintln!("❌ Error: 'ruchy' command not found in PATH");
        eprintln!("Please install Ruchy: https://github.com/paiml/ruchy");
        exit(1);
    }

    // Run the validation script
    println!("🔍 Running RuchyRuchy debugging tools validation...");

    let status = Command::new("ruchy")
        .arg("run")
        .arg(&script_path)
        .status();

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
    println!("    run <file>        Execute Ruchy code with timeout detection");
    println!("    validate, test    Run debugging tools validation (default)");
    println!("    version, -v       Print version information");
    println!("    help, -h          Print this help message");
    println!();
    println!("VALIDATION CHECKS:");
    println!("    - Source map generation and mapping");
    println!("    - Record-replay engine smoke test");
    println!("    - Performance benchmarking");
    println!();
    println!("EXAMPLES:");
    println!("    ruchydbg run test.ruchy --timeout 1000");
    println!("    ruchydbg validate     # Run all validations");
    println!("    ruchydbg --version    # Show version");
    println!();
    println!("For more information, visit:");
    println!("    https://github.com/paiml/ruchyruchy");
}
