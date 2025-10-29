//! Userspace eBPF loader and ring buffer reader

use aya::{Ebpf, maps::RingBuf, programs::TracePoint, include_bytes_aligned};
use std::error::Error;
use std::fmt;
use tokio::task;

/// Include the compiled eBPF program binary
const EBPF_PROGRAM: &[u8] = include_bytes_aligned!(
    "../../../target/bpfel-unknown-none/release/syscall_tracer"
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
    _ebpf: Ebpf,
    ring_buf: RingBuf<Vec<u8>>,
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
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let tracer = SyscallTracer::new().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new() -> Result<Self, EbpfError> {
        // Load eBPF program
        let mut ebpf = Ebpf::load(EBPF_PROGRAM)
            .map_err(|e| {
                if e.to_string().contains("Permission denied") || e.to_string().contains("EPERM") {
                    EbpfError::PermissionDenied(e.to_string())
                } else {
                    EbpfError::LoadFailed(e.to_string())
                }
            })?;

        // Attach to sys_enter tracepoint
        let program_enter: &mut TracePoint = ebpf
            .program_mut("sys_enter")
            .ok_or_else(|| EbpfError::LoadFailed("sys_enter program not found".to_string()))?
            .try_into()
            .map_err(|e: aya::EbpfError| EbpfError::LoadFailed(e.to_string()))?;

        program_enter.load()
            .map_err(|e| EbpfError::LoadFailed(e.to_string()))?;

        program_enter.attach("raw_syscalls", "sys_enter")
            .map_err(|e| EbpfError::AttachFailed(format!("sys_enter: {}", e)))?;

        // Attach to sys_exit tracepoint
        let program_exit: &mut TracePoint = ebpf
            .program_mut("sys_exit")
            .ok_or_else(|| EbpfError::LoadFailed("sys_exit program not found".to_string()))?
            .try_into()
            .map_err(|e: aya::EbpfError| EbpfError::LoadFailed(e.to_string()))?;

        program_exit.load()
            .map_err(|e| EbpfError::LoadFailed(e.to_string()))?;

        program_exit.attach("raw_syscalls", "sys_exit")
            .map_err(|e| EbpfError::AttachFailed(format!("sys_exit: {}", e)))?;

        // Open ring buffer
        let ring_buf: RingBuf<Vec<u8>> = RingBuf::try_from(
            ebpf.take_map("EVENTS")
                .ok_or_else(|| EbpfError::LoadFailed("EVENTS ring buffer not found".to_string()))?
        ).map_err(|e| EbpfError::LoadFailed(e.to_string()))?;

        Ok(Self {
            _ebpf: ebpf,
            ring_buf,
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
    /// # async fn example(mut tracer: SyscallTracer) -> Result<(), Box<dyn std::error::Error>> {
    /// let events = tracer.read_events().await?;
    /// println!("Read {} syscall events", events.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read_events(&mut self) -> Result<Vec<SyscallEvent>, EbpfError> {
        let mut events = Vec::new();
        let ring_buf = &mut self.ring_buf;

        // Spawn blocking task to read from ring buffer
        task::spawn_blocking(move || {
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
            Ok::<Vec<SyscallEvent>, EbpfError>(events)
        }).await
        .map_err(|e| EbpfError::ReadFailed(e.to_string()))?
    }

    /// Read syscall events with a timeout
    ///
    /// Waits for events up to the specified duration.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruchyruchy::tracing::ebpf::SyscallTracer;
    /// # use std::time::Duration;
    /// # async fn example(mut tracer: SyscallTracer) -> Result<(), Box<dyn std::error::Error>> {
    /// let events = tracer.read_events_timeout(Duration::from_secs(1)).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read_events_timeout(
        &mut self,
        timeout: std::time::Duration,
    ) -> Result<Vec<SyscallEvent>, EbpfError> {
        tokio::time::timeout(timeout, self.read_events())
            .await
            .map_err(|_| EbpfError::ReadFailed("Timeout reading events".to_string()))?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires root privileges
    async fn test_syscall_tracer_load() {
        // This test verifies we can load and attach the eBPF program
        let result = SyscallTracer::new().await;

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
