## Feature Request: Box<T> and Vec<T> Runtime Support for Recursive Data Structures

**Ruchy Version**: ruchy 3.95.0
**Project**: RuchyRuchy Bootstrap Compiler
**Tickets Blocked**: BOOTSTRAP-007, BOOTSTRAP-008, BOOTSTRAP-009 (Stage 1: Parser Implementation)

### Problem Statement

Ruchy's parser and runtime currently reject enum variants with `Box<T>` and `Vec<T>` parameters, preventing the implementation of recursive data structures essential for compiler construction.

**Error**: `Syntax error: Expected variant name in enum`

This blocks the bootstrap compiler's parser implementation (Stage 1), which fundamentally requires:
1. Recursive Abstract Syntax Trees (AST) for expressions
2. List structures for statement blocks and function parameters
3. Optional boxed values for control flow

### Minimal Reproduction

#### Example 1: Box<T> for Recursive Structures

```ruchy
// Minimal recursive tree structure
enum Tree {
    Leaf(i32),
    Node(Box<Tree>, Box<Tree>)  // ❌ Syntax error
}

fn make_tree() -> Tree {
    let left = Tree::Leaf(1);
    let right = Tree::Leaf(2);
    Tree::Node(Box::new(left), Box::new(right))
}
```

**Current Behavior**: Parser rejects with "Syntax error: Expected variant name in enum"

**Expected Behavior**: Tree structure compiles and executes successfully

#### Example 2: Vec<T> for Collections

```ruchy
// Statement block with multiple statements
enum Stmt {
    ExprStmt(String),
    Block(Vec<Stmt>)  // ❌ Syntax error
}

fn make_block() -> Stmt {
    let stmts = vec![
        Stmt::ExprStmt("hello".to_string()),
        Stmt::ExprStmt("world".to_string())
    ];
    Stmt::Block(stmts)
}
```

**Current Behavior**: Parser rejects Vec<T> in enum variant

**Expected Behavior**: Block with statement list compiles and executes

### Impact on Bootstrap Compiler

The bootstrap compiler cannot proceed beyond Stage 0 (Lexer) without this feature.

#### Stage 1 Requirements (BLOCKED)

**BOOTSTRAP-007: Pratt Parser for Expressions**

Requires recursive expression AST:
```ruchy
enum Expr {
    Number(String),
    Identifier(String),

    // Binary operations require Box for recursion
    Binary(BinOp, Box<Expr>, Box<Expr>),  // ❌ BLOCKED

    // Unary operations
    Unary(UnOp, Box<Expr>),                // ❌ BLOCKED

    // Function calls with argument list
    Call(Box<Expr>, Vec<Expr>)             // ❌ BLOCKED (both Box and Vec)
}
```

Without this, cannot parse expressions like:
- `1 + 2 * 3` (binary operations)
- `-x` (unary operations)
- `foo(1, 2, 3)` (function calls)

**BOOTSTRAP-008: Recursive Descent for Statements**

Requires statement blocks and optional branches:
```ruchy
enum Stmt {
    // Block of multiple statements
    Block(Vec<Stmt>),                      // ❌ BLOCKED

    // If with optional else
    If(Expr, Box<Stmt>, Option<Box<Stmt>>) // ❌ BLOCKED

    // While loop
    While(Expr, Box<Stmt>)                 // ❌ BLOCKED
}
```

**BOOTSTRAP-009: Parser Self-Parsing Test**

Cannot validate parser correctness without full AST implementation.

### What Currently Works

**Enum Capabilities (v3.95.0)**:
- ✅ Enum with String parameters: `Expr::Number(String)`
- ✅ Enum unit variants: `Expr::BoolTrue`
- ✅ Enum with multiple String params: `Position::Pos(i32, i32, i32)`
- ✅ Pattern matching on enums
- ✅ Enum construction and destructuring

**What's Missing**:
- ❌ Box<T> for recursive structures
- ❌ Vec<T> for collections
- ❌ Option<Box<T>> for optional boxed values

### Workaround Applied (Temporary)

For BOOTSTRAP-006/007, we implemented a **simplified AST** demonstrating concepts:

```ruchy
// Simplified (works but limited)
enum Expr {
    Number(String),
    Identifier(String),
    BoolTrue,
    BoolFalse
}
```

This allows:
- ✅ Demonstrating operator precedence theory
- ✅ Parsing primary expressions (numbers, identifiers)
- ✅ Showing binding power concepts

But **cannot**:
- ❌ Build actual expression trees
- ❌ Parse `1 + 2 * 3` into proper AST
- ❌ Implement full Pratt parser recursion
- ❌ Handle nested expressions

### Use Cases Beyond Compilers

Recursive data structures are fundamental to many domains:

1. **Compilers/Interpreters**: AST, symbol tables, type inference trees
2. **Data Structures**: Binary trees, linked lists, graphs
3. **JSON/XML Parsing**: Nested object structures
4. **Configuration**: Hierarchical settings
5. **Game Development**: Scene graphs, entity hierarchies

### Technical Requirements

**Box<T> Support**:
```ruchy
// Heap allocation for recursive types
let boxed = Box::new(value);
let unboxed = *boxed;

// In enum variants
enum Node {
    Leaf(i32),
    Branch(Box<Node>, Box<Node>)
}
```

**Vec<T> Support**:
```ruchy
// Dynamic arrays
let items = vec![1, 2, 3];
items.push(4);
let first = items[0];

// In enum variants
enum Container {
    Single(i32),
    Multiple(Vec<i32>)
}
```

**Option<Box<T>> Support**:
```ruchy
// Optional boxed values
enum Node {
    Leaf,
    Branch(Box<Node>, Option<Box<Node>>)  // Optional right child
}
```

### Environment

- OS: Linux (6.8.0-85-generic)
- Ruchy version: ruchy 3.95.0
- Working directory: /home/noah/src/ruchyruchy
- Project: Bootstrap Compiler (Stage 1 blocked)

### Files Affected

**Blocked Implementation**:
- `bootstrap/stage1/ast_types.ruchy` (simplified AST only)
- `bootstrap/stage1/expr_parser_simple.ruchy` (conceptual demonstration)
- BOOTSTRAP-007, BOOTSTRAP-008, BOOTSTRAP-009 (cannot proceed)

**Documentation**:
- `BOUNDARIES.md` (limitation documented)
- `INTEGRATION.md` (impact described)
- `roadmap.yaml` (tickets marked blocked)

### Request

Please implement `Box<T>` and `Vec<T>` runtime support to enable:

1. **Parser Development** (BOOTSTRAP-007, 008, 009)
2. **Recursive Data Structures** (fundamental requirement)
3. **Collection Types** (essential for real programs)
4. **Full Bootstrap Compiler** (Stage 1 and beyond)

This is a **critical blocker** for advancing the bootstrap compiler beyond the lexer stage.

### Success Criteria

When implemented, the following should work:

```ruchy
// Test 1: Recursive tree
enum Tree {
    Leaf(i32),
    Node(Box<Tree>, Box<Tree>)
}

fn test_box() {
    let t = Tree::Node(
        Box::new(Tree::Leaf(1)),
        Box::new(Tree::Leaf(2))
    );
    // Pattern matching
    match t {
        Tree::Node(left, right) => println("Branch"),
        Tree::Leaf(n) => println("Leaf")
    }
}

// Test 2: Vec in enum
enum Block {
    Single(String),
    Multiple(Vec<String>)
}

fn test_vec() {
    let items = vec!["a".to_string(), "b".to_string()];
    let block = Block::Multiple(items);
}

// Test 3: Full expression AST
enum Expr {
    Number(i32),
    Binary(BinOp, Box<Expr>, Box<Expr>)
}

fn test_expr() {
    // Parse: 1 + 2
    let expr = Expr::Binary(
        BinOp::Add,
        Box::new(Expr::Number(1)),
        Box::new(Expr::Number(2))
    );
}
```

**Priority**: CRITICAL - Blocks fundamental compiler development patterns

**Impact**: HIGH - Enables entire category of recursive programs

Thank you for the amazing work on Ruchy v3.93.0, v3.94.0, and v3.95.0! The enum tuple variant, .nth(), and loop+mut+tuple fixes were crucial for Stage 0 completion. Box<T> and Vec<T> support would unlock Stage 1 and beyond! 🚀
