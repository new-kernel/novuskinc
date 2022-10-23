pub mod channel {
    extern "C" {
        pub static PROP: u32;
    }
}

pub mod response {
    extern "C" {
        /// Response Success
        pub static SUCCESS: u32;
        /// Response Error
        pub static ERROR: u32;
    }
}

pub mod tag {
    extern "C" {
        /// Get Firmware Revision
        pub static GET_FMWREV: u32;
        /// Get Board Modle
        pub static GET_BORDMODLE: u32;
        /// Get Board Revision
        pub static GET_BOARDREV: u32;
        /// Get MAC Address
        pub static GET_MACADDR: u32;
        /// Get Serial
        pub static GET_SERIAL: u32;
        /// Get ARM Memory
        pub static GET_ARMMEM: u32;
        /// Get VC Memory
        pub static GET_VCMEM: u32;
        /// Get Clocks
        pub static GET_CLOCKS: u32;
        /// Get Command Line
        pub static GET_CMDLINE: u32;
        /// Get DMA channels
        pub static GET_DMACHNLS: u32;
        /// Get Power State
        pub static GET_PWSTATE: u32;
        /// Get Timing
        pub static GET_TIMING: u32;
        /// Get Frame Buffer
        pub static GET_FB: u32;
        /// Get Pitch
        pub static GET_PITCH: u32;

        /// Set Clock Rate
        pub static SET_CLKRATE: u32;
        /// Set Physical Width and Height
        pub static SET_PHYSWH: u32;
        /// Set Virtual Width and Height
        pub static SET_VIRTWH: u32;
        /// Set Virtual Offset
        pub static SET_VIRTOFFSET: u32;
        /// Set Depth
        pub static SET_DEPTH: u32;
        /// Set Pixel Order
        pub static SET_PXLORDER: u32;
    }
}

extern "C" {
    pub fn set_mb_index(i: usize, value: u32);
    pub fn set_mb_buffer(buf: &[u32; 36]);
    pub fn get_mb_index(i: usize) -> u32;
    pub fn get_mb_buffer() -> &'static [u32; 36];
    pub fn call_mb(channel: u32) -> u32;
}
