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
    version: (u8, u8, u8),
    kernel_type: KernelType,
    short: bool,

    /// Kernel modules that have been linked directly linked to the kernel
    linked_modules: Vec<&'static mut KernelModule>,
    /// The file paths to kernel modules that are not linked to the kernel
    module_paths: Option<&'static mut [&'static str; 50]>,
    driver_manager: Option<DeviceDriverManager>,
}

impl Kernel {
    pub fn new(version: (u8, u8, u8), kernel_type: KernelType, short: bool) -> Self {
        return Kernel {
            version: version,
            kernel_type: kernel_type,
            short: short,
            linked_modules: vec![],
            module_paths: None,
            driver_manager: None,
        };
    }

    pub fn add_linked_module(&mut self, module: &'static mut KernelModule) -> u8 {
        self.linked_modules.push(module);
        return 1;
    }

    pub fn run_linked_module(&mut self, module: &'static str) {
        for i in 0..self.linked_modules.len() {
            if self.linked_modules[i].name() == module {
                unsafe { self.linked_modules[i].run(); }
            }
        }
    }

    pub fn start_init(&mut self) {
        // self.kernel_type.run(&mut self);
    }

    pub fn version(&self) -> (u8, u8, u8) {
        self.version
    }
}
