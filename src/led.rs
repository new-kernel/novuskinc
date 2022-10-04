extern "C" {
    /// Turns on an led for ``sleep`` seconds or CPU cycles.
    pub fn led_blink(sleep: usize);
}

pub trait Led {
    fn on(&mut self) {

    }

    fn off(&mut self) {

    }
}
