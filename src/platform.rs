extern "C" {
    /// ``device_init`` is usually called in ``setup_arch``, this function sets up some device
    /// specific functions.
    pub fn device_init() -> u8;

    /// ``early_device_init`` is intended for setting device specific drivers to ``DEVICE_DRIVERS``.
    pub fn early_device_init() -> u8;
}

/// Messages for when a device is initialized unsuccessfully.
pub const DEVICE_INIT_ERRORS: &'static [&'static str; 4] = &[
    "Device initialization shouldn't have failed",
    "Failed to initialize driver[s]",
    "Running on an invalid device",
    "Unknown error",
];

pub const DRIVERS_FAILD: u8 = 1;
pub const INVALID_DEVICE: u8 = 2;
pub const UNKNOWN_ERROR: u8 = 3;
