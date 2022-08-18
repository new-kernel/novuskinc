pub mod macros;
pub mod types;

extern "C" {
    /// ``arch_prepare_init`` uses the architecture specific kernel to get ready for the main kernel
    /// initialization
    pub fn arch_prepare_init();

    /// This function is used to setup the architecture specific kernel
    pub fn setup_arch();

    /// Starts the architecture and main kernel, used for both the main Novusk kernel and a device
    /// specific kernels
    pub fn start_kernel();

    /// ``kernel_init`` is the kernel's "main" function, it initializes all non architecture
    /// specific functions
    pub fn kernel_init();
}
