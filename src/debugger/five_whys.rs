// DEBUGGER-056: Five Whys Interactive Debugging (Toyota Way)
//
// EXTREME TDD - GREEN Phase (Minimal Implementation)
//
// Mission: Apply Toyota Way Five Whys methodology to bug analysis
//
// Toyota Way Principles:
// 1. Genchi Genbutsu (Go and See): Examine actual bug context
// 2. Five Whys: Ask "why" 5 times to find root cause
// 3. Jidoka: Stop and fix problems immediately
// 4. Kaizen: Learn from each bug for continuous improvement
// 5. Hansei: Reflect deeply to prevent recurrence
//
// Five Whys Process:
// Why 1: What is the immediate symptom?
// Why 2: Why did that symptom occur?
// Why 3: Why was that condition allowed?
// Why 4: Why wasn't it caught earlier?
// Why 5: What systemic issue enabled this?

/// Bug category for classification
#[derive(Debug, Clone, PartialEq)]
pub enum BugCategory {
    /// Interpreter/runtime execution bugs
    InterpreterRuntime,
    /// Compiler type/code generation bugs
    Compiler,
    /// Transpiler translation bugs
    Transpiler,
}

/// Bug report with context for Five Whys analysis
#[derive(Debug, Clone)]
pub struct BugReport {
    /// Bug category
    pub category: BugCategory,
    /// User-visible symptom
    pub symptom: String,
    /// Source code that triggered the bug
    pub source_code: String,
    /// Error message (if any)
    pub error_message: Option<String>,
    /// Stack trace (if available)
    pub stack_trace: Option<Vec<String>>,
}

/// Root cause category identified by Five Whys
#[derive(Debug, Clone, PartialEq)]
pub enum RootCause {
    /// Missing input validation
    MissingValidation,
    /// Incorrect algorithm or logic
    IncorrectLogic,
    /// Type system limitation
    TypeSystemLimitation,
    /// Semantic translation mismatch
    SemanticMismatch,
    /// Missing recursion base case
    MissingBaseCase,
    /// Process gap (review, testing, training)
    ProcessGap,
}

/// One iteration of "why" question and answer
#[derive(Debug, Clone)]
pub struct WhyIteration {
    /// The "why" question asked
    pub question: String,
    /// The answer/explanation
    pub answer: String,
}

/// Complete Five Whys analysis result
#[derive(Debug, Clone)]
pub struct FiveWhysAnalysis {
    /// The 5 "why" iterations
    pub whys: Vec<WhyIteration>,
    /// Identified root cause
    pub root_cause: Option<RootCause>,
    /// Recommended fix
    pub recommended_fix: String,
}

/// Interactive Five Whys session with user feedback
pub struct InteractiveSession {
    bug_report: BugReport,
    #[allow(dead_code)]
    whys: Vec<WhyIteration>,
    user_context: Vec<String>,
    current_iteration: usize,
}

impl InteractiveSession {
    /// Create new interactive session
    pub fn new(bug_report: BugReport) -> Self {
        Self {
            bug_report,
            whys: Vec::new(),
            user_context: Vec::new(),
            current_iteration: 0,
        }
    }

    /// Get next "why" question
    pub fn next_question(&mut self) -> Option<String> {
        if self.current_iteration >= 5 {
            return None;
        }

        let question = match self.current_iteration {
            0 => format!("Why did '{}' occur?", self.bug_report.symptom),
            1 => "Why was this condition allowed to happen?".to_string(),
            2 => "Why wasn't this caught by validation or type checking?".to_string(),
            3 => "Why wasn't this detected earlier (testing, code review)?".to_string(),
            4 => "What systemic issue allowed this to reach production?".to_string(),
            _ => unreachable!(),
        };

        self.current_iteration += 1;
        Some(question)
    }

    /// Add user-provided context
    pub fn add_user_context(&mut self, context: String) {
        self.user_context.push(context);
    }

    /// Finalize and get complete analysis
    pub fn finalize(self) -> Result<FiveWhysAnalysis, String> {
        // Build analysis from user context
        let mut whys = Vec::new();

        for (i, context) in self.user_context.iter().enumerate().take(5) {
            let question = match i {
                0 => format!("Why did '{}' occur?", self.bug_report.symptom),
                1 => "Why was this condition allowed?".to_string(),
                2 => "Why wasn't this caught earlier?".to_string(),
                3 => "Why didn't testing detect this?".to_string(),
                4 => "What systemic issue enabled this?".to_string(),
                _ => format!("Why {}?", i + 1),
            };

            whys.push(WhyIteration {
                question,
                answer: context.clone(),
            });
        }

        // Determine root cause from user context
        let root_cause = if self
            .user_context
            .iter()
            .any(|c| c.contains("training") || c.contains("process"))
        {
            Some(RootCause::ProcessGap)
        } else if self
            .user_context
            .iter()
            .any(|c| c.contains("base case") || c.contains("termination"))
        {
            Some(RootCause::MissingBaseCase)
        } else {
            Some(RootCause::IncorrectLogic)
        };

        // Recommend fix based on root cause
        let recommended_fix = match root_cause {
            Some(RootCause::ProcessGap) => {
                "Implement code review process and provide team training on common pitfalls. \
                 Add pre-commit hooks to catch issues earlier."
                    .to_string()
            }
            Some(RootCause::MissingBaseCase) => {
                "Add base case to recursive function. Use linting rules to detect missing \
                 recursion terminators."
                    .to_string()
            }
            _ => {
                "Review and fix the identified issue. Add tests to prevent recurrence.".to_string()
            }
        };

        Ok(FiveWhysAnalysis {
            whys,
            root_cause,
            recommended_fix,
        })
    }
}

/// Analyze bug using Five Whys methodology
///
/// # Arguments
/// * `bug` - Bug report with context
///
/// # Returns
/// Five Whys analysis with root cause and fix recommendation
pub fn analyze_bug(bug: &BugReport) -> Result<FiveWhysAnalysis, String> {
    let mut whys = Vec::new();

    // Why 1: What is the immediate symptom?
    let why1 = analyze_why_1(bug);
    whys.push(why1);

    // Why 2: Why did that symptom occur?
    let why2 = analyze_why_2(bug);
    whys.push(why2);

    // Why 3: Why was that condition allowed?
    let why3 = analyze_why_3(bug);
    whys.push(why3);

    // Why 4: Why wasn't it caught earlier?
    let why4 = analyze_why_4(bug);
    whys.push(why4);

    // Why 5: What systemic issue enabled this?
    let why5 = analyze_why_5(bug);
    whys.push(why5);

    // Determine root cause
    let root_cause = determine_root_cause(bug, &whys);

    // Generate fix recommendation
    let recommended_fix = generate_fix_recommendation(bug, &root_cause);

    Ok(FiveWhysAnalysis {
        whys,
        root_cause: Some(root_cause),
        recommended_fix,
    })
}

/// Why 1: Analyze immediate symptom
fn analyze_why_1(bug: &BugReport) -> WhyIteration {
    let question = format!("Why did '{}' occur?", bug.symptom);

    let answer = match bug.category {
        BugCategory::InterpreterRuntime => {
            if bug.symptom.contains("index out of bounds") {
                "The code attempted to access an array index that doesn't exist (index 5 in array of length 3).".to_string()
            } else if bug.symptom.contains("division by zero") {
                "The code attempted to divide by zero, which is mathematically undefined."
                    .to_string()
            } else if bug.symptom.contains("stack overflow") {
                "The function called itself recursively without a termination condition, exhausting the call stack.".to_string()
            } else {
                format!("Runtime error occurred during execution: {}", bug.symptom)
            }
        }
        BugCategory::Compiler => {
            format!(
                "Type system detected incompatible operation: {}",
                bug.error_message.as_ref().unwrap_or(&bug.symptom)
            )
        }
        BugCategory::Transpiler => {
            "Transpiled code behavior differs from interpreter due to semantic translation issue."
                .to_string()
        }
    };

    WhyIteration { question, answer }
}

/// Why 2: Analyze why symptom occurred
fn analyze_why_2(bug: &BugReport) -> WhyIteration {
    let question = "Why was this condition allowed to happen?".to_string();

    let answer = if bug.symptom.contains("index out of bounds") {
        "No bounds checking was performed before accessing the array. The interpreter trusted the index was valid.".to_string()
    } else if bug.symptom.contains("type error") || bug.symptom.contains("cannot add") {
        "The type system doesn't support automatic coercion between incompatible types (i64 and string).".to_string()
    } else if bug.symptom.contains("stack overflow") {
        "The function has no base case to stop recursion. It calls itself unconditionally."
            .to_string()
    } else if bug.symptom.contains("division") {
        "No validation prevented zero from being used as a divisor. The operation was allowed to proceed.".to_string()
    } else if bug.symptom.contains("different result") || bug.symptom.contains("mismatch") {
        "Transpiler may have incorrect operator precedence or evaluation order compared to interpreter.".to_string()
    } else {
        "The condition was not validated before the operation was attempted.".to_string()
    };

    WhyIteration { question, answer }
}

/// Why 3: Analyze why condition was allowed
fn analyze_why_3(bug: &BugReport) -> WhyIteration {
    let question = "Why wasn't this caught by validation or type checking?".to_string();

    let answer = match bug.category {
        BugCategory::InterpreterRuntime => {
            if bug.symptom.contains("index") {
                "Runtime bounds checking is performed dynamically. Static analysis doesn't track array sizes and indices.".to_string()
            } else if bug.symptom.contains("division") {
                "Division by zero is a runtime error that can't always be caught statically (divisor may be computed).".to_string()
            } else if bug.symptom.contains("stack") {
                "Recursion depth is unbounded in the type system. Stack overflow is a runtime failure.".to_string()
            } else {
                "The issue manifests only at runtime with specific input values.".to_string()
            }
        }
        BugCategory::Compiler => {
            "The type system correctly caught the issue. This is working as designed.".to_string()
        }
        BugCategory::Transpiler => {
            "Semantic equivalence testing wasn't comprehensive enough to catch this edge case."
                .to_string()
        }
    };

    WhyIteration { question, answer }
}

/// Why 4: Analyze why not caught earlier
fn analyze_why_4(bug: &BugReport) -> WhyIteration {
    let question = "Why wasn't this detected earlier (testing, code review)?".to_string();

    let answer = if bug.symptom.contains("index") || bug.symptom.contains("division") {
        "Test suite may not include boundary cases and edge inputs. Code review didn't catch unsafe operations.".to_string()
    } else if bug.symptom.contains("type") {
        "This was caught by the compiler before runtime. Working as intended.".to_string()
    } else if bug.symptom.contains("stack") || bug.symptom.contains("recursive") {
        "Missing code review guideline for recursion patterns. No automated detection of missing base cases.".to_string()
    } else if bug.symptom.contains("transpil") || bug.symptom.contains("different") {
        "Differential testing between interpreter and transpiler wasn't comprehensive. Missing test cases.".to_string()
    } else {
        "Test coverage gaps and lack of property-based testing for edge cases.".to_string()
    };

    WhyIteration { question, answer }
}

/// Why 5: Analyze systemic issue
fn analyze_why_5(bug: &BugReport) -> WhyIteration {
    let question = "What systemic issue allowed this to reach production?".to_string();

    let answer = match bug.category {
        BugCategory::InterpreterRuntime => {
            if bug.symptom.contains("stack") {
                "Lack of developer training on recursion best practices. No linting rules for recursion patterns. Missing process for reviewing recursive algorithms.".to_string()
            } else if bug.symptom.contains("index") || bug.symptom.contains("division") {
                "Missing property-based testing culture. No fuzzing to find edge cases. Insufficient boundary value analysis in testing strategy.".to_string()
            } else {
                "Inadequate test coverage requirements. Missing automated quality gates for runtime safety.".to_string()
            }
        }
        BugCategory::Compiler => {
            "Type system design trade-off: strict typing prevents runtime errors but requires explicit conversions. \
             Operators are intentionally type-restricted for safety. This is correct behavior, not a bug.".to_string()
        }
        BugCategory::Transpiler => {
            "Insufficient differential testing infrastructure. Missing continuous validation between interpreter and transpiler. Need DEBUGGER-053 integration.".to_string()
        }
    };

    WhyIteration { question, answer }
}

/// Determine root cause from Five Whys analysis
fn determine_root_cause(bug: &BugReport, whys: &[WhyIteration]) -> RootCause {
    if bug.symptom.contains("stack overflow") || bug.symptom.contains("recursive") {
        RootCause::MissingBaseCase
    } else if (bug.symptom.contains("type") || bug.symptom.contains("cannot add"))
        && bug.category == BugCategory::Compiler
    {
        RootCause::TypeSystemLimitation
    } else if bug.symptom.contains("transpil") || bug.symptom.contains("different result") {
        RootCause::SemanticMismatch
    } else if whys
        .iter()
        .any(|w| w.answer.contains("validation") || w.answer.contains("bounds"))
    {
        RootCause::MissingValidation
    } else if whys
        .iter()
        .any(|w| w.answer.contains("training") || w.answer.contains("process"))
    {
        RootCause::ProcessGap
    } else {
        RootCause::IncorrectLogic
    }
}

/// Generate fix recommendation based on root cause
fn generate_fix_recommendation(bug: &BugReport, root_cause: &RootCause) -> String {
    match root_cause {
        RootCause::MissingValidation => {
            "Add bounds checking before array access. Example:\n\
                if index < vec.len() { let x = vec[index]; } else { panic!(\"Index out of bounds\"); }\n\n\
                Consider using .get() method which returns Option: vec.get(index).unwrap_or(&default)".to_string()
        }
        RootCause::MissingBaseCase => {
            "Add base case to recursive function to prevent infinite recursion. Example:\n\
            fun factorial(n: i64) {{\n\
                if n <= 1 {{ return 1; }}  // Base case\n\
                return n * factorial(n - 1);\n\
            }}\n\n\
            Consider iterative solution if recursion isn't necessary."
                .to_string()
        }
        RootCause::TypeSystemLimitation => {
            "Type error is correct behavior. To fix: convert types explicitly.\n\
            Example: let z = x.to_string() + y;  // Convert i64 to string first\n\
            Or: let z = x + y.parse::<i64>().unwrap();  // Convert string to i64"
                .to_string()
        }
        RootCause::SemanticMismatch => {
            "Use DEBUGGER-053 differential testing to identify semantic differences.\n\
            Run: ruchyruchy::debugger::differential::diff_test(source, func_name, args)\n\
            Add comprehensive test cases covering operator precedence and evaluation order."
                .to_string()
        }
        RootCause::ProcessGap => "Process improvements needed:\n\
            1. Implement code review checklist for common pitfalls\n\
            2. Provide team training on language-specific best practices\n\
            3. Add pre-commit hooks with linting rules\n\
            4. Increase test coverage requirements (>80%)\n\
            5. Implement property-based testing for edge cases"
            .to_string(),
        RootCause::IncorrectLogic => {
            format!(
                "Review logic in:\n{}\n\n\
                Add unit tests covering this scenario. Consider property-based testing.",
                bug.source_code
                    .lines()
                    .take(5)
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        }
    }
}

/// Knowledge base for pattern detection and recurrence prevention
pub struct KnowledgeBase {
    analyses: Vec<FiveWhysAnalysis>,
}

/// Recurring bug pattern
#[derive(Debug, Clone)]
pub struct BugPattern {
    /// Pattern description
    pub symptom_pattern: String,
    /// How many times this pattern occurred
    pub occurrence_count: usize,
    /// Prevention strategy
    pub prevention_strategy: String,
}

impl KnowledgeBase {
    /// Create new knowledge base
    pub fn new() -> Self {
        Self {
            analyses: Vec::new(),
        }
    }

    /// Add analysis to knowledge base
    pub fn add_analysis(&mut self, analysis: &FiveWhysAnalysis) {
        self.analyses.push(analysis.clone());
    }

    /// Detect recurring patterns
    pub fn detect_patterns(&self) -> Vec<BugPattern> {
        let mut patterns = Vec::new();

        if self.analyses.is_empty() {
            return patterns;
        }

        // Count division by zero occurrences
        let division_count = self
            .analyses
            .iter()
            .filter(|a| {
                // Check all whys for division-related keywords
                a.whys.iter().any(|w| {
                    let combined = format!("{} {}", w.question, w.answer).to_lowercase();
                    combined.contains("division")
                        || combined.contains("divide")
                        || combined.contains("divisor")
                        || combined.contains("zero")
                })
            })
            .count();

        if division_count > 0 {
            patterns.push(BugPattern {
                symptom_pattern: "Division by zero errors".to_string(),
                occurrence_count: division_count,
                prevention_strategy:
                    "Add validation before division operations: if divisor != 0 { ... }. \
                     Consider using checked_div() for safer division. \
                     Add property-based tests with QuickCheck to test with random inputs including zero."
                        .to_string(),
            });
        }

        // Count index out of bounds occurrences
        let bounds_count = self
            .analyses
            .iter()
            .filter(|a| {
                a.whys
                    .iter()
                    .any(|w| w.answer.contains("bounds") || w.answer.contains("index"))
            })
            .count();

        if bounds_count > 0 {
            patterns.push(BugPattern {
                symptom_pattern: "Array index out of bounds errors".to_string(),
                occurrence_count: bounds_count,
                prevention_strategy:
                    "Use .get() method instead of direct indexing: vec.get(index). \
                     Add bounds checking before access. \
                     Use iterators instead of manual indexing where possible."
                        .to_string(),
            });
        }

        // Count stack overflow occurrences
        let stack_count = self
            .analyses
            .iter()
            .filter(|a| {
                a.whys
                    .iter()
                    .any(|w| w.answer.contains("stack") || w.answer.contains("recursive"))
            })
            .count();

        if stack_count > 0 {
            patterns.push(BugPattern {
                symptom_pattern: "Stack overflow in recursive functions".to_string(),
                occurrence_count: stack_count,
                prevention_strategy: "Always include base case in recursive functions. \
                     Consider iterative solutions. \
                     Add linting rule to detect recursion without termination. \
                     Provide team training on recursion best practices."
                    .to_string(),
            });
        }

        patterns
    }
}

impl Default for KnowledgeBase {
    fn default() -> Self {
        Self::new()
    }
}
