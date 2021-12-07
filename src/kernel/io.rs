use super::syscalls::{syscall, READ, WRITE};

/// safe_sys_write
///
/// Writes with syscall() safely
pub fn safe_sys_write(write: u8) {
    unsafe { syscall(WRITE, write); }
}

/// safe_sys_read
///
/// Reads with syscall safely
pub fn safe_sys_read() -> u8 {
    return unsafe { syscall(READ, 0) };
}
