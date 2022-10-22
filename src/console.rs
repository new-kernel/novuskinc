use core::cell::Cell;

extern "C" {
    /// Initializes the kernel console driver.
    pub fn console_init();
    /// Initializes kernel printing, this allows ``printk!`` to work.
    pub fn printk_init();

    /// The kernel console driver
    pub static mut KERNEL_CONSOLE: &'static dyn KernelConsoleDriver;
}

/// The ``KernelConsole`` struct should be used as a field value of the console driver.
///
/// Example:
/// ```rust
/// use novuskinc::console::KernelConsole;
///
/// struct ConsoleDriver {
///     kernel_console: KernelConsole,
/// }
/// ```
pub struct KernelConsole {
    pub name: &'static str,
    pub width: Cell<u16>,
    pub height: Cell<u16>,
    pub line: Cell<u16>,
    pub chars_written: Cell<u64>,
}

impl KernelConsole {
    /// Add the name of the console driver with the console dimensions.
    ///
    /// Example:
    /// ```rust
    ///
    ///
    /// fn console_init() {
    ///     let console = ConsoleDriver {
    ///          //This kernel console can be used for the VGA text mode because it has the same dimensions (80x25).
    ///         kernel_console: KernelConsole::new("VGA Console", 80, 25),
    ///     };
    /// }
    /// ```
    pub fn new(console_name: &'static str, console_width: u16, console_height: u16) -> Self {
        return KernelConsole {
            name: console_name,
            width: Cell::new(console_width),
            height: Cell::new(console_height),
            line: Cell::new(0),
            chars_written: Cell::new(0),
        };
    }

    /// Updates ``self.chars_written``
    pub fn update_chars_written(&self, new_chars: u64) {
        self.chars_written.set(self.chars_written.get() + new_chars);
    }
}

/// The console driver has to implement ``KernelConsoleDriver`` because the functions the driver
/// implements get called by the kernel
pub trait KernelConsoleDriver {
    /// The ``write_character`` function is used to write a single character to a certain place
    fn write_character(&self, c: char, x: u16, y: u16) {

    }

    /// The ``write_string`` function is used to write a string and should call ``write_character``
    /// to write the characters individually
    fn write_string(&self, string: &str, x: u16, y: u16) {
        for b in string.as_bytes() {
            self.write_character(*b as char, x, y);
        }
    }

    /// The ``new_line`` function is used to move to the next line for the next character to be placed
    fn new_line(&self) {

    }

    /// The ``clear_screen`` function is used to clear the kernel console, the option value is an
    /// optional argument, it can be used for whatever you want.
    fn clear_screen(&self, option: u16) {

    }

    /// This should return the ``width`` and ``height`` field of ``KernelConsole.``
    fn dimensions(&self) -> (u16, u16) {
        return (0, 0);
    }
}
