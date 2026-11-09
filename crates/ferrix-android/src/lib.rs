//! # ferrix-android
//!
//! Android JNI glue and native lifecycle controller for Ferrix.
//! Exposes entry points to Kotlin runtime via packed binary patch buffers.

use std::collections::HashMap;
use std::panic::catch_unwind;
use std::sync::Mutex;

use ferrix_core::platform::*;
use ferrix_core::prelude::*;
use ferrix_network::QueryStatus;
use ferrix_renderer_protocol::platform::{PlatformRequest, PlatformResponse};
use jni::objects::{JByteArray, JClass, JString};
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

/// Constructs the declarative UI tree for the Camera + Location Platform Demo App.
fn build_platform_tree(
    cx: &mut ComponentContext,
    perm_status: &PermissionStatus,
    loc_data: &Option<LocationData>,
    cam_state: &CameraState,
) -> (Element, NodeId, NodeId, NodeId) {
    cx.prepare_render();

    let perm_text = match perm_status {
        PermissionStatus::NotRequested => "Permission: Not Requested".to_string(),
        PermissionStatus::Granted => "Permission: Granted".to_string(),
        PermissionStatus::Denied {
            should_show_rationale,
        } => {
            format!("Permission: Denied (rationale: {})", should_show_rationale)
        }
    };

    let loc_text = match loc_data {
        Some(loc) => format!(
            "Location: Lat {:.4}, Lng {:.4}",
            loc.latitude, loc.longitude
        ),
        None => "Location: Inactive".to_string(),
    };

    let cam_text = match cam_state {
        CameraState::Active { width, height } => format!("Camera: Active ({}x{})", width, height),
        CameraState::Inactive => "Camera: Inactive".to_string(),
    };

    let tree = Column::new()
        .padding(20.0)
        .spacing(12.0)
        .child(Text::new("Ferrix Platform SDK (M5)").size(22.0))
        .child(Text::new(perm_text).size(16.0))
        .child(Button::new("Request Camera Permission"))
        .child(Text::new(loc_text).size(16.0))
        .child(Button::new("Start Location Updates"))
        .child(Text::new(cam_text).size(16.0))
        .child(Button::new("Start Camera Preview"))
        .build_with_id(NodeId::new(1), cx.allocator());

    let perm_btn = tree.children[2].id;
    let loc_btn = tree.children[4].id;
    let cam_btn = tree.children[6].id;

    (tree, perm_btn, loc_btn, cam_btn)
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProductItem {
    pub id: u64,
    pub title: String,
    pub price: String,
    pub image_url: String,
    pub likes: u32,
    pub liked: bool,
}

pub fn get_mock_products() -> Vec<ProductItem> {
    vec![
        ProductItem {
            id: 1,
            title: "Ferrix Pro Wireless Headphones".to_string(),
            price: "$199".to_string(),
            image_url: "https://ferrix.dev/img/headphones.png".to_string(),
            likes: 12,
            liked: false,
        },
        ProductItem {
            id: 2,
            title: "Mechanical Keychron K2".to_string(),
            price: "$89".to_string(),
            image_url: "https://ferrix.dev/img/keyboard.png".to_string(),
            likes: 45,
            liked: false,
        },
        ProductItem {
            id: 3,
            title: "LG UltraFine 4K Monitor".to_string(),
            price: "$499".to_string(),
            image_url: "https://ferrix.dev/img/monitor.png".to_string(),
            likes: 27,
            liked: false,
        },
    ]
}

/// Constructs the declarative UI tree for the API-driven store app.
pub fn build_api_tree(
    cx: &mut ComponentContext,
    status: &QueryStatus<Vec<ProductItem>>,
    last_interacted_id: Option<u64>,
) -> (Element, HashMap<NodeId, u64>) {
    cx.prepare_render();

    let mut scroll_content = Column::new()
        .padding(20.0)
        .spacing(16.0)
        .background_color(0xFF0F172A)
        .child(
            Text::new("Ferrix Store (M6 API App)")
                .size(24.0)
                .color(0xFFF8FAFC),
        )
        .child(
            Text::new("Data fetched via use_query & mutated via use_mutation")
                .size(14.0)
                .color(0xFF94A3B8),
        );

    let mut like_buttons = HashMap::new();

    match status {
        QueryStatus::Loading => {
            scroll_content = scroll_content.child(
                Text::new("Loading products...")
                    .size(16.0)
                    .color(0xFF38BDF8),
            );
        }
        QueryStatus::Error(err) => {
            scroll_content = scroll_content.child(
                Text::new(format!("Error loading products: {}", err))
                    .size(16.0)
                    .color(0xFFEF4444),
            );
        }
        QueryStatus::Success(products) => {
            for item in products {
                let is_recent = last_interacted_id == Some(item.id);
                let mut card = Column::new()
                    .key(format!("product_{}", item.id))
                    .padding(16.0)
                    .spacing(8.0)
                    .background_color(0xFF1E293B)
                    .child(Image::new(&item.image_url).key(format!("img_{}", item.id)))
                    .child(Text::new(&item.title).size(18.0).color(0xFFFFFFFF))
                    .child(
                        Text::new(format!("Price: {} | ❤️ {} Likes", item.price, item.likes))
                            .size(14.0)
                            .color(0xFF38BDF8),
                    )
                    .child(
                        Button::new(if item.liked {
                            "❤️ Liked"
                        } else {
                            "🤍 Like"
                        })
                        .background_color(if item.liked { 0xFFE11D48 } else { 0xFF3B82F6 })
                        .text_color(0xFFFFFFFF),
                    );

                if is_recent {
                    card = card.animate_scale(1.05, 250);
                }

                scroll_content = scroll_content.child(card);
            }
        }
    }

    let root = scroll_content.build_with_id(NodeId::new(1), cx.allocator());

    if let QueryStatus::Success(products) = status {
        for (i, item) in products.iter().enumerate() {
            if let Some(card) = root.children.get(2 + i) {
                if let Some(btn) = card.children.get(3) {
                    like_buttons.insert(btn.id, item.id);
                }
            }
        }
    }

    (root, like_buttons)
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
    Platform {
        context: ComponentContext,
        outbox: PlatformOutbox,
        current_tree: Element,
        perm_signal: Signal<PermissionStatus>,
        perm_btn: NodeId,
        loc_signal: Signal<Option<LocationData>>,
        loc_btn: NodeId,
        cam_signal: Signal<CameraState>,
        cam_btn: NodeId,
    },
    Api {
        context: ComponentContext,
        query_signal: Signal<QueryStatus<Vec<ProductItem>>>,
        mutation_handle: MutationHandle<Vec<ProductItem>, u64>,
        current_tree: Element,
        like_buttons: HashMap<NodeId, u64>,
        last_liked_id: Option<u64>,
    },
}

/// Internal state managed on the Rust side of the Android bridge.
pub struct FerrixAndroidRuntime {
    state: Mutex<AppState>,
    pending_patches: Mutex<Option<Vec<u8>>>,
    pending_platform_requests: Mutex<Option<Vec<u8>>>,
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
            pending_platform_requests: Mutex::new(None),
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
            pending_platform_requests: Mutex::new(None),
            jni_call_counter: Mutex::new(0),
        }
    }

    /// Creates a runtime running the Camera + Location Platform Demo App.
    pub fn new_platform() -> Self {
        let mut cx = ComponentContext::with_start_id(2);
        let outbox = PlatformOutbox::new();

        let perm_signal = cx.use_signal(PermissionStatus::NotRequested);
        let loc_signal = cx.use_signal(None);
        let cam_signal = cx.use_signal(CameraState::Inactive);

        let (initial_tree, perm_btn, loc_btn, cam_btn) = build_platform_tree(
            &mut cx,
            &perm_signal.get(),
            &loc_signal.get(),
            &cam_signal.get(),
        );

        let initial_batch = generate_mount_patches(&initial_tree);
        let encoded = initial_batch.encode();

        Self {
            state: Mutex::new(AppState::Platform {
                context: cx,
                outbox,
                current_tree: initial_tree,
                perm_signal,
                perm_btn,
                loc_signal,
                loc_btn,
                cam_signal,
                cam_btn,
            }),
            pending_patches: Mutex::new(Some(encoded)),
            pending_platform_requests: Mutex::new(None),
            jni_call_counter: Mutex::new(0),
        }
    }

    /// Creates a runtime running the API-driven store app.
    pub fn new_api() -> Self {
        let mut cx = ComponentContext::with_start_id(2);
        let query_signal = use_query(&mut cx, "products_query", || Ok(get_mock_products()));

        let q_clone = query_signal.clone();
        let mutation_handle = use_mutation(&mut cx, move |product_id: u64| {
            if let QueryStatus::Success(mut items) = q_clone.get() {
                if let Some(item) = items.iter_mut().find(|p| p.id == product_id) {
                    item.liked = !item.liked;
                    if item.liked {
                        item.likes += 1;
                    } else {
                        item.likes = item.likes.saturating_sub(1);
                    }
                }
                q_clone.set(QueryStatus::Success(items.clone()));
                Ok(items)
            } else {
                Err("Products not loaded".to_string())
            }
        });

        let (initial_tree, like_btns) = build_api_tree(&mut cx, &query_signal.get(), None);
        let initial_batch = generate_mount_patches(&initial_tree);
        let encoded = initial_batch.encode();

        Self {
            state: Mutex::new(AppState::Api {
                context: cx,
                query_signal,
                mutation_handle,
                current_tree: initial_tree,
                like_buttons: like_btns,
                last_liked_id: None,
            }),
            pending_patches: Mutex::new(Some(encoded)),
            pending_platform_requests: Mutex::new(None),
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

    /// Drains any queued platform requests for native Android execution.
    pub fn poll_platform_request(&self) -> Option<Vec<u8>> {
        let state_guard = self.state.lock().unwrap();
        if let AppState::Platform { outbox, .. } = &*state_guard {
            let reqs = outbox.drain();
            if let Some(req) = reqs.into_iter().next() {
                return Some(req.encode());
            }
        }
        None
    }

    /// Handles an incoming platform response from Android and returns resulting UI patches.
    pub fn handle_platform_response(&self, response_bytes: &[u8]) -> Option<Vec<u8>> {
        *self.jni_call_counter.lock().unwrap() += 1;

        let resp = PlatformResponse::decode(response_bytes).ok()?;
        let mut state_guard = self.state.lock().unwrap();

        if let AppState::Platform {
            context,
            current_tree,
            perm_signal,
            loc_signal,
            cam_signal,
            ..
        } = &mut *state_guard
        {
            route_platform_response(&resp, Some(perm_signal), Some(loc_signal), Some(cam_signal));

            let (new_tree, _, _, _) = build_platform_tree(
                context,
                &perm_signal.get(),
                &loc_signal.get(),
                &cam_signal.get(),
            );

            let diff_batch = diff_elements(current_tree, &new_tree);
            *current_tree = new_tree;
            return Some(diff_batch.encode());
        }

        None
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
            AppState::Platform {
                outbox,
                perm_btn,
                loc_btn,
                cam_btn,
                ..
            } => {
                if node_id == *perm_btn {
                    outbox.push(PlatformRequest::RequestPermission {
                        request_id: 1,
                        permission: "android.permission.CAMERA".to_string(),
                    });
                } else if node_id == *loc_btn {
                    outbox.push(PlatformRequest::StartLocationUpdates {
                        min_interval_ms: 1000,
                        min_distance_m: 5.0,
                    });
                } else if node_id == *cam_btn {
                    outbox.push(PlatformRequest::StartCameraPreview {
                        target_node_id: NodeId::new(7),
                    });
                }
            }
            AppState::Api {
                context,
                query_signal,
                mutation_handle,
                current_tree,
                like_buttons,
                last_liked_id,
            } => {
                if let Some(&product_id) = like_buttons.get(&node_id) {
                    (mutation_handle.mutate)(product_id);
                    *last_liked_id = Some(product_id);

                    let (new_tree, new_buttons) =
                        build_api_tree(context, &query_signal.get(), *last_liked_id);
                    let diff_batch = diff_elements(current_tree, &new_tree);

                    *current_tree = new_tree;
                    *like_buttons = new_buttons;

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

/// Initializes the Ferrix Rust runtime with a specific app name ("counter", "todo", or "platform").
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
            "platform" => Box::new(FerrixAndroidRuntime::new_platform()),
            "api" => Box::new(FerrixAndroidRuntime::new_api()),
            _ => Box::new(FerrixAndroidRuntime::new_counter()),
        };
        Box::into_raw(runtime) as jlong
    });

    result.unwrap_or(0)
}

/// Polls any queued platform request bytes from Rust for native execution in Kotlin.
#[no_mangle]
pub extern "system" fn Java_com_ferrix_runtime_FerrixBridge_nativePollPlatformRequests(
    mut env: JNIEnv,
    _class: JClass,
    handle: jlong,
) -> jobject {
    if handle == 0 {
        return std::ptr::null_mut();
    }

    let result = catch_unwind(move || {
        let runtime = unsafe { &*(handle as *const FerrixAndroidRuntime) };
        if let Some(req_bytes) = runtime.poll_platform_request() {
            let mut pending = runtime.pending_platform_requests.lock().unwrap();
            *pending = Some(req_bytes);
            if let Some(ref mut buf) = *pending {
                let bb = unsafe { env.new_direct_byte_buffer(buf.as_mut_ptr(), buf.len()) };
                return bb.map(|b| b.into_raw()).unwrap_or(std::ptr::null_mut());
            }
        }
        std::ptr::null_mut()
    });

    result.unwrap_or(std::ptr::null_mut())
}

/// Dispatches a platform response from Kotlin to Rust and returns any resulting UI patches.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "system" fn Java_com_ferrix_runtime_FerrixBridge_nativeDispatchPlatformResponse(
    mut env: JNIEnv,
    _class: JClass,
    handle: jlong,
    response_bytes: jbyteArray,
) -> jobject {
    if handle == 0 {
        return std::ptr::null_mut();
    }

    let result = catch_unwind(move || {
        let runtime = unsafe { &*(handle as *const FerrixAndroidRuntime) };
        let j_array = unsafe { JByteArray::from_raw(response_bytes) };
        let bytes: Vec<u8> = match env.convert_byte_array(&j_array) {
            Ok(b) => b,
            Err(_) => return std::ptr::null_mut(),
        };

        if let Some(patches) = runtime.handle_platform_response(&bytes) {
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
            let runtime = Box::from_raw(handle as *mut FerrixAndroidRuntime);
            let mut state_guard = runtime.state.lock().unwrap();
            match &mut *state_guard {
                AppState::Counter { context, .. } => context.unmount(),
                AppState::Todo { context, .. } => context.unmount(),
                AppState::Platform { context, .. } => context.unmount(),
                AppState::Api { context, .. } => context.unmount(),
            }
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

    #[test]
    fn test_platform_runtime_event_and_response_cycle() {
        let runtime = FerrixAndroidRuntime::new_platform();
        let (perm_btn, loc_btn, cam_btn) = match &*runtime.state.lock().unwrap() {
            AppState::Platform {
                perm_btn,
                loc_btn,
                cam_btn,
                ..
            } => (*perm_btn, *loc_btn, *cam_btn),
            _ => panic!("Expected AppState::Platform"),
        };

        // 1. Click permission button
        runtime.handle_event(perm_btn, EventType::Click);
        let req_bytes = runtime
            .poll_platform_request()
            .expect("Expected queued permission request");
        let req = PlatformRequest::decode(&req_bytes).unwrap();
        assert_eq!(
            req,
            PlatformRequest::RequestPermission {
                request_id: 1,
                permission: "android.permission.CAMERA".to_string()
            }
        );

        // 2. Deliver permission response
        let resp = PlatformResponse::PermissionResult {
            request_id: 1,
            granted: true,
            should_show_rationale: false,
        };
        let patch_bytes = runtime
            .handle_platform_response(&resp.encode())
            .expect("Expected UI patch buffer");
        let batch = PatchBatch::decode(&patch_bytes).unwrap();
        assert!(batch.patches.iter().any(|p| matches!(
            p,
            Patch::SetText { text, .. } if text == "Permission: Granted"
        )));

        // 3. Click location button
        runtime.handle_event(loc_btn, EventType::Click);
        let req_bytes = runtime
            .poll_platform_request()
            .expect("Expected queued location request");
        let req = PlatformRequest::decode(&req_bytes).unwrap();
        assert!(matches!(req, PlatformRequest::StartLocationUpdates { .. }));

        // 4. Deliver location response
        let resp = PlatformResponse::LocationUpdate {
            latitude: 37.7749,
            longitude: -122.4194,
            accuracy: 5.0,
            timestamp: 123456,
        };
        let patch_bytes = runtime
            .handle_platform_response(&resp.encode())
            .expect("Expected UI patch buffer for location");
        let batch = PatchBatch::decode(&patch_bytes).unwrap();
        assert!(batch.patches.iter().any(|p| matches!(
            p,
            Patch::SetText { text, .. } if text.contains("37.7749")
        )));

        // 5. Click camera button
        runtime.handle_event(cam_btn, EventType::Click);
        let req_bytes = runtime
            .poll_platform_request()
            .expect("Expected queued camera request");
        let req = PlatformRequest::decode(&req_bytes).unwrap();
        assert!(matches!(req, PlatformRequest::StartCameraPreview { .. }));

        // 6. Deliver camera ready response
        let resp = PlatformResponse::CameraReady {
            target_node_id: NodeId::new(7),
            preview_width: 1920,
            preview_height: 1080,
        };
        let patch_bytes = runtime
            .handle_platform_response(&resp.encode())
            .expect("Expected UI patch buffer for camera");
        let batch = PatchBatch::decode(&patch_bytes).unwrap();
        assert!(batch.patches.iter().any(|p| matches!(
            p,
            Patch::SetText { text, .. } if text == "Camera: Active (1920x1080)"
        )));
    }

    #[test]
    fn test_api_runtime_event_and_mutation_cycle() {
        let runtime = FerrixAndroidRuntime::new_api();
        let init_buf = runtime.pending_patches.lock().unwrap().clone().unwrap();
        let init_batch = PatchBatch::decode(&init_buf).unwrap();

        // Must create Image nodes and set ImageUrl properties
        assert!(init_batch.patches.iter().any(|p| matches!(
            p,
            Patch::CreateNode {
                view_type: ViewType::Image,
                ..
            }
        )));
        assert!(init_batch.patches.iter().any(|p| matches!(
            p,
            Patch::SetProperty {
                property: Property::ImageUrl(_),
                ..
            }
        )));

        // Retrieve like button for product 1
        let btn_id = match &*runtime.state.lock().unwrap() {
            AppState::Api { like_buttons, .. } => {
                let (&id, _) = like_buttons.iter().find(|(_, &p_id)| p_id == 1).unwrap();
                id
            }
            _ => panic!("Expected AppState::Api"),
        };

        // 1. Click like button (triggers use_mutation)
        let update_bytes = runtime
            .handle_event(btn_id, EventType::Click)
            .expect("Expected UI patch buffer on like");
        let update_batch = PatchBatch::decode(&update_bytes).unwrap();

        // Assert text updated with 13 likes
        assert!(update_batch.patches.iter().any(|p| matches!(
            p,
            Patch::SetText { text, .. } if text.contains("13 Likes")
        )));

        // Assert button updated to Liked
        assert!(update_batch.patches.iter().any(|p| matches!(
            p,
            Patch::SetText { text, .. } if text == "❤️ Liked"
        )));

        // Assert native animation patch was emitted!
        assert!(update_batch.patches.iter().any(|p| matches!(
            p,
            Patch::SetProperty {
                property: Property::AnimateScale { target, duration_ms },
                ..
            } if (*target - 1.05).abs() < f32::EPSILON && *duration_ms == 250
        )));

        // 2. Click like button again (un-like mutation)
        let unlike_bytes = runtime
            .handle_event(btn_id, EventType::Click)
            .expect("Expected UI patch buffer on unlike");
        let unlike_batch = PatchBatch::decode(&unlike_bytes).unwrap();

        assert!(unlike_batch.patches.iter().any(|p| matches!(
            p,
            Patch::SetText { text, .. } if text.contains("12 Likes")
        )));
    }
}
