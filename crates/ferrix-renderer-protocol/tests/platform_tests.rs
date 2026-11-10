use ferrix_renderer_protocol::platform::{PlatformRequest, PlatformResponse};
use ferrix_renderer_protocol::NodeId;

#[test]
fn test_platform_request_encode_decode() {
    let req = PlatformRequest::RequestPermission {
        request_id: 42,
        permission: "android.permission.ACCESS_FINE_LOCATION".to_string(),
    };
    let bytes = req.encode();
    let decoded = PlatformRequest::decode(&bytes).expect("Failed to decode RequestPermission");
    assert_eq!(req, decoded);

    let loc_req = PlatformRequest::StartLocationUpdates {
        min_interval_ms: 1000,
        min_distance_m: 5.5,
    };
    let bytes = loc_req.encode();
    let decoded = PlatformRequest::decode(&bytes).expect("Failed to decode StartLocationUpdates");
    assert_eq!(loc_req, decoded);

    let cam_req = PlatformRequest::StartCameraPreview {
        target_node_id: NodeId::new(99),
    };
    let bytes = cam_req.encode();
    let decoded = PlatformRequest::decode(&bytes).expect("Failed to decode StartCameraPreview");
    assert_eq!(cam_req, decoded);

    let storage_req = PlatformRequest::WriteStorage {
        request_id: 101,
        key: "user_name".to_string(),
        value: "Ferrix Developer".to_string(),
    };
    let bytes = storage_req.encode();
    let decoded = PlatformRequest::decode(&bytes).expect("Failed to decode WriteStorage");
    assert_eq!(storage_req, decoded);
}

#[test]
fn test_platform_response_encode_decode() {
    let resp = PlatformResponse::PermissionResult {
        request_id: 42,
        granted: true,
        should_show_rationale: false,
    };
    let bytes = resp.encode();
    let decoded = PlatformResponse::decode(&bytes).expect("Failed to decode PermissionResult");
    assert_eq!(resp, decoded);

    let loc_resp = PlatformResponse::LocationUpdate {
        latitude: 37.7749,
        longitude: -122.4194,
        accuracy: 4.2,
        timestamp: 1700000000000,
    };
    let bytes = loc_resp.encode();
    let decoded = PlatformResponse::decode(&bytes).expect("Failed to decode LocationUpdate");
    assert_eq!(loc_resp, decoded);

    let pic_resp = PlatformResponse::PictureTaken {
        request_id: 7,
        path: "/data/user/0/com.ferrix/files/photo.jpg".to_string(),
        width: 1920,
        height: 1080,
    };
    let bytes = pic_resp.encode();
    let decoded = PlatformResponse::decode(&bytes).expect("Failed to decode PictureTaken");
    assert_eq!(pic_resp, decoded);

    let store_resp = PlatformResponse::StorageResult {
        request_id: 101,
        key: "user_name".to_string(),
        value: Some("Ferrix Developer".to_string()),
    };
    let bytes = store_resp.encode();
    let decoded = PlatformResponse::decode(&bytes).expect("Failed to decode StorageResult");
    assert_eq!(store_resp, decoded);
}
