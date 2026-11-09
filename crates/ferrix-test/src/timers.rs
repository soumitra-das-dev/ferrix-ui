//! Deterministic fake timers and scheduler for headless tests.

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;

struct TimerEntry {
    id: u64,
    fire_at_ms: u64,
    interval_ms: Option<u64>,
    callback: Rc<RefCell<dyn FnMut()>>,
}

impl PartialEq for TimerEntry {
    fn eq(&self, other: &Self) -> bool {
        self.fire_at_ms == other.fire_at_ms && self.id == other.id
    }
}

impl Eq for TimerEntry {}

impl PartialOrd for TimerEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimerEntry {
    // Reverse ordering for min-heap
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .fire_at_ms
            .cmp(&self.fire_at_ms)
            .then_with(|| other.id.cmp(&self.id))
    }
}

/// Headless deterministic mock timer engine.
#[derive(Clone, Default)]
pub struct FakeTimers {
    current_time_ms: Rc<RefCell<u64>>,
    next_id: Rc<RefCell<u64>>,
    heap: Rc<RefCell<BinaryHeap<TimerEntry>>>,
}

impl FakeTimers {
    /// Creates a new fake timer engine starting at time 0.
    pub fn new() -> Self {
        Self::default()
    }

    /// Current simulated clock time in milliseconds.
    pub fn now_ms(&self) -> u64 {
        *self.current_time_ms.borrow()
    }

    /// Schedules a one-shot timer after `delay_ms`.
    pub fn set_timeout<F: FnMut() + 'static>(&self, delay_ms: u64, callback: F) -> u64 {
        let mut id_ref = self.next_id.borrow_mut();
        let id = *id_ref;
        *id_ref += 1;

        let fire_at_ms = self.now_ms() + delay_ms;
        self.heap.borrow_mut().push(TimerEntry {
            id,
            fire_at_ms,
            interval_ms: None,
            callback: Rc::new(RefCell::new(callback)),
        });
        id
    }

    /// Schedules a repeating timer every `interval_ms`.
    pub fn set_interval<F: FnMut() + 'static>(&self, interval_ms: u64, callback: F) -> u64 {
        let mut id_ref = self.next_id.borrow_mut();
        let id = *id_ref;
        *id_ref += 1;

        let fire_at_ms = self.now_ms() + interval_ms;
        self.heap.borrow_mut().push(TimerEntry {
            id,
            fire_at_ms,
            interval_ms: Some(interval_ms),
            callback: Rc::new(RefCell::new(callback)),
        });
        id
    }

    /// Clears a scheduled timer by ID.
    pub fn clear_timer(&self, id: u64) {
        let mut heap = self.heap.borrow_mut();
        let retain: Vec<TimerEntry> = heap.drain().filter(|e| e.id != id).collect();
        for entry in retain {
            heap.push(entry);
        }
    }

    /// Number of timers currently pending execution.
    pub fn pending_count(&self) -> usize {
        self.heap.borrow().len()
    }

    /// Advances simulated time by `ms`, firing all expired timers in temporal order.
    pub fn advance_timers_by_time(&self, ms: u64) {
        let target_time = self.now_ms() + ms;
        *self.current_time_ms.borrow_mut() = target_time;

        loop {
            let next_ready = {
                let heap = self.heap.borrow();
                if let Some(entry) = heap.peek() {
                    if entry.fire_at_ms <= target_time {
                        Some((
                            entry.id,
                            entry.fire_at_ms,
                            entry.interval_ms,
                            entry.callback.clone(),
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            };

            if let Some((id, fire_at_ms, interval, callback)) = next_ready {
                self.heap.borrow_mut().pop();

                // Execute callback
                (callback.borrow_mut())();

                // Re-queue if interval
                if let Some(interval_ms) = interval {
                    self.heap.borrow_mut().push(TimerEntry {
                        id,
                        fire_at_ms: fire_at_ms + interval_ms,
                        interval_ms: Some(interval_ms),
                        callback,
                    });
                }
            } else {
                break;
            }
        }
    }

    /// Executes all scheduled timers until the queue is completely drained.
    pub fn run_all_timers(&self) {
        let mut iterations = 0;
        while let Some(entry) = self.heap.borrow_mut().pop() {
            *self.current_time_ms.borrow_mut() = entry.fire_at_ms;
            (entry.callback.borrow_mut())();

            iterations += 1;
            if iterations > 10_000 {
                panic!(
                    "run_all_timers exceeded 10,000 iterations (possible infinite interval loop)"
                );
            }
        }
    }
}
