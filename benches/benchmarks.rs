//! Criterion benchmarks for ruchyruchy JIT compiler.
//!
//! Benchmarks tokenization, parsing, evaluation, and scope management
//! which are the hot paths in the interpreter and JIT pipeline.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use ruchyruchy::interpreter::{Evaluator, Parser, Scope, Value};

const SIMPLE_EXPR: &str = "let x = 1 + 2 * 3;";
const FUNCTION_DEF: &str = r#"
fn add(a, b) {
    return a + b;
}
"#;
const LOOP_PROGRAM: &str = r#"
let sum = 0;
let i = 0;
while i < 10 {
    sum = sum + i;
    i = i + 1;
}
"#;
const NESTED_EXPR: &str = "let result = (1 + 2) * (3 + 4) - (5 * 6) / (7 + 1);";

fn bench_parse_simple_expr(c: &mut Criterion) {
    c.bench_function("parse_simple_expr", |b| {
        b.iter(|| {
            let mut parser = Parser::new(black_box(SIMPLE_EXPR));
            black_box(parser.parse());
        });
    });
}

fn bench_parse_function(c: &mut Criterion) {
    c.bench_function("parse_function_def", |b| {
        b.iter(|| {
            let mut parser = Parser::new(black_box(FUNCTION_DEF));
            black_box(parser.parse());
        });
    });
}

fn bench_parse_loop(c: &mut Criterion) {
    c.bench_function("parse_while_loop", |b| {
        b.iter(|| {
            let mut parser = Parser::new(black_box(LOOP_PROGRAM));
            black_box(parser.parse());
        });
    });
}

fn bench_parse_nested_expr(c: &mut Criterion) {
    c.bench_function("parse_nested_expr", |b| {
        b.iter(|| {
            let mut parser = Parser::new(black_box(NESTED_EXPR));
            black_box(parser.parse());
        });
    });
}

fn bench_eval_simple(c: &mut Criterion) {
    c.bench_function("eval_simple_expr", |b| {
        b.iter(|| {
            let mut parser = Parser::new(black_box(SIMPLE_EXPR));
            let ast = parser.parse().unwrap();
            let mut evaluator = Evaluator::new();
            for node in ast.nodes() {
                let _ = evaluator.eval(node);
            }
        });
    });
}

fn bench_eval_loop(c: &mut Criterion) {
    c.bench_function("eval_while_loop_10_iters", |b| {
        b.iter(|| {
            let mut parser = Parser::new(black_box(LOOP_PROGRAM));
            let ast = parser.parse().unwrap();
            let mut evaluator = Evaluator::new();
            for node in ast.nodes() {
                let _ = evaluator.eval(node);
            }
        });
    });
}

fn bench_scope_operations(c: &mut Criterion) {
    c.bench_function("scope_define_get_100_vars", |b| {
        b.iter(|| {
            let mut scope = Scope::new();
            for i in 0..100 {
                let name = format!("var_{i}");
                let _ = scope.define(name, Value::Integer(i));
            }
            for i in 0..100 {
                let name = format!("var_{i}");
                black_box(scope.get_cloned(&name));
            }
        });
    });
}

fn bench_parse_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_scaling");
    for n_stmts in [1, 5, 10, 25] {
        let program: String = (0..n_stmts)
            .map(|i| format!("let v{i} = {i} + {i} * 2;\n"))
            .collect();
        group.bench_with_input(
            BenchmarkId::new("statements", n_stmts),
            &program,
            |b, program| {
                b.iter(|| {
                    let mut parser = Parser::new(black_box(program));
                    black_box(parser.parse());
                });
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_parse_simple_expr,
    bench_parse_function,
    bench_parse_loop,
    bench_parse_nested_expr,
    bench_eval_simple,
    bench_eval_loop,
    bench_scope_operations,
    bench_parse_scaling,
);
criterion_main!(benches);
