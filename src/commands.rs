//! Slash command definitions and handlers.

#![allow(missing_docs)] // NOTE: Because poise does not apply command doc comments to the command function itself

pub mod registration;
pub use registration::registration;

pub mod count;
pub use count::count;

pub mod nickname;
pub use nickname::nickname;
