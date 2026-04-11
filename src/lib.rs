//! cuda-vessel-bridge: Hardware abstraction for physical device connections.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo { pub device_id: String, pub device_type: String, pub capabilities: Vec<String>, pub is_connected: bool, pub last_seen_ms: u64 }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeStatus { pub total_devices: usize, pub connected: usize, pub capabilities: Vec<String>, pub is_healthy: bool }

pub struct VesselBridge { devices: Vec<DeviceInfo> }
impl VesselBridge {
    pub fn new() -> Self { Self { devices: Vec::new() } }
    pub fn register(&mut self, id: &str, dtype: &str, caps: Vec<&str>) {
        self.devices.push(DeviceInfo { device_id: id.into(), device_type: dtype.into(), capabilities: caps.iter().map(|s| s.to_string()).collect(), is_connected: false, last_seen_ms: 0 });
    }
    pub fn connect(&mut self, id: &str, ts_ms: u64) -> bool {
        if let Some(d) = self.devices.iter_mut().find(|d| d.device_id == id) { d.is_connected = true; d.last_seen_ms = ts_ms; true } else { false }
    }
    pub fn disconnect(&mut self, id: &str) { if let Some(d) = self.devices.iter_mut().find(|d| d.device_id == id) { d.is_connected = false; } }
    pub fn find_by_capability(&self, cap: &str) -> Vec<&DeviceInfo> {
        self.devices.iter().filter(|d| d.is_connected && d.capabilities.iter().any(|c| c == cap)).collect()
    }
    pub fn heartbeat(&mut self, id: &str, ts_ms: u64) -> bool {
        if let Some(d) = self.devices.iter_mut().find(|d| d.device_id == id) { d.last_seen_ms = ts_ms; true } else { false }
    }
    pub fn stale_devices(&self, ts_ms: u64, timeout_ms: u64) -> Vec<String> {
        self.devices.iter().filter(|d| d.is_connected && ts_ms.saturating_sub(d.last_seen_ms) > timeout_ms).map(|d| d.device_id.clone()).collect()
    }
    pub fn status(&self) -> BridgeStatus {
        let connected = self.devices.iter().filter(|d| d.is_connected).count();
        let all_caps: Vec<String> = self.devices.iter().flat_map(|d| d.capabilities.clone()).collect();
        let healthy = connected > 0;
        BridgeStatus { total_devices: self.devices.len(), connected, capabilities: all_caps, is_healthy: healthy }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_register_connect() { let mut b = VesselBridge::new(); b.register("cam1", "camera", vec!["vision", "stream"]); b.connect("cam1", 1000); assert_eq!(b.status().connected, 1); }
    #[test] fn test_find_capability() { let mut b = VesselBridge::new(); b.register("a", "sensor", vec!["temp", "pressure"]); b.connect("a", 1000); assert_eq!(b.find_by_capability("temp").len(), 1); assert_eq!(b.find_by_capability("gpu").len(), 0); }
    #[test] fn test_stale_detection() { let mut b = VesselBridge::new(); b.register("s1", "sensor", vec!["temp"]); b.connect("s1", 1000); assert!(b.stale_devices(20000, 5000).contains(&"s1".into())); assert!(b.stale_devices(2000, 5000).is_empty()); }
    #[test] fn test_disconnect() { let mut b = VesselBridge::new(); b.register("d1", "motor", vec!["actuator"]); b.connect("d1", 1000); b.disconnect("d1"); assert_eq!(b.status().connected, 0); }
    #[test] fn test_empty() { let b = VesselBridge::new(); assert_eq!(b.status().total_devices, 0); assert!(!b.status().is_healthy); }
}