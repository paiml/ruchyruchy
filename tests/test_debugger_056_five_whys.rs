// DEBUGGER-056: Five Whys Interactive Debugging (Toyota Way)
//
// EXTREME TDD - RED Phase
//
// Mission: Build interactive Five Whys analyzer for interpreter/compiler/transpiler bugs
//
// Toyota Way Principles:
// 1. Genchi Genbutsu (Go and See): Examine actual bug in context
// 2. Five Whys: Ask "why" 5 times to find root cause
// 3. Jidoka: Stop and fix problems immediately
// 4. Kaizen: Continuous improvement through learning
// 5. Hansei: Reflect on failures to prevent recurrence
//
// Bug Categories:
// - Interpreter/Runtime: Execution errors, panics, wrong values
// - Compiler: Type errors, code generation issues
// - Transpiler: Translation bugs, semantic mismatches
//
// Acceptance Criteria:
// - All 5 tests passing
// - Interactive analysis with 5 "why" questions
// - Root cause identification
// - Actionable fix recommendations
// - Knowledge base building for recurrence prevention

/// Test 1: Analyze interpreter runtime panic with Five Whys
///
/// Validates that Five Whys can trace from symptom to root cause for panics.
#[test]
fn test_five_whys_interpreter_panic() {
    let bug_report = ruchyruchy::debugger::five_whys::BugReport {
        category: ruchyruchy::debugger::five_whys::BugCategory::InterpreterRuntime,
        symptom: "Panic: index out of bounds accessing vector".to_string(),
        source_code: r#"
fun main() {
    let vec = [1, 2, 3];
    let x = vec[5];  // Out of bounds
    println(x);
}
"#
        .to_string(),
        error_message: Some(
            "thread panicked at 'index out of bounds: the len is 3 but the index is 5'".to_string(),
        ),
        stack_trace: Some(vec![
            "at interpreter::evaluate_index_access".to_string(),
            "at interpreter::evaluate_expression".to_string(),
            "at interpreter::evaluate_statement".to_string(),
        ]),
    };

    // Run Five Whys analysis
    let analysis = ruchyruchy::debugger::five_whys::analyze_bug(&bug_report);

    assert!(analysis.is_ok(), "Five Whys analysis must succeed");

    let result = analysis.unwrap();

    // Must have 5 "why" questions and answers
    assert_eq!(
        result.whys.len(),
        5,
        "Must ask 'why' 5 times (found {})",
        result.whys.len()
    );

    // First why should address immediate symptom
    assert!(
        result.whys[0].question.contains("index out of bounds"),
        "Why 1 must address symptom: {}",
        result.whys[0].question
    );

    // Fifth why should identify root cause
    assert!(
        result.whys[4].answer.len() > 20,
        "Root cause answer must be detailed (found {} chars)",
        result.whys[4].answer.len()
    );

    // Must identify root cause category
    assert!(
        result.root_cause.is_some(),
        "Must identify root cause category"
    );

    // Must provide actionable fix
    assert!(
        !result.recommended_fix.is_empty(),
        "Must recommend actionable fix"
    );

    assert!(
        result.recommended_fix.len() > 20,
        "Fix recommendation must be detailed (found {} chars)",
        result.recommended_fix.len()
    );

    println!("Five Whys Analysis:");
    for (i, why) in result.whys.iter().enumerate() {
        println!("  Why {}: {}", i + 1, why.question);
        println!("  Answer: {}", why.answer);
    }
    println!("Root Cause: {:?}", result.root_cause);
    println!("Fix: {}", result.recommended_fix);
}

/// Test 2: Analyze compiler type error with Five Whys
///
/// Validates root cause analysis for type system failures.
#[test]
fn test_five_whys_compiler_type_error() {
    let bug_report = ruchyruchy::debugger::five_whys::BugReport {
        category: ruchyruchy::debugger::five_whys::BugCategory::Compiler,
        symptom: "Type error: cannot add i64 and string".to_string(),
        source_code: r#"
fun main() {
    let x: i64 = 42;
    let y: string = "hello";
    let z = x + y;  // Type mismatch
}
"#
        .to_string(),
        error_message: Some(
            "type error: binary operator '+' cannot be applied to types 'i64' and 'string'"
                .to_string(),
        ),
        stack_trace: None,
    };

    let analysis = ruchyruchy::debugger::five_whys::analyze_bug(&bug_report);

    assert!(analysis.is_ok(), "Five Whys analysis must succeed");

    let result = analysis.unwrap();

    assert_eq!(result.whys.len(), 5, "Must have 5 why iterations");

    // Must trace to type system design decision
    let root_cause_answer = &result.whys[4].answer;
    assert!(
        root_cause_answer.contains("type") || root_cause_answer.contains("operator"),
        "Root cause must mention type system or operators: {}",
        root_cause_answer
    );

    // Must recommend type-safe solution
    assert!(
        result.recommended_fix.contains("convert")
            || result.recommended_fix.contains("cast")
            || result.recommended_fix.contains("to_string"),
        "Fix must recommend type conversion: {}",
        result.recommended_fix
    );

    println!("Compiler Error Analysis:");
    println!("Root Cause: {:?}", result.root_cause);
    println!("Fix: {}", result.recommended_fix);
}

/// Test 3: Analyze transpiler semantic mismatch with Five Whys
///
/// Validates analysis of translation bugs between languages.
#[test]
fn test_five_whys_transpiler_mismatch() {
    let bug_report = ruchyruchy::debugger::five_whys::BugReport {
        category: ruchyruchy::debugger::five_whys::BugCategory::Transpiler,
        symptom: "Transpiled code produces different result than interpreter".to_string(),
        source_code: r#"
fun factorial(n: i64) {
    if n <= 1 { return 1; }
    return n * factorial(n - 1);
}
"#
        .to_string(),
        error_message: Some(
            "Interpreter: factorial(5) = 120, Transpiled: factorial(5) = 720".to_string(),
        ),
        stack_trace: None,
    };

    let analysis = ruchyruchy::debugger::five_whys::analyze_bug(&bug_report);

    assert!(analysis.is_ok(), "Five Whys analysis must succeed");

    let result = analysis.unwrap();

    assert_eq!(result.whys.len(), 5, "Must have 5 why iterations");

    // Must identify semantic difference as root cause
    let analysis_mentions_semantic = result
        .whys
        .iter()
        .any(|why| why.answer.contains("semantic") || why.answer.contains("translation"));

    assert!(
        analysis_mentions_semantic,
        "Analysis must mention semantic or translation issues"
    );

    // Must recommend differential testing
    assert!(
        result.recommended_fix.contains("test")
            || result.recommended_fix.contains("verify")
            || result.recommended_fix.contains("compare"),
        "Fix must recommend testing/verification: {}",
        result.recommended_fix
    );
}

/// Test 4: Interactive mode with user feedback
///
/// Validates that Five Whys can incorporate user input for deeper analysis.
#[test]
fn test_five_whys_interactive_mode() {
    let bug_report = ruchyruchy::debugger::five_whys::BugReport {
        category: ruchyruchy::debugger::five_whys::BugCategory::InterpreterRuntime,
        symptom: "Stack overflow in recursive function".to_string(),
        source_code: r#"
fun infinite() {
    infinite();
}
"#
        .to_string(),
        error_message: Some("thread has overflowed its stack".to_string()),
        stack_trace: Some(vec!["at infinite".to_string(); 1000]),
    };

    // Create interactive session
    let mut session = ruchyruchy::debugger::five_whys::InteractiveSession::new(bug_report);

    // Step through each "why" question
    for i in 0..5 {
        let question = session.next_question();
        assert!(question.is_some(), "Must have question for iteration {}", i);

        let q = question.unwrap();
        assert!(
            q.len() > 10,
            "Question must be meaningful (found {} chars)",
            q.len()
        );

        // Simulate user providing additional context
        let user_context = match i {
            0 => "The function has no base case",
            1 => "Developer forgot to add termination condition",
            2 => "Code was copied from example without understanding",
            3 => "No code review process caught the issue",
            4 => "Team lacks training on recursion best practices",
            _ => "Additional context",
        };

        session.add_user_context(user_context.to_string());
    }

    // Get final analysis
    let result = session.finalize();

    assert!(
        result.is_ok(),
        "Interactive session must complete successfully"
    );

    let analysis = result.unwrap();

    // Must incorporate user context in root cause
    assert!(
        analysis.root_cause.is_some(),
        "Must identify root cause with user input"
    );

    // Must have process improvement recommendation
    assert!(
        analysis.recommended_fix.contains("review")
            || analysis.recommended_fix.contains("training")
            || analysis.recommended_fix.contains("process"),
        "Fix must address process issues: {}",
        analysis.recommended_fix
    );

    println!("Interactive Analysis:");
    println!("Root Cause: {:?}", analysis.root_cause);
    println!("Process Fix: {}", analysis.recommended_fix);
}

/// Test 5: Knowledge base building for recurrence prevention
///
/// Validates that Five Whys analysis contributes to organizational learning.
#[test]
fn test_five_whys_knowledge_base() -> Result<(), String> {
    // Analyze multiple similar bugs
    let bugs = vec![
        ruchyruchy::debugger::five_whys::BugReport {
            category: ruchyruchy::debugger::five_whys::BugCategory::InterpreterRuntime,
            symptom: "Division by zero panic".to_string(),
            source_code: "fun main() { let x = 10 / 0; }".to_string(),
            error_message: Some("attempt to divide by zero".to_string()),
            stack_trace: None,
        },
        ruchyruchy::debugger::five_whys::BugReport {
            category: ruchyruchy::debugger::five_whys::BugCategory::InterpreterRuntime,
            symptom: "Division by zero in calculation".to_string(),
            source_code: "fun divide(a: i64, b: i64) { return a / b; }".to_string(),
            error_message: Some("attempt to divide by zero".to_string()),
            stack_trace: None,
        },
    ];

    let mut knowledge_base = ruchyruchy::debugger::five_whys::KnowledgeBase::new();

    // Analyze each bug and add to knowledge base
    for bug in &bugs {
        let analysis = ruchyruchy::debugger::five_whys::analyze_bug(bug)?;
        knowledge_base.add_analysis(&analysis);
    }

    // Check for pattern detection
    let patterns = knowledge_base.detect_patterns();

    println!("Detected {} patterns", patterns.len());
    for p in &patterns {
        println!(
            "  - {}: {} occurrences",
            p.symptom_pattern, p.occurrence_count
        );
    }

    assert!(
        !patterns.is_empty(),
        "Must detect recurring patterns in similar bugs"
    );

    // Must identify "division by zero" pattern
    let division_pattern = patterns.iter().find(|p| {
        p.symptom_pattern.to_lowercase().contains("division")
            || p.symptom_pattern.to_lowercase().contains("divide")
    });

    assert!(
        division_pattern.is_some(),
        "Must detect division by zero pattern"
    );

    let pattern = division_pattern.unwrap();

    assert!(
        pattern.occurrence_count >= 2,
        "Pattern must track occurrences (found {})",
        pattern.occurrence_count
    );

    // Must provide prevention strategy
    assert!(
        !pattern.prevention_strategy.is_empty(),
        "Must recommend prevention strategy"
    );

    println!("Knowledge Base Patterns:");
    for pattern in &patterns {
        println!("  Pattern: {}", pattern.symptom_pattern);
        println!("  Occurrences: {}", pattern.occurrence_count);
        println!("  Prevention: {}", pattern.prevention_strategy);
    }

    Ok(())
}

// Data structures for test assertions
// These will be implemented in src/debugger/five_whys.rs

#[allow(dead_code)]
struct FiveWhysAnalysis {
    whys: Vec<WhyIteration>,
    root_cause: Option<RootCause>,
    recommended_fix: String,
}

#[allow(dead_code)]
struct WhyIteration {
    question: String,
    answer: String,
}

#[allow(dead_code)]
#[derive(Debug)]
enum RootCause {
    MissingValidation,
    IncorrectLogic,
    TypeSystemLimitation,
    SemanticMismatch,
    MissingBaseCase,
    ProcessGap,
}
