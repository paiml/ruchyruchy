// DEBUGGER-046: Interactive REPL Debugger (GREEN Phase)
//
// Minimal implementation to make RED phase tests pass.
// Based on bashrs REPL debugger pattern (matklad's debugger-as-REPL).
//
// This implements:
// - DebugSession: Stateful debugging session with execution history
// - DebugCommand: All debug commands (step, print, break, continue, etc.)
// - Time-travel: Record/replay execution for rewind capability
//
// Research-backed: bashrs shows 10x faster debugging with REPL approach

use crate::interpreter::{Ast, Evaluator, Parser};
use std::collections::HashSet;

/// Debug commands available in REPL
#[derive(Debug, Clone, PartialEq)]
pub enum DebugCommand {
    /// Execute one statement and stop
    Step,
    /// Inspect variable value
    Print(String),
    /// Set breakpoint at line
    Break(usize),
    /// Run until next breakpoint or completion
    Continue,
    /// Show current AST node
    Ast,
    /// Display call stack
    Backtrace,
    /// Time-travel backward n steps
    Rewind(usize),
    /// Show available commands
    Help,
}

/// Result of executing a debug command
pub type CommandResult = Result<String, String>;

/// Execution state snapshot for time-travel
#[derive(Debug, Clone)]
struct ExecutionSnapshot {
    line: usize,
    evaluator: Evaluator,
}

/// Interactive debugging session with state tracking
pub struct DebugSession {
    /// Parsed AST of the program
    ast: Ast,
    /// Current evaluator state
    evaluator: Evaluator,
    /// Current execution line (0-indexed)
    current_line: usize,
    /// Breakpoints (line numbers)
    breakpoints: HashSet<usize>,
    /// Execution history for time-travel
    history: Vec<ExecutionSnapshot>,
    /// Whether execution is finished
    finished: bool,
}

impl DebugSession {
    /// Create new debug session from source code
    ///
    /// Parses the source code and prepares for interactive debugging.
    pub fn new(source: &str) -> Result<Self, String> {
        let mut parser = Parser::new(source);
        let ast = parser
            .parse()
            .map_err(|e| format!("Parse error: {:?}", e))?;

        let evaluator = Evaluator::new();

        // Save initial state in history
        let initial_snapshot = ExecutionSnapshot {
            line: 0,
            evaluator: evaluator.clone(),
        };

        Ok(Self {
            ast,
            evaluator,
            current_line: 0,
            breakpoints: HashSet::new(),
            history: vec![initial_snapshot],
            finished: false,
        })
    }

    /// Get current execution line (0-indexed)
    pub fn current_line(&self) -> usize {
        self.current_line
    }

    /// Check if execution is finished
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    /// Check if breakpoint exists at line
    pub fn has_breakpoint_at(&self, line: usize) -> bool {
        self.breakpoints.contains(&line)
    }

    /// Execute a debug command
    pub fn execute_command(&mut self, command: DebugCommand) -> CommandResult {
        match command {
            DebugCommand::Step => self.cmd_step(),
            DebugCommand::Print(var) => self.cmd_print(&var),
            DebugCommand::Break(line) => self.cmd_break(line),
            DebugCommand::Continue => self.cmd_continue(),
            DebugCommand::Ast => self.cmd_ast(),
            DebugCommand::Backtrace => self.cmd_backtrace(),
            DebugCommand::Rewind(n) => self.cmd_rewind(n),
            DebugCommand::Help => self.cmd_help(),
        }
    }

    /// Execute one statement and stop
    fn cmd_step(&mut self) -> CommandResult {
        if self.finished {
            return Err("Execution already finished".to_string());
        }

        // Get nodes from AST
        let nodes = self.ast.nodes();

        if self.current_line >= nodes.len() {
            self.finished = true;
            return Ok("Execution complete".to_string());
        }

        // Execute current statement
        let node = &nodes[self.current_line];
        self.evaluator
            .eval(node)
            .map_err(|e| format!("Evaluation error: {:?}", e))?;

        // Save snapshot before advancing
        self.current_line += 1;

        let snapshot = ExecutionSnapshot {
            line: self.current_line,
            evaluator: self.evaluator.clone(),
        };
        self.history.push(snapshot);

        // Check if finished
        if self.current_line >= nodes.len() {
            self.finished = true;
        }

        Ok(format!("Stepped to line {}", self.current_line))
    }

    /// Inspect variable value
    fn cmd_print(&self, var_name: &str) -> CommandResult {
        // Try to get variable value from evaluator's scope
        match self.evaluator.get_variable(var_name) {
            Some(value) => Ok(format!("{} = {:?}", var_name, value)),
            None => Err(format!(
                "Variable '{}' not found in current scope",
                var_name
            )),
        }
    }

    /// Set breakpoint at line
    fn cmd_break(&mut self, line: usize) -> CommandResult {
        self.breakpoints.insert(line);
        Ok(format!("Breakpoint set at line {}", line))
    }

    /// Run until next breakpoint or completion
    fn cmd_continue(&mut self) -> CommandResult {
        if self.finished {
            return Err("Execution already finished".to_string());
        }

        let nodes = self.ast.nodes();

        // Keep stepping until breakpoint hit or completion
        while self.current_line < nodes.len() {
            // Check if we're at a breakpoint (before executing)
            if self.breakpoints.contains(&self.current_line) {
                return Ok(format!("Breakpoint hit at line {}", self.current_line));
            }

            // Execute current statement
            let node = &nodes[self.current_line];
            self.evaluator
                .eval(node)
                .map_err(|e| format!("Evaluation error: {:?}", e))?;

            // Save snapshot
            self.current_line += 1;
            let snapshot = ExecutionSnapshot {
                line: self.current_line,
                evaluator: self.evaluator.clone(),
            };
            self.history.push(snapshot);
        }

        self.finished = true;
        Ok("Execution complete".to_string())
    }

    /// Show current AST node
    fn cmd_ast(&self) -> CommandResult {
        if self.current_line >= self.ast.nodes().len() {
            return Err("No current statement (execution finished)".to_string());
        }

        let node = &self.ast.nodes()[self.current_line];
        Ok(format!("{:#?}", node))
    }

    /// Display call stack
    fn cmd_backtrace(&self) -> CommandResult {
        // Get call stack from evaluator
        let call_stack = self.evaluator.get_call_stack();

        if call_stack.is_empty() {
            return Ok("(empty call stack - not in function)".to_string());
        }

        let mut output = String::from("Call stack:\n");
        for (i, frame) in call_stack.iter().enumerate() {
            output.push_str(&format!("  #{}: {}\n", i, frame));
        }

        Ok(output)
    }

    /// Time-travel backward n steps
    fn cmd_rewind(&mut self, n: usize) -> CommandResult {
        if n == 0 {
            return Err("Rewind count must be > 0".to_string());
        }

        if n >= self.history.len() {
            return Err(format!(
                "Cannot rewind {} steps (only {} snapshots available)",
                n,
                self.history.len() - 1
            ));
        }

        // Get target snapshot index
        let target_index = self.history.len() - n - 1;

        // Restore state from snapshot
        let snapshot = &self.history[target_index];
        self.current_line = snapshot.line;
        self.evaluator = snapshot.evaluator.clone();
        self.finished = false;

        // Truncate history to this point
        self.history.truncate(target_index + 1);

        Ok(format!("Rewound {} steps to line {}", n, self.current_line))
    }

    /// Show available commands
    fn cmd_help(&self) -> CommandResult {
        Ok(r#"Available debug commands:
  :step              Execute one statement and stop
  :print <var>       Inspect variable value
  :break <line>      Set breakpoint at line
  :continue          Run until next breakpoint or completion
  :ast               Show current AST node structure
  :backtrace         Display call stack
  :rewind <n>        Time-travel backward n steps
  :help              Show this help message
"#
        .to_string())
    }
}

/// Result of a step operation
#[derive(Debug, Clone, PartialEq)]
pub enum StepResult {
    /// Stepped successfully
    Stepped,
    /// Hit breakpoint
    Breakpoint(usize),
    /// Execution finished
    Finished,
}
