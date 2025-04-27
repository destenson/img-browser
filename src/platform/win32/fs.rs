// Windows-specific filesystem utilities
use std::path::PathBuf;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use windows::Win32::UI::Shell::{SHGetKnownFolderPath, FOLDERID_Pictures, FOLDERID_Documents, FOLDERID_Videos, FOLDERID_Music, FOLDERID_Downloads, FOLDERID_Desktop, FOLDERID_RoamingAppData};
use windows::core::{GUID, PWSTR};
use crate::platform::win32::dialogs::wcslen;

use super::super::SpecialFolder;

/// Get path to a known folder in Windows using the Shell API
pub fn get_known_folder_path(folder_id: GUID) -> windows::core::Result<PathBuf> {
    fn pwstr2path(pwstr: PWSTR) -> PathBuf {
        let len = unsafe { wcslen(pwstr.as_ptr()) };
        let slice = unsafe { std::slice::from_raw_parts(pwstr.as_ptr(), len) };
        let os_string = OsString::from_wide(slice);
        PathBuf::from(os_string)
    }
    unsafe {
        let path_ptr = SHGetKnownFolderPath(
            &folder_id,
            windows::Win32::UI::Shell::KNOWN_FOLDER_FLAG(0),
            None,
        )?;
        
        let path = pwstr2path(path_ptr);
        
        Ok(path)
    }
}

/// Get a special folder path based on the folder type
pub fn get_special_folder_path(folder_type: SpecialFolder) -> Option<PathBuf> {
    let folder_id = match folder_type {
        SpecialFolder::Pictures => FOLDERID_Pictures,
        SpecialFolder::Documents => FOLDERID_Documents,
        SpecialFolder::Videos => FOLDERID_Videos,
        SpecialFolder::Music => FOLDERID_Music,
        SpecialFolder::Downloads => FOLDERID_Downloads,
        SpecialFolder::Desktop => FOLDERID_Desktop,
        SpecialFolder::AppData => FOLDERID_RoamingAppData,
    };
    
    get_known_folder_path(folder_id).inspect_err(|e| {
        log::error!("Failed to get special folder path: {}", e);
    }).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_known_folder_path() {
        // Test getting the Pictures folder
        let pictures = get_known_folder_path(FOLDERID_Pictures);
        assert!(pictures.is_ok(), "Failed to get Pictures folder");
        println!("Pictures folder: {:?}", pictures);
        
        // Test getting the Documents folder
        let documents = get_known_folder_path(FOLDERID_Documents);
        assert!(documents.is_ok(), "Failed to get Documents folder");
        println!("Documents folder: {:?}", documents);
    }
}

