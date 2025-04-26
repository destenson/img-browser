#![allow(non_snake_case, unused)]


use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    Graphics::Gdi::{HBITMAP, HDC, DeleteObject, DeleteDC, ReleaseDC, CreateCompatibleDC, CreateCompatibleBitmap, BeginPaint, BitBlt, EndPaint, GetDC, SelectObject, SetDIBitsToDevice, UpdateWindow, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HBRUSH, HGDIOBJ, PAINTSTRUCT, RGBQUAD, SRCCOPY}, 
};

/// Encapsulates a window.
pub struct Window {
    /// The window handle.
    pub hwnd: HWND,
    /// The device context.
    pub hdc: HDC,
    /// The bitmap device context.
    pub hdc_mem: HDC,
    /// The bitmap.
    pub hbitmap: HBITMAP,
    /// The width of the window.
    pub width: i32,
    /// The height of the window.
    pub height: i32,
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            // Clean up resources in reverse order of creation
            DeleteObject(self.hbitmap);
            DeleteDC(self.hdc_mem);
            ReleaseDC(self.hwnd, self.hdc);
        }
    }
}

impl Window {
    pub fn show(&self) -> windows::core::Result<()> {
        unsafe {
            UpdateWindow(self.hwnd).ok()?;
        }
        Ok(())
    }
}

pub struct Platform {}

impl super::Platform for Platform {
    type Window = Window;
    type App = crate::App;
    fn create_window(&self, width: i32, height: i32) -> super::Result<Window> {
        Ok(Window {
            hwnd: HWND::default(),
            hdc: HDC::default(),
            hdc_mem: HDC::default(),
            hbitmap: HBITMAP::default(),
            width,
            height,
        })
    }
    fn message_loop(&self, window: Window, app: &mut Self::App) -> super::Result<()> {
        todo!();
        Ok(())
    }
    fn run(&self, app: crate::App) -> super::Result<()> {
        let crate::App { config, state } = app;

        log::info!("Running on Windows");
        log::info!("Config: {:?}", config);
        log::info!("State: {:?}", state);
        
        // TODO: create the window and run the message loop
        log::info!("Creating window");
        
        let w = self.create_window(config.width as i32, config.height as i32)?;
        
        w.show()?;
        
        log::info!("Running message loop");
        self.message_loop(w, &mut crate::App::default())?;
        
        
        log::info!("Exiting");
        Ok(())
    }
}
