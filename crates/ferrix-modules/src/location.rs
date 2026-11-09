//! `ferrix-location`: Fused GPS provider, geofencing, and background tracking.

use crate::registry::{ModuleCall, ModuleId, NativeModule};

pub struct LocationModule;

impl NativeModule for LocationModule {
    fn id(&self) -> ModuleId {
        ModuleId::Location
    }

    fn name(&self) -> &'static str {
        "ferrix-location"
    }

    fn required_permissions(&self) -> &'static [&'static str] {
        &[
            "android.permission.ACCESS_FINE_LOCATION",
            "android.permission.ACCESS_COARSE_LOCATION",
        ]
    }
}

impl LocationModule {
    pub fn get_current_position_call(call_id: u32, high_accuracy: bool) -> ModuleCall {
        ModuleCall {
            call_id,
            module_id: ModuleId::Location as u16,
            method_id: 1, // get_current_position
            payload: vec![if high_accuracy { 1 } else { 0 }],
        }
    }

    pub fn start_geofence_call(
        call_id: u32,
        identifier: &str,
        latitude: f64,
        longitude: f64,
        radius_m: f32,
    ) -> ModuleCall {
        let mut payload = Vec::new();
        let bytes = identifier.as_bytes();
        payload.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        payload.extend_from_slice(bytes);
        payload.extend_from_slice(&latitude.to_le_bytes());
        payload.extend_from_slice(&longitude.to_le_bytes());
        payload.extend_from_slice(&radius_m.to_le_bytes());

        ModuleCall {
            call_id,
            module_id: ModuleId::Location as u16,
            method_id: 2, // start_geofence
            payload,
        }
    }
}
