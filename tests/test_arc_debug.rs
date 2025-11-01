#[test]
fn test_arc_debug() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        use std::sync::{Arc, Mutex};

        let counter = Arc::new(Mutex::new(0));
        let counter2 = Arc::clone(&counter);

        let mut num = counter2.lock().unwrap();
        *num += 1;

        let final = *counter.lock().unwrap();
        println("Final value:");
        println(final);
        assert(final == 1);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement).expect("Should execute");
    }
}
