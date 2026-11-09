//! `ferrix-notifications`: Local notification scheduling, push tokens, and notification channels.

use crate::registry::{ModuleCall, ModuleId, NativeModule};

pub struct NotificationsModule;

impl NativeModule for NotificationsModule {
    fn id(&self) -> ModuleId {
        ModuleId::Notifications
    }

    fn name(&self) -> &'static str {
        "ferrix-notifications"
    }

    fn required_permissions(&self) -> &'static [&'static str] {
        &["android.permission.POST_NOTIFICATIONS"]
    }
}

impl NotificationsModule {
    pub fn schedule_notification_call(
        call_id: u32,
        title: &str,
        body: &str,
        trigger_in_ms: u64,
    ) -> ModuleCall {
        let mut payload = Vec::new();
        let t_bytes = title.as_bytes();
        payload.extend_from_slice(&(t_bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(t_bytes);
        let b_bytes = body.as_bytes();
        payload.extend_from_slice(&(b_bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(b_bytes);
        payload.extend_from_slice(&trigger_in_ms.to_le_bytes());

        ModuleCall {
            call_id,
            module_id: ModuleId::Notifications as u16,
            method_id: 1, // schedule_notification
            payload,
        }
    }

    pub fn get_push_token_call(call_id: u32) -> ModuleCall {
        ModuleCall {
            call_id,
            module_id: ModuleId::Notifications as u16,
            method_id: 2, // get_push_token
            payload: Vec::new(),
        }
    }

    pub fn set_channel_call(
        call_id: u32,
        channel_id: &str,
        name: &str,
        importance: u8,
    ) -> ModuleCall {
        let mut payload = Vec::new();
        let c_bytes = channel_id.as_bytes();
        payload.extend_from_slice(&(c_bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(c_bytes);
        let n_bytes = name.as_bytes();
        payload.extend_from_slice(&(n_bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(n_bytes);
        payload.push(importance);

        ModuleCall {
            call_id,
            module_id: ModuleId::Notifications as u16,
            method_id: 3, // set_channel
            payload,
        }
    }
}
