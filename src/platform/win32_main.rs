#![allow(non_snake_case, unused)]
use std::{borrow::Borrow, ffi::c_void, io::Read, mem::ManuallyDrop, ops::BitAnd, sync::{Arc, Mutex}};

use windows::{
    core::*, Data::Xml::Dom::*,
    Win32::{
        Foundation::*, Graphics::Gdi::*,
        System::{LibraryLoader::*, Threading::*},
        UI::{Controls::*, HiDpi::*, Input::Ime::*, WindowsAndMessaging::*}
    },
};

use super::Window;

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
    pub show_wm_windowposchanged: bool,
    pub show_wmsz_topright: bool,
    pub show_wmsz_top: bool,
    pub show_dwmncrenderingchanged: bool,
    pub show_wm_paint: bool,
    pub show_wm_nchittest: bool,

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
    show_wm_getminmaxinfo: true,//false,
    show_wm_nccreate: true,//false,
    show_wm_nccalcsize: true,//false,
    show_wm_create: true,//false,
    show_wm_showwindow: true,//false,
    show_wm_windowposchanging: true,//false,
    show_wm_activateapp: true,//false,
    show_wm_ncactivate: true,//false,
    show_wm_geticon: true,//false,
    show_wm_activate: true,//false,
    show_wm_ime_setcontext: true,//false,
    show_wm_ime_notify: true,//false,
    show_wmsz_bottomleft: true,//false,
    show_wm_ncpaint: true,//false,
    show_wm_erasebkgnd: true,//false,
    show_wm_chartoitem: true,
    show_wm_windowposchanged: true,//false,
    show_wmsz_topright: true,//false,
    show_wmsz_top: true,//false,
    show_dwmncrenderingchanged: true,//false,
    show_wm_paint: true,//false,
    show_wm_nchittest: true,

    image_path: "vendor/oculante/res/screenshot_exif.png",
};

// const image_path: &str = "vendor/oculante/res/screenshot_exif.png";
const image_path: &str = DBG_OPTS.image_path;

pub fn main() -> Result<()> {
    use WndProc;

    unsafe {SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE)}
        .expect("Failed to set process DPI awareness");
    let handle_instance = unsafe {GetModuleHandleW(None)}
        .expect("Failed to get module handle");

    let wc = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(window_proc),
        hInstance: handle_instance.into(),
        // hCursor: None,
        // hbrBackground: None,
        lpszClassName: w!("img-browser-rs"),
        cbWndExtra: 0,
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
    log::debug!("Image dimensions: {}x{}", width, height);

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
                    log::info!("WM_QUIT");
                } else {
                    log::info!("WM_CLOSE");
                }
                break;
            },
            WM_CREATE => {
                log::info!("WM_CREATE");
            },
            WM_PAINT => {
                log::info!("{:03} WM_PAINT {} {:?} {:?} {:?}",
                    ctr_base-ctr, msg.time, msg.pt, msg.lParam, msg.wParam);
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
                        log::info!("BitBlt failed: {}", unsafe{GetLastError()}.0);
                    }
                    // dbg!(bres);
    
                    unsafe{SelectObject(hdc_mem, prev_bmp)};
    
                    unsafe{DeleteDC(hdc_mem)}.expect("DeleteDC failed");
                    // log::info!("DeleteDC called");
                }
    
                unsafe{EndPaint(window, &ps)}.expect("EndPaint failed");
                // log::info!("EndPaint called");

                // // TODO: do we continue, or fall through here?
                // return LRESULT(0);
                // continue;
            },
    
            WM_TIMER => {
                log::info!("{:03} WM_TIMER {} {:?} {:?} {:?}",
                    ctr_base-ctr, msg.time, msg.pt, msg.lParam, msg.wParam);
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
            WM_CANCELMODE => log::info!("WM_CANCELMODE"),
            WM_CHANGECBCHAIN => log::info!("WM_CHANGECBCHAIN"),
            WM_CAPTURECHANGED => log::info!("WM_CAPTURECHANGED"),
            WM_CHAR => log::info!("WM_CHAR"),
            WM_COMMAND => log::info!("WM_COMMAND"),
            WM_CTLCOLORBTN => log::info!("WM_CTLCOLORBTN"),
            WM_CTLCOLORDLG => log::info!("WM_CTLCOLORDLG"),
            WM_CTLCOLOREDIT => log::info!("WM_CTLCOLOREDIT"),
            WM_CTLCOLORLISTBOX => log::info!("WM_CTLCOLORLISTBOX"),
            WM_CTLCOLORMSGBOX => log::info!("WM_CTLCOLORMSGBOX"),
            WM_CTLCOLORSCROLLBAR => log::info!("WM_CTLCOLORSCROLLBAR"),
            WM_CTLCOLORSTATIC => log::info!("WM_CTLCOLORSTATIC"),
            WM_DEVICECHANGE => log::info!("WM_DEVICECHANGE"),
            WM_DISPLAYCHANGE => log::info!("WM_DISPLAYCHANGE"),
            WM_DRAWCLIPBOARD => log::info!("WM_DRAWCLIPBOARD"),
            WM_DRAWITEM => log::info!("WM_DRAWITEM"),
            WM_ERASEBKGND => log::info!("WM_ERASEBKGND"),
            WM_GETMINMAXINFO => log::info!("WM_GETMINMAXINFO"),
            WM_INPUT => log::info!("WM_INPUT"),
            WM_KEYDOWN => log::info!("WM_KEYDOWN"),
            WM_KEYLAST => log::info!("WM_KEYLAST"),
            WM_KEYUP => log::info!("WM_KEYUP"),
            WM_LBUTTONDOWN => log::info!("WM_LBUTTONDOWN"),
            WM_LBUTTONUP => log::info!("WM_LBUTTONUP"),
            WM_MOUSEMOVE => log::info!("WM_MOUSEMOVE"),
            WM_RBUTTONDOWN => log::info!("WM_RBUTTONDOWN"),
            WM_RBUTTONUP => log::info!("WM_RBUTTONUP"),
            WM_SIZE => log::info!("WM_SIZE"),
            WM_SYSKEYDOWN => log::info!("WM_SYSKEYDOWN"),
            WM_SYSKEYUP => log::info!("WM_SYSKEYUP"),
            WM_SYSCHAR => log::info!("WM_SYSCHAR"),
            WM_SYSDEADCHAR => log::info!("WM_SYSDEADCHAR"),
            WM_USER => log::info!("WM_USER"),
            WM_XBUTTONDOWN => log::info!("WM_XBUTTONDOWN"),
            WM_XBUTTONUP => log::info!("WM_XBUTTONUP"),
            WM_MOUSEHOVER => log::info!("WM_MOUSEHOVER"),
            WM_MOUSELEAVE => log::info!("WM_MOUSELEAVE"),
            WM_NCACTIVATE => log::info!("WM_NCACTIVATE"),
            WM_NCCALCSIZE => log::info!("WM_NCCALCSIZE"),
            WM_NCHITTEST => log::info!("WM_NCHITTEST"),
            WM_NCLBUTTONDBLCLK => log::info!("WM_NCLBUTTONDBLCLK"),
            WM_NCLBUTTONDOWN => log::info!("WM_NCLBUTTONDOWN"),
            WM_NCLBUTTONUP => log::info!("WM_NCLBUTTONUP"),
            WM_NCMBUTTONDBLCLK => log::info!("WM_NCMBUTTONDBLCLK"),
            WM_NCMBUTTONDOWN => log::info!("WM_NCMBUTTONDOWN"),
            WM_NCMBUTTONUP => log::info!("WM_NCMBUTTONUP"),
            WM_NCMOUSEHOVER => log::info!("WM_NCMOUSEHOVER"),
            WM_NCMOUSELEAVE => log::info!("WM_NCMOUSELEAVE"),
            WM_NCMOUSEMOVE => { // this seeems to be the one that triggers when the mouse moves over the window
                log::info!("{:03} WM_NCMOUSEMOVE {} {:?} {:?} {:?}",
                    ctr_base-ctr, msg.time, msg.pt, msg.lParam, msg.wParam);
            },
            WM_NCPAINT => log::info!("WM_NCPAINT"),
            WM_NCRBUTTONDBLCLK => log::info!("WM_NCRBUTTONDBLCLK"),
            WM_NCRBUTTONDOWN => log::info!("WM_NCRBUTTONDOWN"),
            WM_NCRBUTTONUP => log::info!("WM_NCRBUTTONUP"),
            WM_NCXBUTTONDOWN => log::info!("WM_NCXBUTTONDOWN"),
            WM_NCXBUTTONUP => log::info!("WM_NCXBUTTONUP"),
            WM_NCXBUTTONDBLCLK => log::info!("WM_NCXBUTTONDBLCLK"),
            WM_PAINTICON => log::info!("WM_PAINTICON"),
            WM_SETCURSOR => log::info!("WM_SETCURSOR"),
            WM_SETFOCUS => log::info!("WM_SETFOCUS"),
            WM_SETICON => log::info!("WM_SETICON"),
            WM_SETTEXT => log::info!("WM_SETTEXT"),
            WM_SHOWWINDOW => log::info!("WM_SHOWWINDOW"),
            WM_SYSCOMMAND => log::info!("WM_SYSCOMMAND"),
            WM_THEMECHANGED => log::info!("WM_THEMECHANGED"),
            WM_WINDOWPOSCHANGED => log::info!("WM_WINDOWPOSCHANGED"),
            WM_WINDOWPOSCHANGING => log::info!("WM_WINDOWPOSCHANGING"),
            WM_MOUSEWHEEL => log::info!("WM_MOUSEWHEEL"),
            WM_MOUSEHWHEEL => log::info!("WM_MOUSEHWHEEL"),
            WM_MOUSEACTIVATE => log::info!("WM_MOUSEACTIVATE"),
            WM_INITDIALOG => log::info!("WM_INITDIALOG"),
            WM_DWMNCRENDERINGCHANGED if msg.wParam == WPARAM(1) => {
                log::info!("{:03} WM_DWMNCRENDERINGCHANGED: {} {} {:?} {:?} {:?}",
                    ctr_base-ctr, " on", msg.time, msg.pt, msg.lParam, msg.wParam);
                // continue;
            },
            WM_DWMNCRENDERINGCHANGED if msg.wParam == WPARAM(0) => {
                log::info!("{:03} WM_DWMNCRENDERINGCHANGED: {} {} {:?} {:?} {:?}",
                    ctr_base-ctr, "off", msg.time, msg.pt, msg.lParam, msg.wParam);
                // continue;
            },

            WM_ACTIVATE => log::info!("WM_ACTIVATE: {} {:?}", msg.time, msg),
            WM_ACTIVATEAPP => log::info!("WM_ACTIVATEAPP"),
            WM_AFXFIRST => log::info!("WM_AFXFIRST"),
            WM_AFXLAST => log::info!("WM_AFXLAST"),
            WM_APP => log::info!("WM_APP"),
            WM_APPCOMMAND => log::info!("WM_APPCOMMAND"),
            WM_ASKCBFORMATNAME => log::info!(""),
            WM_CANCELJOURNAL => log::info!(""),
            WM_CANCELMODE => log::info!(""),
            WM_CAPTURECHANGED => log::info!(""),
            WM_CHANGECBCHAIN => log::info!(""),
            WM_CHANGEUISTATE => log::info!(""),
            WM_CHAR => log::info!(""),
            WM_CHARTOITEM => log::info!(""),
            WM_CHILDACTIVATE => log::info!(""),
            WM_CLEAR => log::info!(""),
            WM_CLIPBOARDUPDATE => log::info!(""),
            WM_CLOSE => log::info!(""),
            WM_COMMAND => log::info!(""),
            WM_COMMNOTIFY => log::info!(""),
            WM_COMPACTING => log::info!(""),
            WM_COMPAREITEM => log::info!(""),
            WM_CONTEXTMENU => log::info!(""),
            WM_COPY => log::info!(""),
            WM_COPYDATA => log::info!(""),
            WM_CREATE => log::info!(""),
            WM_CTLCOLORBTN => log::info!(""),
            WM_CTLCOLORDLG => log::info!(""),
            WM_CTLCOLOREDIT => log::info!(""),
            WM_CTLCOLORLISTBOX => log::info!(""),
            WM_CTLCOLORMSGBOX => log::info!(""),
            WM_CTLCOLORSCROLLBAR => log::info!(""),
            WM_CTLCOLORSTATIC => log::info!(""),
            WM_CUT => log::info!(""),
            WM_DEADCHAR => log::info!(""),
            WM_DELETEITEM => log::info!(""),
            WM_DESTROY => log::info!(""),
            WM_DESTROYCLIPBOARD => log::info!(""),
            WM_DEVICECHANGE => log::info!(""),
            WM_DEVMODECHANGE => log::info!(""),
            WM_DISPLAYCHANGE => log::info!(""),
            WM_DPICHANGED => log::info!(""),
            WM_DPICHANGED_AFTERPARENT => log::info!(""),
            WM_DPICHANGED_BEFOREPARENT => log::info!(""),
            WM_DRAWCLIPBOARD => log::info!(""),
            WM_DRAWITEM => log::info!(""),
            WM_DROPFILES => log::info!(""),
            WM_DWMCOLORIZATIONCOLORCHANGED => log::info!(""),
            WM_DWMCOMPOSITIONCHANGED => log::info!(""),
            WM_DWMNCRENDERINGCHANGED => log::info!(""),
            WM_DWMSENDICONICLIVEPREVIEWBITMAP => log::info!(""),
            WM_DWMSENDICONICTHUMBNAIL => log::info!(""),
            WM_DWMWINDOWMAXIMIZEDCHANGE => log::info!(""),
            WM_ENABLE => log::info!(""),
            WM_ENDSESSION => log::info!(""),
            WM_ENTERIDLE => log::info!(""),
            WM_ENTERMENULOOP => log::info!(""),
            WM_ENTERSIZEMOVE => log::info!(""),
            WM_ERASEBKGND => log::info!(""),
            WM_EXITMENULOOP => log::info!(""),
            WM_EXITSIZEMOVE => log::info!(""),
            WM_FONTCHANGE => log::info!(""),
            WM_GESTURE => log::info!(""),
            WM_GESTURENOTIFY => log::info!(""),
            WM_GETDLGCODE => log::info!(""),
            WM_GETDPISCALEDSIZE => log::info!(""),
            WM_GETFONT => log::info!(""),
            WM_GETHOTKEY => log::info!(""),
            WM_GETICON => log::info!(""),
            WM_GETMINMAXINFO => log::info!(""),
            WM_GETOBJECT => log::info!(""),
            WM_GETTEXT => log::info!(""),
            WM_GETTEXTLENGTH => log::info!(""),
            WM_GETTITLEBARINFOEX => log::info!(""),
            WM_HANDHELDFIRST => log::info!(""),
            WM_HANDHELDLAST => log::info!(""),
            WM_HELP => log::info!(""),
            WM_HOTKEY => log::info!(""),
            WM_HSCROLL => log::info!(""),
            WM_HSCROLLCLIPBOARD => log::info!(""),
            WM_ICONERASEBKGND => log::info!(""),
            WM_IME_CHAR => log::info!(""),
            WM_IME_COMPOSITION => log::info!(""),
            WM_IME_COMPOSITIONFULL => log::info!(""),
            WM_IME_CONTROL => log::info!(""),
            WM_IME_ENDCOMPOSITION => log::info!(""),
            WM_IME_KEYDOWN => log::info!(""),
            WM_IME_KEYLAST => log::info!(""),
            WM_IME_KEYUP => log::info!(""),
            WM_IME_NOTIFY => log::info!(""),
            WM_IME_REQUEST => log::info!(""),
            WM_IME_SELECT => log::info!(""),
            WM_IME_SETCONTEXT => log::info!(""),
            WM_IME_STARTCOMPOSITION => log::info!(""),
            WM_INITDIALOG => log::info!(""),
            WM_INITMENU => log::info!(""),
            WM_INITMENUPOPUP => log::info!(""),
            WM_INPUT => log::info!(""),
            WM_INPUTLANGCHANGE => log::info!(""),
            WM_INPUTLANGCHANGEREQUEST => log::info!(""),
            WM_INPUT_DEVICE_CHANGE => log::info!(""),
            WM_KEYDOWN => log::info!(""),
            WM_KEYFIRST => log::info!(""),
            WM_KEYLAST => log::info!(""),
            WM_KEYUP => log::info!(""),
            WM_KILLFOCUS => log::info!(""),
            WM_LBUTTONDBLCLK => log::info!(""),
            WM_LBUTTONDOWN => log::info!(""),
            WM_LBUTTONUP => log::info!(""),
            WM_MBUTTONDBLCLK => log::info!(""),
            WM_MBUTTONDOWN => log::info!(""),
            WM_MBUTTONUP => log::info!(""),
            WM_MDIACTIVATE => log::info!(""),
            WM_MDICASCADE => log::info!(""),
            WM_MDICREATE => log::info!(""),
            WM_MDIDESTROY => log::info!(""),
            WM_MDIGETACTIVE => log::info!(""),
            WM_MDIICONARRANGE => log::info!(""),
            WM_MDIMAXIMIZE => log::info!(""),
            WM_MDINEXT => log::info!(""),
            WM_MDIREFRESHMENU => log::info!(""),
            WM_MDIRESTORE => log::info!(""),
            WM_MDISETMENU => log::info!(""),
            WM_MDITILE => log::info!(""),
            WM_MEASUREITEM => log::info!(""),
            WM_MENUCHAR => log::info!(""),
            WM_MENUCOMMAND => log::info!(""),
            WM_MENUDRAG => log::info!(""),
            WM_MENUGETOBJECT => log::info!(""),
            WM_MENURBUTTONUP => log::info!(""),
            WM_MENUSELECT => log::info!(""),
            WM_MOUSEACTIVATE => log::info!(""),
            WM_MOUSEFIRST => log::info!(""),
            WM_MOUSEHWHEEL => log::info!(""),
            WM_MOUSELAST => log::info!(""),
            WM_MOUSEMOVE => log::info!(""),
            WM_MOUSEWHEEL => log::info!(""),
            WM_MOVE => log::info!(""),
            WM_MOVING => log::info!(""),
            WM_NCACTIVATE => log::info!(""),
            WM_NCCALCSIZE => log::info!(""),
            WM_NCCREATE => log::info!(""),
            WM_NCDESTROY => log::info!(""),
            WM_NCHITTEST => log::info!(""),
            WM_NCLBUTTONDBLCLK => log::info!(""),
            WM_NCLBUTTONDOWN => log::info!(""),
            WM_NCLBUTTONUP => log::info!(""),
            WM_NCMBUTTONDBLCLK => log::info!(""),
            WM_NCMBUTTONDOWN => log::info!(""),
            WM_NCMBUTTONUP => log::info!(""),
            WM_NCMOUSEHOVER => log::info!(""),
            WM_NCMOUSELEAVE => log::info!(""),
            WM_NCMOUSEMOVE => log::info!(""),
            WM_NCPAINT => log::info!(""),
            WM_NCPOINTERDOWN => log::info!(""),
            WM_NCPOINTERUP => log::info!(""),
            WM_NCPOINTERUPDATE => log::info!(""),
            WM_NCRBUTTONDBLCLK => log::info!(""),
            WM_NCRBUTTONDOWN => log::info!(""),
            WM_NCRBUTTONUP => log::info!(""),
            WM_NCXBUTTONDBLCLK => log::info!(""),
            WM_NCXBUTTONDOWN => log::info!(""),
            WM_NCXBUTTONUP => log::info!(""),
            WM_NEXTDLGCTL => log::info!(""),
            WM_NEXTMENU => log::info!(""),
            WM_NOTIFY => log::info!(""),
            WM_NOTIFYFORMAT => log::info!(""),
            WM_NULL => log::info!(""),
            WM_PAINT => log::info!(""),
            WM_PAINTCLIPBOARD => log::info!(""),
            WM_PAINTICON => log::info!(""),
            WM_PALETTECHANGED => log::info!(""),
            WM_PALETTEISCHANGING => log::info!(""),
            WM_PARENTNOTIFY => log::info!(""),
            WM_PASTE => log::info!(""),
            WM_PENWINFIRST => log::info!(""),
            WM_PENWINLAST => log::info!(""),
            WM_POINTERACTIVATE => log::info!(""),
            WM_POINTERCAPTURECHANGED => log::info!(""),
            WM_POINTERDEVICECHANGE => log::info!(""),
            WM_POINTERDEVICEINRANGE => log::info!(""),
            WM_POINTERDEVICEOUTOFRANGE => log::info!(""),
            WM_POINTERDOWN => log::info!(""),
            WM_POINTERENTER => log::info!(""),
            WM_POINTERHWHEEL => log::info!(""),
            WM_POINTERLEAVE => log::info!(""),
            WM_POINTERROUTEDAWAY => log::info!(""),
            WM_POINTERROUTEDRELEASED => log::info!(""),
            WM_POINTERROUTEDTO => log::info!(""),
            WM_POINTERUP => log::info!(""),
            WM_POINTERUPDATE => log::info!(""),
            WM_POINTERWHEEL => log::info!(""),
            WM_POWER => log::info!(""),
            WM_POWERBROADCAST => log::info!(""),
            WM_PRINT => log::info!(""),
            WM_PRINTCLIENT => log::info!(""),
            WM_QUERYDRAGICON => log::info!(""),
            WM_QUERYENDSESSION => log::info!(""),
            WM_QUERYNEWPALETTE => log::info!(""),
            WM_QUERYOPEN => log::info!(""),
            WM_QUERYUISTATE => log::info!(""),
            WM_QUEUESYNC => log::info!(""),
            WM_QUIT => log::info!(""),
            WM_RBUTTONDBLCLK => log::info!(""),
            WM_RBUTTONDOWN => log::info!(""),
            WM_RBUTTONUP => log::info!(""),
            WM_RENDERALLFORMATS => log::info!(""),
            WM_RENDERFORMAT => log::info!(""),
            WM_SETCURSOR => log::info!(""),
            WM_SETFOCUS => log::info!(""),
            WM_SETFONT => log::info!(""),
            WM_SETHOTKEY => log::info!(""),
            WM_SETICON => log::info!(""),
            WM_SETREDRAW => log::info!(""),
            WM_SETTEXT => log::info!(""),
            WM_SETTINGCHANGE => log::info!(""),
            WM_SHOWWINDOW => log::info!(""),
            WM_SIZE => log::info!(""),
            WM_SIZECLIPBOARD => log::info!(""),
            WM_SIZING => log::info!(""),
            WM_SPOOLERSTATUS => log::info!(""),
            WM_STYLECHANGED => log::info!(""),
            WM_STYLECHANGING => log::info!(""),
            WM_SYNCPAINT => log::info!(""),
            WM_SYSCHAR => log::info!(""),
            WM_SYSCOLORCHANGE => log::info!(""),
            WM_SYSCOMMAND => log::info!(""),
            WM_SYSDEADCHAR => log::info!(""),
            WM_SYSKEYDOWN => log::info!(""),
            WM_SYSKEYUP => log::info!(""),
            WM_TABLET_FIRST => log::info!(""),
            WM_TABLET_LAST => log::info!(""),
            WM_TCARD => log::info!(""),
            WM_THEMECHANGED => log::info!(""),
            WM_TIMECHANGE => log::info!(""),
            WM_TIMER => log::info!(""),
            WM_TOOLTIPDISMISS => log::info!(""),
            WM_TOUCH => log::info!(""),
            WM_TOUCHHITTESTING => log::info!(""),
            WM_UNDO => log::info!(""),
            WM_UNICHAR => log::info!(""),
            WM_UNINITMENUPOPUP => log::info!(""),
            WM_UPDATEUISTATE => log::info!(""),
            WM_USER => log::info!(""),
            WM_USERCHANGED => log::info!(""),
            WM_VKEYTOITEM => log::info!(""),
            WM_VSCROLL => log::info!(""),
            WM_VSCROLLCLIPBOARD => log::info!(""),
            WM_WINDOWPOSCHANGED => log::info!(""),
            WM_WINDOWPOSCHANGING => log::info!(""),
            WM_WININICHANGE => log::info!(""),
            WM_WTSSESSION_CHANGE => log::info!(""),
            WM_XBUTTONDBLCLK => log::info!(""),
            WM_XBUTTONDOWN => log::info!(""),
            WM_XBUTTONUP => log::info!(""),
            49306 => {
                log::info!("{:03} WM_UNDOCUMENTED_STARTUP_MSG {} {:?} {:?} {:?}",
                    ctr_base-ctr, msg.time, msg.pt, msg.lParam, msg.wParam);
                // dbg!(msg);
                // dbg!((msg.time, msg.pt, msg.lParam.0, msg.wParam.0));
                continue;
            },
            96 if (msg.lParam.0, msg.wParam.0) == (0, 1) => {
                log::info!("{:03} WM_UNDOCUMENTED_STARTUP_MSG_2:0:1 {} {:?} {:?} {:?}",
                    ctr_base-ctr, msg.time, msg.pt, msg.lParam, msg.wParam);
                // dbg!((msg.time, msg.pt, msg.lParam.0, msg.wParam.0));
                // continue;
            },
            96 if (msg.lParam.0, msg.wParam.0) == (0, 6) => {
                log::info!("{:03} WM_UNDOCUMENTED_STARTUP_MSG_2:0:6 {} {:?} {:?} {:?}",
                    ctr_base-ctr, msg.time, msg.pt, msg.lParam, msg.wParam);
                // dbg!((msg.time, msg.pt, msg.lParam.0, msg.wParam.0));
                // continue;
            },
            96 => {
                log::info!("{:03} WM_UNDOCUMENTED_STARTUP_MSG_2 {} {:?} {:?} {:?}",
                    ctr_base-ctr, msg.time, msg.pt, msg.lParam, msg.wParam);
                // continue;
            },
            // _ => {},
            _ => {log::debug!("{:0x?}", msg);log::debug!("{:?}", msg);dbg!(msg);},
        }
        
        // process input messages by translating virtual keys into characters
        let r= unsafe {TranslateMessage(&msg)}.0;
        if r != 0 {
            log::info!("TranslateMessage returned {} for message: {:?}", r, &msg);
        } else {
            match msg.message {
                49306 => {},
                96 => {},
                WM_TIMER | WM_DWMNCRENDERINGCHANGED | WM_PAINT => {},
                _ => {
                    log::info!("No character message for TranslateMessage");
                    // log::info!("{:03} TranslateMessage returned {} for message: {:?}", ctr_base-ctr, r, &msg);
                },
            }
        }
        let r = unsafe {DispatchMessageW(&msg)}.0;
        match msg.message {
            49306 => {},
            96 => {},
            WM_TIMER | WM_DWMNCRENDERINGCHANGED | WM_PAINT if r == 0 => {},
            _ => {
                log::info!("{:03} DispatchMessageW returned {}", ctr_base-ctr, r);
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
    log::info!("Loaded image dimensions: {} x {}", width, height); // Debug line
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
            log::info!("Failed to set DIB bits: {}", GetLastError().0);
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



unsafe extern "system" fn win_msg_proc(hwnd: HWND, msg: u32, w: WPARAM, l: LPARAM) -> LRESULT {
    if hwnd.is_invalid() {
        log::trace!("Invalid window handle: {:x?}", (hwnd, msg, w, l));
        return DefWindowProcW(hwnd, msg, w, l); // bonus thread message, probably
    }

    let raw = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const Window;
    // if raw.is_null() {
        log::trace!("Null window handle: {:x?}", (hwnd, msg, w, l));
        window_proc(hwnd, msg, w, l) // weird, different window source?
    // } else {
    //     let weak: ManuallyDrop<Weak<Window>> = ManuallyDrop::new(Weak::new());
    //     let weak = ManuallyDrop::new(Weak::from(raw));
    //     let Some(window) = weak.upgrade()
    //     else {
    //         return DefWindowProcW(hwnd, msg, w, l); // lost window
    //     };

    //     window.proc(msg, w, l)
    // }

    // let weak = ManuallyDrop::new(Weak::try_from(raw).unwrap());
    // let Some(window) = weak.upgrade()
    // else {
    //     return DefWindowProcW(hwnd, msg, w, l); // lost window
    // };

    // window.proc(msg, w, l)
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
        log::info!("[{:x?}] {}, {}, {}", hwnd.0, msg, wParam.0, lParam.0);
        // log::trace!("[{:x?}] {}, {}, {}", hwnd.0, msg, wParam.0, lParam.0);
    };
    // log::info!("Message {:?} received", msg);
    // log::info!("Message {:?} received", msg);
    match msg {
        WM_CLOSE => {
            print_msg("WM_CLOSE");
            DestroyWindow(hwnd).expect("DestroyWindow failed");
            return LRESULT(0);
        }

        WM_GETMINMAXINFO => {
            match wm_getminmaxinfo(hwnd, lParam) {
                Ok(_mmi) => if DBG_OPTS.show_wm_getminmaxinfo {
                    // print_msg("WM_GETMINMAXINFO");
                },
                Ok(_) => {},
                Err(e) => {
                    log::info!("Error in WM_GETMINMAXINFO: {:?}", e);
                }
            }
        }
        WM_NCCREATE => {
            match wm_create(WM_NCCREATE, hwnd, lParam) {
                Ok(_cs) => if DBG_OPTS.show_wm_nccreate {
                    // print_msg("WM_NCCREATE");
                }
                Ok(_) => {},
                Err(e) => {
                    log::info!("Error in WM_NCCREATE: {:?}", e);
                }
            }
        }
        WM_NCDESTROY => {
            print_msg("WM_NCDESTROY");
        }
        WM_NCCALCSIZE => {
            match wm_nccalcsize(hwnd, lParam) {
                Ok(_rect) => if DBG_OPTS.show_wm_nccalcsize {
                    print_msg("WM_NCCALCSIZE");
                }
                Ok(_) => {},
                Err(e) => {
                    log::info!("Error in WM_NCCALCSIZE: {:?}", e);
                }
            }
        }
        WM_CREATE => {
            match wm_create(WM_CREATE, hwnd, lParam) {
                Ok(cs) => if DBG_OPTS.show_wm_create {
                    // print_msg("WM_CREATE");
                    let mut rect = RECT {
                        left: 0,
                        top: 0,
                        right: 467,
                        bottom: 318,
                    };
                    let style = WINDOW_STYLE(cs.style as u32);
                    let exstyle = cs.dwExStyle;
                    AdjustWindowRectEx(&mut rect, style, !cs.hMenu.is_invalid(), exstyle);
                    SetWindowPos(
                        hwnd,
                        HWND(std::ptr::null_mut()),
                        0,
                        0,
                        rect.right - rect.left,
                        rect.bottom - rect.top,
                        SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE,
                    );
                },
                Ok(_) => {},
                Err(e) => {
                    log::info!("Error in WM_CREATE: {:?}", e);
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
                    log::info!("Error in WM_WINDOWPOSCHANGING: {:?}", e);
                }
            }
        }
        WM_ACTIVATEAPP => {
            if DBG_OPTS.show_wm_activateapp {
                print_msg("WM_ACTIVATEAPP");
            }
            wm_activateapp(hwnd, wParam, lParam);
            // log::info!("wParam: {:?}, lParam: {:?}", wParam, lParam);
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
            // log::info!("wParam: {:?}, lParam: {:?}", wParam, lParam);
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
        WM_WINDOWPOSCHANGED => {
            if DBG_OPTS.show_wm_windowposchanged {
                print_msg("WM_WINDOWPOSCHANGED");
            }
            wm_windowposchanged(hwnd, lParam);
        }
        WMSZ_TOPRIGHT => {
            if DBG_OPTS.show_wmsz_topright {
                print_msg("WMSZ_TOPRIGHT");
            }
            wm_sizing(WMSZ_TOPRIGHT, hwnd, wParam, lParam);
        }
        WMSZ_TOP => {
            if DBG_OPTS.show_wmsz_top {
                print_msg("WMSZ_TOP");
            }
            wm_sizing(WMSZ_TOP, hwnd, wParam, lParam);
        }
        WM_DWMNCRENDERINGCHANGED => {
            if DBG_OPTS.show_dwmncrenderingchanged {
                if wParam == WPARAM(1) {
                    print_msg("WM_DWMNCRENDERINGCHANGED on");
                } else if wParam == WPARAM(0) {
                    print_msg("WM_DWMNCRENDERINGCHANGED off");
                } else {
                    print_msg("WM_DWMNCRENDERINGCHANGED");
                }
            }
        }
        WM_PAINT => {
            if DBG_OPTS.show_wm_paint {
                print_msg("WM_PAINT");
            }
            wm_paint(hwnd);
        }
        WM_NCHITTEST => {
            if DBG_OPTS.show_wm_nchittest {
                print_msg("WM_NCHITTEST");
            }
            wm_nchittest(hwnd, wParam, lParam);
            log::info!("\n\n\t\t\tlatest");
        }


        // WM_ACTIVATE => log::info!("WM_ACTIVATE"),
        // WM_ACTIVATEAPP => log::info!("WM_ACTIVATEAPP"),
        // WM_AFXFIRST => log::info!("WM_AFXFIRST"),
        // WM_AFXLAST => log::info!("WM_AFXLAST"),
        // WM_APP => log::info!("WM_APP"),
        // WM_APPCOMMAND => log::info!("WM_APPCOMMAND"),
        // WM_ASKCBFORMATNAME => log::info!(""),
        // WM_CANCELJOURNAL => log::info!(""),
        // WM_CANCELMODE => log::info!(""),
        // WM_CAPTURECHANGED => log::info!(""),
        // WM_CHANGECBCHAIN => log::info!(""),
        // WM_CHANGEUISTATE => log::info!(""),
        // WM_CHAR => log::info!(""),
        // WM_CHARTOITEM => log::info!(""),
        // WM_CHILDACTIVATE => log::info!(""),
        // WM_CLEAR => log::info!(""),
        // WM_CLIPBOARDUPDATE => log::info!(""),
        // WM_CLOSE => log::info!(""),
        // WM_COMMAND => log::info!(""),
        // WM_COMMNOTIFY => log::info!(""),
        // WM_COMPACTING => log::info!(""),
        // WM_COMPAREITEM => log::info!(""),
        // WM_CONTEXTMENU => log::info!(""),
        // WM_COPY => log::info!(""),
        // WM_COPYDATA => log::info!(""),
        // WM_CREATE => log::info!(""),
        // WM_CTLCOLORBTN => log::info!(""),
        // WM_CTLCOLORDLG => log::info!(""),
        // WM_CTLCOLOREDIT => log::info!(""),
        // WM_CTLCOLORLISTBOX => log::info!(""),
        // WM_CTLCOLORMSGBOX => log::info!(""),
        // WM_CTLCOLORSCROLLBAR => log::info!(""),
        // WM_CTLCOLORSTATIC => log::info!(""),
        // WM_CUT => log::info!(""),
        // WM_DEADCHAR => log::info!(""),
        // WM_DELETEITEM => log::info!(""),
        // WM_DESTROY => log::info!(""),
        // WM_DESTROYCLIPBOARD => log::info!(""),
        // WM_DEVICECHANGE => log::info!(""),
        // WM_DEVMODECHANGE => log::info!(""),
        // WM_DISPLAYCHANGE => log::info!(""),
        // WM_DPICHANGED => log::info!(""),
        // WM_DPICHANGED_AFTERPARENT => log::info!(""),
        // WM_DPICHANGED_BEFOREPARENT => log::info!(""),
        // WM_DRAWCLIPBOARD => log::info!(""),
        // WM_DRAWITEM => log::info!(""),
        // WM_DROPFILES => log::info!(""),
        // WM_DWMCOLORIZATIONCOLORCHANGED => log::info!(""),
        // WM_DWMCOMPOSITIONCHANGED => log::info!(""),
        // WM_DWMNCRENDERINGCHANGED => log::info!(""),
        // WM_DWMSENDICONICLIVEPREVIEWBITMAP => log::info!(""),
        // WM_DWMSENDICONICTHUMBNAIL => log::info!(""),
        // WM_DWMWINDOWMAXIMIZEDCHANGE => log::info!(""),
        // WM_ENABLE => log::info!(""),
        // WM_ENDSESSION => log::info!(""),
        // WM_ENTERIDLE => log::info!(""),
        // WM_ENTERMENULOOP => log::info!(""),
        // WM_ENTERSIZEMOVE => log::info!(""),
        // WM_ERASEBKGND => log::info!(""),
        // WM_EXITMENULOOP => log::info!(""),
        // WM_EXITSIZEMOVE => log::info!(""),
        // WM_FONTCHANGE => log::info!(""),
        // WM_GESTURE => log::info!(""),
        // WM_GESTURENOTIFY => log::info!(""),
        // WM_GETDLGCODE => log::info!(""),
        // WM_GETDPISCALEDSIZE => log::info!(""),
        // WM_GETFONT => log::info!(""),
        // WM_GETHOTKEY => log::info!(""),
        // WM_GETICON => log::info!(""),
        // WM_GETMINMAXINFO => log::info!(""),
        // WM_GETOBJECT => log::info!(""),
        // WM_GETTEXT => log::info!(""),
        // WM_GETTEXTLENGTH => log::info!(""),
        // WM_GETTITLEBARINFOEX => log::info!(""),
        // WM_HANDHELDFIRST => log::info!(""),
        // WM_HANDHELDLAST => log::info!(""),
        // WM_HELP => log::info!(""),
        // WM_HOTKEY => log::info!(""),
        // WM_HSCROLL => log::info!(""),
        // WM_HSCROLLCLIPBOARD => log::info!(""),
        // WM_ICONERASEBKGND => log::info!(""),
        // WM_IME_CHAR => log::info!(""),
        // WM_IME_COMPOSITION => log::info!(""),
        // WM_IME_COMPOSITIONFULL => log::info!(""),
        // WM_IME_CONTROL => log::info!(""),
        // WM_IME_ENDCOMPOSITION => log::info!(""),
        // WM_IME_KEYDOWN => log::info!(""),
        // WM_IME_KEYLAST => log::info!(""),
        // WM_IME_KEYUP => log::info!(""),
        // WM_IME_NOTIFY => log::info!(""),
        // WM_IME_REQUEST => log::info!(""),
        // WM_IME_SELECT => log::info!(""),
        // WM_IME_SETCONTEXT => log::info!(""),
        // WM_IME_STARTCOMPOSITION => log::info!(""),
        // WM_INITDIALOG => log::info!(""),
        // WM_INITMENU => log::info!(""),
        // WM_INITMENUPOPUP => log::info!(""),
        // WM_INPUT => log::info!(""),
        // WM_INPUTLANGCHANGE => log::info!(""),
        // WM_INPUTLANGCHANGEREQUEST => log::info!(""),
        // WM_INPUT_DEVICE_CHANGE => log::info!(""),
        // WM_KEYDOWN => log::info!(""),
        // WM_KEYFIRST => log::info!(""),
        // WM_KEYLAST => log::info!(""),
        // WM_KEYUP => log::info!(""),
        // WM_KILLFOCUS => log::info!(""),
        // WM_LBUTTONDBLCLK => log::info!(""),
        // WM_LBUTTONDOWN => log::info!(""),
        // WM_LBUTTONUP => log::info!(""),
        // WM_MBUTTONDBLCLK => log::info!(""),
        // WM_MBUTTONDOWN => log::info!(""),
        // WM_MBUTTONUP => log::info!(""),
        // WM_MDIACTIVATE => log::info!(""),
        // WM_MDICASCADE => log::info!(""),
        // WM_MDICREATE => log::info!(""),
        // WM_MDIDESTROY => log::info!(""),
        // WM_MDIGETACTIVE => log::info!(""),
        // WM_MDIICONARRANGE => log::info!(""),
        // WM_MDIMAXIMIZE => log::info!(""),
        // WM_MDINEXT => log::info!(""),
        // WM_MDIREFRESHMENU => log::info!(""),
        // WM_MDIRESTORE => log::info!(""),
        // WM_MDISETMENU => log::info!(""),
        // WM_MDITILE => log::info!(""),
        // WM_MEASUREITEM => log::info!(""),
        // WM_MENUCHAR => log::info!(""),
        // WM_MENUCOMMAND => log::info!(""),
        // WM_MENUDRAG => log::info!(""),
        // WM_MENUGETOBJECT => log::info!(""),
        // WM_MENURBUTTONUP => log::info!(""),
        // WM_MENUSELECT => log::info!(""),
        // WM_MOUSEACTIVATE => log::info!(""),
        // WM_MOUSEFIRST => log::info!(""),
        // WM_MOUSEHWHEEL => log::info!(""),
        // WM_MOUSELAST => log::info!(""),
        // WM_MOUSEMOVE => log::info!(""),
        // WM_MOUSEWHEEL => log::info!(""),
        // WM_MOVE => log::info!(""),
        // WM_MOVING => log::info!(""),
        // WM_NCACTIVATE => log::info!(""),
        // WM_NCCALCSIZE => log::info!(""),
        // WM_NCCREATE => log::info!(""),
        // WM_NCDESTROY => log::info!(""),
        // WM_NCHITTEST => log::info!(""),
        // WM_NCLBUTTONDBLCLK => log::info!(""),
        // WM_NCLBUTTONDOWN => log::info!(""),
        // WM_NCLBUTTONUP => log::info!(""),
        // WM_NCMBUTTONDBLCLK => log::info!(""),
        // WM_NCMBUTTONDOWN => log::info!(""),
        // WM_NCMBUTTONUP => log::info!(""),
        // WM_NCMOUSEHOVER => log::info!(""),
        // WM_NCMOUSELEAVE => log::info!(""),
        // WM_NCMOUSEMOVE => log::info!(""),
        // WM_NCPAINT => log::info!(""),
        // WM_NCPOINTERDOWN => log::info!(""),
        // WM_NCPOINTERUP => log::info!(""),
        // WM_NCPOINTERUPDATE => log::info!(""),
        // WM_NCRBUTTONDBLCLK => log::info!(""),
        // WM_NCRBUTTONDOWN => log::info!(""),
        // WM_NCRBUTTONUP => log::info!(""),
        // WM_NCXBUTTONDBLCLK => log::info!(""),
        // WM_NCXBUTTONDOWN => log::info!(""),
        // WM_NCXBUTTONUP => log::info!(""),
        // WM_NEXTDLGCTL => log::info!(""),
        // WM_NEXTMENU => log::info!(""),
        // WM_NOTIFY => log::info!(""),
        // WM_NOTIFYFORMAT => log::info!(""),
        // WM_NULL => log::info!(""),
        // WM_PAINT => log::info!(""),
        // WM_PAINTCLIPBOARD => log::info!(""),
        // WM_PAINTICON => log::info!(""),
        // WM_PALETTECHANGED => log::info!(""),
        // WM_PALETTEISCHANGING => log::info!(""),
        // WM_PARENTNOTIFY => log::info!(""),
        // WM_PASTE => log::info!(""),
        // WM_PENWINFIRST => log::info!(""),
        // WM_PENWINLAST => log::info!(""),
        // WM_POINTERACTIVATE => log::info!(""),
        // WM_POINTERCAPTURECHANGED => log::info!(""),
        // WM_POINTERDEVICECHANGE => log::info!(""),
        // WM_POINTERDEVICEINRANGE => log::info!(""),
        // WM_POINTERDEVICEOUTOFRANGE => log::info!(""),
        // WM_POINTERDOWN => log::info!(""),
        // WM_POINTERENTER => log::info!(""),
        // WM_POINTERHWHEEL => log::info!(""),
        // WM_POINTERLEAVE => log::info!(""),
        // WM_POINTERROUTEDAWAY => log::info!(""),
        // WM_POINTERROUTEDRELEASED => log::info!(""),
        // WM_POINTERROUTEDTO => log::info!(""),
        // WM_POINTERUP => log::info!(""),
        // WM_POINTERUPDATE => log::info!(""),
        // WM_POINTERWHEEL => log::info!(""),
        // WM_POWER => log::info!(""),
        // WM_POWERBROADCAST => log::info!(""),
        // WM_PRINT => log::info!(""),
        // WM_PRINTCLIENT => log::info!(""),
        // WM_QUERYDRAGICON => log::info!(""),
        // WM_QUERYENDSESSION => log::info!(""),
        // WM_QUERYNEWPALETTE => log::info!(""),
        // WM_QUERYOPEN => log::info!(""),
        // WM_QUERYUISTATE => log::info!(""),
        // WM_QUEUESYNC => log::info!(""),
        // WM_QUIT => log::info!(""),
        // WM_RBUTTONDBLCLK => log::info!(""),
        // WM_RBUTTONDOWN => log::info!(""),
        // WM_RBUTTONUP => log::info!(""),
        // WM_RENDERALLFORMATS => log::info!(""),
        // WM_RENDERFORMAT => log::info!(""),
        // WM_SETCURSOR => log::info!(""),
        // WM_SETFOCUS => log::info!(""),
        // WM_SETFONT => log::info!(""),
        // WM_SETHOTKEY => log::info!(""),
        // WM_SETICON => log::info!(""),
        // WM_SETREDRAW => log::info!(""),
        // WM_SETTEXT => log::info!(""),
        // WM_SETTINGCHANGE => log::info!(""),
        // WM_SHOWWINDOW => log::info!(""),
        // WM_SIZE => log::info!(""),
        // WM_SIZECLIPBOARD => log::info!(""),
        // WM_SIZING => log::info!(""),
        // WM_SPOOLERSTATUS => log::info!(""),
        // WM_STYLECHANGED => log::info!(""),
        // WM_STYLECHANGING => log::info!(""),
        // WM_SYNCPAINT => log::info!(""),
        // WM_SYSCHAR => log::info!(""),
        // WM_SYSCOLORCHANGE => log::info!(""),
        // WM_SYSCOMMAND => log::info!(""),
        // WM_SYSDEADCHAR => log::info!(""),
        // WM_SYSKEYDOWN => log::info!(""),
        // WM_SYSKEYUP => log::info!(""),
        // WM_TABLET_FIRST => log::info!(""),
        // WM_TABLET_LAST => log::info!(""),
        // WM_TCARD => log::info!(""),
        // WM_THEMECHANGED => log::info!(""),
        // WM_TIMECHANGE => log::info!(""),
        // WM_TIMER => log::info!(""),
        // WM_TOOLTIPDISMISS => log::info!(""),
        // WM_TOUCH => log::info!(""),
        // WM_TOUCHHITTESTING => log::info!(""),
        // WM_UNDO => log::info!(""),
        // WM_UNICHAR => log::info!(""),
        // WM_UNINITMENUPOPUP => log::info!(""),
        // WM_UPDATEUISTATE => log::info!(""),
        // WM_USER => log::info!(""),
        // WM_USERCHANGED => log::info!(""),
        // WM_VKEYTOITEM => log::info!(""),
        // WM_VSCROLL => log::info!(""),
        // WM_VSCROLLCLIPBOARD => log::info!(""),
        // WM_WINDOWPOSCHANGED => log::info!(""),
        // WM_WINDOWPOSCHANGING => log::info!(""),
        // WM_WININICHANGE => log::info!(""),
        // WM_WTSSESSION_CHANGE => log::info!(""),
        // WM_XBUTTONDBLCLK => log::info!(""),
        // WM_XBUTTONDOWN => log::info!(""),
        // WM_XBUTTONUP => log::info!(""),
        _ => {
            print_msg("");
            // // log::info!("[{:x?}] 0x{:04x}, {}, {}", hwnd.0, msg, wParam.0, lParam.0);
            // // log::info!("Message {:?} received", msg);
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

/// Sent to a window when the size or position of the window is about to change. An application can use this
/// message to override the window's default maximized size and position, or its default minimum or maximum
/// tracking size.
fn wm_getminmaxinfo(hwnd: HWND, lParam: LPARAM) -> Result<MINMAXINFO> {
    if lParam == LPARAM(0) {
        return Err(Error::new(unsafe{GetLastError().into()}, "lParam is null"));
    }
    let minmaxinfo = unsafe { &mut *(lParam.0 as *mut MINMAXINFO) }.clone();
    if DBG_OPTS.show_wm_getminmaxinfo & false {
        // log::trace!("wm_getminmaxinfo: {:?}", minmaxinfo);
        log::trace!("wm_getminmaxinfo: MINMAXINFO {{");
        log::trace!("   ptReserved:     {{ {}, {} }},", minmaxinfo.ptReserved.x, minmaxinfo.ptReserved.y);
        log::trace!("   ptMaxSize:      {{ {}, {} }},", minmaxinfo.ptMaxSize.x, minmaxinfo.ptMaxSize.y);
        log::trace!("   ptMaxPosition:  {{ {}, {} }},", minmaxinfo.ptMaxPosition.x, minmaxinfo.ptMaxPosition.y);
        log::trace!("   ptMinTrackSize: {{ {}, {} }},", minmaxinfo.ptMinTrackSize.x, minmaxinfo.ptMinTrackSize.y);
        log::trace!("   ptMaxTrackSize: {{ {}, {} }},", minmaxinfo.ptMaxTrackSize.x, minmaxinfo.ptMaxTrackSize.y);
        log::trace!("}}");
    }

    Ok(minmaxinfo)
}

fn parse_string_or_atom(s: PCWSTR) -> Option<String> {
    
    if s.is_null() {
        return None;
    }
    let possible_atom = s.0 as usize;
    // println!("Possible Atom? 0x{:x}", possible_atom);
    if (possible_atom as u32) < 0x10000 {
        // panic!("Atom: 0x{:x}", possible_atom);
        // lookup the atom's string
        return Some(format!("0x{:04x}", possible_atom as u32));
    }
    let mut chars = vec![];
    unsafe {
        let mut i = 0;
        while *s.0.offset(i) != 0 {
            chars.push(*s.0.offset(i));
            i += 1;
        }
    }
    if chars.is_empty() {
        None
    } else {
        Some(String::from_utf16_lossy(&chars))
    }
    // unsafe { String::from_utf16_lossy(std::slice::from_raw_parts(s.0, 256)) }
}


// /// Sent prior to the WM_CREATE message when a window is first created.
// fn wm_nccreate(hwnd: HWND, lParam: LPARAM) -> Result<CREATESTRUCTW> {
//     if lParam == LPARAM(0) {
//         eprintln!("lParam is null");
//         return Err(Error::new(unsafe{GetLastError().into()}, "lParam is null"));
//     }
//     let cs = unsafe { &mut *(lParam.0 as *mut CREATESTRUCTW) }.clone();
// 
//     let terse = false;
//     if DBG_OPTS.show_wm_nccreate {
//         if terse {
//             log::trace!("wm_nccreate: {:?}", cs);
//         } else {
// 
//             log::trace!("wm_nccreate: CREATESTRUCTW {{");
//             log::trace!("  lpCreateParams:     0x{:08x},", cs.lpCreateParams as usize);
//             log::trace!("  hInstance:          {:?}", cs.hInstance);
//             log::trace!("  hMenu:              {:?}", cs.hMenu);
//             log::trace!("  hwndParent:         {:?}", cs.hwndParent);
//             log::trace!("  x, y, cx, cy:       {{ pos: ({}, {}) sz: ({}, {}) }}", cs.x, cs.y, cs.cx, cs.cy);
//             log::trace!("  style:              {:?}", WINDOW_STYLE(cs.style as u32));
//             log::trace!("  ext. style:         {:?}", cs.dwExStyle);
//             match parse_string_or_atom(cs.lpszName) {
//                 Some(s) => log::trace!("  name:               {}", s),
//                 None            => log::trace!("  name:               None"),
//             }
//             match parse_string_or_atom(cs.lpszClass) {
//                 Some(s) => log::trace!("  class:              {}", s),
//                 None            => log::trace!("  class:              None"),
//             }
//             log::trace!("}}");
// 
//         }
//     }
// 
//     Ok(cs)
// }

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

fn wm_create(msg: u32, hwnd: HWND, lParam: LPARAM) -> Result<CREATESTRUCTW> {

    if lParam == LPARAM(0) {
        eprintln!("lParam is null");
        return Err(Error::new(unsafe{GetLastError().into()}, "lParam is null"));
    }
    let cs = unsafe { &mut *(lParam.0 as *mut CREATESTRUCTW) }.clone();

    let (dbg_flag, wm_fn_name) = match msg {
        WM_CREATE => (DBG_OPTS.show_wm_create, "wm_create"),
        WM_NCCREATE => (DBG_OPTS.show_wm_nccreate, "wm_nccreate"),
        _ => unreachable!("wm_create called with invalid message: 0x{:x} {}", msg, msg),
    };

    let terse = false;
    if dbg_flag {
        if terse {
            log::trace!("{}: {:?}", wm_fn_name, cs);
        } else {

            log::trace!("{}: CREATESTRUCTW {{", wm_fn_name);
            log::trace!("  lpCreateParams:     0x{:08x},", cs.lpCreateParams as usize);
            log::trace!("  hInstance:          {:?}", cs.hInstance);
            log::trace!("  hMenu:              {:?}", cs.hMenu);
            log::trace!("  hwndParent:         {:?}", cs.hwndParent);
            log::trace!("  x, y, cx, cy:       pos: ({}, {}) sz: ({}, {})", cs.x, cs.y, cs.cx, cs.cy);
            log::trace!("  style:              {:?}", WINDOW_STYLE(cs.style as u32));
            log::trace!("  ext. style:         {:?}", cs.dwExStyle);
            match parse_string_or_atom(cs.lpszName) {
                Some(s) => log::trace!("  name:               {}", s),
                None            => log::trace!("  name:               None"),
            }
            match parse_string_or_atom(cs.lpszClass) {
                Some(s) => log::trace!("  class:              {}", s),
                None            => log::trace!("  class:              None"),
            }
            log::trace!("}}");

        }
    }

    Ok(cs)

    // if lParam == LPARAM(0) {
    //     log::trace!("lParam is null");
    //     return Err(Error::new(unsafe{GetLastError().into()}, "lParam is null"));
    // }
    // let cs = unsafe { &mut *(lParam.0 as *mut CREATESTRUCTW) }.clone();
    // if DBG_OPTS.show_wm_create {
    //     // log::trace!("wm_create: {:?}", cs);
    //     // if cs.hwndParent.is_invalid() {
    //     //     log::trace!("Creating root window");
    //     // }

    //     log::trace!("wm_create: {:?}", cs);

    //     log::trace!("wm_create: CREATESTRUCTW {{");
    //     log::trace!("  lpCreateParams:     0x{:08x},", cs.lpCreateParams as usize);
    //     log::trace!("  hInstance:          {:?}", cs.hInstance);
    //     log::trace!("  hMenu:              {:?}", cs.hMenu);
    //     log::trace!("  hwndParent:         {:?}", cs.hwndParent);
    //     log::trace!("  x, y, cx, cy:       {{ pos: ({}, {}) sz: ({}, {}) }}", cs.x, cs.y, cs.cx, cs.cy);
    //     log::trace!("  style:              {:?}", WINDOW_STYLE(cs.style as u32));
    //     log::trace!("  ext. style:         {:?}", cs.dwExStyle);
    //     match parse_string_or_atom(cs.lpszName) {
    //         Some(s) => log::trace!("  name:               {}", s),
    //         None            => log::trace!("  name:               None"),
    //     }
    //     match parse_string_or_atom(cs.lpszClass) {
    //         Some(s) => log::trace!("  class:              {}", s),
    //         None            => log::trace!("  class:              None"),
    //     }
    //     log::trace!("}}");

    //     // log::trace!("location  : {} x {}", cs.x, cs.y);
    //     // log::trace!("dimensions: {} x {}", cs.cx, cs.cy);
    //     // // log::trace!("style     : 0x{:x}", createstruct.style);
    //     // let style = WINDOW_STYLE(cs.style as u32);
    //     // log::trace!("style     : {:?}", style);
    //     // log::trace!("style (x) : {:?}", cs.dwExStyle);
    //     // if cs.lpszName.is_null() {
    //     //     log::trace!("name      : null");
    //     // } else {
    //     //     let name = unsafe{cs.lpszName.display()}.to_string();
    //     //     log::trace!("name      : {}", name);
    //     // }

    //     // // // TODO: the following crashes if the class is an ATOM
    //     // // let class = unsafe{cs.lpszClass.display()}.to_string();
    //     // // // let class = cs.lpszClass;
    //     // // // let class = if class.is_null() {
    //     // // //     "null".to_string()
    //     // // // } else {
    //     // // //     unsafe { String::from_utf16_lossy(std::slice::from_raw_parts(class.0, 256)) }
    //     // // // };
    //     // // log::trace!("class     : {}", class);
    // }

    // Ok(cs)
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
    if DBG_OPTS.show_wm_geticon {
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
    // if lParam == LPARAM(0) {
    //     eprintln!("lParam is null");
    //     return;
    // }
    match msg {
        WMSZ_BOTTOMLEFT => {
            // let rect = unsafe { &mut *(lParam.0 as *mut RECT) }.clone();
            if DBG_OPTS.show_wmsz_bottomleft {
                log::trace!("Window is being resized from the bottom-left corner (?)");
                log::trace!("WMSZ_BOTTOMLEFT: wParam: {}, lParam: 0x{:0x}", wParam.0, lParam.0);
            }
            return;
        }
        WMSZ_TOPRIGHT => {
            // let rect = unsafe { &mut *(lParam.0 as *mut RECT) }.clone();
            if DBG_OPTS.show_wmsz_topright {
                log::trace!("Window is being resized from the top-right corner (?)");
                log::trace!("WMSZ_TOPRIGHT: wParam: {}, lParam: 0x{:0x}", wParam.0, lParam.0);
            }
            return;
        }
        WMSZ_TOP => {
            // let rect = unsafe { &mut *(lParam.0 as *mut RECT) }.clone();
            if DBG_OPTS.show_wmsz_top {
                log::trace!("Window is being resized from the top (?)");
                log::trace!("WMSZ_TOP: wParam: {}, lParam: 0x{:0x}", wParam.0, lParam.0);
            }
        }
        _ => {
            // let rect = unsafe { &mut *(lParam.0 as *mut RECT) }.clone();
            log::trace!("wm_sizing: 0x{:x} ({}) wParam: {}, lParam: 0x{:0x}", msg, msg, wParam.0, lParam.0);
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

fn wm_windowposchanged(hwnd: HWND, lParam: LPARAM) {
    if lParam == LPARAM(0) {
        log::trace!("lParam is null");
        return;
    }
    let wp = unsafe { &mut *(lParam.0 as *mut WINDOWPOS) }.clone();
    if DBG_OPTS.show_wm_windowposchanged {
        log::trace!("wm_windowposchanged: {:?}", wp);
        log::trace!("location  : {} x {}", wp.x, wp.y);
        log::trace!("dimensions: {} x {}", wp.cx, wp.cy);
        log::trace!("flags     : 0x{:x}", wp.flags.0);
    }
}

fn wm_paint(hwnd: HWND) {
    _ = unsafe{ValidateRect(hwnd, None)};
    if DBG_OPTS.show_wm_paint {
        if hwnd.is_invalid() {
            log::trace!("wm_paint: hWND: null");
            return;
        } else {
            log::trace!("wm_paint: hWND: 0x{:08p}", hwnd.0);
        }
    }
}

fn wm_nchittest(hwnd: HWND, wParam: WPARAM, lParam: LPARAM) {
    if DBG_OPTS.show_wm_nchittest {
        let x = lParam.0 as i32;
        let y = (lParam.0 >> 16) as i32;
        log::trace!("wm_nchittest: x: {}, y: {}", x, y);
        log::trace!("wm_nchittest: wParam: 0x{:x}, lParam: 0x{:x}", wParam.0, lParam.0);
    }
}


//
