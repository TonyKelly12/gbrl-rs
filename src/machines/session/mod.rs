//! Black box recorder: logs every probe result and position for diagnosing
//! multi-hour run defects. One JSONL file per session; app (or a subscriber task)
//! feeds probe results and optional status snapshots.
//!
//! # Example (with `serial` feature)
//!
//! **Probe path:** App calls `probe_z()`, then `get_status()`, then
//! `session.record_probe(probe_result(success, work_pos, machine_pos))`.
//!
//! **Status path:** Use `machine.subscribe_status()` to get a `broadcast::Receiver<MachineStatus>`.
//! Spawn a task that receives from it and calls `recorder.record_status(status_snapshot_from(&status))`
//! at a chosen interval (e.g. every 1–5 s) or on state change.
//!
//! ```ignore
//! let mut recorder = SessionRecorder::start_session(log_dir)?;
//! let mut rx = machine.subscribe_status();
//! // ... in another task: while let Ok(status) = rx.recv().await { record_status(...); }
//! machine.probe_z(10.0, 50.0).await?;
//! let status = machine.get_status().await;
//! recorder.record_probe(probe_result(true, status.work_pos, status.machine_pos))?;
//! recorder.finish()?;
//! ```

use crate::machines::grbl::Position;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

/// One probe cycle result: success/fail and position at probe time.
#[derive(Clone, Debug, Serialize)]
pub struct ProbeResult {
    pub success: bool,
    pub work_pos: Position,
    pub machine_pos: Position,
    /// Unix timestamp (seconds since epoch, fractional).
    pub ts_secs: f64,
}

/// Snapshot of machine status for position-over-time logging (e.g. throttled).
#[derive(Clone, Debug, Serialize)]
pub struct StatusSnapshot {
    pub state: String,
    pub work_pos: Position,
    pub machine_pos: Position,
    pub feed_rate: f64,
    pub spindle_speed: f64,
    /// Unix timestamp (seconds since epoch, fractional).
    pub ts_secs: f64,
}

/// JSONL line variant: one of these per line in the log.
#[derive(Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum SessionEvent {
    Probe {
        success: bool,
        work_pos: Position,
        machine_pos: Position,
        ts_secs: f64,
    },
    Status {
        state: String,
        work_pos: Position,
        machine_pos: Position,
        feed_rate: f64,
        spindle_speed: f64,
        ts_secs: f64,
    },
}

/// Errors from session recorder.
#[derive(Debug, Error)]
pub enum SessionError {
    #[error("failed to open session file {path}: {source}")]
    OpenFailed {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to write: {0}")]
    WriteFailed(#[from] std::io::Error),
}

fn now_secs() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

/// Session recorder: append-only JSONL log of probe results and status snapshots.
pub struct SessionRecorder {
    writer: BufWriter<File>,
    path: PathBuf,
}

impl SessionRecorder {
    /// Start a new session: create a timestamped log file in `log_dir`.
    /// File name: `session_YYYY-MM-DDTHH-MM-SS.jsonl`.
    pub fn start_session(log_dir: &Path) -> Result<Self, SessionError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        let secs = now.as_secs();
        let (mins, s) = (secs / 60, secs % 60);
        let (hours, m) = (mins / 60, mins % 60);
        let (days, h) = (hours / 24, hours % 24);
        let (y, m_d) = days_to_year_month_day(days as u32);
        let name = format!(
            "session_{:04}-{:02}-{:02}T{:02}-{:02}-{:02}.jsonl",
            y,
            m_d.0,
            m_d.1,
            h,
            m,
            s
        );
        let path = log_dir.join(name);
        let file = File::options()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| SessionError::OpenFailed {
                path: path.clone(),
                source: e,
            })?;
        let writer = BufWriter::new(file);
        Ok(SessionRecorder { writer, path })
    }

    /// Record one probe result (append one JSON line).
    pub fn record_probe(&mut self, result: ProbeResult) -> Result<(), SessionError> {
        let event = SessionEvent::Probe {
            success: result.success,
            work_pos: result.work_pos,
            machine_pos: result.machine_pos,
            ts_secs: result.ts_secs,
        };
        let line = serde_json::to_string(&event).map_err(|e| {
            SessionError::WriteFailed(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
        })?;
        writeln!(self.writer, "{}", line)?;
        Ok(())
    }

    /// Record one status snapshot (append one JSON line).
    pub fn record_status(&mut self, snapshot: StatusSnapshot) -> Result<(), SessionError> {
        let event = SessionEvent::Status {
            state: snapshot.state,
            work_pos: snapshot.work_pos,
            machine_pos: snapshot.machine_pos,
            feed_rate: snapshot.feed_rate,
            spindle_speed: snapshot.spindle_speed,
            ts_secs: snapshot.ts_secs,
        };
        let line = serde_json::to_string(&event).map_err(|e| {
            SessionError::WriteFailed(std::io::Error::new(std::io::ErrorKind::InvalidData, e))
        })?;
        writeln!(self.writer, "{}", line)?;
        Ok(())
    }

    /// Flush and close the log file.
    pub fn finish(mut self) -> Result<(), SessionError> {
        self.writer.flush()?;
        Ok(())
    }

    /// Path to the current session log file.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// Convert days since Unix epoch to (year, (month, day)); simplified.
fn days_to_year_month_day(days: u32) -> (u32, (u32, u32)) {
    // 1970-01-01 = day 0
    let d = days as i64;
    let (y, m, day) = unix_days_to_ymd(d);
    (y as u32, (m as u32, day as u32))
}

fn unix_days_to_ymd(days: i64) -> (i32, i32, i32) {
    const EPOCH: i64 = 719163; // days from year 0 to 1970-01-01 (proleptic Gregorian)
    let d = days + EPOCH;
    let (y, m, d) = date_from_ordinal(d);
    (y, m, d)
}

fn date_from_ordinal(n: i64) -> (i32, i32, i32) {
    let n = n as i32;
    let a = n + 68569;
    let b = (4 * a) / 146097;
    let c = a - (146097 * b + 3) / 4;
    let d = (4000 * (c + 1)) / 1461001;
    let e = c - (1461 * d) / 4 + 31;
    let m = (80 * e) / 2447;
    let day = e - (2447 * m) / 80;
    let mut y = 100 * (b - 49) + d + e / 2447;
    let month = m - (m / 14);
    if month <= 0 {
        y -= 1;
    }
    let month = if month <= 0 { month + 12 } else { month };
    (y, month, day)
}

/// Build a ProbeResult from success and positions (e.g. after probe_z + get_status).
pub fn probe_result(success: bool, work_pos: Position, machine_pos: Position) -> ProbeResult {
    ProbeResult {
        success,
        work_pos,
        machine_pos,
        ts_secs: now_secs(),
    }
}

/// Build a StatusSnapshot from MachineStatus (state formatted as string; no Instant).
pub fn status_snapshot_from(status: &crate::machines::grbl::MachineStatus) -> StatusSnapshot {
    StatusSnapshot {
        state: format!("{:?}", status.state),
        work_pos: status.work_pos.clone(),
        machine_pos: status.machine_pos.clone(),
        feed_rate: status.feed_rate,
        spindle_speed: status.spindle_speed,
        ts_secs: now_secs(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
        fn test_start_session_and_record_probe() {
        let dir = std::env::temp_dir().join("grbl_rs_session_test");
        std::fs::create_dir_all(&dir).unwrap();
        let mut rec = SessionRecorder::start_session(&dir).unwrap();
        let path = rec.path().to_path_buf(); // capture path before finish() moves rec
        let pos = Position {
            x: 0.0,
            y: 0.0,
            z: 1.5,
            a: None,
        };
        rec.record_probe(ProbeResult {
            success: true,
            work_pos: pos.clone(),
            machine_pos: pos.clone(),
            ts_secs: 1000.5,
        })
        .unwrap();
        rec.finish().unwrap();
        let mut f = File::open(path).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        assert!(s.contains("\"event\":\"probe\""));
        assert!(s.contains("\"success\":true"));
        assert!(s.contains("\"ts_secs\":1000.5"));
    }

    #[test]
    fn test_record_status() {
        let dir = std::env::temp_dir().join("grbl_rs_session_test2");
        std::fs::create_dir_all(&dir).unwrap();
        let mut rec = SessionRecorder::start_session(&dir).unwrap();
        let pos = Position {
            x: 10.0,
            y: 20.0,
            z: 0.0,
            a: None,
        };
        rec.record_status(StatusSnapshot {
            state: "Idle".to_string(),
            work_pos: pos.clone(),
            machine_pos: pos,
            feed_rate: 0.0,
            spindle_speed: 0.0,
            ts_secs: 2000.0,
        })
        .unwrap();
        rec.finish().unwrap();
    }
}
