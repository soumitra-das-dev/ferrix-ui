//! Integration tests for Wave 3 Platform APIs and Hooks:
//! Dimensions, PixelRatio, Platform, Keyboard, AppState, BackHandler,
//! Linking, Share, Vibration, Clipboard, Alert, AccessibilityInfo, NetInfo.

use ferrix_core::prelude::*;
use ferrix_renderer_protocol::platform::{PlatformRequest, PlatformResponse};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[test]
fn test_dimensions_and_pixel_ratio() {
    Dimensions::set(WindowDimensions {
        width: 360.0,
        height: 800.0,
        scale: 3.0,
        font_scale: 1.2,
    });

    let dims = Dimensions::get();
    assert_eq!(dims.width, 360.0);
    assert_eq!(dims.height, 800.0);
    assert_eq!(dims.scale, 3.0);
    assert_eq!(dims.font_scale, 1.2);

    assert_eq!(PixelRatio::get(), 3.0);
    assert_eq!(PixelRatio::get_font_scale(), 1.2);

    // Test pixel rounding: (10.2 * 3.0) = 30.6 -> round to 31.0 -> / 3.0 = 10.333333
    let rounded = PixelRatio::round_to_nearest_pixel(10.2);
    assert!((rounded - (31.0 / 3.0)).abs() < 1e-5);

    // Test platform response update
    let update_resp = PlatformResponse::DimensionsChange {
        width: 412.0,
        height: 915.0,
        scale: 2.625,
        font_scale: 1.0,
    };
    route_platform_response(&update_resp, None, None, None);

    let updated_dims = Dimensions::get();
    assert_eq!(updated_dims.width, 412.0);
    assert_eq!(updated_dims.height, 915.0);
}

#[test]
fn test_platform_metadata() {
    assert_eq!(Platform::OS, "android");
    assert!(Platform::is_android());
    assert_eq!(Platform::version(), 34);

    let selected = Platform::select("android_specific", "default_val");
    assert_eq!(selected, "android_specific");
}

#[test]
fn test_keyboard_api_and_updates() {
    let outbox = PlatformOutbox::new();

    Keyboard::set_metrics(KeyboardMetrics {
        is_visible: false,
        height_dp: 0.0,
    });
    assert!(!Keyboard::is_visible());

    // Dismiss keyboard sends typed request
    Keyboard::dismiss(Some(&outbox));
    let requests = outbox.drain();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0], PlatformRequest::DismissKeyboard);

    // Platform response updates keyboard metrics
    let resp = PlatformResponse::KeyboardChange {
        is_visible: true,
        height_dp: 290.0,
    };
    route_platform_response(&resp, None, None, None);

    assert!(Keyboard::is_visible());
    assert_eq!(Keyboard::metrics().height_dp, 290.0);
}

#[test]
fn test_app_state_api_and_transitions() {
    AppState::set(AppStateKind::Active);
    assert_eq!(AppState::current(), AppStateKind::Active);

    // Native sends background event (state = 1)
    route_platform_response(
        &PlatformResponse::AppStateChange { state: 1 },
        None,
        None,
        None,
    );
    assert_eq!(AppState::current(), AppStateKind::Background);

    // Native sends inactive event (state = 2)
    route_platform_response(
        &PlatformResponse::AppStateChange { state: 2 },
        None,
        None,
        None,
    );
    assert_eq!(AppState::current(), AppStateKind::Inactive);
}

#[test]
fn test_back_handler_event_interception() {
    let handled = Arc::new(AtomicBool::new(false));
    let handled_clone = handled.clone();

    BackHandler::add_listener(Arc::new(move || {
        handled_clone.store(true, Ordering::SeqCst);
        true // Handled/consumed
    }));

    route_platform_response(&PlatformResponse::BackPressed, None, None, None);
    assert!(handled.load(Ordering::SeqCst));
}

#[test]
fn test_linking_share_vibration_clipboard_alert_requests() {
    let outbox = PlatformOutbox::new();

    // Linking
    assert!(Linking::can_open_url("https://ferrix.dev"));
    assert!(Linking::can_open_url("tel:+1234567890"));
    assert!(!Linking::can_open_url("ftp://example.com"));

    Linking::open_url(Some(&outbox), "https://ferrix.dev");
    let reqs = outbox.drain();
    assert_eq!(
        reqs,
        vec![PlatformRequest::OpenUrl {
            url: "https://ferrix.dev".to_string()
        }]
    );

    // Share
    Share::share(Some(&outbox), "Ferrix Framework", "Check out Ferrix!");
    let reqs = outbox.drain();
    assert_eq!(
        reqs,
        vec![PlatformRequest::Share {
            title: "Ferrix Framework".to_string(),
            message: "Check out Ferrix!".to_string()
        }]
    );

    // Vibration
    Vibration::vibrate(Some(&outbox), 400);
    Vibration::cancel(Some(&outbox));
    let reqs = outbox.drain();
    assert_eq!(
        reqs,
        vec![
            PlatformRequest::Vibrate { duration_ms: 400 },
            PlatformRequest::Vibrate { duration_ms: 0 }
        ]
    );

    // Clipboard
    Clipboard::set_string(Some(&outbox), "Ferrix Clipboard Text");
    Clipboard::get_string(Some(&outbox), 42);
    let reqs = outbox.drain();
    assert_eq!(
        reqs,
        vec![
            PlatformRequest::SetClipboard {
                text: "Ferrix Clipboard Text".to_string()
            },
            PlatformRequest::GetClipboard { request_id: 42 }
        ]
    );

    // Alert
    Alert::alert(Some(&outbox), "Notice", "Low Battery Warning");
    let reqs = outbox.drain();
    assert_eq!(
        reqs,
        vec![PlatformRequest::ShowAlert {
            title: "Notice".to_string(),
            message: "Low Battery Warning".to_string()
        }]
    );

    // Accessibility announcement
    AccessibilityInfo::announce_for_accessibility(Some(&outbox), "Screen loaded");
    let reqs = outbox.drain();
    assert_eq!(
        reqs,
        vec![PlatformRequest::AnnounceAccessibility {
            message: "Screen loaded".to_string()
        }]
    );
}

#[test]
fn test_net_info_monitoring() {
    NetInfo::set(NetInfoState {
        is_connected: true,
        is_wifi: true,
    });
    assert!(NetInfo::fetch().is_connected);
    assert!(NetInfo::fetch().is_wifi);

    route_platform_response(
        &PlatformResponse::NetInfoUpdate {
            is_connected: false,
            is_wifi: false,
        },
        None,
        None,
        None,
    );

    let net = NetInfo::fetch();
    assert!(!net.is_connected);
    assert!(!net.is_wifi);
}
