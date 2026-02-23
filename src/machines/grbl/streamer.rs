//! G-code streaming task for GRBL-HAL.
//!
//! Reads a g-code file line by line, sends each line to the controller with
//! buffer-based flow control (wait for `ok` or `error` before sending the next),
//! tracks ok/error responses, and pauses while the machine is in Hold, resuming when Idle.
//!
//! # Example
//!
//! ```ignore
//! use grbl_rs::machines::grbl::{stream_file, MachineStatus, Port};
//! use std::path::Path;
//! use std::sync::Arc;
//! use std::time::Duration;
//! use tokio::sync::Mutex;
//!
//! let port = Arc::new(Mutex::new(Port::open("COM1", 115_200)?));
//! let state = Arc::new(Mutex::new(MachineStatus::idle()));
//! let result = stream_file(
//!     port,
//!     state,
//!     Path::new("job.nc"),
//!     Duration::from_millis(30_000),
//! ).await?;
//! ```

#![cfg(feature = "serial")]

use super::port::{Port, PortError};
use super::state::{MachineState, MachineStatus};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

/// Default timeout when waiting for `ok`/`error` after sending a line (30 s).
pub const LINE_RESPONSE_TIMEOUT_MS: u64 = 30_000;

/// Outcome of streaming a single line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LineResult {
    Ok,
    Error(String),
}

/// Result of a streaming run.
#[derive(Clone, Debug, Default)]
pub struct StreamResult {
    /// Total lines sent to the controller.
    pub lines_sent: u32,
    /// Lines that returned `ok`.
    pub lines_ok: u32,
    /// First error response, if any (message only).
    pub first_error: Option<String>,
}

/// Errors from the streamer.
#[derive(Debug, thiserror::Error)]
pub enum StreamerError {
    #[error("port I/O: {0}")]
    Port(#[from] PortError),
    #[error("read file: {0}")]
    ReadFile(#[from] std::io::Error),
    #[error("task join: {0}")]
    Join(#[from] tokio::task::JoinError),
}

/// Returns true if the line should be sent (non-empty, not a comment).
/// GRBL accepts lines starting with `;` as comments (we skip them).
fn is_sendable_line(line: &str) -> bool {
    let trimmed = line.trim();
    !trimmed.is_empty() && !trimmed.starts_with(';')
}

/// Stream a g-code file: read line by line, send with flow control, pause on Hold.
///
/// Uses the shared port and state. For each sendable line: waits until state is
/// not Hold, sends the line, waits for `ok` or `error:...`, then continues.
/// Stops on first error response or when the file is done.
pub async fn stream_file(
    port: Arc<Mutex<Port>>,
    state: Arc<Mutex<MachineStatus>>,
    path: &Path,
    line_response_timeout: Duration,
) -> Result<StreamResult, StreamerError> {
    let content = tokio::fs::read_to_string(path).await?;
    let lines: Vec<&str> = content.lines().collect();
    stream_lines(port, state, lines.into_iter(), line_response_timeout).await
}

/// Stream an iterator of g-code lines with the same flow control as `stream_file`.
pub async fn stream_lines<I, S>(
    port: Arc<Mutex<Port>>,
    state: Arc<Mutex<MachineStatus>>,
    lines: I,
    line_response_timeout: Duration,
) -> Result<StreamResult, StreamerError>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut result = StreamResult::default();
    for line in lines {
        let line = line.as_ref().trim();
        if !is_sendable_line(line) {
            continue;
        }

        // Pause while machine is in Hold; resume when Idle (or Run).
        loop {
            let current = state.lock().await.clone();
            match &current.state {
                MachineState::Hold(_) | MachineState::Door => {
                    debug!("streamer: paused (Hold/Door), waiting...");
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
                _ => break,
            }
        }

        let line = line.to_string();
        let port_clone = Arc::clone(&port);
        let timeout = line_response_timeout;
        let response = tokio::task::spawn_blocking(move || {
            let mut port = port_clone.blocking_lock();
            port.send_line(&line)?;
            let response = port.read_line(timeout)?;
            Ok::<_, PortError>(response)
        })
        .await
        .map_err(StreamerError::Join)?
        .map_err(StreamerError::Port)?;

        let response = response.trim();
        result.lines_sent += 1;

        if response.eq_ignore_ascii_case("ok") {
            result.lines_ok += 1;
        } else if response.starts_with("error:") || response.starts_with("Error:") {
            let msg = response
                .strip_prefix("error:")
                .or_else(|| response.strip_prefix("Error:"))
                .map(str::trim)
                .unwrap_or(response)
                .to_string();
            if result.first_error.is_none() {
                result.first_error = Some(msg.clone());
            }
            warn!("streamer: error response: {}", msg);
            break;
        } else {
            // Unexpected response (e.g. alarm message); treat as error and stop.
            if result.first_error.is_none() {
                result.first_error = Some(response.to_string());
            }
            warn!("streamer: unexpected response: {}", response);
            break;
        }
    }

    info!(
        "streamer: done, sent={} ok={}",
        result.lines_sent, result.lines_ok
    );
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_sendable_line() {
        assert!(!is_sendable_line(""));
        assert!(!is_sendable_line("   "));
        assert!(!is_sendable_line("; comment"));
        assert!(!is_sendable_line("  ; comment"));
        assert!(is_sendable_line("G0 X10"));
        assert!(is_sendable_line("  G0 X10  "));
    }

    #[test]
    fn test_line_result_ok() {
        assert_eq!(LineResult::Ok, LineResult::Ok);
    }

    #[test]
    fn test_stream_result_default() {
        let r = StreamResult::default();
        assert_eq!(r.lines_sent, 0);
        assert_eq!(r.lines_ok, 0);
        assert!(r.first_error.is_none());
    }
}
