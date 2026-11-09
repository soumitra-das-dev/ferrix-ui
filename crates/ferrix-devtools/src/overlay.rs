//! In-app declarative developer overlay component.

use ferrix_core::prelude::*;
use ferrix_renderer_protocol::NodeId;

use crate::fps::FpsMonitor;
use crate::patches::PatchInspector;
use crate::signals::SignalInspector;

/// Builds the visual DevTools in-app overlay showing performance and state metrics.
pub fn build_devtools_overlay(
    cx: &mut ComponentContext,
    fps_monitor: &FpsMonitor,
    patch_inspector: &PatchInspector,
    signal_inspector: &SignalInspector,
) -> Element {
    let fps_val = fps_monitor.fps();
    let dropped = fps_monitor.dropped_frames();
    let patches_val = patch_inspector.total_patches();
    let signals_val = signal_inspector.count();
    let latency = fps_monitor.average_latency_ms();

    Column::new()
        .padding(12.0)
        .spacing(8.0)
        .background_color(0xF00F172A) // Slate 900 with 94% opacity
        .child(
            Row::new()
                .spacing(8.0)
                .child(
                    Text::new("⚡ Ferrix DevTools").size(16.0).color(0xFF38BDF8), // Sky 400
                )
                .child(
                    Text::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                        .size(12.0)
                        .color(0xFF94A3B8),
                ),
        )
        .child(
            Row::new()
                .spacing(12.0)
                .child(Text::new(format!("FPS: {:.1}", fps_val)).size(14.0).color(
                    if fps_val >= 55.0 {
                        0xFF4ADE80 // Green
                    } else {
                        0xFFF87171 // Red
                    },
                ))
                .child(
                    Text::new(format!("Dropped: {}", dropped))
                        .size(14.0)
                        .color(0xFFE2E8F0),
                )
                .child(
                    Text::new(format!("Latency: {:.1}ms", latency))
                        .size(14.0)
                        .color(0xFFE2E8F0),
                ),
        )
        .child(
            Row::new()
                .spacing(12.0)
                .child(
                    Text::new(format!("Patches: {}", patches_val))
                        .size(14.0)
                        .color(0xFFA78BFA), // Purple
                )
                .child(
                    Text::new(format!("Signals: {}", signals_val))
                        .size(14.0)
                        .color(0xFFFBBF24), // Amber
                ),
        )
        .build_with_id(NodeId::new(9900), cx.allocator())
}
