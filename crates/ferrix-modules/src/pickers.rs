//! `ferrix-image-picker` and `ferrix-document-picker`: System pickers with MIME filtering.

use crate::registry::{ModuleCall, ModuleId, NativeModule};

pub struct ImagePickerModule;

impl NativeModule for ImagePickerModule {
    fn id(&self) -> ModuleId {
        ModuleId::ImagePicker
    }

    fn name(&self) -> &'static str {
        "ferrix-image-picker"
    }

    fn required_permissions(&self) -> &'static [&'static str] {
        &["android.permission.READ_MEDIA_IMAGES"]
    }
}

impl ImagePickerModule {
    pub fn launch_library_call(call_id: u32, allows_editing: bool, quality: f32) -> ModuleCall {
        let mut payload = Vec::with_capacity(5);
        payload.push(if allows_editing { 1 } else { 0 });
        payload.extend_from_slice(&quality.to_le_bytes());

        ModuleCall {
            call_id,
            module_id: ModuleId::ImagePicker as u16,
            method_id: 1, // launch_image_library
            payload,
        }
    }
}

pub struct DocumentPickerModule;

impl NativeModule for DocumentPickerModule {
    fn id(&self) -> ModuleId {
        ModuleId::DocumentPicker
    }

    fn name(&self) -> &'static str {
        "ferrix-document-picker"
    }
}

impl DocumentPickerModule {
    pub fn get_document_call(call_id: u32, mime_type: &str) -> ModuleCall {
        let mut payload = Vec::new();
        let bytes = mime_type.as_bytes();
        payload.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(bytes);

        ModuleCall {
            call_id,
            module_id: ModuleId::DocumentPicker as u16,
            method_id: 1, // get_document_async
            payload,
        }
    }
}
