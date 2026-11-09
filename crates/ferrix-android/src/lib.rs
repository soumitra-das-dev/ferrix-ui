//! # ferrix-android
//!
//! Android JNI glue and native lifecycle controller for Ferrix.
//! Exposes entry points to Kotlin runtime via packed binary patch buffers.

use std::collections::HashMap;
use std::panic::catch_unwind;
use std::sync::Mutex;

use ferrix_core::prelude::*;
use jni::objects::{JClass, JString};
use jni::sys::{jbyteArray, jint, jlong, jobject};
use jni::JNIEnv;

pub mod counter_app {
    include!(concat!(env!("OUT_DIR"), "/counter_generated.rs"));
}

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

enum AppState {
    Counter {
        context: ComponentContext,
        props: counter_app::CounterProps,
        current_tree: Element,
        button_id: NodeId,
        _text_id: NodeId,
    },
    Todo {
        context: ComponentContext,
        todos: Vec<TodoItem>,
        next_todo_id: u64,
        current_tree: Element,
        add_button_id: NodeId,
        toggle_map: HashMap<NodeId, u64>,
        delete_map: HashMap<NodeId, u64>,
    },
}

/// Internal state managed on the Rust side of the Android bridge.
pub struct FerrixAndroidRuntime {
    state: Mutex<AppState>,
    pending_patches: Mutex<Option<Vec<u8>>>,
    jni_call_counter: Mutex<u64>,
}

impl FerrixAndroidRuntime {
    /// Creates a runtime running the compiled Counter component (from `component.fsx`).
    pub fn new_counter() -> Self {
        let mut cx = ComponentContext::with_start_id(2);
        let props = counter_app::CounterProps { initial: 0 };
        let current_tree = counter_app::Counter(&mut cx, props.clone());
        let text_id = current_tree.children[1].id;
        let button_id = current_tree.children[2].id;

        let initial_batch = generate_mount_patches(&current_tree);
        let encoded = initial_batch.encode();

        Self {
            state: Mutex::new(AppState::Counter {
                context: cx,
                props,
                current_tree,
                button_id,
                _text_id: text_id,
            }),
            pending_patches: Mutex::new(Some(encoded)),
            jni_call_counter: Mutex::new(0),
        }
    }

    /// Creates a runtime running the reactive Todo app.
    pub fn new_todo() -> Self {
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
            state: Mutex::new(AppState::Todo {
                context: cx,
                todos: initial_todos,
                next_todo_id: 3,
                current_tree: initial_tree,
                add_button_id: add_btn,
                toggle_map: toggles,
                delete_map: deletes,
            }),
            pending_patches: Mutex::new(Some(encoded)),
            jni_call_counter: Mutex::new(0),
        }
    }

    /// Default constructor starts the Counter app.
    pub fn new() -> Self {
        Self::new_counter()
    }

    /// Total JNI calls handled by this runtime instance.
    pub fn jni_call_count(&self) -> u64 {
        *self.jni_call_counter.lock().unwrap()
    }

    /// Handles an incoming UI event from Kotlin and queues any resulting patch updates.
    pub fn handle_event(&self, node_id: NodeId, event_type: EventType) -> Option<Vec<u8>> {
        *self.jni_call_counter.lock().unwrap() += 1;

        if event_type != EventType::Click {
            return None;
        }

        let mut state_guard = self.state.lock().unwrap();
        match &mut *state_guard {
            AppState::Counter {
                context,
                props,
                current_tree,
                button_id,
                ..
            } => {
                if node_id == *button_id {
                    if let Some(signal) = context.get_signal::<i32>(0) {
                        signal.set(signal.get() + 1);
                    }
                    let new_tree = counter_app::Counter(context, props.clone());
                    let diff_batch = diff_elements(current_tree, &new_tree);
                    *current_tree = new_tree;
                    return Some(diff_batch.encode());
                }
            }
            AppState::Todo {
                context,
                todos,
                next_todo_id,
                current_tree,
                add_button_id,
                toggle_map,
                delete_map,
            } => {
                let mut changed = false;
                if node_id == *add_button_id {
                    todos.push(TodoItem {
                        id: *next_todo_id,
                        text: format!("New Ferrix Task #{}", *next_todo_id),
                        completed: false,
                    });
                    *next_todo_id += 1;
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
                    let (new_tree, new_add_btn, new_toggles, new_deletes) =
                        build_todo_tree(context, todos);
                    let diff_batch = diff_elements(current_tree, &new_tree);

                    *current_tree = new_tree;
                    *add_button_id = new_add_btn;
                    *toggle_map = new_toggles;
                    *delete_map = new_deletes;

                    return Some(diff_batch.encode());
                }
            }
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

/// Initializes the Ferrix Rust runtime (defaulting to Counter) and returns heap pointer as `jlong`.
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

/// Initializes the Ferrix Rust runtime with a specific app name ("counter" or "todo").
#[no_mangle]
pub extern "system" fn Java_com_ferrix_runtime_FerrixBridge_nativeInitApp(
    mut env: JNIEnv,
    _class: JClass,
    app_name: JString,
) -> jlong {
    let result = catch_unwind(move || {
        let name_str: String = match env.get_string(&app_name) {
            Ok(s) => s.into(),
            Err(_) => return 0,
        };

        let runtime = match name_str.as_str() {
            "todo" => Box::new(FerrixAndroidRuntime::new_todo()),
            _ => Box::new(FerrixAndroidRuntime::new_counter()),
        };
        Box::into_raw(runtime) as jlong
    });

    result.unwrap_or(0)
}

/// Retrieves the initial UI patch buffer as a direct byte buffer.
#[no_mangle]
pub extern "system" fn Java_com_ferrix_runtime_FerrixBridge_nativeGetInitialPatches(
    mut env: JNIEnv,
    _class: JClass,
    handle: jlong,
) -> jobject {
    if handle == 0 {
        return std::ptr::null_mut();
    }

    let result = catch_unwind(move || {
        let runtime = unsafe { &*(handle as *const FerrixAndroidRuntime) };
        let mut pending = runtime.pending_patches.lock().unwrap();
        if let Some(ref mut buf) = *pending {
            // SAFETY: DirectByteBuffer valid while buffered in runtime mutex
            let bb = unsafe { env.new_direct_byte_buffer(buf.as_mut_ptr(), buf.len()) };
            return bb.map(|b| b.into_raw()).unwrap_or(std::ptr::null_mut());
        }
        std::ptr::null_mut()
    });

    result.unwrap_or(std::ptr::null_mut())
}

/// Dispatches an interactive event from Kotlin to Rust and returns any resulting patches.
#[no_mangle]
pub extern "system" fn Java_com_ferrix_runtime_FerrixBridge_nativeDispatchEvent(
    mut env: JNIEnv,
    _class: JClass,
    handle: jlong,
    node_id: jlong,
    event_type: jint,
    _payload: jbyteArray,
) -> jobject {
    if handle == 0 {
        return std::ptr::null_mut();
    }

    let result = catch_unwind(move || {
        let runtime = unsafe { &*(handle as *const FerrixAndroidRuntime) };
        let ev_type = match EventType::from_u16(event_type as u16) {
            Some(t) => t,
            None => return std::ptr::null_mut(),
        };

        if let Some(patches) = runtime.handle_event(NodeId::new(node_id as u64), ev_type) {
            let mut pending = runtime.pending_patches.lock().unwrap();
            *pending = Some(patches);
            if let Some(ref mut buf) = *pending {
                let bb = unsafe { env.new_direct_byte_buffer(buf.as_mut_ptr(), buf.len()) };
                return bb.map(|b| b.into_raw()).unwrap_or(std::ptr::null_mut());
            }
        }
        std::ptr::null_mut()
    });

    result.unwrap_or(std::ptr::null_mut())
}

/// Destroys the native runtime instance and frees its memory.
#[no_mangle]
pub extern "system" fn Java_com_ferrix_runtime_FerrixBridge_nativeDestroy(
    _env: JNIEnv,
    _class: JClass,
    handle: jlong,
) {
    if handle == 0 {
        return;
    }
    let _ = catch_unwind(|| {
        // SAFETY: Pointer was created by Box::into_raw in nativeInit
        unsafe {
            drop(Box::from_raw(handle as *mut FerrixAndroidRuntime));
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_runtime_initial_batch() {
        let runtime = FerrixAndroidRuntime::new_counter();
        let guard = runtime.pending_patches.lock().unwrap();
        let bytes = guard.as_ref().unwrap();
        let batch = PatchBatch::decode(bytes).expect("Failed to decode initial batch");
        let create_count = batch
            .patches
            .iter()
            .filter(|p| matches!(p, Patch::CreateNode { .. }))
            .count();
        assert_eq!(create_count, 4); // Column, Text, Text, Button
    }

    #[test]
    fn test_counter_runtime_click_produces_single_set_text_patch() {
        let runtime = FerrixAndroidRuntime::new_counter();

        // Node 4 is the Increment button
        let update_bytes = runtime
            .handle_event(NodeId::new(4), EventType::Click)
            .expect("Expected patch buffer on button click");

        let batch = PatchBatch::decode(&update_bytes).expect("Failed to decode update batch");
        assert_eq!(batch.patches.len(), 1, "Must emit exactly 1 patch");

        match &batch.patches[0] {
            Patch::SetText { node_id, text } => {
                assert_eq!(*node_id, NodeId::new(3));
                assert_eq!(text, "Count: 1");
            }
            other => panic!("Expected SetText patch, found {:?}", other),
        }

        assert_eq!(runtime.jni_call_count(), 1, "Exactly 1 call recorded");
    }

    #[test]
    fn test_todo_runtime_event_handling() {
        let runtime = FerrixAndroidRuntime::new_todo();
        let guard = runtime.pending_patches.lock().unwrap();
        assert!(guard.is_some());

        let add_btn = match &*runtime.state.lock().unwrap() {
            AppState::Todo { add_button_id, .. } => *add_button_id,
            _ => panic!("Wrong state"),
        };

        let update_bytes = runtime
            .handle_event(add_btn, EventType::Click)
            .expect("Expected patch buffer on add");
        let batch = PatchBatch::decode(&update_bytes).unwrap();
        assert!(!batch.patches.is_empty());
    }
}
