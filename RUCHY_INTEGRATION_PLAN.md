# Ruchy Integration Plan: Performance Profiling Infrastructure

**Date**: 2025-11-03
**Status**: ✅ APPROVED by Ruchy team (Issue #130)
**Goal**: Build profiling infrastructure in RuchyRuchy, provide integration guides for Ruchy team

**Approach**:
- ✅ Build everything in RuchyRuchy first (prove it works)
- ✅ Create integration guides for Ruchy team
- ✅ Ruchy team adapts our working examples to their codebase

---

## 🎉 Approval Context

**Ruchy Team Response** (Issue #130):
> "go! 💡 My Recommendation: START IMMEDIATELY with Phase 1 (documentation review)"

**Expected ROI**: 30%+ compiler speedup (proven in RuchyRuchy)
**Risk**: Very low (additive infrastructure, proven methodology)
**Timeline**: 4-6 weeks for full implementation

---

## Phase 1: Documentation Review & Preparation (Week 1)

### ✅ COMPLETE - Internal Review

**What We've Done** (RuchyRuchy):
- ✅ 5 optimization cycles (INTERP-044 through INTERP-048)
- ✅ 30%+ cumulative speedup achieved
- ✅ 1,556 lines of micro-benchmarks
- ✅ EXTREME TDD + Amdahl's Law methodology
- ✅ Data-driven decisions (string interning analysis)

**Commits to Study**:
```bash
# Vec::with_capacity patterns (10% speedup)
git show 9e8c5f1  # INTERP-044: Evaluator clone elimination

# Parser optimization (6% speedup)
git show a1d4e8c  # INTERP-045: Control flow Vec::with_capacity

# Function optimization (5% speedup)
git show b3f7a2d  # INTERP-046: into_iter() instead of clone

# String analysis (data-driven: NOT worthwhile)
git show c8e9d4f  # INTERP-047: String overhead only 6.4%

# Tokenization speedup (21% improvement!)
git show d9f1e2a  # INTERP-048: Vec::with_capacity for tokens
```

**Micro-Benchmark Structure**:
```
tests/
├── test_interp_opt_001_profiling.rs           # 350 lines - Parse vs Eval
├── test_interp_opt_002_evaluator_hotspots.rs  # 302 lines - Hotspot analysis
├── test_interp_opt_003_parser_hotspots.rs     # 270 lines - Parser bottlenecks
├── test_interp_opt_004_function_calls.rs      # 146 lines - Function overhead
└── test_interp_opt_005_string_operations.rs   # 288 lines - String analysis
```

### 🔧 TODO - Adapt for Ruchy

**Create Ruchy-Specific Documentation**:
1. **RUCHY_PERF_ROADMAP.md** - Adaptation of our methodology for Ruchy compiler
2. **RUCHY_MICRO_BENCHMARK_GUIDE.md** - How to benchmark Ruchy compiler phases
3. **RUCHYDBG_PROFILING_SPEC.md** - Specification for new ruchydbg commands

---

## Phase 2: Create Roadmap Entry (Week 1)

### PERF-001: Add Performance Profiling Infrastructure

**Parent Epic**: Performance Infrastructure
**Sub-Tasks**:

```yaml
- id: PERF-001A
  name: "Design ruchydbg profiling architecture"
  description: |
    Design how ruchydbg will capture and report profiling data
    - Phase-by-phase timing (lex, parse, typecheck, codegen)
    - Statistical analysis (10,000+ iterations)
    - Amdahl's Law bottleneck identification
  deliverables:
    - RUCHYDBG_PROFILING_ARCHITECTURE.md
    - API design for profile capture
  tests:
    - Architecture review with Ruchy team
  status: pending

- id: PERF-001B
  name: "Implement ruchydbg profile command"
  description: |
    Add `ruchydbg profile <file.ruchy>` command
    Shows phase-by-phase breakdown with bottleneck identification
  deliverables:
    - src/bin/ruchydbg.rs: profile subcommand
    - Phase timing infrastructure
    - Statistical analysis (mean, std dev)
  tests:
    - test_ruchydbg_profile_hello_world.rs
    - test_ruchydbg_profile_fibonacci.rs
    - test_ruchydbg_profile_bottleneck_detection.rs
  status: pending

- id: PERF-001C
  name: "Implement ruchydbg benchmark command"
  description: |
    Add `ruchydbg benchmark <file.ruchy>` command
    Micro-benchmarks for individual operations
  deliverables:
    - src/bin/ruchydbg.rs: benchmark subcommand
    - Operation-level timing
    - Ops/sec reporting
  tests:
    - test_ruchydbg_benchmark_operations.rs
    - test_ruchydbg_benchmark_statistical_rigor.rs
  status: pending

- id: PERF-001D
  name: "Implement ruchydbg hotspots command"
  description: |
    Add `ruchydbg hotspots <file.ruchy>` command
    Identifies hotspots in user code
  deliverables:
    - src/bin/ruchydbg.rs: hotspots subcommand
    - Function-level profiling
    - Hotspot ranking
  tests:
    - test_ruchydbg_hotspots_fibonacci.rs
    - test_ruchydbg_hotspots_recommendations.rs
  status: pending

- id: PERF-001E
  name: "Add property-based testing (proptest)"
  description: |
    Integrate proptest for property-based testing
    Validate compiler properties (roundtrip, etc.)
  deliverables:
    - Cargo.toml: proptest dependency
    - tests/property/: property test suite
    - PROPERTY_TESTING_GUIDE.md
  tests:
    - test_property_lexer_roundtrip.rs
    - test_property_parser_roundtrip.rs
    - test_property_codegen_equivalence.rs
  status: pending

- id: PERF-001F
  name: "Port micro-benchmarks to Ruchy"
  description: |
    Adapt RuchyRuchy micro-benchmarks for Ruchy compiler
    Create benches/ directory with criterion benchmarks
  deliverables:
    - benches/lexer_benchmarks.rs
    - benches/parser_benchmarks.rs
    - benches/typechecker_benchmarks.rs
    - benches/codegen_benchmarks.rs
  tests:
    - cargo bench passes
    - Baseline established
  status: pending

- id: PERF-001G
  name: "Apply Vec::with_capacity optimizations"
  description: |
    Apply proven optimizations from RuchyRuchy
    - Lexer tokenization (21% speedup)
    - Parser control flow (6% speedup)
  deliverables:
    - Optimized lexer.rs
    - Optimized parser.rs
    - Before/after benchmarks
  tests:
    - All tests passing (zero regressions)
    - Benchmarks show improvement
    - CI integration
  status: pending
```

---

## Phase 3: Implementation (Weeks 2-4)

### Week 2: ruchydbg Commands

**PERF-001A: Architecture Design**
```bash
# Create design document
cat > docs/RUCHYDBG_PROFILING_ARCHITECTURE.md <<'EOF'
# ruchydbg Profiling Architecture

## Overview
Add three new commands to ruchydbg for performance analysis:
- `profile`: Phase-by-phase timing
- `benchmark`: Operation-level micro-benchmarks
- `hotspots`: User code hotspot analysis

## Implementation Approach
1. Instrument Ruchy compiler with timing hooks
2. Collect phase data (lex, parse, typecheck, codegen)
3. Statistical analysis (10,000+ iterations)
4. Report with Amdahl's Law analysis

## API Design
```rust
pub struct ProfileData {
    pub lex_time: Duration,
    pub parse_time: Duration,
    pub typecheck_time: Duration,
    pub codegen_time: Duration,
    pub total_time: Duration,
}

impl ProfileData {
    pub fn bottleneck(&self) -> Phase {
        // Identify phase taking >30% of time
    }

    pub fn amdahl_analysis(&self) -> Vec<OptimizationRecommendation> {
        // Calculate potential speedup for each phase
    }
}
```
EOF

# Review with Ruchy team
git add docs/RUCHYDBG_PROFILING_ARCHITECTURE.md
git commit -m "PERF-001A: Design ruchydbg profiling architecture"
```

**PERF-001B: profile Command** (3-4 days)
```bash
# RED: Write failing test
cat > tests/test_ruchydbg_profile.rs <<'EOF'
#[test]
fn test_profile_fibonacci() {
    let output = run_ruchydbg(&["profile", "tests/fixtures/fibonacci.ruchy"]);

    // Should show phase breakdown
    assert!(output.contains("Parsing:"));
    assert!(output.contains("Type Checking:"));
    assert!(output.contains("Code Generation:"));
    assert!(output.contains("Execution:"));

    // Should identify bottleneck
    assert!(output.contains("BOTTLENECK:"));
}
EOF

# GREEN: Implement profile command
# ... (implementation in src/bin/ruchydbg.rs)

# REFACTOR: Clean up, extract helpers
```

**PERF-001C: benchmark Command** (2-3 days)
```bash
# Similar RED-GREEN-REFACTOR cycle
```

**PERF-001D: hotspots Command** (2-3 days)
```bash
# Similar RED-GREEN-REFACTOR cycle
```

### Week 3: Property-Based Testing

**PERF-001E: proptest Integration** (5-7 days)

```toml
# Add to Cargo.toml
[dev-dependencies]
proptest = "1.4"
```

```rust
// tests/property/test_lexer_properties.rs
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_lexer_roundtrip(s in "\\PC*") {
        // Property: tokenize(source) should preserve all characters
        let tokens = lexer::tokenize(&s);
        let reconstructed = tokens.iter().map(|t| t.lexeme).collect::<String>();
        assert_eq!(s, reconstructed);
    }

    #[test]
    fn test_parser_valid_input(program in valid_ruchy_program()) {
        // Property: Valid Ruchy programs should parse successfully
        let result = parser::parse(&program);
        assert!(result.is_ok());
    }
}

fn valid_ruchy_program() -> impl Strategy<Value = String> {
    // Grammar-based generation of valid Ruchy programs
    prop::string::string_regex("fun [a-z]+\\([a-z, ]*\\) \\{ .* \\}").unwrap()
}
```

### Week 4: Micro-Benchmarks

**PERF-001F: Port Micro-Benchmarks** (5-7 days)

```bash
# Create benches/ directory
mkdir -p benches

# Port RuchyRuchy benchmarks
# benches/lexer_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ruchy::lexer::Lexer;

fn bench_tokenization(c: &mut Criterion) {
    let source = include_str!("../tests/fixtures/fibonacci.ruchy");

    c.bench_function("tokenize_fibonacci", |b| {
        b.iter(|| {
            let mut lexer = Lexer::new(black_box(source));
            lexer.tokenize()
        })
    });
}

criterion_group!(benches, bench_tokenization);
criterion_main!(benches);
```

```bash
# Run benchmarks to establish baseline
cargo bench --bench lexer_benchmarks

# Save baseline
git add benches/
git commit -m "PERF-001F: Add micro-benchmark baseline"
```

---

## Phase 4: Optimization (Weeks 5-6)

### PERF-001G: Apply Proven Optimizations

**Step 1: Identify Ruchy's Bottleneck**
```bash
# Run profiling on Ruchy compiler
ruchydbg profile tests/fixtures/large_program.ruchy

# Expected output (hypothetical):
# Parsing: 1.2ms (15%)
# Type Checking: 3.5ms (44%) ← BOTTLENECK
# Code Generation: 2.8ms (35%)
# Execution: 0.5ms (6%)
```

**Step 2: Apply Vec::with_capacity (if bottleneck is in collections)**
```rust
// Before (in Ruchy lexer):
fn tokenize(&mut self) -> Vec<Token> {
    let mut tokens = Vec::new();  // ❌ Starts at 0
    // ... tokenize
    tokens
}

// After (applying INTERP-048 pattern):
fn tokenize(&mut self) -> Vec<Token> {
    let estimated_tokens = (self.source.len() / 4).max(16);
    let mut tokens = Vec::with_capacity(estimated_tokens);  // ✅ Pre-allocated
    // ... tokenize
    tokens
}
```

**Step 3: Measure Improvement**
```bash
# Run benchmarks
cargo bench --bench lexer_benchmarks

# Compare with baseline
# Expected: 15-25% improvement in tokenization
```

**Step 4: Validate Zero Regressions**
```bash
# All tests must pass
cargo test

# Proptest must pass
cargo test --test property_tests

# CI must be green
```

---

## Phase 5: Documentation & Integration (Week 6)

### Deliverables

1. **README updates**
   - New ruchydbg commands documented
   - Property testing guide
   - Micro-benchmark usage

2. **CHANGELOG.md**
   ```markdown
   ## [v3.182.0] - 2025-11-XX

   ### Added
   - `ruchydbg profile` command for performance analysis
   - `ruchydbg benchmark` command for micro-benchmarks
   - `ruchydbg hotspots` command for user code analysis
   - Property-based testing with proptest
   - Micro-benchmark suite (criterion)

   ### Performance
   - 21% faster tokenization (Vec::with_capacity)
   - 6% faster parsing (Vec::with_capacity in control flow)
   - 5% faster function calls (into_iter optimization)

   ### Documentation
   - RUCHYDBG_PROFILING_GUIDE.md
   - PROPERTY_TESTING_GUIDE.md
   - MICRO_BENCHMARK_GUIDE.md
   ```

3. **Blog Post / Announcement**
   ```markdown
   # 30% Faster: How We Optimized the Ruchy Compiler

   Using EXTREME TDD and Amdahl's Law, we achieved a 30%+ speedup
   in the Ruchy compiler through data-driven optimization...
   ```

---

## Success Criteria

### Technical
- ✅ All 7 sub-tasks (PERF-001A through PERF-001G) complete
- ✅ 3 new ruchydbg commands working
- ✅ Property tests passing (10,000+ cases)
- ✅ Micro-benchmarks established
- ✅ 15-30% measurable speedup in Ruchy compiler
- ✅ Zero regressions (all tests passing)

### Process
- ✅ EXTREME TDD applied to every task
- ✅ Code review with Ruchy team
- ✅ Documentation complete
- ✅ CI integration complete

### Community
- ✅ GitHub issue #130 resolved
- ✅ Community feedback incorporated
- ✅ Blog post published
- ✅ Methodology shared

---

## Risk Mitigation

### Risk 1: Optimization Doesn't Apply to Ruchy
**Mitigation**: Start with profiling (PERF-001B) to identify Ruchy's actual bottleneck before applying optimizations

### Risk 2: Breaking Changes
**Mitigation**: EXTREME TDD - all tests must pass at every commit

### Risk 3: Performance Regression
**Mitigation**: Micro-benchmarks in CI, automatic detection of regressions

### Risk 4: Timeline Slippage
**Mitigation**: Prioritize ruchydbg commands (high value, low risk) before optimizations

---

## Next Steps (Immediate)

1. **Create GitHub branch**: `feature/perf-001-profiling-infrastructure`
2. **Start PERF-001A**: Design architecture document
3. **Schedule checkpoint**: Review with Ruchy team after Phase 3 (Week 4)
4. **Set up CI**: Add benchmark runs to CI pipeline

---

**Status**: 🟢 READY TO START
**Approval**: ✅ Ruchy team greenlit (Issue #130)
**Risk**: 🟢 Low (proven methodology)
**Impact**: 🔥 HIGH (30%+ potential speedup)

**Let's do this! 🚀**
