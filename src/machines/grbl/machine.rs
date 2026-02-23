//! Public API: single interface to the GRBL-HAL controller.
//!
//! `GrblMachine` owns the connection, runs the status poller, and exposes
//! connect, disconnect, jog, home, run_file, get_status, and probe_z.
//! Everything else (port, poller, streamer, parser, motion) is internal.

#![cfg(feature = "serial")]

use super::commands::{GrblCommand, RealtimeCommand};
use super::motion::{translate_lines, MotionConfig};
use super::port::{Port, PortError, DEFAULT_BAUD};
use super::poller::{run_poller, PollerHandle, STATUS_READ_TIMEOUT_MS};
use super::state::MachineStatus;
use super::streamer::{stream_lines, StreamResult, LINE_RESPONSE_TIMEOUT_MS};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{broadcast, Mutex};
use tokio::task::JoinHandle;
use thiserror::Error;
use tracing::info;

/// Errors from the public GrblMachine API.
#[derive(Debug, Error)]
pub enum GrblError {
    #[error("port: {0}")]
    Port(#[from] PortError),
    #[error("streamer: {0}")]
    Streamer(#[from] super::streamer::StreamerError),
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),
}

/// Single public interface to a GRBL-HAL controller.
///
/// Connect with [`GrblMachine::connect`], then use jog, home, run_file,
/// get_status, probe_z. Call [`GrblMachine::disconnect`] when done (or drop).
pub struct GrblMachine {
    port: Arc<Mutex<Port>>,
    state: Arc<Mutex<MachineStatus>>,
    _broadcast_tx: broadcast::Sender<MachineStatus>,
    poller_handle: JoinHandle<Result<(), super::poller::PollerError>>,
    motion_config: Arc<Mutex<MotionConfig>>,
}

impl GrblMachine {
    /// Connect to the controller at the given port (e.g. `"COM3"` or `"/dev/ttyUSB0"`).
    /// Starts the status poller in the background. Uses 115200 baud.
    pub async fn connect(port_name: &str) -> Result<Self, GrblError> {
        let port = Port::open(port_name, DEFAULT_BAUD)?;
        let port = Arc::new(Mutex::new(port));
        let state = Arc::new(Mutex::new(MachineStatus::idle()));
        let (tx, _rx) = broadcast::channel(16);

        let handle = PollerHandle {
            port: Arc::clone(&port),
            state: Arc::clone(&state),
            tx: tx.clone(),
        };
        let poller_handle = tokio::spawn(async move {
            run_poller(
                handle,
                Duration::from_millis(200),
                Duration::from_millis(STATUS_READ_TIMEOUT_MS),
            )
            .await
        });

        info!("GrblMachine connected to {}", port_name);
        Ok(GrblMachine {
            port,
            state,
            _broadcast_tx: tx,
            poller_handle,
            motion_config: Arc::new(Mutex::new(MotionConfig::default())),
        })
    }

    /// Disconnect: stop the poller and close the port.
    pub async fn disconnect(self) {
        self.poller_handle.abort();
        let _ = self.poller_handle.await;
        info!("GrblMachine disconnected");
    }

    /// Jog: send `$J=<gcode>`. Example: `"G21G91X10F500"` for relative 10 mm on X at 500 mm/min.
    pub async fn jog(&self, gcode: &str) -> Result<(), GrblError> {
        let cmd = GrblCommand::Jog(gcode.to_string()).to_string();
        let port = Arc::clone(&self.port);
        let line = cmd;
        tokio::task::spawn_blocking(move || {
            let mut port = port.blocking_lock();
            port.send_line(&line)
        })
        .await
        .map_err(|e| GrblError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))??;
        Ok(())
    }

    /// Home: send `$H` (run homing cycle).
    pub async fn home(&self) -> Result<(), GrblError> {
        let line = GrblCommand::Home.to_string();
        let port = Arc::clone(&self.port);
        tokio::task::spawn_blocking(move || {
            let mut port = port.blocking_lock();
            port.send_line(&line)
        })
        .await
        .map_err(|e| GrblError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))??;
        Ok(())
    }

    /// Run a g-code file: translate Y moves (bed extension) then stream with flow control.
    /// Pauses on Hold, resumes on Idle. Returns stream result (lines sent, first error if any).
    pub async fn run_file(&self, path: &Path) -> Result<StreamResult, GrblError> {
        let content = tokio::fs::read_to_string(path).await?;
        let lines: Vec<&str> = content.lines().collect();
        let config = self.motion_config.lock().await.clone();
        let translated = translate_lines(&lines, &config);
        let port = Arc::clone(&self.port);
        let state = Arc::clone(&self.state);
        let timeout = Duration::from_millis(LINE_RESPONSE_TIMEOUT_MS);
        let result =
            stream_lines(port, state, translated.into_iter(), timeout).await?;
        Ok(result)
    }

    /// Current machine status (from the poller). Clone of the shared state.
    pub async fn get_status(&self) -> MachineStatus {
        self.state.lock().await.clone()
    }

    /// Probe Z: send G38.2 Z toward negative (e.g. `G38.2 Z-10 F50`). Fails if probe not triggered.
    pub async fn probe_z(&self, distance_mm: f64, feed_mm_min: f64) -> Result<(), GrblError> {
        let line = format!("G38.2 Z-{:.4} F{:.4}", distance_mm, feed_mm_min);
        let port = Arc::clone(&self.port);
        let line_response_timeout = Duration::from_millis(LINE_RESPONSE_TIMEOUT_MS);
        tokio::task::spawn_blocking(move || {
            let mut port = port.blocking_lock();
            port.send_line(&line)?;
            let _ = port.read_line(line_response_timeout)?;
            Ok::<_, PortError>(())
        })
        .await
        .map_err(|e| GrblError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))??;
        Ok(())
    }

    /// Unlock after alarm (send `$X`).
    pub async fn unlock(&self) -> Result<(), GrblError> {
        let line = GrblCommand::Unlock.to_string();
        let port = Arc::clone(&self.port);
        tokio::task::spawn_blocking(move || {
            let mut port = port.blocking_lock();
            port.send_line(&line)
        })
        .await
        .map_err(|e| GrblError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))??;
        Ok(())
    }

    /// Send a real-time command (single byte, no newline): e.g. jog cancel, feed override.
    pub async fn send_realtime(&self, cmd: RealtimeCommand) -> Result<(), GrblError> {
        let byte = cmd.as_byte();
        let port = Arc::clone(&self.port);
        tokio::task::spawn_blocking(move || {
            let mut port = port.blocking_lock();
            port.send_byte(byte)
        })
        .await
        .map_err(|e| GrblError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))??;
        Ok(())
    }

    /// Set the motion config (gantry Y limit and bed axis) used by `run_file`.
    pub async fn set_motion_config(&self, config: MotionConfig) {
        *self.motion_config.lock().await = config;
    }
}

impl Drop for GrblMachine {
    fn drop(&mut self) {
        self.poller_handle.abort();
    }
}

/// List available serial ports (for connection UI). Requires `serial` feature.
pub fn list_ports() -> Result<Vec<super::port::PortInfo>, GrblError> {
    Ok(super::port::list_ports()?)
}
