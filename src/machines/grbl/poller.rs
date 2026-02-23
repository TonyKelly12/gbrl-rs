//! Status polling task for GRBL-HAL.
//!
//! Async task that sends `?` every 200 ms, parses the response with the parser,
//! updates shared `Arc<Mutex<MachineStatus>>`, and broadcasts the new status.
//! Port I/O runs in `spawn_blocking` so the async runtime is not blocked.
//!
//! # Example
//!
//! ```ignore
//! use grbl_rs::machines::grbl::{Port, run_poller, PollerHandle, MachineStatus};
//! use std::sync::Arc;
//! use std::time::Duration;
//! use tokio::sync::{broadcast, Mutex};
//!
//! let port = Port::open("COM1", 115_200).unwrap();
//! let (tx, _rx) = broadcast::channel(16);
//! let handle = PollerHandle {
//!     port: Arc::new(Mutex::new(port)),
//!     state: Arc::new(Mutex::new(MachineStatus::idle())),
//!     tx,
//! };
//! tokio::spawn(async move {
//!     let _ = run_poller(
//!         handle,
//!         Duration::from_millis(200),
//!         Duration::from_millis(500),
//!     ).await;
//! });
//! ```

#![cfg(feature = "serial")]

use super::parser::parse_status;
use super::port::{Port, PortError};
use super::state::MachineStatus;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{broadcast, Mutex};
use tracing::{debug, warn};

/// Default poll interval (brief: 200 ms).
pub const POLL_INTERVAL_MS: u64 = 200;

/// Default read timeout when waiting for status line (500 ms).
pub const STATUS_READ_TIMEOUT_MS: u64 = 500;

/// Shared port and state for the poller. The port is locked only during send/read.
pub struct PollerHandle {
    /// Serial port (shared with future command sender).
    pub port: Arc<Mutex<Port>>,
    /// Current machine status; updated every poll.
    pub state: Arc<Mutex<MachineStatus>>,
    /// Broadcast sender for status updates (e.g. UI, session logger).
    pub tx: broadcast::Sender<MachineStatus>,
}

/// Runs the poll loop. Sends `?` every `interval`, parses response, updates `state`, broadcasts.
/// Returns when the broadcast receiver is dropped (no more subscribers) or on a fatal error.
pub async fn run_poller(
    handle: PollerHandle,
    interval: Duration,
    read_timeout: Duration,
) -> Result<(), PollerError> {
    let mut ticker = tokio::time::interval(interval);
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    loop {
        ticker.tick().await;

        // Blocking port I/O in a separate thread so we don't block the async runtime.
        let port = Arc::clone(&handle.port);
        let timeout = read_timeout;
        let line = tokio::task::spawn_blocking(move || {
            let mut port = port.blocking_lock();
            port.send_line("?")?;
            port.read_line(timeout)
        })
        .await
        .map_err(|e| PollerError::Join(e))?
        .map_err(PollerError::Port)?;

        let now = Instant::now();
        match parse_status(line.trim(), now) {
            Ok(status) => {
                {
                    let mut state = handle.state.lock().await;
                    *state = status.clone();
                }
                if handle.tx.send(status).is_err() {
                    debug!("poller: no broadcast receivers, stopping");
                    return Ok(());
                }
            }
            Err(e) => {
                warn!("poller: parse error: {}", e);
                // Continue polling; next tick may succeed.
            }
        }
    }
}

/// Errors from the poller loop.
#[derive(Debug, thiserror::Error)]
pub enum PollerError {
    #[error("port I/O: {0}")]
    Port(#[from] PortError),
    #[error("task join: {0}")]
    Join(#[from] tokio::task::JoinError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poll_interval_constant() {
        assert_eq!(POLL_INTERVAL_MS, 200);
    }

    #[test]
    fn test_status_read_timeout_constant() {
        assert_eq!(STATUS_READ_TIMEOUT_MS, 500);
    }
}
