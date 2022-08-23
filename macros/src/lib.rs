//#[macro_use] extern crate alloc;
#[macro_use] extern crate proc_macro;

use proc_macro::TokenStream;

/// ``module_init`` is used for defining the start of a kernel module, this should be used for setting
/// up the module and the things that it will do.
///
/// There are two functions that need to be defined for a kernel module to work, ``_init_<km_name>``
/// and ``_end_<km_name>``.
///
/// Examples:
/// ```rust
/// // A module that initializes a driver inside the kernel
/// fn _init_driver() {
///     // Setup the driver
/// }
///
/// module_init!(ModuleType::InKernel, driver);
/// ```
///
/// ```rust
/// // A module that runs as a binary:
/// fn _init_driver() {
///     // Setup the driver
/// }
///
/// // A "main" function doesn't need to be defined, this will do it for you
/// module_init!(ModuleType::InUser, driver);
#[proc_macro]
pub fn module_init(input: TokenStream) -> TokenStream {
    let args = Vec::from_iter(input.to_string().split(" ").map(String::from));
    let mut output = String::new();

    match &*args[0] {
        "ModuleType" => { },
        _ => panic!("Expected 'ModuleType', got: {}", args[0]),
    }

    match &*args[2].replace(",", "") {
        "InKernel" => {
            output = format!("{}{}{}{}{}{}{}",
                           "#[no_mangle]\npub unsafe extern \"C\" fn ", "init_", args[3], "() {\n",
                           "_init_", args[3], "();\n}"
            );
        },
        "InUser" => {
            output = format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                    "#[no_mangle]\npub unsafe extern \"C\" fn ", "init_", args[3], "() {\n",
                    "_init_", args[3], "();\n}\n",
                    "#[no_mangle]\n
                    pub unsafe extern \"C\" fn main() {",
                    "init_", args[3], "();\n",
                    "end_", args[3], "();\n}",
            );
        },
        _ => panic!("Expected 'InKernel' or 'InUser', got: {}", args[2]),
    }

    return output.parse().unwrap();
}

/// The ``module_end`` macro is used for defining the end of a kernel module, this should be used
/// for starting the driver or adding it to ``DRIVER_MANAGER`` if ``module_init`` used ``InKernel``.
///
/// Example:
/// ```rust
/// fn _end_driver() {
///     // Finish driver
/// }
///
/// module_end!(driver);
#[proc_macro]
pub fn module_end(input: TokenStream) -> TokenStream {
    let mut output = String::new();

    output = format!("{}{}{}{}{}{}{}",
                     "#[no_mangle]\npub unsafe extern \"C\" fn ", "end_", input.to_string(), "() {\n",
                     "_end_", input.to_string(), "();\n}"
    );

    return output.parse().unwrap();
}
