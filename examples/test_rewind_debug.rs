use ruchyruchy::debugger::repl_debugger::{DebugCommand, DebugSession};

fn main() {
    let code = r#"
        let x = 5;
        let y = 10;
        let z = x + y;
    "#;

    let mut session = DebugSession::new(code).expect("Should create session");

    println!("Initial state: line {}", session.current_line());

    // Step 1
    session.execute_command(DebugCommand::Step).expect("Step 1");
    println!("After step 1: line {}", session.current_line());
    println!(
        "  x = {:?}",
        session.execute_command(DebugCommand::Print("x".to_string()))
    );

    // Step 2
    session.execute_command(DebugCommand::Step).expect("Step 2");
    println!("After step 2: line {}", session.current_line());
    println!(
        "  y = {:?}",
        session.execute_command(DebugCommand::Print("y".to_string()))
    );

    // Step 3
    session.execute_command(DebugCommand::Step).expect("Step 3");
    println!("After step 3: line {}", session.current_line());
    println!(
        "  z = {:?}",
        session.execute_command(DebugCommand::Print("z".to_string()))
    );

    // Rewind 1
    let result = session.execute_command(DebugCommand::Rewind(1));
    println!("\nAfter rewind 1: {:?}", result);
    println!("Current line: {}", session.current_line());
    println!(
        "  x = {:?}",
        session.execute_command(DebugCommand::Print("x".to_string()))
    );
    println!(
        "  y = {:?}",
        session.execute_command(DebugCommand::Print("y".to_string()))
    );
    println!(
        "  z = {:?}",
        session.execute_command(DebugCommand::Print("z".to_string()))
    );
}
