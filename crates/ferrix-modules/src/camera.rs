//! `ferrix-camera`: CameraX integration, barcode scanning, HDR, and picture capture.

use crate::registry::{ModuleCall, ModuleId, NativeModule};

pub struct CameraModule;

impl NativeModule for CameraModule {
    fn id(&self) -> ModuleId {
        ModuleId::Camera
    }

    fn name(&self) -> &'static str {
        "ferrix-camera"
    }

    fn required_permissions(&self) -> &'static [&'static str] {
        &["android.permission.CAMERA"]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CameraCaptureOptions {
    pub quality: f32,
    pub enable_hdr: bool,
    pub front_camera: bool,
}

impl CameraCaptureOptions {
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(9);
        buf.extend_from_slice(&self.quality.to_le_bytes());
        buf.push(if self.enable_hdr { 1 } else { 0 });
        buf.push(if self.front_camera { 1 } else { 0 });
        buf
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 6 {
            return None;
        }
        let quality = f32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let enable_hdr = bytes[4] != 0;
        let front_camera = bytes[5] != 0;
        Some(Self {
            quality,
            enable_hdr,
            front_camera,
        })
    }
}

impl CameraModule {
    pub fn take_picture_call(call_id: u32, options: CameraCaptureOptions) -> ModuleCall {
        ModuleCall {
            call_id,
            module_id: ModuleId::Camera as u16,
            method_id: 1, // take_picture
            payload: options.encode(),
        }
    }

    pub fn scan_barcode_call(call_id: u32) -> ModuleCall {
        ModuleCall {
            call_id,
            module_id: ModuleId::Camera as u16,
            method_id: 2, // scan_barcode
            payload: Vec::new(),
        }
    }
}
