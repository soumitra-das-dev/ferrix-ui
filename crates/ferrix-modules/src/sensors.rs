//! `ferrix-sensors`: Accelerometer, Gyroscope, Magnetometer, Barometer, and Pedometer.

use crate::registry::{ModuleCall, ModuleId, NativeModule};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SensorType {
    Accelerometer = 1,
    Gyroscope = 2,
    Magnetometer = 3,
    Barometer = 4,
    Pedometer = 5,
}

pub struct SensorsModule;

impl NativeModule for SensorsModule {
    fn id(&self) -> ModuleId {
        ModuleId::Sensors
    }

    fn name(&self) -> &'static str {
        "ferrix-sensors"
    }

    fn required_permissions(&self) -> &'static [&'static str] {
        &["android.permission.ACTIVITY_RECOGNITION"]
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThreeAxisData {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub timestamp: u64,
}

impl ThreeAxisData {
    pub fn decode(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 20 {
            return None;
        }
        let x = f32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let y = f32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let z = f32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let timestamp = u64::from_le_bytes(bytes[12..20].try_into().unwrap());
        Some(Self { x, y, z, timestamp })
    }
}

impl SensorsModule {
    pub fn start_sensor_call(
        call_id: u32,
        sensor_type: SensorType,
        interval_ms: u32,
    ) -> ModuleCall {
        let mut payload = Vec::with_capacity(5);
        payload.push(sensor_type as u8);
        payload.extend_from_slice(&interval_ms.to_le_bytes());

        ModuleCall {
            call_id,
            module_id: ModuleId::Sensors as u16,
            method_id: 1, // start_sensor
            payload,
        }
    }

    pub fn stop_sensor_call(call_id: u32, sensor_type: SensorType) -> ModuleCall {
        ModuleCall {
            call_id,
            module_id: ModuleId::Sensors as u16,
            method_id: 2, // stop_sensor
            payload: vec![sensor_type as u8],
        }
    }
}
