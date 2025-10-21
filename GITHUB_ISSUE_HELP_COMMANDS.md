# Bug Report: Help Output Shows Unimplemented Commands

**To be filed at**: https://github.com/paiml/ruchy/issues

## Bug Report: `ruchy --help` Shows Non-Existent Commands

**Ruchy Version**: v3.106.0
**Project**: Main Ruchy Compiler
**Reporter**: RuchyRuchy Development (via Claude Code)

### Reproduction Steps
1. Run `ruchy --help`
2. Observe commands listed in help output
3. Try to run `ruchy publish --help` or `ruchy add --help`
4. Commands appear to work but don't actually exist

### Minimal Reproduction
```bash
$ ruchy --help | grep -i package
  add              Add a package dependency
  publish          Publish a package to the registry

$ ruchy publish --help
# Shows help output with --registry, --dry-run, etc.

$ ruchy add --help
# Shows help output with --version, --dev, --registry, etc.
```

### Expected Behavior
- Help output should only show **implemented** commands
- OR: Commands should be marked as "Coming Soon" or "Experimental"
- OR: Commands should actually be implemented if shown in help

### Actual Behavior
- `ruchy add` and `ruchy publish` appear in help output
- These commands show detailed help text
- Commands reference `https://ruchy.dev/registry` (which may not exist)
- **These appear to be placeholder/unimplemented features**

### Impact
**Severity**: Medium - Confusing for users

- Users expect advertised commands to work
- Leads to confusion about Ruchy's package management capabilities
- RuchyRuchy project attempted to use these commands based on help output
- Currently blocking: Need to know the actual deployment mechanism

### Context
While developing RuchyRuchy debugging toolkit, we wanted to dogfood Ruchy's package management instead of using crates.io. The help output suggested Ruchy has `add` and `publish` commands, but these don't appear to be implemented.

### Questions for Maintainers
1. Are `ruchy add` and `ruchy publish` planned features?
2. Should we use crates.io for now?
3. Is there a Ruchy package registry roadmap?
4. Should help output be updated to remove these commands?

### Suggested Fix
**Option 1**: Remove from help output until implemented
**Option 2**: Add "(Coming Soon)" marker in help text
**Option 3**: Implement the commands (larger effort)

### Environment
- OS: Linux 6.8.0-85-generic
- Ruchy install: cargo install ruchy
- Shell: bash

### Workaround
Using crates.io (cargo publish) for RuchyRuchy debugging toolkit distribution.

---

**Filed by**: RuchyRuchy Development Team
**Date**: October 21, 2025
**Related Work**: DOCS-017 (Crates.io Package Preparation)
