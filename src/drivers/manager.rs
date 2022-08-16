use alloc::vec::Vec;
use crate::drivers::empty::EmptyDriver;
use crate::drivers::names::NONE;
use super::Driver;

extern "C" {
    /// This is a static that contains all the drivers that Novusk will use, it is defined in
    /// ``drivers/drivers.rs`` which should be included in the architecture specific kernel.
    pub static mut DRIVER_MANAGER: DeviceDriverManager;
}

/// ``DeviceDriverManager`` is used to manage main device drivers. The drivers are stored in a
/// ``Vec`` which requires an allocator, if a kernel doesn't have an allocator or a proper working
/// one then this struct can't be used.
///
/// Most architectures Novusk supports have an allocator but for the ones that don't have their own
/// driver manager.
pub struct DeviceDriverManager {
    pub drivers: Vec<&'static mut dyn Driver>,
}

impl DeviceDriverManager {
    pub fn new() -> Self {
        return DeviceDriverManager {
            drivers: vec![],
        }
    }

    /// ``add_driver`` is used to add a device driver to the manager, the "driver" argument can be
    /// any struct that implements ``Driver``
    pub fn add_driver(&mut self, driver: &'static mut dyn Driver) {
        self.drivers.push(driver);
    }

    /// The ``get_driver`` function returns a device driver. The "name" argument is what
    /// ``driver_name`` returns from ``Driver``
    pub fn get_driver(&'static mut self, name: &'static str) -> Option<&'static mut dyn Driver> {
        for d in 0..self.drivers.len() {
            if name == self.drivers[d].name() {
                return Some(self.drivers[d]);
            }
        }

        return None;
    }
}
