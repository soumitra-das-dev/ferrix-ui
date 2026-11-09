//! # ferrix-android
//!
//! Android JNI glue and native lifecycle controller for Ferrix.
//! Exposes entry points to Kotlin runtime via packed binary patch buffers.

use std::panic::catch_unwind;
use std::sync::Mutex;

use ferrix_core::prelude::*;
use jni::objects::JClass;
use jni::sys::{jbyteArray, jint, jlong, jobject};
use jni::JNIEnv;

/// Helper function to construct the declarative UI tree for a given counter value.
fn build_ui_tree(counter: u32) -> Element {
    let alloc = NodeIdAllocator::with_start(2);
    Column::new()
        .padding(20.0)
        .spacing(16.0)
        .background_color(0xFFF1F5F9)
        .child(
            Text::new("Ferrix Element System (M2)")
                .size(22.0)
                .color(0xFF0F172A),
        )
        .child(
            Text::new("Demonstrating Column, Row, Text, and Button from Rust")
                .size(14.0)
                .color(0xFF475569),
        )
        .child(
            Row::new()
                .spacing(12.0)
                .child(
                    Button::new("Increment")
                        .background_color(0xFF2563EB)
                        .text_color(0xFFFFFFFF),
                )
                .child(
                    Button::new("Reset")
                        .background_color(0xFFDC2626)
                        .text_color(0xFFFFFFFF),
                ),
        )
        .child(
            Text::new(format!("Counter: {}", counter))
                .size(18.0)
                .color(0xFF1E293B),
        )
        .build_with_id(NodeId::new(1), &alloc)
}

/// Internal state managed on the Rust side of the Android bridge.
pub struct FerrixAndroidRuntime {
    current_tree: Mutex<Element>,
    pending_patches: Mutex<Option<Vec<u8>>>,
    text_counter: Mutex<u32>,
}

impl FerrixAndroidRuntime {
    pub fn new() -> Self {
        let initial_tree = build_ui_tree(0);
        let initial_batch = generate_mount_patches(&initial_tree);
        let encoded = initial_batch.encode();

        Self {
            current_tree: Mutex::new(initial_tree),
            pending_patches: Mutex::new(Some(encoded)),
            text_counter: Mutex::new(0),
        }
    }

    /// Handles an incoming UI event from Kotlin and queues any resulting patch updates.
    pub fn handle_event(&self, node_id: NodeId, event_type: EventType) -> Option<Vec<u8>> {
        let mut counter = self.text_counter.lock().unwrap();
        let should_update = if node_id == NodeId(5) && event_type == EventType::Click {
            // Node 5 is "Increment"
            *counter += 1;
            true
        } else if node_id == NodeId(6) && event_type == EventType::Click {
            // Node 6 is "Reset"
            *counter = 0;
            true
        } else {
            false
        };

        if should_update {
            let new_tree = build_ui_tree(*counter);
            let mut tree_guard = self.current_tree.lock().unwrap();
            let diff_batch = diff_elements(&tree_guard, &new_tree);
            *tree_guard = new_tree;
            return Some(diff_batch.encode());
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
                Ok(bb) => bb.into_raw(),
                Err(err) => {
                    tracing::error!("Failed to create DirectByteBuffer: {:?}", err);
                    std::ptr::null_mut()
                }
            }
        } else {
            std::ptr::null_mut()
        }
    });

    result.unwrap_or(std::ptr::null_mut())
}

/// Dispatches an event from Kotlin to Rust and returns any resulting patches.
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

        // SAFETY: `handle` is checked non-null and valid.
        let runtime = unsafe { &*(handle as *const FerrixAndroidRuntime) };
        let target_node = NodeId(node_id as u64);
        let ev_type = match EventType::from_u16(event_type as u16) {
            Some(et) => et,
            None => return std::ptr::null_mut(),
        };

        if let Some(patch_bytes) = runtime.handle_event(target_node, ev_type) {
            let mut guard = runtime.pending_patches.lock().unwrap();
            *guard = Some(patch_bytes);
            if let Some(ref mut buf) = *guard {
                // SAFETY: pointer valid during buffer lifetime in mutex.
                let bb = unsafe { env.new_direct_byte_buffer(buf.as_mut_ptr(), buf.len()) };
                return bb.map(|b| b.into_raw()).unwrap_or(std::ptr::null_mut());
            }
        }

        std::ptr::null_mut()
    });

    result.unwrap_or(std::ptr::null_mut())
}

/// Destroys the native runtime instance and releases its allocated memory.
#[no_mangle]
pub extern "system" fn Java_com_ferrix_runtime_FerrixBridge_nativeDestroy(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    let _ = catch_unwind(|| {
        if handle != 0 {
            // SAFETY: Reclaims ownership of raw pointer allocated in `nativeInit`.
            unsafe {
                let _ = Box::from_raw(handle as *mut FerrixAndroidRuntime);
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
        let bytes = guard.as_ref().unwrap();

        let batch = PatchBatch::decode(bytes).expect("Failed to decode initial patch batch");

        // Verify that all 4 view types (Column, Row, Text, Button) are created
        let view_types: Vec<_> = batch
            .patches
            .iter()
            .filter_map(|p| match p {
                Patch::CreateNode { view_type, .. } => Some(*view_type),
                _ => None,
            })
            .collect();

        assert!(view_types.contains(&ViewType::Column));
        assert!(view_types.contains(&ViewType::Row));
        assert!(view_types.contains(&ViewType::Text));
        assert!(view_types.contains(&ViewType::Button));
    }

    #[test]
    fn test_runtime_event_handling() {
        let runtime = FerrixAndroidRuntime::new();

        // Node 5 is "Increment"
        let patch_bytes = runtime
            .handle_event(NodeId(5), EventType::Click)
            .expect("Expected patch buffer on Increment click");

        let batch = PatchBatch::decode(&patch_bytes).expect("Failed to decode patch batch");
        assert_eq!(batch.len(), 1);
        match &batch.patches[0] {
            Patch::SetText { node_id, text } => {
                assert_eq!(*node_id, NodeId(7));
                assert_eq!(text, "Counter: 1");
            }
            other => panic!("Expected SetText patch, got {:?}", other),
        }

        // Node 6 is "Reset"
        let patch_bytes = runtime
            .handle_event(NodeId(6), EventType::Click)
            .expect("Expected patch buffer on Reset click");

        let batch = PatchBatch::decode(&patch_bytes).expect("Failed to decode patch batch");
        assert_eq!(batch.len(), 1);
        match &batch.patches[0] {
            Patch::SetText { node_id, text } => {
                assert_eq!(*node_id, NodeId(7));
                assert_eq!(text, "Counter: 0");
            }
            other => panic!("Expected SetText patch, got {:?}", other),
        }
    }
}
