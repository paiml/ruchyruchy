// DEBUGGER-050: AST Visualization Tools (GREEN Phase Priority 2)
//
// Pain Point (PARSER-079): 110k tokens spent debugging parser issues
// - Needed AST visualization to understand parser output
// - Needed JSON export for tool integration
// - Needed Graphviz export for visual debugging
// - Needed AST diff to compare working vs broken code
//
// Solution: Comprehensive AST visualization toolkit

use crate::interpreter::parser::{AstNode, Parser};

/// Generate AST as JSON for tool integration
///
/// DEBUGGER-050 Priority 2: JSON export enables automated analysis
pub fn visualize_ast(source: &str) -> String {
    let mut parser = Parser::new(source);
    match parser.parse() {
        Ok(ast) => {
            let nodes = ast.nodes();
            ast_to_json(nodes, 0)
        }
        Err(e) => format!("{{\"error\": \"{}\"}}", e),
    }
}

/// Generate AST as Graphviz DOT format for visualization
///
/// DEBUGGER-050 Priority 2: Visual debugging with graphviz
pub fn visualize_ast_graphviz(source: &str) -> String {
    let mut parser = Parser::new(source);
    match parser.parse() {
        Ok(ast) => {
            let mut output = String::from("digraph AST {\n");
            output.push_str("  node [shape=box];\n\n");

            let nodes = ast.nodes();
            let mut counter = 0;
            for node in nodes {
                ast_to_dot(node, &mut output, &mut counter, None);
            }

            output.push_str("}\n");
            output
        }
        Err(e) => format!("digraph AST {{\n  error [label=\"{}\"];\n}}\n", e),
    }
}

/// Generate AST with source location metadata
///
/// DEBUGGER-050 Priority 2: Line/column tracking for source mapping
pub fn visualize_ast_with_locations(source: &str) -> String {
    let mut parser = Parser::new(source);
    match parser.parse() {
        Ok(ast) => {
            let nodes = ast.nodes();
            // For now, approximate locations (full location tracking requires parser changes)
            let mut output = String::from("{\n  \"nodes\": [\n");
            for (i, node) in nodes.iter().enumerate() {
                if i > 0 {
                    output.push_str(",\n");
                }
                output.push_str("    {\n");
                output.push_str(&format!("      \"line\": {},\n", i + 1));
                output.push_str("      \"column\": 1,\n");
                output.push_str(&format!("      \"node\": {}\n", ast_node_to_json(node, 6)));
                output.push_str("    }");
            }
            output.push_str("\n  ]\n}\n");
            output
        }
        Err(e) => format!("{{\"error\": \"{}\"}}", e),
    }
}

/// Generate AST with error handling (returns Result)
///
/// DEBUGGER-050 Priority 2: Proper error handling for partial AST
pub fn visualize_ast_partial(source: &str) -> Result<String, String> {
    let mut parser = Parser::new(source);
    match parser.parse() {
        Ok(ast) => {
            let nodes = ast.nodes();
            Ok(ast_to_json(nodes, 0))
        }
        Err(e) => Err(format!("{}", e)),
    }
}

/// Compare ASTs from two versions (differential testing)
///
/// DEBUGGER-050 Priority 2: AST diff for understanding parser changes
pub fn ast_diff(before: &str, after: &str) -> String {
    let mut parser1 = Parser::new(before);
    let mut parser2 = Parser::new(after);

    let ast1 = parser1.parse();
    let ast2 = parser2.parse();

    match (ast1, ast2) {
        (Ok(ast1), Ok(ast2)) => {
            let nodes1 = ast1.nodes();
            let nodes2 = ast2.nodes();

            let mut output = String::from("AST Diff:\n=========\n\n");

            if nodes1.len() != nodes2.len() {
                output.push_str(&format!(
                    "Node count changed: {} -> {}\n\n",
                    nodes1.len(),
                    nodes2.len()
                ));
            }

            let min_len = nodes1.len().min(nodes2.len());
            for i in 0..min_len {
                let diff = node_diff(&nodes1[i], &nodes2[i]);
                if !diff.is_empty() {
                    output.push_str(&format!("Node {}:\n{}\n", i, diff));
                }
            }

            output
        }
        (Err(e), _) => format!("Error in 'before': {}\n", e),
        (_, Err(e)) => format!("Error in 'after': {}\n", e),
    }
}

/// Show AST construction step-by-step
///
/// DEBUGGER-050 Priority 2: Understand parser decisions at each token
pub fn visualize_ast_steps(source: &str) -> Vec<String> {
    let mut parser = Parser::new(source);

    // For now, show token-by-token parse
    // Full step-by-step requires instrumenting parser
    match parser.debug_get_tokens() {
        Ok(tokens) => tokens
            .iter()
            .enumerate()
            .map(|(i, token)| format!("Step {}: Parse {}", i + 1, token))
            .collect(),
        Err(_) => vec!["Error: Failed to tokenize".to_string()],
    }
}

/// Show inferred types in AST (for type debugging)
///
/// DEBUGGER-050 Priority 2: Type information overlay on AST
pub fn visualize_typed_ast(source: &str) -> String {
    let mut parser = Parser::new(source);
    match parser.parse() {
        Ok(ast) => {
            let nodes = ast.nodes();
            // For now, approximate types (full type inference requires evaluator integration)
            let mut output = String::from("{\n  \"nodes\": [\n");
            for (i, node) in nodes.iter().enumerate() {
                if i > 0 {
                    output.push_str(",\n");
                }
                output.push_str("    {\n");
                output.push_str(&format!(
                    "      \"inferred_type\": \"{}\",\n",
                    infer_type(node)
                ));
                output.push_str(&format!("      \"node\": {}\n", ast_node_to_json(node, 6)));
                output.push_str("    }");
            }
            output.push_str("\n  ]\n}\n");
            output
        }
        Err(e) => format!("{{\"error\": \"{}\"}}", e),
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Convert AST nodes to JSON
fn ast_to_json(nodes: &[AstNode], indent: usize) -> String {
    let indent_str = "  ".repeat(indent);
    let mut output = String::from("{\n");
    output.push_str(&format!("{}  \"nodes\": [\n", indent_str));

    for (i, node) in nodes.iter().enumerate() {
        if i > 0 {
            output.push_str(",\n");
        }
        output.push_str(&format!(
            "{}    {}",
            indent_str,
            ast_node_to_json(node, indent + 2)
        ));
    }

    output.push('\n');
    output.push_str(&format!("{}  ]\n", indent_str));
    output.push_str(&format!("{}}}", indent_str));
    output
}

/// Convert single AST node to JSON
fn ast_node_to_json(node: &AstNode, indent: usize) -> String {
    let indent_str = "  ".repeat(indent);
    match node {
        AstNode::FunctionDef { name, params, body } => {
            let mut body_json = String::from("[\n");
            for (i, stmt) in body.iter().enumerate() {
                if i > 0 {
                    body_json.push_str(",\n");
                }
                body_json.push_str(&format!(
                    "{}      {}",
                    indent_str,
                    ast_node_to_json(stmt, indent + 3)
                ));
            }
            body_json.push_str(&format!("\n{}    ]", indent_str));

            format!(
                "{{\n{}  \"type\": \"FunctionDef\",\n{}  \"name\": \"{}\",\n{}  \"params\": {:?},\n{}  \"body\": {}\n{}}}",
                indent_str, indent_str, name, indent_str, params, indent_str, body_json, indent_str
            )
        }
        AstNode::LetDecl { name, value } => {
            format!(
                "{{\n{}  \"type\": \"LetDecl\",\n{}  \"name\": \"{}\",\n{}  \"value\": {}\n{}}}",
                indent_str,
                indent_str,
                name,
                indent_str,
                ast_node_to_json(value, indent + 1),
                indent_str
            )
        }
        AstNode::IntegerLiteral(n) => {
            format!("{{\"type\": \"IntegerLiteral\", \"value\": {}}}", n)
        }
        AstNode::Return { value } => {
            if let Some(expr) = value {
                format!(
                    "{{\n{}  \"type\": \"Return\",\n{}  \"value\": {}\n{}}}",
                    indent_str,
                    indent_str,
                    ast_node_to_json(expr, indent + 1),
                    indent_str
                )
            } else {
                "{\"type\": \"Return\", \"value\": null}".to_string()
            }
        }
        AstNode::BinaryOp { left, op, right } => {
            format!(
                "{{\n{}  \"type\": \"BinaryOp\",\n{}  \"op\": \"{:?}\",\n{}  \"left\": {},\n{}  \"right\": {}\n{}}}",
                indent_str,
                indent_str,
                op,
                indent_str,
                ast_node_to_json(left, indent + 1),
                indent_str,
                ast_node_to_json(right, indent + 1),
                indent_str
            )
        }
        _ => format!("{{\"type\": \"{:?}\"}}", node),
    }
}

/// Convert AST node to Graphviz DOT format
fn ast_to_dot(node: &AstNode, output: &mut String, counter: &mut usize, parent: Option<usize>) {
    let current_id = *counter;
    *counter += 1;

    let label = match node {
        AstNode::FunctionDef { name, .. } => format!("FunctionDef: {}", name),
        AstNode::LetDecl { name, .. } => format!("LetDecl: {}", name),
        AstNode::IntegerLiteral(n) => format!("IntegerLiteral: {}", n),
        AstNode::Return { .. } => "Return".to_string(),
        AstNode::BinaryOp { op, .. } => format!("BinaryOp: {:?}", op),
        _ => format!("{:?}", node),
    };

    output.push_str(&format!("  node{} [label=\"{}\"];\n", current_id, label));

    if let Some(parent_id) = parent {
        output.push_str(&format!("  node{} -> node{};\n", parent_id, current_id));
    }

    // Recursively handle children
    match node {
        AstNode::FunctionDef { body, .. } => {
            for child in body {
                ast_to_dot(child, output, counter, Some(current_id));
            }
        }
        AstNode::LetDecl { value, .. } => {
            ast_to_dot(value, output, counter, Some(current_id));
        }
        AstNode::Return { value: Some(expr) } => {
            ast_to_dot(expr, output, counter, Some(current_id));
        }
        AstNode::Return { value: None } => {
            // No value to render
        }
        AstNode::BinaryOp { left, right, .. } => {
            ast_to_dot(left, output, counter, Some(current_id));
            ast_to_dot(right, output, counter, Some(current_id));
        }
        _ => {}
    }
}

/// Compute diff between two AST nodes
fn node_diff(node1: &AstNode, node2: &AstNode) -> String {
    // Detect type changes and value changes (recursively)
    match (node1, node2) {
        (AstNode::IntegerLiteral(n1), AstNode::IntegerLiteral(n2)) if n1 != n2 => {
            format!("  IntegerLiteral: {} -> {}\n", n1, n2)
        }
        (
            AstNode::LetDecl {
                name: n1,
                value: v1,
            },
            AstNode::LetDecl {
                name: n2,
                value: v2,
            },
        ) => {
            let mut diff = String::new();
            if n1 != n2 {
                diff.push_str(&format!("  LetDecl name: {} -> {}\n", n1, n2));
            }
            // Recursively compare values
            let value_diff = node_diff(v1, v2);
            if !value_diff.is_empty() {
                diff.push_str(&value_diff);
            }
            diff
        }
        _ if std::mem::discriminant(node1) != std::mem::discriminant(node2) => {
            format!("  Node type changed: {:?} -> {:?}\n", node1, node2)
        }
        _ => String::new(),
    }
}

/// Infer type from AST node (simple heuristic)
fn infer_type(node: &AstNode) -> &'static str {
    match node {
        AstNode::IntegerLiteral(_) => "i64",
        AstNode::FloatLiteral(_) => "f64",
        AstNode::StringLiteral(_) => "String",
        AstNode::BooleanLiteral(_) => "bool",
        AstNode::FunctionDef { .. } => "fn",
        AstNode::LetDecl { value, .. } => infer_type(value), // Infer from value
        _ => "unknown",
    }
}
