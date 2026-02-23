//! GRBL-HAL communication module.
//!
//! **Public API:** [`GrblMachine`] (with `serial` feature) — connect, disconnect,
//! jog, home, run_file, get_status, probe_z. Use [`list_ports`] to discover ports.
//!
//! Types used by the API (state, commands, motion config) are re-exported.

mod commands;
mod motion;
mod parser;
mod state;

#[cfg(feature = "serial")]
mod machine;
#[cfg(feature = "serial")]
mod port;
#[cfg(feature = "serial")]
mod poller;
#[cfg(feature = "serial")]
mod streamer;

pub use commands::*;
pub use motion::*;
pub use parser::*;
pub use state::*;

#[cfg(feature = "serial")]
pub use machine::*;
#[cfg(feature = "serial")]
pub use port::PortInfo;
#[cfg(feature = "serial")]
pub use streamer::StreamResult;
