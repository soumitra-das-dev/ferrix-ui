//! `ferrix-secure-store`: Hardware Keystore backed EncryptedSharedPreferences.

use crate::registry::{ModuleCall, ModuleId, NativeModule};

pub struct SecureStoreModule;

impl NativeModule for SecureStoreModule {
    fn id(&self) -> ModuleId {
        ModuleId::SecureStore
    }

    fn name(&self) -> &'static str {
        "ferrix-secure-store"
    }
}

impl SecureStoreModule {
    pub fn set_item_call(call_id: u32, key: &str, value: &str) -> ModuleCall {
        let mut payload = Vec::new();
        let k_bytes = key.as_bytes();
        payload.extend_from_slice(&(k_bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(k_bytes);
        let v_bytes = value.as_bytes();
        payload.extend_from_slice(&(v_bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(v_bytes);

        ModuleCall {
            call_id,
            module_id: ModuleId::SecureStore as u16,
            method_id: 1, // set_item
            payload,
        }
    }

    pub fn get_item_call(call_id: u32, key: &str) -> ModuleCall {
        let mut payload = Vec::new();
        let k_bytes = key.as_bytes();
        payload.extend_from_slice(&(k_bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(k_bytes);

        ModuleCall {
            call_id,
            module_id: ModuleId::SecureStore as u16,
            method_id: 2, // get_item
            payload,
        }
    }

    pub fn delete_item_call(call_id: u32, key: &str) -> ModuleCall {
        let mut payload = Vec::new();
        let k_bytes = key.as_bytes();
        payload.extend_from_slice(&(k_bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(k_bytes);

        ModuleCall {
            call_id,
            module_id: ModuleId::SecureStore as u16,
            method_id: 3, // delete_item
            payload,
        }
    }
}
