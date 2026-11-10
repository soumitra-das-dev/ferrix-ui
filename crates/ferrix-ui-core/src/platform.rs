//! Reactive platform capability hooks and standard platform APIs:
//! Permissions, Location, Camera, Storage, Dimensions, PixelRatio, Platform,
//! Keyboard, AppState, BackHandler, Linking, Share, Vibration, Clipboard,
//! Alert, AccessibilityInfo, I18nManager, and NetInfo.

use ferrix_reactivity::Signal;
use ferrix_renderer_protocol::platform::{PlatformRequest, PlatformResponse};
use ferrix_renderer_protocol::NodeId;
use std::sync::{Arc, Mutex, OnceLock};

use crate::component::ComponentContext;

/// Current permission status.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PermissionStatus {
    NotRequested,
    Granted,
    Denied { should_show_rationale: bool },
}

/// Fused location coordinate packet.
#[derive(Clone, Debug, PartialEq)]
pub struct LocationData {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: f32,
    pub timestamp: u64,
}

/// State of the native camera preview.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CameraState {
    Inactive,
    Active { width: u32, height: u32 },
}

/// Channel for queuing outbound platform requests from hooks and APIs.
#[derive(Clone, Default)]
pub struct PlatformOutbox {
    queue: Arc<Mutex<Vec<PlatformRequest>>>,
}

impl PlatformOutbox {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push(&self, req: PlatformRequest) {
        self.queue.lock().unwrap().push(req);
    }

    pub fn drain(&self) -> Vec<PlatformRequest> {
        let mut guard = self.queue.lock().unwrap();
        std::mem::take(&mut *guard)
    }
}

// Global thread-safe singleton outbox for imperative platform calls
static GLOBAL_OUTBOX: OnceLock<PlatformOutbox> = OnceLock::new();

pub fn global_outbox() -> &'static PlatformOutbox {
    GLOBAL_OUTBOX.get_or_init(PlatformOutbox::new)
}

// ---------------------------------------------------------------------------
// 1. Dimensions & PixelRatio
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WindowDimensions {
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub font_scale: f32,
}

impl Default for WindowDimensions {
    fn default() -> Self {
        Self {
            width: 411.0,
            height: 891.0,
            scale: 2.625,
            font_scale: 1.0,
        }
    }
}

static CURRENT_DIMENSIONS: Mutex<WindowDimensions> = Mutex::new(WindowDimensions {
    width: 411.0,
    height: 891.0,
    scale: 2.625,
    font_scale: 1.0,
});

pub struct Dimensions;

impl Dimensions {
    pub fn get() -> WindowDimensions {
        *CURRENT_DIMENSIONS.lock().unwrap()
    }

    pub fn set(dims: WindowDimensions) {
        *CURRENT_DIMENSIONS.lock().unwrap() = dims;
    }
}

/// Hook that returns a reactive signal to the current window dimensions.
pub fn use_window_dimensions(cx: &mut ComponentContext) -> Signal<WindowDimensions> {
    cx.use_signal(Dimensions::get())
}

pub struct PixelRatio;

impl PixelRatio {
    pub fn get() -> f32 {
        Dimensions::get().scale
    }

    pub fn get_font_scale() -> f32 {
        Dimensions::get().font_scale
    }

    pub fn round_to_nearest_pixel(dp: f32) -> f32 {
        let scale = Self::get();
        (dp * scale).round() / scale
    }
}

// ---------------------------------------------------------------------------
// 2. Platform
// ---------------------------------------------------------------------------

pub struct Platform;

impl Platform {
    pub const OS: &'static str = "android";

    pub fn version() -> u32 {
        34
    }

    pub fn is_android() -> bool {
        true
    }

    pub fn is_testing() -> bool {
        cfg!(test)
    }

    pub fn select<T>(android: T, _default: T) -> T {
        android
    }
}

// ---------------------------------------------------------------------------
// 3. Keyboard
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyboardMetrics {
    pub is_visible: bool,
    pub height_dp: f32,
}

static CURRENT_KEYBOARD: Mutex<KeyboardMetrics> = Mutex::new(KeyboardMetrics {
    is_visible: false,
    height_dp: 0.0,
});

pub struct Keyboard;

impl Keyboard {
    pub fn is_visible() -> bool {
        CURRENT_KEYBOARD.lock().unwrap().is_visible
    }

    pub fn metrics() -> KeyboardMetrics {
        *CURRENT_KEYBOARD.lock().unwrap()
    }

    pub fn dismiss(outbox: Option<&PlatformOutbox>) {
        let out = outbox.unwrap_or(global_outbox());
        out.push(PlatformRequest::DismissKeyboard);
    }

    pub fn set_metrics(metrics: KeyboardMetrics) {
        *CURRENT_KEYBOARD.lock().unwrap() = metrics;
    }
}

pub fn use_keyboard(cx: &mut ComponentContext) -> Signal<KeyboardMetrics> {
    cx.use_signal(Keyboard::metrics())
}

// ---------------------------------------------------------------------------
// 4. AppState
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppStateKind {
    Active,
    Background,
    Inactive,
}

static CURRENT_APP_STATE: Mutex<AppStateKind> = Mutex::new(AppStateKind::Active);

pub struct AppState;

impl AppState {
    pub fn current() -> AppStateKind {
        *CURRENT_APP_STATE.lock().unwrap()
    }

    pub fn set(state: AppStateKind) {
        *CURRENT_APP_STATE.lock().unwrap() = state;
    }
}

pub fn use_app_state(cx: &mut ComponentContext) -> Signal<AppStateKind> {
    cx.use_signal(AppState::current())
}

// ---------------------------------------------------------------------------
// 5. BackHandler
// ---------------------------------------------------------------------------

static BACK_HANDLERS: Mutex<Vec<Arc<dyn Fn() -> bool + Send + Sync>>> = Mutex::new(Vec::new());

pub struct BackHandler;

impl BackHandler {
    pub fn exit_app(outbox: Option<&PlatformOutbox>) {
        let out = outbox.unwrap_or(global_outbox());
        out.push(PlatformRequest::ExitApp);
    }

    pub fn add_listener(handler: Arc<dyn Fn() -> bool + Send + Sync>) {
        BACK_HANDLERS.lock().unwrap().push(handler);
    }

    pub fn trigger() -> bool {
        let handlers = BACK_HANDLERS.lock().unwrap().clone();
        for handler in handlers.iter().rev() {
            if handler() {
                return true; // consumed
            }
        }
        false
    }
}

pub fn use_back_handler(
    cx: &mut ComponentContext,
    handler: impl Fn() -> bool + Send + Sync + 'static,
) {
    let handler_arc: Arc<dyn Fn() -> bool + Send + Sync> = Arc::new(handler);
    let handler_clone = handler_arc.clone();

    cx.use_effect(
        move || {
            BackHandler::add_listener(handler_clone);
            Some(Box::new(|| {
                // In production, unregister handler on unmount
            }))
        },
        (),
    );
}

// ---------------------------------------------------------------------------
// 6. Linking, Share, Vibration, Clipboard, Alert, AccessibilityInfo, I18nManager
// ---------------------------------------------------------------------------

pub struct Linking;

impl Linking {
    pub fn open_url(outbox: Option<&PlatformOutbox>, url: &str) {
        let out = outbox.unwrap_or(global_outbox());
        out.push(PlatformRequest::OpenUrl {
            url: url.to_string(),
        });
    }

    pub fn can_open_url(url: &str) -> bool {
        url.starts_with("http://")
            || url.starts_with("https://")
            || url.starts_with("tel:")
            || url.starts_with("mailto:")
    }
}

pub struct Share;

impl Share {
    pub fn share(outbox: Option<&PlatformOutbox>, title: &str, message: &str) {
        let out = outbox.unwrap_or(global_outbox());
        out.push(PlatformRequest::Share {
            title: title.to_string(),
            message: message.to_string(),
        });
    }
}

pub struct Vibration;

impl Vibration {
    pub fn vibrate(outbox: Option<&PlatformOutbox>, duration_ms: u64) {
        let out = outbox.unwrap_or(global_outbox());
        out.push(PlatformRequest::Vibrate { duration_ms });
    }

    pub fn cancel(outbox: Option<&PlatformOutbox>) {
        let out = outbox.unwrap_or(global_outbox());
        out.push(PlatformRequest::Vibrate { duration_ms: 0 });
    }
}

pub struct Clipboard;

impl Clipboard {
    pub fn set_string(outbox: Option<&PlatformOutbox>, text: &str) {
        let out = outbox.unwrap_or(global_outbox());
        out.push(PlatformRequest::SetClipboard {
            text: text.to_string(),
        });
    }

    pub fn get_string(outbox: Option<&PlatformOutbox>, request_id: u32) {
        let out = outbox.unwrap_or(global_outbox());
        out.push(PlatformRequest::GetClipboard { request_id });
    }
}

pub type ClipboardSetter = Arc<dyn Fn(&str) + Send + Sync>;

pub fn use_clipboard(
    cx: &mut ComponentContext,
    outbox: &PlatformOutbox,
) -> (Signal<String>, ClipboardSetter) {
    let clip_sig = cx.use_signal(String::new());
    let outbox_clone = outbox.clone();
    let set_fn = Arc::new(move |text: &str| {
        Clipboard::set_string(Some(&outbox_clone), text);
    });
    (clip_sig, set_fn)
}

pub struct Alert;

impl Alert {
    pub fn alert(outbox: Option<&PlatformOutbox>, title: &str, message: &str) {
        let out = outbox.unwrap_or(global_outbox());
        out.push(PlatformRequest::ShowAlert {
            title: title.to_string(),
            message: message.to_string(),
        });
    }
}

pub struct AccessibilityInfo;

impl AccessibilityInfo {
    pub fn is_screen_reader_enabled() -> bool {
        false
    }

    pub fn announce_for_accessibility(outbox: Option<&PlatformOutbox>, message: &str) {
        let out = outbox.unwrap_or(global_outbox());
        out.push(PlatformRequest::AnnounceAccessibility {
            message: message.to_string(),
        });
    }
}

pub use crate::theme::I18nManager;

// ---------------------------------------------------------------------------
// 7. NetInfo
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NetInfoState {
    pub is_connected: bool,
    pub is_wifi: bool,
}

static CURRENT_NET_INFO: Mutex<NetInfoState> = Mutex::new(NetInfoState {
    is_connected: true,
    is_wifi: true,
});

pub struct NetInfo;

impl NetInfo {
    pub fn fetch() -> NetInfoState {
        *CURRENT_NET_INFO.lock().unwrap()
    }

    pub fn set(state: NetInfoState) {
        *CURRENT_NET_INFO.lock().unwrap() = state;
    }
}

pub fn use_net_info(cx: &mut ComponentContext) -> Signal<NetInfoState> {
    cx.use_signal(NetInfo::fetch())
}

// ---------------------------------------------------------------------------
// Permissions, Location, Camera (existing Wave 0/M5 hooks)
// ---------------------------------------------------------------------------

pub type PermissionTrigger = Arc<dyn Fn() + Send + Sync>;

/// Reactive hook: `use_permission`
pub fn use_permission(
    cx: &mut ComponentContext,
    outbox: &PlatformOutbox,
    permission: &str,
    request_id: u32,
) -> (Signal<PermissionStatus>, PermissionTrigger) {
    let status_signal = cx.use_signal(PermissionStatus::NotRequested);

    let perm_string = permission.to_string();
    let outbox_clone = outbox.clone();
    let trigger: Arc<dyn Fn() + Send + Sync> = Arc::new(move || {
        outbox_clone.push(PlatformRequest::RequestPermission {
            request_id,
            permission: perm_string.clone(),
        });
    });

    (status_signal, trigger)
}

/// Reactive hook: `use_location`
pub fn use_location(
    cx: &mut ComponentContext,
    outbox: &PlatformOutbox,
    min_interval_ms: u64,
    min_distance_m: f32,
) -> Signal<Option<LocationData>> {
    let location_signal = cx.use_signal(None);

    let outbox_mount = outbox.clone();
    let outbox_cleanup = outbox.clone();

    cx.use_effect(
        move || {
            outbox_mount.push(PlatformRequest::StartLocationUpdates {
                min_interval_ms,
                min_distance_m,
            });

            Some(Box::new(move || {
                outbox_cleanup.push(PlatformRequest::StopLocationUpdates);
            }))
        },
        (),
    );

    location_signal
}

/// Reactive hook: `use_camera`
pub fn use_camera(
    cx: &mut ComponentContext,
    outbox: &PlatformOutbox,
    target_node_id: NodeId,
) -> Signal<CameraState> {
    let camera_signal = cx.use_signal(CameraState::Inactive);

    let outbox_mount = outbox.clone();
    let outbox_cleanup = outbox.clone();

    cx.use_effect(
        move || {
            outbox_mount.push(PlatformRequest::StartCameraPreview { target_node_id });

            Some(Box::new(move || {
                outbox_cleanup.push(PlatformRequest::StopCamera);
            }))
        },
        target_node_id.raw(),
    );

    camera_signal
}

/// Handles an incoming platform response and routes it to the corresponding reactive signals / global state.
pub fn route_platform_response(
    response: &PlatformResponse,
    permission_signal: Option<&Signal<PermissionStatus>>,
    location_signal: Option<&Signal<Option<LocationData>>>,
    camera_signal: Option<&Signal<CameraState>>,
) {
    match response {
        PlatformResponse::PermissionResult {
            granted,
            should_show_rationale,
            ..
        } => {
            if let Some(sig) = permission_signal {
                sig.set(if *granted {
                    PermissionStatus::Granted
                } else {
                    PermissionStatus::Denied {
                        should_show_rationale: *should_show_rationale,
                    }
                });
            }
        }
        PlatformResponse::LocationUpdate {
            latitude,
            longitude,
            accuracy,
            timestamp,
        } => {
            if let Some(sig) = location_signal {
                sig.set(Some(LocationData {
                    latitude: *latitude,
                    longitude: *longitude,
                    accuracy: *accuracy,
                    timestamp: *timestamp,
                }));
            }
        }
        PlatformResponse::CameraReady {
            preview_width,
            preview_height,
            ..
        } => {
            if let Some(sig) = camera_signal {
                sig.set(CameraState::Active {
                    width: *preview_width,
                    height: *preview_height,
                });
            }
        }
        PlatformResponse::DimensionsChange {
            width,
            height,
            scale,
            font_scale,
        } => {
            Dimensions::set(WindowDimensions {
                width: *width,
                height: *height,
                scale: *scale,
                font_scale: *font_scale,
            });
        }
        PlatformResponse::KeyboardChange {
            is_visible,
            height_dp,
        } => {
            Keyboard::set_metrics(KeyboardMetrics {
                is_visible: *is_visible,
                height_dp: *height_dp,
            });
        }
        PlatformResponse::AppStateChange { state } => {
            let app_state = match state {
                0 => AppStateKind::Active,
                1 => AppStateKind::Background,
                _ => AppStateKind::Inactive,
            };
            AppState::set(app_state);
        }
        PlatformResponse::NetInfoUpdate {
            is_connected,
            is_wifi,
        } => {
            NetInfo::set(NetInfoState {
                is_connected: *is_connected,
                is_wifi: *is_wifi,
            });
        }
        PlatformResponse::BackPressed => {
            BackHandler::trigger();
        }
        _ => {}
    }
}
