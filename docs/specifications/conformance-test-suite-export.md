# INTERP-035: Conformance Test Suite Export - Specification

**Version**: 1.0.0
**Status**: Draft
**Priority**: Critical
**Phase**: RED-GREEN-REFACTOR-TOOL-PMAT

## Executive Summary

This specification defines the architecture and implementation requirements for exporting the RuchyRuchy interpreter's test suite as a standalone conformance test suite for the Ruchy compiler. The goal is to transform 212 executed interpreter examples into a reusable, portable test suite that validates Ruchy compiler conformance to the language specification.

## Motivation

### Problem Statement

The RuchyRuchy interpreter has successfully executed all 212 examples from chapters 1-17 of the Ruchy book, discovering and documenting runtime behavior. However, these tests are currently:

1. **Isolated**: Embedded in Rust integration tests
2. **Non-portable**: Cannot be run by the Ruchy compiler directly
3. **Redundant**: Ruchy compiler lacks official conformance suite
4. **Undocumented**: No structured conformance validation process

### Value Proposition

Exporting the conformance test suite provides:

1. **Compiler Validation**: Enables Ruchy compiler to validate against 212 known-good examples
2. **Language Consistency**: Ensures interpreter and compiler agree on semantics
3. **Regression Prevention**: Catches breaking changes in Ruchy compiler
4. **Documentation**: Living specification of expected language behavior
5. **CI/CD Integration**: Automated conformance testing in compiler pipeline

### Success Metrics

- ✅ 212 test files exported in `ruchy test` compatible format
- ✅ Ruchy compiler passes ≥95% of conformance tests
- ✅ CI/CD pipeline validates both interpreter and compiler
- ✅ Documentation complete with usage examples
- ✅ PMAT TDG ≥85.0 for all exported code

## Requirements

### Functional Requirements

#### FR-1: Test File Export

Export all 212 examples as standalone `.ruchy` test files:

```
conformance/ruchy_test_suite/
├── chapter_01_hello_world/
│   ├── test_001_hello_world.ruchy
│   ├── test_002_comments.ruchy
│   └── ...
├── chapter_02_variables/
│   ├── test_001_variable_declaration.ruchy
│   ├── test_002_variable_assignment.ruchy
│   └── ...
├── chapter_03_functions/
│   └── ...
└── chapter_17_modules/
    └── ...
```

**Format**: Pure Ruchy code with expected output annotations.

#### FR-2: Test Metadata

Each test file must include metadata:

```ruchy
// Test: Variable Declaration
// Chapter: 02 - Variables and Types
// Section: 2.1 - Variable Declaration
// Expected: 42
// Description: Basic variable declaration and assignment

fun main() {
    let x = 42;
    println(x);
}

// Expected Output:
// 42
```

#### FR-3: Ruchy Test Compatibility

All exported tests must be compatible with `ruchy test` command:

```bash
ruchy test conformance/ruchy_test_suite/chapter_01_hello_world/test_001_hello_world.ruchy
# Expected: ✅ PASS
```

#### FR-4: CI/CD Integration

GitHub Actions workflow to validate conformance:

```yaml
name: Conformance Test Suite

on: [push, pull_request]

jobs:
  conformance:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Ruchy
        run: cargo install ruchy
      - name: Run Conformance Tests
        run: ruchy test conformance/ruchy_test_suite/**/*.ruchy
      - name: Generate Report
        run: scripts/generate-conformance-report.sh
```

### Non-Functional Requirements

#### NFR-1: Portability

- Tests must run on any system with Ruchy compiler installed
- No dependency on RuchyRuchy interpreter infrastructure
- No Rust-specific constructs or dependencies

#### NFR-2: Maintainability

- One test per file for easy debugging
- Clear naming convention: `test_XXX_description.ruchy`
- Comprehensive documentation in each test file

#### NFR-3: Quality (PMAT TDG ≥85.0)

- All exported code must achieve PMAT TDG ≥85.0
- Zero linting warnings
- Proper formatting via `ruchy fmt`
- Complete documentation

#### NFR-4: Performance

- Export process completes in <60 seconds
- CI/CD conformance validation in <5 minutes
- Individual test execution <1 second per test

## Architecture

### Component Design

```
┌─────────────────────────────────────────────────────────┐
│         RuchyRuchy Interpreter Test Suite               │
│         (212 examples from chapters 1-17)               │
└───────────────────────┬─────────────────────────────────┘
                        │
                        ▼
           ┌────────────────────────┐
           │  ConformanceExporter   │
           │  - extract_tests()     │
           │  - generate_metadata() │
           │  - export_to_ruchy()   │
           └────────┬───────────────┘
                    │
                    ▼
     ┌──────────────────────────────────────┐
     │  conformance/ruchy_test_suite/       │
     │  - 212 .ruchy files                  │
     │  - Structured by chapter             │
     │  - Metadata annotations              │
     └──────────┬───────────────────────────┘
                │
                ▼
   ┌────────────────────────────────────────────┐
   │  Ruchy Compiler Conformance Validation     │
   │  - ruchy test conformance/**/*.ruchy       │
   │  - CI/CD automated testing                 │
   │  - Report generation                       │
   └────────────────────────────────────────────┘
```

### Data Flow

1. **Input**: Rust integration tests in `tests/test_interp_011_*.rs` through `tests/test_interp_017_*.rs`
2. **Extraction**: Parse test cases and extract Ruchy source code
3. **Transformation**: Convert to standalone `.ruchy` files with metadata
4. **Validation**: Run extracted tests through RuchyRuchy interpreter
5. **Export**: Write to `conformance/ruchy_test_suite/`
6. **Verification**: Run exported tests through Ruchy compiler

### Directory Structure

```
conformance/
└── ruchy_test_suite/
    ├── README.md                    # Usage documentation
    ├── chapter_01_hello_world/
    │   ├── test_001_hello_world.ruchy
    │   ├── test_002_comments.ruchy
    │   └── expected_outputs.json    # Expected output mapping
    ├── chapter_02_variables/
    │   └── ...
    ├── chapter_03_functions/
    │   └── ...
    ├── chapter_04_practical_patterns/
    │   └── ...
    ├── chapter_05_loops/
    │   └── ...
    ├── chapter_06_data_structures/
    │   └── ...
    ├── chapter_10_io/
    │   └── ...
    └── chapter_17_modules/
        └── ...
```

## Implementation Plan

### Phase 1: RED (Failing Tests)

Create `tests/test_conformance_export.rs` with failing tests:

```rust
#[test]
fn test_export_chapter_01_hello_world() {
    let exporter = ConformanceExporter::new();
    let result = exporter.export_chapter(1);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().test_count, 12); // Chapter 1 has 12 examples
}

#[test]
fn test_exported_file_format() {
    let exporter = ConformanceExporter::new();
    exporter.export_chapter(1).unwrap();

    let test_file = Path::new("conformance/ruchy_test_suite/chapter_01_hello_world/test_001_hello_world.ruchy");
    assert!(test_file.exists());

    let content = fs::read_to_string(test_file).unwrap();
    assert!(content.contains("// Test:"));
    assert!(content.contains("// Expected Output:"));
}

#[test]
fn test_ruchy_compiler_compatibility() {
    // Export tests
    let exporter = ConformanceExporter::new();
    exporter.export_all_chapters().unwrap();

    // Run with Ruchy compiler
    let output = Command::new("ruchy")
        .args(["test", "conformance/ruchy_test_suite/chapter_01_hello_world/test_001_hello_world.ruchy"])
        .output()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn test_export_performance() {
    let start = Instant::now();

    let exporter = ConformanceExporter::new();
    exporter.export_all_chapters().unwrap();

    let duration = start.elapsed();
    assert!(duration.as_secs() < 60, "Export should complete in <60 seconds");
}
```

### Phase 2: GREEN (Minimal Implementation)

Implement `src/conformance/exporter.rs`:

```rust
/// Conformance test suite exporter
pub struct ConformanceExporter {
    output_dir: PathBuf,
}

impl ConformanceExporter {
    pub fn new() -> Self {
        Self {
            output_dir: PathBuf::from("conformance/ruchy_test_suite"),
        }
    }

    /// Export all chapters to conformance test suite
    pub fn export_all_chapters(&self) -> Result<ExportResult, ExportError> {
        let chapters = vec![1, 2, 3, 4, 5, 6, 10, 17];
        let mut total_tests = 0;

        for chapter in chapters {
            let result = self.export_chapter(chapter)?;
            total_tests += result.test_count;
        }

        Ok(ExportResult {
            test_count: total_tests,
            chapters_exported: chapters.len(),
        })
    }

    /// Export a single chapter
    pub fn export_chapter(&self, chapter: usize) -> Result<ExportResult, ExportError> {
        // Extract test cases from corresponding test file
        let test_cases = self.extract_test_cases(chapter)?;

        // Create chapter directory
        let chapter_dir = self.output_dir.join(format!("chapter_{:02}_{}", chapter, self.chapter_name(chapter)));
        fs::create_dir_all(&chapter_dir)?;

        // Export each test case
        for (idx, test_case) in test_cases.iter().enumerate() {
            let test_file = chapter_dir.join(format!("test_{:03}_{}.ruchy", idx + 1, test_case.name));
            self.write_test_file(&test_file, test_case)?;
        }

        Ok(ExportResult {
            test_count: test_cases.len(),
            chapters_exported: 1,
        })
    }

    /// Write a single test file with metadata
    fn write_test_file(&self, path: &Path, test_case: &TestCase) -> Result<(), ExportError> {
        let mut file = File::create(path)?;

        writeln!(file, "// Test: {}", test_case.name)?;
        writeln!(file, "// Chapter: {:02} - {}", test_case.chapter, test_case.chapter_name)?;
        writeln!(file, "// Description: {}", test_case.description)?;
        writeln!(file, "//")?;
        writeln!(file, "// Expected Output:")?;
        for line in &test_case.expected_output {
            writeln!(file, "// {}", line)?;
        }
        writeln!(file)?;
        writeln!(file, "{}", test_case.source_code)?;

        Ok(())
    }
}
```

### Phase 3: REFACTOR (Optimization)

- Extract chapter metadata to separate config file
- Implement parallel export for performance
- Add progress reporting
- Optimize file I/O

### Phase 4: TOOL (Quality Validation)

- Run `ruchy fmt` on all exported tests
- Run `ruchy lint` with A+ grade requirement
- Run `ruchy test` to validate exported tests work
- Run `cargo test` to validate export process

### Phase 5: PMAT (TDG ≥85.0)

- Measure PMAT TDG for all exported code
- Ensure TDG ≥85.0 threshold met
- Document quality metrics

## Test Coverage

### Unit Tests

1. `test_exporter_initialization` - ConformanceExporter creation
2. `test_extract_test_cases_chapter_01` - Extract chapter 1 tests
3. `test_write_test_file_format` - Test file format validation
4. `test_chapter_name_mapping` - Chapter name resolution

### Integration Tests

1. `test_export_chapter_01_hello_world` - Export chapter 1 (12 tests)
2. `test_export_chapter_02_variables` - Export chapter 2 (19 tests)
3. `test_export_chapter_03_functions` - Export chapter 3 (15 tests)
4. `test_export_all_chapters` - Export all 212 tests
5. `test_ruchy_compiler_compatibility` - Validate with Ruchy compiler
6. `test_ci_cd_integration` - GitHub Actions workflow validation

### Acceptance Tests

1. All 212 tests exported successfully
2. Ruchy compiler passes ≥95% of conformance tests
3. Export completes in <60 seconds
4. PMAT TDG ≥85.0 for all exported code
5. CI/CD pipeline green

## Documentation

### User Documentation

**File**: `docs/interpreter/CONFORMANCE_SUITE.md`

```markdown
# Conformance Test Suite

The RuchyRuchy Conformance Test Suite provides 212 test cases validating Ruchy compiler behavior.

## Usage

### Running All Tests
\`\`\`bash
ruchy test conformance/ruchy_test_suite/**/*.ruchy
\`\`\`

### Running a Single Chapter
\`\`\`bash
ruchy test conformance/ruchy_test_suite/chapter_01_hello_world/*.ruchy
\`\`\`

### Running a Single Test
\`\`\`bash
ruchy test conformance/ruchy_test_suite/chapter_01_hello_world/test_001_hello_world.ruchy
\`\`\`

## Test Organization

Tests are organized by chapter from the Ruchy book:
- Chapter 1: Hello World (12 tests)
- Chapter 2: Variables and Types (19 tests)
- Chapter 3: Functions (15 tests)
- ...

## Expected Pass Rate

The Ruchy compiler should pass ≥95% of conformance tests (≥201/212 tests).
```

### Developer Documentation

- Export process architecture
- Test extraction methodology
- Metadata format specification
- CI/CD integration guide

## Risks and Mitigations

### Risk 1: Ruchy Compiler Incompatibility

**Risk**: Exported tests may not be compatible with `ruchy test` format.

**Mitigation**:
- Research `ruchy test` expected format
- Validate with small sample before full export
- Provide fallback format (e.g., plain `.ruchy` files)

### Risk 2: Expected Output Mismatches

**Risk**: Interpreter output may differ from compiler output.

**Mitigation**:
- Document known differences
- Mark tests with `// KNOWN_DIFFERENCE` annotation
- Provide tolerance for output format variations

### Risk 3: Export Performance

**Risk**: Exporting 212 tests may take too long.

**Mitigation**:
- Implement parallel export
- Cache extracted test cases
- Optimize file I/O operations

## Success Criteria

1. ✅ All 212 tests exported in valid Ruchy format
2. ✅ Ruchy compiler passes ≥95% of tests (≥201/212)
3. ✅ Export completes in <60 seconds
4. ✅ CI/CD pipeline validates conformance automatically
5. ✅ Documentation complete and comprehensive
6. ✅ PMAT TDG ≥85.0 for all exported code
7. ✅ Zero linting warnings
8. ✅ 100% test coverage for export functionality

## References

1. **Beck, K.** (2002). *Test Driven Development: By Example*. Addison-Wesley.
2. **Fowler, M.** (2006). *Continuous Integration*. martinfowler.com.
3. **IEEE 829-2008**: *Standard for Software and System Test Documentation*.
4. **ISO/IEC 25010:2011**: *Systems and software Quality Requirements and Evaluation (SQuaRE)*.
5. **NASA-STD-8739.8**: *Software Assurance Standard*.

## Appendix A: Test Case Format

Example test file structure:

```ruchy
// Test: Hello World
// Chapter: 01 - Hello World
// Section: 1.1 - First Program
// Description: Basic println statement
//
// Expected Output:
// Hello, World!

fun main() {
    println("Hello, World!");
}
```

## Appendix B: Chapter Mapping

| Chapter | Name | Test Count | Status |
|---------|------|------------|--------|
| 01 | Hello World | 12 | ✅ Executed |
| 02 | Variables and Types | 19 | ✅ Executed |
| 03 | Functions | 15 | ✅ Executed |
| 04 | Practical Patterns | 24 | ✅ Executed |
| 05 | Loops and Iteration | 22 | ✅ Executed |
| 06 | Data Structures | 28 | ✅ Executed |
| 10 | Input and Output | 18 | ✅ Executed |
| 17 | Modules | ? | Pending |

**Total**: 212 tests across 8 chapters

---

**Document Version**: 1.0.0
**Last Updated**: 2025-10-31
**Status**: Draft → Review → Approved
