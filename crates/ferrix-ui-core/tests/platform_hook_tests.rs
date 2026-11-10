use ferrix_core::component::ComponentContext;
use ferrix_core::platform::*;
use ferrix_renderer_protocol::platform::{PlatformRequest, PlatformResponse};
use ferrix_renderer_protocol::NodeId;

#[test]
fn test_permission_request_and_response_routing() {
    let mut cx = ComponentContext::new();
    let outbox = PlatformOutbox::new();

    let (perm_signal, request_fn) =
        use_permission(&mut cx, &outbox, "android.permission.CAMERA", 1);
    assert_eq!(perm_signal.get(), PermissionStatus::NotRequested);

    // Call request trigger
    request_fn();

    // Verify outbox queued request
    let queued = outbox.drain();
    assert_eq!(queued.len(), 1);
    assert_eq!(
        queued[0],
        PlatformRequest::RequestPermission {
            request_id: 1,
            permission: "android.permission.CAMERA".to_string(),
        }
    );

    // Route response: Granted
    route_platform_response(
        &PlatformResponse::PermissionResult {
            request_id: 1,
            granted: true,
            should_show_rationale: false,
        },
        Some(&perm_signal),
        None,
        None,
    );

    assert_eq!(perm_signal.get(), PermissionStatus::Granted);

    // Route response: Denied with rationale
    route_platform_response(
        &PlatformResponse::PermissionResult {
            request_id: 1,
            granted: false,
            should_show_rationale: true,
        },
        Some(&perm_signal),
        None,
        None,
    );

    assert_eq!(
        perm_signal.get(),
        PermissionStatus::Denied {
            should_show_rationale: true
        }
    );
}

#[test]
fn test_location_hook_lifecycle_and_cancellation() {
    let mut cx = ComponentContext::new();
    let outbox = PlatformOutbox::new();

    let loc_signal = use_location(&mut cx, &outbox, 1000, 5.0);
    assert!(loc_signal.get().is_none());

    // Outbox should contain StartLocationUpdates from mount effect
    let queued = outbox.drain();
    assert_eq!(queued.len(), 1);
    assert_eq!(
        queued[0],
        PlatformRequest::StartLocationUpdates {
            min_interval_ms: 1000,
            min_distance_m: 5.0,
        }
    );

    // Route location update
    route_platform_response(
        &PlatformResponse::LocationUpdate {
            latitude: 37.7749,
            longitude: -122.4194,
            accuracy: 3.5,
            timestamp: 123456789,
        },
        None,
        Some(&loc_signal),
        None,
    );

    let loc = loc_signal.get().expect("Expected LocationData");
    assert_eq!(loc.latitude, 37.7749);
    assert_eq!(loc.accuracy, 3.5);

    // Trigger component unmount cleanup
    cx.unmount();

    // Outbox must contain StopLocationUpdates!
    let teardown_queued = outbox.drain();
    assert_eq!(teardown_queued.len(), 1);
    assert_eq!(teardown_queued[0], PlatformRequest::StopLocationUpdates);
}

#[test]
fn test_camera_hook_lifecycle_and_cancellation() {
    let mut cx = ComponentContext::new();
    let outbox = PlatformOutbox::new();
    let target_node = NodeId::new(42);

    let cam_signal = use_camera(&mut cx, &outbox, target_node);
    assert_eq!(cam_signal.get(), CameraState::Inactive);

    // Outbox should have StartCameraPreview
    let queued = outbox.drain();
    assert_eq!(queued.len(), 1);
    assert_eq!(
        queued[0],
        PlatformRequest::StartCameraPreview {
            target_node_id: target_node
        }
    );

    // Route CameraReady
    route_platform_response(
        &PlatformResponse::CameraReady {
            target_node_id: target_node,
            preview_width: 1280,
            preview_height: 720,
        },
        None,
        None,
        Some(&cam_signal),
    );

    assert_eq!(
        cam_signal.get(),
        CameraState::Active {
            width: 1280,
            height: 720
        }
    );

    // Unmount
    cx.unmount();

    // Outbox must contain StopCamera
    let teardown = outbox.drain();
    assert_eq!(teardown.len(), 1);
    assert_eq!(teardown[0], PlatformRequest::StopCamera);
}
