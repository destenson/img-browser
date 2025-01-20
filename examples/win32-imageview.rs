#![allow(non_snake_case)]
use std::{io::Read, sync::{Arc, Mutex}};

use windows::{
    core::*, Data::Xml::Dom::*,
    Win32::{
        Foundation::*, Graphics::Gdi::*,
        System::{LibraryLoader::*, Threading::*}, UI::{Controls::*, HiDpi::*, WindowsAndMessaging::*}
    },
};

use image::DynamicImage;

use defer::defer;

const image_path: &str = "vendor/oculante/res/screenshot_exif.png";

fn main() -> Result<()> {

    unsafe {SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE)}.expect("Failed to set process DPI awareness");
    let handle_instance = unsafe {GetModuleHandleW(None)}.expect("Failed to get module handle");

    let wc = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(window_proc),
        hInstance: handle_instance.into(),
        // hCursor: HCURSOR(std::ptr::null_mut()),
        // hbrBackground: HBRUSH(std::ptr::null_mut()),
        lpszClassName: w!("img-browser-rs"),
        ..Default::default()
    };

    if unsafe {RegisterClassExW(&wc)} == 0 {
        panic!("Failed to register window class.");
    }

    let (image_bmp, width, height) = load_image_as_bitmap(image_path);
    println!("Image dimensions: {}x{}", width, height);

    let window = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE(0),
            w!("STATIC"),
            w!("Image Viewer"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width as i32*2,
            height as i32*2,
            None,
            None,
            handle_instance,
            None,
        )
    }?;

    // let hdcMem: Arc<Mutex<HDC>> = Arc::new(Mutex::new(HDC::default()));

    // defer!({
    //     if !hBitmap.is_invalid() {
    //         let r = unsafe { DeleteObject(hBitmap) };
    //         if r == FALSE {
    //             panic!("Failed to delete object");
    //         }
    //     }
    // });

    // // c. copy the image data into the bitmap
    let copy_bitmap = |hdc: HDC| {
        // create a bitmap from the image
        let bmp_info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width as i32,
                biHeight: -(height as i32), // Negative for top-down bitmap
                biPlanes: 1,
                biBitCount: 32,
                biCompression: 0, // Use BI_RGB for uncompressed RGB
                ..Default::default()
            },
            bmiColors: [RGBQUAD{ rgbBlue: 0, rgbGreen: 0, rgbRed: 0, rgbReserved: 0 }; 1],
        };

        // let r = unsafe { SelectObject(*hdcMem.lock().unwrap(), image_bmp) };
        // if r.is_invalid() {
        //     panic!("Failed to select object");
        // }

        // // // c. copy the image data into the bitmap
        // let img_data = img.to_rgba8();
        // let img_src = img_data.as_ptr();
        // let img_src = img_src as *const std::ffi::c_void;
        // unsafe {std::ptr::copy(img_src, bits, img_data.len())};

        // unsafe {
        //     let r = SetDIBits(hdc, hbm, 0, height as u32, img_src, &bmp_info, DIB_RGB_COLORS);
        //     if r == 0 {
        //         panic!("Failed to set DIBits");
        //     }
        // }

        // TODO: how do I know if this succeeded or is even the right way do do it?
    };

    // unsafe {
    //     let r = SetDIBits(hdc, hBitmap, 0, height as u32, img_src, &bmp, DIB_RGB_COLORS);
    //     if r == 0 {
    //         panic!("Failed to set DIBits");
    //     }
    // }

    // d. draw the bitmap into the device context
    let draw = |hwnd: HWND| {
        // let mut paint = PAINTSTRUCT::default();
        // let hdc = unsafe { BeginPaint(window, &mut paint) };
        // if hdc.is_invalid() {
        //     panic!("Failed to begin paint");
        // }
        // defer!({
        //     println!("defer end paint");
        //     let r = unsafe {EndPaint(window, &paint)};
        //     if r == FALSE {
        //         panic!("Failed to end paint");
        //     }
        // });

        // let mut hbm = BITMAP::default();
        // let bm = &mut hbm;
        // let bm: *mut BITMAP = bm;
        // let bm = bm as *mut std::ffi::c_void;
        // let hBitmap = *hBitmap.lock().unwrap();
        // let r = unsafe {
        //     GetObjectW(hBitmap, std::mem::size_of::<BITMAP>() as i32, Some(bm))
        // };
        // println!("GetObjectW returned {}", r);
    
        // let hdcMem = *hdcMem.lock().unwrap();
        // unsafe {
        //     BitBlt(hdc, 0, 0, hbm.bmWidth, hbm.bmHeight, hdcMem, 0, 0, SRCCOPY)
        // }.expect("BitBlt failed");
        // println!("BitBlt complete");
        println!("drawing");
        let mut ps: PAINTSTRUCT = unsafe {std::mem::zeroed()};
        let hdc = unsafe{BeginPaint(hwnd, &mut ps)};
        
        if let Some(bitmap) = Some(image_bmp) {
            let hdc_mem = unsafe {CreateCompatibleDC(hdc)};
            let prev_bmp = unsafe {SelectObject(hdc_mem, bitmap)};

            let bres = unsafe {
                BitBlt(hdc, 0, 0, width, height, hdc_mem, 0, 0, SRCCOPY)
            };
            if bres.is_err() {
                println!("BitBlt failed: {}", unsafe{GetLastError()}.0);
            }
            // dbg!(bres);

            unsafe {SelectObject(hdc_mem, prev_bmp)};

            unsafe {DeleteDC(hdc_mem)}.expect("DeleteDC failed");
        }

        unsafe {EndPaint(hwnd, &ps)}.expect("EndPaint failed");
        println!("finished drawing");
        return LRESULT(0);
    };

    let ctr_base = 50;
    let mut ctr = ctr_base;
    // e. run the message loop
    let mut msg = MSG::default();
    loop {
        ctr -= 1;
        if ctr == 0 {
            break;
        }
        let r = unsafe { GetMessageW(&mut msg, None, 0, 0) };
        r.expect("GetMessageW failed");
        if r.0 == 0 {
            break;
        }
        match msg.message {
            WM_QUIT | WM_CLOSE => {
                if msg.message == WM_QUIT {
                    println!("WM_QUIT");
                } else {
                    println!("WM_CLOSE");
                }
                break;
            },
            WM_CREATE => {
                println!("WM_CREATE");

                // a. create a device context for the window
                let hdc = unsafe { GetDC(window) };
                if hdc.is_invalid() {
                    panic!("Failed to get device context");
                }
                defer!({
                    let r = unsafe {ReleaseDC(window, hdc)};
                    if r == 0 {
                        panic!("Failed to release device context");
                    }
                });            
                
                // let compat = unsafe { CreateCompatibleDC(hdc) };
                // if compat.is_invalid() {
                //     panic!("Failed to create compatible device context");
                // }
                // *hdcMem.lock().unwrap() = compat;

                // copy_bitmap(hdc);

                // continue;
            },
            // WM_PAINT => {
            //     println!("WM_PAINT");
            //     let r = unsafe {
            //         ValidateRect(window, None)
            //     };
            //     r.expect("ValidateRect failed");
            //     if r.0 == 0 {
            //         panic!("Failed to validate rect");
            //     }
            //     draw(window);
            //     // continue;
            // },
            WM_PAINT => {
                println!("WM_PAINT received");
                let mut ps: PAINTSTRUCT = unsafe{std::mem::zeroed()};
                let hdc = unsafe{BeginPaint(window, &mut ps)};
                
                if let Some(bitmap) = Some(image_bmp) {
                    let hdc_mem = unsafe{CreateCompatibleDC(hdc)};
                    let prev_bmp = unsafe{SelectObject(hdc_mem, bitmap)};
    
                    // Use WIDTH and HEIGHT for the dimensions
                    let bres = unsafe {
                        BitBlt(hdc, 0, 0, width, height, hdc_mem, 0, 0, SRCCOPY)
                    };
                    if bres.is_err() {
                        println!("BitBlt failed: {}", unsafe{GetLastError()}.0);
                    }
                    // dbg!(bres);
    
                    unsafe{SelectObject(hdc_mem, prev_bmp)};
    
                    unsafe{DeleteDC(hdc_mem)}.expect("DeleteDC failed");
                    println!("DeleteDC called");
                }
    
                unsafe{EndPaint(window, &ps)}.expect("EndPaint failed");
                println!("EndPaint called");
                // return LRESULT(0);
                continue;
            },
    
            WM_DESTROY => {
                // let hdcMem = *hdcMem.lock().unwrap();
                // let r = unsafe {DeleteDC(hdcMem)};
                // r.expect("DeleteDC failed");
                // let hBitmap = *hBitmap.lock().unwrap();
                let r = unsafe {DeleteObject(image_bmp)};
                r.expect("DeleteObject failed");
                unsafe { PostQuitMessage(0) };
                // continue;
            },
            WM_ACTIVATEAPP => println!("WM_ACTIVATEAPP"),
            WM_ACTIVATE => println!("WM_ACTIVATE"),
            WM_AFXFIRST => println!("WM_AFXFIRST"),
            WM_AFXLAST => println!("WM_AFXLAST"),
            WM_APP => println!("WM_APP"),
            WM_APPCOMMAND => println!("WM_APPCOMMAND"),
            WM_CANCELMODE => println!("WM_CANCELMODE"),
            WM_CHANGECBCHAIN => println!("WM_CHANGECBCHAIN"),
            WM_CAPTURECHANGED => println!("WM_CAPTURECHANGED"),
            WM_CHAR => println!("WM_CHAR"),
            WM_COMMAND => println!("WM_COMMAND"),
            WM_CTLCOLORBTN => println!("WM_CTLCOLORBTN"),
            WM_CTLCOLORDLG => println!("WM_CTLCOLORDLG"),
            WM_CTLCOLOREDIT => println!("WM_CTLCOLOREDIT"),
            WM_CTLCOLORLISTBOX => println!("WM_CTLCOLORLISTBOX"),
            WM_CTLCOLORMSGBOX => println!("WM_CTLCOLORMSGBOX"),
            WM_CTLCOLORSCROLLBAR => println!("WM_CTLCOLORSCROLLBAR"),
            WM_CTLCOLORSTATIC => println!("WM_CTLCOLORSTATIC"),
            WM_DEVICECHANGE => println!("WM_DEVICECHANGE"),
            WM_DISPLAYCHANGE => println!("WM_DISPLAYCHANGE"),
            WM_DRAWCLIPBOARD => println!("WM_DRAWCLIPBOARD"),
            WM_DRAWITEM => println!("WM_DRAWITEM"),
            WM_ERASEBKGND => println!("WM_ERASEBKGND"),
            WM_GETMINMAXINFO => println!("WM_GETMINMAXINFO"),
            WM_INPUT => println!("WM_INPUT"),
            WM_KEYDOWN => println!("WM_KEYDOWN"),
            WM_KEYLAST => println!("WM_KEYLAST"),
            WM_KEYUP => println!("WM_KEYUP"),
            WM_LBUTTONDOWN => println!("WM_LBUTTONDOWN"),
            WM_LBUTTONUP => println!("WM_LBUTTONUP"),
            WM_MOUSEMOVE => println!("WM_MOUSEMOVE"),
            WM_RBUTTONDOWN => println!("WM_RBUTTONDOWN"),
            WM_RBUTTONUP => println!("WM_RBUTTONUP"),
            WM_SIZE => println!("WM_SIZE"),
            WM_SYSKEYDOWN => println!("WM_SYSKEYDOWN"),
            WM_SYSKEYUP => println!("WM_SYSKEYUP"),
            WM_SYSCHAR => println!("WM_SYSCHAR"),
            WM_SYSDEADCHAR => println!("WM_SYSDEADCHAR"),
            WM_USER => println!("WM_USER"),
            WM_XBUTTONDOWN => println!("WM_XBUTTONDOWN"),
            WM_XBUTTONUP => println!("WM_XBUTTONUP"),
            WM_MOUSEHOVER => println!("WM_MOUSEHOVER"),
            WM_MOUSELEAVE => println!("WM_MOUSELEAVE"),
            WM_NCACTIVATE => println!("WM_NCACTIVATE"),
            WM_NCCALCSIZE => println!("WM_NCCALCSIZE"),
            WM_NCHITTEST => println!("WM_NCHITTEST"),
            WM_NCLBUTTONDBLCLK => println!("WM_NCLBUTTONDBLCLK"),
            WM_NCLBUTTONDOWN => println!("WM_NCLBUTTONDOWN"),
            WM_NCLBUTTONUP => println!("WM_NCLBUTTONUP"),
            WM_NCMBUTTONDBLCLK => println!("WM_NCMBUTTONDBLCLK"),
            WM_NCMBUTTONDOWN => println!("WM_NCMBUTTONDOWN"),
            WM_NCMBUTTONUP => println!("WM_NCMBUTTONUP"),
            WM_NCMOUSEHOVER => println!("WM_NCMOUSEHOVER"),
            WM_NCMOUSELEAVE => println!("WM_NCMOUSELEAVE"),
            WM_NCMOUSEMOVE => println!("WM_NCMOUSEMOVE"), // this seeems to be the one that triggers when the mouse moves over the window
            WM_NCPAINT => println!("WM_NCPAINT"),
            WM_NCRBUTTONDBLCLK => println!("WM_NCRBUTTONDBLCLK"),
            WM_NCRBUTTONDOWN => println!("WM_NCRBUTTONDOWN"),
            WM_NCRBUTTONUP => println!("WM_NCRBUTTONUP"),
            WM_NCXBUTTONDOWN => println!("WM_NCXBUTTONDOWN"),
            WM_NCXBUTTONUP => println!("WM_NCXBUTTONUP"),
            WM_NCXBUTTONDBLCLK => println!("WM_NCXBUTTONDBLCLK"),
            WM_PAINTICON => println!("WM_PAINTICON"),
            WM_SETCURSOR => println!("WM_SETCURSOR"),
            WM_SETFOCUS => println!("WM_SETFOCUS"),
            WM_SETICON => println!("WM_SETICON"),
            WM_SETTEXT => println!("WM_SETTEXT"),
            WM_SHOWWINDOW => println!("WM_SHOWWINDOW"),
            WM_SYSCOMMAND => println!("WM_SYSCOMMAND"),
            WM_THEMECHANGED => println!("WM_THEMECHANGED"),
            WM_TIMER => println!("WM_TIMER"), // this goes off twice for some reason
            WM_WINDOWPOSCHANGED => println!("WM_WINDOWPOSCHANGED"),
            WM_WINDOWPOSCHANGING => println!("WM_WINDOWPOSCHANGING"),
            WM_MOUSEWHEEL => println!("WM_MOUSEWHEEL"),
            WM_MOUSEHWHEEL => println!("WM_MOUSEHWHEEL"),
            WM_MOUSEACTIVATE => println!("WM_MOUSEACTIVATE"),
            WM_INITDIALOG => println!("WM_INITDIALOG"),
            WM_DWMNCRENDERINGCHANGED if msg.wParam == WPARAM(1) => println!("WM_DWMNCRENDERINGCHANGED: {}", "on"),
            WM_DWMNCRENDERINGCHANGED if msg.wParam == WPARAM(0) => println!("WM_DWMNCRENDERINGCHANGED: {}", "off"),
            _ => {},
            // _ => {dbg!(msg);},
        }
        unsafe {
            let r= TranslateMessage(&msg).0;
            if r != 0 {
                println!("TranslateMessage returned {} for message: {:?}", r, &msg);
            }
            let r = DispatchMessageW(&msg).0;
            println!("{:03} DispatchMessageW returned {}", ctr_base-ctr, r);
        }
    }

    // std::thread::sleep(std::time::Duration::from_secs(2));

    Ok(())

}

fn load_image_as_bitmap(img_path: &str) -> (HGDIOBJ, i32, i32) {
    use image::GenericImageView;
    // Load the image using the `image` crate
    let img = image::open(img_path).expect("Failed to load image");
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

    // Return the bitmap handle
    (hbitmap.into(), width as i32, height as i32)
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CLOSE => {
            println!("WM_CLOSE received");
            DestroyWindow(hwnd).expect("DestroyWindow failed");
            return LRESULT(0);
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

