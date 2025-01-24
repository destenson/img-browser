use windows::Win32::{
    Foundation::HWND,
    Graphics::Gdi::{HBITMAP, HDC},
};

/// Encapsulates a window.
pub struct Window {
    /// The window handle.
    hwnd: HWND,
    /// The device context.
    hdc: HDC,
    /// The bitmap device context.
    hdc_mem: HDC,
    /// The bitmap.
    hbitmap: HBITMAP,
    /// The width of the window.
    width: i32,
    /// The height of the window.
    height: i32,
}
