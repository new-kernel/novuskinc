/// Reboots the device when used with ``set_power_mode``.
pub const PM_REBOOT: u32 = 0;
/// Shutsdown the device when used with ``set_power_mode``.
pub const PM_SHUTDOWN: u32 = 1;

pub const PM_SLEEP: u32 = 2;

extern "C" {
    pub fn reboot() -> !;
    pub fn shutdown() -> !;
    pub fn set_power_mode(mode: u32) -> u32;
}
