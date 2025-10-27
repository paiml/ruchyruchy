# MutFuzz - Combined Mutation + Fuzz Testing Tool for Ruchy

**Date**: 2025-10-27
**Status**: Design Complete, Implementation Blocked by Parser
**Context**: QUALITY-002 MUTATION + FUZZ phases

---

## Overview

MutFuzz is a **specialized debugging tool** for Ruchy that combines:
1. **Mutation Testing**: Systematic code mutations to test suite quality
2. **Fuzz Testing**: Random/grammar-based inputs to find crashes

Inspired by patterns from `paiml-mcp-agent-toolkit` mutation testing engine.

---

## Architecture

### Core Components

```
MutFuzz Tool
├── Mutation Engine
│   ├── Mutation Operators (AOR, ROR, UOR, SDL)
│   ├── AST Traversal & Manipulation
│   ├── Test Executor (RAII file safety)
│   └── Mutation Score Calculator
└── Fuzz Engine
    ├── Grammar-Based Generator (valid Ruchy)
    ├── Mutation-Based Generator (mutated valid)
    ├── Random Generator (pure chaos)
    └── Crash/Error Detector
```

### Mutation Operators

Based on industry-standard patterns:

1. **AOR (Arithmetic Operator Replacement)**
   - `+` ↔ `-`, `*` ↔ `/`
   - Tests arithmetic logic coverage

2. **ROR (Relational Operator Replacement)**
   - `<` ↔ `<=`, `==` ↔ `!=`
   - Tests comparison/boundary logic

3. **UOR (Unary Operator Replacement)**
   - Remove `!`, remove `-`
   - Tests boolean logic

4. **SDL (Statement Deletion)**
   - Delete non-critical statements
   - Tests necessity of code

### Fuzz Strategies

1. **Grammar-Based**: Generate valid Ruchy syntax
2. **Mutation-Based**: Mutate valid inputs (typos, missing tokens)
3. **Random**: Pure chaos (stress test parser)

---

## Implementation Patterns (from paiml-mcp-agent-toolkit)

### 1. AST Manipulation

```rust
// Visitor pattern for AST traversal
struct MutationVisitor {
    mutants: Vec<Mutant>,
    operators: Vec<Box<dyn MutationOperator>>,
}

impl syn::visit::Visit for MutationVisitor {
    fn visit_expr(&mut self, expr: &Expr) {
        for operator in &self.operators {
            if operator.can_mutate(expr) {
                let mutants = operator.mutate(expr);
                self.mutants.extend(mutants);
            }
        }
        syn::visit::visit_expr(self, expr);
    }
}
```

### 2. Safe Mutation Execution (RAII Pattern)

```rust
pub async fn execute_mutant(&self, mutant: &Mutant) -> Result<MutationResult> {
    // Backup original file (RAII ensures restoration)
    let guard = MutantGuard::new(&mutant.file_path).await?;

    // Write mutated source
    fs::write(&mutant.file_path, &mutant.mutated_source).await?;

    // Run tests with timeout
    let test_result = timeout(
        self.timeout,
        run_test_command(&mutant)
    ).await;

    // Restore original (automatic via Drop trait)
    guard.restore().await?;

    Ok(MutationResult { status, ... })
}
```

### 3. Mutation Score Calculation

```
mutation_score = killed / (total - compile_errors - equivalent)

Where:
- killed = mutations caught by tests
- total = all mutations generated
- compile_errors = mutations that don't compile (excluded)
- equivalent = mutations semantically identical to original (excluded)
```

### 4. Quality Thresholds

- **Excellent**: ≥85% mutation score
- **Good**: ≥70% mutation score
- **Fair**: ≥50% mutation score
- **Poor**: <50% mutation score

---

## Ruchy Implementation (Designed)

### File Structure

```
bootstrap/stage3/mutfuzz/
├── engine.ruchy          # Core mutation engine
├── operators.ruchy       # Mutation operators (AOR, ROR, UOR, SDL)
├── executor.ruchy        # Safe test execution with RAII
├── fuzzer.ruchy          # Fuzz input generation
└── scorer.ruchy          # Mutation score calculation

validation/quality/
├── mutfuzz_test.ruchy    # Test the MutFuzz tool itself
└── dead_code_mutfuzz.ruchy  # Apply MutFuzz to dead code detector
```

### Usage Example

```ruchy
fun main() {
    // Run combined mutation + fuzz testing
    let mutfuzz = MutFuzz::new()
    let results = mutfuzz.test_file(
        "validation/quality/dead_code_simple_test.ruchy",
        "ruchy run validation/quality/dead_code_simple_test.ruchy"
    )

    println("Mutation Score: " + results.mutation_score.to_string() + "%")
    println("Fuzz Crashes: " + results.fuzz_crashes.to_string())

    if results.mutation_score >= 85.0 && results.fuzz_crashes == 0 {
        println("✅ PASS - High quality code")
    }
}
```

---

## Implementation Status

### ✅ Complete

1. **Research**: Analyzed paiml-mcp-agent-toolkit patterns
2. **Architecture**: Designed MutFuzz tool structure
3. **Patterns**: Identified AST manipulation, RAII safety, scoring
4. **Concept**: Created working demonstration (partial)

### ❌ Blocked

**Ruchy Parser Bug**: Files >300 LOC trigger parser errors

**Evidence**:
- 336 LOC file: `✗ Syntax error: Expected RightBrace, found Let`
- 340 LOC file: Same error
- Braces perfectly balanced, syntax valid
- **GitHub Issue #65**: Updated with findings

**Impact**: Cannot implement full MutFuzz tool until parser fixed

---

## Workaround: Split Implementation

To work around parser limitations, split into multiple <200 LOC files:

```
mutfuzz/
├── types.ruchy (50 LOC)      # Struct definitions
├── operators.ruchy (150 LOC) # Mutation operators
├── fuzzer.ruchy (150 LOC)    # Fuzz generators
├── executor.ruchy (150 LOC)  # Test execution
└── main.ruchy (100 LOC)      # Entry point
```

Total: 600 LOC across 5 files (vs 1 file hitting parser bug)

---

## Expected Results

### Mutation Testing

For `dead_code_simple_test.ruchy`:

| Mutation | Type | Expected Result |
|----------|------|----------------|
| `+` → `-` | AOR | Killed (counter logic tested) |
| `*` → `/` | AOR | Killed (percentage calc tested) |
| `>` → `>=` | ROR | Killed (boundary tested) |
| `&&` → `||` | ROR | Killed (range tested) |
| Remove `!` | UOR | Killed (boolean logic tested) |
| Delete `println` | SDL | **Survived** (output not tested) |

**Expected Score**: ~85% (6/7 mutations killed)

### Fuzz Testing

| Strategy | Input Example | Expected Result |
|----------|--------------|----------------|
| Grammar | `fun test() { return 0 }` | Pass (valid syntax) |
| Mutation | `fun test() { returrrn 0 }` | Error (gracefully handled) |
| Random | `{{{{{{}}}}}}` | Error (no crash) |
| Edge | Empty string | Pass (handled) |

**Expected**: 0 crashes, graceful error handling

---

## Integration with QUALITY-002

MutFuzz tool provides **Phase 5 (MUTATION)** and **Phase 7 (FUZZ)** of EXTREME TDD:

```
QUALITY-002 Phases:
1. RED ✅
2. GREEN ✅
3. REFACTOR ✅
4. TOOL ✅
5. MUTATION ⏳ ← MutFuzz provides this
6. PROPERTY ⏳
7. FUZZ ⏳ ← MutFuzz provides this
8. PMAT ⏳
```

---

## Next Steps

1. **Wait for Parser Fix**: Monitor GitHub issue #65
2. **Implement Split Architecture**: Create 5 files <200 LOC each
3. **Test on Real Code**: Apply to bootstrap compiler stages
4. **ML Enhancement** (optional): Add prediction layer from paiml-mcp-agent-toolkit
5. **Property Testing**: Add property-based testing (Phase 6)
6. **PMAT**: Add performance metrics (Phase 8)

---

## Value Proposition

### Why MutFuzz is Valuable

1. **Ruchy-Specific**: Leverages compiler insights (AST, types, semantics)
2. **Combined Approach**: Mutation + Fuzz in one tool (no other tool does this)
3. **Quality Assessment**: Objective test suite quality measurement
4. **Crash Detection**: Find parser/runtime bugs via fuzzing
5. **Self-Hosting**: Ruchy testing Ruchy (dogfooding excellence)

### Comparison to Existing Tools

| Tool | Mutation | Fuzz | Language-Aware | Combined |
|------|----------|------|----------------|----------|
| cargo-mutants | ✅ | ❌ | ✅ (Rust) | ❌ |
| AFL/libFuzzer | ❌ | ✅ | ❌ | ❌ |
| **MutFuzz** | ✅ | ✅ | ✅ (Ruchy) | ✅ |

---

## Conclusion

MutFuzz is a **well-architected tool** that would provide significant value to Ruchy development. Implementation is blocked by Ruchy parser limitations at ~300+ LOC, but the design is sound and ready for implementation once parser is fixed or via split-file workaround.

**Status**: Design complete, awaiting parser fix or split implementation.

---

**Project**: RuchyRuchy Bootstrap Compiler
**Ticket**: QUALITY-002 (MUTATION + FUZZ phases)
**References**:
- paiml-mcp-agent-toolkit mutation engine
- Industry-standard mutation operators (AOR, ROR, UOR, SDL)
- EXTREME TDD methodology
