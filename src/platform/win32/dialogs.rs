use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

use windows::core::*;
use windows::Win32::System::Com::CoCreateInstance;
use windows::Win32::System::Com::CoInitializeEx;
use windows::Win32::System::Com::CoTaskMemFree;
use windows::Win32::System::Com::CLSCTX_INPROC_SERVER;
use windows::Win32::System::Com::COINIT_APARTMENTTHREADED;
use windows::Win32::System::Com::COINIT_DISABLE_OLE1DDE;
use windows::Win32::UI::Controls::*;
use windows::Win32::UI::Shell::*;
use windows::Win32::UI::Shell::Common::*;
use windows::Win32::Foundation::*;

/// Opens a folder browser dialog and returns the selected path
pub fn open_folder_dialog(hwnd: HWND, title: &str) -> Result<Option<PathBuf>> {
    unsafe {
        // Initialize COM
        CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE).ok()?;
        
        // Create the FileOpenDialog instance
        let dialog: IFileOpenDialog = CoCreateInstance(
            &FileOpenDialog,
            None,
            CLSCTX_INPROC_SERVER
        )?;
        
        // Set the dialog title
        let title: HSTRING = title.into();
        dialog.SetTitle(&title)?;
        
        // Set options to pick folders
        let options = dialog.GetOptions()?;
        dialog.SetOptions(options | FOS_PICKFOLDERS)?;
        
        // Show the dialog
        let result = dialog.Show(Some(hwnd));
        
        if let Err(error) = &result {
            // User cancelled the dialog
            if error.code() == HRESULT(SDIAG_E_CANCELLED) {
                return Ok(None);
            }
            // Some other error occurred
            return Err(error.clone());
        }
        
        // Get the selected item
        let item = dialog.GetResult()?;
        
        // Get the path
        let path_raw = item.GetDisplayName(SIGDN_FILESYSPATH)?;
        
        // Convert to PathBuf
        let path_buf = get_path_from_pwstr(path_raw);
        
        // Free the PWSTR
        CoTaskMemFree(Some(path_raw.as_ptr() as _));
        
        // Return the path
        Ok(Some(path_buf))
    }
}

/// Opens a file open dialog and returns the selected path
pub fn open_file_dialog(
    hwnd: HWND, 
    title: &str, 
    filter_name: &str, 
    filter_spec: &str
) -> Result<Option<PathBuf>> {
    unsafe {
        // Initialize COM
        CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE).ok()?;
        
        // Create the FileOpenDialog instance
        let dialog: IFileOpenDialog = CoCreateInstance(
            &FileOpenDialog,
            None,
            CLSCTX_INPROC_SERVER
        )?;
        
        // Set the dialog title
        let title: HSTRING = title.into();
        dialog.SetTitle(&title)?;
        
        // Set file filter
        if !filter_spec.is_empty() {
            let filter_name: PCWSTR = PCWSTR::from_raw(to_wide_string(filter_name).as_ptr());
            let filter_spec: PCWSTR = PCWSTR::from_raw(to_wide_string(filter_spec).as_ptr());
            
            let filter = COMDLG_FILTERSPEC {
                pszName: filter_name,
                pszSpec: filter_spec,
            };
            
            dialog.SetFileTypes(&[filter])?;
        }
        
        // Show the dialog
        let result = dialog.Show(Some(hwnd));
        
        if let Err(error) = &result {
            // User cancelled the dialog
            if error.code() == HRESULT(SDIAG_E_CANCELLED) {
                return Ok(None);
            }
            // Some other error occurred
            return Err(error.clone());
        }
        
        // Get the selected item
        let item = dialog.GetResult()?;
        
        // Get the path
        let path_raw = item.GetDisplayName(SIGDN_FILESYSPATH)?;
        
        // Convert to PathBuf
        let path_buf = get_path_from_pwstr(path_raw);
        
        // Free the PWSTR
        CoTaskMemFree(Some(path_raw.as_ptr() as _));
        
        // Return the path
        Ok(Some(path_buf))
    }
}

/// Helper function to convert PWSTR to PathBuf
fn get_path_from_pwstr(pwstr: PWSTR) -> PathBuf {
    let len = unsafe { wcslen(pwstr.as_ptr()) };
    let slice = unsafe { std::slice::from_raw_parts(pwstr.as_ptr(), len) };
    let os_string = OsString::from_wide(slice);
    PathBuf::from(os_string)
}

/// Helper function to convert &str to Vec<u16> (wide string)
fn to_wide_string(s: &str) -> Vec<u16> {
    // add null terminator
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Helper function to get length of wide string
pub unsafe fn wcslen(s: *const u16) -> usize {
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    len
}
