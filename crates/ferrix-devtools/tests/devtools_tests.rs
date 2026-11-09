use ferrix_core::prelude::*;
use ferrix_devtools::prelude::*;
use ferrix_renderer_protocol::{Patch, PatchBatch};

#[test]
fn test_fps_monitor_calculation_and_dropped_frames() {
    let mut monitor = FpsMonitor::new(10);

    // Initial default FPS
    assert_eq!(monitor.fps(), 60.0);
    assert_eq!(monitor.dropped_frames(), 0);

    // Record healthy 60fps frames (16.0ms)
    for _ in 0..5 {
        monitor.record_frame(16.0);
    }
    assert!(monitor.fps() >= 60.0);
    assert_eq!(monitor.dropped_frames(), 0);

    // Record a stutter frame (40.0ms)
    monitor.record_frame(40.0);
    assert_eq!(monitor.dropped_frames(), 1);
    assert!(monitor.fps() < 60.0);
}

#[test]
fn test_patch_inspector_metrics() {
    let mut inspector = PatchInspector::new();

    let mut batch = PatchBatch::new();
    batch.push(Patch::CreateNode {
        node_id: NodeId::new(2),
        view_type: ferrix_renderer_protocol::ViewType::Text,
    });
    batch.push(Patch::SetText {
        node_id: NodeId::new(2),
        text: "DevTools Verification".to_string(),
    });

    inspector.record_batch(&batch);

    assert_eq!(inspector.total_batches(), 1);
    assert_eq!(inspector.total_patches(), 2);
    assert_eq!(inspector.type_counts().get("CreateNode"), Some(&1));
    assert_eq!(inspector.type_counts().get("SetText"), Some(&1));
    assert_eq!(inspector.recent_batches().len(), 1);
}

#[test]
fn test_signal_inspector_recording() {
    let mut inspector = SignalInspector::new();

    inspector.record_value("counter", "0");
    assert_eq!(inspector.count(), 1);
    assert_eq!(inspector.get("counter").unwrap().current_value, "0");
    assert_eq!(inspector.get("counter").unwrap().update_count, 1);

    inspector.record_value("counter", "1");
    assert_eq!(inspector.get("counter").unwrap().current_value, "1");
    assert_eq!(inspector.get("counter").unwrap().update_count, 2);
}

#[test]
fn test_background_scheduler_lifecycle() {
    let mut scheduler = BackgroundScheduler::new();

    scheduler.schedule_task("sync_user_data", 1000, true, 3600_000);
    assert_eq!(scheduler.tasks().len(), 1);

    let task = scheduler.get_task("sync_user_data").unwrap();
    assert_eq!(task.status, TaskStatus::Scheduled);
    assert!(task.is_periodic);

    scheduler.update_status("sync_user_data", true);
    assert_eq!(
        scheduler.get_task("sync_user_data").unwrap().status,
        TaskStatus::Completed
    );

    scheduler.cancel_task("sync_user_data");
    assert_eq!(
        scheduler.get_task("sync_user_data").unwrap().status,
        TaskStatus::Cancelled
    );
}

#[test]
fn test_devtools_overlay_tree_generation() {
    let mut cx = ComponentContext::with_start_id(2);
    let mut fps = FpsMonitor::new(10);
    fps.record_frame(16.6);

    let mut patches = PatchInspector::new();
    let mut batch = PatchBatch::new();
    batch.push(Patch::SetText {
        node_id: NodeId::new(3),
        text: "Sample".to_string(),
    });
    patches.record_batch(&batch);

    let mut signals = SignalInspector::new();
    signals.record_value("theme", "Dark");

    let overlay = build_devtools_overlay(&mut cx, &fps, &patches, &signals);

    assert_eq!(
        overlay.view_type,
        ferrix_renderer_protocol::ViewType::Column
    );
    assert!(!overlay.children.is_empty());
}
