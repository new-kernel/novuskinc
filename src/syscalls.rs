use core::slice::from_raw_parts;

extern "C" {
    /// ``syscall`` is used to invoke a system call, this is the easiest way for a user to interact
    /// with the kernel outside of the kernel. System calls are implemented and run differently in
    /// different versions of Novusk.
    pub fn syscall(sys_num: usize, sys_args: &[*const u8]) -> &[*const u8];

    /// This function is defined by the architecture specific kernel, it matches the ``sys_num``
    /// with the system call it represents and calls it with the arguments needed for the system
    /// call.
    pub fn arch_syscall(sys_num: usize, sys_args: &[*const u8]) -> &[*const u8];


    pub fn setup_console(width: u8, height: u8, x_pos: u8, y_pos: u8);

    // User system calls
    pub fn sys_write(option: u8, write: u8, len: u8);
    // Console write
    pub fn sys_cwrite(write: *const u8, len: u8);
    // GPU write
    pub fn sys_gpu_write(index: u8, value: *const u8);

    pub fn sys_read(option: u8, buf: u8) -> *const u8;
    // Keyboard read
    pub fn sys_keyread(buf: u8) -> *const u8;
    // GPU read
    pub fn sys_gpu_read(index: u8) -> *const u8;

    // Add a kernel module
    pub fn sys_add_kernel_module(name: *const u8, contents: &'static [u8]) -> u8;
    // Run a kernel module
    pub fn sys_init_kernel_module(name: *const u8) -> u8;
    // Remove a kernel module
    pub fn sys_remove_kernel_module(name: *const u8) -> u8;
    // Sleep
    pub fn sys_sleep(seconds: u8, cycles: u8);
    // Exit
    pub fn sys_exit(code: u8) -> u8;
    // GPU call
    pub fn sys_gpu_call(arg: *const u8) -> *const u8;

    /* Kernel "system calls", functions that are similar to system calls that are used inside the
       kernel */
    // Add a kernel module

    // GPU write
    pub fn ksys_gpu_write(index: u8, value: *const u8);

    // GPU read
    pub fn ksys_gpu_read(index: u8) -> *const u8;

    // Add a kernel module
    pub fn ksys_add_kernel_module(name: *const u8, contents: &'static [u8]) -> u8;
    // Run a kernel module
    pub fn ksys_init_kernel_module(name: *const u8) -> u8;
    // Remove a kernel module
    pub fn ksys_remove_kernel_module(name: *const u8) -> u8;
    // GPU call
    pub fn ksys_gpu_call(arg: *const u8) -> *const u8;
}
