extern "C" {
    /// Used for adding a timer (usually an interrupt timer) to Novusk's timer list.
    pub fn add_timer(name: &'static str);
    /// Updates a timer, should be called in a timer interrupt's handler function.
    pub fn update_timer(name: &'static str, update: f64);
    /// Gets the value of a timer.
    pub fn get_timer_value(name: &'static str) -> f64;

    pub fn device_timer_init();
}

pub trait Timer {
    fn value(&self) -> u32 {
        0
    }

    fn interval(&self) -> u32 {
        0
    }

    fn set_value(&self, val: u32) {

    }
}
