//! `ferrix-audio`: Audio playback, streaming, and background audio state.

use crate::registry::{ModuleCall, ModuleId, NativeModule};

pub struct AudioModule;

impl NativeModule for AudioModule {
    fn id(&self) -> ModuleId {
        ModuleId::Audio
    }

    fn name(&self) -> &'static str {
        "ferrix-audio"
    }

    fn required_permissions(&self) -> &'static [&'static str] {
        &["android.permission.MODIFY_AUDIO_SETTINGS"]
    }
}

impl AudioModule {
    pub fn play_call(call_id: u32, uri: &str, volume: f32, loop_audio: bool) -> ModuleCall {
        let mut payload = Vec::new();
        let bytes = uri.as_bytes();
        payload.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(bytes);
        payload.extend_from_slice(&volume.to_le_bytes());
        payload.push(if loop_audio { 1 } else { 0 });

        ModuleCall {
            call_id,
            module_id: ModuleId::Audio as u16,
            method_id: 1, // play
            payload,
        }
    }

    pub fn pause_call(call_id: u32) -> ModuleCall {
        ModuleCall {
            call_id,
            module_id: ModuleId::Audio as u16,
            method_id: 2, // pause
            payload: Vec::new(),
        }
    }

    pub fn stop_call(call_id: u32) -> ModuleCall {
        ModuleCall {
            call_id,
            module_id: ModuleId::Audio as u16,
            method_id: 3, // stop
            payload: Vec::new(),
        }
    }
}
