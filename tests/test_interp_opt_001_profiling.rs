// INTERP-OPT-001: Rigorous Performance Profiling
//
// NASA-Level Quality: Measure first, optimize second
// Zero tolerance for guessing - we need EXACT data
//
// Mission: Identify THE bottleneck (>50% of time) using Amdahl's Law
//
// Current State: 34.71ms for Hello World
// Target: Identify where EVERY millisecond goes
//
// Tools:
// - cargo flamegraph (visual bottleneck identification)
// - perf stat (cache misses, branch mispredictions, IPC)
// - criterion (statistical rigor, outlier detection)
//
// Hypothesis to VALIDATE (not assume):
// - Parser: ?% of time (MEASURE, don't guess)
// - Evaluator: ?% of time (MEASURE)
// - Allocations: ?% of time (MEASURE)
// - Other: ?% of time (MEASURE)

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::Parser;
use std::time::Instant;

/// Test: Measure Hello World Breakdown with Nanosecond Precision
///
/// NASA-Level Requirement: Break down 34.71ms into measurable components
#[test]
fn test_hello_world_breakdown() {
    let program = r#"println("Hello, World!")"#;
    let iterations = 1000; // Statistical significance

    println!("\n=== Hello World Breakdown (1000 iterations) ===");

    // Measure PARSING time
    let parse_start = Instant::now();
    let mut asts = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let mut parser = Parser::new(program);
        let ast = parser.parse().expect("Parse should succeed");
        asts.push(ast);
    }
    let parse_time = parse_start.elapsed();
    let parse_per_iter_us = parse_time.as_micros() as f64 / iterations as f64;

    println!("Parse time:     {:.2} µs/iter", parse_per_iter_us);

    // Measure EVALUATION time (separate from parsing)
    let eval_start = Instant::now();
    for ast in &asts {
        let mut eval = Evaluator::new();
        for statement in ast.nodes() {
            let _ = eval.eval(statement); // Ignore output for pure timing
        }
    }
    let eval_time = eval_start.elapsed();
    let eval_per_iter_us = eval_time.as_micros() as f64 / iterations as f64;

    println!("Eval time:      {:.2} µs/iter", eval_per_iter_us);

    // Total time
    let total_time = parse_time + eval_time;
    let total_per_iter_us = total_time.as_micros() as f64 / iterations as f64;

    println!("Total time:     {:.2} µs/iter", total_per_iter_us);
    println!();

    // Amdahl's Law Analysis
    let parse_pct = (parse_time.as_micros() as f64 / total_time.as_micros() as f64) * 100.0;
    let eval_pct = (eval_time.as_micros() as f64 / total_time.as_micros() as f64) * 100.0;

    println!("=== Amdahl's Law Breakdown ===");
    println!("Parse:  {:.1}% of total time", parse_pct);
    println!("Eval:   {:.1}% of total time", eval_pct);

    // Identify THE bottleneck (>50% of time)
    if parse_pct > 50.0 {
        println!("\n🎯 BOTTLENECK: Parser ({:.1}%)", parse_pct);
        println!("   Optimization target: Reduce parsing overhead");
    } else if eval_pct > 50.0 {
        println!("\n🎯 BOTTLENECK: Evaluator ({:.1}%)", eval_pct);
        println!("   Optimization target: Reduce evaluation overhead");
    } else {
        println!("\n⚠️ No single dominant bottleneck (both ~50%)");
        println!("   Optimization target: Both parser and evaluator");
    }

    println!();
}

/// Test: Fibonacci Recursive - Computation Dominated
///
/// This should be evaluator-dominated (function calls, scope lookups)
#[test]
fn test_fibonacci_breakdown() {
    let program = r#"
        let fib = 10;
        let a = 0;
        let b = 1;
        let i = 2;
        while i <= fib {
            let temp = a + b;
            a = b;
            b = temp;
            i = i + 1;
        }
        b
    "#;

    let iterations = 1000;

    println!("\n=== Fibonacci(10) Breakdown (1000 iterations) ===");

    // Parse
    let parse_start = Instant::now();
    let mut asts = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let mut parser = Parser::new(program);
        let ast = parser.parse().expect("Parse should succeed");
        asts.push(ast);
    }
    let parse_time = parse_start.elapsed();

    // Eval
    let eval_start = Instant::now();
    for ast in &asts {
        let mut eval = Evaluator::new();
        for statement in ast.nodes() {
            let _ = eval.eval(statement);
        }
    }
    let eval_time = eval_start.elapsed();

    let total_time = parse_time + eval_time;

    let parse_us = parse_time.as_micros() as f64 / iterations as f64;
    let eval_us = eval_time.as_micros() as f64 / iterations as f64;
    let total_us = total_time.as_micros() as f64 / iterations as f64;

    println!("Parse:  {:.2} µs/iter", parse_us);
    println!("Eval:   {:.2} µs/iter", eval_us);
    println!("Total:  {:.2} µs/iter", total_us);

    let parse_pct = (parse_time.as_micros() as f64 / total_time.as_micros() as f64) * 100.0;
    let eval_pct = (eval_time.as_micros() as f64 / total_time.as_micros() as f64) * 100.0;

    println!();
    println!("Parse:  {:.1}%", parse_pct);
    println!("Eval:   {:.1}%", eval_pct);

    if eval_pct > 60.0 {
        println!("\n✅ Expected: Eval-dominated for computation");
    }

    println!();
}

/// Test: Large Program - Parse Dominated
///
/// This should be parser-dominated (many statements, large AST)
#[test]
fn test_large_program_breakdown() {
    // Generate large program (100 variable declarations)
    let mut program = String::new();
    for i in 0..100 {
        program.push_str(&format!("let var{} = {};\n", i, i));
    }

    let iterations = 100; // Fewer iterations for large program

    println!("\n=== Large Program (100 statements, 100 iterations) ===");

    // Parse
    let parse_start = Instant::now();
    let mut asts = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let mut parser = Parser::new(&program);
        let ast = parser.parse().expect("Parse should succeed");
        asts.push(ast);
    }
    let parse_time = parse_start.elapsed();

    // Eval
    let eval_start = Instant::now();
    for ast in &asts {
        let mut eval = Evaluator::new();
        for statement in ast.nodes() {
            let _ = eval.eval(statement);
        }
    }
    let eval_time = eval_start.elapsed();

    let total_time = parse_time + eval_time;

    let parse_us = parse_time.as_micros() as f64 / iterations as f64;
    let eval_us = eval_time.as_micros() as f64 / iterations as f64;
    let total_us = total_time.as_micros() as f64 / iterations as f64;

    println!("Parse:  {:.2} µs/iter", parse_us);
    println!("Eval:   {:.2} µs/iter", eval_us);
    println!("Total:  {:.2} µs/iter", total_us);

    let parse_pct = (parse_time.as_micros() as f64 / total_time.as_micros() as f64) * 100.0;
    let eval_pct = (eval_time.as_micros() as f64 / total_time.as_micros() as f64) * 100.0;

    println!();
    println!("Parse:  {:.1}%", parse_pct);
    println!("Eval:   {:.1}%", eval_pct);

    if parse_pct > 60.0 {
        println!("\n✅ Expected: Parse-dominated for large programs");
    }

    println!();
}

/// Test: Memory Allocation Profiling
///
/// Use system allocator stats to measure heap pressure
#[test]
fn test_memory_allocation_pressure() {
    let program = r#"
        let a = 10;
        let b = 20;
        let c = a + b;
        c
    "#;

    let iterations = 10000;

    println!("\n=== Memory Allocation Pressure (10000 iterations) ===");

    // Measure total allocations (parse + eval together)
    let start = Instant::now();
    for _ in 0..iterations {
        let mut parser = Parser::new(program);
        let ast = parser.parse().expect("Parse should succeed");

        let mut eval = Evaluator::new();
        for statement in ast.nodes() {
            let _ = eval.eval(statement);
        }
    }
    let duration = start.elapsed();

    let per_iter_us = duration.as_micros() as f64 / iterations as f64;
    let throughput = iterations as f64 / duration.as_secs_f64();

    println!("Time per iteration: {:.2} µs", per_iter_us);
    println!("Throughput: {:.0} iterations/sec", throughput);
    println!();
    println!("Note: Run with `valgrind --tool=massif` for heap profiling");
    println!("      Or use `cargo bench` with criterion for detailed stats");
    println!();
}

/// Test: Comparative Analysis
///
/// Compare simple vs complex programs to identify scaling behavior
#[test]
fn test_comparative_analysis() {
    println!("\n=== Comparative Analysis ===");

    let programs = vec![
        ("Empty", "", 10000),
        ("Simple", "let a = 10; a", 5000),
        ("Medium", "let a = 10; let b = 20; let c = a + b; c", 1000),
        (
            "Complex",
            "let x = 1; let y = 2; let z = x + y; let w = z * 2; w",
            1000,
        ),
    ];

    println!(
        "{:<10} {:>12} {:>12} {:>12}",
        "Program", "Parse µs", "Eval µs", "Total µs"
    );
    println!("{}", "=".repeat(50));

    for (name, program, iterations) in programs {
        // Parse
        let parse_start = Instant::now();
        let mut asts = Vec::with_capacity(iterations);
        for _ in 0..iterations {
            let mut parser = Parser::new(program);
            let ast = parser.parse().expect("Parse should succeed");
            asts.push(ast);
        }
        let parse_time = parse_start.elapsed();

        // Eval
        let eval_start = Instant::now();
        for ast in &asts {
            let mut eval = Evaluator::new();
            for statement in ast.nodes() {
                let _ = eval.eval(statement);
            }
        }
        let eval_time = eval_start.elapsed();

        let parse_us = parse_time.as_micros() as f64 / iterations as f64;
        let eval_us = eval_time.as_micros() as f64 / iterations as f64;
        let total_us = parse_us + eval_us;

        println!(
            "{:<10} {:>12.2} {:>12.2} {:>12.2}",
            name, parse_us, eval_us, total_us
        );
    }

    println!();
    println!("Observation: Look for non-linear scaling (O(n²) is bad)");
    println!();
}

/// Test: Generate Summary Report
///
/// Consolidate all profiling data into actionable recommendations
#[test]
fn test_generate_profiling_summary() {
    println!("\n============================================================");
    println!("INTERP-OPT-001: Profiling Summary");
    println!("============================================================");

    println!("\n📊 Run all profiling tests:");
    println!("   cargo test --test test_interp_opt_001_profiling --release -- --nocapture");

    println!("\n🔥 Generate flamegraph:");
    println!("   cargo flamegraph --test test_interp_opt_001_profiling");
    println!("   Open: flamegraph.svg");

    println!("\n📈 Run perf analysis:");
    println!("   perf stat -d cargo test --test test_interp_opt_001_profiling --release");
    println!("   Look for: cache-misses, branch-misses, instructions per cycle");

    println!("\n💾 Memory profiling:");
    println!("   valgrind --tool=massif cargo test --test test_interp_opt_001_profiling");
    println!("   ms_print massif.out.<pid>");

    println!("\n🎯 Next Steps:");
    println!("   1. Identify THE bottleneck (>50% of time)");
    println!("   2. Apply Amdahl's Law: Focus ONLY on the bottleneck");
    println!("   3. Fix root cause (not symptoms)");
    println!("   4. Measure again to validate speedup");

    println!("\n⚠️ NASA-Level Quality:");
    println!("   - NO GUESSING - Use actual profiling data");
    println!("   - NO PREMATURE OPTIMIZATION - Measure first");
    println!("   - NO INCREMENTAL FIXES - Attack the bottleneck");
    println!("   - ZERO TOLERANCE - Fix it right");

    println!("\n============================================================");
}
