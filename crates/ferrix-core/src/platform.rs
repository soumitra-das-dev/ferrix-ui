//! Reactive platform capability hooks: permissions, fused location, camera, and storage.

use ferrix_reactivity::Signal;
use ferrix_renderer_protocol::platform::{PlatformRequest, PlatformResponse};
use ferrix_renderer_protocol::NodeId;
use std::sync::{Arc, Mutex};

use crate::component::ComponentContext;

/// Current permission status.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PermissionStatus {
    NotRequested,
    Granted,
    Denied { should_show_rationale: bool },
}

/// Fused location coordinate packet.
#[derive(Clone, Debug, PartialEq)]
pub struct LocationData {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f32,
    pub timestamp: u64,
}

/// State of the native camera preview.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CameraState {
    Inactive,
    Active { width: u32, height: u32 },
}

/// Channel for queuing outbound platform requests from hooks.
#[derive(Clone, Default)]
pub struct PlatformOutbox {
    queue: Arc<Mutex<Vec<PlatformRequest>>>,
}

impl PlatformOutbox {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push(&self, req: PlatformRequest) {
        self.queue.lock().unwrap().push(req);
    }

    pub fn drain(&self) -> Vec<PlatformRequest> {
        let mut guard = self.queue.lock().unwrap();
        std::mem::take(&mut *guard)
    }
}

/// Reactive hook: `use_permission`
/// Binds runtime permission checking and requesting with rationale state.
pub fn use_permission(
    cx: &mut ComponentContext,
    outbox: &PlatformOutbox,
    permission: &str,
    request_id: u32,
) -> (Signal<PermissionStatus>, Arc<dyn Fn() + Send + Sync>) {
    let status_signal = cx.use_signal(PermissionStatus::NotRequested);

    let perm_string = permission.to_string();
    let outbox_clone = outbox.clone();
    let trigger: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
        outbox_clone.push(PlatformRequest::RequestPermission {
            request_id,
            permission: perm_string.clone(),
        });
    });

    (status_signal, trigger)
}

/// Reactive hook: `use_location`
/// Automatically starts location updates on mount and cancels them on component unmount.
pub fn use_location(
    cx: &mut ComponentContext,
    outbox: &PlatformOutbox,
    min_interval_ms: u64,
    min_distance_m: f32,
) -> Signal<Option<LocationData>> {
    let location_signal = cx.use_signal(None);

    let outbox_mount = outbox.clone();
    let outbox_cleanup = outbox.clone();

    cx.use_effect(
        move || {
            // Subscribe to location updates
            outbox_mount.push(PlatformRequest::StartLocationUpdates {
                min_interval_ms,
                min_distance_m,
            });

            // Cleanup closure runs on unmount
            Some(Box::new(move || {
                outbox_cleanup.push(PlatformRequest::StopLocationUpdates);
            }))
        },
        (), // Runs once on mount
    );

    location_signal
}

/// Reactive hook: `use_camera`
/// Binds camera preview to a target View NodeId with lifecycle teardown on unmount.
pub fn use_camera(
    cx: &mut ComponentContext,
    outbox: &PlatformOutbox,
    target_node_id: NodeId,
) -> Signal<CameraState> {
    let camera_signal = cx.use_signal(CameraState::Inactive);

    let outbox_mount = outbox.clone();
    let outbox_cleanup = outbox.clone();

    cx.use_effect(
        move || {
            outbox_mount.push(PlatformRequest::StartCameraPreview { target_node_id });

            Some(Box::new(move || {
                outbox_cleanup.push(PlatformRequest::StopCamera);
            }))
        },
        target_node_id.raw(),
    );

    camera_signal
}

/// Handles an incoming platform response and routes it to the corresponding reactive signals.
pub fn route_platform_response(
    response: &PlatformResponse,
    permission_signal: Option<&Signal<PermissionStatus>>,
    location_signal: Option<&Signal<Option<LocationData>>>,
    camera_signal: Option<&Signal<CameraState>>,
) {
    match response {
        PlatformResponse::PermissionResult {
            granted,
            should_show_rationale,
            ..
        } => {
            if let Some(sig) = permission_signal {
                sig.set(if *granted {
                    PermissionStatus::Granted
                } else {
                    PermissionStatus::Denied {
                        should_show_rationale: *should_show_rationale,
                    }
                });
            }
        }
        PlatformResponse::LocationUpdate {
            latitude,
            longitude,
            accuracy,
            timestamp,
        } => {
            if let Some(sig) = location_signal {
                sig.set(Some(LocationData {
                    latitude: *latitude,
                    longitude: *longitude,
                    accuracy: *accuracy,
                    timestamp: *timestamp,
                }));
            }
        }
        PlatformResponse::CameraReady {
            preview_width,
            preview_height,
            ..
        } => {
            if let Some(sig) = camera_signal {
                sig.set(CameraState::Active {
                    width: *preview_width,
                    height: *preview_height,
                });
            }
        }
        _ => {}
    }
}
