# Debugging Toolkit

**Status**: 🚧 In Progress - Vertical Slice 1 (Weeks 1-12)
**Goal**: Minimal Viable Time-Travel Debugger
**Specification**: [ruchyruchy-debugging-tools-spec.md](../../../docs/specifications/ruchyruchy-debugging-tools-spec.md)

## Overview

The RuchyRuchy Debugging Toolkit is a world-class debugging infrastructure built on modern computer science research and NASA-level engineering standards. The toolkit features:

- **Symbiotic Compiler-Debugger Architecture**: Embedded self-hosted compiler for maximum semantic awareness
- **Time-Travel Debugging**: Record-replay engine for backward stepping
- **Formally-Verified Correctness**: Mathematical proofs via Coq for critical algorithms
- **Extreme TDD Methodology**: RED-GREEN-REFACTOR-VERIFY with mutation/fuzz/property testing
- **Developer Experience Validation**: Usability testing with real developers

## Vertical Slice Approach

Following the Toyota Way principle of continuous learning, the debugging toolkit is built in **vertical value slices** rather than horizontal phases. Each slice delivers a complete, end-to-end experience of increasing capability:

### Vertical Slice 1: Minimal Viable Time-Travel Debugger (Weeks 1-12)

**Goal**: Prove time-travel debugging is feasible, deliver most exciting feature first, create walking skeleton.

**Features**:
- DEBUG-001: Minimal Source Maps (line-number mapping only)
- DEBUG-008-MINIMAL: Basic Record-Replay Engine (in-memory, <1000 steps)
- DEBUG-003-MINIMAL: Basic DAP Server (5 commands only: launch, break, continue, stepForward, stepBackward)

**Value Proposition**: Developers can experience backward stepping within first quarter, generating enthusiasm and early feedback.

**Risk Mitigation**: Tests most complex feature (record-replay) early, validates core architecture.

**Demo Experience (End of Week 12)**:
```bash
$ ruchydbg my_program.ruchy
> break main:10      # Set breakpoint
> run                # Start execution
> step               # Step forward
> step               # Step forward
> back               # Step BACKWARD! (Time-travel!)
> back               # Step backward again
> print my_var       # Inspect variable at this historical point
```

**Acceptance Criteria**:
- ✅ Time-travel debugger works end-to-end
- ✅ Can debug simple programs (<100 LOC) with backward stepping
- ✅ Proves feasibility of record-replay architecture
- ✅ Generates developer enthusiasm and feedback
- ✅ All Tier 2 quality gates passing

## Quality Standards

### Tiered Quality Gates

**Tier 1: Pre-Commit** (<1 second feedback)
- Syntax validation (`ruchy check`)
- Lint (A+ grade, `ruchy lint`)
- Unit tests for changed code (`ruchy test --fast`)

**Tier 2: Pre-Merge/PR** (5-10 minute feedback)
- All unit and integration tests
- PMAT TDG score (≥85)
- Incremental mutation testing
- Used for Vertical Slice 1

**Tier 3: Nightly Build** (2-4 hour feedback)
- 100% mutation score
- Exhaustive fuzz testing (10K+ inputs)
- Exhaustive property testing (10K+ cases)
- Formal verification (Coq proofs)
- Used for production releases

### Developer Experience Validation

Every feature includes DevEx validation:

**Cognitive Walkthroughs** (during RED phase):
- Mock UI before implementation
- Verify users can discover functionality without documentation

**Usability Testing** (during VERIFY phase):
- 5 developers matching target personas
- Task completion rate >80%
- User satisfaction >4/5

**Personas**:
- Systems Programmer (Rust/C++ background)
- Data Scientist (Python background)
- Application Developer (JS/TS background)

## Implementation Progress

### Completed

- **DEBUG-001: Source Map Generation** (Weeks 1-3)
  - Status: GREEN Phase Complete ✅
  - Tests: 20 tests (100% passing)
  - Property Tests: 150 cases (100 roundtrip + 50 monotonicity)
  - Implementation: 1:1 line mapping, minimal serialization
  - File: `validation/debugging/test_source_maps.ruchy` (628 lines)
  - Next: REFACTOR Phase (deferred to post-MVP)

### In Progress

- **DEBUG-008-MINIMAL: Basic Record-Replay Engine** (Weeks 5-8)
  - Status: RED Phase Complete ✅
  - Tests: 20 tests (14 failing, 6 accidentally passing)
  - Property Tests: 99 cases (50 roundtrip + 49 monotonicity)
  - Scope: In-memory logging, <1000 steps, naive replay
  - File: `validation/debugging/test_record_replay.ruchy` (620+ lines)
  - Next: GREEN Phase - Implement recording/replay engine

### Planned

- DEBUG-003-MINIMAL: Basic DAP Server (Weeks 9-12)

## References

- [Debugging Tools Specification](../../../docs/specifications/ruchyruchy-debugging-tools-spec.md)
- [Vertical Slice 1 Roadmap](../../../docs/specifications/ruchyruchy-debugging-tools-spec.md#vertical-slice-1-minimal-viable-time-travel-debugger-weeks-1-12)
- [Quality Assurance Framework](../../../docs/specifications/ruchyruchy-debugging-tools-spec.md#6-quality-assurance-framework)
