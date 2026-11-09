//! Background task management and scheduling for Android WorkManager.

use std::collections::HashMap;

/// Lifecycle status of a background task.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TaskStatus {
    Scheduled,
    Running,
    Completed,
    Cancelled,
    Failed,
}

/// Metadata describing a scheduled background job.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BackgroundTask {
    pub task_id: String,
    pub delay_ms: u64,
    pub is_periodic: bool,
    pub interval_ms: u64,
    pub status: TaskStatus,
}

/// Coordinates background job scheduling between the Rust runtime and Android WorkManager.
#[derive(Clone, Debug, Default)]
pub struct BackgroundScheduler {
    tasks: HashMap<String, BackgroundTask>,
}

impl BackgroundScheduler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Schedules a one-shot or periodic background task.
    pub fn schedule_task(
        &mut self,
        task_id: impl Into<String>,
        delay_ms: u64,
        is_periodic: bool,
        interval_ms: u64,
    ) {
        let id = task_id.into();
        self.tasks.insert(
            id.clone(),
            BackgroundTask {
                task_id: id,
                delay_ms,
                is_periodic,
                interval_ms,
                status: TaskStatus::Scheduled,
            },
        );
    }

    /// Cancels a scheduled or running background task.
    pub fn cancel_task(&mut self, task_id: &str) -> bool {
        if let Some(task) = self.tasks.get_mut(task_id) {
            task.status = TaskStatus::Cancelled;
            true
        } else {
            false
        }
    }

    /// Marks task completion status reported by Android WorkManager.
    pub fn update_status(&mut self, task_id: &str, success: bool) -> bool {
        if let Some(task) = self.tasks.get_mut(task_id) {
            task.status = if success {
                TaskStatus::Completed
            } else {
                TaskStatus::Failed
            };
            true
        } else {
            false
        }
    }

    /// Returns a list of all active or pending tasks.
    pub fn tasks(&self) -> Vec<BackgroundTask> {
        self.tasks.values().cloned().collect()
    }

    /// Retrieves a task by ID.
    pub fn get_task(&self, task_id: &str) -> Option<&BackgroundTask> {
        self.tasks.get(task_id)
    }
}
