//! MeshForge Tauri app: desktop shell around the grbl-rs library.
//!
//! Exposes GRBL operations (e.g. list serial ports) as Tauri commands for the frontend.
//! Set `MESHFORGE_MOCK=1` to use mock data so you can run and test without hardware.

use grbl_rs::machines::grbl::{list_ports, PortInfo};
use serde::Serialize;

/// Serial port info for the frontend (name and display title).
#[derive(Clone, Debug, Serialize)]
pub struct PortInfoDto {
    pub name: String,
    pub title: String,
}

fn is_mock_env() -> bool {
    std::env::var("MESHFORGE_MOCK").as_deref() == Ok("1")
}

/// Returns true if mock mode is enabled (MESHFORGE_MOCK=1). UI can show "Demo mode".
#[tauri::command]
fn is_mock_mode() -> bool {
    is_mock_env()
}

/// List available serial ports. When MESHFORGE_MOCK=1, returns fake ports so you can test without hardware.
#[tauri::command]
fn list_serial_ports() -> Result<Vec<PortInfoDto>, String> {
    if is_mock_env() {
        return Ok(mock_ports());
    }
    list_ports().map_err(|e| e.to_string()).map(|ports| {
        ports
            .into_iter()
            .map(|p: PortInfo| PortInfoDto {
                name: p.name,
                title: p.title,
            })
            .collect()
    })
}

fn mock_ports() -> Vec<PortInfoDto> {
    vec![
        PortInfoDto {
            name: "COM3".to_string(),
            title: "Mock CNC (COM3)".to_string(),
        },
        PortInfoDto {
            name: "/dev/ttyUSB0".to_string(),
            title: "Mock CNC (ttyUSB0)".to_string(),
        },
    ]
}

/// Mock machine status for UI testing. Position, state, and feed/spindle; no real hardware.
#[derive(Clone, Debug, Serialize)]
pub struct MockStatusDto {
    pub state: String,
    pub work_pos: MockPositionDto,
    pub machine_pos: MockPositionDto,
    pub feed_rate: f64,
    pub spindle_speed: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct MockPositionDto {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub a: Option<f64>,
}

/// Returns fake machine status (Idle, positions at 0,0,0). Use when MESHFORGE_MOCK=1 to drive the UI.
#[tauri::command]
fn get_mock_status() -> MockStatusDto {
    MockStatusDto {
        state: "Idle".to_string(),
        work_pos: MockPositionDto {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            a: None,
        },
        machine_pos: MockPositionDto {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            a: None,
        },
        feed_rate: 0.0,
        spindle_speed: 0.0,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_serial_ports,
            is_mock_mode,
            get_mock_status,
        ])
        .run(tauri::generate_context!())
        .expect("error running MeshForge app");
}
