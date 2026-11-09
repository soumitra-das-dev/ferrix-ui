//! `ferrix-haptics`: VibrationEffect patterns (impact, notification, selection).

use crate::registry::{ModuleCall, ModuleId, NativeModule};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ImpactFeedbackStyle {
    Light = 1,
    Medium = 2,
    Heavy = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NotificationFeedbackType {
    Success = 1,
    Warning = 2,
    Error = 3,
}

pub struct HapticsModule;

impl NativeModule for HapticsModule {
    fn id(&self) -> ModuleId {
        ModuleId::Haptics
    }

    fn name(&self) -> &'static str {
        "ferrix-haptics"
    }

    fn required_permissions(&self) -> &'static [&'static str] {
        &["android.permission.VIBRATE"]
    }
}

impl HapticsModule {
    pub fn impact_call(call_id: u32, style: ImpactFeedbackStyle) -> ModuleCall {
        ModuleCall {
            call_id,
            module_id: ModuleId::Haptics as u16,
            method_id: 1, // impact
            payload: vec![style as u8],
        }
    }

    pub fn notification_call(call_id: u32, feedback_type: NotificationFeedbackType) -> ModuleCall {
        ModuleCall {
            call_id,
            module_id: ModuleId::Haptics as u16,
            method_id: 2, // notification
            payload: vec![feedback_type as u8],
        }
    }

    pub fn selection_call(call_id: u32) -> ModuleCall {
        ModuleCall {
            call_id,
            module_id: ModuleId::Haptics as u16,
            method_id: 3, // selection
            payload: Vec::new(),
        }
    }
}
