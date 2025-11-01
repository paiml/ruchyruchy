#[test]
fn test_arc_multiple_clones() {
    use ruchyruchy::interpreter::evaluator::Evaluator;
    use ruchyruchy::interpreter::parser::Parser;

    let code = r#"
        use std::sync::{Arc, Mutex};

        let counter = Arc::new(Mutex::new(0));
        let counter2 = Arc::clone(&counter);
        let counter3 = Arc::clone(&counter);

        let mut num = counter2.lock().unwrap();
        *num += 1;

        let mut num2 = counter3.lock().unwrap();
        *num2 += 1;

        let final_count = *counter.lock().unwrap();
        println("Final count:");
        println(final_count);
        assert(final_count == 2);
    "#;

    let mut parser = Parser::new(code);
    let ast = parser.parse().expect("Should parse");

    let mut eval = Evaluator::new();
    for statement in ast.nodes() {
        eval.eval(statement).expect("Should execute");
    }
}
