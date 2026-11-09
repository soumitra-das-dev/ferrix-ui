//! `ferrix-file-system`: Scoped Storage, SAF, and file I/O operations.

use crate::registry::{ModuleCall, ModuleId, NativeModule};

pub struct FileSystemModule;

impl NativeModule for FileSystemModule {
    fn id(&self) -> ModuleId {
        ModuleId::FileSystem
    }

    fn name(&self) -> &'static str {
        "ferrix-file-system"
    }
}

impl FileSystemModule {
    pub fn read_as_string_call(call_id: u32, path: &str) -> ModuleCall {
        let mut payload = Vec::new();
        let bytes = path.as_bytes();
        payload.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(bytes);

        ModuleCall {
            call_id,
            module_id: ModuleId::FileSystem as u16,
            method_id: 1, // read_as_string
            payload,
        }
    }

    pub fn write_as_string_call(call_id: u32, path: &str, contents: &str) -> ModuleCall {
        let mut payload = Vec::new();
        let p_bytes = path.as_bytes();
        payload.extend_from_slice(&(p_bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(p_bytes);
        let c_bytes = contents.as_bytes();
        payload.extend_from_slice(&(c_bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(c_bytes);

        ModuleCall {
            call_id,
            module_id: ModuleId::FileSystem as u16,
            method_id: 2, // write_as_string
            payload,
        }
    }

    pub fn delete_call(call_id: u32, path: &str) -> ModuleCall {
        let mut payload = Vec::new();
        let bytes = path.as_bytes();
        payload.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(bytes);

        ModuleCall {
            call_id,
            module_id: ModuleId::FileSystem as u16,
            method_id: 3, // delete
            payload,
        }
    }

    pub fn read_directory_call(call_id: u32, path: &str) -> ModuleCall {
        let mut payload = Vec::new();
        let bytes = path.as_bytes();
        payload.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(bytes);

        ModuleCall {
            call_id,
            module_id: ModuleId::FileSystem as u16,
            method_id: 4, // read_directory
            payload,
        }
    }
}
