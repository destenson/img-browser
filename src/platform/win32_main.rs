#![allow(non_snake_case, unused)]
use std::{borrow::Borrow, ffi::c_void, io::Read, ops::BitAnd, sync::{Arc, Mutex}};

use windows::{
    core::*, Data::Xml::Dom::*,
    Win32::{
        Foundation::*, Graphics::Gdi::*,
        System::{LibraryLoader::*, Threading::*},
        UI::{Controls::*, HiDpi::*, Input::Ime::*, WindowsAndMessaging::*}
    },
};

use image::DynamicImage;

use defer::defer;

struct DbgOpts {
    pub show_wm_getminmaxinfo: bool,
    pub show_wm_nccreate: bool,
    pub show_wm_nccalcsize: bool,
    pub show_wm_create: bool,
    pub show_wm_showwindow: bool,
    pub show_wm_windowposchanging: bool,
    pub show_wm_activateapp: bool,
    pub show_wm_ncactivate: bool,
    pub show_wm_geticon: bool,
    pub show_wm_activate: bool,
    pub show_wm_ime_setcontext: bool,
    pub show_wm_ime_notify: bool,
    pub show_wmsz_bottomleft: bool,
    pub show_wm_ncpaint: bool,
    pub show_wm_erasebkgnd: bool,
    pub show_wm_chartoitem: bool,

    // pub show_title_bar: bool,
    // pub show_fps: bool,
    // pub show_mouse: bool,
    // pub show_keyboard: bool,
    // pub show_touch: bool,
    // pub show_gamepad: bool,
    // pub show_joystick: bool,
    // pub show_controller: bool,
    // pub show_pointer: bool,
    // pub show_gesture: bool,
    // pub show_touchpad: bool,
    // pub show_stylus: bool,
    // pub show_pen: bool,
    // pub show_ink: bool,
  
    image_path: &'static str,
}

const DBG_OPTS: DbgOpts = DbgOpts{
    show_wm_getminmaxinfo: false,
    show_wm_nccreate: false,
    show_wm_nccalcsize: false,
    show_wm_create: false,
    show_wm_showwindow: false,
    show_wm_windowposchanging: false,
    show_wm_activateapp: false,
    show_wm_ncactivate: false,
    show_wm_geticon: false,
    show_wm_activate: false,
    show_wm_ime_setcontext: false,
    show_wm_ime_notify: false,
    show_wmsz_bottomleft: false,
    show_wm_ncpaint: false,
    show_wm_erasebkgnd: false,
    show_wm_chartoitem: true,

    image_path: "vendor/oculante/res/screenshot_exif.png",
};

// const image_path: &str = "vendor/oculante/res/screenshot_exif.png";
const image_path: &str = DBG_OPTS.image_path;

pub fn main() -> Result<()> {
    use WndProc;

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


    let class_atom = unsafe {RegisterClassExW(&wc)};
    if class_atom == 0 {
        panic!("Failed to register window class.");
    }
    let class_atom: PCWSTR = unsafe {
        std::mem::transmute(class_atom as usize)
    };

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
            class_atom,
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

            WM_ACTIVATE => println!("WM_ACTIVATE: {} {:?}", msg.time, msg),
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
                continue;
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
    wParam: WPARAM,
    lParam: LPARAM,
) -> LRESULT {
    let print_msg = |str_msg: &str| {
        let msg = match str_msg {
            "" => format!("0x{:04x} {}", msg, msg),
            m => format!("{}", m),
        };
        // println!("[{:x?}] {}, {}, {}", hwnd.0, msg, wParam.0, lParam.0);
        log::trace!("[{:x?}] {}, {}, {}", hwnd.0, msg, wParam.0, lParam.0);
    };
    // log::info!("Message {:?} received", msg);
    // println!("Message {:?} received", msg);
    match msg {
        WM_CLOSE => {
            print_msg("WM_CLOSE");
            DestroyWindow(hwnd).expect("DestroyWindow failed");
            return LRESULT(0);
        }

        WM_GETMINMAXINFO => {
            match wm_getminmaxinfo(hwnd, lParam) {
                Ok(_mmi) => {
                    if DBG_OPTS.show_wm_getminmaxinfo {
                        print_msg("WM_GETMINMAXINFO");
                    }
                },
                Err(e) => {
                    println!("Error in WM_GETMINMAXINFO: {:?}", e);
                }
            }
        }
        WM_NCCREATE => {
            match wm_nccreate(hwnd, lParam) {
                Ok(_cs) => {
                    if DBG_OPTS.show_wm_nccreate {
                        print_msg("WM_NCCREATE");
                    }
                },
                Err(e) => {
                    println!("Error in WM_NCCREATE: {:?}", e);
                }
            }
        }
        WM_NCDESTROY => {
            print_msg("WM_NCDESTROY");
        }
        WM_NCCALCSIZE => {
            match wm_nccalcsize(hwnd, lParam) {
                Ok(_rect) => {
                    if DBG_OPTS.show_wm_nccalcsize {
                        print_msg("WM_NCCALCSIZE");
                    }
                },
                Err(e) => {
                    println!("Error in WM_NCCALCSIZE: {:?}", e);
                }
            }
        }
        WM_CREATE => {
            match wm_create(hwnd, lParam) {
                Ok(_cs) => {
                    if DBG_OPTS.show_wm_create {
                        print_msg("WM_CREATE");
                    }
                },
                Err(e) => {
                    println!("Error in WM_CREATE: {:?}", e);
                }
            }
        }
        WM_SHOWWINDOW => {
            if DBG_OPTS.show_wm_showwindow {
                print_msg("WM_SHOWWINDOW");
            }
            wm_showwindow(hwnd, wParam, lParam);
        }
        WM_WINDOWPOSCHANGING => {
            match wm_windowposchanging(hwnd, lParam) {
                Ok(_wpos) => {
                    if DBG_OPTS.show_wm_windowposchanging {
                        print_msg("WM_WINDOWPOSCHANGING");
                    }
                },
                Err(e) => {
                    println!("Error in WM_WINDOWPOSCHANGING: {:?}", e);
                }
            }
        }
        WM_ACTIVATEAPP => {
            if DBG_OPTS.show_wm_activateapp {
                print_msg("WM_ACTIVATEAPP");
            }
            wm_activateapp(hwnd, wParam, lParam);
            // println!("wParam: {:?}, lParam: {:?}", wParam, lParam);
        },
        WM_NCACTIVATE => {
            if DBG_OPTS.show_wm_ncactivate {
                print_msg("WM_NCACTIVATE");
            }
            wm_ncactivate(hwnd, wParam, lParam);
        }
        WM_GETICON => {
            if DBG_OPTS.show_wm_geticon {
                print_msg("WM_GETICON");
            }
            wm_geticon(hwnd, wParam, lParam);
        }
        WM_ACTIVATE => {
            if DBG_OPTS.show_wm_activate {
                print_msg("WM_ACTIVATE");
            }
            wm_activate(hwnd, wParam, lParam);
            // println!("wParam: {:?}, lParam: {:?}", wParam, lParam);
        }
        WM_IME_SETCONTEXT => {
            if DBG_OPTS.show_wm_ime_setcontext {
                print_msg("WM_IME_SETCONTEXT");
            }
            wm_ime_setcontext(hwnd, wParam, lParam);
        }
        WM_IME_NOTIFY => {
            if DBG_OPTS.show_wm_ime_notify {
                print_msg("WM_IME_NOTIFY");
            }
            wm_ime_notify(hwnd, wParam, lParam);
        }
        WMSZ_BOTTOMLEFT => {
            if DBG_OPTS.show_wmsz_bottomleft {
                print_msg("WMSZ_BOTTOMLEFT");
            }
            wm_sizing(WMSZ_BOTTOMLEFT, hwnd, wParam, lParam);
        }
        WM_NCPAINT => {
            if DBG_OPTS.show_wm_ncpaint {
                print_msg("WM_NCPAINT");
            }
            wm_ncpaint(hwnd, wParam, lParam);
            println!("\n\n\t\t\tlatest");
        }
        WM_ERASEBKGND => {
            if DBG_OPTS.show_wm_erasebkgnd {
                print_msg("WM_ERASEBKGND");
            }
            wm_erasebkgnd(hwnd, wParam);
        }
        WM_CHARTOITEM => {
            if DBG_OPTS.show_wm_chartoitem {
                print_msg("WM_CHARTOITEM");
            }
            wm_chartoitem(hwnd, wParam, lParam);
        }


        // WM_ACTIVATE => println!("WM_ACTIVATE"),
        // WM_ACTIVATEAPP => println!("WM_ACTIVATEAPP"),
        // WM_AFXFIRST => println!("WM_AFXFIRST"),
        // WM_AFXLAST => println!("WM_AFXLAST"),
        // WM_APP => println!("WM_APP"),
        // WM_APPCOMMAND => println!("WM_APPCOMMAND"),
        // WM_ASKCBFORMATNAME => println!(""),
        // WM_CANCELJOURNAL => println!(""),
        // WM_CANCELMODE => println!(""),
        // WM_CAPTURECHANGED => println!(""),
        // WM_CHANGECBCHAIN => println!(""),
        // WM_CHANGEUISTATE => println!(""),
        // WM_CHAR => println!(""),
        // WM_CHARTOITEM => println!(""),
        // WM_CHILDACTIVATE => println!(""),
        // WM_CLEAR => println!(""),
        // WM_CLIPBOARDUPDATE => println!(""),
        // WM_CLOSE => println!(""),
        // WM_COMMAND => println!(""),
        // WM_COMMNOTIFY => println!(""),
        // WM_COMPACTING => println!(""),
        // WM_COMPAREITEM => println!(""),
        // WM_CONTEXTMENU => println!(""),
        // WM_COPY => println!(""),
        // WM_COPYDATA => println!(""),
        // WM_CREATE => println!(""),
        // WM_CTLCOLORBTN => println!(""),
        // WM_CTLCOLORDLG => println!(""),
        // WM_CTLCOLOREDIT => println!(""),
        // WM_CTLCOLORLISTBOX => println!(""),
        // WM_CTLCOLORMSGBOX => println!(""),
        // WM_CTLCOLORSCROLLBAR => println!(""),
        // WM_CTLCOLORSTATIC => println!(""),
        // WM_CUT => println!(""),
        // WM_DEADCHAR => println!(""),
        // WM_DELETEITEM => println!(""),
        // WM_DESTROY => println!(""),
        // WM_DESTROYCLIPBOARD => println!(""),
        // WM_DEVICECHANGE => println!(""),
        // WM_DEVMODECHANGE => println!(""),
        // WM_DISPLAYCHANGE => println!(""),
        // WM_DPICHANGED => println!(""),
        // WM_DPICHANGED_AFTERPARENT => println!(""),
        // WM_DPICHANGED_BEFOREPARENT => println!(""),
        // WM_DRAWCLIPBOARD => println!(""),
        // WM_DRAWITEM => println!(""),
        // WM_DROPFILES => println!(""),
        // WM_DWMCOLORIZATIONCOLORCHANGED => println!(""),
        // WM_DWMCOMPOSITIONCHANGED => println!(""),
        // WM_DWMNCRENDERINGCHANGED => println!(""),
        // WM_DWMSENDICONICLIVEPREVIEWBITMAP => println!(""),
        // WM_DWMSENDICONICTHUMBNAIL => println!(""),
        // WM_DWMWINDOWMAXIMIZEDCHANGE => println!(""),
        // WM_ENABLE => println!(""),
        // WM_ENDSESSION => println!(""),
        // WM_ENTERIDLE => println!(""),
        // WM_ENTERMENULOOP => println!(""),
        // WM_ENTERSIZEMOVE => println!(""),
        // WM_ERASEBKGND => println!(""),
        // WM_EXITMENULOOP => println!(""),
        // WM_EXITSIZEMOVE => println!(""),
        // WM_FONTCHANGE => println!(""),
        // WM_GESTURE => println!(""),
        // WM_GESTURENOTIFY => println!(""),
        // WM_GETDLGCODE => println!(""),
        // WM_GETDPISCALEDSIZE => println!(""),
        // WM_GETFONT => println!(""),
        // WM_GETHOTKEY => println!(""),
        // WM_GETICON => println!(""),
        // WM_GETMINMAXINFO => println!(""),
        // WM_GETOBJECT => println!(""),
        // WM_GETTEXT => println!(""),
        // WM_GETTEXTLENGTH => println!(""),
        // WM_GETTITLEBARINFOEX => println!(""),
        // WM_HANDHELDFIRST => println!(""),
        // WM_HANDHELDLAST => println!(""),
        // WM_HELP => println!(""),
        // WM_HOTKEY => println!(""),
        // WM_HSCROLL => println!(""),
        // WM_HSCROLLCLIPBOARD => println!(""),
        // WM_ICONERASEBKGND => println!(""),
        // WM_IME_CHAR => println!(""),
        // WM_IME_COMPOSITION => println!(""),
        // WM_IME_COMPOSITIONFULL => println!(""),
        // WM_IME_CONTROL => println!(""),
        // WM_IME_ENDCOMPOSITION => println!(""),
        // WM_IME_KEYDOWN => println!(""),
        // WM_IME_KEYLAST => println!(""),
        // WM_IME_KEYUP => println!(""),
        // WM_IME_NOTIFY => println!(""),
        // WM_IME_REQUEST => println!(""),
        // WM_IME_SELECT => println!(""),
        // WM_IME_SETCONTEXT => println!(""),
        // WM_IME_STARTCOMPOSITION => println!(""),
        // WM_INITDIALOG => println!(""),
        // WM_INITMENU => println!(""),
        // WM_INITMENUPOPUP => println!(""),
        // WM_INPUT => println!(""),
        // WM_INPUTLANGCHANGE => println!(""),
        // WM_INPUTLANGCHANGEREQUEST => println!(""),
        // WM_INPUT_DEVICE_CHANGE => println!(""),
        // WM_KEYDOWN => println!(""),
        // WM_KEYFIRST => println!(""),
        // WM_KEYLAST => println!(""),
        // WM_KEYUP => println!(""),
        // WM_KILLFOCUS => println!(""),
        // WM_LBUTTONDBLCLK => println!(""),
        // WM_LBUTTONDOWN => println!(""),
        // WM_LBUTTONUP => println!(""),
        // WM_MBUTTONDBLCLK => println!(""),
        // WM_MBUTTONDOWN => println!(""),
        // WM_MBUTTONUP => println!(""),
        // WM_MDIACTIVATE => println!(""),
        // WM_MDICASCADE => println!(""),
        // WM_MDICREATE => println!(""),
        // WM_MDIDESTROY => println!(""),
        // WM_MDIGETACTIVE => println!(""),
        // WM_MDIICONARRANGE => println!(""),
        // WM_MDIMAXIMIZE => println!(""),
        // WM_MDINEXT => println!(""),
        // WM_MDIREFRESHMENU => println!(""),
        // WM_MDIRESTORE => println!(""),
        // WM_MDISETMENU => println!(""),
        // WM_MDITILE => println!(""),
        // WM_MEASUREITEM => println!(""),
        // WM_MENUCHAR => println!(""),
        // WM_MENUCOMMAND => println!(""),
        // WM_MENUDRAG => println!(""),
        // WM_MENUGETOBJECT => println!(""),
        // WM_MENURBUTTONUP => println!(""),
        // WM_MENUSELECT => println!(""),
        // WM_MOUSEACTIVATE => println!(""),
        // WM_MOUSEFIRST => println!(""),
        // WM_MOUSEHWHEEL => println!(""),
        // WM_MOUSELAST => println!(""),
        // WM_MOUSEMOVE => println!(""),
        // WM_MOUSEWHEEL => println!(""),
        // WM_MOVE => println!(""),
        // WM_MOVING => println!(""),
        // WM_NCACTIVATE => println!(""),
        // WM_NCCALCSIZE => println!(""),
        // WM_NCCREATE => println!(""),
        // WM_NCDESTROY => println!(""),
        // WM_NCHITTEST => println!(""),
        // WM_NCLBUTTONDBLCLK => println!(""),
        // WM_NCLBUTTONDOWN => println!(""),
        // WM_NCLBUTTONUP => println!(""),
        // WM_NCMBUTTONDBLCLK => println!(""),
        // WM_NCMBUTTONDOWN => println!(""),
        // WM_NCMBUTTONUP => println!(""),
        // WM_NCMOUSEHOVER => println!(""),
        // WM_NCMOUSELEAVE => println!(""),
        // WM_NCMOUSEMOVE => println!(""),
        // WM_NCPAINT => println!(""),
        // WM_NCPOINTERDOWN => println!(""),
        // WM_NCPOINTERUP => println!(""),
        // WM_NCPOINTERUPDATE => println!(""),
        // WM_NCRBUTTONDBLCLK => println!(""),
        // WM_NCRBUTTONDOWN => println!(""),
        // WM_NCRBUTTONUP => println!(""),
        // WM_NCXBUTTONDBLCLK => println!(""),
        // WM_NCXBUTTONDOWN => println!(""),
        // WM_NCXBUTTONUP => println!(""),
        // WM_NEXTDLGCTL => println!(""),
        // WM_NEXTMENU => println!(""),
        // WM_NOTIFY => println!(""),
        // WM_NOTIFYFORMAT => println!(""),
        // WM_NULL => println!(""),
        // WM_PAINT => println!(""),
        // WM_PAINTCLIPBOARD => println!(""),
        // WM_PAINTICON => println!(""),
        // WM_PALETTECHANGED => println!(""),
        // WM_PALETTEISCHANGING => println!(""),
        // WM_PARENTNOTIFY => println!(""),
        // WM_PASTE => println!(""),
        // WM_PENWINFIRST => println!(""),
        // WM_PENWINLAST => println!(""),
        // WM_POINTERACTIVATE => println!(""),
        // WM_POINTERCAPTURECHANGED => println!(""),
        // WM_POINTERDEVICECHANGE => println!(""),
        // WM_POINTERDEVICEINRANGE => println!(""),
        // WM_POINTERDEVICEOUTOFRANGE => println!(""),
        // WM_POINTERDOWN => println!(""),
        // WM_POINTERENTER => println!(""),
        // WM_POINTERHWHEEL => println!(""),
        // WM_POINTERLEAVE => println!(""),
        // WM_POINTERROUTEDAWAY => println!(""),
        // WM_POINTERROUTEDRELEASED => println!(""),
        // WM_POINTERROUTEDTO => println!(""),
        // WM_POINTERUP => println!(""),
        // WM_POINTERUPDATE => println!(""),
        // WM_POINTERWHEEL => println!(""),
        // WM_POWER => println!(""),
        // WM_POWERBROADCAST => println!(""),
        // WM_PRINT => println!(""),
        // WM_PRINTCLIENT => println!(""),
        // WM_QUERYDRAGICON => println!(""),
        // WM_QUERYENDSESSION => println!(""),
        // WM_QUERYNEWPALETTE => println!(""),
        // WM_QUERYOPEN => println!(""),
        // WM_QUERYUISTATE => println!(""),
        // WM_QUEUESYNC => println!(""),
        // WM_QUIT => println!(""),
        // WM_RBUTTONDBLCLK => println!(""),
        // WM_RBUTTONDOWN => println!(""),
        // WM_RBUTTONUP => println!(""),
        // WM_RENDERALLFORMATS => println!(""),
        // WM_RENDERFORMAT => println!(""),
        // WM_SETCURSOR => println!(""),
        // WM_SETFOCUS => println!(""),
        // WM_SETFONT => println!(""),
        // WM_SETHOTKEY => println!(""),
        // WM_SETICON => println!(""),
        // WM_SETREDRAW => println!(""),
        // WM_SETTEXT => println!(""),
        // WM_SETTINGCHANGE => println!(""),
        // WM_SHOWWINDOW => println!(""),
        // WM_SIZE => println!(""),
        // WM_SIZECLIPBOARD => println!(""),
        // WM_SIZING => println!(""),
        // WM_SPOOLERSTATUS => println!(""),
        // WM_STYLECHANGED => println!(""),
        // WM_STYLECHANGING => println!(""),
        // WM_SYNCPAINT => println!(""),
        // WM_SYSCHAR => println!(""),
        // WM_SYSCOLORCHANGE => println!(""),
        // WM_SYSCOMMAND => println!(""),
        // WM_SYSDEADCHAR => println!(""),
        // WM_SYSKEYDOWN => println!(""),
        // WM_SYSKEYUP => println!(""),
        // WM_TABLET_FIRST => println!(""),
        // WM_TABLET_LAST => println!(""),
        // WM_TCARD => println!(""),
        // WM_THEMECHANGED => println!(""),
        // WM_TIMECHANGE => println!(""),
        // WM_TIMER => println!(""),
        // WM_TOOLTIPDISMISS => println!(""),
        // WM_TOUCH => println!(""),
        // WM_TOUCHHITTESTING => println!(""),
        // WM_UNDO => println!(""),
        // WM_UNICHAR => println!(""),
        // WM_UNINITMENUPOPUP => println!(""),
        // WM_UPDATEUISTATE => println!(""),
        // WM_USER => println!(""),
        // WM_USERCHANGED => println!(""),
        // WM_VKEYTOITEM => println!(""),
        // WM_VSCROLL => println!(""),
        // WM_VSCROLLCLIPBOARD => println!(""),
        // WM_WINDOWPOSCHANGED => println!(""),
        // WM_WINDOWPOSCHANGING => println!(""),
        // WM_WININICHANGE => println!(""),
        // WM_WTSSESSION_CHANGE => println!(""),
        // WM_XBUTTONDBLCLK => println!(""),
        // WM_XBUTTONDOWN => println!(""),
        // WM_XBUTTONUP => println!(""),
        _ => {
            print_msg("");
            // // println!("[{:x?}] 0x{:04x}, {}, {}", hwnd.0, msg, wParam.0, lParam.0);
            // // println!("Message {:?} received", msg);
            // return DefWindowProcW(hwnd, msg, wParam, lParam);
        },
    }
    return DefWindowProcW(hwnd, msg, wParam, lParam);
    // LRESULT(0)
}

pub trait WndProc: Borrow<WNDPROC> {
    // unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    //     DefWindowProcW(hwnd, msg, wparam, lparam)
    // }
    unsafe extern "system" fn wnd_proc(&self,
        hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}

impl<T: Borrow<WNDPROC>> WndProc for T where T: Fn(HWND, u32, WPARAM, LPARAM) -> LRESULT {
    unsafe extern "system" fn wnd_proc(&self,
        hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        self(hwnd, msg, wparam, lparam)
    }
}

impl WndProc for WNDPROC {
    unsafe extern "system" fn wnd_proc(&self, hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match self {
            Some(wnd_proc) => wnd_proc(hwnd, msg, wparam, lparam),
            None => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

/// Sent to a window when the size or position of the window is about to change. An application can use this message to override the window's default maximized size and position, or its default minimum or maximum tracking size.
fn wm_getminmaxinfo(hwnd: HWND, lParam: LPARAM) -> Result<MINMAXINFO> {
    if lParam == LPARAM(0) {
        return Err(Error::new(unsafe{GetLastError().into()}, "lParam is null"));
    }
    let minmaxinfo = unsafe { &mut *(lParam.0 as *mut MINMAXINFO) }.clone();
    if DBG_OPTS.show_wm_getminmaxinfo {
        log::trace!("wm_getminmaxinfo: {:?}", minmaxinfo);
    }

    Ok(minmaxinfo)
}



/// Sent prior to the WM_CREATE message when a window is first created.
fn wm_nccreate(hwnd: HWND, lParam: LPARAM) -> Result<CREATESTRUCTW> {
    if lParam == LPARAM(0) {
        eprintln!("lParam is null");
        return Err(Error::new(unsafe{GetLastError().into()}, "lParam is null"));
    }
    let cs = unsafe { &mut *(lParam.0 as *mut CREATESTRUCTW) }.clone();

    if DBG_OPTS.show_wm_nccreate {
        log::trace!("wm_nccreate: {:?}", cs);

        log::trace!("location  : {} x {}", cs.x, cs.y);
        log::trace!("dimensions: {} x {}", cs.cx, cs.cy);
        // log::trace!("style     : 0x{:x}", createstruct.style);
        let style = WINDOW_STYLE(cs.style as u32);
        log::trace!("style     : {:?}", style);
        log::trace!("style (x) : {:?}", cs.dwExStyle);
        if cs.lpszName.is_null() {
            log::trace!("name      : null");
        } else {
            let name = unsafe{cs.lpszName.display()}.to_string();
            log::trace!("name      : {}", name);
        }

        // // TODO: the following crashes if the class is an ATOM
        // let class = unsafe{cs.lpszClass.display()}.to_string();
        // // let class = cs.lpszClass;
        // // let class = if class.is_null() {
        // //     "null".to_string()
        // // } else {
        // //     unsafe { String::from_utf16_lossy(std::slice::from_raw_parts(class.0, 256)) }
        // // };
        // log::trace!("class     : {}", class);
    }

    Ok(cs)
}

fn wm_nccalcsize(hwnd: HWND, lParam: LPARAM) -> Result<RECT> {
    if lParam == LPARAM(0) {
        log::trace!("lParam is null");
        return Err(Error::new(unsafe{GetLastError().into()}, "lParam is null"));
    }
    let rect = unsafe { &mut *(lParam.0 as *mut RECT) }.clone();
    if DBG_OPTS.show_wm_nccalcsize {
        log::trace!("wm_nccalcsize: {:?}", rect);
    }
    Ok(rect)
}

fn wm_create(hwnd: HWND, lParam: LPARAM) -> Result<CREATESTRUCTW> {
    if lParam == LPARAM(0) {
        log::trace!("lParam is null");
        return Err(Error::new(unsafe{GetLastError().into()}, "lParam is null"));
    }
    let cs = unsafe { &mut *(lParam.0 as *mut CREATESTRUCTW) }.clone();
    if DBG_OPTS.show_wm_create {
        log::trace!("wm_create: {:?}", cs);
        if cs.hwndParent.is_invalid() {
            log::trace!("Creating root window");
        }
        log::trace!("location  : {} x {}", cs.x, cs.y);
        log::trace!("dimensions: {} x {}", cs.cx, cs.cy);
        // log::trace!("style     : 0x{:x}", createstruct.style);
        let style = WINDOW_STYLE(cs.style as u32);
        log::trace!("style     : {:?}", style);
        log::trace!("style (x) : {:?}", cs.dwExStyle);
        if cs.lpszName.is_null() {
            log::trace!("name      : null");
        } else {
            let name = unsafe{cs.lpszName.display()}.to_string();
            log::trace!("name      : {}", name);
        }

        // // TODO: the following crashes if the class is an ATOM
        // let class = unsafe{cs.lpszClass.display()}.to_string();
        // // let class = cs.lpszClass;
        // // let class = if class.is_null() {
        // //     "null".to_string()
        // // } else {
        // //     unsafe { String::from_utf16_lossy(std::slice::from_raw_parts(class.0, 256)) }
        // // };
        // log::trace!("class     : {}", class);
    }

    Ok(cs)
}


fn wm_showwindow(hwnd: HWND, wParam: WPARAM, lParam: LPARAM) {
    if DBG_OPTS.show_wm_showwindow {
        log::trace!("wm_showwindow: wParam: {}, lParam: {}", wParam.0, lParam.0);
        match (wParam.0, lParam.0) {
            (1, 0) => {
                log::trace!("Window is being shown due to a call to ShowWindow");
            },
            (0, 0) => {
                log::trace!("Window is being hidden due to a call to ShowWindow");
            },
            (s, p) if p == SW_PARENTCLOSING.0 as isize => {
                log::trace!("Window is being {} because its owner window is being minimized.", if s == 1 {"shown"} else {"hidden"});
            }
            (s, p) if p == SW_OTHERZOOM.0 as isize => {
                log::trace!("Window is being {} because it is being covered by another window that has been maximized.", if s == 1 {"shown"} else {"hidden"});
            }
            (s, p) if p == SW_PARENTOPENING.0 as isize => {
                log::trace!("Window is being {} because its owner window is being restored.", if s == 1 {"shown"} else {"hidden"});
            }
            (s, p) if p == SW_OTHERUNZOOM.0 as isize => {
                log::trace!("Window is being {} because a maximize window was restored or minimized.", if s == 1 {"shown"} else {"hidden"});
            }
            (s, p) => {
                log::trace!("Window is being {} because of reason #{}.", if s == 1 {"shown"} else {"hidden"}, p);
            }
        }
    }
}

fn wm_windowposchanging(hwnd: HWND, lParam: LPARAM) -> Result<WINDOWPOS> {
    if lParam == LPARAM(0) {
        log::trace!("lParam is null");
        return Err(Error::new(unsafe{GetLastError().into()}, "lParam is null"));
    }
    let wp = unsafe { &mut *(lParam.0 as *mut WINDOWPOS) }.clone();
    if DBG_OPTS.show_wm_windowposchanging {
        log::trace!("wm_windowposchanging: {:?}", wp);
        log::trace!("location  : {} x {}", wp.x, wp.y);
        log::trace!("dimensions: {} x {}", wp.cx, wp.cy);
        log::trace!("flags     : 0x{:x}", wp.flags.0);
    }
    Ok(wp)
}

fn wm_activate(hwnd: HWND, wParam: WPARAM, lParam: LPARAM) {
    if DBG_OPTS.show_wm_activate {
        if wParam.0 == WA_ACTIVE as usize {
            log::trace!("Window is being activated (owning thread: [0x{:x}])", lParam.0);
        } else if wParam.0 == WA_CLICKACTIVE as usize {
            log::trace!("Window is being activated by a mouse click (owning thread: [0x{:x}])", lParam.0);
        } else if wParam.0 == WA_INACTIVE as usize {
            log::trace!("Window is being deactivated (owning thread: [0x{:x}])", lParam.0);
        }
    }
}
fn wm_activateapp(hwnd: HWND, wParam: WPARAM, lParam: LPARAM) {
    if DBG_OPTS.show_wm_activateapp {
        if wParam.0 == WA_ACTIVE as usize {
            log::trace!("App is being activated (owning thread: [0x{:x}])", lParam.0);
        } else if wParam.0 == WA_CLICKACTIVE as usize {
            log::trace!("App is being activated by a mouse click (owning thread: [0x{:x}])", lParam.0);
        } else if wParam.0 == WA_INACTIVE as usize {
            log::trace!("App is being deactivated (owning thread: [0x{:x}])", lParam.0);
        }
    }
}

fn wm_ncactivate(hwnd: HWND, wParam: WPARAM, lParam: LPARAM) {
    if DBG_OPTS.show_wm_ncactivate {
        if wParam.0 == 1 {
            log::trace!("Non-client area is being activated");
        } else if wParam.0 == 0 {
            log::trace!("Non-client area is being deactivated");
        }
    }
}

fn wm_geticon(hwnd: HWND, wParam: WPARAM, lParam: LPARAM) {
    if DBG_OPTS.show_wm_geticon || lParam.0 != 0 {
        if wParam.0 == ICON_SMALL as usize {
            log::trace!("Message requesting small icon ({} dpi)", lParam.0);
        } else if wParam.0 == ICON_BIG as usize {
            log::trace!("Message requesting big icon ({} dpi)", lParam.0);
        } else if wParam.0 == ICON_SMALL2 as usize {
            log::trace!("Message requesting small icon 2 ({} dpi)", lParam.0);
        } else {
            log::trace!("Message requesting icon of unknown size (wParam: {:?} lParam: {:?})", wParam, lParam);
        }
    }
}

fn wm_ime_setcontext(hwnd: HWND, wParam: WPARAM, lParam: LPARAM) {
    if DBG_OPTS.show_wm_ime_setcontext {
        if wParam.0 == 1 {
            log::trace!("IME is being set to active");
        } else if wParam.0 == 0 {
            log::trace!("IME is being set to inactive");
        } else {
            log::trace!("IME is being set to unknown state (wParam: {:?} lParam: {:?})", wParam, lParam);
        }

        if lParam.0 == ISC_SHOWUIALL as isize {
            log::trace!("IME is being set to show all UI.");
            // dbg!(lParam.0, ISC_SHOWUIALL as isize);
        } else {
            // if (lParam.0 & (ISC_SHOWUIALLCANDIDATEWINDOW as isize)) == ISC_SHOWUIALLCANDIDATEWINDOW as isize {
            //     log::trace!("IME is being set to show all candidate window.");
            //     dbg!(lParam.0, ISC_SHOWUIALLCANDIDATEWINDOW as isize);
            // }
            for i in 0..4 as usize {
                if (lParam.0 & ((ISC_SHOWUICANDIDATEWINDOW<<i) as isize)) == (ISC_SHOWUICANDIDATEWINDOW as isize) {
                    log::trace!("IME is being set to show candidate window {}.", i);
                    dbg!(lParam.0, (ISC_SHOWUICANDIDATEWINDOW<<i) as isize);
                }
            }

        }
        if (lParam.0 & (ISC_SHOWUICOMPOSITIONWINDOW as isize)) == ISC_SHOWUICOMPOSITIONWINDOW as isize {
            log::trace!("IME is being set to show composition window. (Show the composition window by user interface window.)");
            // dbg!(lParam.0, ISC_SHOWUICOMPOSITIONWINDOW as isize);
        }
        if (lParam.0 & (ISC_SHOWUIGUIDELINE as isize)) == ISC_SHOWUIGUIDELINE as isize {
            log::trace!("IME is being set to show guide line. (Show the guide window by user interface window.)");
            // dbg!(lParam.0, ISC_SHOWUIGUIDELINE as isize);
        }
    }
}

fn wm_ime_notify(hwnd: HWND, wParam: WPARAM, lParam: LPARAM) {
    if DBG_OPTS.show_wm_ime_notify {

        if wParam.0 == IMN_CHANGECANDIDATE as usize {
            log::trace!("IME is notifying of candidate change.");
        }
        if wParam.0 == IMN_CLOSECANDIDATE as usize {
            log::trace!("IME is notifying of candidate window closing.");
        }
        if wParam.0 == IMN_CLOSESTATUSWINDOW as usize {
            log::trace!("IME is closing the status window.");
        }
        if wParam.0 == IMN_GUIDELINE as usize {
            log::trace!("IME is notifying of guideline.");
        }
        if wParam.0 == IMN_OPENCANDIDATE as usize {
            log::trace!("IME is opening the candidate window.");
        }
        if wParam.0 == IMN_OPENSTATUSWINDOW as usize {
            log::trace!("IME is opening the status window.");
        }
        if wParam.0 == IMN_SETCANDIDATEPOS as usize {
            log::trace!("IME is setting candidate position.");
        }
        if wParam.0 == IMN_SETCOMPOSITIONFONT as usize {
            log::trace!("IME is setting composition font.");
        }
        if wParam.0 == IMN_SETCOMPOSITIONWINDOW as usize {
            log::trace!("IME is setting composition window.");
        }
        if wParam.0 == IMN_SETCONVERSIONMODE as usize {
            log::trace!("IME is setting conversion mode.");
        }
        if wParam.0 == IMN_SETOPENSTATUS as usize {
            log::trace!("IME is setting open status.");
        }
        if wParam.0 == IMN_SETSENTENCEMODE as usize {
            log::trace!("IME is setting sentence mode.");
        }
        if wParam.0 == IMN_SETSTATUSWINDOWPOS as usize {
            log::trace!("IME is setting status window position.");
        }
        if lParam.0 != 0 {
            log::trace!("IME notify event params (wParam: {:?} lParam: {:?})", wParam, lParam);
        }
    }
}

fn wm_sizing(msg: u32, hwnd: HWND, wParam: WPARAM, lParam: LPARAM) {
    match msg {
        WMSZ_BOTTOMLEFT => {
            if DBG_OPTS.show_wmsz_bottomleft {
                log::trace!("Window is being resized from the bottom-left corner (?).");
            }
            return;
        }
        _ => {
            log::trace!("wm_sizing: 0x{:x} ({}) wParam: {}, lParam: {}", msg, msg, wParam.0, lParam.0);
        }
    }
}

fn wm_ncpaint(hwnd: HWND, wParam: WPARAM, lParam: LPARAM) {
    if DBG_OPTS.show_wm_ncpaint {
        log::trace!("wm_ncpaint: wParam: {}, lParam: {}", wParam.0, lParam.0);
    }
}

fn wm_erasebkgnd(hwnd: HWND, wParam: WPARAM) {
    if DBG_OPTS.show_wm_erasebkgnd {
        log::trace!("wm_erasebkgnd: hDC: 0x{:08x}", wParam.0);
    }
}

fn wm_chartoitem(hwnd: HWND, wParam: WPARAM, lParam: LPARAM) {
    if DBG_OPTS.show_wm_chartoitem {
        let key = wParam.0 as u16;
        let pos = (wParam.0 >> 16) as u16;
        log::trace!("wm_chartoitem: char: {}, pos: {}, hWND: 0x{:08x}", key, pos, lParam.0);
    }
}


//
