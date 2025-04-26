#![allow(non_snake_case, unused)]
use std::{borrow::Borrow, ffi::c_void, io::Read, mem::ManuallyDrop, ops::BitAnd, sync::{Arc, Mutex}};

use windows::{
    core::*, Win32::{
        Foundation::*, Graphics::Gdi::*, System::{LibraryLoader::*, Threading::*},
        UI::{Controls::*, HiDpi::*, WindowsAndMessaging::*}
    },
};

use crate::platform::win32::Window;
use crate::App;

pub fn run_window_loop(mut window: Window, app: &mut App) -> windows::core::Result<()> {
    unsafe {
        // Register window class
        let instance = GetModuleHandleA(None)?;
        let window_class = s!("img-browser-window-class");

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance.into(),
            lpszClassName: window_class,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        if atom == 0 {
            return Err(Error::from_win32());
        }

        // Store app data as user data in the window
        let app_ptr = app as *mut App as *mut c_void;
        
        // Create window
        let hwnd = CreateWindowExA(
            WS_EX_OVERLAPPEDWINDOW,
            window_class,
            s!("Image Browser"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            window.width,
            window.height,
            None,
            None,
            instance,
            Some(&window as *const _ as *const c_void),
        )?;

        if hwnd.is_invalid() {
            return Err(Error::from_win32());
        }
        
        // Set the window handle in our Window struct
        window.hwnd = hwnd;

        // Store window pointer in the window's user data
        SetWindowLongPtrA(window.hwnd, GWLP_USERDATA, &window as *const _ as isize);
        
        log::info!("Window created successfully with handle: {:?}", window.hwnd.0);

        // Show window
        ShowWindow(window.hwnd, SW_SHOWNORMAL).ok()?;
        UpdateWindow(window.hwnd).ok()?;

        // Message loop
        let mut message = MSG::default();
        while GetMessageA(&mut message, HWND::default(), 0, 0).into() {
            TranslateMessage(&message);
            DispatchMessageA(&message);
        }

        Ok(())
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // Get the window pointer from window's user data
    let window_ptr = GetWindowLongPtrA(hwnd, GWLP_USERDATA) as *const Window;
    
    match message {
        WM_CREATE => {
            // The window is being created
            let create_struct = lparam.0 as *const CREATESTRUCTA;
            if !create_struct.is_null() {
                // Store the window pointer in the window's user data
                let window_ptr = (*create_struct).lpCreateParams as *const Window;
                if !window_ptr.is_null() {
                    SetWindowLongPtrA(hwnd, GWLP_USERDATA, window_ptr as isize);
                }
            }
            LRESULT(0)
        },
        
        WM_PAINT => {
            // Paint the window
            let window = if !window_ptr.is_null() {
                Some(&*window_ptr)
            } else {
                None
            };
            
            // Call our paint function
            wm_paint(hwnd, window);
            LRESULT(0)
        },
        
        WM_CLOSE => {
            // Close the window
            DestroyWindow(hwnd).expect("Failed to destroy window");
            LRESULT(0)
        },
        
        WM_DESTROY => {
            // Post quit message to exit message loop
            PostQuitMessage(0);
            LRESULT(0)
        },
        
        _ => DefWindowProcA(hwnd, message, wparam, lparam),
    }
}

// Debug options for controlling logging
struct DebugOpts {
    show_wm_paint: bool,
}

// Global debug options
static DBG_OPTS: DebugOpts = DebugOpts {
    show_wm_paint: false,
};

/// Handle WM_PAINT messages
pub fn wm_paint(hwnd: HWND, window: Option<&Window>) {
    if DBG_OPTS.show_wm_paint {
        log::trace!("wm_paint: hWND: 0x{:08p}", hwnd.0);
    }
    
    let mut ps: PAINTSTRUCT = unsafe { std::mem::zeroed() };
    let hdc = unsafe { BeginPaint(hwnd, &mut ps) };
    
    if let Some(window) = window {
        if !window.hbitmap.is_invalid() {
            let hdc_mem = unsafe { CreateCompatibleDC(hdc) };
            let prev_bmp = unsafe { SelectObject(hdc_mem, window.hbitmap) };

            let bres = unsafe {
                BitBlt(hdc, 0, 0, window.width, window.height, hdc_mem, 0, 0, SRCCOPY)
            };
            
            if bres.is_err() {
                log::error!("BitBlt failed: {}", unsafe { GetLastError().0 });
            }

            unsafe { SelectObject(hdc_mem, prev_bmp) };
            unsafe { DeleteDC(hdc_mem) }.expect("DeleteDC failed");
        }
    } else {
        // No window data available, just validate the rect
        unsafe { ValidateRect(hwnd, None) };
    }
    
    unsafe { EndPaint(hwnd, &ps) }.expect("EndPaint failed");
}
