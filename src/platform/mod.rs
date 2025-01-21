

#[cfg(target_os = "windows")]
pub mod win32_main;

#[cfg(target_os = "windows")]
pub use win32_main::main;
