# eBPF Development Setup Guide

**Purpose**: Setup instructions for developing DEBUGGER-015 (eBPF Syscall Tracing)

**Status**: GREEN Phase - Minimal implementation

## Prerequisites

### System Requirements

- **Linux Kernel**: 5.10+ (for BTF support)
- **Architecture**: x86_64 or ARM64
- **Permissions**: Root or `CAP_BPF` capability

### Required Tools

#### 1. LLVM and Clang

eBPF programs compile to BPF bytecode using LLVM:

```bash
# Ubuntu/Debian
sudo apt-get install llvm clang

# Fedora/RHEL
sudo dnf install llvm clang

# Arch
sudo pacman -S llvm clang
```

**Version**: LLVM 11+ recommended

#### 2. bpf-linker

Rust BPF linker for Aya:

```bash
cargo install bpf-linker
```

**Note**: This replaces the need for `llvm-strip` and other LLVM tools.

#### 3. bpftool (Optional, for debugging)

```bash
# Ubuntu/Debian
sudo apt-get install linux-tools-common linux-tools-generic

# Fedora/RHEL
sudo dnf install bpftool

# From source
git clone https://github.com/libbpf/bpftool.git
cd bpftool/src
make
sudo make install
```

**Use**: Inspect loaded eBPF programs and maps:
```bash
sudo bpftool prog list
sudo bpftool map list
```

## Project Structure

DEBUGGER-015 uses a **workspace** structure with two crates:

```
ruchyruchy/
├── Cargo.toml                    # Workspace root
├── ruchyruchy/                   # Main library (userspace)
│   ├── Cargo.toml
│   └── src/
│       └── tracing/
│           └── ebpf/
│               ├── mod.rs        # eBPF loader and reader
│               └── syscall_reader.rs
└── ruchyruchy-ebpf/              # eBPF programs (kernel space)
    ├── Cargo.toml
    └── src/
        └── syscall_tracer.rs     # eBPF program
```

**Why Separate Crates**:
- eBPF programs compile to BPF bytecode (different target)
- Userspace programs compile to native code (x86_64/ARM64)
- Aya requires `#![no_std]` and `#![no_main]` for eBPF code

## Aya Dependencies

### Userspace Crate (`ruchyruchy/Cargo.toml`)

```toml
[dependencies]
aya = { version = "0.12", features = ["async_tokio"] }
aya-log = "0.2"
tokio = { version = "1", features = ["full"] }
```

### eBPF Crate (`ruchyruchy-ebpf/Cargo.toml`)

```toml
[package]
name = "ruchyruchy-ebpf"
version = "0.1.0"
edition = "2021"

[dependencies]
aya-bpf = "0.1"
aya-log-ebpf = "0.1"

[profile.release]
opt-level = 3
lto = true

[[bin]]
name = "syscall_tracer"
path = "src/syscall_tracer.rs"
```

## Building eBPF Programs

### Compile eBPF Code

```bash
cd ruchyruchy-ebpf
cargo build --release --target bpfel-unknown-none
```

**Output**: `target/bpfel-unknown-none/release/syscall_tracer`

**Note**: `bpfel` = BPF little-endian (x86_64, ARM64)

### Build Script Integration

For automated builds, add to `ruchyruchy/build.rs`:

```rust
use std::process::Command;

fn main() {
    // Build eBPF programs
    let status = Command::new("cargo")
        .args(&["build", "--release", "--manifest-path", "../ruchyruchy-ebpf/Cargo.toml"])
        .status()
        .expect("Failed to build eBPF programs");

    if !status.success() {
        panic!("eBPF build failed");
    }

    // Tell cargo to rerun if eBPF sources change
    println!("cargo:rerun-if-changed=../ruchyruchy-ebpf/src");
}
```

## Running eBPF Programs

### Permissions Required

eBPF programs require elevated permissions:

**Option 1: Run as root**
```bash
sudo cargo run --example ebpf_demo
```

**Option 2: Grant CAP_BPF capability** (Linux 5.8+)
```bash
# Grant capability to binary
sudo setcap cap_bpf+ep target/debug/ruchydbg

# Run without sudo
cargo run --example ebpf_demo
```

**Option 3: Development with bpftrace**

For quick testing without compilation:
```bash
sudo bpftrace -e 'tracepoint:raw_syscalls:sys_enter { @[comm] = count(); }'
```

## Verification Checklist

Before starting development:

- [ ] Linux kernel 5.10+ (`uname -r`)
- [ ] LLVM/Clang installed (`clang --version`)
- [ ] bpf-linker installed (`bpf-linker --version`)
- [ ] Can compile eBPF: `cd ruchyruchy-ebpf && cargo build`
- [ ] Can run with sudo: `sudo cargo run --example ebpf_demo`

## Troubleshooting

### Error: "bpf-linker not found"

```bash
cargo install bpf-linker
```

### Error: "Permission denied" when loading eBPF

Run with `sudo` or grant `CAP_BPF` capability.

### Error: "Invalid argument" when attaching

Kernel might not support BTF. Check:
```bash
cat /sys/kernel/btf/vmlinux  # Should exist on 5.10+
```

### Error: "Program type not supported"

Update kernel to 5.10+ or enable `CONFIG_BPF_SYSCALL` in kernel config.

## Development Workflow

### 1. Edit eBPF Program

```bash
vim ruchyruchy-ebpf/src/syscall_tracer.rs
```

### 2. Compile eBPF

```bash
cd ruchyruchy-ebpf
cargo build --release
```

### 3. Edit Userspace Loader

```bash
vim src/tracing/ebpf/syscall_reader.rs
```

### 4. Run Integration Test

```bash
sudo cargo test test_ebpf_syscall_capture -- --ignored
```

### 5. Debug with bpftool

```bash
sudo bpftool prog list          # List loaded programs
sudo bpftool map dump id <ID>   # Dump map contents
sudo bpftool prog dump xlated id <ID>  # Disassemble
```

## CI/CD Considerations

### GitHub Actions

eBPF requires privileged containers:

```yaml
jobs:
  test-ebpf:
    runs-on: ubuntu-latest
    container:
      image: ubuntu:22.04
      options: --privileged  # Required for eBPF
    steps:
      - name: Install dependencies
        run: |
          apt-get update
          apt-get install -y llvm clang linux-headers-generic

      - name: Install bpf-linker
        run: cargo install bpf-linker

      - name: Build eBPF
        run: cd ruchyruchy-ebpf && cargo build --release

      - name: Run tests
        run: sudo cargo test --test test_ebpf_syscall_tracing -- --ignored
```

**Note**: Some CI environments (like GitHub Actions) may restrict eBPF. Consider:
- Using self-hosted runners with eBPF support
- Skipping eBPF tests in CI (run locally with `--ignored`)
- Using Docker with `--privileged` flag

## References

1. **Aya Book**: https://aya-rs.dev/book/
2. **Aya Template**: https://github.com/aya-rs/aya-template
3. **Tutorial**: "Track Linux Syscalls with Rust and eBPF" (2025)
4. **Kernel Requirements**: https://www.kernel.org/doc/html/latest/bpf/

## Next Steps

1. ✅ Install prerequisites (LLVM, bpf-linker)
2. ⏳ Create workspace structure
3. ⏳ Write minimal eBPF program
4. ⏳ Test loading and attaching
5. ⏳ Capture first syscall event

## Support

If you encounter issues:
1. Check kernel version: `uname -r` (need 5.10+)
2. Verify eBPF support: `zgrep CONFIG_BPF /proc/config.gz`
3. Review Aya documentation: https://aya-rs.dev/
4. File issue at: https://github.com/paiml/ruchyruchy/issues

---

**Status**: This guide is part of DEBUGGER-015 GREEN Phase

**Last Updated**: 2025-10-29
