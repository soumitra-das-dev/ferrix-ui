//! `ferrix-local-authentication`: BiometricPrompt hardware checking and authentication.

use crate::registry::{ModuleCall, ModuleId, NativeModule};

pub struct LocalAuthModule;

impl NativeModule for LocalAuthModule {
    fn id(&self) -> ModuleId {
        ModuleId::LocalAuth
    }

    fn name(&self) -> &'static str {
        "ferrix-local-authentication"
    }

    fn required_permissions(&self) -> &'static [&'static str] {
        &["android.permission.USE_BIOMETRIC"]
    }
}

impl LocalAuthModule {
    pub fn has_hardware_call(call_id: u32) -> ModuleCall {
        ModuleCall {
            call_id,
            module_id: ModuleId::LocalAuth as u16,
            method_id: 1, // has_hardware
            payload: Vec::new(),
        }
    }

    pub fn is_enrolled_call(call_id: u32) -> ModuleCall {
        ModuleCall {
            call_id,
            module_id: ModuleId::LocalAuth as u16,
            method_id: 2, // is_enrolled
            payload: Vec::new(),
        }
    }

    pub fn authenticate_call(call_id: u32, prompt_message: &str) -> ModuleCall {
        let mut payload = Vec::new();
        let bytes = prompt_message.as_bytes();
        payload.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(bytes);

        ModuleCall {
            call_id,
            module_id: ModuleId::LocalAuth as u16,
            method_id: 3, // authenticate
            payload,
        }
    }
}
