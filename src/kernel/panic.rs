extern "C" {
    /// This gets called by Rust's panic handler
    pub fn kernel_panic_handler() -> !;

    /// Shows that the kernel panicked, this will print something or make an LED flash
    /// depending on what Novusk is running on.
    pub fn device_indicate_panic();

    /// Checks the [``Dif``](link) file what to do during a panic
    pub fn check_dif_panic();
}