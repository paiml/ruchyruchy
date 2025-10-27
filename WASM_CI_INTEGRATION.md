# WebAssembly CI/CD Integration Plan

## Overview

This document outlines the plan for integrating the WebAssembly compilation target into the main Ruchy CI/CD pipeline. This integration ensures that the WebAssembly target is automatically built, tested, and validated alongside the existing TypeScript and Rust targets.

## CI/CD Pipeline Updates

### GitHub Actions Workflow

Update the main `.github/workflows/ci.yml` file to include WebAssembly-specific steps:

```yaml
name: Ruchy CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Set up Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      
      - name: Install dependencies
        run: |
          cargo install wasm-bindgen-cli
          cargo install wasm-pack
          npm install -g serve
      
      - name: Build
        run: cargo build --verbose
      
      - name: Run main tests
        run: cargo test --verbose
      
      # WebAssembly-specific steps
      - name: Build WebAssembly target
        run: |
          cargo build --target wasm32-unknown-unknown
          wasm-bindgen --target web target/wasm32-unknown-unknown/debug/ruchy.wasm --out-dir wasm-build
      
      - name: Run WebAssembly tests
        run: |
          cd validation/wasm
          cargo test --target wasm32-unknown-unknown
      
      - name: Run browser tests
        run: |
          cd validation/wasm/browser-tests
          npm install
          npm test
      
      - name: Build examples
        run: |
          cd examples/wasm
          ./build-all.sh
      
      - name: Run WebAssembly integration tests
        run: cargo test --test wasm_integration

  property_testing:
    runs-on: ubuntu-latest
    steps:
      # ... (existing steps)
      
      # WebAssembly property testing
      - name: Run WebAssembly property tests
        run: |
          cd validation/wasm
          cargo test --test property_multi_target -- --ignored

  fuzz_testing:
    runs-on: ubuntu-latest
    steps:
      # ... (existing steps)
      
      # WebAssembly fuzz testing
      - name: Run WebAssembly fuzz tests
        run: |
          cd validation/wasm
          cargo test --test fuzz_multi_target -- --ignored

  performance_testing:
    runs-on: ubuntu-latest
    steps:
      # ... (existing steps)
      
      # WebAssembly performance testing
      - name: Run WebAssembly benchmarks
        run: |
          cd validation/wasm
          cargo bench --bench benchmark_multi_target
```

### Nightly Performance Testing

Add a nightly workflow for comprehensive WebAssembly performance testing:

```yaml
name: WebAssembly Nightly Performance

on:
  schedule:
    - cron: '0 2 * * *'  # Run at 2 AM UTC every day

jobs:
  performance_testing:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      # ... (setup steps)
      
      - name: Run comprehensive benchmarks
        run: |
          cd validation/wasm
          cargo bench --bench benchmark_multi_target -- --all
      
      - name: Compare with baseline
        run: |
          cd validation/wasm
          ./compare-benchmarks.sh
      
      - name: Notify on regression
        if: ${{ failure() }}
        uses: actions/github-script@v5
        with:
          script: |
            github.issues.create({
              owner: context.repo.owner,
              repo: context.repo.repo,
              title: 'WebAssembly performance regression detected',
              body: 'The nightly performance test detected a regression. See workflow run: ${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}'
            })
```

### Browser Testing Matrix

Add a workflow to test the WebAssembly target across different browsers:

```yaml
name: WebAssembly Browser Testing

on:
  push:
    branches: [ main ]
    paths:
      - 'bootstrap/stage3/wasm_**'
      - 'validation/wasm/**'
  pull_request:
    branches: [ main ]
    paths:
      - 'bootstrap/stage3/wasm_**'
      - 'validation/wasm/**'

jobs:
  browser_testing:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        browser: [chrome, firefox, edge]
    steps:
      - uses: actions/checkout@v3
      
      # ... (setup steps)
      
      - name: Build test app
        run: |
          cd validation/wasm/browser-tests
          npm install
          npm run build
      
      - name: Run browser tests
        run: |
          cd validation/wasm/browser-tests
          npm run test:${{ matrix.browser }}
```

## Test Infrastructure Updates

### Browser Test Environment

Create a browser testing environment for WebAssembly:

1. **Directory**: `validation/wasm/browser-tests`

2. **Test Setup**:
   ```javascript
   // browser-test-setup.js
   const puppeteer = require('puppeteer');
   const { Server } = require('http');
   const handler = require('serve-handler');
   
   async function setupBrowserTest() {
     // Start a local web server
     const server = new Server((req, res) => {
       return handler(req, res, { public: 'build' });
     });
     await new Promise(resolve => server.listen(3000, resolve));
     
     // Launch browser
     const browser = await puppeteer.launch({
       headless: true,
       args: ['--no-sandbox', '--disable-setuid-sandbox']
     });
     const page = await browser.newPage();
     
     return { server, browser, page };
   }
   
   module.exports = { setupBrowserTest };
   ```

3. **Test Runner**:
   ```javascript
   // browser-test-runner.js
   const { setupBrowserTest } = require('./browser-test-setup');
   
   async function runTests() {
     const { server, browser, page } = await setupBrowserTest();
     
     try {
       // Navigate to test page
       await page.goto('http://localhost:3000');
       
       // Wait for tests to complete
       await page.waitForSelector('#test-results', { timeout: 30000 });
       
       // Get test results
       const testResults = await page.evaluate(() => {
         return document.querySelector('#test-results').textContent;
       });
       
       // Parse and report results
       const results = JSON.parse(testResults);
       console.log(`Tests: ${results.total}, Passed: ${results.passed}, Failed: ${results.failed}`);
       
       if (results.failed > 0) {
         console.error('Test failures:');
         results.failures.forEach(failure => {
           console.error(`- ${failure.name}: ${failure.message}`);
         });
         process.exit(1);
       }
     } finally {
       await browser.close();
       server.close();
     }
   }
   
   runTests().catch(error => {
     console.error('Test runner error:', error);
     process.exit(1);
   });
   ```

4. **Package Configuration**:
   ```json
   {
     "name": "ruchy-wasm-browser-tests",
     "version": "1.0.0",
     "scripts": {
       "build": "webpack --mode production",
       "test": "node browser-test-runner.js",
       "test:chrome": "BROWSER=chrome node browser-test-runner.js",
       "test:firefox": "BROWSER=firefox node browser-test-runner.js",
       "test:edge": "BROWSER=edge node browser-test-runner.js"
     },
     "dependencies": {
       "puppeteer": "^19.0.0",
       "serve-handler": "^6.1.3",
       "webpack": "^5.74.0",
       "webpack-cli": "^4.10.0"
     }
   }
   ```

### Automated Example Testing

Create a script to build and test all examples:

```bash
#!/bin/bash
# examples/wasm/build-all.sh

set -euo pipefail

# Get the directory of the script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Find all example directories with a compile.sh script
EXAMPLE_DIRS=$(find "$SCRIPT_DIR" -type f -name "compile.sh" -exec dirname {} \;)

# Build each example
for DIR in $EXAMPLE_DIRS; do
  echo "Building example: $(basename "$DIR")"
  (cd "$DIR" && ./compile.sh)
done

echo "All examples built successfully!"
```

## Quality Gate Integration

### Update Quality Gates Script

Modify `scripts/quality-gates.sh` to include WebAssembly-specific checks:

```bash
#!/bin/bash

set -euo pipefail

# Run standard quality checks
cargo clippy --all-targets
cargo test
cargo fmt --check

# WebAssembly-specific checks
echo "Running WebAssembly quality checks..."

# Check WebAssembly target builds
cargo build --target wasm32-unknown-unknown

# Run WebAssembly tests
cd validation/wasm
cargo test --target wasm32-unknown-unknown

# Check browser compatibility
cd browser-tests
npm test

# Verify examples
cd ../../examples/wasm
./build-all.sh

echo "All quality checks passed!"
```

### Add Pre-Commit Hook

Update `.git/hooks/pre-commit` to include WebAssembly checks:

```bash
#!/bin/bash

set -euo pipefail

# Check if any WebAssembly-related files were changed
if git diff --cached --name-only | grep -q -E 'wasm|WebAssembly'; then
  echo "WebAssembly files changed, running specific checks..."
  
  # Run WebAssembly-specific checks
  cargo check --target wasm32-unknown-unknown
  
  # Check WebAssembly examples
  if git diff --cached --name-only | grep -q "examples/wasm"; then
    cd examples/wasm
    ./build-all.sh
  fi
fi

# Continue with standard checks
cargo clippy --all-targets
cargo fmt --check
```

## Continuous Deployment Updates

### WebAssembly Playground Deployment

Add a workflow to deploy the WebAssembly playground:

```yaml
name: Deploy WebAssembly Playground

on:
  push:
    branches: [ main ]
    paths:
      - 'playground/**'
      - 'bootstrap/stage3/wasm_**'

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      # ... (setup steps)
      
      - name: Build playground
        run: |
          cd playground
          npm install
          npm run build
      
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./playground/build
          destination_dir: playground
```

### Documentation Deployment

Update the documentation deployment workflow to include WebAssembly documentation:

```yaml
name: Deploy Documentation

on:
  push:
    branches: [ main ]
    paths:
      - 'docs/**'
      - 'examples/**'

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      # ... (setup steps)
      
      - name: Build documentation
        run: |
          cd docs
          npm install
          npm run build
      
      # Build WebAssembly examples for documentation
      - name: Build WebAssembly examples
        run: |
          cd examples/wasm
          ./build-all.sh
          cp -r . ../../docs/build/examples/wasm
      
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./docs/build
```

## Monitoring and Alerting

### Performance Monitoring

Add a script to track WebAssembly performance metrics:

```bash
#!/bin/bash
# scripts/monitor-wasm-performance.sh

set -euo pipefail

# Run benchmarks
cd validation/wasm
cargo bench --bench benchmark_multi_target > benchmark_results.txt

# Extract key metrics
SMALL_FUNCTION_TIME=$(grep "Small Functions" benchmark_results.txt | awk '{print $5}')
MEDIUM_PROJECT_TIME=$(grep "Medium Project" benchmark_results.txt | awk '{print $5}')
LARGE_PROJECT_TIME=$(grep "Large Project" benchmark_results.txt | awk '{print $5}')

# Compare with thresholds
if (( $(echo "$SMALL_FUNCTION_TIME > 50" | bc -l) )); then
  echo "⚠️ Small function compilation time exceeds threshold: ${SMALL_FUNCTION_TIME}ms (max 50ms)"
  EXIT_CODE=1
fi

if (( $(echo "$MEDIUM_PROJECT_TIME > 200" | bc -l) )); then
  echo "⚠️ Medium project compilation time exceeds threshold: ${MEDIUM_PROJECT_TIME}ms (max 200ms)"
  EXIT_CODE=1
fi

if (( $(echo "$LARGE_PROJECT_TIME > 500" | bc -l) )); then
  echo "⚠️ Large project compilation time exceeds threshold: ${LARGE_PROJECT_TIME}ms (max 500ms)"
  EXIT_CODE=1
fi

# Store metrics for trending
mkdir -p .metrics
echo "$(date +%Y-%m-%d,%H:%M:%S),${SMALL_FUNCTION_TIME},${MEDIUM_PROJECT_TIME},${LARGE_PROJECT_TIME}" >> .metrics/wasm_performance.csv

exit ${EXIT_CODE:-0}
```

### Regression Detection

Add a script to detect WebAssembly performance regressions:

```bash
#!/bin/bash
# scripts/detect-wasm-regressions.sh

set -euo pipefail

# Load historical data
METRICS_FILE=.metrics/wasm_performance.csv

# Calculate baseline (average of last 5 runs)
BASELINE_SMALL=$(tail -n 5 $METRICS_FILE | cut -d',' -f2 | awk '{ sum += $1 } END { print sum / 5 }')
BASELINE_MEDIUM=$(tail -n 5 $METRICS_FILE | cut -d',' -f3 | awk '{ sum += $1 } END { print sum / 5 }')
BASELINE_LARGE=$(tail -n 5 $METRICS_FILE | cut -d',' -f4 | awk '{ sum += $1 } END { print sum / 5 }')

# Get current metrics
CURRENT_RUN=$(tail -n 1 $METRICS_FILE)
CURRENT_SMALL=$(echo $CURRENT_RUN | cut -d',' -f2)
CURRENT_MEDIUM=$(echo $CURRENT_RUN | cut -d',' -f3)
CURRENT_LARGE=$(echo $CURRENT_RUN | cut -d',' -f4)

# Check for regressions (>10% increase)
SMALL_THRESHOLD=$(echo "$BASELINE_SMALL * 1.1" | bc -l)
MEDIUM_THRESHOLD=$(echo "$BASELINE_MEDIUM * 1.1" | bc -l)
LARGE_THRESHOLD=$(echo "$BASELINE_LARGE * 1.1" | bc -l)

if (( $(echo "$CURRENT_SMALL > $SMALL_THRESHOLD" | bc -l) )); then
  echo "⚠️ Small function performance regression: ${CURRENT_SMALL}ms vs baseline ${BASELINE_SMALL}ms"
  EXIT_CODE=1
fi

if (( $(echo "$CURRENT_MEDIUM > $MEDIUM_THRESHOLD" | bc -l) )); then
  echo "⚠️ Medium project performance regression: ${CURRENT_MEDIUM}ms vs baseline ${BASELINE_MEDIUM}ms"
  EXIT_CODE=1
fi

if (( $(echo "$CURRENT_LARGE > $LARGE_THRESHOLD" | bc -l) )); then
  echo "⚠️ Large project performance regression: ${CURRENT_LARGE}ms vs baseline ${BASELINE_LARGE}ms"
  EXIT_CODE=1
fi

exit ${EXIT_CODE:-0}
```

## Release Automation

### Version Update Script

Create a script to update all version references:

```bash
#!/bin/bash
# scripts/update-versions.sh

set -euo pipefail

# Get the new version from argument
if [ $# -ne 1 ]; then
  echo "Usage: $0 <new_version>"
  echo "Example: $0 1.25.0"
  exit 1
fi

NEW_VERSION=$1

# Update version in Cargo.toml
sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml

# Update version in package.json files
find . -name "package.json" -type f -exec sed -i "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" {} \;

# Update version in documentation
sed -i "s/v[0-9]\+\.[0-9]\+\.[0-9]\+/v$NEW_VERSION/g" docs/src/version.md
sed -i "s/v[0-9]\+\.[0-9]\+\.[0-9]\+/v$NEW_VERSION/g" README.md

# Update WebAssembly-specific files
sed -i "s/v[0-9]\+\.[0-9]\+\.[0-9]\+/v$NEW_VERSION/g" docs/guides/WASM_INTEGRATION_GUIDE.md
sed -i "s/v[0-9]\+\.[0-9]\+\.[0-9]\+/v$NEW_VERSION/g" WASM_RELEASE_PLAN.md

echo "Version updated to $NEW_VERSION in all files"
```

### Release Workflow

Add a workflow for automated release creation:

```yaml
name: Create Release

on:
  workflow_dispatch:
    inputs:
      version:
        description: 'Version number (e.g., 1.25.0)'
        required: true

jobs:
  create_release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      # Update version numbers
      - name: Update version numbers
        run: ./scripts/update-versions.sh ${{ github.event.inputs.version }}
      
      # Build for all targets
      - name: Build all targets
        run: |
          cargo build --release
          cargo build --target wasm32-unknown-unknown --release
      
      # Run tests
      - name: Run tests
        run: cargo test --release
      
      # Create GitHub release
      - name: Create Release
        id: create_release
        uses: actions/create-release@v1
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tag_name: v${{ github.event.inputs.version }}
          release_name: Ruchy v${{ github.event.inputs.version }}
          body_path: CHANGELOG.md
          draft: true
          prerelease: false
      
      # Upload binaries
      - name: Upload binaries
        uses: actions/upload-release-asset@v1
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          upload_url: ${{ steps.create_release.outputs.upload_url }}
          asset_path: ./target/release/ruchy
          asset_name: ruchy-${{ github.event.inputs.version }}-linux-x64
          asset_content_type: application/octet-stream
      
      # Upload WebAssembly bundle
      - name: Package WebAssembly bundle
        run: |
          mkdir -p wasm-bundle
          cp target/wasm32-unknown-unknown/release/ruchy.wasm wasm-bundle/
          cp -r examples/wasm wasm-bundle/examples
          cp docs/guides/WASM_INTEGRATION_GUIDE.md wasm-bundle/
          tar czf ruchy-wasm-${{ github.event.inputs.version }}.tar.gz wasm-bundle
      
      - name: Upload WebAssembly bundle
        uses: actions/upload-release-asset@v1
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          upload_url: ${{ steps.create_release.outputs.upload_url }}
          asset_path: ./ruchy-wasm-${{ github.event.inputs.version }}.tar.gz
          asset_name: ruchy-wasm-${{ github.event.inputs.version }}.tar.gz
          asset_content_type: application/gzip
```

## Integration with Development Workflow

### PR Templates

Update `.github/PULL_REQUEST_TEMPLATE.md` to include WebAssembly considerations:

```markdown
## PR Description

<!-- Describe your changes -->

## Type of Change

- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## WebAssembly Impact

- [ ] No impact on WebAssembly target
- [ ] Minor impact (passes all tests)
- [ ] Significant impact (requires WebAssembly-specific testing)
- [ ] WebAssembly-focused change

## Testing Performed

- [ ] Unit tests
- [ ] Integration tests
- [ ] WebAssembly tests
- [ ] Browser tests
- [ ] Manual testing

## Checklist

- [ ] I have added tests that prove my fix or feature works
- [ ] I have updated documentation accordingly
- [ ] My changes generate no new warnings
- [ ] All WebAssembly examples still build and work correctly
```

### Issue Templates

Add a WebAssembly-specific issue template (`.github/ISSUE_TEMPLATE/wasm_issue.md`):

```markdown
---
name: WebAssembly Issue
about: Report an issue with the WebAssembly target
title: '[WASM] '
labels: webassembly
assignees: ''
---

## Issue Description

<!-- A clear description of the issue -->

## Environment

- Ruchy version:
- Browser/Node.js version:
- Operating system:

## Reproduction Steps

1. 
2. 
3. 

## Expected Behavior

<!-- What you expected to happen -->

## Actual Behavior

<!-- What actually happened -->

## Additional Context

<!-- Any other information or screenshots -->

## Minimal Reproduction Code

```ruchy
// Add minimal code to reproduce the issue
```
```

### Developer Documentation

Update `docs/CONTRIBUTING.md` with WebAssembly-specific guidance:

```markdown
## WebAssembly Development

When making changes that affect the WebAssembly target, follow these guidelines:

1. **Testing**: Always run the WebAssembly-specific tests:
   ```bash
   cd validation/wasm
   cargo test --target wasm32-unknown-unknown
   ```

2. **Browser Testing**: Test in at least Chrome and Firefox:
   ```bash
   cd validation/wasm/browser-tests
   npm test
   ```

3. **Performance**: Run benchmarks to ensure no performance regressions:
   ```bash
   cd validation/wasm
   cargo bench --bench benchmark_multi_target
   ```

4. **Examples**: Verify that all examples still work:
   ```bash
   cd examples/wasm
   ./build-all.sh
   ```

5. **Documentation**: Update WebAssembly documentation if your changes affect user-facing functionality.

6. **Integration**: Consider impacts on all three targets (WebAssembly, TypeScript, Rust) when making changes to shared components.
```

## Implementation Plan

To implement this CI/CD integration, follow these steps:

1. **Infrastructure Setup** (Week 1)
   - Set up WebAssembly build environment in CI
   - Create browser testing infrastructure
   - Add performance monitoring scripts

2. **Workflow Creation** (Week 1-2)
   - Create GitHub Actions workflows
   - Set up nightly performance testing
   - Create release automation workflow

3. **Developer Tools** (Week 2)
   - Update pre-commit hooks
   - Create PR and issue templates
   - Update contribution guidelines

4. **Testing** (Week 2-3)
   - Verify all workflows with test PRs
   - Validate performance monitoring
   - Test release process

5. **Documentation** (Week 3)
   - Update CI/CD documentation
   - Create developer guides
   - Document testing procedures

6. **Training** (Week 3-4)
   - Train core team on new processes
   - Provide guidance for contributors
   - Create internal knowledge base

## Conclusion

This CI/CD integration plan ensures that the WebAssembly compilation target is fully incorporated into the Ruchy development workflow. By automating testing, validation, and release processes, we can maintain high quality and reliability for the WebAssembly target alongside the existing TypeScript and Rust targets.

The plan covers all aspects of integration, from GitHub Actions workflows to developer tools and documentation. This comprehensive approach will make it easy for the team to maintain and enhance the WebAssembly target while ensuring that it continues to meet quality standards.