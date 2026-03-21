// Utility command handlers for ruchydbg: tokenize, compare, trace,
// validation, five-whys analysis.
//
// Extracted from main.rs for file-health compliance (<2000 lines).

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{exit, Command};

use super::{EXIT_ERROR, EXIT_SUCCESS};

pub(crate) fn run_tokenize(args: &[String]) {
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

pub(crate) fn run_compare(args: &[String]) {
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

pub(crate) fn run_trace(args: &[String]) {
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

pub(crate) fn print_tokenize_help() {
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

pub(crate) fn print_compare_help() {
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

pub(crate) fn print_trace_help() {
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

pub(crate) fn run_validation() {
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

pub(crate) fn find_validation_script() -> PathBuf {
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

pub(crate) fn run_five_whys(args: &[String]) {
    use ruchyruchy::debugger::five_whys::{self, BugReport, KnowledgeBase};

    // Check for help first
    if args.len() >= 3 && (args[2] == "--help" || args[2] == "-h") {
        print_five_whys_help();
        exit(EXIT_SUCCESS);
    }

    // Parse arguments
    let mut files: Vec<String> = Vec::new();
    let mut format = "text"; // text or json
    let mut output_file: Option<String> = None;
    let mut interactive = false;
    let mut use_knowledge_base = false;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--format" => {
                if i + 1 < args.len() {
                    format = &args[i + 1];
                    i += 2;
                } else {
                    eprintln!("Error: --format requires a value (text or json)");
                    exit(EXIT_ERROR);
                }
            }
            "--output" | "-o" => {
                if i + 1 < args.len() {
                    output_file = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --output requires a file path");
                    exit(EXIT_ERROR);
                }
            }
            "--interactive" | "-i" => {
                interactive = true;
                i += 1;
            }
            "--knowledge-base" | "-k" => {
                use_knowledge_base = true;
                i += 1;
            }
            arg if !arg.starts_with("--") => {
                files.push(arg.to_string());
                i += 1;
            }
            unknown => {
                eprintln!("Error: Unknown option: {}", unknown);
                print_five_whys_help();
                exit(EXIT_ERROR);
            }
        }
    }

    // Validate files
    if files.is_empty() {
        eprintln!("Error: Missing file argument");
        eprintln!("Usage: ruchydbg five-whys <bug-report.json> [options]");
        eprintln!("Try 'ruchydbg five-whys --help' for more information");
        exit(EXIT_ERROR);
    }

    // Process bugs
    let mut analyses = Vec::new();
    let mut knowledge_base = if use_knowledge_base {
        Some(KnowledgeBase::new())
    } else {
        None
    };

    for file_path in &files {
        // Read file
        let content = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error: Failed to read file '{}': {}", file_path, e);
                exit(EXIT_ERROR);
            }
        };

        // Parse JSON
        let bug_report: BugReport = match serde_json::from_str(&content) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Error: Failed to parse JSON from '{}': {}", file_path, e);
                exit(EXIT_ERROR);
            }
        };

        // Perform analysis
        let analysis = if interactive && files.len() == 1 {
            // Interactive mode
            run_interactive_analysis(bug_report)
        } else {
            // Non-interactive analysis
            match five_whys::analyze_bug(&bug_report) {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("Error: Analysis failed: {}", e);
                    exit(EXIT_ERROR);
                }
            }
        };

        analyses.push(analysis.clone());

        // Add to knowledge base
        if let Some(ref mut kb) = knowledge_base {
            kb.add_analysis(&analysis);
        }
    }

    // Generate output
    let output = if format == "json" {
        // JSON output
        if analyses.len() == 1 {
            serde_json::to_string_pretty(&analyses[0]).unwrap()
        } else {
            serde_json::to_string_pretty(&analyses).unwrap()
        }
    } else {
        // Human-readable text output
        let mut result = String::new();
        for (idx, analysis) in analyses.iter().enumerate() {
            if analyses.len() > 1 {
                result.push_str(&format!("\n=== Analysis {} ===\n\n", idx + 1));
            }
            result.push_str("Five Whys Analysis:\n");
            for (i, why) in analysis.whys.iter().enumerate() {
                result.push_str(&format!("  Why {}: {}\n", i + 1, why.question));
                result.push_str(&format!("  Answer: {}\n\n", why.answer));
            }
            result.push_str(&format!("Root Cause: {:?}\n", analysis.root_cause));
            result.push_str(&format!("Recommended Fix: {}\n", analysis.recommended_fix));
        }

        // Add knowledge base patterns if requested
        if let Some(ref kb) = knowledge_base {
            let patterns = kb.detect_patterns();
            if !patterns.is_empty() {
                result.push_str("\n=== Detected Patterns ===\n\n");
                for pattern in &patterns {
                    result.push_str(&format!("Pattern: {}\n", pattern.symptom_pattern));
                    result.push_str(&format!("  Occurrences: {}\n", pattern.occurrence_count));
                    result.push_str(&format!(
                        "  Prevention: {}\n\n",
                        pattern.prevention_strategy
                    ));
                }
            }
        }

        result
    };

    // Write output
    if let Some(ref path) = output_file {
        match fs::write(path, &output) {
            Ok(_) => {
                println!("Analysis written to {}", path);
                exit(EXIT_SUCCESS);
            }
            Err(e) => {
                eprintln!("Error: Failed to write to '{}': {}", path, e);
                exit(EXIT_ERROR);
            }
        }
    } else {
        print!("{}", output);
        exit(EXIT_SUCCESS);
    }
}

pub(crate) fn run_interactive_analysis(
    bug_report: ruchyruchy::debugger::five_whys::BugReport,
) -> ruchyruchy::debugger::five_whys::FiveWhysAnalysis {
    use ruchyruchy::debugger::five_whys::InteractiveSession;

    let mut session = InteractiveSession::new(bug_report);

    println!("=== Interactive Five Whys Analysis ===\n");

    for i in 0..5 {
        if let Some(question) = session.next_question() {
            println!("Why {}: {}", i + 1, question);
            print!("Your answer: ");
            io::stdout().flush().unwrap();

            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read input");

            session.add_user_context(user_input.trim().to_string());
            println!();
        }
    }

    match session.finalize() {
        Ok(analysis) => analysis,
        Err(e) => {
            eprintln!("Error: Interactive session failed: {}", e);
            exit(EXIT_ERROR);
        }
    }
}

pub(crate) fn print_five_whys_help() {
    println!("Five Whys Interactive Debugging (Toyota Way)");
    println!();
    println!("USAGE:");
    println!("    ruchydbg five-whys <bug-report.json> [options]");
    println!("    ruchydbg five-whys <bug1.json> <bug2.json> ... --knowledge-base");
    println!();
    println!("DESCRIPTION:");
    println!("    Perform root cause analysis using Toyota Way Five Whys methodology.");
    println!("    Trace from symptom to root cause by asking 'why' 5 times.");
    println!();
    println!("OPTIONS:");
    println!("    --format <type>        Output format: 'text' or 'json' (default: text)");
    println!("    --output, -o <file>    Write analysis to file instead of stdout");
    println!("    --interactive, -i      Interactive mode with user feedback");
    println!("    --knowledge-base, -k   Detect patterns across multiple bug reports");
    println!("    --help, -h             Show this help message");
    println!();
    println!("BUG REPORT FORMAT (JSON):");
    println!(
        r#"    {{
      "category": "InterpreterRuntime" | "Compiler" | "Transpiler",
      "symptom": "Brief description of the bug",
      "source_code": "Code that triggers the bug",
      "error_message": "Optional: Error message",
      "stack_trace": ["Optional: Stack trace lines"]
    }}"#
    );
    println!();
    println!("EXAMPLES:");
    println!("    # Basic analysis");
    println!("    ruchydbg five-whys bug-report.json");
    println!();
    println!("    # JSON output");
    println!("    ruchydbg five-whys bug-report.json --format json");
    println!();
    println!("    # Save to file");
    println!("    ruchydbg five-whys bug-report.json --output analysis.txt");
    println!();
    println!("    # Interactive mode");
    println!("    ruchydbg five-whys bug-report.json --interactive");
    println!();
    println!("    # Pattern detection across multiple bugs");
    println!("    ruchydbg five-whys bug1.json bug2.json bug3.json --knowledge-base");
    println!();
    println!("TOYOTA WAY PRINCIPLES:");
    println!("    - Genchi Genbutsu: Go and see actual bug in context");
    println!("    - Five Whys: Ask why 5 times to find root cause");
    println!("    - Jidoka: Stop and fix problems immediately");
    println!("    - Kaizen: Continuous improvement through learning");
    println!("    - Hansei: Reflect on failures to prevent recurrence");
}

