use core::fmt::Write;
use crate::console::KernelConsoleDriver;
use crate::drivers::{Driver, DriverResult};
use crate::drivers::names::NONE;
use crate::fb::FrameBufferGraphics;
use crate::keyboard::KeyboardInput;
use crate::led::Led;
use crate::prelude::{Serial, Storage};

pub struct EmptyDriver;

impl KernelConsoleDriver for EmptyDriver {}

impl FrameBufferGraphics for EmptyDriver {}

impl KeyboardInput for EmptyDriver {}

impl Storage for EmptyDriver {}

impl Write for EmptyDriver {
    fn write_str(&mut self, s: &str) -> core::fmt::Result { todo!() }
}

impl Serial for EmptyDriver {}

impl Led for EmptyDriver {}

impl Driver for EmptyDriver {
    fn driver_name(&self) -> &'static str {
        "Empty driver"
    }

    fn name(&self) -> &'static str {
        NONE
    }

    fn init(&self) -> DriverResult {
        Err("Unimplemented")
    }
}
