## Bug Report: Box<Enum> Recursive Structure + Match Statement Parser Error

**Ruchy Version**: ruchy 3.96.0
**Project**: RuchyRuchy Bootstrap Compiler
**Ticket**: BOOTSTRAP-010 (Type Environment - Stage 2 Type Checker)

### Reproduction Steps

1. Create file with recursive enum using Box<T>
2. Define function with match statement on the enum
3. Try to compile with `ruchy check`

### Minimal Reproduction Code

```ruchy
// This code triggers parser error at end of file
enum TypeEnv {
    Empty,
    Extend(String, Box<TypeEnv>)
}

enum Option {
    None,
    Some(i32)
}

fun lookup(env: TypeEnv, name: String) -> Option {
    match env {
        TypeEnv::Empty => Option::None,
        TypeEnv::Extend(var, rest) => {
            if var == name {
                Option::Some(42)
            } else {
                Option::None
            }
        }
    }
}

fun main() {
    let env = TypeEnv::Empty;
    println("Test");
}

main();
```

### Expected Behavior

File should compile successfully. The enum with Box<T>, match statement, and function definitions are all valid Ruchy syntax based on:
- BOOTSTRAP-006: Box<T> in enums works (v3.96.0)
- BOOTSTRAP-007: Match statements work
- Combining these should work

### Actual Behavior

**Error**:
```
✗ file.ruchy:27: Syntax error: Expected RightBrace, found Match
Error: file.ruchy:27: Syntax error: Expected RightBrace, found Match
```

Line 27 is `main();` (the call to main at end of file).

The error message is misleading - it suggests a brace mismatch, but:
- All braces are balanced (verified with grep)
- The error points to the wrong line (points to `main();` instead of actual issue)

### Full Error Output

```bash
$ ruchy check bootstrap/stage2/type_env_simple.ruchy
✗ bootstrap/stage2/type_env_simple.ruchy:96: Syntax error: Expected RightBrace, found Match
Error: bootstrap/stage2/type_env_simple.ruchy:96: Syntax error: Expected RightBrace, found Match
```

### Context

Working on BOOTSTRAP-010: Type Environment for Hindley-Milner type inference. Need recursive environment structure:

```ruchy
enum TypeEnv {
    Empty,
    Extend(String, Scheme, Box<TypeEnv>)
}
```

This is essential for type checking - can't proceed without recursive environment.

### Impact

**Blocks**: BOOTSTRAP-010, BOOTSTRAP-011, BOOTSTRAP-012 (entire Stage 2 Type Checker)

**Severity**: High - Type checker implementation depends on this

### Workaround

None currently viable. Cannot implement proper type environment without:
1. Recursive data structure (Box<TypeEnv>)
2. Pattern matching on the structure
3. Functions that use both

### Working Examples

**This works** (simple Box<T> enum):
```ruchy
enum Type {
    TInt,
    TFun(Box<Type>, Box<Type>)
}

fun main() {
    let t = Type::TInt;
    println("OK");
}

main();
```

**This works** (match on non-recursive enum):
```ruchy
enum MyOption {
    None,
    Some(i32)
}

fun test() -> bool {
    let opt = MyOption::Some(42);
    match opt {
        MyOption::None => false,
        MyOption::Some(_) => true
    }
}

fun main() {
    test();
}

main();
```

**This FAILS** (Box<Enum> + match together):
The combination of recursive Box<Enum> + match statement triggers parser error.

### Environment

- OS: Linux 6.8.0-85-generic
- Ruchy install: ruchy 3.96.0
- File encoding: UTF-8

### Additional Notes

This worked perfectly in BOOTSTRAP-006/007 for AST types:

```ruchy
enum Expr {
    Binary(BinOp, Box<Expr>, Box<Expr>)  // Recursive Box<Expr>
}

// And we could match on it...
```

So Box<T> recursion itself works. The issue seems specific to combining:
- Box<Enum> where Enum references itself
- Match statement on that enum
- In a function context

### Files

- Failing file: `bootstrap/stage2/type_env_simple.ruchy`
- RED phase test (works): `bootstrap/stage2/test_type_environment.ruchy`
- Working Box<T> example: `bootstrap/stage1/ast_types_recursive.ruchy`

### Related Issues

- [Box<T> and Vec<T> Support](https://github.com/paiml/ruchy/issues/XXX) - Fixed in v3.96.0
- This appears to be a different issue with parsing Box<T> in specific contexts
