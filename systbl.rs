cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        pub const READ: i32 = 0;
        pub const WRITE: i32 = 1;
        pub const VERSION: i32 = 2;
        pub const REBOOT: i32 = 3;
    }
}

cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        pub const VERSION: i32 = 1;
        pub const READ: i32 = 63;
        pub const WRITE: i32 = 64;
        pub const REBOOT: i32 = 142;
    }
}

cfg_if! {
    if #[cfg(target_arch = "arm")] {
        pub const READ: i32 = 3;
        pub const WRITE: i32 = 4;
        pub const REBOOT: i32 = 88;
        pub const VERSION: i32 = 2;
    }
}