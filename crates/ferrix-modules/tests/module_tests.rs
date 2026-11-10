//! Unit & integration tests for Expo-style native module registry and typed calls.

use ferrix_modules::*;

#[test]
fn test_default_module_registry() {
    let registry = create_default_registry();
    assert_eq!(registry.count(), 12);

    let camera = registry
        .get(ModuleId::Camera)
        .expect("Camera module registered");
    assert_eq!(camera.name(), "ferrix-camera");
    assert_eq!(
        camera.required_permissions(),
        &["android.permission.CAMERA"]
    );

    let location = registry
        .get(ModuleId::Location)
        .expect("Location module registered");
    assert_eq!(location.name(), "ferrix-location");
    assert!(location
        .required_permissions()
        .contains(&"android.permission.ACCESS_FINE_LOCATION"));

    let haptics = registry
        .get(ModuleId::Haptics)
        .expect("Haptics module registered");
    assert_eq!(haptics.name(), "ferrix-haptics");

    let secure_store = registry
        .get(ModuleId::SecureStore)
        .expect("SecureStore registered");
    assert_eq!(secure_store.name(), "ferrix-secure-store");

    let auth = registry
        .get(ModuleId::LocalAuth)
        .expect("LocalAuth registered");
    assert_eq!(auth.name(), "ferrix-local-authentication");

    // Lifecycle broadcast
    registry.broadcast_start();
    registry.broadcast_stop();
    registry.broadcast_destroy();
}

#[test]
fn test_cancellation_token() {
    let token = CancellationToken::new();
    assert!(!token.is_cancelled());

    let token_clone = token.clone();
    token_clone.cancel();
    assert!(token.is_cancelled());
}

#[test]
fn test_module_call_roundtrip() {
    let call = ModuleCall {
        call_id: 101,
        module_id: ModuleId::Camera as u16,
        method_id: 1,
        payload: vec![1, 2, 3, 4, 5],
    };

    let encoded = call.encode();
    let decoded = ModuleCall::decode(&encoded).expect("Decode should succeed");
    assert_eq!(call, decoded);
}

#[test]
fn test_module_result_roundtrip() {
    let results = vec![
        ModuleResult::Success {
            call_id: 1,
            payload: vec![0xCA, 0xFE, 0xBA, 0xBE],
        },
        ModuleResult::Error {
            call_id: 2,
            code: "ERR_PERMISSION_DENIED".to_string(),
            message: "User rejected biometric prompt".to_string(),
        },
        ModuleResult::Cancelled { call_id: 3 },
    ];

    for res in results {
        let encoded = res.encode();
        let decoded = ModuleResult::decode(&encoded).expect("Decode should succeed");
        assert_eq!(res, decoded);
    }
}

#[test]
fn test_typed_module_builders() {
    // Camera
    let cam_call = CameraModule::take_picture_call(
        1,
        CameraCaptureOptions {
            quality: 0.85,
            enable_hdr: true,
            front_camera: false,
        },
    );
    assert_eq!(cam_call.module_id, ModuleId::Camera as u16);
    assert_eq!(cam_call.method_id, 1);

    // Location
    let loc_call = LocationModule::start_geofence_call(2, "office", 37.7749, -122.4194, 200.0);
    assert_eq!(loc_call.module_id, ModuleId::Location as u16);
    assert_eq!(loc_call.method_id, 2);

    // Sensors
    let sensor_call = SensorsModule::start_sensor_call(3, SensorType::Accelerometer, 50);
    assert_eq!(sensor_call.module_id, ModuleId::Sensors as u16);

    // Audio
    let audio_call = AudioModule::play_call(4, "https://ferrix.dev/sound.mp3", 0.75, false);
    assert_eq!(audio_call.module_id, ModuleId::Audio as u16);

    // Haptics
    let haptic_call = HapticsModule::impact_call(5, ImpactFeedbackStyle::Heavy);
    assert_eq!(haptic_call.module_id, ModuleId::Haptics as u16);

    // SecureStore
    let store_call = SecureStoreModule::set_item_call(6, "user_token", "secret_jwt_xyz");
    assert_eq!(store_call.module_id, ModuleId::SecureStore as u16);

    // FileSystem
    let fs_call = FileSystemModule::write_as_string_call(7, "/data/notes.txt", "Ferrix File IO");
    assert_eq!(fs_call.module_id, ModuleId::FileSystem as u16);

    // Pickers
    let picker_call = ImagePickerModule::launch_library_call(8, true, 0.9);
    assert_eq!(picker_call.module_id, ModuleId::ImagePicker as u16);

    let doc_call = DocumentPickerModule::get_document_call(9, "application/pdf");
    assert_eq!(doc_call.module_id, ModuleId::DocumentPicker as u16);

    // MediaLibrary
    let media_call = MediaLibraryModule::get_assets_call(10, 50);
    assert_eq!(media_call.module_id, ModuleId::MediaLibrary as u16);

    // Notifications
    let notif_call = NotificationsModule::schedule_notification_call(
        11,
        "Reminder",
        "Check Ferrix updates",
        5000,
    );
    assert_eq!(notif_call.module_id, ModuleId::Notifications as u16);

    // LocalAuth
    let auth_call = LocalAuthModule::authenticate_call(12, "Unlock Ferrix App");
    assert_eq!(auth_call.module_id, ModuleId::LocalAuth as u16);
}
