#![allow(warnings)]

pub mod packets;
pub mod decryption;
pub mod types;
pub mod start_capture;
mod macros;

pub use start_capture::*;
pub use decryption::*;

#[cfg(all(feature = "serde", feature = "bincode"))]
compile_error!(
    "You cannot enable both `serde` and `bincode` features at the same time. \
     Pick exactly one: `cargo build --features serde` or `cargo build --features bincode`"
);