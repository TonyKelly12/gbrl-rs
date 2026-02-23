//! GRBL-HAL communication module.
//!
//! Phase 1: parser, state types, commands, port, poller, streamer, and motion.
//! Public API (GrblMachine) is added in the final phase.

mod commands;
mod motion;
mod parser;
mod state;

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
pub use port::*;
#[cfg(feature = "serial")]
pub use poller::*;
#[cfg(feature = "serial")]
pub use streamer::*;
