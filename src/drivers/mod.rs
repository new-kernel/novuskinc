use crate::prelude::*;

pub mod empty;
pub mod manager;
pub mod names;

extern "C" {
    /// ``get_driver`` gets a driver named ``name``. It is usually used for architectures that don't
    /// have a good or proper allocator.
    pub fn get_driver(name: &'static str) -> Option<&'static mut dyn Driver>;

    /// Adds a driver to the architecture's device driver struct.
    pub fn add_driver(driver: &'static mut dyn Driver);

    /// Removes a driver
    pub fn remove_driver(name: &'static str);
}

pub type DriverResult = Result<(), &'static str>;

pub trait Driver: KernelConsoleDriver + FrameBufferGraphics + KeyboardInput + Storage + Serial + Led + Timer {
    /// The ``driver_name`` function should be used to return the driver's name
    ///
    /// Example:
    /// ```rust
    ///     fn driver_name(&self) -> &'static str {
    ///         return "ANCD (A New Console Driver)";
    ///     }
    ///
    fn driver_name(&self) -> &'static str { return "None" }

    /// The ``name`` should return the "type" of driver it is so the kernel can easily index it
    /// in the driver manager
    ///
    /// Example:
    /// ```rust
    ///     fn name(&self) -> &'static str {
    ///         return "Console Driver" // This is for a console driver
    ///     }
    /// ```
    ///
    /// [This page](link) has a list of driver names and what they're used for.
    fn name(&self) -> &'static str { return "None"; }

    /// The init function needs to be used to initialize the driver, it is what the kernel calls to
    /// start it.
    fn init(&mut self) -> DriverResult {
        return Err("Unimplemented");
    }
}
