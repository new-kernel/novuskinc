/// For defining the main function in a kernel module
/// Example:
/// ```rust
/// fn my_module_init() {
/// 	/* Stuff */
/// }
///
/// module_init!(my_module_init);
/// // module init
/// ```

#[macro_export]
macro_rules! module_init {
	($init:path) => {
        #[export_name = "main"]
        pub extern "Rust" fn main() {
            let i: fn() = $init;

            extern "Rust" {
                fn end_module();
            }

            i();
            end_module();
        }
    };
}

/// For defining the end function in a kernel module
/// Example:
/// ```rust
/// fn my_module_init() {
/// 	/* Stuff */
/// }
///
/// fn my_module_end() {
/// 	/* Stuff */
/// }
///
/// module_end!(my_module_init);
/// // module init
/// ```

#[macro_export]
macro_rules! module_end {
    ($end:ident) => {
        #[no_manlge]
        pub extern "Rust" fn end_module() {
            let e = $end();
            e();
        }
    }
}