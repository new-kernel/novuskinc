use alloc::vec::Vec;
use core::borrow::BorrowMut;
use core::ops::Deref;
use crate::drivers::manager::DeviceDriverManager;
use crate::kernel::types::KernelType;
use crate::module::KernelModule;

pub mod macros;
pub mod panic;
pub mod syscalls;
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

    pub fn run_kernel_module(module: &'static str);

    static mut KERNEL: spin::Mutex<Kernel>;
}

pub struct Kernel {
    pub version: (u8, u8, u8),
    pub kernel_type: KernelType,
    pub short: bool,

    /// Names of kernel modules that have been linked directly linked to the kernel
    pub module_names: Vec<&'static str>,
}

impl Kernel {
    pub fn new(version: (u8, u8, u8), kernel_type: KernelType, short: bool) -> Self {
        return Kernel {
            version: version,
            kernel_type: kernel_type,
            short: short,
            module_names: vec![],
        };
    }

    pub fn add_linked_module_name(&mut self, name: &'static str) {
        self.module_names.push(name);
    }

    pub fn run_modules(&mut self) {
        for module in &self.module_names {
            unsafe { run_kernel_module(*module); }
        }
    }

    pub fn start_init(&mut self) {

    }

    pub fn version(&self) -> (u8, u8, u8) {
        self.version
    }
}
