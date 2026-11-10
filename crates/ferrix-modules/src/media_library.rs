//! `ferrix-media-library`: Android MediaStore queries for photos, albums, and video assets.

use crate::registry::{ModuleCall, ModuleId, NativeModule};

pub struct MediaLibraryModule;

impl NativeModule for MediaLibraryModule {
    fn id(&self) -> ModuleId {
        ModuleId::MediaLibrary
    }

    fn name(&self) -> &'static str {
        "ferrix-media-library"
    }

    fn required_permissions(&self) -> &'static [&'static str] {
        &[
            "android.permission.READ_MEDIA_IMAGES",
            "android.permission.READ_MEDIA_VIDEO",
        ]
    }
}

impl MediaLibraryModule {
    pub fn get_assets_call(call_id: u32, limit: u32) -> ModuleCall {
        ModuleCall {
            call_id,
            module_id: ModuleId::MediaLibrary as u16,
            method_id: 1, // get_assets
            payload: limit.to_le_bytes().to_vec(),
        }
    }

    pub fn save_to_library_call(call_id: u32, local_uri: &str) -> ModuleCall {
        let mut payload = Vec::new();
        let bytes = local_uri.as_bytes();
        payload.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(bytes);

        ModuleCall {
            call_id,
            module_id: ModuleId::MediaLibrary as u16,
            method_id: 2, // save_to_library
            payload,
        }
    }
}
