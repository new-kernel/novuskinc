use core::fmt::{Debug, Formatter};
use core::mem::size_of;
use core::ptr::read;
use core::slice::from_raw_parts;
pub use novuskinc_macros::{module_end, module_init};

#[allow(non_snake_case)]
pub unsafe fn KernelModule_to_bytes(km: &KernelModule) -> &[u8] {
    return from_raw_parts(km as *const KernelModule as *const u8, size_of::<KernelModule>());
}

#[allow(non_snake_case)]
pub unsafe fn bytes_to_KernelModule(bytes: &[u8]) -> KernelModule {
    return read(bytes.as_ptr() as *const _);
}

/// A struct for kernel modules that are linked to Novusk
pub struct KernelModule {
    name: &'static str,
    pub initialized: bool,
    pub success: bool,
    _init: Option<unsafe extern "C" fn(&mut KernelModule)>,
    _end: Option<unsafe extern "C" fn(&mut KernelModule)>,
}

impl KernelModule {
    pub fn new(name: &'static str, _init: Option<unsafe extern "C" fn(&mut KernelModule)>, _end: Option<unsafe extern "C" fn(&mut KernelModule)>) -> Self {
        return KernelModule {
            name: name,
            initialized: false,
            success: false,
            _init: _init,
            _end: _end,
        };
    }

    pub unsafe fn run(&mut self) {
        if self._init.is_some() {
            (self._init.unwrap())(self);
        }

        if self._end.is_some() {
            (self._end.unwrap())(self);
        }
    }

    pub fn name(&self) -> &'static str {
        return self.name;
    }

    /// Should get called at the end of ``_init_<km>``
    pub fn initialized(&mut self) {
        self.initialized = true;
    }

    /// Should get called at the end of ``_end_<km>`` if the module ran successfully
    pub fn success(&mut self) {
        self.success = true;
    }

    /// Should get called if an error occurs
    pub fn error(&mut self) {
        self.success = false;
    }

    /// Returns ``self.success``
    pub fn is_successful(&self) -> bool {
        return self.success;
    }
}

impl Debug for KernelModule {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("KernelModule")
            .field("name", &self.name)
            .field("initialized", &self.initialized)
            .field("success", &self.success)
            .finish()
    }
}

pub enum ModuleType {
    /// ``InKernel`` is for a module that is linked to Novusk
    InKernel,
    /// ``InUser`` is for a module that runs outside of Novusk
    InUser,
}

#[macro_export]
macro_rules! start_module {
    ($($km_name_init:ident)*, $($km_name_end:ident)*) => {
        extern "C" {
            fn $($km_name_init)*();
            fn $($km_name_end)*();
        }

        $($km_name_init)*();
        $($km_name_end)*();
    };
}
