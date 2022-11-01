use core::borrow::BorrowMut;
use core::fmt::Arguments;

extern "C" {
    /// This should be defined in the device specific driver, it returns the ((``width``, ``height``), and ``address``).
    pub fn device_display_info() -> ((u32, u32), usize);
}

#[derive(Copy, Clone)]
pub struct FbInfo {
    pub name: &'static str,
    pub fb_size: (u32, u32),
    pub fb_addr: *mut u32,
}

impl FbInfo {
    pub fn new() -> Self {
        return FbInfo {
            name: "None",
            fb_size: (0, 0),
            fb_addr: 0x0 as *mut u32,
        };
    }

    pub fn set(&mut self, fb_name: &'static str, size: (u32, u32), addr: *mut u32) {
        self.name = fb_name;
        self.fb_size = size;
        self.fb_addr = addr;
    }
}

pub trait FrameBufferGraphics {
    fn graphics_write(&self, byte: u8, x: usize, y: usize) {

    }

    fn graphics_write_string(&self, string: &str, x: usize, y: usize) {
        let mut nx = x;
        let mut ny = y;

        for b in string.as_bytes() {
            nx += 1;

            if *b == b'\n' {
                ny += 1;
                nx = 0;
            }

            self.graphics_write(*b, nx, ny);
        }
    }

    fn graphics_write_fmt(&self, fmt: Arguments) {

    }

    fn graphics_pixel(&self, color: Color, x: u32, y: u32) {

    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    RGB {
        r: u32,
        g: u32,
        b: u32,
    },
    Hex {
        d: u32
    },
}
