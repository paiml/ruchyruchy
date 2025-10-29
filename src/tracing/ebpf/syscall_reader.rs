//! Userspace eBPF loader and ring buffer reader

use aya::{Ebpf, maps::{MapData, RingBuf}, programs::TracePoint, include_bytes_aligned};
use std::error::Error;
use std::fmt;

/// Include the compiled eBPF program binary
///
/// Note: This path is relative to the crate root during compilation.
/// The eBPF program must be built first using:
/// `cd ruchyruchy-ebpf && cargo +nightly build --release -Z build-std=core`
const EBPF_PROGRAM: &[u8] = include_bytes_aligned!(
    concat!(env!("CARGO_MANIFEST_DIR"), "/target/bpfel-unknown-none/release/syscall_tracer")
);

/// Syscall event structure (must match eBPF definition)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SyscallEvent {
    pub pid: u32,
    pub syscall_nr: i64,
    pub timestamp_ns: u64,
    pub is_enter: u8,  // 1 for enter, 0 for exit
    pub _padding: [u8; 7],
}

/// Error types for eBPF operations
#[derive(Debug)]
pub enum EbpfError {
    /// Failed to load eBPF program
    LoadFailed(String),
    /// Failed to attach to tracepoint
    AttachFailed(String),
    /// Failed to read events from ring buffer
    ReadFailed(String),
    /// Permission denied (need root or CAP_BPF)
    PermissionDenied(String),
}

impl fmt::Display for EbpfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EbpfError::LoadFailed(msg) => write!(f, "Failed to load eBPF program: {}", msg),
            EbpfError::AttachFailed(msg) => write!(f, "Failed to attach eBPF program: {}", msg),
            EbpfError::ReadFailed(msg) => write!(f, "Failed to read eBPF events: {}", msg),
            EbpfError::PermissionDenied(msg) => {
                write!(f, "Permission denied: {}. Run with sudo or grant CAP_BPF capability.", msg)
            }
        }
    }
}

impl Error for EbpfError {}

/// Syscall tracer using eBPF
///
/// Loads eBPF programs and reads syscall events from the kernel.
pub struct SyscallTracer {
    ebpf: Ebpf,
}

impl SyscallTracer {
    /// Create a new syscall tracer
    ///
    /// This will:
    /// 1. Load the eBPF program into the kernel
    /// 2. Attach to sys_enter and sys_exit tracepoints
    /// 3. Open the ring buffer for reading events
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Insufficient permissions (need root or CAP_BPF)
    /// - Kernel doesn't support eBPF (need 5.10+)
    /// - Failed to attach to tracepoints
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ruchyruchy::tracing::ebpf::SyscallTracer;
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let tracer = SyscallTracer::new()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Result<Self, EbpfError> {
        // Load eBPF program
        let mut ebpf = Ebpf::load(EBPF_PROGRAM)
            .map_err(|e| {
                let err_str = e.to_string();
                if err_str.contains("Permission denied") || err_str.contains("EPERM") {
                    EbpfError::PermissionDenied(err_str)
                } else {
                    EbpfError::LoadFailed(err_str)
                }
            })?;

        // Attach to sys_enter tracepoint
        let program_enter: &mut TracePoint = ebpf
            .program_mut("sys_enter")
            .ok_or_else(|| EbpfError::LoadFailed("sys_enter program not found".to_string()))?
            .try_into()
            .map_err(|e: aya::programs::ProgramError| EbpfError::LoadFailed(e.to_string()))?;

        program_enter.load()
            .map_err(|e: aya::programs::ProgramError| EbpfError::LoadFailed(e.to_string()))?;

        program_enter.attach("raw_syscalls", "sys_enter")
            .map_err(|e: aya::programs::ProgramError| EbpfError::AttachFailed(format!("sys_enter: {}", e)))?;

        // Attach to sys_exit tracepoint
        let program_exit: &mut TracePoint = ebpf
            .program_mut("sys_exit")
            .ok_or_else(|| EbpfError::LoadFailed("sys_exit program not found".to_string()))?
            .try_into()
            .map_err(|e: aya::programs::ProgramError| EbpfError::LoadFailed(e.to_string()))?;

        program_exit.load()
            .map_err(|e: aya::programs::ProgramError| EbpfError::LoadFailed(e.to_string()))?;

        program_exit.attach("raw_syscalls", "sys_exit")
            .map_err(|e: aya::programs::ProgramError| EbpfError::AttachFailed(format!("sys_exit: {}", e)))?;

        Ok(Self {
            ebpf,
        })
    }

    /// Read syscall events from the ring buffer
    ///
    /// This is a non-blocking read that returns all available events.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruchyruchy::tracing::ebpf::SyscallTracer;
    /// # fn example(mut tracer: SyscallTracer) -> Result<(), Box<dyn std::error::Error>> {
    /// let events = tracer.read_events()?;
    /// println!("Read {} syscall events", events.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_events(&mut self) -> Result<Vec<SyscallEvent>, EbpfError> {
        let mut events = Vec::new();

        // Get ring buffer from map
        let map = self.ebpf.map_mut("EVENTS")
            .ok_or_else(|| EbpfError::ReadFailed("EVENTS ring buffer not found".to_string()))?;

        let mut ring_buf = RingBuf::try_from(map)
            .map_err(|e| EbpfError::ReadFailed(e.to_string()))?;

        while let Some(item) = ring_buf.next() {
            if item.len() != std::mem::size_of::<SyscallEvent>() {
                eprintln!("Warning: Unexpected event size: {} bytes", item.len());
                continue;
            }

            // Safety: We verified the size matches SyscallEvent
            let event: SyscallEvent = unsafe {
                std::ptr::read(item.as_ptr() as *const SyscallEvent)
            };
            events.push(event);
        }

        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires root privileges
    fn test_syscall_tracer_load() {
        // This test verifies we can load and attach the eBPF program
        let result = SyscallTracer::new();

        match result {
            Ok(_tracer) => {
                // Success - eBPF program loaded and attached
                println!("✅ eBPF program loaded successfully");
            }
            Err(EbpfError::PermissionDenied(msg)) => {
                println!("⚠️  Test skipped: {}", msg);
                println!("   Run with: sudo cargo test --features ebpf -- --ignored");
            }
            Err(e) => {
                panic!("Failed to load eBPF: {:?}", e);
            }
        }
    }

    #[test]
    fn test_syscall_event_size() {
        // Verify SyscallEvent matches the eBPF definition
        assert_eq!(
            std::mem::size_of::<SyscallEvent>(),
            32,  // 4 (pid) + 8 (syscall_nr) + 8 (timestamp) + 1 (is_enter) + 7 (padding) + 4 (alignment)
            "SyscallEvent size must match eBPF definition"
        );
    }
}
