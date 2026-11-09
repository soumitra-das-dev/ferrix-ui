//! # ferrix-android
//!
//! Android JNI glue and native lifecycle controller for Ferrix.
//! Exposes entry points to Kotlin runtime via packed binary patch buffers.

use std::panic::catch_unwind;
use std::sync::Mutex;

use ferrix_renderer_protocol::{EventType, NodeId, Patch, PatchBatch, Property, ViewType};
use jni::objects::JClass;
use jni::sys::{jbyteArray, jint, jlong, jobject};
use jni::JNIEnv;

/// Internal state managed on the Rust side of the Android bridge.
pub struct FerrixAndroidRuntime {
    pending_patches: Mutex<Option<Vec<u8>>>,
    text_counter: Mutex<u32>,
}

impl FerrixAndroidRuntime {
    pub fn new() -> Self {
        let mut initial_batch = PatchBatch::new();

        // 1. Root container (Column)
        initial_batch.push(Patch::CreateNode {
            node_id: NodeId(1),
            view_type: ViewType::Column,
        });
        initial_batch.push(Patch::SetProperty {
            node_id: NodeId(1),
            property: Property::Padding {
                left: 16.0,
                top: 24.0,
                right: 16.0,
                bottom: 24.0,
            },
        });
        initial_batch.push(Patch::SetProperty {
            node_id: NodeId(1),
            property: Property::Spacing(12.0),
        });

        // 2. Native TextView rendered from Rust
        initial_batch.push(Patch::CreateNode {
            node_id: NodeId(2),
            view_type: ViewType::Text,
        });
        initial_batch.push(Patch::SetText {
            node_id: NodeId(2),
            text: "Hello from Ferrix Native Rust!".to_string(),
        });
        initial_batch.push(Patch::SetProperty {
            node_id: NodeId(2),
            property: Property::FontSize(22.0),
        });

        // 3. Interactive Native Button
        initial_batch.push(Patch::CreateNode {
            node_id: NodeId(3),
            view_type: ViewType::Button,
        });
        initial_batch.push(Patch::SetText {
            node_id: NodeId(3),
            text: "Tap to mutate from Rust".to_string(),
        });
        initial_batch.push(Patch::AttachEventListener {
            node_id: NodeId(3),
            event_type: EventType::Click,
        });

        // 4. Hierarchy assembly
        initial_batch.push(Patch::AppendChild {
            parent_id: NodeId(1),
            child_id: NodeId(2),
        });
        initial_batch.push(Patch::AppendChild {
            parent_id: NodeId(1),
            child_id: NodeId(3),
        });

        let encoded = initial_batch.encode();

        Self {
            pending_patches: Mutex::new(Some(encoded)),
            text_counter: Mutex::new(0),
        }
    }

    /// Handles an incoming UI event from Kotlin and queues any resulting patch updates.
    pub fn handle_event(&self, node_id: NodeId, event_type: EventType) -> Option<Vec<u8>> {
        if node_id == NodeId(3) && event_type == EventType::Click {
            let mut counter = self.text_counter.lock().unwrap();
            *counter += 1;

            let mut batch = PatchBatch::new();
            batch.push(Patch::SetText {
                node_id: NodeId(2),
                text: format!("Rust Reactive Count: {}", *counter),
            });
            return Some(batch.encode());
        }
        None
    }
}

impl Default for FerrixAndroidRuntime {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// JNI Export Functions
// com.ferrix.runtime.FerrixBridge
// ============================================================================

/// Initializes the Ferrix Rust runtime and returns a raw heap pointer as `jlong`.
#[no_mangle]
pub extern "system" fn Java_com_ferrix_runtime_FerrixBridge_nativeInit(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let result = catch_unwind(|| {
        let runtime = Box::new(FerrixAndroidRuntime::new());
        Box::into_raw(runtime) as jlong
    });

    result.unwrap_or(0)
}

/// Retrieves the initial patch command buffer as a `DirectByteBuffer`.
#[no_mangle]
pub extern "system" fn Java_com_ferrix_runtime_FerrixBridge_nativeGetInitialPatches(
    mut env: JNIEnv,
    _class: JClass,
    handle: jlong,
) -> jobject {
    let result = catch_unwind(move || {
        if handle == 0 {
            return std::ptr::null_mut();
        }

        // SAFETY: `handle` was allocated via Box::into_raw in `nativeInit`
        // and is guaranteed to be valid until `nativeDestroy`.
        let runtime = unsafe { &*(handle as *const FerrixAndroidRuntime) };
        let mut guard = runtime.pending_patches.lock().unwrap();

        if let Some(ref mut buf) = *guard {
            // SAFETY: `buf` slice pointer remains valid as long as `guard` holds the buffer in memory.
            let byte_buffer = unsafe { env.new_direct_byte_buffer(buf.as_mut_ptr(), buf.len()) };
            match byte_buffer {
                Ok(obj) => obj.into_raw(),
                Err(_) => std::ptr::null_mut(),
            }
        } else {
            std::ptr::null_mut()
        }
    });

    result.unwrap_or(std::ptr::null_mut())
}

/// Dispatches an event (click, text change) from Kotlin to Rust.
/// Returns a `DirectByteBuffer` with any immediate response patches, or null if none.
#[no_mangle]
pub extern "system" fn Java_com_ferrix_runtime_FerrixBridge_nativeDispatchEvent(
    mut env: JNIEnv,
    _class: JClass,
    handle: jlong,
    node_id: jlong,
    event_type: jint,
    _payload: jbyteArray,
) -> jobject {
    let result = catch_unwind(move || {
        if handle == 0 {
            return std::ptr::null_mut();
        }

        // SAFETY: `handle` is checked for zero and originates from `Box::into_raw`.
        let runtime = unsafe { &*(handle as *const FerrixAndroidRuntime) };
        let event_type_enum = match EventType::from_u16(event_type as u16) {
            Some(et) => et,
            None => return std::ptr::null_mut(),
        };

        if let Some(patches) = runtime.handle_event(NodeId(node_id as u64), event_type_enum) {
            let mut guard = runtime.pending_patches.lock().unwrap();
            *guard = Some(patches);
            if let Some(ref mut buf) = *guard {
                // SAFETY: Direct buffer points to valid bytes inside the runtime's mutex guard.
                let byte_buffer =
                    unsafe { env.new_direct_byte_buffer(buf.as_mut_ptr(), buf.len()) };
                return byte_buffer
                    .map(|obj| obj.into_raw())
                    .unwrap_or(std::ptr::null_mut());
            }
        }

        std::ptr::null_mut()
    });

    result.unwrap_or(std::ptr::null_mut())
}

/// Tears down the runtime and frees all native memory.
#[no_mangle]
pub extern "system" fn Java_com_ferrix_runtime_FerrixBridge_nativeDestroy(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    let _ = catch_unwind(move || {
        if handle != 0 {
            // SAFETY: Reclaiming ownership of the pointer allocated via `Box::into_raw`.
            unsafe {
                drop(Box::from_raw(handle as *mut FerrixAndroidRuntime));
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_initial_batch() {
        let runtime = FerrixAndroidRuntime::new();
        let guard = runtime.pending_patches.lock().unwrap();
        assert!(guard.is_some());
        let encoded = guard.as_ref().unwrap();

        let batch = PatchBatch::decode(encoded).expect("failed to decode initial patch batch");
        assert_eq!(batch.patches.len(), 11);

        // Verify root node
        assert_eq!(
            batch.patches[0],
            Patch::CreateNode {
                node_id: NodeId(1),
                view_type: ViewType::Column
            }
        );

        // Verify text node
        assert_eq!(
            batch.patches[3],
            Patch::CreateNode {
                node_id: NodeId(2),
                view_type: ViewType::Text
            }
        );
        assert_eq!(
            batch.patches[4],
            Patch::SetText {
                node_id: NodeId(2),
                text: "Hello from Ferrix Native Rust!".to_string()
            }
        );
    }

    #[test]
    fn test_runtime_event_handling() {
        let runtime = FerrixAndroidRuntime::new();
        let response_bytes = runtime
            .handle_event(NodeId(3), EventType::Click)
            .expect("expected response patches on button click");

        let batch = PatchBatch::decode(&response_bytes).expect("failed to decode response batch");
        assert_eq!(batch.patches.len(), 1);
        assert_eq!(
            batch.patches[0],
            Patch::SetText {
                node_id: NodeId(2),
                text: "Rust Reactive Count: 1".to_string()
            }
        );
    }
}
