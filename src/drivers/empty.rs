use crate::console::KernelConsoleDriver;
use crate::drivers::{Driver, DriverResult};
use crate::drivers::names::NONE;
use crate::fb::FrameBufferGraphics;
use crate::keyboard::KeyboardInput;
use crate::prelude::Storage;

pub struct EmptyDriver;

impl KernelConsoleDriver for EmptyDriver {}

impl FrameBufferGraphics for EmptyDriver {}

impl KeyboardInput for EmptyDriver {}

impl Storage for EmptyDriver {}

impl Driver for EmptyDriver {
    fn driver_name(&self) -> &'static str {
        "Empty driver"
    }

    fn name(&self) -> &'static str {
        NONE
    }

    fn init(&mut self) -> DriverResult {
        Err("Unimplemented")
    }
}
