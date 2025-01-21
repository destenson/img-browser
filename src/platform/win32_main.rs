#![allow(non_snake_case, unused)]
use std::{io::Read, ops::BitAnd, sync::{Arc, Mutex}};

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

pub fn main() -> Result<()> {

    unsafe {SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE)}.expect("Failed to set process DPI awareness");
    let handle_instance = unsafe {GetModuleHandleW(None)}.expect("Failed to get module handle");

    let wc = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(window_proc),
        hInstance: handle_instance.into(),
        // hCursor: None,
        // hbrBackground: None,
        lpszClassName: w!("img-browser-rs"),
        ..Default::default()
    };

    if unsafe {RegisterClassExW(&wc)} == 0 {
        panic!("Failed to register window class.");
    }

    let (image_bmp, width, height) = load_image_as_bitmap(image_path);
    println!("Image dimensions: {}x{}", width, height);

    let dwexstyle = WS_EX_TOPMOST
        | WS_EX_WINDOWEDGE
        | WS_EX_LEFT
        | WS_EX_ACCEPTFILES
        | WS_EX_APPWINDOW;
    // WS_EX_LEFT | WS_EX_OVERLAPPEDWINDOW | WS_EX_WINDOWEDGE,// | WS_EX_LAYERED,
    // WS_EX_LEFT | WS_EX_OVERLAPPEDWINDOW | WS_EX_LAYERED | WS_EX_WINDOWEDGE,

    let window = unsafe {
        CreateWindowExW(dwexstyle,
            w!("STATIC"),
            w!("Image Viewer"),
            WS_OVERLAPPEDWINDOW,
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

    if window.is_invalid() {
        panic!("Failed to create window");
    }

    // let transparency = 255;
    // unsafe {SetLayeredWindowAttributes(window, COLORREF(0), transparency, LWA_ALPHA)}.expect("Failed to set layered window attributes");
    
    let was_hidden = unsafe {ShowWindow(window, SW_SHOWNORMAL)}.0 == 0;
    assert!(was_hidden, "Failed to show window");
    
    // let _ = unsafe {UpdateWindow(window)}.expect("Failed to update window"); // draw never gets called if this is uncommented
    
    // let mut msg = unsafe {std::mem::zeroed()};
    // while unsafe{GetMessageW(&mut msg, HWND(std::ptr::null_mut()), 0, 0)}.into() {
    //     let _ = unsafe{TranslateMessage(&msg)};
    //     unsafe{DispatchMessageW(&msg)};
    // }
    
    // panic!();

    // let ctr_base = 51;
    let ctr_base = 16;
    let mut ctr = ctr_base;
    loop {
        ctr -= 1;
        if ctr == 0 {
            break;
        }
        let mut msg = MSG::default();
        let r = unsafe { GetMessageW(&mut msg, None, 0, 0) };
        r.expect("GetMessageW failed");
        if r.0 == 0 {
            break;
        }
        // match msg.message.bitand(0xffff) {
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

                // // a. create a device context for the window
                // let hdc = unsafe { GetDC(window) };
                // if hdc.is_invalid() {
                //     panic!("Failed to get device context");
                // }
                // defer!({
                //     let r = unsafe {ReleaseDC(window, hdc)};
                //     if r == 0 {
                //         panic!("Failed to release device context");
                //     }
                // });

                // resize the window
                let r = unsafe {SetWindowPos(window, HWND_TOP, 0, 0, width, height, SWP_NOMOVE | SWP_NOZORDER)};
                if r.is_err() {
                    panic!("Failed to set window position");
                }
            },
            WM_PAINT => {
                print!("{:03} WM_PAINT ", ctr_base-ctr);
                println!("{} {:?} {:?} {:?}", msg.time, msg.pt, msg.lParam, msg.wParam);
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
                    // println!("DeleteDC called");
                }
    
                unsafe{EndPaint(window, &ps)}.expect("EndPaint failed");
                // println!("EndPaint called");

                // // TODO: do we continue, or fall through here?
                // return LRESULT(0);
                // continue;
            },
    
            WM_TIMER => {
                print!("{:03} WM_TIMER ", ctr_base-ctr);
                println!("{} {:?} {:?} {:?}", msg.time, msg.pt, msg.lParam, msg.wParam);
                // dbg!(msg);
                // continue;
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
            WM_NCMOUSEMOVE => { // this seeems to be the one that triggers when the mouse moves over the window
                print!("{:03} WM_NCMOUSEMOVE ", ctr_base-ctr);
                println!("{} {:?} {:?} {:?}", msg.time, msg.pt, msg.lParam, msg.wParam);
            },
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
            WM_WINDOWPOSCHANGED => println!("WM_WINDOWPOSCHANGED"),
            WM_WINDOWPOSCHANGING => println!("WM_WINDOWPOSCHANGING"),
            WM_MOUSEWHEEL => println!("WM_MOUSEWHEEL"),
            WM_MOUSEHWHEEL => println!("WM_MOUSEHWHEEL"),
            WM_MOUSEACTIVATE => println!("WM_MOUSEACTIVATE"),
            WM_INITDIALOG => println!("WM_INITDIALOG"),
            WM_DWMNCRENDERINGCHANGED if msg.wParam == WPARAM(1) => {
                print!("{:03} WM_DWMNCRENDERINGCHANGED: {} ", ctr_base-ctr, " on");
                println!("{} {:?} {:?} {:?}", msg.time, msg.pt, msg.lParam, msg.wParam);
                // continue;
            },
            WM_DWMNCRENDERINGCHANGED if msg.wParam == WPARAM(0) => {
                print!("{:03} WM_DWMNCRENDERINGCHANGED: {} ", ctr_base-ctr, "off");
                println!("{} {:?} {:?} {:?}", msg.time, msg.pt, msg.lParam, msg.wParam);
                // continue;
            },

            WM_ACTIVATE => println!("WM_ACTIVATE"),
            WM_ACTIVATEAPP => println!("WM_ACTIVATEAPP"),
            WM_AFXFIRST => println!("WM_AFXFIRST"),
            WM_AFXLAST => println!("WM_AFXLAST"),
            WM_APP => println!("WM_APP"),
            WM_APPCOMMAND => println!("WM_APPCOMMAND"),
            WM_ASKCBFORMATNAME => println!(""),
            WM_CANCELJOURNAL => println!(""),
            WM_CANCELMODE => println!(""),
            WM_CAPTURECHANGED => println!(""),
            WM_CHANGECBCHAIN => println!(""),
            WM_CHANGEUISTATE => println!(""),
            WM_CHAR => println!(""),
            WM_CHARTOITEM => println!(""),
            WM_CHILDACTIVATE => println!(""),
            WM_CLEAR => println!(""),
            WM_CLIPBOARDUPDATE => println!(""),
            WM_CLOSE => println!(""),
            WM_COMMAND => println!(""),
            WM_COMMNOTIFY => println!(""),
            WM_COMPACTING => println!(""),
            WM_COMPAREITEM => println!(""),
            WM_CONTEXTMENU => println!(""),
            WM_COPY => println!(""),
            WM_COPYDATA => println!(""),
            WM_CREATE => println!(""),
            WM_CTLCOLORBTN => println!(""),
            WM_CTLCOLORDLG => println!(""),
            WM_CTLCOLOREDIT => println!(""),
            WM_CTLCOLORLISTBOX => println!(""),
            WM_CTLCOLORMSGBOX => println!(""),
            WM_CTLCOLORSCROLLBAR => println!(""),
            WM_CTLCOLORSTATIC => println!(""),
            WM_CUT => println!(""),
            WM_DEADCHAR => println!(""),
            WM_DELETEITEM => println!(""),
            WM_DESTROY => println!(""),
            WM_DESTROYCLIPBOARD => println!(""),
            WM_DEVICECHANGE => println!(""),
            WM_DEVMODECHANGE => println!(""),
            WM_DISPLAYCHANGE => println!(""),
            WM_DPICHANGED => println!(""),
            WM_DPICHANGED_AFTERPARENT => println!(""),
            WM_DPICHANGED_BEFOREPARENT => println!(""),
            WM_DRAWCLIPBOARD => println!(""),
            WM_DRAWITEM => println!(""),
            WM_DROPFILES => println!(""),
            WM_DWMCOLORIZATIONCOLORCHANGED => println!(""),
            WM_DWMCOMPOSITIONCHANGED => println!(""),
            WM_DWMNCRENDERINGCHANGED => println!(""),
            WM_DWMSENDICONICLIVEPREVIEWBITMAP => println!(""),
            WM_DWMSENDICONICTHUMBNAIL => println!(""),
            WM_DWMWINDOWMAXIMIZEDCHANGE => println!(""),
            WM_ENABLE => println!(""),
            WM_ENDSESSION => println!(""),
            WM_ENTERIDLE => println!(""),
            WM_ENTERMENULOOP => println!(""),
            WM_ENTERSIZEMOVE => println!(""),
            WM_ERASEBKGND => println!(""),
            WM_EXITMENULOOP => println!(""),
            WM_EXITSIZEMOVE => println!(""),
            WM_FONTCHANGE => println!(""),
            WM_GESTURE => println!(""),
            WM_GESTURENOTIFY => println!(""),
            WM_GETDLGCODE => println!(""),
            WM_GETDPISCALEDSIZE => println!(""),
            WM_GETFONT => println!(""),
            WM_GETHOTKEY => println!(""),
            WM_GETICON => println!(""),
            WM_GETMINMAXINFO => println!(""),
            WM_GETOBJECT => println!(""),
            WM_GETTEXT => println!(""),
            WM_GETTEXTLENGTH => println!(""),
            WM_GETTITLEBARINFOEX => println!(""),
            WM_HANDHELDFIRST => println!(""),
            WM_HANDHELDLAST => println!(""),
            WM_HELP => println!(""),
            WM_HOTKEY => println!(""),
            WM_HSCROLL => println!(""),
            WM_HSCROLLCLIPBOARD => println!(""),
            WM_ICONERASEBKGND => println!(""),
            WM_IME_CHAR => println!(""),
            WM_IME_COMPOSITION => println!(""),
            WM_IME_COMPOSITIONFULL => println!(""),
            WM_IME_CONTROL => println!(""),
            WM_IME_ENDCOMPOSITION => println!(""),
            WM_IME_KEYDOWN => println!(""),
            WM_IME_KEYLAST => println!(""),
            WM_IME_KEYUP => println!(""),
            WM_IME_NOTIFY => println!(""),
            WM_IME_REQUEST => println!(""),
            WM_IME_SELECT => println!(""),
            WM_IME_SETCONTEXT => println!(""),
            WM_IME_STARTCOMPOSITION => println!(""),
            WM_INITDIALOG => println!(""),
            WM_INITMENU => println!(""),
            WM_INITMENUPOPUP => println!(""),
            WM_INPUT => println!(""),
            WM_INPUTLANGCHANGE => println!(""),
            WM_INPUTLANGCHANGEREQUEST => println!(""),
            WM_INPUT_DEVICE_CHANGE => println!(""),
            WM_KEYDOWN => println!(""),
            WM_KEYFIRST => println!(""),
            WM_KEYLAST => println!(""),
            WM_KEYUP => println!(""),
            WM_KILLFOCUS => println!(""),
            WM_LBUTTONDBLCLK => println!(""),
            WM_LBUTTONDOWN => println!(""),
            WM_LBUTTONUP => println!(""),
            WM_MBUTTONDBLCLK => println!(""),
            WM_MBUTTONDOWN => println!(""),
            WM_MBUTTONUP => println!(""),
            WM_MDIACTIVATE => println!(""),
            WM_MDICASCADE => println!(""),
            WM_MDICREATE => println!(""),
            WM_MDIDESTROY => println!(""),
            WM_MDIGETACTIVE => println!(""),
            WM_MDIICONARRANGE => println!(""),
            WM_MDIMAXIMIZE => println!(""),
            WM_MDINEXT => println!(""),
            WM_MDIREFRESHMENU => println!(""),
            WM_MDIRESTORE => println!(""),
            WM_MDISETMENU => println!(""),
            WM_MDITILE => println!(""),
            WM_MEASUREITEM => println!(""),
            WM_MENUCHAR => println!(""),
            WM_MENUCOMMAND => println!(""),
            WM_MENUDRAG => println!(""),
            WM_MENUGETOBJECT => println!(""),
            WM_MENURBUTTONUP => println!(""),
            WM_MENUSELECT => println!(""),
            WM_MOUSEACTIVATE => println!(""),
            WM_MOUSEFIRST => println!(""),
            WM_MOUSEHWHEEL => println!(""),
            WM_MOUSELAST => println!(""),
            WM_MOUSEMOVE => println!(""),
            WM_MOUSEWHEEL => println!(""),
            WM_MOVE => println!(""),
            WM_MOVING => println!(""),
            WM_NCACTIVATE => println!(""),
            WM_NCCALCSIZE => println!(""),
            WM_NCCREATE => println!(""),
            WM_NCDESTROY => println!(""),
            WM_NCHITTEST => println!(""),
            WM_NCLBUTTONDBLCLK => println!(""),
            WM_NCLBUTTONDOWN => println!(""),
            WM_NCLBUTTONUP => println!(""),
            WM_NCMBUTTONDBLCLK => println!(""),
            WM_NCMBUTTONDOWN => println!(""),
            WM_NCMBUTTONUP => println!(""),
            WM_NCMOUSEHOVER => println!(""),
            WM_NCMOUSELEAVE => println!(""),
            WM_NCMOUSEMOVE => println!(""),
            WM_NCPAINT => println!(""),
            WM_NCPOINTERDOWN => println!(""),
            WM_NCPOINTERUP => println!(""),
            WM_NCPOINTERUPDATE => println!(""),
            WM_NCRBUTTONDBLCLK => println!(""),
            WM_NCRBUTTONDOWN => println!(""),
            WM_NCRBUTTONUP => println!(""),
            WM_NCXBUTTONDBLCLK => println!(""),
            WM_NCXBUTTONDOWN => println!(""),
            WM_NCXBUTTONUP => println!(""),
            WM_NEXTDLGCTL => println!(""),
            WM_NEXTMENU => println!(""),
            WM_NOTIFY => println!(""),
            WM_NOTIFYFORMAT => println!(""),
            WM_NULL => println!(""),
            WM_PAINT => println!(""),
            WM_PAINTCLIPBOARD => println!(""),
            WM_PAINTICON => println!(""),
            WM_PALETTECHANGED => println!(""),
            WM_PALETTEISCHANGING => println!(""),
            WM_PARENTNOTIFY => println!(""),
            WM_PASTE => println!(""),
            WM_PENWINFIRST => println!(""),
            WM_PENWINLAST => println!(""),
            WM_POINTERACTIVATE => println!(""),
            WM_POINTERCAPTURECHANGED => println!(""),
            WM_POINTERDEVICECHANGE => println!(""),
            WM_POINTERDEVICEINRANGE => println!(""),
            WM_POINTERDEVICEOUTOFRANGE => println!(""),
            WM_POINTERDOWN => println!(""),
            WM_POINTERENTER => println!(""),
            WM_POINTERHWHEEL => println!(""),
            WM_POINTERLEAVE => println!(""),
            WM_POINTERROUTEDAWAY => println!(""),
            WM_POINTERROUTEDRELEASED => println!(""),
            WM_POINTERROUTEDTO => println!(""),
            WM_POINTERUP => println!(""),
            WM_POINTERUPDATE => println!(""),
            WM_POINTERWHEEL => println!(""),
            WM_POWER => println!(""),
            WM_POWERBROADCAST => println!(""),
            WM_PRINT => println!(""),
            WM_PRINTCLIENT => println!(""),
            WM_QUERYDRAGICON => println!(""),
            WM_QUERYENDSESSION => println!(""),
            WM_QUERYNEWPALETTE => println!(""),
            WM_QUERYOPEN => println!(""),
            WM_QUERYUISTATE => println!(""),
            WM_QUEUESYNC => println!(""),
            WM_QUIT => println!(""),
            WM_RBUTTONDBLCLK => println!(""),
            WM_RBUTTONDOWN => println!(""),
            WM_RBUTTONUP => println!(""),
            WM_RENDERALLFORMATS => println!(""),
            WM_RENDERFORMAT => println!(""),
            WM_SETCURSOR => println!(""),
            WM_SETFOCUS => println!(""),
            WM_SETFONT => println!(""),
            WM_SETHOTKEY => println!(""),
            WM_SETICON => println!(""),
            WM_SETREDRAW => println!(""),
            WM_SETTEXT => println!(""),
            WM_SETTINGCHANGE => println!(""),
            WM_SHOWWINDOW => println!(""),
            WM_SIZE => println!(""),
            WM_SIZECLIPBOARD => println!(""),
            WM_SIZING => println!(""),
            WM_SPOOLERSTATUS => println!(""),
            WM_STYLECHANGED => println!(""),
            WM_STYLECHANGING => println!(""),
            WM_SYNCPAINT => println!(""),
            WM_SYSCHAR => println!(""),
            WM_SYSCOLORCHANGE => println!(""),
            WM_SYSCOMMAND => println!(""),
            WM_SYSDEADCHAR => println!(""),
            WM_SYSKEYDOWN => println!(""),
            WM_SYSKEYUP => println!(""),
            WM_TABLET_FIRST => println!(""),
            WM_TABLET_LAST => println!(""),
            WM_TCARD => println!(""),
            WM_THEMECHANGED => println!(""),
            WM_TIMECHANGE => println!(""),
            WM_TIMER => println!(""),
            WM_TOOLTIPDISMISS => println!(""),
            WM_TOUCH => println!(""),
            WM_TOUCHHITTESTING => println!(""),
            WM_UNDO => println!(""),
            WM_UNICHAR => println!(""),
            WM_UNINITMENUPOPUP => println!(""),
            WM_UPDATEUISTATE => println!(""),
            WM_USER => println!(""),
            WM_USERCHANGED => println!(""),
            WM_VKEYTOITEM => println!(""),
            WM_VSCROLL => println!(""),
            WM_VSCROLLCLIPBOARD => println!(""),
            WM_WINDOWPOSCHANGED => println!(""),
            WM_WINDOWPOSCHANGING => println!(""),
            WM_WININICHANGE => println!(""),
            WM_WTSSESSION_CHANGE => println!(""),
            WM_XBUTTONDBLCLK => println!(""),
            WM_XBUTTONDOWN => println!(""),
            WM_XBUTTONUP => println!(""),
            49306 => {
                print!("{:03} WM_UNDOCUMENTED_STARTUP_MSG ", ctr_base-ctr);
                println!("{} {:?} {:?} {:?}", msg.time, msg.pt, msg.lParam, msg.wParam);
                // dbg!(msg);
                // dbg!((msg.time, msg.pt, msg.lParam.0, msg.wParam.0));
                // continue;
            },
            96 if (msg.lParam.0, msg.wParam.0) == (0, 1) => {
                print!("{:03} WM_UNDOCUMENTED_STARTUP_MSG_2:0:1 ", ctr_base-ctr);
                println!("{} {:?} {:?} {:?}", msg.time, msg.pt, msg.lParam, msg.wParam);
                // dbg!((msg.time, msg.pt, msg.lParam.0, msg.wParam.0));
                // continue;
            },
            96 if (msg.lParam.0, msg.wParam.0) == (0, 6) => {
                print!("{:03} WM_UNDOCUMENTED_STARTUP_MSG_2:0:6 ", ctr_base-ctr);
                println!("{} {:?} {:?} {:?}", msg.time, msg.pt, msg.lParam, msg.wParam);
                // dbg!((msg.time, msg.pt, msg.lParam.0, msg.wParam.0));
                // continue;
            },
            96 => {
                print!("{:03} WM_UNDOCUMENTED_STARTUP_MSG_2 ", ctr_base-ctr);
                println!("{} {:?} {:?} {:?}", msg.time, msg.pt, msg.lParam, msg.wParam);
                // continue;
            },
            // _ => {},
            _ => {dbg!(msg);},
        }
        
        // process input messages by translating virtual keys into characters
        let r= unsafe {TranslateMessage(&msg)}.0;
        if r != 0 {
            println!("TranslateMessage returned {} for message: {:?}", r, &msg);
        } else {
            match msg.message {
                49306 => {},
                96 => {},
                WM_TIMER | WM_DWMNCRENDERINGCHANGED | WM_PAINT => {},
                _ => {
                    println!("No character message for TranslateMessage");
                    // println!("{:03} TranslateMessage returned {} for message: {:?}", ctr_base-ctr, r, &msg);
                },
            }
        }
        let r = unsafe {DispatchMessageW(&msg)}.0;
        match msg.message {
            49306 => {},
            96 => {},
            WM_TIMER | WM_DWMNCRENDERINGCHANGED | WM_PAINT if r == 0 => {},
            _ => {
                println!("{:03} DispatchMessageW returned {}", ctr_base-ctr, r);
            },
        }
    }

    unsafe { PostQuitMessage(0) };

    std::thread::sleep(std::time::Duration::from_secs(2));

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
    log::info!("Message {:?} received", msg);
    println!("Message {:?} received", msg);
    match msg {
        WM_CLOSE => {
            println!("WM_CLOSE received");
            DestroyWindow(hwnd).expect("DestroyWindow failed");
            return LRESULT(0);
        }
        _ => {
            println!("Message {:?} received", msg);
            DefWindowProcW(hwnd, msg, wparam, lparam)
        },
    }
}

pub trait WndProc {
    // unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    //     DefWindowProcW(hwnd, msg, wparam, lparam)
    // }
    unsafe extern "system" fn wnd_proc(&self,
        hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}

impl<T> WndProc for T where T: Fn(HWND, u32, WPARAM, LPARAM) -> LRESULT {}

impl WndProc for WNDPROC {
    unsafe extern "system" fn wnd_proc(&self, hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match self {
            Some(wnd_proc) => wnd_proc(hwnd, msg, wparam, lparam),
            None => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}


