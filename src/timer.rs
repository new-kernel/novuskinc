extern "C" {
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
