//! Minimal binary: parses a hard-coded GRBL-HAL status string (no serial port).
//! Confirms the parser and state types are wired correctly.

use grbl_rs::machines::grbl::{parse_status, MachineState};
use std::time::Instant;

fn main() {
    let line = "<Idle|MPos:0.000,0.000,0.000|WPos:0.000,0.000,0.000|FS:0,0>";
    match parse_status(line, Instant::now()) {
        Ok(status) => {
            println!("State: {:?}", status.state);
            println!("MPos: {:?}", status.machine_pos);
            println!("WPos: {:?}", status.work_pos);
            assert!(matches!(status.state, MachineState::Idle));
        }
        Err(e) => println!("Parse error: {}", e),
    }
}
