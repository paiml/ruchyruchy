# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 🚨 P0 CRITICAL: ALWAYS USE LOCAL BUILDS (DOGFOODING MANDATE)

**MANDATORY - ZERO TOLERANCE - BLOCKING - P0 PRIORITY**

YOU ARE BUILDING THE `ruchydbg` DEBUGGER - YOU MUST USE YOUR OWN BUILD!

### NEVER Use Installed Binaries

**FORBIDDEN**:
```bash
ruchydbg run test.ruchy              # ❌ WRONG - uses ~/.cargo/bin/ruchydbg (old version)
which ruchydbg                       # ❌ WRONG - points to installed version
/home/noah/.cargo/bin/ruchydbg       # ❌ WRONG - not your changes!
```

**MANDATORY**:
```bash
./target/debug/ruchydbg run test.ruchy    # ✅ CORRECT - uses YOUR parser changes
./target/debug/ruchydbg validate          # ✅ CORRECT - tests YOUR implementation
cargo run --bin ruchydbg -- run test.ruchy  # ✅ CORRECT - always fresh build
```

### Why This is P0 Critical

1. **You ARE the debugger** - `ruchydbg` is the RuchyRuchy interpreter binary you're developing
2. **Installed version is STALE** - doesn't include your parser fixes
3. **Defeats dogfooding** - testing old code invalidates the entire project purpose
4. **False validation** - bugs may appear "fixed" when they're not in your code
5. **Wastes time** - debugging the wrong codebase

### Before ANY ruchydbg Command

**ALWAYS**:
1. Check you're using `./target/debug/ruchydbg` (note the `./`)
2. Rebuild if needed: `cargo build --bin ruchydbg`
3. Verify version matches: `./target/debug/ruchydbg --version` should show current project version
4. Never use bare `ruchydbg` command without `./target/debug/` prefix

### Pre-commit Hook Enforcement

Add to `.git/hooks/pre-commit`:
```bash
# Verify no references to bare 'ruchydbg' command in commits
if git diff --cached | grep -E "^\+.*[^/]ruchydbg " | grep -v "# "; then
    echo "❌ ERROR: Found bare 'ruchydbg' command - MUST use './target/debug/ruchydbg'"
    exit 1
fi
```

**REMEMBER**: The point of this project is DOGFOODING our own debugging tools. Using the installed version defeats this entirely.

## Project Overview

RuchyRuchy is an educational compiler infrastructure project supporting the Ruchy programming language ecosystem. The project provides educational resources, development tools, and extensive validation frameworks for understanding compiler construction and testing compiler robustness.

## 🚨 CRITICAL: Bug Discovery Protocol

**MANDATORY PROCEDURE - ZERO TOLERANCE - BLOCKING**

When you discover ANY bug or not-implemented feature in Ruchy OR RuchyRuchy:

### Ruchy Compiler Bugs (https://github.com/paiml/ruchy/issues)

1. **STOP THE LINE** - Immediately halt all work
2. **FILE GITHUB ISSUE** - Create issue at https://github.com/paiml/ruchy/issues
3. **EXTREME DETAIL REQUIRED** - Issue MUST contain:
   - Exact reproduction steps
   - Minimal reproduction code (pure Ruchy)
   - Expected behavior vs actual behavior
   - Ruchy version (`ruchy --version`)
   - Full error output (copy-paste verbatim)
   - Context: what you were trying to accomplish
   - Impact: how this blocks current work
   - Workaround: if any exists

**Ruchy Issue Template**:
```markdown
## Bug Report: [Short Description]

**Ruchy Version**: [output of `ruchy --version`]
**Project**: RuchyRuchy Bootstrap Compiler
**Ticket**: [BOOTSTRAP-XXX or VALID-XXX]

### Reproduction Steps
1. [Step 1]
2. [Step 2]
3. [etc]

### Minimal Reproduction Code
```ruchy
// Exact code that triggers the bug
[paste code here]
```

### Expected Behavior
[What should happen]

### Actual Behavior
[What actually happens]

### Full Error Output
```
[Complete error message, copy-paste verbatim]
```

### Context
[What you were trying to accomplish when you found this]

### Impact
[How this blocks current work - e.g., "Blocks BOOTSTRAP-002", "Prevents character stream implementation"]

### Workaround
[If any workaround exists, describe it]

### Environment
- OS: [Linux/Mac/Windows]
- Ruchy install: [Cargo/Binary/etc]
```

**After Filing Ruchy Issue**:
1. Document the bug in BOUNDARIES.md
2. Implement workaround if possible
3. Continue with alternative approach
4. Reference GitHub issue # in commit messages
5. Update BUG_DISCOVERY_REPORT.md with finding

**Example**:
```
Discovered: Enum tuple variants cause "No match arm matched" runtime error
Action: Filed https://github.com/paiml/ruchy/issues/XXX
Workaround: Using unit variants only for Position tracking
Status: BOOTSTRAP-002 proceeding with simplified enum design
```

### RuchyRuchy Project Bugs (https://github.com/paiml/ruchyruchy/issues)

**MANDATORY - BLOCKING - ALL BUGS MUST BE TRACKED**

When you discover ANY bug in the RuchyRuchy codebase itself (not Ruchy compiler):

1. **FILE GITHUB ISSUE** - Create issue at https://github.com/paiml/ruchyruchy/issues
2. **CREATE TICKET** - Add to roadmap.yaml with proper ticket ID
3. **DOCUMENT** - Add to BUG_DISCOVERY_REPORT.md with:
   - Bug ID (BUG-XXX)
   - Severity (CRITICAL/HIGH/MEDIUM/LOW)
   - Discovery technique used
   - Reproduction steps
   - Impact assessment
   - Fix status

**RuchyRuchy Issue Template**:
```markdown
## Bug Report: [Short Description]

**Component**: [bootstrap/discovery/validation/etc]
**Severity**: [CRITICAL/HIGH/MEDIUM/LOW]
**Discovery**: [Which discovery technique found this]
**Version**: [ruchyruchy version]

### Reproduction Steps
1. [Step 1]
2. [Step 2]

### Expected Behavior
[What should happen]

### Actual Behavior
[What actually happens]

### Impact
[How this affects users/developers]

### Suggested Fix
[If known]
```

**Bug Filing Requirements (MANDATORY)**:
- ✅ ALL bugs MUST have GitHub issue
- ✅ ALL bugs MUST have ticket in roadmap.yaml
- ✅ ALL bugs MUST be in BUG_DISCOVERY_REPORT.md
- ✅ Critical bugs MUST be fixed before release
- ✅ All bug fixes MUST have tests
- ✅ All bug fixes MUST update CHANGELOG.md

**Severity Definitions**:
- **CRITICAL**: Crash, data loss, security vulnerability, blocks all work
- **HIGH**: Major functionality broken, blocks specific features
- **MEDIUM**: Minor functionality broken, workaround exists
- **LOW**: Cosmetic issues, documentation errors

**Bug Discovery Tracking**:
Every bug discovery session MUST:
1. Update BUG_DISCOVERY_REPORT.md with findings
2. File GitHub issues for all CRITICAL and HIGH bugs
3. Create tickets in roadmap.yaml for fixes
4. Update CHANGELOG.md with discoveries
5. Commit with VALID-XXX ticket ID

## Critical Requirements

### MUST Use Pure Ruchy Tooling (Dogfooding)
**ALL testing and validation infrastructure in this project MUST use Ruchy binary tools:**
- `ruchy test` - Run tests (pure Ruchy test files)
- `ruchy lint` - Lint code with A+ quality requirements
- `ruchy fmt` - Format code to canonical style
- `ruchy prove` - Formal verification and property testing
- `ruchy score` - Quality scoring and complexity analysis
- `ruchy runtime` - Performance analysis and boundary testing
- `ruchy check` - Type checking and syntax validation

**Rationale**: A Ruchy project MUST dogfood Ruchy tools. Using external toolchains undermines credibility and prevents self-hosting validation.

### MUST Use BashRS for Bash Script Validation and Generation

**MANDATORY - ZERO TOLERANCE - BLOCKING**

ALL bash scripts (.sh files) MUST be validated and/or generated using **BashRS** (https://github.com/paiml/bashrs), which is OUR OWN TOOL:

**BashRS - OUR Tool (MANDATORY)**:
1. **Repository**: https://github.com/paiml/bashrs
2. **Purpose**: Rust-to-bash transpiler + validation
3. **Install**: `cargo install bashrs`
4. **Usage**: Write Rust code, generate type-safe bash
5. **Validation**: `bashrs lint`, `bashrs check`, `bashrs analyze`

**Why BashRS** (Our Tool):
- **Dogfooding**: We wrote it, we MUST use it
- **Type Safety**: Write bash logic in Rust (memory-safe, type-safe)
- **Validation**: Catches errors before runtime
- **Reproducibility**: Deterministic bash generation from Rust
- **Debuggability**: Rust tooling for bash scripts
- **Quality**: Enforces best practices automatically

**BashRS Bug Discovery Protocol** (MANDATORY):

When you encounter ANY bug, missing feature, or limitation in BashRS:

1. **STOP THE LINE** - Immediately halt work
2. **FILE GITHUB ISSUE** at https://github.com/paiml/bashrs/issues
3. **EXTREME DETAIL REQUIRED** - Issue MUST contain:
   - Exact reproduction steps
   - Minimal reproduction code (Rust and generated bash)
   - Expected behavior vs actual behavior
   - BashRS version (`bashrs --version`)
   - Full error output (copy-paste verbatim)
   - Context: what you were trying to accomplish
   - Impact: how this blocks current work
   - Workaround: if any exists

**Example BashRS Issue**:
```markdown
## Bug Report: bashrs lint fails on valid bash script

**BashRS Version**: [output of `bashrs --version`]
**Project**: RuchyRuchy
**Context**: Trying to lint existing bash scripts in pre-commit hook

### Reproduction Steps
1. Create bash script with: [code]
2. Run: bashrs lint script.sh
3. Error: [error message]

### Expected Behavior
bashrs lint should validate existing bash scripts (not just Rust-generated ones)

### Actual Behavior
bashrs lint fails with parse error

### Impact
Blocks pre-commit hook validation for bash scripts

### Workaround
[If any - e.g., use shellcheck temporarily, or rewrite in Rust first]

### Request
Add bash-to-bash validation mode, or clarify usage model
```

**After Filing BashRS Issue**:
1. Document in project notes (BOUNDARIES.md, issue tracker)
2. Implement workaround if possible (rewrite in Rust, simplify script)
3. Reference issue # in commits
4. Continue with alternative approach until fixed
5. **NEVER**: Use shellcheck or other non-BashRS tools as workaround

**Enforcement** (BASHRS ONLY - ZERO TOLERANCE):
1. **Pre-commit Hook**: ALL .sh files MUST pass `bashrs lint` (errors only, warnings non-blocking)
2. **Quality Gates**: BashRS validation BLOCKING
3. **Book Scripts**: ALL reproduction scripts MUST pass bashrs lint
4. **ALL Scripts**: MUST be bashrs-generated OR pass bashrs lint
5. **NEVER shellcheck**: We dogfood BashRS, our own tool

**BashRS Commands Explained**:
- `bashrs lint <script.sh>` - Static analysis of existing bash scripts (use for all .sh files)
- `bashrs check <source.rs>` - Validate Rust code before transpiling to bash (use for .rs files)
- `bashrs build <source.rs>` - Transpile Rust to bash (generates .sh from .rs)
- `bashrs analyze <script.sh>` - Security analysis (non-blocking warnings)

**BashRS Usage**:
```bash
# Generate bash from Rust (PREFERRED for new scripts)
# File: scripts/validate-book.rs
bashrs check scripts/validate-book.rs    # Validate Rust source
bashrs build scripts/validate-book.rs > scripts/validate-book.sh  # Transpile

# Validate existing bash scripts
bashrs lint scripts/validate-book.sh     # Static analysis (MANDATORY)
bashrs analyze scripts/validate-book.sh  # Security analysis (non-blocking)

# Analyze for security
bashrs analyze scripts/validate-book.sh

# Format (if supported)
bashrs fmt scripts/validate-book.sh
```

**Pre-commit Integration** (BASHRS ONLY):
```bash
# In .git/hooks/pre-commit
for file in $(git diff --cached --name-only --diff-filter=ACM | grep '\.sh$'); do
    bashrs lint "$file" || exit 1
    bashrs check "$file" || exit 1
done
```

**Required Bash Script Standards** (Enforced by BashRS):
1. **Shebang**: `#!/bin/bash` (not #!/bin/sh)
2. **Safety**: `set -euo pipefail` at top
3. **Documentation**: Header comment with purpose, exit codes
4. **Exit Codes**: Explicit exit 0/1
5. **Variables**: Use `${VAR}` not `$VAR`
6. **Quoting**: Quote all variables: `"$VAR"`
7. **Error Handling**: Explicit error messages
8. **Idempotence**: Scripts MUST be re-runnable
9. **Reproducibility**: Deterministic behavior
10. **Debuggability**: Clear error messages and logging

**No bash script is complete without**:
- ✅ bashrs lint passing (zero errors)
- ✅ bashrs check passing (syntax valid)
- ✅ bashrs analyze passing (no security issues)
- ✅ set -euo pipefail enabled
- ✅ Proper exit codes (0 = success, 1 = failure)
- ✅ Full error handling
- ✅ All variables quoted
- ✅ Security best practices

**CRITICAL**: If bashrs has issues, file bug at https://github.com/paiml/bashrs/issues
**NEVER**: Fall back to shellcheck or other tools - BashRS is OUR tool, we dogfood it

### MUST Use Correct Ruchy Syntax

**MANDATORY - ZERO TOLERANCE**

Ruchy has specific syntax that differs from Rust. The following are **REQUIRED**:

1. **Function Keyword**: MUST use `fun` (NOT `fn`)
   ```ruchy
   // ✅ Correct
   fun main() {
       println("Hello");
   }

   // ❌ Wrong - will fail ruchy check
   fn main() {
       println("Hello");
   }
   ```

2. **Common Mistakes to Avoid**:
   - ❌ `fn` - This is Rust, not Ruchy
   - ✅ `fun` - This is the correct Ruchy keyword
   - All function declarations MUST use `fun`

3. **Enforcement**:
   - Pre-commit hooks check for `fn` keyword usage
   - `ruchy check` will fail if `fn` is used instead of `fun`
   - Quality gates block commits with incorrect syntax

**Discovery**: During Phase 1, 148 function declarations needed correction from `fn` to `fun`. This is now enforced at the quality gate level to prevent regression.

### MUST Maintain TDD Book Documentation

**MANDATORY - ZERO TOLERANCE - BLOCKING**

Following the pattern from `../ruchy-book`, `../ruchy`, and `../paiml-mcp-agent-toolkit`, this project MUST maintain a comprehensive book documenting all development via EXTREME TDD with full tool validation:

**Book Requirements** (ALL MANDATORY):
1. **Location**: `book/` directory at repository root
2. **Format**: Markdown chapters published via GitHub Pages
3. **Structure**: `book/src/SUMMARY.md` with chapter links
4. **Build**: mdBook with automated validation
5. **Publishing**: GitHub Actions workflow for automatic deployment
6. **Reproducibility**: All examples MUST be executable via scripts
7. **Debuggability**: All code examples MUST be debuggable
8. **Tool Validation**: ALL chapters MUST validate against 16 Ruchy tools + ruchyruchy debuggers

**16 Ruchy Tools - ALL MUST BE VALIDATED**:
1. `ruchy check` - Syntax and type checking
2. `ruchy test` - Test execution
3. `ruchy lint` - Code quality (A+ grade required)
4. `ruchy fmt` - Code formatting
5. `ruchy prove` - Formal verification
6. `ruchy score` - Quality metrics (>0.8 required)
7. `ruchy runtime` - Performance analysis
8. `ruchy build` - Compilation
9. `ruchy run` - Execution
10. `ruchy doc` - Documentation generation
11. `ruchy bench` - Benchmarking
12. `ruchy profile` - Performance profiling
13. `ruchy coverage` - Code coverage
14. `ruchy deps` - Dependency analysis
15. `ruchy security` - Security scanning
16. `ruchy complexity` - Complexity analysis

**RuchyRuchy Debuggers - ALL MUST BE VALIDATED**:
1. `ruchydbg validate` - Debugging tools validation
2. Source map generation validation
3. Record-replay time-travel validation
4. Performance regression checks

**Content Requirements - EVERY ticket MUST have**:
1. **RED Phase**: Document the failing test first
   - Test code in pure Ruchy
   - Why it fails
   - Expected behavior vs actual behavior
   - Validation: `ruchy test` shows failure

2. **GREEN Phase**: Document the minimal implementation
   - Code that makes test pass
   - No extra features beyond test requirements
   - Validation: `ruchy test` shows success

3. **REFACTOR Phase**: Document improvements
   - What was refactored
   - Why (performance, clarity, maintainability)
   - Validation: Tests still passing

4. **TOOL VALIDATION Phase** (NEW - MANDATORY):
   - Run ALL 16 Ruchy tools
   - Run ALL ruchyruchy debuggers
   - Document results for each tool
   - BLOCKING if any tool fails

5. **REPRODUCIBILITY Phase** (NEW - MANDATORY):
   - Provide executable script
   - Script MUST reproduce all results
   - Script MUST be idempotent
   - Script MUST exit with status code

6. **DEBUGGABILITY Phase** (NEW - MANDATORY):
   - Code MUST be debuggable with ruchydbg
   - Source maps MUST be validated
   - Time-travel MUST be demonstrated
   - Performance MUST be benchmarked

**Book Structure**:
```
book/
├── book.toml                    # mdBook configuration
├── src/
│   ├── SUMMARY.md              # Table of contents
│   ├── introduction.md         # Project overview
│   ├── phase1_infrastructure/
│   │   ├── chapter.md
│   │   └── tickets/
│   │       ├── infra-001-roadmap.md
│   │       └── infra-002-quality-gates.md
│   ├── phase2_validation/
│   │   ├── chapter.md
│   │   └── tickets/
│   │       ├── valid-001-self-compilation.md
│   │       ├── valid-003-property-testing.md
│   │       ├── valid-004-fuzz-testing.md
│   │       └── valid-005-boundary-analysis.md
│   ├── phase3_bootstrap/
│   │   ├── chapter.md
│   │   └── stage0/
│   │       ├── bootstrap-001-token-types.md
│   │       ├── bootstrap-002-char-stream.md
│   │       └── bootstrap-003-core-lexer.md
│   └── discoveries/
│       ├── boundaries.md       # Links to BOUNDARIES.md
│       └── runtime-enhancements.md
```

**Chapter Template** (for each ticket - EXTREME TDD):
```markdown
# TICKET-XXX: Feature Name

## Context
[Why this feature is needed, what problem it solves]

## RED: Write Failing Test
[The test that was written first]
```ruchy
// Test code here (MUST be in repository)
// File: validation/tests/test_TICKET_XXX.ruchy
```
**Expected**: [What should happen]
**Actual**: [What currently happens - failure]
**Validation**: `ruchy test validation/tests/test_TICKET_XXX.ruchy` exits with status 1

## GREEN: Minimal Implementation
[Code that makes test pass]
```ruchy
// Implementation code (MUST be in repository)
// File: bootstrap/stageN/TICKET_XXX_implementation.ruchy
```
**Result**: ✅ Test passes
**Validation**: `ruchy test validation/tests/test_TICKET_XXX.ruchy` exits with status 0

## REFACTOR: Improvements
[Any refactoring done while keeping tests green]

## TOOL VALIDATION (MANDATORY - ALL 16 TOOLS)
Execute and document results:
```bash
./scripts/validate-ticket-TICKET-XXX.sh
```

Results:
1. `ruchy check`: ✅ Pass / ❌ Fail [error message]
2. `ruchy test`: ✅ X/X tests passing
3. `ruchy lint`: ✅ A+ grade
4. `ruchy fmt`: ✅ No formatting changes
5. `ruchy prove`: ✅ Properties verified
6. `ruchy score`: ✅ Score X.XX (>0.8 required)
7. `ruchy runtime`: ✅ Performance within bounds
8. `ruchy build`: ✅ Compilation successful
9. `ruchy run`: ✅ Execution successful
10. `ruchy doc`: ✅ Documentation generated
11. `ruchy bench`: ✅ Benchmarks within thresholds
12. `ruchy profile`: ✅ No performance regressions
13. `ruchy coverage`: ✅ >80% coverage achieved
14. `ruchy deps`: ✅ No dependency issues
15. `ruchy security`: ✅ No security vulnerabilities
16. `ruchy complexity`: ✅ Complexity <20 per function

**RuchyRuchy Debugger Validation**:
1. `ruchydbg validate`: ✅ All checks passing
2. Source maps: ✅ 1:1 line mapping verified
3. Time-travel: ✅ Backward stepping works
4. Performance: ✅ <6s validation (target: 0.013s achieved)

## REPRODUCIBILITY (MANDATORY)
**Script**: `scripts/reproduce-ticket-TICKET-XXX.sh`
```bash
#!/bin/bash
# Reproduces all results for TICKET-XXX
# Exit status: 0 = success, 1 = failure
# Idempotent: Can be run multiple times

set -euo pipefail

echo "Reproducing TICKET-XXX results..."

# Run all tests
ruchy test validation/tests/test_TICKET_XXX.ruchy

# Run all validations
ruchy check bootstrap/stageN/TICKET_XXX_implementation.ruchy
ruchy lint bootstrap/stageN/TICKET_XXX_implementation.ruchy
# ... (all 16 tools)

echo "✅ All results reproduced successfully"
exit 0
```

**Execution**:
```bash
chmod +x scripts/reproduce-ticket-TICKET-XXX.sh
./scripts/reproduce-ticket-TICKET-XXX.sh
# Exit status: 0
```

## DEBUGGABILITY (MANDATORY)
**Debug Session Example**:
```bash
# Start debugging session
ruchydbg validate validation/tests/test_TICKET_XXX.ruchy

# Verify source maps
# Verify time-travel stepping
# Verify performance benchmarks
```

**Results**:
- Source map accuracy: 100% (N/N lines mapped)
- Time-travel steps: X backward, Y forward
- Performance: <0.1s per debug operation

## Discoveries
[Any boundaries, bugs, or learnings discovered]

## Next Steps
[What this enables, next ticket to implement]

## Validation Summary
- ✅ RED phase: Test failed as expected
- ✅ GREEN phase: Test passed
- ✅ REFACTOR phase: Tests still passing
- ✅ TOOL VALIDATION: All 16 tools passing
- ✅ DEBUGGER VALIDATION: All debuggers working
- ✅ REPRODUCIBILITY: Script exits with status 0
- ✅ DEBUGGABILITY: Debug session successful

**Status**: 🟢 COMPLETE (7/7 phases validated)
```

**GitHub Pages Deployment**:
```yaml
# .github/workflows/book.yml
name: Deploy Book
on:
  push:
    branches: [main]
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Setup mdBook
        uses: peaceiris/actions-mdbook@v1
      - name: Build book
        run: mdbook build
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./book/book
```

**Integration with Development** (MANDATORY - BLOCKING):
- Every commit for a ticket MUST update corresponding book chapter
- Book chapters MUST be written in EXTREME TDD order (RED-GREEN-REFACTOR-TOOL-VALIDATION-REPRODUCIBILITY-DEBUGGABILITY)
- Book serves as living documentation of development process
- Book published automatically to GitHub Pages on push
- ALL code examples MUST exist in repository
- ALL scripts MUST be executable and validated
- ALL tool validations MUST be documented

**Examples to Follow**:
- `../ruchy-book`: Language reference and tutorial structure
- `../ruchy`: Compiler implementation documentation
- `../paiml-mcp-agent-toolkit`: MCP server development documentation

**Enforcement** (ZERO TOLERANCE - BLOCKING):
1. **Pre-commit Hook Checks**:
   - Book chapter MUST exist for every ticket
   - Book chapter MUST include all 7 phases (RED, GREEN, REFACTOR, TOOL VALIDATION, REPRODUCIBILITY, DEBUGGABILITY, SUMMARY)
   - All referenced code files MUST exist in repository
   - All referenced scripts MUST be executable
   - Book MUST be buildable via `mdbook build`

2. **Validation Script** (`scripts/validate-book.sh`):
   ```bash
   #!/bin/bash
   # Validates book completeness and correctness
   # Exit status: 0 = valid, 1 = invalid

   set -euo pipefail

   echo "🔍 Validating book..."

   # Check book builds
   cd book && mdbook build

   # Check all tickets have chapters
   for ticket in $(grep -E "^\s+- id: (INFRA|VALID|BOOTSTRAP)-" ../roadmap.yaml | sed 's/.*id: //'); do
       if ! grep -q "$ticket" src/SUMMARY.md; then
           echo "❌ ERROR: Missing book chapter for $ticket"
           exit 1
       fi
   done

   # Check all code examples exist
   for file in $(grep -r "File: " src/ | sed 's/.*File: //' | sed 's/`.*//'); do
       if [ ! -f "../$file" ]; then
           echo "❌ ERROR: Missing code file: $file"
           exit 1
       fi
   done

   # Check all scripts are executable
   for script in $(grep -r "scripts/" src/ | grep -o "scripts/[^'\" ]*" | sort -u); do
       if [ ! -x "../$script" ]; then
           echo "❌ ERROR: Script not executable: $script"
           exit 1
       fi
   done

   echo "✅ Book validation passed!"
   exit 0
   ```

3. **Pre-commit Hook Addition**:
   - Book validation script MUST be called in pre-commit hook
   - Commits BLOCKED if book validation fails
   - No bypass allowed (--no-verify FORBIDDEN)

4. **Roadmap Integration**:
   - Ticket status "completed" REQUIRES book chapter existence
   - Book chapter MUST include all 7 validation phases
   - Reproducibility script MUST exit with status 0

**Ticket Completion Checklist** (ALL MANDATORY):
- [ ] RED phase: Test written and failing
- [ ] GREEN phase: Test passing
- [ ] REFACTOR phase: Code cleaned up
- [ ] TOOL VALIDATION: All 16 Ruchy tools validated
- [ ] DEBUGGER VALIDATION: All ruchyruchy debuggers validated
- [ ] REPRODUCIBILITY: Script created and tested (exit status 0)
- [ ] DEBUGGABILITY: Debug session documented
- [ ] BOOK CHAPTER: Chapter created with all 7 phases
- [ ] CODE FILES: All referenced files exist in repository
- [ ] SCRIPTS: All scripts executable and validated
- [ ] ROADMAP: Ticket marked "completed" in roadmap.yaml
- [ ] INTEGRATION.MD: Status updated

**Pre-Commit Hook Enforcement**:
```bash
# In .git/hooks/pre-commit
echo "🔍 Validating book completeness..."
./scripts/validate-book.sh || exit 1
```

**No ticket is complete without:**
1. Book chapter with all 7 phases documented
2. All code examples in repository
3. Reproducibility script passing
4. All 16 tools validated
5. All debuggers validated
6. mdBook building successfully

## Phase 2: Validation & Robustness (Current Focus)

### Mission
Extensive validation of Ruchy tooling against Ruchy code compiled by Ruchy, with heavy focus on property testing and fuzz testing to discover the exact boundaries where our tools work and where they fail.

### Core Validation Objectives
1. **Self-Compilation Testing**: Validate tools against Ruchy-compiled code
2. **Property-Based Testing**: Mathematical property validation via `ruchy prove`
3. **Fuzz Testing**: Boundary and edge case discovery
4. **Pure Ruchy Dogfooding**: All tests written in Ruchy using Ruchy tools
5. **Performance Analysis**: Comprehensive boundary mapping via `ruchy runtime`

### Property Testing Requirements
All property tests MUST:
- Be written in pure Ruchy (.ruchy files)
- Use `ruchy prove` for mathematical property validation
- Test mathematical properties (e.g., roundtrip: `parse(emit(ast)) = ast`)
- Run minimum 10,000 test cases per property via `ruchy test`
- Include shrinking for minimal failure cases
- Achieve >80% coverage via `ruchy score`

### Fuzz Testing Requirements
All fuzz tests MUST:
- Be implemented in pure Ruchy
- Generate both valid and invalid inputs using Ruchy
- Use grammar-based generation for valid cases
- Track crash/hang/timeout statistics via `ruchy runtime`
- Minimize failing test cases using Ruchy tooling
- Store corpus for regression testing in .ruchy format

### Validation Pipeline (Pure Ruchy Only)
```bash
# All validation MUST use Ruchy tools
ruchy test validation/**/*.ruchy       # Run all validation tests
ruchy fmt validation/**/*.ruchy        # Format all validation code
ruchy lint validation/**/*.ruchy       # Lint with A+ requirement
ruchy prove validation/**/*.ruchy      # Formal verification
ruchy score validation/**/*.ruchy      # Quality analysis >0.8
ruchy runtime validation/**/*.ruchy    # Performance validation
```

## Architecture

### Bootstrap Progression
The compiler follows a four-stage bootstrap sequence:
- **Stage 0 (Lexer)**: 1K LOC - Tokenizes source code including itself
- **Stage 1 (Parser)**: 3K LOC - Parses Stage 0+1 using Pratt parser and recursive descent
- **Stage 2 (TypeCheck)**: 5K LOC - Type checks all stages using Algorithm W
- **Stage 3 (CodeGen)**: 6K LOC - Generates TypeScript/Rust code for all stages

### Repository Structure
```
ruchyruchy/
├── bootstrap/           # Progressive compiler stages
│   ├── stage0/         # Lexical analysis (tokens, lexer)
│   ├── stage1/         # Syntax analysis (parser, AST)
│   ├── stage2/         # Type inference (Algorithm W, unification)
│   └── stage3/         # Code generation (TypeScript/Rust emission)
├── validation/         # Pure Ruchy testing and validation
│   ├── property/       # Property-based tests (.ruchy files)
│   ├── fuzz/          # Fuzz testing infrastructure (.ruchy files)
│   ├── boundary/      # Boundary analysis tests (.ruchy files)
│   └── regression/    # Regression test suite (.ruchy files)
└── docs/              # Specifications and documentation
```

## Development Commands

### Phase 2 Validation Commands (Pure Ruchy Only)
```bash
# Property Testing (via Ruchy)
ruchy test validation/property/lexer_test.ruchy
ruchy test validation/property/parser_test.ruchy
ruchy test validation/property/types_test.ruchy
ruchy test validation/property/codegen_test.ruchy

# Property Verification (Mathematical Proofs)
ruchy prove validation/property/lexer_test.ruchy
ruchy prove validation/property/parser_test.ruchy
ruchy prove validation/property/types_test.ruchy
ruchy prove validation/property/codegen_test.ruchy

# Fuzz Testing (via Ruchy)
ruchy test validation/fuzz/fuzzer.ruchy
ruchy test validation/fuzz/grammar_gen.ruchy
ruchy test validation/fuzz/differential.ruchy

# Boundary Analysis (via Ruchy)
ruchy runtime validation/boundary/perf_limits.ruchy
ruchy test validation/boundary/feature_matrix.ruchy
ruchy test validation/boundary/error_recovery.ruchy

# Quality Analysis
ruchy score validation/**/*.ruchy  # Must achieve >0.8 score
ruchy lint validation/**/*.ruchy   # Must achieve A+ grade
```

### Bootstrap Build Commands
```bash
# Progressive bootstrap stages (each depends on previous)
make stage0              # Build Stage 0: Lexer (1K LOC, tokenizes itself)
make stage1              # Build Stage 1: Parser (3K LOC, parses Stage 0+1)
make stage2              # Build Stage 2: TypeCheck (5K LOC, types all stages) 
make stage3              # Build Stage 3: CodeGen (6K LOC, compiles everything)
make bootstrap-all       # Build all stages in sequence with validation
```

### Self-Compilation Testing
```bash
# Test each stage's self-compilation capability
make test-stage0         # Test self-tokenization: ./lexer < lexer.ruchy
make test-stage1         # Test self-parsing with roundtrip validation
make test-stage2         # Test self-type-checking with Algorithm W
make test-stage3         # Test self-compilation (bit-identical output)
make test-self-compilation  # Complete self-compilation test suite
make test-differential   # Compare output with production compiler
```

### Toyota Way Quality Gates (MANDATORY - BLOCKING - Pure Ruchy Only)
```bash
make quality-gate        # Run ALL mandatory quality checks (BLOCKING)
make validate           # Comprehensive validation including quality gates
make ruchy-lint         # A+ grade requirement via ruchy lint
make ruchy-test         # All test suites via ruchy test
make ruchy-prove        # Formal verification via ruchy prove
make ruchy-score        # Quality score >0.8 via ruchy score
make ruchy-runtime      # Performance analysis via ruchy runtime
```

### Ruchy Formal Verification Integration (Dogfooding Excellence)
```bash
# Showcase Ruchy's advanced capabilities on bootstrap code
make verify-all         # Run formal verification on all stages
make complexity-analysis # BigO complexity analysis with ruchy runtime
make provability-check  # Mathematical correctness proofs via ruchy prove
make quality-scoring    # Unified quality assessment with ruchy score
```

### Toyota Way Continuous Improvement
```bash
make analyze-complexity  # Find complexity hotspots (Genchi Genbutsu)
make kaizen-refactor    # Generate continuous improvement plan
make quality-report     # Comprehensive quality metrics dashboard
```

## Development Workflow

### Quality Requirements (Pure Ruchy Dogfooding)
- All functions must have <20 cyclomatic complexity
- Code must pass `ruchy lint` checks with A+ grade
- Performance target: <5% overhead vs production compiler
- Memory usage: <100MB peak RSS for 10K LOC input
- All tests must run via `ruchy test` (.ruchy files only)
- Coverage must exceed 80% via `ruchy score`
- All validation code must be written in pure Ruchy

### Testing Strategy
- **Differential Testing**: Compare output with production Ruchy compiler
- **Self-compilation**: Each stage must compile itself
- **Performance Validation**: Throughput targets vary by stage
- **Property Testing**: Roundtrip validation for parser (parse(ast.emit()) == ast)
- **Fuzz Testing**: Grammar-based and mutation-based fuzzing
- **Boundary Testing**: Find exact limits of functionality

### File Extensions and Languages (Pure Ruchy Only)
- `.ruchy` files contain Ruchy language source code
- Generated output is TypeScript (`.ts`) or Rust code (`.rs`) 
- Test files MUST be pure Ruchy (`.ruchy`) run via `ruchy test`
- Build artifacts go in `build/` directory
- JSON intermediate representations for AST and typed AST

## Key Implementation Details

### Stage 0 - Lexer
- Implements minimal token set (12 keywords, literals, operators, delimiters)
- Target: >10K LOC/s throughput
- Self-validation: tokenizes own source code
- Property: `concat(tokenize(a), tokenize(b)) = tokenize(a + b)`

### Stage 1 - Parser  
- Uses Pratt parser for expressions with operator precedence
- Recursive descent for declarations and statements
- Target: >5K LOC/s throughput
- Roundtrip property testing required
- Property: `parse(emit(ast)) = ast` (verified via `ruchy prove`)

### Stage 2 - Type Inference
- Implements Algorithm W (Hindley-Milner type inference)
- Includes constraint solving and generalization
- Must handle occurs check and infinite type prevention
- Target: O(n log n) complexity
- Property: Well-typed programs don't crash

### Stage 3 - Code Generation
- Emits idiomatic TypeScript and Rust code
- Generated code must pass `ruchy fmt` and `ruchy lint`
- Target: >10K LOC/s throughput
- Must achieve semantic equivalence with source
- Property: Behavior preservation across targets

## Validation and Integration

The project maintains an INTEGRATION.md file tracking:
- Build matrix status for each stage
- Differential testing results comparing with production compiler
- Performance regression tracking
- Quality gate compliance
- Ruchy tool validation results

Success is measured by:
1. Self-compilation achieving bit-identical output
2. All property tests passing (10,000+ cases each)
3. Zero crashes from fuzz testing (1M+ inputs)
4. Complete boundary documentation
5. 80%+ test coverage via `ruchy score`

## Project Management Protocol (Toyota Way)

### Sacred Rules (Zero Tolerance - BLOCKING)
1. **NEVER bypass quality gates** - `git commit --no-verify` is FORBIDDEN
2. **ALWAYS test self-compilation** - Every stage must compile itself progressively  
3. **NEVER exceed complexity budget** - All functions <20 cognitive complexity
4. **ZERO SATD tolerance** - No TODO/FIXME/HACK comments allowed
5. **ALWAYS use formal verification** - Every algorithm must pass `ruchy provability`
6. **NEVER implement without specification** - All work must reference ROADMAP.md tasks
7. **ALWAYS use pure Ruchy** - All tests/validation via ruchy binary

### Sprint-Based Development Process (MANDATORY)
**CRITICAL**: Every sprint ends with commit and push to GitHub

```bash
# Sprint Structure (1-2 week cycles aligned with ROADMAP_PHASE2.md)
Sprint N: VALID-XXX/PROP-XXX/FUZZ-XXX Implementation
├── Day 1-3: Implementation following Toyota Way
├── Day 4-6: Property/fuzz testing via ruchy test  
├── Day 7-10: Boundary analysis and documentation
└── END: git commit && git push (MANDATORY)

# Sprint Commit Message Format (MANDATORY - Pure Ruchy Only)
git commit -m "VALID-XXX: Implement [component] with pure Ruchy validation

Component: [Validation/Property/Fuzz/Boundary]
Tests: [X] property tests, [Y] fuzz cases via ruchy test
Coverage: [Z]% via ruchy score (>0.8 required)
Boundaries: [List of discovered limits]
Performance: [actual] vs [target] metrics via ruchy runtime

Toyota Way: [Kaizen/Genchi Genbutsu/Jidoka] principle applied
Dogfooding: 100% pure Ruchy implementation and testing

**MANDATORY**: Updated roadmap.yaml with ticket status (completed/in_progress)
**MANDATORY**: Updated INTEGRATION.md or CHANGELOG.md with progress

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

### Quality Gate Integration Protocol
**Pre-commit hooks are MANDATORY and BLOCKING:**
```bash
# Install quality gates (REQUIRED for all contributors)
make install-hooks

# Every commit automatically validates:
# 1. Ruchy formal verification (ruchy prove)
# 2. Self-compilation tests (progressive validation)
# 3. Complexity analysis (functions <20)
# 4. Lint and test standards (ruchy lint A+ grade)
# 5. Format check (ruchy fmt --check)
# 6. Quality score (ruchy score >0.8)
```

### INTEGRATION.md Tracking (Single Source of Truth)
Following the pattern from ../ruchy-book, all progress must be tracked in `INTEGRATION.md`:
- **Real-time validation progress** (VALID/PROP/FUZZ/BOUND completion)
- **Property test results** with case counts
- **Fuzz test statistics** (crashes, hangs, coverage)
- **Boundary documentation** (limits and capabilities)
- **Ruchy tool dogfooding** results

**MANDATORY**: Update INTEGRATION.md after every sprint completion.

### Version Management Protocol
```bash
# FOOLPROOF version update process (like ../ruchy-book)
make sync-version       # Updates everything automatically
                       # - Detects latest Ruchy version
                       # - Updates all references
                       # - Tests bootstrap compatibility
                       # - Updates integration docs
```

### Performance Target Validation
All stages must meet empirical performance targets:
- **Stage 0 Lexer**: >10K LOC/s throughput measurement
- **Stage 1 Parser**: >5K LOC/s with roundtrip validation
- **Stage 2 TypeCheck**: O(n log n) complexity proof via `ruchy runtime`
- **Stage 3 CodeGen**: >10K LOC/s with bit-identical output validation
- **Property Tests**: >1000 cases/second via `ruchy test`
- **Fuzz Tests**: >10K inputs/second generation rate

### Continuous Deployment Protocol
Following ../rosetta-ruchy pattern:
1. **Every stage completion** triggers automatic release
2. **GitHub push MANDATORY** after stage validation
3. **Version bumping** follows semantic versioning
4. **Quality metrics** tracked in release notes
5. **Ruchy quality reports** (score, lint, prove) included in releases

### The Kaizen Refactoring Loop
```bash
# Step 1: Find complexity hotspots (Genchi Genbutsu)
make analyze-complexity

# Step 2: Generate improvement plan (Jidoka)
make kaizen-refactor --target bootstrap/stage[N]/

# Step 3: Apply improvements and validate
make quality-gate

# Step 4: Validate with Ruchy tools
ruchy test validation/**/*.ruchy
ruchy fmt validation/**/*.ruchy
ruchy lint validation/**/*.ruchy
ruchy prove validation/**/*.ruchy
ruchy score validation/**/*.ruchy
```
- push changes at end of each sprint to GitHub
# important-instruction-reminders
Do what has been asked; nothing more, nothing less.
NEVER create files unless they're absolutely necessary for achieving your goal.
ALWAYS prefer editing an existing file to creating a new one.
NEVER proactively create documentation files (*.md) or README files. Only create documentation files if explicitly requested by the User.

      
      IMPORTANT: this context may or may not be relevant to your tasks. You should not respond to this context unless it is highly relevant to your task.
- no feature can be added or code changed without following TDD.  All examples must be PURE ruchy and use ruchy tooling to test like ../ruchy-repl-demos.
- lets look at the work from ../ruchy-repl-demo and ../ruchy-book and ensure we copy the style.  in particulare we want TDD.  Ruchy tooling dogfooded, and to never work on code that isn't in roadmap and doesn't have a ticket.