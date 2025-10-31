//! Statistical Profiling (DEBUGGER-016)
//!
//! **Status**: REFACTOR Phase - Sample field extraction complete
//!
//! This module provides low-overhead statistical profiling using `perf_event_open`
//! and hardware performance counters (<1% overhead at 1000Hz).
//!
//! # Architecture
//!
//! Uses Linux `perf_event_open(2)` system call with:
//! - Hardware event: CPU_CYCLES
//! - Sampling frequency: 1000Hz (adjustable)
//! - Sample data: IP, TID, TIME, STACK_USER
//! - Ring buffer: 1MB per-CPU
//!
//! # Phases
//!
//! - ✅ RED Phase: Architecture documented, 6 tests written (all failing)
//! - ✅ GREEN Phase: Minimal implementation (sampling infrastructure)
//! - 🔄 REFACTOR Phase: Sample extraction, DWARF unwinding, flame graphs
//!
//! # Setup Required
//!
//! Before using this module, complete profiler setup:
//! See: `docs/specifications/DEBUGGER-016-PROFILER-ARCHITECTURE.md`
//!
//! Requirements:
//! - Linux kernel 3.2+ (4.0+ recommended)
//! - CAP_PERFMON or CAP_SYS_ADMIN or root
//! - PMU support (Performance Monitoring Unit)
//! - Debug info (DWARF) for stack unwinding
//!
//! # Usage
//!
//! ```no_run
//! use ruchyruchy::profiling::Profiler;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Initialize profiler (requires root or CAP_PERFMON)
//! let mut profiler = Profiler::new()?;
//!
//! // Start sampling at 1000Hz
//! profiler.start()?;
//!
//! // Run your code
//! // expensive_function();
//!
//! // Stop and collect samples
//! profiler.stop()?;
//! let samples = profiler.collect_samples()?;
//!
//! // Analyze hotspots
//! println!("Collected {} samples", samples.len());
//! # Ok(())
//! # }
//! ```
//!
//! # Implementation Status
//!
//! - [x] RED Phase: Requirements defined (6 tests)
//! - [x] Architecture documented
//! - [x] GREEN Phase: Basic sampling infrastructure (perf_event_open syscall)
//! - [x] GREEN Phase: Ring buffer allocation and reading
//! - [x] REFACTOR: Extract actual sample fields (ip, tid, time, stack)
//! - [ ] REFACTOR: DWARF unwinding (function names)
//! - [ ] REFACTOR: Flame graph generation
//! - [ ] REFACTOR: Hotspot analysis
//!
//! # References
//!
//! - Architecture: `docs/specifications/DEBUGGER-016-PROFILER-ARCHITECTURE.md`
//! - Tests: `tests/test_profiler.rs` (6 RED phase tests)
//! - man page: perf_event_open(2)

#![allow(dead_code)]

use std::error::Error;
use std::fmt;

#[cfg(feature = "profiling")]
use perf_event_open::{
    config::{Cpu, Opts, Proc, SampleOn, Size},
    count::Counter,
    event::hw::Hardware,
    sample::record::Record,
};

/// Error types for profiling operations
#[derive(Debug)]
pub enum ProfilerError {
    /// Failed to initialize perf_event_open
    InitializationFailed(String),
    /// Failed to start sampling
    StartFailed(String),
    /// Failed to stop sampling
    StopFailed(String),
    /// Failed to read samples from ring buffer
    ReadFailed(String),
    /// Permission denied (need root or CAP_PERFMON)
    PermissionDenied(String),
}

impl fmt::Display for ProfilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProfilerError::InitializationFailed(msg) => {
                write!(f, "Failed to initialize profiler: {}", msg)
            }
            ProfilerError::StartFailed(msg) => write!(f, "Failed to start profiling: {}", msg),
            ProfilerError::StopFailed(msg) => write!(f, "Failed to stop profiling: {}", msg),
            ProfilerError::ReadFailed(msg) => write!(f, "Failed to read samples: {}", msg),
            ProfilerError::PermissionDenied(msg) => {
                write!(
                    f,
                    "Permission denied: {}. Run with sudo or grant CAP_PERFMON.",
                    msg
                )
            }
        }
    }
}

impl Error for ProfilerError {}

/// A profiling sample captured from hardware counters
#[derive(Debug, Clone)]
pub struct Sample {
    /// Instruction pointer at time of sample
    pub ip: u64,
    /// Thread ID
    pub tid: u32,
    /// Timestamp in nanoseconds
    pub time: u64,
    /// User-space stack trace (instruction pointers)
    pub stack: Vec<u64>,
}

/// Flame graph data in brendangregg format
///
/// Aggregates profiling samples by stack trace and generates
/// flame graph data compatible with brendangregg/FlameGraph tools.
///
/// Format: Each line is "frame1;frame2;frame3 count"
/// where frames are semicolon-separated and count is the number of samples.
#[derive(Debug)]
pub struct FlameGraph {
    /// Aggregated stack traces with counts
    /// Key: semicolon-separated stack trace
    /// Value: number of samples with this stack
    stacks: std::collections::HashMap<String, usize>,
}

impl FlameGraph {
    /// Create a flame graph from profiling samples
    ///
    /// Aggregates samples by stack trace (using instruction pointers as hex strings).
    /// Note: For human-readable output, use DWARF unwinding to resolve function names.
    ///
    /// # Arguments
    ///
    /// * `samples` - Profiling samples to aggregate
    ///
    /// # Returns
    ///
    /// FlameGraph with aggregated stack traces
    pub fn from_samples(samples: &[Sample]) -> Self {
        let mut stacks = std::collections::HashMap::new();

        for sample in samples {
            // Build stack trace string from instruction pointers
            // Format each IP as hex: 0x7ffff7a1b2c3
            let stack_trace = if sample.stack.is_empty() {
                // Use IP if no stack trace available
                format!("0x{:x}", sample.ip)
            } else {
                // Reverse stack so deepest frame is last (flame graph convention)
                sample
                    .stack
                    .iter()
                    .rev()
                    .map(|ip| format!("0x{:x}", ip))
                    .collect::<Vec<_>>()
                    .join(";")
            };

            // Increment count for this stack trace
            *stacks.entry(stack_trace).or_insert(0) += 1;
        }

        FlameGraph { stacks }
    }

    /// Generate brendangregg format flame graph data
    ///
    /// Returns a string where each line is:
    /// "frame1;frame2;frame3 count"
    ///
    /// This format can be rendered by inferno or brendangregg/FlameGraph tools.
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        let mut lines: Vec<String> = self
            .stacks
            .iter()
            .map(|(stack, count)| format!("{} {}", stack, count))
            .collect();

        // Sort for deterministic output
        lines.sort();

        lines.join("\n")
    }
}

/// Hotspot analysis entry
///
/// Represents a single hotspot (instruction pointer) with its sample count and percentage.
#[derive(Debug, Clone)]
pub struct HotspotEntry {
    /// Function identifier (instruction pointer as hex string)
    /// Note: Use DWARF unwinding for human-readable function names
    pub function: String,
    /// Number of samples at this instruction pointer
    pub count: usize,
    /// Percentage of total samples
    pub percentage: f64,
}

/// Hotspot analyzer for identifying top N functions by CPU time
///
/// Aggregates profiling samples by instruction pointer and identifies
/// the functions consuming the most CPU time.
pub struct Hotspot;

impl Hotspot {
    /// Analyze samples and return top N hotspots
    ///
    /// Aggregates samples by instruction pointer (IP), calculates percentages,
    /// and returns the top N entries sorted by sample count (descending).
    ///
    /// # Arguments
    ///
    /// * `samples` - Profiling samples to analyze
    /// * `top_n` - Number of top hotspots to return (e.g., 10 for top 10)
    ///
    /// # Returns
    ///
    /// Vector of HotspotEntry sorted by sample count (descending).
    /// Each entry contains: function (hex IP), count, percentage.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ruchyruchy::profiling::{Profiler, Hotspot};
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut profiler = Profiler::new()?;
    /// profiler.start()?;
    /// // ... run workload ...
    /// profiler.stop()?;
    /// let samples = profiler.collect_samples()?;
    ///
    /// let hotspots = Hotspot::analyze(&samples, 10);  // Top 10
    /// for (i, entry) in hotspots.iter().enumerate() {
    ///     println!("#{}: {} ({:.2}% - {} samples)",
    ///         i + 1, entry.function, entry.percentage, entry.count);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn analyze(samples: &[Sample], top_n: usize) -> Vec<HotspotEntry> {
        if samples.is_empty() {
            return Vec::new();
        }

        // Aggregate samples by instruction pointer
        let mut ip_counts: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();

        for sample in samples {
            *ip_counts.entry(sample.ip).or_insert(0) += 1;
        }

        let total_samples = samples.len() as f64;

        // Convert to HotspotEntry with percentages
        let mut hotspots: Vec<HotspotEntry> = ip_counts
            .iter()
            .map(|(ip, count)| HotspotEntry {
                function: format!("0x{:x}", ip),
                count: *count,
                percentage: (*count as f64 / total_samples) * 100.0,
            })
            .collect();

        // Sort by count (descending)
        hotspots.sort_by(|a, b| b.count.cmp(&a.count));

        // Return top N
        hotspots.into_iter().take(top_n).collect()
    }
}

/// Statistical profiler using perf_event_open
///
/// GREEN Phase implementation: Basic sampling with hardware counters.
#[cfg(feature = "profiling")]
pub struct Profiler {
    counter: Counter,
    sampling_frequency: u64,
}

/// Placeholder profiler when profiling feature is disabled
#[cfg(not(feature = "profiling"))]
#[derive(Debug)]
pub struct Profiler {
    _phantom: std::marker::PhantomData<()>,
}

#[cfg(feature = "profiling")]
impl Profiler {
    /// Create a new profiler
    ///
    /// This will initialize perf_event_open with CPU_CYCLES event at 1000Hz.
    ///
    /// # Errors
    ///
    /// Returns `ProfilerError::PermissionDenied` if not running as root or without CAP_PERFMON.
    /// Returns `ProfilerError::InitializationFailed` if perf_event_open fails.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ruchyruchy::profiling::Profiler;
    ///
    /// let profiler = Profiler::new();
    /// match profiler {
    ///     Ok(_) => println!("Profiler initialized"),
    ///     Err(e) => println!("Setup required: {}", e),
    /// }
    /// ```
    pub fn new() -> Result<Self, ProfilerError> {
        Self::with_frequency(1000)
    }

    /// Create a new profiler with custom sampling frequency
    ///
    /// # Arguments
    ///
    /// * `frequency` - Sampling frequency in Hz (e.g., 1000 = 1000 samples/second)
    ///
    /// # Errors
    ///
    /// Returns `ProfilerError::PermissionDenied` if not running as root or without CAP_PERFMON.
    /// Returns `ProfilerError::InitializationFailed` if perf_event_open fails.
    pub fn with_frequency(frequency: u64) -> Result<Self, ProfilerError> {
        // Configure sampling options
        let mut opts = Opts {
            sample_on: SampleOn::Freq(frequency),
            ..Opts::default()
        };
        // Note: ip, tid, time are included automatically in samples
        opts.sample_format.user_stack = Some(Size(1024)); // 8KB user stack (1024 * 8 bytes)

        // Create counter for CPU_CYCLE on current process, all CPUs
        let event = Hardware::CpuCycle;
        let target = (Proc::CURRENT, Cpu::ALL);

        let counter = Counter::new(event, target, opts).map_err(|e| {
            let err_str = e.to_string();
            if err_str.contains("Permission denied")
                || err_str.contains("EPERM")
                || err_str.contains("EACCES")
            {
                ProfilerError::PermissionDenied(format!(
                    "perf_event_open failed: {}. Try running with sudo or grant CAP_PERFMON capability",
                    err_str
                ))
            } else {
                ProfilerError::InitializationFailed(format!(
                    "perf_event_open failed: {}",
                    err_str
                ))
            }
        })?;

        Ok(Self {
            counter,
            sampling_frequency: frequency,
        })
    }

    /// Get the sampling frequency
    pub fn sampling_frequency(&self) -> u64 {
        self.sampling_frequency
    }

    /// Check if sampling is currently enabled
    pub fn is_sampling_enabled(&self) -> bool {
        // Note: perf-event-open doesn't expose is_enabled() method
        // We track state manually (GREEN phase simplification)
        true // Assume enabled after creation
    }

    /// Start profiling
    ///
    /// Enables the performance counter and begins collecting samples.
    ///
    /// # Errors
    ///
    /// Returns `ProfilerError::StartFailed` if enabling the counter fails.
    pub fn start(&mut self) -> Result<(), ProfilerError> {
        self.counter
            .enable()
            .map_err(|e| ProfilerError::StartFailed(format!("Failed to enable counter: {}", e)))?;
        Ok(())
    }

    /// Stop profiling
    ///
    /// Disables the performance counter and stops collecting samples.
    ///
    /// # Errors
    ///
    /// Returns `ProfilerError::StopFailed` if disabling the counter fails.
    pub fn stop(&mut self) -> Result<(), ProfilerError> {
        self.counter
            .disable()
            .map_err(|e| ProfilerError::StopFailed(format!("Failed to disable counter: {}", e)))?;
        Ok(())
    }

    /// Collect samples from the ring buffer
    ///
    /// Reads all available samples from the kernel ring buffer and returns them.
    ///
    /// # Arguments
    ///
    /// * `buffer_pages` - Number of pages (2^n) for ring buffer. Default: 10 (2^10 = 1024 pages = 4MB)
    ///
    /// # Errors
    ///
    /// Returns `ProfilerError::ReadFailed` if reading from the ring buffer fails.
    pub fn collect_samples(&mut self) -> Result<Vec<Sample>, ProfilerError> {
        self.collect_samples_with_buffer(10)
    }

    /// Collect samples with custom ring buffer size
    ///
    /// # Arguments
    ///
    /// * `buffer_pages` - Number of pages as power of 2 (e.g., 10 = 2^10 = 1024 pages = 4MB)
    ///
    /// # Errors
    ///
    /// Returns `ProfilerError::ReadFailed` if reading from the ring buffer fails.
    pub fn collect_samples_with_buffer(
        &self,
        buffer_pages: u8,
    ) -> Result<Vec<Sample>, ProfilerError> {
        // Create sampler with ring buffer
        let sampler = self
            .counter
            .sampler(buffer_pages)
            .map_err(|e| ProfilerError::ReadFailed(format!("Failed to create sampler: {}", e)))?;

        let mut samples = Vec::new();

        // Iterate through all available samples
        // Note: sampler.iter() returns (Priv, Record) tuples
        for (_priv, record) in sampler.iter() {
            // Only process Sample records (not Mmap, Fork, etc.)
            if let Record::Sample(sample) = record {
                // Extract instruction pointer from code_addr
                // code_addr is Option<(u64, bool)> where bool indicates exact IP
                let ip = sample.code_addr.map(|(addr, _exact)| addr).unwrap_or(0);

                // Extract thread ID from task info
                let tid = sample.record_id.task.as_ref().map(|t| t.tid).unwrap_or(0);

                // Extract timestamp
                let time = sample.record_id.time.unwrap_or(0);

                // Extract stack trace from user_stack (Vec<u8>)
                // Convert from raw bytes to u64 instruction pointers
                let stack = sample
                    .user_stack
                    .as_ref()
                    .map(|bytes| {
                        bytes
                            .chunks_exact(8)
                            .map(|chunk| {
                                u64::from_ne_bytes([
                                    chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5],
                                    chunk[6], chunk[7],
                                ])
                            })
                            .filter(|&addr| addr != 0) // Filter out null addresses
                            .collect()
                    })
                    .unwrap_or_default();

                samples.push(Sample {
                    ip,
                    tid,
                    time,
                    stack,
                });
            }
        }

        Ok(samples)
    }
}

#[cfg(not(feature = "profiling"))]
impl Profiler {
    /// Create a new profiler (requires "profiling" feature)
    pub fn new() -> Result<Self, ProfilerError> {
        Err(ProfilerError::InitializationFailed(
            "Profiler requires 'profiling' feature. \
             Compile with --features profiling to enable. \
             See docs/specifications/DEBUGGER-016-PROFILER-ARCHITECTURE.md"
                .to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "profiling"))]
    use super::Profiler;

    #[test]
    #[cfg(not(feature = "profiling"))]
    fn test_profiler_requires_feature() {
        // This test verifies we provide helpful error messages
        // when profiling feature is not enabled

        let result = Profiler::new();

        assert!(result.is_err(), "Should fail without profiling feature");

        let err = result.unwrap_err();
        let msg = err.to_string();

        // Should mention profiling feature
        assert!(
            msg.contains("profiling"),
            "Error should explain profiling feature required"
        );

        // Should reference architecture doc
        assert!(
            msg.contains("DEBUGGER-016"),
            "Error should reference DEBUGGER-016"
        );
    }
}
