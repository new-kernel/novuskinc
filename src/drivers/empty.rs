use core::fmt::Write;
use crate::console::KernelConsoleDriver;
use crate::drivers::{Driver, DriverResult};
use crate::drivers::names::NONE;
use crate::fb::FrameBufferGraphics;
use crate::keyboard::KeyboardInput;
use crate::led::Led;
use crate::prelude::{Serial, Storage};
use crate::timer::Timer;

pub struct EmptyDriver {
    pub v: u32,
}

impl EmptyDriver {

}

impl KernelConsoleDriver for EmptyDriver {}

impl FrameBufferGraphics for EmptyDriver {}

impl KeyboardInput for EmptyDriver {}

impl Storage for EmptyDriver {}

impl Write for EmptyDriver {
    fn write_str(&mut self, s: &str) -> core::fmt::Result { todo!() }
}

impl Serial for EmptyDriver {}

impl Led for EmptyDriver {}

impl Timer for EmptyDriver {
    fn set_value(&mut self, val: u32) {
        self.v = val;
    }

    fn value(&self) -> u32 {
        self.v
    }
}

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
