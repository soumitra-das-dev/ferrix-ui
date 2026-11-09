//! # ferrix-android
//!
//! Android JNI glue and native lifecycle controller for Ferrix.
//! Exposes entry points to Kotlin runtime via packed binary patch buffers.

use std::collections::HashMap;
use std::panic::catch_unwind;
use std::sync::Mutex;

use ferrix_core::prelude::*;
use jni::objects::JClass;
use jni::sys::{jbyteArray, jint, jlong, jobject};
use jni::JNIEnv;

#[derive(Clone, Debug, PartialEq)]
pub struct TodoItem {
    pub id: u64,
    pub text: String,
    pub completed: bool,
}

/// Constructs the declarative UI tree for the interactive Todo App.
fn build_todo_tree(
    cx: &mut ComponentContext,
    items: &[TodoItem],
) -> (Element, NodeId, HashMap<NodeId, u64>, HashMap<NodeId, u64>) {
    cx.prepare_render();

    let mut col = Column::new()
        .padding(20.0)
        .spacing(12.0)
        .background_color(0xFFF8FAFC)
        .child(
            Text::new("Ferrix Todo App (M3)")
                .size(24.0)
                .color(0xFF0F172A),
        )
        .child(
            Button::new("+ Add Task")
                .background_color(0xFF16A34A)
                .text_color(0xFFFFFFFF),
        );

    for item in items {
        let status = if item.completed { "[x] " } else { "[ ] " };
        let row = Row::new()
            .key(format!("todo_{}", item.id))
            .spacing(8.0)
            .child(
                Text::new(format!("{}{}", status, item.text))
                    .size(16.0)
                    .color(if item.completed {
                        0xFF94A3B8
                    } else {
                        0xFF1E293B
                    }),
            )
            .child(
                Button::new("Toggle")
                    .background_color(0xFF3B82F6)
                    .text_color(0xFFFFFFFF),
            )
            .child(
                Button::new("Delete")
                    .background_color(0xFFEF4444)
                    .text_color(0xFFFFFFFF),
            );
        col = col.child(row);
    }

    let tree = col.build_with_id(NodeId::new(1), cx.allocator());

    // Map button node IDs to actions
    let add_button_id = tree.children[1].id;
    let mut toggle_map = HashMap::new();
    let mut delete_map = HashMap::new();

    // Keyed todo rows start at index 2
    for (i, item) in items.iter().enumerate() {
        if let Some(row) = tree.children.get(2 + i) {
            if row.children.len() >= 3 {
                toggle_map.insert(row.children[1].id, item.id);
                delete_map.insert(row.children[2].id, item.id);
            }
        }
    }

    (tree, add_button_id, toggle_map, delete_map)
}

/// Internal state managed on the Rust side of the Android bridge.
pub struct FerrixAndroidRuntime {
    context: Mutex<ComponentContext>,
    todos: Mutex<Vec<TodoItem>>,
    next_todo_id: Mutex<u64>,
    current_tree: Mutex<Element>,
    add_button_id: Mutex<NodeId>,
    toggle_map: Mutex<HashMap<NodeId, u64>>,
    delete_map: Mutex<HashMap<NodeId, u64>>,
    pending_patches: Mutex<Option<Vec<u8>>>,
}

impl FerrixAndroidRuntime {
    pub fn new() -> Self {
        let mut cx = ComponentContext::with_start_id(2);
        let initial_todos = vec![
            TodoItem {
                id: 1,
                text: "Explore Ferrix Architecture".to_string(),
                completed: true,
            },
            TodoItem {
                id: 2,
                text: "Build Reactive Runtime".to_string(),
                completed: false,
            },
        ];

        let (initial_tree, add_btn, toggles, deletes) = build_todo_tree(&mut cx, &initial_todos);
        let initial_batch = generate_mount_patches(&initial_tree);
        let encoded = initial_batch.encode();

        Self {
            context: Mutex::new(cx),
            todos: Mutex::new(initial_todos),
            next_todo_id: Mutex::new(3),
            current_tree: Mutex::new(initial_tree),
            add_button_id: Mutex::new(add_btn),
            toggle_map: Mutex::new(toggles),
            delete_map: Mutex::new(deletes),
            pending_patches: Mutex::new(Some(encoded)),
        }
    }

    /// Handles an incoming UI event from Kotlin and queues any resulting patch updates.
    pub fn handle_event(&self, node_id: NodeId, event_type: EventType) -> Option<Vec<u8>> {
        if event_type != EventType::Click {
            return None;
        }

        let mut todos = self.todos.lock().unwrap();
        let add_btn = *self.add_button_id.lock().unwrap();
        let toggle_map = self.toggle_map.lock().unwrap().clone();
        let delete_map = self.delete_map.lock().unwrap().clone();

        let mut changed = false;

        if node_id == add_btn {
            let mut next_id = self.next_todo_id.lock().unwrap();
            todos.push(TodoItem {
                id: *next_id,
                text: format!("New Ferrix Task #{}", *next_id),
                completed: false,
            });
            *next_id += 1;
            changed = true;
        } else if let Some(&item_id) = toggle_map.get(&node_id) {
            if let Some(item) = todos.iter_mut().find(|it| it.id == item_id) {
                item.completed = !item.completed;
                changed = true;
            }
        } else if let Some(&item_id) = delete_map.get(&node_id) {
            todos.retain(|it| it.id != item_id);
            changed = true;
        }

        if changed {
            let mut cx = self.context.lock().unwrap();
            let (new_tree, new_add_btn, new_toggles, new_deletes) =
                build_todo_tree(&mut cx, &todos);

            let mut tree_guard = self.current_tree.lock().unwrap();
            let diff_batch = diff_elements(&tree_guard, &new_tree);

            *tree_guard = new_tree;
            *self.add_button_id.lock().unwrap() = new_add_btn;
            *self.toggle_map.lock().unwrap() = new_toggles;
            *self.delete_map.lock().unwrap() = new_deletes;

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

        // Verify initial batch contains Column, Text, Button, Row
        let view_types: Vec<_> = batch
            .patches
            .iter()
            .filter_map(|p| match p {
                Patch::CreateNode { view_type, .. } => Some(*view_type),
                _ => None,
            })
            .collect();

        assert!(view_types.contains(&ViewType::Column));
        assert!(view_types.contains(&ViewType::Text));
        assert!(view_types.contains(&ViewType::Button));
        assert!(view_types.contains(&ViewType::Row));
    }

    #[test]
    fn test_runtime_event_handling() {
        let runtime = FerrixAndroidRuntime::new();
        let add_btn = *runtime.add_button_id.lock().unwrap();

        // 1. Click Add Task
        let patch_bytes = runtime
            .handle_event(add_btn, EventType::Click)
            .expect("Expected patch buffer on Add Task click");

        let batch = PatchBatch::decode(&patch_bytes).expect("Failed to decode patch batch");
        assert!(!batch.is_empty());
        assert!(batch.patches.iter().any(|p| matches!(
            p,
            Patch::SetText { text, .. } if text == "[ ] New Ferrix Task #3"
        )));

        // 2. Click Toggle on Item 2
        let toggle_node = runtime
            .toggle_map
            .lock()
            .unwrap()
            .iter()
            .find(|(_, &item_id)| item_id == 2)
            .map(|(&btn_id, _)| btn_id)
            .expect("Expected toggle button for item 2");

        let patch_bytes = runtime
            .handle_event(toggle_node, EventType::Click)
            .expect("Expected patch buffer on Toggle click");
        let batch = PatchBatch::decode(&patch_bytes).expect("Failed to decode patch batch");
        assert!(batch.patches.iter().any(|p| matches!(
            p,
            Patch::SetText { text, .. } if text == "[x] Build Reactive Runtime"
        )));

        // 3. Click Delete on Item 1
        let delete_node = runtime
            .delete_map
            .lock()
            .unwrap()
            .iter()
            .find(|(_, &item_id)| item_id == 1)
            .map(|(&btn_id, _)| btn_id)
            .expect("Expected delete button for item 1");

        let patch_bytes = runtime
            .handle_event(delete_node, EventType::Click)
            .expect("Expected patch buffer on Delete click");
        let batch = PatchBatch::decode(&patch_bytes).expect("Failed to decode patch batch");
        assert!(batch
            .patches
            .iter()
            .any(|p| matches!(p, Patch::RemoveChild { .. })));
    }
}
