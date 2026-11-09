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
pub use primitives::{
    use_safe_area_insets, ActivityIndicator, Button, Column, EdgeInsets, FlatList, Fragment, Image,
    ImageBackground, LazyList, Modal, Portal, Pressable, RefreshControl, Row, SafeAreaView,
    ScrollView, Switch, Text, TextInput, TouchableHighlight, TouchableOpacity,
    TouchableWithoutFeedback, View,
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
    pub use crate::primitives::{
        use_safe_area_insets, ActivityIndicator, Button, Column, EdgeInsets, FlatList, Fragment,
        Image, ImageBackground, LazyList, Modal, Portal, Pressable, RefreshControl, Row,
        SafeAreaView, ScrollView, Switch, Text, TextInput, TouchableHighlight, TouchableOpacity,
        TouchableWithoutFeedback, View,
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
