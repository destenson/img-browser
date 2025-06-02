use std::path::Path;
use image::imageops::FilterType;
use windows::Win32::Foundation::{GetLastError, HWND};
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleBitmap, CreateCompatibleDC,
    DeleteDC, GetDC, ReleaseDC,
    SelectObject, SetDIBitsToDevice, 
    BITMAPINFO, BITMAPINFOHEADER, 
    BI_RGB, DIB_RGB_COLORS,
    HGDIOBJ, RGBQUAD,
};
use crate::platform::win32::main::get_app_from_window;

/// Loads an image from a file path and returns it as a bitmap handle
pub fn load_image_as_bitmap<P: AsRef<Path>>(hwnd: HWND, img_path: P) -> (HGDIOBJ, i32, i32) {
    
    if let Some(app) = unsafe { get_app_from_window(hwnd) } {
        use image::GenericImageView;
        // Load the image using the `image` crate
        let img = image::open(&img_path).expect("Failed to load image");
        let (mut img_width, mut img_height) = img.dimensions();
        log::info!("Loaded image dimensions: {} x {}", img_width, img_height);

        let (window_width, window_height) = app.state.window_size;
        let (mut width, mut height) = (img_width, img_height);
        if img_width > window_width as u32 {
            width = window_width as u32;
            height = (img_height as f32 * (window_width as f32 / img_width as f32)) as u32;
        }
        if img_height > window_height as u32 {
            height = window_height as u32;
            width = (img_width as f32 * (window_height as f32 / img_height as f32)) as u32;
        }
        
        let mut img = img
            .resize_exact(width, height, FilterType::Lanczos3)
            .to_rgba8();
    
        // Convert from RGBA to BGRA by swapping R and B channels
        for pixel in img.pixels_mut() {
            let r = pixel[0];
            let b = pixel[2];
            pixel[0] = b;
            pixel[2] = r;
        }
    
    
        // Create a device context for the entire screen
        let hdc_screen = unsafe { GetDC(None) }; // Get the screen's device context
        let hdc = unsafe { CreateCompatibleDC(Some(hdc_screen)) };
    
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
    
        let prev_bmp = unsafe { SelectObject(hdc, hbitmap.into()) };
    
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
            ReleaseDC(None, hdc_screen);
        }

        app.state.set_current_image(img_path.as_ref(), app.state.window_size);

        // Return the bitmap handle
        (hbitmap.into(), width as i32, height as i32)
    } else {
        panic!("Failed to get app from window");
    }
    
}

/// Loads an image from a file path and returns it as a bitmap handle
pub fn load_image_as_bitmap_unscaled<P: AsRef<Path>>(img_path: P) -> (HGDIOBJ, i32, i32) {
    use image::GenericImageView;
    // Load the image using the `image` crate
    let img = image::open(img_path).expect("Failed to load image");
    let (width, height) = img.dimensions();
    log::info!("Loaded image dimensions: {} x {}", width, height);
    let mut img = img.to_rgba8();

    // Convert from RGBA to BGRA by swapping R and B channels
    for pixel in img.pixels_mut() {
        let r = pixel[0];
        let b = pixel[2];
        pixel[0] = b;
        pixel[2] = r;
    }

    // Create a device context for the entire screen
    let hdc_screen = unsafe { GetDC(None) }; // Get the screen's device context
    let hdc = unsafe { CreateCompatibleDC(Some(hdc_screen)) };

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

    let prev_bmp = unsafe { SelectObject(hdc, hbitmap.into()) };

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
        ReleaseDC(None, hdc_screen);
    }

    // Return the bitmap handle
    (hbitmap.into(), width as i32, height as i32)
}
