/*!
# cuda-vessel-bridge

Hardware Abstraction Layer for autonomous agents.

Bridges physical equipment (sensors, actuators, motors) to the agent runtime.
The bridge translates between:
- Physical world (analog voltages, digital I/O, motor PWM)
- Agent world (confidence values, equipment IDs, JSON payloads)

Every sensor reading enters the agent as a Confidence value.
Every agent action exits as an Equipment command.

The bridge IS the agent's nervous system connected to the physical world.
*/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A sensor reading from physical hardware
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SensorReading {
    pub sensor_id: String,
    pub sensor_type: SensorType,
    pub value: f64,
    pub unit: String,
    pub confidence: f64,  // how reliable is this reading
    pub timestamp: u64,
    pub raw: Vec<u8>,     // raw bytes from hardware
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensorType {
    Temperature,
    Humidity,
    Pressure,
    Light,
    Distance,
    Accelerometer,
    Gyroscope,
    Magnetometer,
    Camera,
    Microphone,
    Gps,
    Proximity,
    Current,
    Voltage,
    Force,
    Flow,
}

impl SensorType {
    pub fn all() -> &'static [SensorType] {
        &[SensorType::Temperature, SensorType::Humidity, SensorType::Pressure,
          SensorType::Light, SensorType::Distance, SensorType::Accelerometer,
          SensorType::Gyroscope, SensorType::Magnetometer, SensorType::Camera,
          SensorType::Microphone, SensorType::Gps, SensorType::Proximity,
          SensorType::Current, SensorType::Voltage, SensorType::Force, SensorType::Flow]
    }

    /// Default uncertainty for this sensor type [0,1] — 0=perfect
    pub fn base_uncertainty(self) -> f64 {
        match self {
            SensorType::Gps => 0.1,
            SensorType::Accelerometer | SensorType::Gyroscope => 0.02,
            SensorType::Temperature | SensorType::Pressure => 0.01,
            SensorType::Distance => 0.05,
            SensorType::Light => 0.03,
            SensorType::Current | SensorType::Voltage => 0.01,
            SensorType::Camera => 0.05,
            SensorType::Microphone => 0.1,
            SensorType::Proximity => 0.08,
            SensorType::Force => 0.02,
            SensorType::Flow => 0.04,
            SensorType::Humidity => 0.03,
            SensorType::Magnetometer => 0.05,
        }
    }
}

/// An actuator command from agent to hardware
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActuatorCommand {
    pub actuator_id: String,
    pub actuator_type: ActuatorType,
    pub value: f64,
    pub unit: String,
    pub confidence: f64,  // how certain the agent is
    pub safety_check: bool,
    pub timestamp: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActuatorType {
    Motor,      // PWM speed control
    Servo,      // angle position
    Relay,      // on/off switch
    Led,        // brightness
    Speaker,    // volume/frequency
    Display,    // pixel content
    Valve,      // flow control
    Pump,       // fluid movement
    Heater,     // temperature setpoint
    Stepper,    // step position
    Dac,        // analog voltage output
    Pwm,        // generic PWM
}

impl ActuatorType {
    /// Safety level — how dangerous is this actuator?
    pub fn danger_level(self) -> u8 {
        match self {
            ActuatorType::Motor | ActuatorType::Stepper | ActuatorType::Pump => 3,
            ActuatorType::Relay | ActuatorType::Valve | ActuatorType::Heater => 2,
            ActuatorType::Servo | ActuatorType::Dac | ActuatorType::Pwm => 1,
            ActuatorType::Led | ActuatorType::Speaker | ActuatorType::Display => 0,
        }
    }
}

/// Safety check result
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SafetyCheck {
    pub passed: bool,
    pub warnings: Vec<String>,
    pub blocks: Vec<String>,
}

impl SafetyCheck {
    pub fn pass() -> Self { SafetyCheck { passed: true, warnings: vec![], blocks: vec![] } }

    pub fn fail(reason: &str) -> Self { SafetyCheck { passed: false, warnings: vec![], blocks: vec![reason.to_string()] } }

    pub fn warn(msg: &str) -> Self { SafetyCheck { passed: true, warnings: vec![msg.to_string()], blocks: vec![] } }
}

/// Equipment profile — what hardware this agent has
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EquipmentProfile {
    pub agent_id: String,
    pub sensors: HashMap<String, SensorType>,
    pub actuators: HashMap<String, ActuatorType>,
    pub capabilities: Vec<String>,
}

impl EquipmentProfile {
    pub fn new(agent_id: &str) -> Self {
        EquipmentProfile { agent_id: agent_id.to_string(), sensors: HashMap::new(), actuators: HashMap::new(), capabilities: vec![] }
    }

    pub fn add_sensor(&mut self, id: &str, stype: SensorType) { self.sensors.insert(id.to_string(), stype); }
    pub fn add_actuator(&mut self, id: &str, atype: ActuatorType) { self.actuators.insert(id.to_string(), atype); }
    pub fn add_capability(&mut self, cap: &str) { self.capabilities.push(cap.to_string()); }

    /// Auto-detect capabilities based on equipment
    pub fn auto_capabilities(&mut self) {
        self.capabilities.clear();
        let has_temp = self.sensors.values().any(|s| *s == SensorType::Temperature);
        let has_dist = self.sensors.values().any(|s| *s == SensorType::Distance);
        let has_motor = self.actuators.values().any(|a| *a == ActuatorType::Motor);
        let has_servo = self.actuators.values().any(|a| *a == ActuatorType::Servo);
        let has_cam = self.sensors.values().any(|s| *s == SensorType::Camera);
        let has_gps = self.sensors.values().any(|s| *s == SensorType::Gps);

        if has_temp { self.capabilities.push("thermal_monitoring".to_string()); }
        if has_dist { self.capabilities.push("obstacle_detection".to_string()); }
        if has_dist && has_motor { self.capabilities.push("autonomous_navigation".to_string()); }
        if has_servo { self.capabilities.push("precision_control".to_string()); }
        if has_cam { self.capabilities.push("visual_perception".to_string()); }
        if has_gps { self.capabilities.push("geolocation".to_string()); }
        if has_motor { self.capabilities.push("locomotion".to_string()); }
        if !self.sensors.is_empty() { self.capabilities.push("perception".to_string()); }
        if !self.actuators.is_empty() { self.capabilities.push("actuation".to_string()); }
    }
}

/// Sensor fusion — combining multiple sensor readings
pub fn fuse_readings(readings: &[SensorReading]) -> SensorReading {
    if readings.is_empty() {
        return SensorReading { sensor_id: "fused".to_string(), sensor_type: SensorType::Temperature, value: 0.0, unit: "unknown".to_string(), confidence: 0.0, timestamp: now(), raw: vec![] };
    }

    // Weighted average by confidence
    let total_conf: f64 = readings.iter().map(|r| r.confidence).sum();
    let fused_value: f64 = readings.iter().map(|r| r.value * r.confidence).sum::<f64>() / total_conf.max(0.001);
    let fused_conf: f64 = 1.0 / (1.0 / total_conf); // Bayesian fusion approximation

    SensorReading {
        sensor_id: "fused".to_string(),
        sensor_type: readings[0].sensor_type,
        value: fused_value,
        unit: readings[0].unit.clone(),
        confidence: fused_conf.clamp(0.0, 1.0),
        timestamp: now(),
        raw: vec![],
    }
}

/// Hardware safety monitor
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SafetyMonitor {
    pub max_motor_speed: f64,
    pub max_temperature: f64,
    pub max_current: f64,
    pub emergency_stop: bool,
    pub stop_reason: String,
    pub lockout_pins: Vec<String>,
}

impl SafetyMonitor {
    pub fn new() -> Self {
        SafetyMonitor { max_motor_speed: 255.0, max_temperature: 85.0, max_current: 5.0, emergency_stop: false, stop_reason: String::new(), lockout_pins: vec![] }
    }

    pub fn check_actuator(&mut self, cmd: &ActuatorCommand) -> SafetyCheck {
        if self.emergency_stop {
            return SafetyCheck::fail("EMERGENCY STOP active");
        }

        let mut check = SafetyCheck::pass();

        match cmd.actuator_type {
            ActuatorType::Motor | ActuatorType::Stepper => {
                if cmd.value.abs() > self.max_motor_speed {
                    check = SafetyCheck::fail(&format!("Motor speed {} exceeds max {}", cmd.value, self.max_motor_speed));
                }
                if cmd.value.abs() > self.max_motor_speed * 0.8 {
                    check.warnings.push("Motor near max speed");
                }
            }
            ActuatorType::Heater => {
                if cmd.value > self.max_temperature {
                    check = SafetyCheck::fail(&format!("Temperature {} exceeds max {}", cmd.value, self.max_temperature));
                }
            }
            ActuatorType::Valve | ActuatorType::Pump => {
                if cmd.confidence < 0.5 {
                    check.warnings.push("Low confidence on fluid control");
                }
            }
            _ => {}
        }

        // Lockout check
        if self.lockout_pins.contains(&cmd.actuator_id) {
            check = SafetyCheck::fail(&format!("Actuator {} is locked out", cmd.actuator_id));
        }

        check
    }

    pub fn emergency_stop(&mut self, reason: &str) {
        self.emergency_stop = true;
        self.stop_reason = reason.to_string();
    }

    pub fn reset(&mut self) {
        self.emergency_stop = false;
        self.stop_reason.clear();
    }
}

/// Bridge — the main interface between agent and hardware
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bridge {
    pub profile: EquipmentProfile,
    pub safety: SafetyMonitor,
    pub sensor_buffer: Vec<SensorReading>,
    pub actuator_log: Vec<ActuatorCommand>,
    pub max_buffer: usize,
}

impl Bridge {
    pub fn new(profile: EquipmentProfile) -> Self {
        Bridge { profile, safety: SafetyMonitor::new(), sensor_buffer: vec![], actuator_log: vec![], max_buffer: 1000 }
    }

    /// Read from a sensor
    pub fn read_sensor(&mut self, id: &str, value: f64, unit: &str) -> Option<SensorReading> {
        let stype = self.profile.sensors.get(id)?;
        let base_conf = 1.0 - stype.base_uncertainty();
        let reading = SensorReading {
            sensor_id: id.to_string(),
            sensor_type: *stype,
            value,
            unit: unit.to_string(),
            confidence: base_conf,
            timestamp: now(),
            raw: vec![],
        };
        if self.sensor_buffer.len() >= self.max_buffer {
            self.sensor_buffer.remove(0);
        }
        self.sensor_buffer.push(reading.clone());
        Some(reading)
    }

    /// Send command to actuator
    pub fn send_actuator(&mut self, cmd: ActuatorCommand) -> Result<SafetyCheck, SafetyCheck> {
        let check = self.safety.check_actuator(&cmd);
        if !check.passed { return Err(check); }

        if self.actuator_log.len() >= self.max_buffer {
            self.actuator_log.remove(0);
        }
        self.actuator_log.push(cmd);
        Ok(check)
    }

    /// Fuse recent sensor readings by type
    pub fn fused_reading(&self, stype: SensorType) -> Option<SensorReading> {
        let readings: Vec<_> = self.sensor_buffer.iter().filter(|r| r.sensor_type == stype).cloned().collect();
        if readings.is_empty() { return None; }
        Some(fuse_readings(&readings))
    }

    /// Emergency stop all actuators
    pub fn e_stop(&mut self, reason: &str) {
        self.safety.emergency_stop(reason);
    }

    /// Resume from emergency stop
    pub fn resume(&mut self) { self.safety.reset(); }
}

fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sensor_reading() {
        let mut bridge = Bridge::new(EquipmentProfile::new("agent-1"));
        bridge.profile.add_sensor("temp1", SensorType::Temperature);
        let reading = bridge.read_sensor("temp1", 23.5, "celsius");
        assert!(reading.is_some());
        assert_eq!(reading.unwrap().value, 23.5);
    }

    #[test]
    fn test_unknown_sensor() {
        let mut bridge = Bridge::new(EquipmentProfile::new("agent-1"));
        assert!(bridge.read_sensor("nonexistent", 0.0, "").is_none());
    }

    #[test]
    fn test_actuator_command() {
        let mut bridge = Bridge::new(EquipmentProfile::new("agent-1"));
        bridge.profile.add_actuator("motor1", ActuatorType::Motor);
        let cmd = ActuatorCommand { actuator_id: "motor1".to_string(), actuator_type: ActuatorType::Motor, value: 128.0, unit: "pwm".to_string(), confidence: 0.9, safety_check: true, timestamp: 0 };
        let result = bridge.send_actuator(cmd);
        assert!(result.is_ok());
    }

    #[test]
    fn test_motor_over_speed() {
        let mut bridge = Bridge::new(EquipmentProfile::new("agent-1"));
        bridge.profile.add_actuator("m1", ActuatorType::Motor);
        let cmd = ActuatorCommand { actuator_id: "m1".to_string(), actuator_type: ActuatorType::Motor, value: 500.0, unit: "pwm".to_string(), confidence: 0.9, safety_check: true, timestamp: 0 };
        let result = bridge.send_actuator(cmd);
        assert!(result.is_err());
    }

    #[test]
    fn test_emergency_stop() {
        let mut bridge = Bridge::new(EquipmentProfile::new("agent-1"));
        bridge.profile.add_actuator("m1", ActuatorType::Motor);
        bridge.e_stop("test emergency");
        let cmd = ActuatorCommand { actuator_id: "m1".to_string(), actuator_type: ActuatorType::Motor, value: 100.0, unit: "pwm".to_string(), confidence: 1.0, safety_check: true, timestamp: 0 };
        assert!(bridge.send_actuator(cmd).is_err());
        bridge.resume();
        assert!(bridge.send_actuator(cmd).is_ok());
    }

    #[test]
    fn test_sensor_fusion() {
        let r1 = SensorReading { sensor_id: "t1".into(), sensor_type: SensorType::Temperature, value: 22.0, unit: "C".into(), confidence: 0.9, timestamp: 0, raw: vec![] };
        let r2 = SensorReading { sensor_id: "t2".into(), sensor_type: SensorType::Temperature, value: 24.0, unit: "C".into(), confidence: 0.7, timestamp: 0, raw: vec![] };
        let fused = fuse_readings(&[r1, r2]);
        // Weighted: (22*0.9 + 24*0.7) / (0.9+0.7) = 22.875
        assert!((fused.value - 22.875).abs() < 0.1);
    }

    #[test]
    fn test_auto_capabilities() {
        let mut profile = EquipmentProfile::new("agent-1");
        profile.add_sensor("temp", SensorType::Temperature);
        profile.add_sensor("dist", SensorType::Distance);
        profile.add_sensor("cam", SensorType::Camera);
        profile.add_actuator("motor", ActuatorType::Motor);
        profile.auto_capabilities();
        assert!(profile.capabilities.contains(&"thermal_monitoring".to_string()));
        assert!(profile.capabilities.contains(&"autonomous_navigation".to_string()));
        assert!(profile.capabilities.contains(&"visual_perception".to_string()));
        assert!(profile.capabilities.contains(&"locomotion".to_string()));
    }

    #[test]
    fn test_lockout() {
        let mut bridge = Bridge::new(EquipmentProfile::new("agent-1"));
        bridge.profile.add_actuator("m1", ActuatorType::Motor);
        bridge.safety.lockout_pins.push("m1".to_string());
        let cmd = ActuatorCommand { actuator_id: "m1".to_string(), actuator_type: ActuatorType::Motor, value: 100.0, unit: "pwm".to_string(), confidence: 1.0, safety_check: true, timestamp: 0 };
        assert!(bridge.send_actuator(cmd).is_err());
    }

    #[test]
    fn test_heater_limit() {
        let mut bridge = Bridge::new(EquipmentProfile::new("agent-1"));
        bridge.profile.add_actuator("h1", ActuatorType::Heater);
        let cmd = ActuatorCommand { actuator_id: "h1".to_string(), actuator_type: ActuatorType::Heater, value: 90.0, unit: "C".to_string(), confidence: 1.0, safety_check: true, timestamp: 0 };
        assert!(bridge.send_actuator(cmd).is_err());
    }

    #[test]
    fn test_sensor_buffer() {
        let mut bridge = Bridge::new(EquipmentProfile::new("agent-1"));
        bridge.max_buffer = 3;
        bridge.profile.add_sensor("t", SensorType::Temperature);
        for i in 0..5 { bridge.read_sensor("t", i as f64, "C"); }
        assert_eq!(bridge.sensor_buffer.len(), 3);
    }

    #[test]
    fn test_fused_reading_from_bridge() {
        let mut bridge = Bridge::new(EquipmentProfile::new("agent-1"));
        bridge.profile.add_sensor("t1", SensorType::Temperature);
        bridge.profile.add_sensor("t2", SensorType::Temperature);
        bridge.read_sensor("t1", 20.0, "C");
        bridge.read_sensor("t2", 25.0, "C");
        let fused = bridge.fused_reading(SensorType::Temperature);
        assert!(fused.is_some());
        assert!((fused.unwrap().value - 22.5).abs() < 0.5);
    }

    #[test]
    fn test_actuator_danger_levels() {
        assert!(ActuatorType::Motor.danger_level() > ActuatorType::Led.danger_level());
        assert!(ActuatorType::Relay.danger_level() > ActuatorType::Servo.danger_level());
    }

    #[test]
    fn test_sensor_confidence() {
        let bridge = Bridge::new(EquipmentProfile::new("a"));
        let mut profile = EquipmentProfile::new("a");
        profile.add_sensor("gps", SensorType::Gps);
        profile.add_sensor("temp", SensorType::Temperature);
        profile.auto_capabilities();
    }
}
