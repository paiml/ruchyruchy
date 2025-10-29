#![no_std]
#![no_main]

use aya_ebpf::{
    macros::{map, tracepoint},
    maps::RingBuf,
    programs::TracePointContext,
    helpers,
};

/// Syscall event structure (must match userspace definition)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SyscallEvent {
    pub pid: u32,
    pub syscall_nr: i64,
    pub timestamp_ns: u64,
    pub is_enter: u8,  // 1 for enter, 0 for exit
    pub _padding: [u8; 7],  // Align to 8 bytes
}

#[map]
static EVENTS: RingBuf = RingBuf::with_byte_size(256 * 1024, 0);

/// Tracepoint: raw_syscalls:sys_enter
///
/// Captures syscall entry events
#[tracepoint]
pub fn sys_enter(ctx: TracePointContext) -> u32 {
    match try_sys_enter(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_sys_enter(ctx: TracePointContext) -> Result<u32, u32> {
    // Read syscall number from tracepoint context
    // For raw_syscalls:sys_enter, the layout is:
    //   long id;           // syscall number (offset 8)
    //   unsigned long args[6];
    let syscall_nr: i64 = unsafe { ctx.read_at(8).map_err(|_| 1u32)? };

    // Get current PID
    let pid = (helpers::bpf_get_current_pid_tgid() >> 32) as u32;

    // Get timestamp
    let timestamp_ns = unsafe { helpers::bpf_ktime_get_ns() };

    // Create event
    let event = SyscallEvent {
        pid,
        syscall_nr,
        timestamp_ns,
        is_enter: 1,
        _padding: [0; 7],
    };

    // Write to ring buffer
    if let Some(mut entry) = EVENTS.reserve::<SyscallEvent>(0) {
        unsafe {
            let ptr: *mut SyscallEvent = entry.as_mut_ptr() as *mut SyscallEvent;
            *ptr = event;
        }
        entry.submit(0);
    }

    Ok(0)
}

/// Tracepoint: raw_syscalls:sys_exit
///
/// Captures syscall exit events
#[tracepoint]
pub fn sys_exit(ctx: TracePointContext) -> u32 {
    match try_sys_exit(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_sys_exit(ctx: TracePointContext) -> Result<u32, u32> {
    // Read syscall number from tracepoint context
    // For raw_syscalls:sys_exit, the layout is:
    //   long id;           // syscall number (offset 8)
    //   long ret;          // return value
    let syscall_nr: i64 = unsafe { ctx.read_at(8).map_err(|_| 1u32)? };

    // Get current PID
    let pid = (helpers::bpf_get_current_pid_tgid() >> 32) as u32;

    // Get timestamp
    let timestamp_ns = unsafe { helpers::bpf_ktime_get_ns() };

    // Create event
    let event = SyscallEvent {
        pid,
        syscall_nr,
        timestamp_ns,
        is_enter: 0,
        _padding: [0; 7],
    };

    // Write to ring buffer
    if let Some(mut entry) = EVENTS.reserve::<SyscallEvent>(0) {
        unsafe {
            let ptr: *mut SyscallEvent = entry.as_mut_ptr() as *mut SyscallEvent;
            *ptr = event;
        }
        entry.submit(0);
    }

    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
