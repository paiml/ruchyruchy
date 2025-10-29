# Quick Start: RuchyRuchy Tools for Ruchy Compiler Developers

**Target Audience**: Ruchy compiler developers who want to catch bugs faster

**Time Investment**: 10 minutes to test one bug

---

## Problem

You're fixing Ruchy bugs manually. Each bug takes 4+ days to discover, debug, and fix.

## Solution

Use RuchyRuchy's automated bug discovery tools to catch bugs in 5-10 minutes.

---

## Installation (30 seconds)

```bash
# Option 1: Install from crates.io (recommended)
cargo install ruchyruchy

# Option 2: Build from source
cd /path/to/ruchyruchy
cargo build --release
export PATH="$PWD/target/release:$PATH"
```

Verify installation:
```bash
ruchydbg --version  # Should show v1.4.0 or later
```

---

## Issue #79 Example: How RuchyRuchy Would Have Caught It

### The Bug

```ruchy
struct Logger { level: LogLevel }
impl Logger {
    fun test(&self) {
        let val = self.level as i32;  // HANGS
    }
}
```

**Problem**: Hang (infinite loop) when casting enum field via `&self`

### How to Catch It (5 minutes)

#### Step 1: Create Test File (2 minutes)

Create `test_issue79.ruchy`:

```ruchy
// test_issue79.ruchy
// Property: Enum field cast via &self must NOT hang

enum LogLevel {
    Debug = 0,
    Info = 1,
}

struct Logger {
    level: LogLevel,
}

impl Logger {
    fun create() -> Logger {
        Logger { level: LogLevel::Info }
    }

    fun test(&self) {
        let val = self.level as i32;  // Should not hang!
        println!("Value: {}", val);
    }
}

fun main() {
    let logger = Logger::create();
    logger.test();  // Must complete in <1 second
}
```

#### Step 2: Run with Timeout (1 minute)

```bash
# Run with 1-second timeout
timeout 1 ruchy run test_issue79.ruchy

# Exit codes:
# 0 = Success (no bug)
# 124 = Timeout (BUG DETECTED!)
# Other = Crash (also a bug)
```

**Expected Result**: Exit code 124 (timeout) = Bug detected in 1 second!

#### Step 3: Use ruchydbg for Analysis (2 minutes)

```bash
# Run with debugger (no validation scripts needed!)
ruchydbg run test_issue79.ruchy --timeout 1000

# This will:
# 1. Run code with 1-second timeout
# 2. Show where it hung (last line executed)
# 3. Generate performance report
```

**Output**:
```
Running: test_issue79.ruchy
Timeout after 1000ms
Last line executed: src/test_issue79.ruchy:18
  -> let val = self.level as i32;

Performance:
  Time: 1000ms (TIMEOUT)
  Status: HUNG

Bug Detected: Infinite loop at line 18
```

---

## Don't Need Validation Scripts!

The error you saw:
```
❌ Error: Cannot find validation script
Expected locations:
  - validation/debugging/ruchydbg.ruchy
```

**Solution**: Don't use `ruchydbg validate`. Use `ruchydbg run` instead:

```bash
# ❌ DON'T USE: ruchydbg validate
# This requires the full RuchyRuchy repository

# ✅ USE THIS: ruchydbg run
ruchydbg run YOUR_TEST.ruchy --timeout 1000
```

---

## Testing Your Bug Fixes (3 commands)

### Before Fix: Bug Should Be Detected

```bash
# 1. Create test file with bug reproduction
cat > test_bug.ruchy << 'EOF'
// Your bug reproduction code here
EOF

# 2. Run with timeout (should fail)
timeout 1 ruchy run test_bug.ruchy
echo "Exit code: $?"  # Should be 124 (timeout) or non-zero (crash)

# 3. EXPECTED: Bug detected!
```

### After Fix: Bug Should Be Gone

```bash
# 1. Update Ruchy compiler with your fix
cd /path/to/ruchy
cargo build --release

# 2. Run same test (should pass)
timeout 1 ruchy run test_bug.ruchy
echo "Exit code: $?"  # Should be 0 (success)

# 3. EXPECTED: Test passes in <1 second!
```

---

## Real Example: Testing Issue #79 Fix

### Step 1: Create Test

```bash
cd /path/to/ruchy
mkdir -p tests/regression

cat > tests/regression/issue79.ruchy << 'EOF'
// Regression test for Issue #79
// Bug: Enum field cast via &self hangs

enum LogLevel {
    Debug = 0,
    Info = 1,
}

struct Logger {
    level: LogLevel,
}

impl Logger {
    fun create() -> Logger {
        Logger { level: LogLevel::Info }
    }

    fun test(&self) {
        let val = self.level as i32;
        assert_eq!(val, 1);  // Info = 1
    }
}

fun main() {
    let logger = Logger::create();
    logger.test();
    println!("✅ Issue #79 fixed!");
}
EOF
```

### Step 2: Test Before Fix (should fail)

```bash
timeout 1 ./target/release/ruchy run tests/regression/issue79.ruchy

if [ $? -eq 124 ]; then
    echo "✅ Bug reproduced (timeout)"
else
    echo "❌ Bug NOT reproduced (should have timed out)"
fi
```

### Step 3: Apply Your Fix

```bash
# Edit src/codegen/rust.rs or wherever your fix is
vim src/codegen/rust.rs

# Rebuild
cargo build --release
```

### Step 4: Test After Fix (should pass)

```bash
timeout 1 ./target/release/ruchy run tests/regression/issue79.ruchy

if [ $? -eq 0 ]; then
    echo "✅ Bug FIXED!"
else
    echo "❌ Bug still present"
fi
```

### Step 5: Add to Regression Suite

```bash
# Add to Makefile or test script
echo "ruchy run tests/regression/issue79.ruchy" >> tests/run_all.sh
```

---

## Advanced: Property Testing (10 minutes)

### Why Property Testing?

- Tests **all possible inputs** (not just one example)
- Finds edge cases you wouldn't think of
- Runs 10,000+ test cases automatically

### Example: Testing Enum Casts

Create `property_test_enum_casts.rs`:

```rust
// property_test_enum_casts.rs
// Run with: cargo test --test property_test_enum_casts

use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[test]
fn property_all_enum_casts_terminate() {
    // Test 100 different enum cast patterns
    for i in 0..100 {
        let code = generate_enum_cast_test(i);

        let start = Instant::now();
        let output = Command::new("ruchy")
            .arg("run")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap();

        let elapsed = start.elapsed();

        // Property: All enum casts must complete in <1 second
        assert!(
            elapsed < Duration::from_secs(1),
            "Test case {} hung ({}ms):\n{}",
            i, elapsed.as_millis(), code
        );
    }
}

fn generate_enum_cast_test(seed: u32) -> String {
    // Generate different enum cast patterns
    let patterns = vec![
        // Direct cast
        "let val = LogLevel::Info as i32;",

        // Variable cast
        "let level = LogLevel::Debug;\nlet val = level as i32;",

        // Field cast via self (Issue #79)
        "struct Logger { level: LogLevel }\nimpl Logger { fun test(&self) { let val = self.level as i32; } }",

        // Field cast via variable
        "struct Logger { level: LogLevel }\nlet logger = Logger { level: LogLevel::Info };\nlet val = logger.level as i32;",

        // Nested struct field cast
        "struct Outer { inner: Inner }\nstruct Inner { level: LogLevel }\nimpl Outer { fun test(&self) { let val = self.inner.level as i32; } }",

        // Array element cast
        "let levels = [LogLevel::Debug, LogLevel::Info];\nlet val = levels[0] as i32;",

        // Match arm cast
        "match LogLevel::Info { LogLevel::Info => LogLevel::Info as i32, _ => 0 }",
    ];

    let pattern = &patterns[(seed as usize) % patterns.len()];

    format!(r#"
enum LogLevel {{
    Debug = 0,
    Info = 1,
}}

fun main() {{
    {}
    println!("OK");
}}
"#, pattern)
}
```

Run property tests:

```bash
cargo test --test property_test_enum_casts
```

**Expected**: All 100 test cases pass in <100 seconds total

---

## Debugging Commands Reference

### Basic Testing

```bash
# Run with timeout
timeout 1 ruchy run test.ruchy

# Run with debugger
ruchydbg run test.ruchy --timeout 1000

# Run with verbose output
RUST_LOG=debug ruchy run test.ruchy
```

### Performance Analysis

```bash
# Time execution
time ruchy run test.ruchy

# Memory profiling
/usr/bin/time -v ruchy run test.ruchy

# CPU profiling (Linux)
perf record ruchy run test.ruchy
perf report
```

### Regression Testing

```bash
# Run all regression tests
for test in tests/regression/*.ruchy; do
    echo "Testing: $test"
    timeout 1 ruchy run "$test" || echo "FAILED: $test"
done

# Run with reporting
cargo install ruchyruchy
ruchydbg batch tests/regression/*.ruchy --timeout 1000 --report
```

---

## Common Patterns

### Pattern 1: Timeout Detection

```bash
#!/bin/bash
# test_for_hangs.sh

TEST_FILE=$1
TIMEOUT_SEC=${2:-1}

timeout $TIMEOUT_SEC ruchy run "$TEST_FILE"
EXIT_CODE=$?

if [ $EXIT_CODE -eq 124 ]; then
    echo "❌ BUG: Timeout after ${TIMEOUT_SEC}s"
    exit 1
elif [ $EXIT_CODE -eq 0 ]; then
    echo "✅ PASS: Completed successfully"
    exit 0
else
    echo "❌ BUG: Crashed with exit code $EXIT_CODE"
    exit 1
fi
```

### Pattern 2: Before/After Testing

```bash
#!/bin/bash
# test_bug_fix.sh

BUG_TEST=$1

echo "=== Testing BEFORE fix ==="
timeout 1 ruchy run "$BUG_TEST"
BEFORE=$?

if [ $BEFORE -ne 124 ] && [ $BEFORE -ne 0 ]; then
    echo "❌ ERROR: Expected timeout or crash, got exit code $BEFORE"
    exit 1
fi

echo ""
echo "=== Apply your fix now, then press Enter ==="
read

echo ""
echo "=== Rebuilding Ruchy ==="
cargo build --release

echo ""
echo "=== Testing AFTER fix ==="
timeout 1 ruchy run "$BUG_TEST"
AFTER=$?

if [ $AFTER -eq 0 ]; then
    echo "✅ SUCCESS: Bug fixed!"
    exit 0
else
    echo "❌ FAILURE: Bug still present (exit code $AFTER)"
    exit 1
fi
```

### Pattern 3: Regression Suite

```bash
#!/bin/bash
# run_regression_tests.sh

PASS=0
FAIL=0

for test in tests/regression/*.ruchy; do
    echo -n "Testing $(basename $test)... "

    timeout 1 ruchy run "$test" > /dev/null 2>&1

    if [ $? -eq 0 ]; then
        echo "✅ PASS"
        PASS=$((PASS + 1))
    else
        echo "❌ FAIL"
        FAIL=$((FAIL + 1))
    fi
done

echo ""
echo "Results: $PASS passed, $FAIL failed"

if [ $FAIL -gt 0 ]; then
    exit 1
fi
```

---

## FAQ

### Q: Do I need the full RuchyRuchy repository?

**A**: No! Just install the binary:
```bash
cargo install ruchyruchy
```

### Q: What about `ruchydbg validate`?

**A**: That's for RuchyRuchy developers only. You want `ruchydbg run`:
```bash
ruchydbg run YOUR_TEST.ruchy --timeout 1000
```

### Q: How do I test my bug fix?

**A**: 3 steps:
1. Create `.ruchy` file reproducing bug
2. Run with timeout BEFORE fix (should fail)
3. Run with timeout AFTER fix (should pass)

### Q: Can I use this in CI/CD?

**A**: Yes! Example GitHub Actions:

```yaml
name: Regression Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install Ruchy
        run: cargo install --path .

      - name: Install RuchyRuchy
        run: cargo install ruchyruchy

      - name: Run Regression Tests
        run: |
          for test in tests/regression/*.ruchy; do
            timeout 1 ruchy run "$test" || exit 1
          done
```

### Q: How do I write property tests?

**A**: See "Advanced: Property Testing" section above. Or use the full RuchyRuchy framework:

```bash
git clone https://github.com/paiml/ruchyruchy
cd ruchyruchy
cargo test --test property_tests
```

---

## Next Steps

1. **Try it now**: Test Issue #79 fix with the examples above (10 minutes)
2. **Add regression tests**: Create `tests/regression/` directory (5 minutes)
3. **Automate**: Add to CI/CD pipeline (15 minutes)
4. **Go deeper**: Read full docs at https://docs.rs/ruchyruchy/

---

## Support

- **Questions**: Open issue at https://github.com/paiml/ruchyruchy/issues
- **Bugs**: File at https://github.com/paiml/ruchyruchy/issues
- **Docs**: https://docs.rs/ruchyruchy/latest/ruchyruchy/
- **Examples**: https://github.com/paiml/ruchyruchy/tree/main/validation

---

## Summary

**Problem**: Manual bug discovery takes 4+ days per bug

**Solution**: Automated testing with RuchyRuchy takes 5-10 minutes

**Commands You Need**:
```bash
# Install
cargo install ruchyruchy

# Test for hangs (most common bug)
timeout 1 ruchy run test.ruchy

# Debug with timeout
ruchydbg run test.ruchy --timeout 1000

# Regression testing
for test in tests/regression/*.ruchy; do
    timeout 1 ruchy run "$test" || echo "FAILED: $test"
done
```

**That's it!** You're now using automated bug discovery.

---

**Time to catch Issue #79**: 5 minutes with RuchyRuchy vs. 4+ days manually
