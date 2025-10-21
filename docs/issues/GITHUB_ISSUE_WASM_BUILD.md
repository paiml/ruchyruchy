# WASM Build Failing: Feature Gate Issues

## Issue Summary

**Component**: WASM compilation
**Severity**: MEDIUM (workaround available)
**Impact**: Cannot build Ruchy for WASM target

## Problem Description

The WASM build is failing due to non-WASM-compatible code not being properly feature-gated. This prevents compiling Ruchy to WebAssembly.

## Build Error

```bash
$ wasm-pack build --target web
   Compiling ruchy v3.99.2
error[E0282]: type annotations needed
  --> src/bench/http.rs:137:X
   |
   | [HTTP-related code attempting type inference]
   |
   = note: cannot infer type for type parameter `T`

error: aborting due to previous error
```

## Root Cause

The codebase has non-WASM-compatible code (HTTP clients, benchmarking, etc.) that isn't properly behind `#[cfg(not(target_arch = "wasm32"))]` guards.

### Specific Issues

1. **HTTP Code**: `src/bench/http.rs:137` - Type inference fails
2. **Dependencies**: Non-WASM crates (reqwest, mio, etc.) not feature-gated
3. **Benchmarks**: Benchmark code trying to compile for WASM

### Affected Dependencies

These should be behind WASM feature gates:
- `reqwest` - HTTP client (not WASM-compatible)
- `mio` - Async I/O (not WASM-compatible)
- Benchmark-related dependencies

## Context

**What Works**:
- ✅ WASM REPL evaluation fix is complete (commit 7cfd31dd)
- ✅ Code changes are ready
- ✅ Tests passing (11/11, plus 600 property test iterations)

**What's Blocked**:
- ❌ WASM compilation fails
- ❌ Cannot build `.wasm` binary
- ❌ Cannot deploy WASM version

## Recommended Fixes

### Option 1: Feature Gate Non-WASM Code (BEST)

Add `#[cfg]` guards around non-WASM code:

```rust
// src/bench/http.rs
#[cfg(not(target_arch = "wasm32"))]
mod http {
    // All HTTP code here
}

// Cargo.toml
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
reqwest = "0.x"
mio = "0.x"
```

### Option 2: Separate Feature Flags (FLEXIBLE)

Create feature flags for optional components:

```toml
# Cargo.toml
[features]
default = ["http", "benchmarks"]
http = ["dep:reqwest"]
benchmarks = ["dep:criterion"]

[dependencies]
reqwest = { version = "0.x", optional = true }
criterion = { version = "0.x", optional = true }
```

Then build WASM without these features:
```bash
wasm-pack build --target web --no-default-features
```

### Option 3: Use Published WASM (TEMPORARY)

For immediate deployment:
```bash
# Publish new version with REPL fix
cargo publish

# Users install from crates.io (pre-built WASM)
cargo install ruchy
```

## Detailed Error Analysis

### HTTP Type Inference Issue

The type inference error in `src/bench/http.rs:137` suggests:
- Generic type `T` cannot be inferred
- Likely in async HTTP request handling
- WASM doesn't support this anyway

**Solution**: Gate entire HTTP module behind `#[cfg(not(target_arch = "wasm32"))]`

### Dependency Issues

Non-WASM dependencies being compiled:
- **reqwest**: Requires OS networking stack
- **mio**: Requires OS async I/O primitives
- **tokio** (possibly): May need WASM-specific features

**Solution**: Make these dependencies conditional:
```toml
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
reqwest = "0.x"
mio = "0.x"
```

## Workaround for Users

Until fixed, users can:

1. **Use published version** from crates.io (pre-built WASM)
2. **Use native binary** instead of WASM
3. **Manually patch** by adding feature gates

## Impact

**Medium Priority**:
- Native builds work fine ✅
- REPL fix is complete ✅
- Only affects WASM deployment ⚠️

## Testing Checklist

When fixing, ensure:
- [ ] `cargo build` works (native)
- [ ] `cargo build --target wasm32-unknown-unknown` works (WASM)
- [ ] `wasm-pack build --target web` works (WASM packaging)
- [ ] Native tests pass: `cargo test`
- [ ] WASM tests pass: `wasm-pack test --node`
- [ ] Feature flags work: `cargo build --no-default-features`

## Example Fix

### Before (Broken)
```rust
// src/bench/http.rs
use reqwest;  // ❌ Always compiled, even for WASM

pub fn benchmark_http() {
    // HTTP code
}
```

### After (Fixed)
```rust
// src/bench/http.rs
#[cfg(not(target_arch = "wasm32"))]
use reqwest;  // ✅ Only for non-WASM

#[cfg(not(target_arch = "wasm32"))]
pub fn benchmark_http() {
    // HTTP code
}

#[cfg(target_arch = "wasm32")]
pub fn benchmark_http() {
    // No-op or WASM-compatible alternative
}
```

## Related Work

**Completed**:
- ✅ Commit 7cfd31dd: WASM REPL evaluation fix
- ✅ 11/11 tests passing
- ✅ 600 property test iterations passing

**Blocked**:
- ❌ WASM build and deployment

## Environment

- **Ruchy Version**: v3.99.2
- **Rust**: Latest stable
- **wasm-pack**: Latest
- **Target**: `wasm32-unknown-unknown`

## Priority

**MEDIUM** - Native builds work, WASM deployment blocked but has workarounds.

---

**Reporter**: RuchyRuchy Bootstrap Compiler Project
**Date**: October 20, 2025
**Related Commit**: 7cfd31dd (WASM REPL fix)
