//! Machine state types for GRBL-HAL.
//!
//! Types only — no logic. Used by the parser and eventually by the poller
//! and other tasks that hold or broadcast machine status.

use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Position in machine or work coordinates.
/// Supports optional A (rotary) axis for GRBL-HAL.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    /// Rotary axis, if present.
    pub a: Option<f64>,
}

/// Reason for Hold state (e.g. feed hold, safety door).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HoldReason {
    FeedHold,
    SafetyDoor,
    /// GRBL-HAL may report other hold reasons; capture as string.
    Other(String),
}

/// Alarm code from GRBL-HAL. Matches alarms.h (codes 1–21). Unknown codes
/// map to `Unknown(n)`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlarmCode {
    HardLimit,                      // 1
    SoftLimit,                      // 2
    AbortCycle,                     // 3
    ProbeFailInitial,               // 4
    ProbeFailContact,               // 5
    HomingFailReset,                // 6
    HomingFailDoor,                 // 7
    FailPulloff,                    // 8
    HomingFailApproach,             // 9
    EStop,                          // 10
    HomingRequired,                 // 11
    LimitsEngaged,                  // 12
    ProbeProtect,                   // 13
    Spindle,                        // 14
    HomingFailAutoSquaringApproach, // 15
    SelftestFailed,                 // 16
    MotorFault,                     // 17
    HomingFail,                     // 18
    ModbusException,                // 19
    ExpanderException,              // 20
    NvsFailed,                      // 21
    /// Unknown or extended GRBL-HAL alarm code (e.g. 22–255).
    Unknown(u8),
}

impl From<u8> for AlarmCode {
    fn from(n: u8) -> Self {
        match n {
            1 => AlarmCode::HardLimit,
            2 => AlarmCode::SoftLimit,
            3 => AlarmCode::AbortCycle,
            4 => AlarmCode::ProbeFailInitial,
            5 => AlarmCode::ProbeFailContact,
            6 => AlarmCode::HomingFailReset,
            7 => AlarmCode::HomingFailDoor,
            8 => AlarmCode::FailPulloff,
            9 => AlarmCode::HomingFailApproach,
            10 => AlarmCode::EStop,
            11 => AlarmCode::HomingRequired,
            12 => AlarmCode::LimitsEngaged,
            13 => AlarmCode::ProbeProtect,
            14 => AlarmCode::Spindle,
            15 => AlarmCode::HomingFailAutoSquaringApproach,
            16 => AlarmCode::SelftestFailed,
            17 => AlarmCode::MotorFault,
            18 => AlarmCode::HomingFail,
            19 => AlarmCode::ModbusException,
            20 => AlarmCode::ExpanderException,
            21 => AlarmCode::NvsFailed,
            _ => AlarmCode::Unknown(n),
        }
    }
}

/// Input pin state (limit switches, probe). GRBL-HAL reports these
/// in status when configured; we use booleans for the PROVerXL layout
/// (X, Y, Z limits + probe).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinState {
    pub limit_x: bool,
    pub limit_y: bool,
    pub limit_z: bool,
    pub probe: bool,
}

/// High-level machine state from status string.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MachineState {
    Idle,
    Run,
    Hold(HoldReason),
    Jog,
    Alarm(AlarmCode),
    /// GRBL-HAL uses "Door" for safety door; we map to Hold(SafetyDoor) or
    /// keep a variant if needed elsewhere.
    Door,
    Check,
    Home,
    Sleep,
    Unknown(String),
}

/// Full machine status parsed from a single `?` status response.
#[derive(Clone, Debug, Serialize)]
pub struct MachineStatus {
    pub state: MachineState,
    pub machine_pos: Position,
    pub work_pos: Position,
    pub feed_rate: f64,
    pub spindle_speed: f64,
    pub input_pins: PinState,
    /// Set by the caller (e.g. poller) when the status was received;
    /// not serialized (Instant has no meaningful serialization).
    #[serde(skip_serializing)]
    pub last_updated: Instant,
}

impl MachineStatus {
    /// Initial status before any poll (e.g. for shared state when starting the poller).
    pub fn idle() -> Self {
        Self {
            state: MachineState::Idle,
            machine_pos: Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                a: None,
            },
            work_pos: Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                a: None,
            },
            feed_rate: 0.0,
            spindle_speed: 0.0,
            input_pins: PinState::default(),
            last_updated: Instant::now(),
        }
    }
}

impl<'de> Deserialize<'de> for MachineStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// Helper to deserialize all fields except `last_updated`, then set it to `Instant::now()`.
        #[derive(Deserialize)]
        struct MachineStatusDto {
            state: MachineState,
            machine_pos: Position,
            work_pos: Position,
            feed_rate: f64,
            spindle_speed: f64,
            input_pins: PinState,
        }
        let dto = MachineStatusDto::deserialize(deserializer)?;
        Ok(MachineStatus {
            state: dto.state,
            machine_pos: dto.machine_pos,
            work_pos: dto.work_pos,
            feed_rate: dto.feed_rate,
            spindle_speed: dto.spindle_speed,
            input_pins: dto.input_pins,
            last_updated: Instant::now(),
        })
    }
}

