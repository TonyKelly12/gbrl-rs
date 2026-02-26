//! Serial port ownership and I/O for GRBL-HAL.
//!
//! Owns the serial connection exclusively. Connect at 115200 baud (8N1),
//! send line-based commands, read line responses with configurable timeout.
//! Used by the poller and command queue task; they send through channels,
//! this module performs the actual read/write.
//!
//! # Example (requires `serial` feature and a connected controller)
//!
//! ```ignore
//! use grbl_rs::machines::grbl::{Port, list_ports, DEFAULT_BAUD};
//! use std::time::Duration;
//!
//! let ports = list_ports().unwrap();
//! let name = ports.first().map(|p| p.name.as_str()).unwrap_or("COM1");
//! let mut port = Port::open(name, DEFAULT_BAUD).unwrap();
//! port.send_line("?").unwrap();
//! let line = port.read_line(Duration::from_millis(500)).unwrap();
//! ```
//! 

/// ToDO Check if connection is open or close
/// ToDo and and remove connections
/// TODO Close Connection
/// TODO Set Write Filter
/// TODO Check for timeout
/// TODO update options
/// TODO Destroy Connection
/// TODO Add connection monitoring and logging
/// 

#![cfg(feature = "serial")]

use std::io::{Read, Write};
use std::time::Duration;
use thiserror::Error;

/// Default baud rate for GRBL-HAL (brief: 115200).
pub const DEFAULT_BAUD: u32 = 115_200;

/// Errors from port open, list, send, or read.
#[derive(Debug, Error)]
pub enum PortError {
    #[error("failed to open port {port}: {source}")]
    OpenFailed {
        port: String,
        #[source]
        source: serialport::Error,
    },
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("read timeout after {0:?}")]
    Timeout(Duration),
    #[error("serial port error: {0}")]
    Serial(#[from] serialport::Error),
}

/// Information about an available serial port (name and title for display).
#[derive(Clone, Debug)]
pub struct PortInfo {
    pub name: String,
    pub title: String,
}

/// Open serial connection to GRBL. Owns the port; send and read with timeout.
pub struct Port {
    inner: Box<dyn serialport::SerialPort>,
}

impl Port {
    /// Open the named port at the given baud rate (typically 115200 for GRBL).
    /// 8 data bits, 1 stop bit, no parity (8N1).
    pub fn open(port_name: &str, baud: u32) -> Result<Self, PortError> {
        let inner = serialport::new(port_name, baud)
            .data_bits(serialport::DataBits::Eight)
            .stop_bits(serialport::StopBits::One)
            .parity(serialport::Parity::None)
            .timeout(Duration::from_millis(100))
            .open()
            .map_err(|e| PortError::OpenFailed {
                port: port_name.to_string(),
                source: e,
            })?;
        Ok(Port { inner })
    }

    /// Send a line to the controller. Appends `\r\n` (GRBL expects CRLF).
    /// The line should not contain newlines (use the command string only).
    pub fn send_line(&mut self, line: &str) -> Result<(), PortError> {
        self.inner.write_all(line.as_bytes())?;
        self.inner.write_all(b"\r\n")?;
        self.inner.flush()?;
        Ok(())
    }

    /// Send a single real-time byte (no newline). Use for `RealtimeCommand::as_byte()`.
    pub fn send_byte(&mut self, byte: u8) -> Result<(), PortError> {
        self.inner.write_all(&[byte])?;
        self.inner.flush()?;
        Ok(())
    }

    /// Set the read timeout. Used before `read_line` so reads don't block forever.
    /// Pass `None` for a long default timeout (serialport expects `Duration`, not `Option`).
    pub fn set_read_timeout(&mut self, timeout: Option<Duration>) -> Result<(), PortError> {
        self.inner
            .set_timeout(timeout.unwrap_or(Duration::from_secs(86400)))?;
        Ok(())
    }

    /// Read one line (until `\n` or `\r\n`). Strips trailing `\r`/`\n`.
    /// Uses the given timeout for each read; returns `Err(PortError::Timeout(d))` if
    /// no newline is received in time.
    pub fn read_line(&mut self, timeout: Duration) -> Result<String, PortError> {
        self.inner.set_timeout(timeout)?;
        let mut buf = Vec::new();
        let mut single = [0u8; 1];
        loop {
            match self.inner.read(&mut single) {
                Ok(0) => break,
                Ok(1..) => {
                    if single[0] == b'\n' {
                        break;
                    }
                    if single[0] != b'\r' {
                        buf.push(single[0]);
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    return Err(PortError::Timeout(timeout));
                }
                Err(e) => return Err(PortError::Io(e)),
            }
        }
        Ok(String::from_utf8_lossy(&buf).into_owned())
    }
}

/// List available serial ports. Names can be passed to `Port::open`.
pub fn list_ports() -> Result<Vec<PortInfo>, PortError> {
    let ports = serialport::available_ports()?;
    Ok(ports
        .into_iter()
        .map(|p| {
            let name = p.port_name.clone();
            let title = match p.port_type {
                serialport::SerialPortType::UsbPort(usb) => {
                    format!(
                        "{} (USB {}:{})",
                        name,
                        usb.vid,
                        usb.pid
                    )
                }
                serialport::SerialPortType::PciPort => format!("{} (PCI)", name),
                serialport::SerialPortType::BluetoothPort => format!("{} (Bluetooth)", name),
                serialport::SerialPortType::Unknown => name.clone(),
            };
            PortInfo { name, title }
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_baud() {
        assert_eq!(DEFAULT_BAUD, 115_200);
    }

    #[test]
    fn test_list_ports_no_panic() {
        // Just ensure we can call it; may be empty on CI.
        let _ = list_ports();
    }
}
