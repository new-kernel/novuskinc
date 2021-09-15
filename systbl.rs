cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        pub const READ: i32 = 0;
        pub const WRITE: i32 = 1;
        pub const VERSION: i32 = 2;
        pub const REBOOT: i32 = 3;
    }
}
