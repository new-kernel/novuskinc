pub use novuskinc_macros::{module_end, module_init};

pub enum ModuleType {
    /// ``InKernel`` is for a module that is linked to Novusk
    InKernel,
    /// ``InUser`` is for a module that runs outside of Novusk, it is used for
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
