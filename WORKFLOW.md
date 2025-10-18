# RuchyRuchy Development Workflow

## Ticket-Driven Development with Extreme TDD

This project uses **strict ticket-driven development** inspired by pmat (../paiml-mcp-agent-toolkit) and enforced through pre-commit hooks.

---

## 🎫 Workflow Overview

### 1. Install Git Hooks (First Time Setup)

```bash
make install-hooks
```

This installs:
- **pre-commit**: Quality gates + SATD detection + documentation sync
- **commit-msg**: Ticket ID validation

### 2. Select a Ticket from roadmap.yaml

Open `roadmap.yaml` and find a pending ticket:

```yaml
- id: BOOTSTRAP-001
  title: "Token Type Definitions"
  priority: critical
  status: pending  # ← Look for pending tickets
```

Available ticket prefixes:
- `INFRA-XXX`: Infrastructure (hooks, CI/CD, tooling)
- `VALID-XXX`: Validation infrastructure (property, fuzz, boundary)
- `BOOTSTRAP-XXX`: Bootstrap compiler implementation
- `PROP-XXX`: Property testing specific tickets
- `FUZZ-XXX`: Fuzz testing specific tickets
- `BOUND-XXX`: Boundary analysis tickets

### 3. Write Failing Tests (RED)

**Pure Ruchy TDD** - Write tests BEFORE implementation:

```bash
# Create test file first
vim validation/tests/test_bootstrap_001.ruchy
```

Example test structure:
```ruchy
// Test for BOOTSTRAP-001: Token Type Definitions

fun test_token_type_completeness() -> bool {
    // Test all 70+ token types are defined
    true  // Will fail until implemented
}

fun test_keyword_lookup() -> bool {
    // Test keyword lookup is O(1)
    true
}

fun main() {
    run_test("Token completeness", test_token_type_completeness);
    run_test("Keyword lookup", test_keyword_lookup);
}
```

Run tests - they should FAIL:
```bash
ruchy test validation/tests/test_bootstrap_001.ruchy
# ❌ Tests should fail
```

### 4. Minimal Implementation (GREEN)

Implement **just enough** to pass the tests:

```bash
# Implement the feature
vim bootstrap/stage0/token.ruchy
```

Run tests until GREEN:
```bash
ruchy test validation/tests/test_bootstrap_001.ruchy
# ✅ Tests should pass
```

### 5. Refactor with Quality (REFACTOR)

Improve code while maintaining green tests:

```bash
# Refactor for clarity
ruchy fmt bootstrap/stage0/token.ruchy
ruchy lint bootstrap/stage0/token.ruchy  # Must achieve A+ grade
ruchy score bootstrap/stage0/token.ruchy  # Must be >0.8
```

### 6. Run Dogfooding Suite

```bash
make dogfood-check   # Syntax validation
make dogfood-lint    # Lint check
make dogfood-quality # Quality tools
```

### 7. Update Documentation

**MANDATORY** - Pre-commit hooks will block without this:

```bash
# Update INTEGRATION.md with status
vim INTEGRATION.md

# Mark ticket complete in roadmap.yaml
vim roadmap.yaml
# Change: status: in_progress → status: completed

# Add changelog entry
vim CHANGELOG.md
```

### 8. Commit with Ticket ID

**CRITICAL**: Commit message MUST start with ticket ID:

```bash
git add -A
git commit -m "BOOTSTRAP-001: Implement token type definitions

Component: Stage 0 Lexer
Tests: 5 property tests via ruchy test
Coverage: 100% (all code paths tested)
Quality: A+ lint grade, score 0.95

Implements complete TokenType enum with 70+ variants.
Includes keyword lookup function with O(1) performance.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

### 9. Pre-commit Quality Gates

Hooks will automatically check:
1. ✅ Ticket ID format (BOOTSTRAP-001, VALID-003, etc.)
2. ✅ Zero SATD (no TODO/FIXME/HACK comments)
3. ✅ Documentation sync (roadmap.yaml or INTEGRATION.md updated)
4. ✅ Ruchy syntax (`ruchy check`)
5. ✅ Ruchy lint (A+ grade required)
6. ✅ PMAT TDG score (≥85)
7. ✅ Roadmap structure valid
8. ✅ File size limits

If any gate fails, commit is **BLOCKED**.

### 10. Push to GitHub

```bash
git push origin main
```

---

## 📋 Ticket Lifecycle

### Ticket States

```yaml
status: pending      # Not started
status: in_progress  # Currently working on
status: completed    # Done and merged
```

### Updating Ticket Status

When starting a ticket:
```yaml
- id: BOOTSTRAP-001
  title: "Token Type Definitions"
  status: in_progress  # ← Mark as in_progress
```

When completing a ticket:
```yaml
- id: BOOTSTRAP-001
  title: "Token Type Definitions"
  status: completed  # ← Mark as completed
  completed_date: 2025-10-18
```

---

## 🚫 What Gets Blocked

### Pre-commit Hook Blocks:

1. **No Ticket ID**
```bash
git commit -m "Add token types"  # ❌ BLOCKED
# Must be: "BOOTSTRAP-001: Add token types"
```

2. **SATD Comments**
```ruchy
// TODO: Fix this later  # ❌ BLOCKED
// FIXME: Performance    # ❌ BLOCKED
// HACK: Temporary       # ❌ BLOCKED
```

3. **No Documentation Update**
```bash
# Modified: bootstrap/stage0/token.ruchy
# Not modified: roadmap.yaml or INTEGRATION.md
# ❌ BLOCKED - Must update docs with code
```

4. **Syntax Errors**
```bash
ruchy check file.ruchy  # ❌ Fails
# Pre-commit BLOCKED
```

5. **Lint Failures**
```bash
ruchy lint file.ruchy  # Grade: B
# ❌ BLOCKED - Must achieve A+ grade
```

6. **Low TDG Score**
```bash
pmat tdg .  # Score: 82
# ❌ BLOCKED - Must be ≥85
```

### Commit-msg Hook Blocks:

1. **Invalid Ticket Format**
```bash
git commit -m "BOOT-001: Add tokens"  # ❌ Invalid prefix
git commit -m "bootstrap-001: Add tokens"  # ❌ Lowercase
git commit -m "001: Add tokens"  # ❌ No prefix
```

2. **Missing Ticket in Roadmap**
```bash
git commit -m "BOOTSTRAP-999: Add tokens"
# ⚠️  Warning: Ticket not found in roadmap.yaml
# (Currently non-blocking)
```

---

## 🎯 Quality Standards

### Mandatory Requirements

Every commit MUST:
- ✅ Reference a ticket from roadmap.yaml
- ✅ Have zero SATD comments
- ✅ Update documentation (roadmap.yaml or INTEGRATION.md)
- ✅ Pass ruchy check (syntax valid)
- ✅ Pass ruchy lint (A+ grade)
- ✅ Maintain TDG score ≥85
- ✅ Include tests written in pure Ruchy
- ✅ Follow RED-GREEN-REFACTOR cycle

### Code Quality Thresholds

```yaml
max_complexity: 20        # McCabe cyclomatic complexity
max_cognitive: 15         # Cognitive complexity
min_coverage: 0.80        # Test coverage (80%+)
satd_tolerance: 0         # Zero SATD allowed
tdg_score_min: 85         # TDG grade A- minimum
lint_grade: "A+"          # Ruchy lint grade
```

---

## 📚 Commands Reference

### Hook Management

```bash
make install-hooks      # Install git hooks
make validate-roadmap   # Validate roadmap.yaml structure
```

### Quality Checks (Run Before Commit)

```bash
make dogfood-check      # Syntax validation (all files)
make dogfood-lint       # Lint validation (A+ required)
make dogfood-quality    # Quality tool suite
make pmat-quality-gate  # PMAT TDG check (≥85)
```

### Testing

```bash
ruchy test <file>.ruchy                   # Run tests
ruchy test --coverage <file>.ruchy        # With coverage
make dogfood-test                         # Test all validation files
```

### Validation

```bash
ruchy check <file>.ruchy    # Syntax check
ruchy lint <file>.ruchy     # Lint check (A+ target)
ruchy fmt <file>.ruchy      # Format code
ruchy score <file>.ruchy    # Quality score (>0.8 target)
ruchy prove <file>.ruchy    # Formal verification
```

---

## 🔄 Example Complete Workflow

```bash
# 1. Install hooks (first time only)
make install-hooks

# 2. Select ticket from roadmap.yaml
# Selected: BOOTSTRAP-001 (Token Type Definitions)

# 3. Write tests (RED)
vim validation/tests/test_bootstrap_001.ruchy
ruchy test validation/tests/test_bootstrap_001.ruchy
# ❌ Tests fail (expected)

# 4. Implement (GREEN)
vim bootstrap/stage0/token.ruchy
ruchy test validation/tests/test_bootstrap_001.ruchy
# ✅ Tests pass

# 5. Refactor
ruchy fmt bootstrap/stage0/token.ruchy
ruchy lint bootstrap/stage0/token.ruchy  # A+ grade
ruchy score bootstrap/stage0/token.ruchy  # 0.95

# 6. Run quality checks
make dogfood-check
make dogfood-lint
make dogfood-quality
make pmat-quality-gate

# 7. Update documentation
vim roadmap.yaml  # Mark BOOTSTRAP-001 as completed
vim INTEGRATION.md  # Update status
vim CHANGELOG.md  # Add entry

# 8. Commit (hooks will validate)
git add -A
git commit -m "BOOTSTRAP-001: Implement token type definitions

Component: Stage 0 Lexer
Tests: 5 property tests via ruchy test
Coverage: 100%
Quality: A+ lint grade, score 0.95

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>"

# Pre-commit runs automatically:
# ✅ Ticket ID valid
# ✅ Zero SATD
# ✅ Documentation updated
# ✅ Syntax valid
# ✅ Lint A+
# ✅ TDG ≥85
# ✅ Roadmap valid

# 9. Push
git push origin main
```

---

## 🚨 Troubleshooting

### Hook Blocked My Commit

1. **Read the error message** - it tells you exactly what's wrong
2. **Fix the issue** - don't try to bypass hooks
3. **Re-run the commit**

### Forgot to Add Ticket ID

```bash
# Amend the commit message
git commit --amend

# Add ticket ID at the start
# Change: "Add token types"
# To:     "BOOTSTRAP-001: Add token types"
```

### Need to Bypass Hooks (EMERGENCY ONLY)

```bash
# ⚠️  USE ONLY IN TRUE EMERGENCIES
git commit --no-verify -m "EMERGENCY: Fix critical bug"

# Then immediately create a ticket and fix properly
```

### Update Hooks After Changes

```bash
# Hooks are in scripts/ directory
# Update scripts/pre-commit or scripts/commit-msg
# Then reinstall:
make install-hooks
```

---

## 📖 Additional Resources

- **roadmap.yaml**: All tickets and project plan
- **INTEGRATION.md**: Current project status and metrics
- **CLAUDE.md**: Claude Code specific instructions
- **Makefile**: All available commands

---

## 🎯 Success Criteria

A commit is ready when:
- ✅ All tests pass (RED → GREEN)
- ✅ Code is refactored and clean
- ✅ Lint grade is A+
- ✅ Quality score >0.8
- ✅ TDG score ≥85
- ✅ Zero SATD comments
- ✅ Documentation updated
- ✅ Ticket ID in commit message
- ✅ All quality gates pass

Follow this workflow for **every single commit** to maintain project quality and traceability.
