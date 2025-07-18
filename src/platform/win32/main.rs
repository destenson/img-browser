#![allow(non_snake_case, unused)]
use std::{borrow::Borrow, ffi::c_void, io::Read, mem::ManuallyDrop, ops::BitAnd, sync::{Arc, Mutex}};
use std::path::PathBuf;
use std::str::FromStr;
use gst::TaskHandle;
use windows::{
    core::*, Win32::{
        Foundation::*, Graphics::Gdi::*, System::{LibraryLoader::*, Threading::*},
        UI::{Controls::*, HiDpi::*, Input::KeyboardAndMouse::{GetKeyState, VK_CONTROL}, WindowsAndMessaging::*}
    },
};
use windows::Win32::UI::Shell::{DragAcceptFiles, DragFinish, DragQueryFileA, HDROP};
use crate::platform::win32::Window;
use crate::App;

// Constants for menu commands
const ID_FILE_OPEN: u16 = 101;
const ID_FOLDER_OPEN: u16 = 102;
const ID_FILE_EXIT: u16 = 103;

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

        // Create application menu
        let h_menu = CreateMenu()?;
        let h_file_menu = CreatePopupMenu()?;
        
        // Add menu items to File menu
        AppendMenuA(h_file_menu, MENU_ITEM_FLAGS(0), ID_FILE_OPEN as usize, s!("&Open File...\tCtrl+O"))?;
        AppendMenuA(h_file_menu, MENU_ITEM_FLAGS(0), ID_FOLDER_OPEN as usize, s!("Open &Folder...\tCtrl+F"))?;
        AppendMenuA(h_file_menu, MF_SEPARATOR, 0, None)?;
        AppendMenuA(h_file_menu, MENU_ITEM_FLAGS(0), ID_FILE_EXIT as usize, s!("E&xit"))?;
        
        // Add File menu to main menu
        AppendMenuA(h_menu, MF_POPUP, h_file_menu.0 as usize, s!("&File"))?;

        // Store app data as user data in the window
        let app_ptr = app as *mut App;
        
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
            Some(h_menu), // Set the menu for the window
            Some(instance.into()),
            Some(&window as *const _ as *const c_void),
        )?;

        if hwnd.is_invalid() {
            return Err(Error::from_win32());
        }
        
        // Set the window handle in our Window struct
        window.hwnd = hwnd;

        // Store window pointer in the window's user data
        SetWindowLongPtrA(window.hwnd, GWLP_USERDATA, &window as *const _ as isize);
        
        let hdata = HANDLE(app_ptr as *mut c_void);
        // Store the App pointer as a property of the window
        SetPropA(hwnd, s!("AppPtr"), Some(hdata));
        
        // enable drag-and-drop file support
        DragAcceptFiles(hwnd, true);
        
        log::info!("Window created successfully with handle: {:?}", window.hwnd.0);

        // Show window
        ShowWindow(window.hwnd, SW_SHOWNORMAL).ok()?;
        UpdateWindow(window.hwnd).ok()?;

        // Message loop
        let mut message = MSG::default();
        while GetMessageA(&mut message, None, 0, 0).into() {
            TranslateMessage(&message);
            DispatchMessageA(&message);
        }
    }
    Ok(())
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // Get the window pointer from window's user data
    // let window_ptr = GetWindowLongPtrA(hwnd, GWLP_USERDATA) as *const Window;
    let mut window_ptr = GetWindowLongPtrA(hwnd, GWLP_USERDATA) as *mut Window;
    
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
        
        WM_COMMAND => {
            // Handle menu commands
            let command_id = (wparam.0 & 0xFFFF) as u16;
            
            match command_id {
                ID_FILE_OPEN => {
                    handle_open_file(hwnd);
                    LRESULT(0)
                },
                ID_FOLDER_OPEN => {
                    handle_open_folder(hwnd);
                    LRESULT(0)
                },
                ID_FILE_EXIT => {
                    PostMessageA(Some(hwnd), WM_CLOSE, WPARAM(0), LPARAM(0));
                    LRESULT(0)
                },
                _ => DefWindowProcA(hwnd, message, wparam, lparam),
            }
        },
        
        WM_KEYDOWN => {
            // Handle keyboard shortcuts
            let virtual_key = wparam.0 as u16;
            let ctrl_pressed = GetKeyState(VK_CONTROL.0 as i32) < 0;
            
            if ctrl_pressed {
                match virtual_key as u8 as char {
                    'O' => {
                        handle_open_file(hwnd);
                        return LRESULT(0);
                    },
                    'F' => {
                        handle_open_folder(hwnd);
                        return LRESULT(0);
                    },
                    _ => {}
                }
            }
            
            DefWindowProcA(hwnd, message, wparam, lparam)
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
        
        WM_DROPFILES => {
            log::debug!("Files dropped onto window: {:?}", wparam);
            // Handle file drops
            let hdrop = HDROP(wparam.0 as *mut c_void);
            let mut file_count = 0;
            unsafe {
                file_count = DragQueryFileA(hdrop, 0xFFFFFFFF, None);
                if file_count > 0 {
                    for i in 0..file_count {
                        let mut file_name = vec![0u8; MAX_PATH as usize];
                        let len = DragQueryFileA(hdrop, i, Some(&mut file_name));
                        if len > 0 {
                            file_name.truncate(len as usize);
                            let path = String::from_utf8_lossy(&file_name).to_string();
                            log::info!("Dropped file: {}", path);

                            match load_image_from_path(&path, hwnd) {
                                Ok(filename) => {
                                    log::info!("Image loaded successfully: {}", path);
                                    let title = format!("Image Browser - {}", filename);
                                    SetWindowTextA(hwnd, PCSTR::from_raw(title.as_bytes().as_ptr()));

                                    // Load the image into the window
                                    let window_ptr = GetWindowLongPtrA(hwnd, GWLP_USERDATA) as *mut Window;
                                    if !window_ptr.is_null() {
                                        let window = &mut *window_ptr;
                                        let _ = window.load_image(&path);

                                        // Force a repaint
                                        InvalidateRect(Some(hwnd), None, false);
                                        UpdateWindow(hwnd);
                                    } else {
                                        log::error!("Failed to get window pointer from user data");
                                    }

                                },
                                Err(e) => {
                                    log::error!("Failed to load image: {}", e);

                                    // Show error message
                                    unsafe {
                                        let error_msg = format!("Failed to load image: {}", e);
                                        MessageBoxA(Some(hwnd), PCSTR(error_msg.as_ptr()), s!("Error"), MB_ICONERROR | MB_OK);
                                    }
                                }
                            }
                        }
                    }
                }
                DragFinish(hdrop);
            }
            LRESULT(0)
        }
        
        WM_SIZE => {
            // Update the window size in our Window struct
            if !window_ptr.is_null() {
                let window = unsafe { &mut *window_ptr };
                log::warn!("WM_SIZE (was {} x {})", window.width, window.height);

                // Handle window resizing
                let width = (lparam.0 & 0xffff) as u32 as i32;
                let height = (((lparam.0 >> 16) & 0xffff) as u32) as i32;
                log::info!("Window resized to {}x{}", width, height);
            
                window.width = width;
                window.height = height;
                
                // Force a repaint
                InvalidateRect(Some(hwnd), None, false);
                UpdateWindow(hwnd);
            }
            
            LRESULT(0)
        }
        
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


#[derive(Debug)]
enum LoadImageError {
    AppNotFoundError,
    ImageLoadError(image::ImageError),
    ApplicationError(crate::app::Error),
    InfallibleError,
}
impl std::fmt::Display for LoadImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadImageError::AppNotFoundError => write!(f, "Application not found for window"),
            LoadImageError::ImageLoadError(e) => write!(f, "Image load error: {}", e),
            LoadImageError::ApplicationError(e) => write!(f, "Application error: {}", e),
            LoadImageError::InfallibleError => write!(f, "Infallible error occurred"),
        }
    }
}
impl From<core::convert::Infallible> for LoadImageError {
    fn from(_: core::convert::Infallible) -> Self {
        LoadImageError::InfallibleError
    }
}

fn load_image_from_path<P: AsRef<std::path::Path>>(path: P, hwnd: HWND) -> std::result::Result<String, LoadImageError> {
    // Try to get image dimensions
    let (width, height) = image::image_dimensions(&path).map_err(LoadImageError::ImageLoadError)?;
    // Update the app state with the selected image
    unsafe {
        if let Some(app) = get_app_from_window(hwnd) {
            app.load_image_from_path(&path, (width, height)).map_err(LoadImageError::ApplicationError)?;
 
            // Update window title with the selected file
            let pb = path.as_ref().to_path_buf();
            let filename = pb.file_name().unwrap_or_default();
            let paths = filename.as_encoded_bytes().utf8_chunks().collect::<Vec<_>>();
            // log::trace!("path chunks: {:?}", paths);
            Ok(paths[0].valid().to_string())
        } else {
            Err(LoadImageError::AppNotFoundError)
        }
    }
}

// Debug options for controlling logging
struct DebugOpts {
    show_wm_paint: bool,
}

// Global debug options
static DBG_OPTS: DebugOpts = DebugOpts {
    show_wm_paint: cfg!(debug_assertions),
};

/// Handle WM_PAINT messages
pub fn wm_paint(hwnd: HWND, window: Option<&Window>) {
    if DBG_OPTS.show_wm_paint {
        log::trace!("wm_paint: hWND: {:08p}", hwnd.0);
    }
    
    let mut ps: PAINTSTRUCT = unsafe { std::mem::zeroed() };
    let hdc = unsafe { BeginPaint(hwnd, &mut ps) };
    
    if let Some(window) = window {
        if !window.hbitmap.is_invalid() {
            let hdc_mem = unsafe { CreateCompatibleDC(Some(hdc)) };
            let prev_bmp = unsafe { SelectObject(hdc_mem, window.hbitmap.into()) };

            let bres = unsafe {
                BitBlt(hdc, 0, 0, window.width, window.height, Some(hdc_mem), 0, 0, SRCCOPY)
            };
            
            if bres.is_err() {
                log::error!("BitBlt failed: {}", unsafe { GetLastError().0 });
            }

            unsafe { SelectObject(hdc_mem, prev_bmp) };
            unsafe { DeleteDC(hdc_mem) }.expect("DeleteDC failed");
        }
    } else {
        // No window data available, just validate the rect
        unsafe { ValidateRect(Some(hwnd), None) };
    }
    
    unsafe { EndPaint(hwnd, &ps) }.expect("EndPaint failed");
}

/// Helper function to get the app from window's user data
pub(crate) unsafe fn get_app_from_window(hwnd: HWND) -> Option<&'static mut App> {
    let app_ptr = GetWindowLongPtrA(hwnd, GWLP_USERDATA) as *mut c_void;
    if app_ptr.is_null() {
        return None;
    }
    
    // Get the Window struct from user data
    let window_ptr = app_ptr as *const Window;
    if window_ptr.is_null() {
        return None;
    }
    
    // Find the app pointer in the window's properties
    let app_ptr = GetPropA(hwnd, s!("AppPtr")).0 as *mut App;
    if app_ptr.is_null() {
        return None;
    }
    
    Some(&mut *app_ptr)
}

/// Handle opening a file
fn handle_open_file(hwnd: HWND) {
    log::info!("Opening file dialog");
    
    // Use the open_file_dialog function we created
    let result = super::open_file_dialog(
        hwnd, 
        "Open Image File", 
        "Image Files", 
        "*.jpg;*.jpeg;*.png;*.gif;*.bmp;*.webp"
    );
    
    match result {
        Ok(Some(path)) => {
            log::info!("Selected file: {}", path.display());

            match load_image_from_path(&path, hwnd) {
                Ok(filename) => {
                    log::info!("Image loaded successfully: {}", path.display());
                    let title = format!("Image Browser - {}", filename);
                    unsafe { SetWindowTextA(hwnd, PCSTR::from_raw(title.as_bytes().as_ptr())) };

                    // Load the image into the window
                    let window_ptr = unsafe { GetWindowLongPtrA(hwnd, GWLP_USERDATA) as *mut Window };
                    if !window_ptr.is_null() {
                        let window = unsafe { &mut *window_ptr };
                        let _ = window.load_image(path);

                        // Force a repaint
                        unsafe {
                            InvalidateRect(Some(hwnd), None, false);
                            UpdateWindow(hwnd);
                        }
                    } else {
                        log::error!("Failed to get window pointer from user data");
                    }

                },
                Err(e) => {
                    log::error!("Failed to load image: {}", e);

                    // Show error message
                    unsafe {
                        let error_msg = format!("Failed to load image: {}", e);
                        MessageBoxA(Some(hwnd), PCSTR(error_msg.as_ptr()), s!("Error"), MB_ICONERROR | MB_OK);
                    }
                }
            }

        },
        Ok(None) => {
            log::info!("File dialog cancelled");
        },
        Err(e) => {
            log::error!("Error opening file dialog: {:?}", e);
            
            // Show error message
            unsafe {
                let error_msg = format!("Error opening file dialog: {:?}", e);
                MessageBoxA(Some(hwnd), PCSTR(error_msg.as_ptr()), s!("Error"), MB_ICONERROR | MB_OK);
            }
        }
    }
}

/// Handle opening a folder
fn handle_open_folder(hwnd: HWND) {
    log::info!("Opening folder dialog");
    
    // Use the open_folder_dialog function we created
    let result = super::open_folder_dialog(hwnd, "Select Image Folder");
    
    match result {
        Ok(Some(path)) => {
            log::info!("Selected folder: {}", path.display());
            
            // Update the app state with the selected folder
            unsafe {
                if let Some(app) = get_app_from_window(hwnd) {
                    match app.state.set_current_directory(&path) {
                        Ok(_) => {
                            // TODO: fix the string conversion (sometimes the window title is corrupt especially at the end)
                            // Update window title with the selected folder
                            let filename = path.file_name().unwrap_or_default();
                            let paths = filename.as_encoded_bytes().utf8_chunks().collect::<Vec<_>>();
                            println!("path chunks: {:?}", paths);
                            let title = format!("Image Browser - {}", paths[0].valid());
                            // let title = format!("Image Browser - {}", path.file_name().unwrap().to_str().unwrap());
                            SetWindowTextA(hwnd, PCSTR::from_raw(title.as_bytes().as_ptr()));
                            
                            // If the app is configured for recursive scanning
                            if app.config.recursive {
                                match app.state.update_media_db_for_current_directory(true) {
                                    Ok(items_scanned) => {
                                        if items_scanned == 0 {
                                            log::warn!("No images found in directory");
                                        } else {
                                            log::info!("Scanned {} items in directory", items_scanned);
                                            app.state.save_media_db(&app.config).inspect_err(|e| {
                                                log::error!("Failed to save media database: {}", e);
                                            }).expect("Failed to save media database");
                                        }
                                        
                                    },
                                    Err(e) =>log::error!("Failed to scan directory: {}", e)
                                }
                            }
                            
                            // Force a repaint to show the directory contents
                            InvalidateRect(Some(hwnd), None, false);
                            UpdateWindow(hwnd);
                        },
                        Err(e) => {
                            log::error!("Failed to set directory: {}", e);
                            
                            // Show error message
                            let error_msg = format!("Failed to set directory: {}", e);
                            MessageBoxA(Some(hwnd), PCSTR(error_msg.as_ptr()), s!("Error"), MB_ICONERROR | MB_OK);
                        }
                    }
                }
            }
        },
        Ok(None) => {
            log::info!("Folder dialog cancelled");
        },
        Err(e) => {
            log::error!("Error opening folder dialog: {:?}", e);
            
            // Show error message
            unsafe {
                let error_msg = format!("Error opening folder dialog: {:?}", e);
                MessageBoxA(Some(hwnd), PCSTR(error_msg.as_ptr()), s!("Error"), MB_ICONERROR | MB_OK);
            }
        }
    }
}
