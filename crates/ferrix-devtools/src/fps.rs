//! Frame rate monitoring and frame drop detection.

use std::collections::VecDeque;

/// Monitors Android/Rust frame execution times and calculates rolling FPS.
#[derive(Clone, Debug)]
pub struct FpsMonitor {
    window_size: usize,
    samples_ms: VecDeque<f32>,
    dropped_frames: usize,
}

impl Default for FpsMonitor {
    fn default() -> Self {
        Self::new(30)
    }
}

impl FpsMonitor {
    /// Creates an FPS monitor with a maximum sample window size.
    pub fn new(window_size: usize) -> Self {
        Self {
            window_size,
            samples_ms: VecDeque::with_capacity(window_size),
            dropped_frames: 0,
        }
    }

    /// Records the execution duration of a frame in milliseconds.
    pub fn record_frame(&mut self, duration_ms: f32) {
        if self.samples_ms.len() >= self.window_size {
            self.samples_ms.pop_front();
        }
        self.samples_ms.push_back(duration_ms);

        // A frame taking longer than 16.67ms misses a 60Hz frame window
        if duration_ms > 16.67 {
            self.dropped_frames += 1;
        }
    }

    /// Calculates rolling average FPS over the recorded window.
    pub fn fps(&self) -> f32 {
        if self.samples_ms.is_empty() {
            return 60.0;
        }

        let total: f32 = self.samples_ms.iter().sum();
        let avg_duration = total / self.samples_ms.len() as f32;

        if avg_duration > 0.0 {
            (1000.0 / avg_duration).min(120.0)
        } else {
            60.0
        }
    }

    /// Total count of dropped frames since monitoring began.
    pub fn dropped_frames(&self) -> usize {
        self.dropped_frames
    }

    /// Average frame latency in milliseconds over the recorded window.
    pub fn average_latency_ms(&self) -> f32 {
        if self.samples_ms.is_empty() {
            return 0.0;
        }
        let total: f32 = self.samples_ms.iter().sum();
        total / self.samples_ms.len() as f32
    }
}
