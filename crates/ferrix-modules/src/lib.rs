//! # ferrix-modules
//!
//! Expo-style native modules with compile-time static registration, typed binary calls,
//! lifecycle management, permission gating, and async cancellation tokens.

pub mod audio;
pub mod camera;
pub mod file_system;
pub mod haptics;
pub mod local_auth;
pub mod location;
pub mod media_library;
pub mod notifications;
pub mod pickers;
pub mod registry;
pub mod secure_store;
pub mod sensors;

pub use audio::AudioModule;
pub use camera::{CameraCaptureOptions, CameraModule};
pub use file_system::FileSystemModule;
pub use haptics::{HapticsModule, ImpactFeedbackStyle, NotificationFeedbackType};
pub use local_auth::LocalAuthModule;
pub use location::LocationModule;
pub use media_library::MediaLibraryModule;
pub use notifications::NotificationsModule;
pub use pickers::{DocumentPickerModule, ImagePickerModule};
pub use registry::{
    CancellationToken, ModuleCall, ModuleId, ModuleResult, NativeModule, StaticModuleRegistry,
};
pub use secure_store::SecureStoreModule;
pub use sensors::{SensorType, SensorsModule, ThreeAxisData};

/// Returns a default statically-initialized module registry populated with all core Expo-style modules.
pub fn create_default_registry() -> StaticModuleRegistry {
    let registry = StaticModuleRegistry::new();
    registry.register(std::sync::Arc::new(CameraModule));
    registry.register(std::sync::Arc::new(LocationModule));
    registry.register(std::sync::Arc::new(SensorsModule));
    registry.register(std::sync::Arc::new(AudioModule));
    registry.register(std::sync::Arc::new(HapticsModule));
    registry.register(std::sync::Arc::new(SecureStoreModule));
    registry.register(std::sync::Arc::new(FileSystemModule));
    registry.register(std::sync::Arc::new(ImagePickerModule));
    registry.register(std::sync::Arc::new(DocumentPickerModule));
    registry.register(std::sync::Arc::new(MediaLibraryModule));
    registry.register(std::sync::Arc::new(NotificationsModule));
    registry.register(std::sync::Arc::new(LocalAuthModule));
    registry
}
