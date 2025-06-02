#![allow(non_snake_case, unused)]

mod main;
mod dialogs;
mod fs;
mod bmp;

use std::path::{Path, PathBuf};
use image::imageops::FilterType;
pub use dialogs::{open_file_dialog, open_folder_dialog};
pub use main::run_window_loop;
pub use fs::get_known_folder_path;

use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM, GetLastError},
    Graphics::Gdi::{HBITMAP, HDC, DeleteObject, DeleteDC, ReleaseDC, CreateCompatibleDC, CreateCompatibleBitmap, BeginPaint, BitBlt, EndPaint, GetDC, SelectObject, SetDIBitsToDevice, UpdateWindow, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HBRUSH, HGDIOBJ, PAINTSTRUCT, RGBQUAD, SRCCOPY},
    UI::HiDpi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE},
};
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use crate::platform::win32::bmp::{load_image_as_bitmap, load_image_as_bitmap_unscaled};

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
    /// The path to the image, if any.
    pub img_path: Option<PathBuf>,
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            // Clean up resources in reverse order of creation
            DeleteObject(self.hbitmap.into());
            DeleteDC(self.hdc_mem);
            ReleaseDC(Some(self.hwnd), self.hdc);
        }
    }
}

impl Window {
    pub fn show(&self) -> windows::core::Result<()> {
        // Only update if we have a valid window handle
        if !self.hwnd.is_invalid() {
            unsafe {
                UpdateWindow(self.hwnd).ok()?;
            }
        }
        Ok(())
    }
    
    pub fn load_image_unscaled<P: AsRef<Path>>(&mut self, path: P) -> windows::core::Result<()> {
        let (bitmap, width, height) = load_image_as_bitmap_unscaled(path);
        
        // Clean up old bitmap if it exists
        if !self.hbitmap.is_invalid() {
            unsafe { DeleteObject(self.hbitmap.into()) };
        }
        
        // Convert HGDIOBJ to HBITMAP
        self.hbitmap = unsafe { HBITMAP(bitmap.0) };
        self.width = width;
        self.height = height;
        
        Ok(())
    }
    
    pub fn load_image<P: AsRef<Path>>(&mut self, path: P) -> windows::core::Result<()> {
        let (bitmap, width, height) = load_image_as_bitmap(self.hwnd, &path);
        
        // Clean up old bitmap if it exists
        if !self.hbitmap.is_invalid() {
            unsafe { DeleteObject(self.hbitmap.into()) };
        }
        
        // Convert HGDIOBJ to HBITMAP
        self.hbitmap = unsafe { HBITMAP(bitmap.0) };
        self.width = width;
        self.height = height;
        self.img_path = Some(path.as_ref().to_path_buf());
        
        Ok(())
    }
    
    pub fn is_valid(&self) -> bool {
        !self.hwnd.is_invalid()
    }
}

pub struct Platform {}

impl super::Platform for Platform {
    type Window = Window;
    type App = crate::App;
    
    fn create_window(&self, width: i32, height: i32) -> super::Result<Window> {
        // Get the DC for the screen
        let hdc = unsafe { GetDC(None) };
        let hdc_mem = unsafe { CreateCompatibleDC(Some(hdc)) };
        let hbitmap = unsafe { CreateCompatibleBitmap(hdc, width, height) };
        if hdc.is_invalid() || hdc_mem.is_invalid() || hbitmap.is_invalid() {
            let error = unsafe { GetLastError() };
            return Err(crate::Error::PlatformError(format!(
                "Failed to create window resources: {:?}",
                error
            )));
        }
        
        // Create a window struct without initializing the hwnd
        // The actual window will be created in the message loop
        Ok(Window {
            hwnd: HWND::default(), // This will be set properly in run_window_loop
            hdc,
            hdc_mem,
            hbitmap,
            width,
            height,
            img_path: None,
        })
    }
    
    fn message_loop(&self, window: Window, app: &mut Self::App) -> super::Result<()> {
        // Call the window loop function from win32_main.rs
        Ok(run_window_loop(window, app)?)
    }
    
    fn run(&self, mut app: crate::App) -> super::Result<()> {
        let crate::App { ref config, ref mut state } = app;

        log::trace!("Running on Windows");
        log::trace!("{}", config);
        log::trace!("{}", state);
        
        // Enable DPI awareness
        unsafe {
            let _ = SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE);
        }
        
        // Determine window size based on image size if an image is provided
        let (window_width, window_height) = if let Some(path) = &config.image_path {
            // Try to load the image first to get its dimensions
            match image::image_dimensions(path) {
                Ok((width, height)) => {
                    log::info!("Image dimensions: {} x {}", width, height);
                    // Update the app state with the image info
                    state.set_current_image(path.clone(), (width, height));
                    (width as i32, height as i32)
                },
                Err(e) => {
                    log::error!("Failed to get image dimensions: {}", e);
                    (config.width as i32, config.height as i32)
                }
            }
        } else {
            (config.width as i32, config.height as i32)
        };
        
        // Create the window with appropriate dimensions
        log::info!("Creating window with dimensions: {} x {}", window_width, window_height);
        let mut window = self.create_window(window_width, window_height)?;
        
        // Load image if an image path was provided
        if let Some(path) = &config.image_path {
            log::info!("Loading image: {}", path);
            if let Err(e) = window.load_image(path) {
                log::error!("Failed to load image: {:?}", e);
            }
        }
        
        // Run the message loop, which will properly create and show the window
        log::debug!("Running message loop");
        self.message_loop(window, &mut app)
    }
    
    fn get_special_folder(&self, folder_type: super::SpecialFolder) -> Option<std::path::PathBuf> {
        fs::get_special_folder_path(folder_type)
    }
    
    fn create_directory(&self, path: &std::path::Path) -> super::Result<()> {
        // Use our Windows-specific directory creation
        fs::create_directory_windows(path)
            .map_err(|e| std::io::Error::new(
                std::io::ErrorKind::Other, 
                format!("Failed to create directory: {}", e)
            ).into())
    }
    
    fn directory_exists(&self, path: &std::path::Path) -> bool {
        // Check if the directory exists using Windows API
        fs::directory_exists_windows(path)
    }
}

