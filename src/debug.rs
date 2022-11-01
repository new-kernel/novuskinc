extern "C" {
    /// Returns if the ``debug`` feature is being used in Novusk
    pub fn _debug_on() -> bool;

    /// Used for printing while debugging, can be used as a stopping point (not a breakpoint)
    pub fn _debug_write(fmt: core::fmt::Arguments, stop: bool);
}
