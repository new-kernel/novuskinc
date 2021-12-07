pub use super::systbl::*;

extern "C" {
    /// Syscall (system call) function
    ///
    /// Example:
    /// ```rust
    /// use novuskinc::kernel::syscalls::*;
    ///
    /// fn wirte_stuff() {
    ///	    unsafe { syscall(WRITE, b'A'); }
    /// }
    /// ```
    ///
    /// WRITE is a system call number that is from an architecture specific system call table.
    /// Read [TODO](documentation_link) for more information
    ///
    /// The syscall function should come from libunistd.rlib which should be linked to any Novusk
    /// application.
    pub fn syscall(sys_num: i32, sys_arg: u8) -> u8;
}
