//! Per-Thread Trace Buffers
//!
//! Lock-free SPSC (single-producer, single-consumer) ring buffer for trace events.
//! Each thread has its own buffer, eliminating lock contention.

use super::events::TraceEvent;
use std::cell::RefCell;
use std::collections::VecDeque;

/// Per-thread trace buffer (lock-free SPSC)
pub struct TraceBuffer {
    /// Events buffer (using VecDeque as simple SPSC)
    events: VecDeque<TraceEvent>,
    /// Maximum capacity
    capacity: usize,
    /// Number of dropped events (when buffer full)
    dropped: usize,
}

impl TraceBuffer {
    /// Create new trace buffer with given capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            events: VecDeque::with_capacity(capacity),
            capacity,
            dropped: 0,
        }
    }

    /// Push event to buffer
    pub fn push(&mut self, event: TraceEvent) {
        if self.events.len() >= self.capacity {
            // Buffer full, drop oldest event
            self.events.pop_front();
            self.dropped += 1;
        }
        self.events.push_back(event);
    }

    /// Drain all events from buffer
    pub fn drain(&mut self) -> Vec<TraceEvent> {
        self.events.drain(..).collect()
    }

    /// Get number of events in buffer
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Get number of dropped events
    pub fn dropped(&self) -> usize {
        self.dropped
    }
}

thread_local! {
    /// Thread-local trace buffer
    static THREAD_BUFFER: RefCell<TraceBuffer> = RefCell::new(TraceBuffer::new(10000));
}

/// Record trace event to current thread's buffer
pub fn record_event(event: TraceEvent) {
    THREAD_BUFFER.with(|buf| {
        buf.borrow_mut().push(event);
    });
}

/// Drain all events from current thread's buffer
pub fn drain_thread_events() -> Vec<TraceEvent> {
    THREAD_BUFFER.with(|buf| buf.borrow_mut().drain())
}

/// Get statistics for current thread's buffer
pub fn thread_buffer_stats() -> (usize, usize) {
    THREAD_BUFFER.with(|buf| {
        let b = buf.borrow();
        (b.len(), b.dropped())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tracing::events::{function_enter, SourceLocation};

    #[test]
    fn test_buffer_push_and_drain() {
        let mut buffer = TraceBuffer::new(100);

        let event = function_enter(
            "test",
            vec![],
            SourceLocation {
                file: "test.ruchy".to_string(),
                line: 1,
                column: 1,
            },
        );

        buffer.push(event.clone());
        assert_eq!(buffer.len(), 1);

        let drained = buffer.drain();
        assert_eq!(drained.len(), 1);
        assert_eq!(buffer.len(), 0);
    }

    #[test]
    fn test_buffer_overflow_drops_oldest() {
        let mut buffer = TraceBuffer::new(3);

        for i in 0..5 {
            let event = function_enter(
                &format!("fn{}", i),
                vec![],
                SourceLocation {
                    file: "test.ruchy".to_string(),
                    line: 1,
                    column: 1,
                },
            );
            buffer.push(event);
        }

        assert_eq!(buffer.len(), 3); // Buffer capped at capacity
        assert_eq!(buffer.dropped(), 2); // Dropped 2 oldest events

        let drained = buffer.drain();
        // Should have events for fn2, fn3, fn4 (fn0, fn1 dropped)
        match &drained[0] {
            TraceEvent::FunctionEnter(entry) => {
                assert_eq!(entry.name, "fn2");
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_thread_local_buffer() {
        // Record some events
        for i in 0..5 {
            let event = function_enter(
                &format!("fn{}", i),
                vec![],
                SourceLocation {
                    file: "test.ruchy".to_string(),
                    line: i as u32,
                    column: 1,
                },
            );
            record_event(event);
        }

        // Check stats
        let (len, dropped) = thread_buffer_stats();
        assert_eq!(len, 5);
        assert_eq!(dropped, 0);

        // Drain events
        let events = drain_thread_events();
        assert_eq!(events.len(), 5);

        // Buffer should be empty now
        let (len, _) = thread_buffer_stats();
        assert_eq!(len, 0);
    }
}
