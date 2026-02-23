//! Pure parsing for GRBL-HAL responses.
//!
//! No async, no I/O — only string/line parsing. Used by the poller and
//! other tasks that receive data from the serial port.

use super::state::*;
use std::collections::HashMap;
use std::time::Instant;
use thiserror::Error;

/// Errors produced when parsing GRBL-HAL response strings.
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("invalid status line: {0}")]
    InvalidStatus(String),
    #[error("invalid position: {0}")]
    InvalidPosition(String),
    #[error("invalid settings line: {0}")]
    InvalidSettingsLine(String),
    #[error("invalid alarm message: {0}")]
    InvalidAlarm(String),
}

/// Parses a single real-time status line (response to `?`).
///
/// Input format: `<State|MPos:x,y,z[,a]|WPos:x,y,z[,a]|FS:feed,spindle>`
/// Angle brackets are optional. GRBL-HAL uses comma-separated FS: feed,spindle.
///
/// Caller provides `last_updated` (e.g. `Instant::now()`) so the poller can
/// set the exact receive time.
pub fn parse_status(line: &str, last_updated: Instant) -> Result<MachineStatus, ParseError> {
    let s = line.trim();
    // Strip optional angle brackets.
    let s = s.strip_prefix('<').unwrap_or(s).strip_suffix('>').unwrap_or(s);
    let parts: Vec<&str> = s.split('|').collect();
    let state_token = parts.first().map(|p| p.trim()).unwrap_or("");
    if state_token.is_empty() {
        return Err(ParseError::InvalidStatus("empty status".into()));
    }

    let state = parse_state(state_token)?;
    let mut machine_pos = Position {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        a: None,
    };
    let mut work_pos = Position {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        a: None,
    };
    let mut feed_rate = 0.0_f64;
    let mut spindle_speed = 0.0_f64;

    for part in parts.iter().skip(1) {
        let part = part.trim();
        if let Some(pos_str) = part.strip_prefix("MPos:") {
            machine_pos = parse_position(pos_str)?;
        } else if let Some(pos_str) = part.strip_prefix("WPos:") {
            work_pos = parse_position(pos_str)?;
        } else if let Some(fs_str) = part.strip_prefix("FS:") {
            let (feed, spindle) = parse_fs(fs_str)?;
            feed_rate = feed;
            spindle_speed = spindle;
        }
        // Optional: parse Pn: or other pin state if present in GRBL-HAL
    }

    Ok(MachineStatus {
        state,
        machine_pos,
        work_pos,
        feed_rate,
        spindle_speed,
        input_pins: PinState::default(),
        last_updated,
    })
}

/// Parses the state token (first segment). GRBL-HAL states: Idle, Run, Hold,
/// Jog, Alarm, Door, Check, Home, Sleep. Door/Check map to Hold or dedicated variants.
fn parse_state(s: &str) -> Result<MachineState, ParseError> {
    let s = s.trim();
    // Substate in parentheses, e.g. "Hold:0" or "Alarm:1"
    let (base, rest) = match s.find(':') {
        Some(i) => (&s[..i], Some(&s[i + 1..])),
        None => (s, None),
    };
    let base = base.trim();
    match base {
        "Idle" => Ok(MachineState::Idle),
        "Run" => Ok(MachineState::Run),
        "Hold" => {
            let reason = rest
                .and_then(|r| r.trim().parse::<u8>().ok())
                .map(|_| HoldReason::FeedHold)
                .unwrap_or(HoldReason::FeedHold);
            Ok(MachineState::Hold(reason))
        }
        "Jog" => Ok(MachineState::Jog),
        "Alarm" => {
            let code = rest
                .and_then(|r| r.trim().parse::<u8>().ok())
                .map(AlarmCode::from)
                .unwrap_or(AlarmCode::Unknown(0));
            Ok(MachineState::Alarm(code))
        }
        "Door" => Ok(MachineState::Door),
        "Check" => Ok(MachineState::Check),
        "Home" => Ok(MachineState::Home),
        "Sleep" => Ok(MachineState::Sleep),
        _ => Ok(MachineState::Unknown(s.to_string())),
    }
}

/// Parses "x,y,z" or "x,y,z,a" into Position.
fn parse_position(s: &str) -> Result<Position, ParseError> {
    let parts: Vec<&str> = s.split(',').map(str::trim).collect();
    if parts.len() < 3 {
        return Err(ParseError::InvalidPosition(format!(
            "expected at least x,y,z, got: {}",
            s
        )));
    }
    let x: f64 = parts[0]
        .parse()
        .map_err(|_| ParseError::InvalidPosition(format!("invalid x: {}", parts[0])))?;
    let y: f64 = parts[1]
        .parse()
        .map_err(|_| ParseError::InvalidPosition(format!("invalid y: {}", parts[1])))?;
    let z: f64 = parts[2]
        .parse()
        .map_err(|_| ParseError::InvalidPosition(format!("invalid z: {}", parts[2])))?;
    let a = parts.get(3).and_then(|s| s.parse().ok());
    Ok(Position { x, y, z, a })
}

/// Parses "feed,spindle" (GRBL-HAL FS field).
fn parse_fs(s: &str) -> Result<(f64, f64), ParseError> {
    let parts: Vec<&str> = s.split(',').map(str::trim).collect();
    if parts.len() < 2 {
        return Err(ParseError::InvalidStatus(format!(
            "FS expected feed,spindle: {}",
            s
        )));
    }
    let feed: f64 = parts[0]
        .parse()
        .map_err(|_| ParseError::InvalidStatus(format!("invalid feed: {}", parts[0])))?;
    let spindle: f64 = parts[1]
        .parse()
        .map_err(|_| ParseError::InvalidStatus(format!("invalid spindle: {}", parts[1])))?;
    Ok((feed, spindle))
}

/// Parses an alarm message string into an alarm code.
///
/// GRBL-HAL typically sends "ALARM:n" or "error:n". Accepts a line that
/// contains a numeric alarm code (e.g. after "ALARM:" or "error:").
pub fn parse_alarm_code(s: &str) -> Result<AlarmCode, ParseError> {
    let s = s.trim();
    // Try "ALARM: n" or "error:n" style
    let num_str = s
        .strip_prefix("ALARM:")
        .or_else(|| s.strip_prefix("ALARM: "))
        .or_else(|| s.strip_prefix("error:"))
        .or_else(|| s.strip_prefix("error: "))
        .map(str::trim)
        .unwrap_or(s);
    let n: u8 = num_str
        .parse()
        .map_err(|_| ParseError::InvalidAlarm(s.to_string()))?;
    Ok(AlarmCode::from(n))
}

/// Parsed settings from a `$$` response: setting number -> value string.
/// Values are kept as strings; callers may interpret as int/float/bool as needed.
#[derive(Clone, Debug, Default)]
pub struct GrblSettings {
    pub raw: HashMap<u32, String>,
}

/// Parses the lines of a `$$` settings response.
///
/// Each line should be `$N=value`. Empty lines and a trailing `ok` are
/// skipped. Malformed lines are skipped (no error) so we tolerate
/// occasional garbage; for strict parsing we could return ParseError instead.
pub fn parse_settings(lines: &str) -> Result<GrblSettings, ParseError> {
    let mut raw = HashMap::new();
    for line in lines.lines() {
        let line = line.trim();
        if line.is_empty() || line == "ok" {
            continue;
        }
        if let Some(rest) = line.strip_prefix('$') {
            if let Some((num_str, value)) = rest.split_once('=') {
                if let Ok(n) = num_str.trim().parse::<u32>() {
                    raw.insert(n, value.trim().to_string());
                }
            }
        }
    }
    Ok(GrblSettings { raw })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_parse_status_idle_bare() {
        let line = "Idle|MPos:0,0,0|WPos:0,0,0|FS:0,0";
        let t = Instant::now();
        let st = parse_status(line, t).unwrap();
        assert!(matches!(st.state, MachineState::Idle));
        assert_eq!(st.machine_pos.x, 0.0);
        assert_eq!(st.work_pos.z, 0.0);
        assert_eq!(st.feed_rate, 0.0);
        assert_eq!(st.spindle_speed, 0.0);
    }

    #[test]
    fn test_parse_status_with_angle_brackets() {
        let line = "<Idle|MPos:0.000,0.000,0.000|WPos:0.000,0.000,0.000|FS:0,0>";
        let t = Instant::now();
        let st = parse_status(line, t).unwrap();
        assert!(matches!(st.state, MachineState::Idle));
        assert_eq!(st.machine_pos.x, 0.0);
    }

    #[test]
    fn test_parse_status_with_fourth_axis() {
        let line = "Idle|MPos:0,0,0,0|WPos:0,0,0,0|FS:100,500";
        let t = Instant::now();
        let st = parse_status(line, t).unwrap();
        assert_eq!(st.machine_pos.a, Some(0.0));
        assert_eq!(st.work_pos.a, Some(0.0));
        assert_eq!(st.feed_rate, 100.0);
        assert_eq!(st.spindle_speed, 500.0);
    }

    #[test]
    fn test_parse_status_run() {
        let line = "Run|MPos:10.5,20,0|WPos:10.5,20,0|FS:200,1000";
        let t = Instant::now();
        let st = parse_status(line, t).unwrap();
        assert!(matches!(st.state, MachineState::Run));
        assert_eq!(st.machine_pos.x, 10.5);
        assert_eq!(st.machine_pos.y, 20.0);
    }

    #[test]
    fn test_parse_status_hold() {
        let line = "Hold|MPos:0,0,0|WPos:0,0,0|FS:0,0";
        let t = Instant::now();
        let st = parse_status(line, t).unwrap();
        assert!(matches!(st.state, MachineState::Hold(_)));
    }

    #[test]
    fn test_parse_status_jog() {
        let line = "Jog|MPos:1,2,3|WPos:1,2,3|FS:500,0";
        let t = Instant::now();
        let st = parse_status(line, t).unwrap();
        assert!(matches!(st.state, MachineState::Jog));
    }

    #[test]
    fn test_parse_status_alarm() {
        let line = "Alarm:1|MPos:0,0,0|WPos:0,0,0|FS:0,0";
        let t = Instant::now();
        let st = parse_status(line, t).unwrap();
        assert!(matches!(st.state, MachineState::Alarm(AlarmCode::HardLimit)));
    }

    #[test]
    fn test_parse_status_home() {
        let line = "Home|MPos:0,0,0|WPos:0,0,0|FS:0,0";
        let t = Instant::now();
        let st = parse_status(line, t).unwrap();
        assert!(matches!(st.state, MachineState::Home));
    }

    #[test]
    fn test_parse_status_sleep() {
        let line = "Sleep|MPos:0,0,0|WPos:0,0,0|FS:0,0";
        let t = Instant::now();
        let st = parse_status(line, t).unwrap();
        assert!(matches!(st.state, MachineState::Sleep));
    }

    #[test]
    fn test_parse_status_unknown_state() {
        let line = "CustomState|MPos:0,0,0|WPos:0,0,0|FS:0,0";
        let t = Instant::now();
        let st = parse_status(line, t).unwrap();
        assert!(matches!(st.state, MachineState::Unknown(ref s) if s == "CustomState"));
    }

    #[test]
    fn test_parse_status_invalid_empty() {
        let t = Instant::now();
        let err = parse_status("", t).unwrap_err();
        assert!(matches!(err, ParseError::InvalidStatus(_)));
    }

    #[test]
    fn test_parse_status_invalid_position() {
        let t = Instant::now();
        let err = parse_status("Idle|MPos:bad,0,0|WPos:0,0,0|FS:0,0", t).unwrap_err();
        assert!(matches!(err, ParseError::InvalidPosition(_)));
    }

    #[test]
    fn test_parse_alarm_code_alarm_prefix() {
        let code = parse_alarm_code("ALARM:1").unwrap();
        assert_eq!(code, AlarmCode::HardLimit);
    }

    #[test]
    fn test_parse_alarm_code_alarm_space() {
        let code = parse_alarm_code("ALARM: 2").unwrap();
        assert_eq!(code, AlarmCode::SoftLimit);
    }

    #[test]
    fn test_parse_alarm_code_unknown_number() {
        let code = parse_alarm_code("ALARM:99").unwrap();
        assert!(matches!(code, AlarmCode::Unknown(99)));
    }

    #[test]
    fn test_parse_alarm_code_invalid() {
        let err = parse_alarm_code("not a number").unwrap_err();
        assert!(matches!(err, ParseError::InvalidAlarm(_)));
    }

    #[test]
    fn test_parse_settings() {
        let lines = "$0=10\n$1=25\n$21=0\nok\n";
        let settings = parse_settings(lines).unwrap();
        assert_eq!(settings.raw.get(&0), Some(&"10".to_string()));
        assert_eq!(settings.raw.get(&1), Some(&"25".to_string()));
        assert_eq!(settings.raw.get(&21), Some(&"0".to_string()));
        assert!(!settings.raw.contains_key(&99));
    }

    #[test]
    fn test_parse_settings_empty_and_ok() {
        let lines = "\nok\n";
        let settings = parse_settings(lines).unwrap();
        assert!(settings.raw.is_empty());
    }

    #[test]
    fn test_parse_settings_grbl_hal_high_number() {
        let lines = "$340=0\nok";
        let settings = parse_settings(lines).unwrap();
        assert_eq!(settings.raw.get(&340), Some(&"0".to_string()));
    }
}
