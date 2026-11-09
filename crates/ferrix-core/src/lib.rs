//! # ferrix-core
//!
//! Core component model, element tree, builders, and reconciliation engine for Ferrix.

pub mod component;
pub mod element;
pub mod handle;
pub mod node_id;
pub mod platform;
pub mod primitives;
pub mod query_hooks;
pub mod reconcile;
pub mod style;
pub mod theme;

pub use component::{ComponentContext, State};
pub use element::{Element, ElementDef};
pub use handle::ElementHandle;
pub use node_id::NodeIdAllocator;
pub use platform::{
    global_outbox, route_platform_response, use_app_state, use_back_handler, use_camera,
    use_clipboard, use_keyboard, use_location, use_net_info, use_permission, use_window_dimensions,
    AccessibilityInfo, Alert, AppState, AppStateKind, BackHandler, CameraState, Clipboard,
    ClipboardSetter, Dimensions, Keyboard, KeyboardMetrics, Linking, LocationData, NetInfo,
    NetInfoState, PermissionStatus, PermissionTrigger, PixelRatio, Platform, PlatformOutbox, Share,
    Vibration, WindowDimensions,
};
pub use primitives::{
    use_safe_area_insets, ActivityIndicator, Badge, Button, Center, Checkbox, Column, Divider,
    EdgeInsets, FlatList, Fragment, Grid, Icon, Image, ImageBackground, LazyList, LinearGradient,
    Modal, Portal, Pressable, ProgressBar, RefreshControl, Row, SafeAreaView, ScrollView, Slider,
    Spacer, Stack, Switch, Text, TextInput, TouchableHighlight, TouchableOpacity,
    TouchableWithoutFeedback, View, ZStack,
};
pub use query_hooks::{use_mutation, use_query, MutationHandle};
pub use reconcile::{diff_elements, generate_mount_patches};
pub use style::{Color, Style, StyleSheet};
pub use theme::{use_color_scheme, ColorScheme, DynamicColors, I18nManager};

pub mod prelude {
    pub use crate::component::{ComponentContext, State};
    pub use crate::element::{Element, ElementDef};
    pub use crate::handle::ElementHandle;
    pub use crate::node_id::NodeIdAllocator;
    pub use crate::platform::{
        global_outbox, route_platform_response, use_app_state, use_back_handler, use_camera,
        use_clipboard, use_keyboard, use_location, use_net_info, use_permission,
        use_window_dimensions, AccessibilityInfo, Alert, AppState, AppStateKind, BackHandler,
        CameraState, Clipboard, ClipboardSetter, Dimensions, Keyboard, KeyboardMetrics, Linking,
        LocationData, NetInfo, NetInfoState, PermissionStatus, PermissionTrigger, PixelRatio,
        Platform, PlatformOutbox, Share, Vibration, WindowDimensions,
    };
    pub use crate::primitives::{
        use_safe_area_insets, ActivityIndicator, Badge, Button, Center, Checkbox, Column, Divider,
        EdgeInsets, FlatList, Fragment, Grid, Icon, Image, ImageBackground, LazyList,
        LinearGradient, Modal, Portal, Pressable, ProgressBar, RefreshControl, Row, SafeAreaView,
        ScrollView, Slider, Spacer, Stack, Switch, Text, TextInput, TouchableHighlight,
        TouchableOpacity, TouchableWithoutFeedback, View, ZStack,
    };
    pub use crate::query_hooks::{use_mutation, use_query, MutationHandle};
    pub use crate::reconcile::{diff_elements, generate_mount_patches};
    pub use crate::style::{Color, Style, StyleSheet};
    pub use crate::theme::{use_color_scheme, ColorScheme, DynamicColors, I18nManager};
    pub use ferrix_reactivity::prelude::*;
    pub use ferrix_renderer_protocol::{
        Command, CommandBatch, EventType, NodeId, Patch, PatchBatch, Property, ViewType, Visibility,
    };
}
