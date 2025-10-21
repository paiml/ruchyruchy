# Parser Debugger MVP - Quick Start

**Status**: ✅ **READY TO USE** - Ship today, iterate tomorrow
**Issue**: [#1 - Add Parser Debugging Tools](https://github.com/paiml/ruchyruchy/issues/1)
**Goal**: Better parser error messages for Ruchy compiler team

---

## What You Get

**BEFORE** (Standard parser error):
```
Error: Expected RightBrace, got Semicolon
```

**AFTER** (With parser debugger):
```
╔════════════════════════════════════════════════════════════╗
║  PARSER ERROR - Enhanced Debugging                        ║
╚════════════════════════════════════════════════════════════╝

📍 Location: line 1, column 29
❌ Expected: LeftBrace
❌ Got: Semicolon

📚 Parse Context:
   Current: IfStatement (depth: 3)

💡 Suggestion:
   Add '{' before ';' - missing block opening
   Hint: You're inside IfStatement

═══════════════════════════════════════════════════════════
```

---

## Quick Start (5 minutes)

### 1. Try the Demo

```bash
cd /path/to/ruchyruchy
ruchy run bootstrap/debugger/parser_debugger_simple_mvp.ruchy
```

You'll see a working example with integration guide.

### 2. Integrate Into Your Parser

**File**: `parser_debugger_simple_mvp.ruchy` (165 LOC, pure Ruchy)

**Step 1**: Create debugger at parser start
```ruchy
let debugger = parser_debugger_new();
```

**Step 2**: Push when entering parse rule
```ruchy
fun parse_if_statement(debugger: ParserDebugger, ...) -> ... {
    let debugger2 = parser_debugger_push(debugger, "IfStatement".to_string());

    // ... your parsing logic ...

    let debugger3 = parser_debugger_pop(debugger2);
    // ...
}
```

**Step 3**: Report errors with context
```ruchy
// Instead of:
// return Err("Expected RightBrace");

// Use:
parser_debugger_error(
    debugger,
    "RightBrace".to_string(),
    current_token.to_string(),
    line,
    column
);
return Err("Expected RightBrace");
```

---

## API Reference

### `parser_debugger_new() -> ParserDebugger`
Create new debugger instance.

### `parser_debugger_push(debugger, context) -> ParserDebugger`
Enter a parse rule. Returns new debugger state.
- `context`: e.g., "FunctionDef", "Block", "IfStatement"

### `parser_debugger_pop(debugger) -> ParserDebugger`
Exit a parse rule. Returns new debugger state.

### `parser_debugger_error(debugger, expected, got, line, column)`
Show enhanced error with context.
- `expected`: Token type expected (e.g., "RightBrace")
- `got`: Token actually found (e.g., "Semicolon")
- `line`, `column`: Source location

---

## Features

✅ **Zero Dependencies** - Pure Ruchy, no external libs
✅ **Functional State** - Works within Ruchy's limitations
✅ **Smart Suggestions** - Context-aware error hints
✅ **165 LOC** - Simple, readable, hackable
✅ **Ready Today** - No EXTREME TDD ceremony (for now)

---

## Iteration Plan

This is **MVP v1** - ship fast, improve together:

**Now** (Week 1):
- ✅ Basic context tracking (depth + current rule)
- ✅ Smart suggestions for common errors
- ✅ Integration guide

**Soon** (Week 2-3):
- Full parse stack (Vec-based, when Ruchy supports it better)
- Source code snippets in errors
- Color-coded output
- AST visualization

**Later** (Week 4+):
- DAP protocol integration (VS Code debugging)
- Time-travel debugging
- Breakpoints in parser

---

## Questions?

File issues at: https://github.com/paiml/ruchyruchy/issues/1

**Let's iterate together!** 🚀

---

## Files

- `parser_debugger_simple_mvp.ruchy` - **USE THIS** (simplified, works today)
- `parser_debugger_mvp.ruchy` - Full version (has Vec indexing issues, needs Ruchy fixes)
- `README.md` - This file

---

**Built with**: TDD-lite, dogfooding Ruchy, solving real problems
**Shipping**: TODAY ✅
