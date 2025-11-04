// JIT Benchmark Demo: Show real performance difference
//
// This demo compiles and runs a compute-heavy program through both
// the interpreter and JIT compiler, showing the speedup.

use ruchyruchy::interpreter::evaluator::Evaluator;
use ruchyruchy::interpreter::parser::Parser;
use ruchyruchy::jit::JitCompiler;
use std::time::Instant;

fn main() {
    // Test program: Nested loops with arithmetic (compute-intensive)
    let source = r#"
fun main() {
    let sum = 0;
    let i = 0;
    while i < 100 {
        let j = 0;
        while j < 100 {
            sum = sum + (i * j);
            j = j + 1;
        }
        i = i + 1;
    }
    return sum;
}
"#;

    println!("🚀 JIT Benchmark Demo");
    println!("==========================================");
    println!("Program: Nested loops (100x100) with arithmetic");
    println!();

    // Parse once
    let mut parser = Parser::new(source);
    let ast = parser.parse().expect("Parse failed");

    // ========================================
    // INTERPRETER BENCHMARK
    // ========================================
    println!("📊 Running Interpreter...");
    let mut evaluator = Evaluator::new();

    // Load functions
    for node in ast.nodes() {
        evaluator.eval(node).expect("Eval failed");
    }

    // Measure interpreter execution
    let interp_start = Instant::now();
    let iterations = 100;
    for _ in 0..iterations {
        evaluator
            .eval(&ruchyruchy::interpreter::parser::AstNode::FunctionCall {
                name: "main".to_string(),
                args: vec![],
            })
            .expect("Call failed");
    }
    let interp_time = interp_start.elapsed();
    let interp_per_iter = interp_time.as_micros() / iterations;

    println!(
        "   Time: {:?} ({} µs/iteration)",
        interp_time, interp_per_iter
    );

    // ========================================
    // JIT BENCHMARK
    // ========================================
    println!();
    println!("⚡ Running JIT Compiler...");

    let mut jit = JitCompiler::new().expect("JIT init failed");

    // Compile main function
    let compile_start = Instant::now();
    let mut main_fn: Option<fn() -> i64> = None;
    for node in ast.nodes() {
        if let ruchyruchy::interpreter::parser::AstNode::FunctionDef { name, params, body } = node {
            if name == "main" {
                main_fn = Some(
                    jit.compile_function_with_params(
                        params,
                        &ruchyruchy::interpreter::parser::AstNode::Block {
                            statements: body.clone(),
                        },
                    )
                    .expect("Compile main failed"),
                );
            }
        }
    }
    let compile_time = compile_start.elapsed();
    println!("   Compilation: {:?}", compile_time);

    let main_fn = main_fn.expect("No main function");

    // Measure JIT execution
    let jit_start = Instant::now();
    for _ in 0..iterations {
        main_fn();
    }
    let jit_time = jit_start.elapsed();
    let jit_per_iter = jit_time.as_micros() / iterations;

    println!(
        "   Execution: {:?} ({} µs/iteration)",
        jit_time, jit_per_iter
    );

    // ========================================
    // RESULTS
    // ========================================
    println!();
    println!("📈 Performance Comparison");
    println!("==========================================");
    println!("Interpreter: {} µs/iter", interp_per_iter);
    println!("JIT:         {} µs/iter", jit_per_iter);

    let speedup = interp_per_iter as f64 / jit_per_iter as f64;
    println!();
    if speedup > 1.0 {
        println!("✨ JIT is {:.2}x FASTER than interpreter!", speedup);
    } else {
        println!(
            "⚠️  Interpreter is {:.2}x faster (overhead cost)",
            1.0 / speedup
        );
    }

    println!();
    println!("💡 JIT Overhead:");
    println!("   Compilation time: {:?}", compile_time);
    println!(
        "   Break-even point: ~{} iterations",
        compile_time.as_micros() / (interp_per_iter.saturating_sub(jit_per_iter))
    );
}
