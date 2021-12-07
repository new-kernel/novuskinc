/// The module_init! macro is used for defining the start function of a kernel module
///
/// Example:
/// ```rust
/// fn my_module_start() {
///     printk!("My Module started!\n");
/// }
///
/// module_init!(my_module_init, my_module_start);
/// ```
///
#[macro_export]
macro_rules! module_init {
    ($km_name_init:ident, $km_name_start:ident) => {
        #[no_mangle]
        pub extern "C" fn $km_name_init() {
            $km_name_start();
        }
    };
}

/// The module_end! macro is used for defining the end function of a kernel module, this function
/// should be used to check if the init function was successful.
///
/// Example:
/// ```rust
/// fn my_module_finish() {
///     printk!("My module end!\n");
/// }
///
/// module_end!(my_module_end, my_module_finish)
/// ```
///
#[macro_export]
macro_rules! module_end {
   ($km_name_end:ident, $km_name_finish:ident) => {
        #[no_mangle]
        pub extern "C" fn $km_name_end() {
            $km_name_finish();
        }
    };
}

/// The start_module! macro is used for starting a kernel module, make sure your module is linked to
/// your project.
///
/// Example:
/// ```rust
/// start_module!(km_module_init, my_module_end);
///
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
