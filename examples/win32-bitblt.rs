
use image::{GenericImageView, ImageBuffer, Rgba};
use imp::CanInto;
use windows::{
    core::*,
    Win32::{
        Foundation::{GetLastError, COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, WPARAM}, 
        Graphics::Gdi::{BeginPaint, BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, EndPaint, GetDC, ReleaseDC, SelectObject, SetDIBitsToDevice, UpdateWindow, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HBRUSH, HGDIOBJ, PAINTSTRUCT, RGBQUAD, SRCCOPY}, 
        System::LibraryLoader::GetModuleHandleW, 
        UI::{HiDpi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE}, WindowsAndMessaging::{CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW, GetMessageW, RegisterClassExW, SetLayeredWindowAttributes, ShowWindow, TranslateMessage, CS_HREDRAW, CS_VREDRAW, HCURSOR, HMENU, LWA_ALPHA, SHOW_WINDOW_CMD, WM_CLOSE, WM_PAINT, WNDCLASSEXW, WS_EX_LAYERED, WS_EX_TOPMOST, WS_OVERLAPPEDWINDOW, WS_POPUP, WS_VISIBLE}}
    }
};

static mut BITMAP_HANDLE: Option<HGDIOBJ> = None;
static mut WIDTH: Option<i32> = None;
static mut HEIGHT: Option<i32> = None;

fn main() {
    let show_title_bar = true;
    unsafe {
        let _ = SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE);
        let handle_instance = GetModuleHandleW(None).unwrap();
        let window_class = "Glorp Class";
        let window_name = "Glorp Overlay";

        let wc = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            hInstance: handle_instance.into(),
            // hCursor: HCURSOR(std::ptr::null_mut()),
            // hbrBackground: HBRUSH(std::ptr::null_mut()),
            lpszClassName: w!("Glorp Class"),
            ..Default::default()
        };

        if RegisterClassExW(&wc) == 0 {
            panic!("Failed to register window class.");
        }

        BITMAP_HANDLE = Some(load_image_as_bitmap("vendor/oculante/res/screenshot_exif.png")); // Update with your image path

        // Retrieve width and height from the loaded image
        let (width, height) = (
            WIDTH.unwrap_or(30), HEIGHT.unwrap_or(30)
        ); // Default values if image loading fails

        let dwstyle = if show_title_bar {
            WS_OVERLAPPEDWINDOW | WS_VISIBLE
        } else {
            WS_POPUP | WS_VISIBLE
        };
        let hwnd = CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TOPMOST, 
            &HSTRING::from(window_class), 
            &HSTRING::from(window_name),
            dwstyle, 
            width/2,
            height/2, 
            width*3/2,
            height*3/2, 
            HWND(std::ptr::null_mut()),
            HMENU(std::ptr::null_mut()), 
            handle_instance, 
            Some(std::ptr::null_mut())
        ).unwrap();

        let transparency = 255;
        SetLayeredWindowAttributes(hwnd, COLORREF(0), transparency, LWA_ALPHA).unwrap();

        let _ = ShowWindow(hwnd, SHOW_WINDOW_CMD(1));
        let _ = UpdateWindow(hwnd);

        let mut msg = std::mem::zeroed();
        while GetMessageW(&mut msg, HWND(std::ptr::null_mut()), 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        if let Some(bitmap) = BITMAP_HANDLE {
            let _ = DeleteObject(bitmap);
        }
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_PAINT => {
            println!("WM_PAINT received");
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);
            
            if let Some(bitmap) = BITMAP_HANDLE {
                let hdc_mem = CreateCompatibleDC(hdc);
                let prev_bmp = SelectObject(hdc_mem, bitmap);

                // Use WIDTH and HEIGHT for the dimensions
                if let (Some(width), Some(height)) = (WIDTH, HEIGHT) {
                    let bres = BitBlt(hdc, 0, 0, width, height, hdc_mem, 0, 0, SRCCOPY);
                    if bres.is_err() {
                        println!("BitBlt failed: {}", GetLastError().0);
                    }
                    // dbg!(bres);
                } else {
                    panic!("Width or height not set correctly");
                }

                SelectObject(hdc_mem, prev_bmp);

                DeleteDC(hdc_mem).expect("DeleteDC failed");
                println!("DeleteDC called");
            }

            EndPaint(hwnd, &ps).expect("EndPaint failed");
            println!("EndPaint called");
            return LRESULT(0);
        }
        WM_CLOSE => {
            println!("WM_CLOSE received");
            DestroyWindow(hwnd).expect("DestroyWindow failed");
            return LRESULT(0);
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn load_image_as_bitmap(image_path: &str) -> HGDIOBJ {
    // Load the image using the `image` crate
    let img = image::open(image_path).expect("Failed to load image");
    let (width, height) = img.dimensions();
    println!("Loaded image dimensions: {} x {}", width, height); // Debug line
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
        biCompression: 0, // Use BI_RGB for uncompressed RGB
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
            println!("Failed to set DIB bits: {}", GetLastError().0);
        }
    }

    // Restore the device context
    unsafe { SelectObject(hdc, prev_bmp) };

    unsafe {
        let _ = DeleteDC(hdc);
        ReleaseDC(HWND(std::ptr::null_mut()), hdc_screen);
    }

    // Set global width and height
    unsafe {
        WIDTH = Some(width as i32);
        HEIGHT = Some(height as i32);
    }

    // Return the bitmap handle
    hbitmap.into()
}

