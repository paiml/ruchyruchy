# ruchyruchy-repo-spec.md

## RuchyRuchy: Self-Hosting Compiler Bootstrap

**Objective**: Empirical validation of Ruchy's self-compilation capability through incremental bootstrap stages.

## Core Architecture

### Design Principle: Progressive Bootstrap

The compiler implements a four-stage bootstrap sequence, each stage compiling the next with increasing feature coverage:

```
Stage 0 (Lexer)     → 1K LOC  → Tokenizes itself
Stage 1 (Parser)    → 3K LOC  → Parses Stage 0+1
Stage 2 (TypeCheck) → 5K LOC  → Types Stage 0+1+2
Stage 3 (CodeGen)   → 6K LOC  → Compiles all stages
```

### Repository Structure

```
ruchyruchy/
├── bootstrap/
│   ├── stage0/
│   │   ├── lexer.ruchy         # Minimal tokenizer
│   │   └── token.ruchy         # Token definitions
│   ├── stage1/
│   │   ├── parser.ruchy        # Pratt + recursive descent
│   │   └── ast.ruchy           # AST representation
│   ├── stage2/
│   │   ├── infer.ruchy         # Algorithm W
│   │   └── unify.ruchy         # Constraint solver
│   └── stage3/
│       ├── emit.ruchy          # Rust code generation
│       └── opt.ruchy           # Peephole optimizer
├── validation/
│   ├── differential.ruchy      # Output equivalence
│   ├── property.ruchy          # QuickCheck properties
│   └── bench.ruchy             # Performance regression
├── INTEGRATION.md              # Test matrix results
└── Makefile                    # Quality gates
```

## Stage 0: Lexical Analysis (1K LOC)

### Minimal Token Set

```ruchy
// bootstrap/stage0/token.ruchy
pub enum Token {
    // Keywords (12 essential)
    Let, Fun, If, Else, Match, Return,
    Struct, Enum, Impl, Pub, Mod, Use,
    
    // Literals
    Int(i64), Float(f64), Str(String), Bool(bool),
    
    // Identifiers
    Ident(String),
    
    // Operators (arithmetic + comparison)
    Plus, Minus, Star, Slash, Percent,
    Eq, Ne, Lt, Le, Gt, Ge,
    
    // Delimiters
    LParen, RParen, LBrace, RBrace,
    Comma, Semi, Arrow, Dot,
    
    Eof,
}
```

### Scanner Implementation

```ruchy
// bootstrap/stage0/lexer.ruchy
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    current: char,
}

impl Lexer {
    pub fun scan(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            self.skip_whitespace();
            
            match self.current {
                '0'..='9' => tokens.push(self.number()),
                'a'..='z' | 'A'..='Z' | '_' => {
                    tokens.push(self.ident_or_keyword())
                }
                '"' => tokens.push(self.string()),
                '+' => tokens.push(self.single(Token::Plus)),
                '-' => tokens.push(self.single(Token::Minus)),
                '(' => tokens.push(self.single(Token::LParen)),
                ')' => tokens.push(self.single(Token::RParen)),
                _ if self.is_at_end() => break,
                _ => self.advance(), // Skip unknown
            }
        }
        
        tokens.push(Token::Eof);
        tokens
    }
    
    fun number(&mut self) -> Token {
        let start = self.pos;
        while self.current.is_ascii_digit() {
            self.advance();
        }
        
        if self.current == '.' && self.peek().is_ascii_digit() {
            self.advance(); // consume '.'
            while self.current.is_ascii_digit() {
                self.advance();
            }
            let text = self.input[start..self.pos].iter().collect();
            Token::Float(text.parse().unwrap())
        } else {
            let text = self.input[start..self.pos].iter().collect();
            Token::Int(text.parse().unwrap())
        }
    }
}
```

**Validation**: `./lexer < lexer.ruchy` produces 500+ tokens in <10ms

## Stage 1: Syntax Analysis (3K LOC)

### Parser Architecture

```ruchy
// bootstrap/stage1/parser.ruchy
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fun parse(&mut self) -> Result<Module, ParseError> {
        let mut items = Vec::new();
        
        while !self.is_at_end() {
            items.push(self.declaration()?);
        }
        
        Ok(Module { items })
    }
    
    fun declaration(&mut self) -> Result<Item, ParseError> {
        match self.peek() {
            Token::Fun => self.function(),
            Token::Struct => self.structure(),
            Token::Let => self.global(),
            _ => Err(ParseError::ExpectedDeclaration),
        }
    }
    
    fun function(&mut self) -> Result<Item, ParseError> {
        self.consume(Token::Fun)?;
        let name = self.identifier()?;
        self.consume(Token::LParen)?;
        let params = self.parameters()?;
        self.consume(Token::RParen)?;
        let ret_type = self.type_annotation()?;
        let body = self.block()?;
        
        Ok(Item::Function(Function {
            name, params, ret_type, body
        }))
    }
    
    // Pratt parser for expressions
    fun expression(&mut self, min_prec: i32) -> Result<Expr, ParseError> {
        let mut left = self.primary()?;
        
        while let Some(op) = self.peek_operator() {
            let prec = self.precedence(op);
            if prec < min_prec { break; }
            
            self.advance();
            let right = self.expression(prec + 1)?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        
        Ok(left)
    }
}
```

**Validation**: Roundtrip property `parse(ast.emit()) == ast`

## Stage 2: Type Inference (5K LOC)

### Algorithm W Implementation

```ruchy
// bootstrap/stage2/infer.ruchy
pub enum Type {
    Var(u32),
    Con(String),
    App(Box<Type>, Box<Type>),
}

pub struct Inferencer {
    env: HashMap<String, Scheme>,
    subst: HashMap<u32, Type>,
    fresh: u32,
}

impl Inferencer {
    pub fun infer(&mut self, expr: &Expr) -> Result<Type, TypeError> {
        match expr {
            Expr::Var(x) => {
                let scheme = self.env.get(x)
                    .ok_or(TypeError::Unbound(x.clone()))?;
                Ok(self.instantiate(scheme))
            }
            
            Expr::App(f, arg) => {
                let tf = self.infer(f)?;
                let ta = self.infer(arg)?;
                let tv = self.fresh_var();
                self.unify(tf, Type::App(
                    Box::new(Type::App(
                        Box::new(Type::Con("->".into())),
                        Box::new(ta)
                    )),
                    Box::new(tv.clone())
                ))?;
                Ok(tv)
            }
            
            Expr::Lam(x, body) => {
                let tx = self.fresh_var();
                self.env.insert(x.clone(), Scheme::mono(tx.clone()));
                let tb = self.infer(body)?;
                self.env.remove(x);
                Ok(Type::App(
                    Box::new(Type::App(
                        Box::new(Type::Con("->".into())),
                        Box::new(tx)
                    )),
                    Box::new(tb)
                ))
            }
            
            Expr::Let(x, e, body) => {
                let te = self.infer(e)?;
                let scheme = self.generalize(te);
                self.env.insert(x.clone(), scheme);
                let result = self.infer(body)?;
                self.env.remove(x);
                Ok(result)
            }
        }
    }
    
    fun unify(&mut self, t1: Type, t2: Type) -> Result<(), TypeError> {
        let t1 = self.apply_subst(&t1);
        let t2 = self.apply_subst(&t2);
        
        match (t1, t2) {
            (Type::Var(v1), Type::Var(v2)) if v1 == v2 => Ok(()),
            (Type::Var(v), t) | (t, Type::Var(v)) => {
                if self.occurs(v, &t) {
                    Err(TypeError::Infinite)
                } else {
                    self.subst.insert(v, t);
                    Ok(())
                }
            }
            (Type::Con(c1), Type::Con(c2)) if c1 == c2 => Ok(()),
            (Type::App(f1, a1), Type::App(f2, a2)) => {
                self.unify(*f1, *f2)?;
                self.unify(*a1, *a2)
            }
            _ => Err(TypeError::Mismatch),
        }
    }
}
```

**Validation**: Type-checks entire bootstrap codebase in O(n log n)

## Stage 3: Code Generation (6K LOC)

### Rust Emission

```ruchy
// bootstrap/stage3/emit.ruchy
pub struct Emitter {
    buffer: String,
    indent: usize,
}

impl Emitter {
    pub fun emit_module(&mut self, module: &TypedModule) -> String {
        for item in &module.items {
            self.emit_item(item);
            self.newline();
        }
        self.buffer.clone()
    }
    
    fun emit_item(&mut self, item: &TypedItem) {
        match item {
            TypedItem::Function(f) => {
                self.emit_function(f);
            }
            TypedItem::Struct(s) => {
                self.emit_struct(s);
            }
        }
    }
    
    fun emit_function(&mut self, func: &TypedFunction) {
        if func.is_public {
            self.write("pub ");
        }
        self.write("fn ");
        self.write(&func.name);
        self.write("(");
        
        for (i, param) in func.params.iter().enumerate() {
            if i > 0 { self.write(", "); }
            self.write(&param.name);
            self.write(": ");
            self.emit_type(&param.ty);
        }
        
        self.write(") -> ");
        self.emit_type(&func.ret_type);
        self.write(" {");
        self.indent += 1;
        self.newline();
        
        self.emit_expr(&func.body);
        
        self.indent -= 1;
        self.newline();
        self.write("}");
    }
    
    fun emit_expr(&mut self, expr: &TypedExpr) {
        match expr {
            TypedExpr::Var(name) => self.write(name),
            TypedExpr::Literal(lit) => self.emit_literal(lit),
            TypedExpr::Binary(left, op, right) => {
                self.write("(");
                self.emit_expr(left);
                self.write(" ");
                self.write(op.as_str());
                self.write(" ");
                self.emit_expr(right);
                self.write(")");
            }
            TypedExpr::Call(func, args) => {
                self.emit_expr(func);
                self.write("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 { self.write(", "); }
                    self.emit_expr(arg);
                }
                self.write(")");
            }
        }
    }
}
```

**Validation**: Generated Rust compiles with zero warnings

## Quality Gates

### Performance Requirements

| Stage | Metric | Target | Measurement |
|-------|--------|--------|-------------|
| Lexer | Throughput | >10K LOC/s | `hyperfine ./lexer < large.ruchy` |
| Parser | Throughput | >5K LOC/s | `hyperfine ./parser < large.ruchy` |
| TypeCheck | Complexity | O(n log n) | Empirical scaling test |
| CodeGen | Throughput | >10K LOC/s | `hyperfine ./codegen < typed.json` |

### Correctness Validation

```ruchy
// validation/differential.ruchy
pub fun validate_equivalence(source: &str) -> Result<(), DiffError> {
    // Compile with production Ruchy
    let expected = Command::new("ruchy")
        .arg("compile")
        .arg("-")
        .stdin(source)
        .output()?;
    
    // Compile with bootstrap compiler
    let actual = Command::new("./ruchyruchy")
        .arg("compile")
        .arg("-")
        .stdin(source)
        .output()?;
    
    // Compare generated Rust
    if expected.stdout != actual.stdout {
        return Err(DiffError::OutputMismatch);
    }
    
    // Both should compile with rustc
    let rust_check = Command::new("rustc")
        .arg("--emit=metadata")
        .stdin(actual.stdout)
        .status()?;
    
    if !rust_check.success() {
        return Err(DiffError::InvalidRust);
    }
    
    Ok(())
}
```

## INTEGRATION.md Template

```markdown
# RuchyRuchy Integration Status

**Date**: 2024-12-16
**Ruchy Version**: 1.8.6
**Bootstrap Progress**: Stage 0/3

## Build Matrix

| Stage | LOC | Compiles | Self-Compiles | Tests | Performance |
|-------|-----|----------|---------------|-------|-------------|
| Stage 0 | 987 | ✅ | ✅ | 48/50 | 12.3K LOC/s |
| Stage 1 | 2834 | ✅ | ❌ | 72/100 | - |
| Stage 2 | 4921 | ⚠️ | ❌ | 0/200 | - |
| Stage 3 | 5782 | ❌ | ❌ | 0/300 | - |

## Differential Testing

| Test Case | Production | Bootstrap | Match |
|-----------|------------|-----------|-------|
| hello.ruchy | `fn main() { println!("Hello"); }` | `fn main() { println!("Hello"); }` | ✅ |
| fib.ruchy | [2.3KB output] | [2.3KB output] | ✅ |
| types.ruchy | [8.7KB output] | [8.7KB output] | ✅ |

## Performance Regression

| Benchmark | Baseline | Current | Delta |
|-----------|----------|---------|-------|
| Lexer (10K LOC) | 0.82s | 0.81s | -1.2% |
| Parser (10K LOC) | 1.94s | - | - |
| Full Pipeline | 4.31s | - | - |
```

## Roadmap

### Phase 1: Lexer Bootstrap (2 weeks)
- [x] Token definitions
- [x] Scanner core
- [x] Keyword recognition
- [ ] Self-tokenization test
- [ ] Performance optimization

### Phase 2: Parser Bootstrap (3 weeks)
- [ ] AST definitions
- [ ] Recursive descent framework
- [ ] Pratt expression parser
- [ ] Self-parsing validation
- [ ] Roundtrip testing

### Phase 3: Type System (4 weeks)
- [ ] Type representation
- [ ] Algorithm W core
- [ ] Constraint solving
- [ ] Generalization
- [ ] Self-type-checking

### Phase 4: Code Generation (3 weeks)
- [ ] Rust AST mapping
- [ ] Expression emission
- [ ] Pattern compilation
- [ ] Self-compilation test
- [ ] Fixpoint validation

## Makefile

```makefile
# Toyota Way: Stop the line on quality issues
.DEFAULT_GOAL := validate
.PHONY: stage0 stage1 stage2 stage3 validate bench clean

# Progressive bootstrap
stage0:
	@echo "Building Stage 0 (Lexer)..."
	ruchy compile bootstrap/stage0/lexer.ruchy -o build/lexer0
	./build/lexer0 < bootstrap/stage0/lexer.ruchy > build/tokens.txt
	@echo "✓ Self-tokenization: $$(wc -l < build/tokens.txt) tokens"

stage1: stage0
	@echo "Building Stage 1 (Parser)..."
	ruchy compile bootstrap/stage1/parser.ruchy -o build/parser1
	./build/parser1 < bootstrap/stage1/parser.ruchy > build/ast.json
	@echo "✓ Self-parsing: $$(jq '.items | length' build/ast.json) items"

stage2: stage1
	@echo "Building Stage 2 (Type Checker)..."
	ruchy compile bootstrap/stage2/infer.ruchy -o build/typeck2
	./build/typeck2 < build/ast.json > build/typed.json
	@echo "✓ Self-type-checking complete"

stage3: stage2
	@echo "Building Stage 3 (Code Generator)..."
	ruchy compile bootstrap/stage3/emit.ruchy -o build/codegen3
	./build/codegen3 < build/typed.json > build/output.rs
	rustc --emit=metadata build/output.rs
	@echo "✓ Self-compilation complete"

# Quality gates
validate:
	@ruchy complexity bootstrap/**/*.ruchy --max 20
	@ruchy lint bootstrap/**/*.ruchy
	@cargo test --workspace

# Performance validation
bench:
	hyperfine --warmup 3 --export-json bench.json \
		'./build/lexer0 < testdata/10k.ruchy' \
		'ruchy compile --emit=tokens testdata/10k.ruchy'

# Integration report
integration:
	@date '+Date: %Y-%m-%d' > INTEGRATION.md
	@echo "Ruchy Version: $$(ruchy --version)" >> INTEGRATION.md
	@./scripts/test-matrix.sh >> INTEGRATION.md

clean:
	rm -rf build/
```

## Success Metrics

1. **Correctness**: Bit-identical Rust output for test suite
2. **Performance**: <5% overhead vs production compiler
3. **Complexity**: All functions <20 cyclomatic complexity
4. **Memory**: Peak RSS <100MB for 10K LOC input
5. **Binary Size**: Stripped binary <5MB per stage

## Getting Started

```bash
git clone https://github.com/pragmatic-ai-labs/ruchyruchy
cd ruchyruchy
make stage0  # Begin bootstrap
make validate  # Run quality gates
make integration  # Generate report
```

---

*Self-hosting: The ultimate compiler validation*