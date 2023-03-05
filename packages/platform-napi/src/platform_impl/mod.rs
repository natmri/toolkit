#[cfg(windows_platform)]
#[path = "windows/mod.rs"]
mod platform;
#[cfg(linux_platform)]
#[path = "linux/mod.rs"]
mod platform;
#[cfg(macos_platform)]
#[path = "macos/mod.rs"]
mod platform;

pub use self::platform::*;
