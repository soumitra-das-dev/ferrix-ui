//! Compile-time static native module registry, typed call dispatch, and cancellation tokens.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// Unique identifier for each statically registered native module.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ModuleId {
    Camera = 1,
    Location = 2,
    Sensors = 3,
    Audio = 4,
    Haptics = 5,
    SecureStore = 6,
    FileSystem = 7,
    ImagePicker = 8,
    DocumentPicker = 9,
    MediaLibrary = 10,
    Notifications = 11,
    LocalAuth = 12,
}

impl ModuleId {
    pub fn from_raw(raw: u16) -> Option<Self> {
        match raw {
            1 => Some(Self::Camera),
            2 => Some(Self::Location),
            3 => Some(Self::Sensors),
            4 => Some(Self::Audio),
            5 => Some(Self::Haptics),
            6 => Some(Self::SecureStore),
            7 => Some(Self::FileSystem),
            8 => Some(Self::ImagePicker),
            9 => Some(Self::DocumentPicker),
            10 => Some(Self::MediaLibrary),
            11 => Some(Self::Notifications),
            12 => Some(Self::LocalAuth),
            _ => None,
        }
    }
}

/// Token passed to asynchronous module invocations to support non-blocking cancellation.
#[derive(Clone, Default, Debug)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }
}

/// Outbound typed binary call to a native module method.
#[derive(Clone, Debug, PartialEq)]
pub struct ModuleCall {
    pub call_id: u32,
    pub module_id: u16,
    pub method_id: u16,
    pub payload: Vec<u8>,
}

impl ModuleCall {
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + self.payload.len());
        buf.extend_from_slice(&self.call_id.to_le_bytes());
        buf.extend_from_slice(&self.module_id.to_le_bytes());
        buf.extend_from_slice(&self.method_id.to_le_bytes());
        buf.extend_from_slice(&(self.payload.len() as u32).to_le_bytes());
        buf.extend_from_slice(&self.payload);
        buf
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 12 {
            return None;
        }
        let call_id = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let module_id = u16::from_le_bytes(bytes[4..6].try_into().unwrap());
        let method_id = u16::from_le_bytes(bytes[6..8].try_into().unwrap());
        let len = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
        if bytes.len() < 12 + len {
            return None;
        }
        let payload = bytes[12..12 + len].to_vec();
        Some(Self {
            call_id,
            module_id,
            method_id,
            payload,
        })
    }
}

/// Inbound result from a native module call.
#[derive(Clone, Debug, PartialEq)]
pub enum ModuleResult {
    Success {
        call_id: u32,
        payload: Vec<u8>,
    },
    Error {
        call_id: u32,
        code: String,
        message: String,
    },
    Cancelled {
        call_id: u32,
    },
}

impl ModuleResult {
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match self {
            Self::Success { call_id, payload } => {
                buf.extend_from_slice(&1u16.to_le_bytes());
                buf.extend_from_slice(&call_id.to_le_bytes());
                buf.extend_from_slice(&(payload.len() as u32).to_le_bytes());
                buf.extend_from_slice(payload);
            }
            Self::Error {
                call_id,
                code,
                message,
            } => {
                buf.extend_from_slice(&2u16.to_le_bytes());
                buf.extend_from_slice(&call_id.to_le_bytes());
                let c_bytes = code.as_bytes();
                buf.extend_from_slice(&(c_bytes.len() as u32).to_le_bytes());
                buf.extend_from_slice(c_bytes);
                let m_bytes = message.as_bytes();
                buf.extend_from_slice(&(m_bytes.len() as u32).to_le_bytes());
                buf.extend_from_slice(m_bytes);
            }
            Self::Cancelled { call_id } => {
                buf.extend_from_slice(&3u16.to_le_bytes());
                buf.extend_from_slice(&call_id.to_le_bytes());
            }
        }
        buf
    }

    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 6 {
            return None;
        }
        let tag = u16::from_le_bytes(bytes[0..2].try_into().unwrap());
        let call_id = u32::from_le_bytes(bytes[2..6].try_into().unwrap());
        match tag {
            1 => {
                if bytes.len() < 10 {
                    return None;
                }
                let len = u32::from_le_bytes(bytes[6..10].try_into().unwrap()) as usize;
                if bytes.len() < 10 + len {
                    return None;
                }
                Some(Self::Success {
                    call_id,
                    payload: bytes[10..10 + len].to_vec(),
                })
            }
            2 => {
                let mut cursor = 6;
                if bytes.len() < cursor + 4 {
                    return None;
                }
                let c_len =
                    u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
                cursor += 4;
                if bytes.len() < cursor + c_len {
                    return None;
                }
                let code = std::str::from_utf8(&bytes[cursor..cursor + c_len])
                    .ok()?
                    .to_string();
                cursor += c_len;

                if bytes.len() < cursor + 4 {
                    return None;
                }
                let m_len =
                    u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
                cursor += 4;
                if bytes.len() < cursor + m_len {
                    return None;
                }
                let message = std::str::from_utf8(&bytes[cursor..cursor + m_len])
                    .ok()?
                    .to_string();

                Some(Self::Error {
                    call_id,
                    code,
                    message,
                })
            }
            3 => Some(Self::Cancelled { call_id }),
            _ => None,
        }
    }
}

/// Common contract for all native modules.
pub trait NativeModule: Send + Sync {
    fn id(&self) -> ModuleId;
    fn name(&self) -> &'static str;
    fn required_permissions(&self) -> &'static [&'static str] {
        &[]
    }
    fn on_start(&self) {}
    fn on_stop(&self) {}
    fn on_destroy(&self) {}
}

/// Static, reflection-free module registry.
#[derive(Default)]
pub struct StaticModuleRegistry {
    modules: Mutex<Vec<Arc<dyn NativeModule>>>,
}

impl StaticModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: Mutex::new(Vec::new()),
        }
    }

    pub fn register(&self, module: Arc<dyn NativeModule>) {
        self.modules.lock().unwrap().push(module);
    }

    pub fn get(&self, id: ModuleId) -> Option<Arc<dyn NativeModule>> {
        self.modules
            .lock()
            .unwrap()
            .iter()
            .find(|m| m.id() == id)
            .cloned()
    }

    pub fn count(&self) -> usize {
        self.modules.lock().unwrap().len()
    }

    pub fn broadcast_start(&self) {
        for m in self.modules.lock().unwrap().iter() {
            m.on_start();
        }
    }

    pub fn broadcast_stop(&self) {
        for m in self.modules.lock().unwrap().iter() {
            m.on_stop();
        }
    }

    pub fn broadcast_destroy(&self) {
        for m in self.modules.lock().unwrap().iter() {
            m.on_destroy();
        }
    }
}
