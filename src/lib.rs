#![no_std]

#[macro_use] extern crate alloc;

pub mod console;
pub mod elf;
pub mod fb;
pub mod fs;
pub mod firmware;
pub mod drivers;
pub mod irq;
pub mod keyboard;
pub mod kernel;
pub mod led;
pub mod mb;
pub mod module;
pub mod net;
pub mod platform;
pub mod power;
pub mod prelude;
pub mod serial;
pub mod storage;
pub mod timer;

#[cfg(feature = "v3")]
pub mod v3;

#[cfg(feature = "v3")]
pub use v3::*;

#[cfg(feature = "v4")]
pub mod v4;

#[cfg(feature = "v4")]
pub use v4::*;
