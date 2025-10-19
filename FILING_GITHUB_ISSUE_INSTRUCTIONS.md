# Instructions for Filing GitHub Issue

## Issue Details

**Repository**: https://github.com/paiml/ruchy/issues

**Issue File**: `GITHUB_ISSUE_VARIABLE_COLLISION.md`

**Issue Type**: Bug Report

**Severity**: HIGH - Type safety violation

## Steps to File

### Option 1: Manual Filing (Recommended)

1. **Navigate to GitHub Issues**:
   ```bash
   # Open in browser
   open https://github.com/paiml/ruchy/issues/new
   ```

2. **Copy Issue Content**:
   ```bash
   cat GITHUB_ISSUE_VARIABLE_COLLISION.md
   ```

3. **Fill in GitHub Issue Form**:
   - **Title**: Variable name collision in nested function calls with tuple unpacking causes type corruption
   - **Labels**: `bug`, `runtime`, `type-safety`, `high-severity`
   - **Body**: Paste the entire content from `GITHUB_ISSUE_VARIABLE_COLLISION.md`

4. **Attach Files** (if possible):
   - Minimal reproduction: `bug_variable_collision.ruchy` (from the markdown code block)
   - Reference: Link to BOUNDARIES.md section

5. **Submit Issue**

### Option 2: Using GitHub CLI

If `gh` CLI is installed:

```bash
# Create issue from file
gh issue create \
  --repo paiml/ruchy \
  --title "Variable name collision in nested function calls with tuple unpacking causes type corruption" \
  --label bug,runtime,type-safety,high-severity \
  --body-file GITHUB_ISSUE_VARIABLE_COLLISION.md
```

### Option 3: Create Issue Later

The issue content is saved in `GITHUB_ISSUE_VARIABLE_COLLISION.md` and can be filed at any time.

## After Filing

1. **Update BOUNDARIES.md** with the GitHub issue number:
   ```markdown
   **GitHub Issue**: https://github.com/paiml/ruchy/issues/[NUMBER]
   ```

2. **Update INTEGRATION.md** with issue reference

3. **Commit the update**:
   ```bash
   git add BOUNDARIES.md INTEGRATION.md
   git commit -m "DOCS: Add GitHub issue reference for variable collision bug"
   git push
   ```

## Validation

After filing, verify the issue is visible at:
```
https://github.com/paiml/ruchy/issues/[NUMBER]
```

## Expected Response

The Ruchy team will likely:
1. Confirm the bug with the minimal reproduction
2. Investigate the root cause in the runtime
3. Implement a fix in a future version (v3.97.0+)
4. Add regression tests to prevent recurrence
5. Close the issue when fixed

## Our Workaround

Until fixed, we continue using the workaround (renaming variables) which has been validated with 5000+ test cases.

---

**Note**: This issue filing is part of the Bug Discovery Protocol for the RuchyRuchy bootstrap compiler project. We follow a systematic approach: discover, reproduce, analyze, workaround, document, report, and continue.
