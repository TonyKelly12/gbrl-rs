//! Typed GRBL/GRBL-HAL commands.
//!
//! Line-based commands implement `Display` to produce the exact string sent
//! over serial (the port adds newline). Real-time commands are single bytes
//! with no newline; use `as_byte()` for the wire format.

use std::fmt;

/// Line-based GRBL command. Format with `Display` (e.g. `.to_string()`) to get
/// the serial string. The port layer adds the line terminator.
#[derive(Clone, Debug, PartialEq)]
pub enum GrblCommand {
    /// Request status report (sends `?`).
    StatusRequest,
    /// Request all settings (sends `$$`).
    SettingsRequest,
    /// Run homing cycle (sends `$H`).
    Home,
    /// Unlock after alarm (sends `$X`).
    Unlock,
    /// Jog: `$J=<gcode>`. Pass the full gcode part, e.g. `G21G91X10F500`.
    Jog(String),
    /// Probe cycle: G38.2 or G38.3 with axis, distance, feed. Stored as raw gcode line.
    ProbeCycle(String),
    /// Set work coordinate system zero: G10 L20 Pn X Y Z.
    SetWcsZero { p: u8, x: f64, y: f64, z: f64 },
    /// Activate WCS: G54, G55, G56, G57, G58, G59, G59.1, G59.2, G59.3 (P1..P9).
    ActivateWcs(u8),
    /// Raw g-code line (e.g. from file for streamer). Sent as-is.
    GcodeLine(String),
}

impl fmt::Display for GrblCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GrblCommand::StatusRequest => write!(f, "?"),
            GrblCommand::SettingsRequest => write!(f, "$$"),
            GrblCommand::Home => write!(f, "$H"),
            GrblCommand::Unlock => write!(f, "$X"),
            GrblCommand::Jog(gcode) => write!(f, "$J={}", gcode),
            GrblCommand::ProbeCycle(line) => write!(f, "{}", line),
            GrblCommand::SetWcsZero { p, x, y, z } => {
                write!(f, "G10 L20 P{} X{} Y{} Z{}", p, x, y, z)
            }
            GrblCommand::ActivateWcs(n) => {
                // G54 = P1, G55 = P2, ... G59 = P6, G59.1 = P7, G59.2 = P8, G59.3 = P9
                let s = match *n {
                    1 => "G54",
                    2 => "G55",
                    3 => "G56",
                    4 => "G57",
                    5 => "G58",
                    6 => "G59",
                    7 => "G59.1",
                    8 => "G59.2",
                    9 => "G59.3",
                    _ => return write!(f, "G59.3"), // fallback
                };
                write!(f, "{}", s)
            }
            GrblCommand::GcodeLine(line) => write!(f, "{}", line),
        }
    }
}

/// Real-time single-byte command. Sent without a newline; use `as_byte()` when writing to the port.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RealtimeCommand {
    /// Soft reset (Ctrl-X). Byte 0x18.
    SoftReset,
    /// Safety door. Byte 0x84.
    SafetyDoor,
    /// Jog cancel. Byte 0x85.
    JogCancel,
    /// Feed override 100%. Byte 0x90.
    FeedOverride100,
    /// Feed override +10%. Byte 0x91.
    FeedOverridePlus10,
    /// Feed override -10%. Byte 0x92.
    FeedOverrideMinus10,
}

impl RealtimeCommand {
    /// Returns the single byte to send on the serial line (no newline).
    pub fn as_byte(self) -> u8 {
        match self {
            RealtimeCommand::SoftReset => 0x18,
            RealtimeCommand::SafetyDoor => 0x84,
            RealtimeCommand::JogCancel => 0x85,
            RealtimeCommand::FeedOverride100 => 0x90,
            RealtimeCommand::FeedOverridePlus10 => 0x91,
            RealtimeCommand::FeedOverrideMinus10 => 0x92,
        }
    }
}

impl fmt::Display for RealtimeCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:02X}", self.as_byte())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_request_display() {
        assert_eq!(GrblCommand::StatusRequest.to_string(), "?");
    }

    #[test]
    fn test_settings_request_display() {
        assert_eq!(GrblCommand::SettingsRequest.to_string(), "$$");
    }

    #[test]
    fn test_home_display() {
        assert_eq!(GrblCommand::Home.to_string(), "$H");
    }

    #[test]
    fn test_unlock_display() {
        assert_eq!(GrblCommand::Unlock.to_string(), "$X");
    }

    #[test]
    fn test_jog_display() {
        assert_eq!(
            GrblCommand::Jog("G21G91X10F500".into()).to_string(),
            "$J=G21G91X10F500"
        );
        assert_eq!(
            GrblCommand::Jog("G21G90X0Y0F1000".into()).to_string(),
            "$J=G21G90X0Y0F1000"
        );
    }

    #[test]
    fn test_probe_cycle_display() {
        assert_eq!(
            GrblCommand::ProbeCycle("G38.2 Z-10 F50".into()).to_string(),
            "G38.2 Z-10 F50"
        );
    }

    #[test]
    fn test_set_wcs_zero_display() {
        assert_eq!(
            GrblCommand::SetWcsZero {
                p: 1,
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
            .to_string(),
            "G10 L20 P1 X0 Y0 Z0"
        );
    }

    #[test]
    fn test_activate_wcs_display() {
        assert_eq!(GrblCommand::ActivateWcs(1).to_string(), "G54");
        assert_eq!(GrblCommand::ActivateWcs(6).to_string(), "G59");
        assert_eq!(GrblCommand::ActivateWcs(9).to_string(), "G59.3");
    }

    #[test]
    fn test_gcode_line_display() {
        assert_eq!(
            GrblCommand::GcodeLine("G0 X10 Y20".into()).to_string(),
            "G0 X10 Y20"
        );
    }

    #[test]
    fn test_realtime_soft_reset_byte() {
        assert_eq!(RealtimeCommand::SoftReset.as_byte(), 0x18);
    }

    #[test]
    fn test_realtime_safety_door_byte() {
        assert_eq!(RealtimeCommand::SafetyDoor.as_byte(), 0x84);
    }

    #[test]
    fn test_realtime_jog_cancel_byte() {
        assert_eq!(RealtimeCommand::JogCancel.as_byte(), 0x85);
    }

    #[test]
    fn test_realtime_feed_override_bytes() {
        assert_eq!(RealtimeCommand::FeedOverride100.as_byte(), 0x90);
        assert_eq!(RealtimeCommand::FeedOverridePlus10.as_byte(), 0x91);
        assert_eq!(RealtimeCommand::FeedOverrideMinus10.as_byte(), 0x92);
    }
}
