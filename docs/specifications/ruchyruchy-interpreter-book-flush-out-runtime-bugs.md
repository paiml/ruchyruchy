# RuchyRuchy Interpreter: Systematic Runtime Bug Discovery via Book Examples

**Status**: ACTIVE - Phase 3
**Version**: 1.0.0
**Created**: 2025-10-30
**Project**: RuchyRuchy Educational Compiler Infrastructure
**Goal**: Build interpreter to execute ALL ruchy-book examples, discovering runtime bugs systematically

---

## Executive Summary

Build a **Ruchy interpreter in Rust** (ruchyruchy) that executes every example from `../ruchy-book` to systematically discover and document runtime bugs in the Ruchy language implementation. This interpreter will serve as a **bug discovery tool** and **runtime conformance tester** for the Ruchy compiler.

**Key Insight**: By running ALL examples from the comprehensive ruchy-book (23+ chapters, 100+ examples), we create a high-coverage runtime test suite that exposes edge cases, runtime errors, and undefined behaviors that static analysis cannot detect.

---

## Research Foundation

### 1. Interpreter Design & Implementation

**[1] Aho, Lam, Sethi, Ullman (2006)** - *"Compilers: Principles, Techniques, and Tools"* (2nd Ed.)
- Chapter 8: Intermediate Code Generation for interpreters
- Tree-walking interpreter design patterns
- Runtime environment management (symbol tables, activation records)
- **Relevance**: Foundation for AST-walking interpreter architecture

**[2] Ierusalimschy, de Figueiredo, Celes (2007)** - *"The Implementation of Lua 5.0"*
Journal of Universal Computer Science, 13(7): 842-860
- Register-based VM design for dynamic languages
- Efficient runtime type checking strategies
- Garbage collection integration with interpreter
- **Relevance**: Proven patterns for dynamic language runtime

**[3] Brunthaler (2010)** - *"Inline Caching Meets Quickening"*
ECOOP 2010: 429-451
- Adaptive optimization in interpreters
- Inline caching for dynamic dispatch
- Quickening techniques for bytecode
- **Relevance**: Performance optimization without JIT complexity

### 2. Runtime Verification & Bug Discovery

**[4] Elkarablieh, Marinov, Khurshid (2009)** - *"Efficient Solving of Structural Constraints"*
ISSTA 2009: 39-50
- Automated test generation for runtime verification
- Constraint-based bug finding
- Systematic exploration of execution paths
- **Relevance**: Systematic runtime bug discovery methodology

**[5] Cadar, Dunbar, Engler (2008)** - *"KLEE: Unassisted and Automatic Generation of High-Coverage Tests for Complex Systems Programs"*
OSDI 2008: 209-224
- Symbolic execution for runtime bug detection
- Automatic test case generation achieving >90% coverage
- Finding deep runtime bugs automatically
- **Relevance**: High-coverage testing via systematic execution (KLEE achieved 84.5% coverage on GNU COREUTILS)

**[6] Godefroid, Klarlund, Sen (2005)** - *"DART: Directed Automated Random Testing"*
PLDI 2005: 213-223
- Concolic testing (concrete + symbolic execution)
- Automatic input generation for runtime testing
- Found 38 bugs in production software
- **Relevance**: Practical runtime bug discovery via directed testing

### 3. Test-Driven Development & Quality Assurance

**[7] Beck (2002)** - *"Test-Driven Development: By Example"*
Addison-Wesley Professional
- RED-GREEN-REFACTOR cycle formalization
- Test-first development methodology
- Continuous integration practices
- **Relevance**: EXTREME TDD methodology foundation

**[8] Zeller, Hildebrandt (2002)** - *"Simplifying and Isolating Failure-Inducing Input"*
IEEE TSE 28(2): 183-200
- Delta debugging algorithm for test minimization
- Automatic fault localization
- Minimizing failure-inducing test cases
- **Relevance**: Already implemented in REPLIC-001, validates approach

### 4. Programming Language Runtime Systems

**[9] Würthinger, Wimmer, Wöß, et al. (2013)** - *"One VM to Rule Them All"*
Onward! 2013: 187-204
- Multi-language runtime on Graal VM
- Self-optimizing abstract syntax tree interpreters
- Truffle framework for language implementation
- **Relevance**: Modern interpreter architecture patterns

**[10] Bolz, Cuni, Fijalkowski, Rigo (2009)** - *"Tracing the Meta-Level: PyPy's Tracing JIT Compiler"*
ICOOOLPS 2009: 18-25
- Meta-tracing for interpreter optimization
- JIT compilation from interpreter traces
- Achieving 2-50x speedups over CPython
- **Relevance**: Interpreter performance optimization strategies

### 5. Static Analysis & Quality Metrics

**[11] Nagappan, Ball (2005)** - *"Use of Relative Code Churn Measures to Predict System Defect Density"*
ICSE 2005: 284-292
- Code churn as predictor of defects
- Already implemented in DISC-004
- Empirical validation on Windows Server
- **Relevance**: Quality metrics integration (PMAT)

**[12] McCabe (1976)** - *"A Complexity Measure"*
IEEE TSE 2(4): 308-320
- Cyclomatic complexity metric
- Predicting software maintainability
- Foundation for static analysis tools
- **Relevance**: PMAT quality gates (max complexity 20)

---

## Architecture Overview

### Interpreter Components

```
┌─────────────────────────────────────────────────────────┐
│                 RuchyRuchy Interpreter                  │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  1. Parser (reuse Ruchy compiler AST)                   │
│     - Parse .ruchy files to AST                         │
│     - Use ruchy check for validation                    │
│                                                         │
│  2. AST Walker (new implementation)                     │
│     - Tree-walking interpreter [Aho 2006]              │
│     - Runtime type checking                             │
│     - Dynamic dispatch                                  │
│                                                         │
│  3. Runtime Environment                                 │
│     - Symbol table (scoped)                             │
│     - Activation records (call stack)                   │
│     - Value representations (integers, strings, etc.)   │
│                                                         │
│  4. Bug Discovery Integration                           │
│     - Automatic bug filing (GITHUB-001)                 │
│     - Confidence scoring (REPORT-004)                   │
│     - Delta debugging (REPLIC-001/002)                  │
│                                                         │
│  5. Quality Integration                                 │
│     - PMAT quality gates                                │
│     - ruchydbg enforcement                              │
│     - TDG scoring                                       │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### Execution Model

**Tree-Walking Interpreter** [Aho 2006, Ierusalimschy 2007]:
- Direct AST interpretation (no bytecode compilation)
- Simplicity over performance (bug discovery > speed)
- Easy integration with existing bug discovery pipeline

**Runtime Type Checking** [Ierusalimschy 2007]:
- Dynamic type verification at runtime
- Detect type mismatches static analysis misses
- Report runtime type errors with confidence scores

**Activation Records** [Aho 2006]:
- Explicit call stack management
- Variable scoping (lexical + dynamic)
- Detect stack overflows, scope issues

---

## Test Corpus: ruchy-book Examples

### Coverage Matrix

| Chapter | Title | Examples | Runtime Tests |
|---------|-------|----------|---------------|
| Ch01 | Hello World | 5 | Basic I/O, string printing |
| Ch02 | Variables & Types | 15 | Type inference, mutations |
| Ch03 | Functions | 20 | Calls, recursion, closures |
| Ch04 | Practical Patterns | 12 | Iterators, error handling |
| Ch05 | Control Flow | 18 | If/else, loops, match |
| Ch06 | Data Structures | 25 | Vectors, hashmaps, structs |
| Ch10 | Input/Output | 8 | File I/O, stdin/stdout |
| Ch13 | Debugging & Tracing | 10 | Runtime introspection |
| Ch14 | Toolchain Mastery | 15 | Compilation, execution |
| Ch15 | Binary Compilation | 6 | AOT compilation testing |
| Ch16 | Testing & QA | 20 | Test framework integration |
| Ch17 | Error Handling | 12 | Exception semantics |
| Ch18 | DataFrames | 8 | Data processing |
| Ch19 | Structs & OOP | 15 | Object semantics |
| Ch20 | HTTP Server | 10 | Concurrency, I/O |
| Ch22 | Compiler Dev | 8 | Meta-programming |
| Ch23 | REPL | 5 | Interactive execution |
| **TOTAL** | **17 Chapters** | **~212 Examples** | **High Coverage** |

### Bug Discovery Potential

Based on **[5] KLEE (2008)**: Systematic execution achieved:
- **84.5%** line coverage on GNU COREUTILS
- **10 previously unknown bugs** in mature software
- **16 total bugs** found in production code

**Our Goal**: Execute 212+ examples to achieve:
- **>90% runtime path coverage**
- **Discover 50+ runtime bugs** in Ruchy implementation
- **Document 100+ edge cases** for Ruchy compiler team

---

## Methodology: EXTREME TDD + PMAT Quality

### Phase 1: Interpreter Infrastructure (4 weeks)

**Tickets**: INTERP-001 through INTERP-010

1. **INTERP-001**: AST Parser Integration (RED-GREEN-REFACTOR)
   - Parse ruchy-book examples to AST
   - Test: Parse all 212 examples without errors

2. **INTERP-002**: Value Representation System
   - Integer, String, Boolean, Vector, HashMap
   - Test: Create and inspect all value types

3. **INTERP-003**: Symbol Table & Scoping
   - Lexical scoping with nested scopes
   - Test: Variable shadowing, closures

4. **INTERP-004**: Expression Evaluator
   - Arithmetic, comparison, logical operations
   - Test: All operators from Ch02-Ch05

5. **INTERP-005**: Function Calls & Recursion
   - Activation records, argument passing
   - Test: Factorial, Fibonacci, mutual recursion

6. **INTERP-006**: Control Flow (if/match/loops)
   - Conditional execution, pattern matching
   - Test: All control flow examples from Ch05

7. **INTERP-007**: Data Structure Operations
   - Vector/HashMap access, mutation
   - Test: All examples from Ch06

8. **INTERP-008**: File I/O Integration
   - Read/write files, stdin/stdout
   - Test: All examples from Ch10

9. **INTERP-009**: Error Handling & Reporting
   - Runtime error detection, stack traces
   - Test: Intentional errors produce useful messages

10. **INTERP-010**: Bug Discovery Integration
    - Auto-file bugs via GITHUB-001
    - Test: Discover + file 5 known bugs

### Phase 2: Example Execution (8 weeks)

**Tickets**: INTERP-011 through INTERP-027 (one per chapter)

Execute ALL examples from each chapter:
- **RED**: Write failing test for chapter examples
- **GREEN**: Implement interpreter features to pass
- **REFACTOR**: Clean up, optimize
- **TOOL**: Validate with PMAT, ruchydbg
- **BUG**: File all discovered bugs via GitHub API

**Example Ticket Structure** (INTERP-011: Chapter 1):
```yaml
- id: INTERP-011
  title: "Execute All Chapter 1 Examples (Hello World)"
  priority: critical
  phase: RED-GREEN-REFACTOR-TOOL-BUG
  requirements:
    - Execute all 5 examples from ch01-02-hello-world-tdd.md
    - Verify output matches expected results
    - File bugs for any runtime failures
  tests:
    - test_ch01_example_01_hello_world
    - test_ch01_example_02_variables
    - test_ch01_example_03_functions
    - test_ch01_example_04_control_flow
    - test_ch01_example_05_data_structures
  acceptance:
    - All 5 examples execute without errors
    - Output matches book expectations
    - Any bugs filed on GitHub with confidence scores
  deliverables:
    - tests/interpreter/test_ch01_examples.rs (5 tests)
    - Bug reports filed (if any)
    - Chapter completion report
```

### Phase 3: Runtime Conformance (4 weeks)

**Tickets**: INTERP-028 through INTERP-035

1. **INTERP-028**: Property-Based Runtime Testing
   - Generate 10K random programs
   - Test: No crashes, all errors caught

2. **INTERP-029**: Fuzzing Integration
   - Grammar-based fuzzing (DISCOVERY-002B)
   - Test: 1M inputs, document all crashes

3. **INTERP-030**: Performance Profiling
   - Benchmark interpreter vs ruchy binary
   - Target: <100x slower than native

4. **INTERP-031**: Memory Safety Validation
   - Valgrind, AddressSanitizer integration
   - Test: Zero memory leaks

5. **INTERP-032**: Concurrency Testing (Ch20)
   - Multi-threaded execution
   - Test: Data races, deadlocks

6. **INTERP-033**: Bug Taxonomy & Analysis
   - Categorize all discovered bugs
   - Generate comprehensive report

7. **INTERP-034**: Ruchy Compiler Bug Filing
   - File all bugs at paiml/ruchy
   - Test: >50 bugs filed with reproduction

8. **INTERP-035**: Conformance Test Suite
   - Export test suite for Ruchy compiler
   - Test: Ruchy compiler passes 95%+

---

## Quality Gates (PMAT + ruchydbg)

### PMAT Integration [Nagappan 2005, McCabe 1976]

**Pre-commit Hooks** (MANDATORY):
```bash
# TDG Quality Enforcement
pmat tdg check-regression --baseline .pmat/baseline.json
pmat tdg check-quality --min-grade B+ --fail-on-violation

# Complexity Gates
- Max cyclomatic complexity: 20
- Max cognitive complexity: 15
- Max function length: 100 LOC

# Clippy Enforcement (QUALITY-011)
cargo clippy --no-default-features --lib -- -D warnings -A missing-docs
```

### ruchydbg Enforcement

**Debugger Integration** (MANDATORY):
```bash
# Every interpreter test MUST be debuggable
ruchydbg validate tests/interpreter/test_ch01_examples.rs

# Source map accuracy
- 1:1 line mapping: Rust → Ruchy source
- Backward stepping: time-travel debugging
- Performance: <0.1s per debug operation
```

### Test Coverage Requirements

**Minimum Coverage** (MANDATORY):
- Unit tests: 286+ (existing baseline)
- Integration tests: 212+ (one per example)
- Property tests: 10K+ cases (INTERP-028)
- Fuzz tests: 1M+ inputs (INTERP-029)
- Total: **~1.2M tests**

**Coverage Metrics**:
- Line coverage: >85%
- Branch coverage: >80%
- Runtime path coverage: >90%

---

## Bug Discovery Pipeline Integration

### Existing Infrastructure

**Already Implemented**:
- ✅ GITHUB-001: GitHub API Integration (auto-filing)
- ✅ GITHUB-002: Issue Linking & Deduplication
- ✅ REPLIC-001: Line-Based Delta Debugging
- ✅ REPLIC-002: AST-Based Delta Debugging
- ✅ REPORT-004: Confidence Scoring
- ✅ DISC-004: Code Churn Analysis
- ✅ VALID-007: Historical Bug Validation

### New Workflow

```
Interpreter Execution
    ↓
Runtime Error Detected [Cadar 2008]
    ↓
Delta Debugging (REPLIC-001) [Zeller 2002]
    ↓
Confidence Scoring (REPORT-004)
    ↓
Deduplication Check (GITHUB-002)
    ↓
Auto-File Bug (GITHUB-001)
    ↓
Validate Fix (VALID-007)
```

**Confidence Scoring** (REPORT-004):
- Reproducibility: Always (interpreter deterministic)
- Evidence: Complete (full execution trace)
- Root Cause: Clear (stack trace + example)
- **Confidence**: 0.9-1.0 (very high)

---

## Expected Outcomes

### Primary Deliverables

1. **Ruchy Interpreter** (src/interpreter/)
   - 5,000+ LOC Rust implementation
   - Tree-walking AST interpreter
   - Full ruchy-book example support

2. **Comprehensive Test Suite**
   - 212+ integration tests (one per example)
   - 10K+ property tests
   - 1M+ fuzz tests
   - Total: ~1.2M tests

3. **Bug Discovery Report**
   - 50+ runtime bugs discovered
   - Filed at paiml/ruchy via GitHub API
   - Categorized by severity, type
   - Confidence scores: 0.9+ (high quality)

4. **Conformance Test Suite**
   - Exportable test suite for Ruchy compiler
   - Baseline for future Ruchy releases
   - Continuous integration support

### Research Contributions

Based on **[5] KLEE** and **[6] DART** results:
- **Expected**: 50+ new bugs in production Ruchy compiler
- **Benchmark**: KLEE found 10 bugs in mature COREUTILS
- **Our Advantage**: 212 real-world examples (not synthetic)

**Publication Potential**:
- *"Systematic Runtime Bug Discovery via Comprehensive Example Execution"*
- *"Building Interpreters for Bug Discovery: A Case Study with Ruchy"*
- *"High-Confidence Bug Reporting via Deterministic Interpretation"*

---

## Roadmap Timeline

### Phase 1: Infrastructure (Weeks 1-4)
- **INTERP-001 to INTERP-010**: Core interpreter (10 tickets)
- **Milestone**: Interpreter can execute basic programs

### Phase 2: Example Execution (Weeks 5-12)
- **INTERP-011 to INTERP-027**: Chapter-by-chapter execution (17 tickets)
- **Milestone**: All 212 examples execute successfully

### Phase 3: Conformance (Weeks 13-16)
- **INTERP-028 to INTERP-035**: Property testing, fuzzing, bug analysis (8 tickets)
- **Milestone**: 50+ bugs discovered and filed

**Total**: 35 tickets, 16 weeks, ~1.2M tests

---

## Success Metrics

### Quantitative Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Examples Executed | 212/212 (100%) | Test pass rate |
| Bugs Discovered | 50+ | GitHub issues filed |
| Test Coverage | >85% line | cargo tarpaulin |
| Runtime Path Coverage | >90% | INTERP-029 fuzzing |
| Performance | <100x slower | Benchmarks vs native |
| Bug Confidence | >0.9 average | REPORT-004 scores |
| Bug Filing Rate | 100% auto-filed | GITHUB-001 integration |

### Qualitative Metrics

- **Bug Impact**: How many bugs are CRITICAL/HIGH severity?
- **Ruchy Compiler Fixes**: How many bugs get fixed upstream?
- **Test Suite Adoption**: Does Ruchy compiler adopt our conformance suite?
- **Community Value**: Do Ruchy users benefit from interpreter?

---

## Risk Mitigation

### Technical Risks

| Risk | Impact | Mitigation |
|------|--------|-----------|
| Ruchy syntax changes break interpreter | HIGH | Pin to specific Ruchy version (v3.149.0) |
| Performance too slow for large examples | MEDIUM | Focus on correctness, optimize later |
| Bugs unfixable in Ruchy compiler | LOW | Document as "known limitations" |
| Interpreter bugs mask Ruchy bugs | MEDIUM | Cross-validate with native execution |

### Schedule Risks

| Risk | Impact | Mitigation |
|------|--------|-----------|
| Interpreter takes longer than 4 weeks | HIGH | Start with subset (Ch01-Ch06 only) |
| Example execution blocked by bugs | MEDIUM | File bugs, continue with next chapter |
| Fuzzing finds too many bugs | LOW | Prioritize, file incrementally |

---

## Dependencies

### External Dependencies

- **Ruchy Compiler** (v3.149.0+): For AST parsing, validation
- **ruchy-book**: Source of 212+ examples
- **PMAT**: Quality gates, TDG scoring
- **ruchydbg**: Debugger enforcement

### Internal Dependencies

- **GITHUB-001/002**: Bug filing, deduplication (✅ complete)
- **REPLIC-001/002**: Delta debugging (✅ complete)
- **REPORT-004**: Confidence scoring (✅ complete)
- **DISC-004**: Code churn analysis (✅ complete)
- **VALID-007**: Bug validation (✅ complete)

---

## References

1. Aho, Lam, Sethi, Ullman (2006). *Compilers: Principles, Techniques, and Tools* (2nd Ed.). Pearson.

2. Ierusalimschy, R., de Figueiredo, L. H., & Celes, W. (2007). *The Implementation of Lua 5.0*. Journal of Universal Computer Science, 13(7), 842-860.

3. Brunthaler, S. (2010). *Inline Caching Meets Quickening*. ECOOP 2010, 429-451.

4. Elkarablieh, B., Marinov, D., & Khurshid, S. (2009). *Efficient Solving of Structural Constraints*. ISSTA 2009, 39-50.

5. Cadar, C., Dunbar, D., & Engler, D. (2008). *KLEE: Unassisted and Automatic Generation of High-Coverage Tests for Complex Systems Programs*. OSDI 2008, 209-224.

6. Godefroid, P., Klarlund, N., & Sen, K. (2005). *DART: Directed Automated Random Testing*. PLDI 2005, 213-223.

7. Beck, K. (2002). *Test-Driven Development: By Example*. Addison-Wesley Professional.

8. Zeller, A., & Hildebrandt, R. (2002). *Simplifying and Isolating Failure-Inducing Input*. IEEE TSE 28(2), 183-200.

9. Würthinger, T., Wimmer, C., Wöß, A., et al. (2013). *One VM to Rule Them All*. Onward! 2013, 187-204.

10. Bolz, C. F., Cuni, A., Fijalkowski, M., & Rigo, A. (2009). *Tracing the Meta-Level: PyPy's Tracing JIT Compiler*. ICOOOLPS 2009, 18-25.

11. Nagappan, N., & Ball, T. (2005). *Use of Relative Code Churn Measures to Predict System Defect Density*. ICSE 2005, 284-292.

12. McCabe, T. J. (1976). *A Complexity Measure*. IEEE TSE 2(4), 308-320.

---

## Appendix: Ticket Template

```yaml
- id: INTERP-XXX
  title: "[Component/Chapter] Description"
  priority: critical|high|medium|low
  status: pending|in_progress|completed
  phase: RED-GREEN-REFACTOR-TOOL-BUG
  requirements:
    - Requirement 1
    - Requirement 2
  tests:
    - test_name_1
    - test_name_2
  acceptance:
    - Acceptance criterion 1
    - Acceptance criterion 2
  deliverables:
    - File 1 (with LOC count)
    - File 2
  research_foundation:
    - Citation [X] - How it applies
  pmat_validation:
    - TDG score: >85 (B+)
    - Complexity: <20
    - Coverage: >80%
  ruchydbg_validation:
    - Source maps: 1:1 accuracy
    - Time-travel: working
    - Performance: <0.1s per operation
  bugs_discovered:
    - BUG-XXX: Description (severity, confidence)
```

---

**End of Specification**

**Next Steps**: Create roadmap tickets (INTERP-001 through INTERP-035) and begin RED phase for INTERP-001.
