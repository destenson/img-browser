#![allow(non_snake_case, unused)]

mod main;

use main::run_window_loop;

use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM, GetLastError},
    Graphics::Gdi::{HBITMAP, HDC, DeleteObject, DeleteDC, ReleaseDC, CreateCompatibleDC, CreateCompatibleBitmap, BeginPaint, BitBlt, EndPaint, GetDC, SelectObject, SetDIBitsToDevice, UpdateWindow, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HBRUSH, HGDIOBJ, PAINTSTRUCT, RGBQUAD, SRCCOPY},
    UI::HiDpi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE},
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
        // Only update if we have a valid window handle
        if !self.hwnd.is_invalid() {
            unsafe {
                UpdateWindow(self.hwnd).ok()?;
            }
        }
        Ok(())
    }
    
    pub fn load_image(&mut self, path: &str) -> windows::core::Result<()> {
        let (bitmap, width, height) = load_image_as_bitmap(path);
        
        // Clean up old bitmap if it exists
        if !self.hbitmap.is_invalid() {
            unsafe { DeleteObject(self.hbitmap) };
        }
        
        // Convert HGDIOBJ to HBITMAP
        self.hbitmap = unsafe { HBITMAP(bitmap.0) };
        self.width = width;
        self.height = height;
        
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
        let hdc = unsafe { GetDC(HWND::default()) };
        let hdc_mem = unsafe { CreateCompatibleDC(hdc) };
        let hbitmap = unsafe { CreateCompatibleBitmap(hdc, width, height) };
        
        // Create a window struct without initializing the hwnd
        // The actual window will be created in the message loop
        Ok(Window {
            hwnd: HWND::default(), // This will be set properly in run_window_loop
            hdc,
            hdc_mem,
            hbitmap,
            width,
            height,
        })
    }
    
    fn message_loop(&self, window: Window, app: &mut Self::App) -> super::Result<()> {
        // Call the window loop function from win32_main.rs
        Ok(run_window_loop(window, app)?)
    }
    
    fn run(&self, mut app: crate::App) -> super::Result<()> {
        let crate::App { ref config, ref mut state } = app;

        log::info!("Running on Windows");
        log::info!("Config: {:?}", config);
        log::info!("State: {:?}", state);
        
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
        log::info!("Running message loop");
        self.message_loop(window, &mut app)
    }
}

/// Loads an image from a file path and returns it as a bitmap handle
fn load_image_as_bitmap(img_path: &str) -> (HGDIOBJ, i32, i32) {
    use image::GenericImageView;
    // Load the image using the `image` crate
    let img = image::open(img_path).expect("Failed to load image");
    let (width, height) = img.dimensions();
    log::info!("Loaded image dimensions: {} x {}", width, height);
    let img = img.to_rgba8();
    
    // Create a device context for the entire screen
    let hdc_screen = unsafe { GetDC(None) }; // Get the screen's device context
    let hdc = unsafe { CreateCompatibleDC(hdc_screen) };

    // Create a compatible bitmap
    let hbitmap = unsafe {
        CreateCompatibleBitmap(
            hdc_screen,
            width as i32,
            height as i32,
        )
    };

    if hbitmap.is_invalid() {
        panic!("Failed to create compatible bitmap.");
    }

    // Set the bitmap bits
    let bmp_info_header = BITMAPINFOHEADER {
        biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
        biWidth: width as i32,
        biHeight: -(height as i32), // Negative for top-down bitmap
        biPlanes: 1,
        biBitCount: 32,
        biCompression: BI_RGB.0, // Use BI_RGB for uncompressed RGB
        ..Default::default()
    };

    let prev_bmp = unsafe { SelectObject(hdc, hbitmap) };

    let mut bmp_info = BITMAPINFO {
        bmiHeader: bmp_info_header,
        bmiColors: [RGBQUAD {
            rgbRed: 0,
            rgbGreen: 0,
            rgbBlue: 0,
            rgbReserved: 0,
        }; 1],
    };

    unsafe {
        let res = SetDIBitsToDevice(
            hdc,
            0,
            0,
            width,
            height,
            0,
            0,
            0,
            height,
            img.as_raw().as_ptr() as *const _,
            &mut bmp_info,
            DIB_RGB_COLORS,
        );

        if res == 0 {
            log::error!("Failed to set DIB bits: {}", GetLastError().0);
        }
    }

    // Restore the device context
    unsafe { SelectObject(hdc, prev_bmp) };

    unsafe {
        let _ = DeleteDC(hdc);
        ReleaseDC(HWND(std::ptr::null_mut()), hdc_screen);
    }

    // Return the bitmap handle
    (hbitmap.into(), width as i32, height as i32)
}
