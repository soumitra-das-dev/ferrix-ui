//! Typed binary protocol for native platform capabilities (Permissions, Location, Camera, Storage).

use crate::{NodeId, ProtocolError};

pub const PLATFORM_MAGIC: [u8; 4] = [0x46, 0x58, 0x50, 0x4C]; // "FXPL"
pub const PLATFORM_VERSION: u16 = 1;

/// Request sent from Rust to native Android adapters.
#[derive(Clone, Debug, PartialEq)]
pub enum PlatformRequest {
    RequestPermission {
        request_id: u32,
        permission: String,
    },
    StartLocationUpdates {
        min_interval_ms: u64,
        min_distance_m: f32,
    },
    StopLocationUpdates,
    StartCameraPreview {
        target_node_id: NodeId,
    },
    StopCamera,
    TakePicture {
        request_id: u32,
    },
    ReadStorage {
        request_id: u32,
        key: String,
    },
    WriteStorage {
        request_id: u32,
        key: String,
        value: String,
    },
}

impl PlatformRequest {
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match self {
            Self::RequestPermission {
                request_id,
                permission,
            } => {
                buf.extend_from_slice(&1u16.to_le_bytes());
                buf.extend_from_slice(&request_id.to_le_bytes());
                let bytes = permission.as_bytes();
                buf.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
                buf.extend_from_slice(bytes);
            }
            Self::StartLocationUpdates {
                min_interval_ms,
                min_distance_m,
            } => {
                buf.extend_from_slice(&2u16.to_le_bytes());
                buf.extend_from_slice(&min_interval_ms.to_le_bytes());
                buf.extend_from_slice(&min_distance_m.to_le_bytes());
            }
            Self::StopLocationUpdates => {
                buf.extend_from_slice(&3u16.to_le_bytes());
            }
            Self::StartCameraPreview { target_node_id } => {
                buf.extend_from_slice(&4u16.to_le_bytes());
                buf.extend_from_slice(&target_node_id.raw().to_le_bytes());
            }
            Self::StopCamera => {
                buf.extend_from_slice(&5u16.to_le_bytes());
            }
            Self::TakePicture { request_id } => {
                buf.extend_from_slice(&6u16.to_le_bytes());
                buf.extend_from_slice(&request_id.to_le_bytes());
            }
            Self::ReadStorage { request_id, key } => {
                buf.extend_from_slice(&7u16.to_le_bytes());
                buf.extend_from_slice(&request_id.to_le_bytes());
                let bytes = key.as_bytes();
                buf.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
                buf.extend_from_slice(bytes);
            }
            Self::WriteStorage {
                request_id,
                key,
                value,
            } => {
                buf.extend_from_slice(&8u16.to_le_bytes());
                buf.extend_from_slice(&request_id.to_le_bytes());
                let k_bytes = key.as_bytes();
                buf.extend_from_slice(&(k_bytes.len() as u32).to_le_bytes());
                buf.extend_from_slice(k_bytes);
                let v_bytes = value.as_bytes();
                buf.extend_from_slice(&(v_bytes.len() as u32).to_le_bytes());
                buf.extend_from_slice(v_bytes);
            }
        }
        buf
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, ProtocolError> {
        if bytes.len() < 2 {
            return Err(ProtocolError::UnexpectedEof {
                offset: bytes.len(),
                expected: 2,
            });
        }
        let tag = u16::from_le_bytes(bytes[0..2].try_into().unwrap());
        match tag {
            1 => {
                if bytes.len() < 10 {
                    return Err(ProtocolError::UnexpectedEof {
                        offset: bytes.len(),
                        expected: 10,
                    });
                }
                let request_id = u32::from_le_bytes(bytes[2..6].try_into().unwrap());
                let len = u32::from_le_bytes(bytes[6..10].try_into().unwrap()) as usize;
                if bytes.len() < 10 + len {
                    return Err(ProtocolError::UnexpectedEof {
                        offset: bytes.len(),
                        expected: 10 + len,
                    });
                }
                let permission = std::str::from_utf8(&bytes[10..10 + len])
                    .map_err(|_| ProtocolError::InvalidUtf8)?
                    .to_string();
                Ok(Self::RequestPermission {
                    request_id,
                    permission,
                })
            }
            2 => {
                if bytes.len() < 14 {
                    return Err(ProtocolError::UnexpectedEof {
                        offset: bytes.len(),
                        expected: 14,
                    });
                }
                let min_interval_ms = u64::from_le_bytes(bytes[2..10].try_into().unwrap());
                let min_distance_m = f32::from_le_bytes(bytes[10..14].try_into().unwrap());
                Ok(Self::StartLocationUpdates {
                    min_interval_ms,
                    min_distance_m,
                })
            }
            3 => Ok(Self::StopLocationUpdates),
            4 => {
                if bytes.len() < 10 {
                    return Err(ProtocolError::UnexpectedEof {
                        offset: bytes.len(),
                        expected: 10,
                    });
                }
                let node_id = NodeId::new(u64::from_le_bytes(bytes[2..10].try_into().unwrap()));
                Ok(Self::StartCameraPreview {
                    target_node_id: node_id,
                })
            }
            5 => Ok(Self::StopCamera),
            6 => {
                if bytes.len() < 6 {
                    return Err(ProtocolError::UnexpectedEof {
                        offset: bytes.len(),
                        expected: 6,
                    });
                }
                let request_id = u32::from_le_bytes(bytes[2..6].try_into().unwrap());
                Ok(Self::TakePicture { request_id })
            }
            7 => {
                if bytes.len() < 10 {
                    return Err(ProtocolError::UnexpectedEof {
                        offset: bytes.len(),
                        expected: 10,
                    });
                }
                let request_id = u32::from_le_bytes(bytes[2..6].try_into().unwrap());
                let len = u32::from_le_bytes(bytes[6..10].try_into().unwrap()) as usize;
                if bytes.len() < 10 + len {
                    return Err(ProtocolError::UnexpectedEof {
                        offset: bytes.len(),
                        expected: 10 + len,
                    });
                }
                let key = std::str::from_utf8(&bytes[10..10 + len])
                    .map_err(|_| ProtocolError::InvalidUtf8)?
                    .to_string();
                Ok(Self::ReadStorage { request_id, key })
            }
            8 => {
                let mut cursor = 2;
                if bytes.len() < cursor + 4 {
                    return Err(ProtocolError::UnexpectedEof {
                        offset: bytes.len(),
                        expected: cursor + 4,
                    });
                }
                let request_id = u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap());
                cursor += 4;

                let k_len =
                    u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
                cursor += 4;
                if bytes.len() < cursor + k_len {
                    return Err(ProtocolError::UnexpectedEof {
                        offset: bytes.len(),
                        expected: cursor + k_len,
                    });
                }
                let key = std::str::from_utf8(&bytes[cursor..cursor + k_len])
                    .map_err(|_| ProtocolError::InvalidUtf8)?
                    .to_string();
                cursor += k_len;

                let v_len =
                    u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
                cursor += 4;
                if bytes.len() < cursor + v_len {
                    return Err(ProtocolError::UnexpectedEof {
                        offset: bytes.len(),
                        expected: cursor + v_len,
                    });
                }
                let value = std::str::from_utf8(&bytes[cursor..cursor + v_len])
                    .map_err(|_| ProtocolError::InvalidUtf8)?
                    .to_string();

                Ok(Self::WriteStorage {
                    request_id,
                    key,
                    value,
                })
            }
            other => Err(ProtocolError::UnknownPlatformTag(other)),
        }
    }
}

/// Response or event sent from native Android adapters to Rust.
#[derive(Clone, Debug, PartialEq)]
pub enum PlatformResponse {
    PermissionResult {
        request_id: u32,
        granted: bool,
        should_show_rationale: bool,
    },
    LocationUpdate {
        latitude: f64,
        longitude: f64,
        accuracy: f32,
        timestamp: u64,
    },
    CameraReady {
        target_node_id: NodeId,
        preview_width: u32,
        preview_height: u32,
    },
    PictureTaken {
        request_id: u32,
        path: String,
        width: u32,
        height: u32,
    },
    StorageResult {
        request_id: u32,
        key: String,
        value: Option<String>,
    },
    Error {
        request_id: u32,
        message: String,
    },
}

impl PlatformResponse {
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        match self {
            Self::PermissionResult {
                request_id,
                granted,
                should_show_rationale,
            } => {
                buf.extend_from_slice(&1u16.to_le_bytes());
                buf.extend_from_slice(&request_id.to_le_bytes());
                buf.push(if *granted { 1 } else { 0 });
                buf.push(if *should_show_rationale { 1 } else { 0 });
            }
            Self::LocationUpdate {
                latitude,
                longitude,
                accuracy,
                timestamp,
            } => {
                buf.extend_from_slice(&2u16.to_le_bytes());
                buf.extend_from_slice(&latitude.to_le_bytes());
                buf.extend_from_slice(&longitude.to_le_bytes());
                buf.extend_from_slice(&accuracy.to_le_bytes());
                buf.extend_from_slice(&timestamp.to_le_bytes());
            }
            Self::CameraReady {
                target_node_id,
                preview_width,
                preview_height,
            } => {
                buf.extend_from_slice(&3u16.to_le_bytes());
                buf.extend_from_slice(&target_node_id.raw().to_le_bytes());
                buf.extend_from_slice(&preview_width.to_le_bytes());
                buf.extend_from_slice(&preview_height.to_le_bytes());
            }
            Self::PictureTaken {
                request_id,
                path,
                width,
                height,
            } => {
                buf.extend_from_slice(&4u16.to_le_bytes());
                buf.extend_from_slice(&request_id.to_le_bytes());
                buf.extend_from_slice(&width.to_le_bytes());
                buf.extend_from_slice(&height.to_le_bytes());
                let bytes = path.as_bytes();
                buf.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
                buf.extend_from_slice(bytes);
            }
            Self::StorageResult {
                request_id,
                key,
                value,
            } => {
                buf.extend_from_slice(&5u16.to_le_bytes());
                buf.extend_from_slice(&request_id.to_le_bytes());
                let k_bytes = key.as_bytes();
                buf.extend_from_slice(&(k_bytes.len() as u32).to_le_bytes());
                buf.extend_from_slice(k_bytes);
                if let Some(v) = value {
                    buf.push(1);
                    let v_bytes = v.as_bytes();
                    buf.extend_from_slice(&(v_bytes.len() as u32).to_le_bytes());
                    buf.extend_from_slice(v_bytes);
                } else {
                    buf.push(0);
                }
            }
            Self::Error {
                request_id,
                message,
            } => {
                buf.extend_from_slice(&6u16.to_le_bytes());
                buf.extend_from_slice(&request_id.to_le_bytes());
                let bytes = message.as_bytes();
                buf.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
                buf.extend_from_slice(bytes);
            }
        }
        buf
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, ProtocolError> {
        if bytes.len() < 2 {
            return Err(ProtocolError::UnexpectedEof {
                offset: bytes.len(),
                expected: 2,
            });
        }
        let tag = u16::from_le_bytes(bytes[0..2].try_into().unwrap());
        match tag {
            1 => {
                if bytes.len() < 8 {
                    return Err(ProtocolError::UnexpectedEof {
                        offset: bytes.len(),
                        expected: 8,
                    });
                }
                let request_id = u32::from_le_bytes(bytes[2..6].try_into().unwrap());
                let granted = bytes[6] != 0;
                let should_show_rationale = bytes[7] != 0;
                Ok(Self::PermissionResult {
                    request_id,
                    granted,
                    should_show_rationale,
                })
            }
            2 => {
                if bytes.len() < 30 {
                    return Err(ProtocolError::UnexpectedEof {
                        offset: bytes.len(),
                        expected: 30,
                    });
                }
                let latitude = f64::from_le_bytes(bytes[2..10].try_into().unwrap());
                let longitude = f64::from_le_bytes(bytes[10..18].try_into().unwrap());
                let accuracy = f32::from_le_bytes(bytes[18..22].try_into().unwrap());
                let timestamp = u64::from_le_bytes(bytes[22..30].try_into().unwrap());
                Ok(Self::LocationUpdate {
                    latitude,
                    longitude,
                    accuracy,
                    timestamp,
                })
            }
            3 => {
                if bytes.len() < 18 {
                    return Err(ProtocolError::UnexpectedEof {
                        offset: bytes.len(),
                        expected: 18,
                    });
                }
                let target_node_id =
                    NodeId::new(u64::from_le_bytes(bytes[2..10].try_into().unwrap()));
                let preview_width = u32::from_le_bytes(bytes[10..14].try_into().unwrap());
                let preview_height = u32::from_le_bytes(bytes[14..18].try_into().unwrap());
                Ok(Self::CameraReady {
                    target_node_id,
                    preview_width,
                    preview_height,
                })
            }
            4 => {
                if bytes.len() < 18 {
                    return Err(ProtocolError::UnexpectedEof {
                        offset: bytes.len(),
                        expected: 18,
                    });
                }
                let request_id = u32::from_le_bytes(bytes[2..6].try_into().unwrap());
                let width = u32::from_le_bytes(bytes[6..10].try_into().unwrap());
                let height = u32::from_le_bytes(bytes[10..14].try_into().unwrap());
                let len = u32::from_le_bytes(bytes[14..18].try_into().unwrap()) as usize;
                if bytes.len() < 18 + len {
                    return Err(ProtocolError::UnexpectedEof {
                        offset: bytes.len(),
                        expected: 18 + len,
                    });
                }
                let path = std::str::from_utf8(&bytes[18..18 + len])
                    .map_err(|_| ProtocolError::InvalidUtf8)?
                    .to_string();
                Ok(Self::PictureTaken {
                    request_id,
                    path,
                    width,
                    height,
                })
            }
            5 => {
                let mut cursor = 2;
                if bytes.len() < cursor + 4 {
                    return Err(ProtocolError::UnexpectedEof {
                        offset: bytes.len(),
                        expected: cursor + 4,
                    });
                }
                let request_id = u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap());
                cursor += 4;

                let k_len =
                    u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
                cursor += 4;
                if bytes.len() < cursor + k_len {
                    return Err(ProtocolError::UnexpectedEof {
                        offset: bytes.len(),
                        expected: cursor + k_len,
                    });
                }
                let key = std::str::from_utf8(&bytes[cursor..cursor + k_len])
                    .map_err(|_| ProtocolError::InvalidUtf8)?
                    .to_string();
                cursor += k_len;

                let has_val = bytes[cursor] != 0;
                cursor += 1;
                let value = if has_val {
                    let v_len =
                        u32::from_le_bytes(bytes[cursor..cursor + 4].try_into().unwrap()) as usize;
                    cursor += 4;
                    if bytes.len() < cursor + v_len {
                        return Err(ProtocolError::UnexpectedEof {
                            offset: bytes.len(),
                            expected: cursor + v_len,
                        });
                    }
                    Some(
                        std::str::from_utf8(&bytes[cursor..cursor + v_len])
                            .map_err(|_| ProtocolError::InvalidUtf8)?
                            .to_string(),
                    )
                } else {
                    None
                };

                Ok(Self::StorageResult {
                    request_id,
                    key,
                    value,
                })
            }
            6 => {
                if bytes.len() < 10 {
                    return Err(ProtocolError::UnexpectedEof {
                        offset: bytes.len(),
                        expected: 10,
                    });
                }
                let request_id = u32::from_le_bytes(bytes[2..6].try_into().unwrap());
                let len = u32::from_le_bytes(bytes[6..10].try_into().unwrap()) as usize;
                if bytes.len() < 10 + len {
                    return Err(ProtocolError::UnexpectedEof {
                        offset: bytes.len(),
                        expected: 10 + len,
                    });
                }
                let message = std::str::from_utf8(&bytes[10..10 + len])
                    .map_err(|_| ProtocolError::InvalidUtf8)?
                    .to_string();
                Ok(Self::Error {
                    request_id,
                    message,
                })
            }
            other => Err(ProtocolError::UnknownPlatformTag(other)),
        }
    }
}
