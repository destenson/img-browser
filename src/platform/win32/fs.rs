// Windows-specific filesystem utilities
use std::path::PathBuf;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use windows::Win32::UI::Shell::{SHGetKnownFolderPath, FOLDERID_Pictures, FOLDERID_Documents, FOLDERID_Videos, FOLDERID_Music, FOLDERID_Downloads, FOLDERID_Desktop, FOLDERID_RoamingAppData};
use windows::core::{GUID, PWSTR};
use crate::platform::win32::dialogs::wcslen;
use windows::Win32::Storage::FileSystem::{CreateDirectoryW, GetFileAttributesW, FILE_ATTRIBUTE_DIRECTORY};
use windows::Win32::Foundation::{ERROR_ALREADY_EXISTS, GetLastError};

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

/// Create a directory using Windows API directly
/// Will create all intermediate directories as needed
pub fn create_directory_windows(path: &std::path::Path) -> windows::core::Result<()> {
    // Convert the path to a UTF-16 encoded string that Windows APIs expect
    let wide_path = path.to_string_lossy().encode_utf16().collect::<Vec<u16>>();
    
    // Terminate with a null character
    let mut wide_path_null = wide_path.clone();
    wide_path_null.push(0);
    
    // Check if the directory already exists
    unsafe {
        let attrs = GetFileAttributesW(windows::core::PCWSTR::from_raw(wide_path_null.as_ptr()));
        if attrs != u32::MAX && attrs & (FILE_ATTRIBUTE_DIRECTORY.0 as u32) != 0 {
            // Directory already exists
            return Ok(());
        }
        
        // Create the directory
        let result = CreateDirectoryW(
            windows::core::PCWSTR::from_raw(wide_path_null.as_ptr()),
            None, // No security attributes
        ).inspect_err(|e| {
            log::error!("Failed to create directory: {}", e);
        })?;
        
        let error = GetLastError();
        if error == ERROR_ALREADY_EXISTS {
            // This is fine - directory already exists
            return Ok(());
        }
        
        // Try creating parent directories first
        if let Some(parent) = path.parent() {
            if parent.as_os_str().len() > 0 {
                create_directory_windows(parent)?;
                
                // Try creating the directory again
                CreateDirectoryW(
                    windows::core::PCWSTR::from_raw(wide_path_null.as_ptr()),
                    None,
                ).map_err(|e| {
                    let error = GetLastError();
                    if error != ERROR_ALREADY_EXISTS {
                        return windows::core::Error::from_win32();
                    } else {
                        return e;
                    }
                })?;
            }
        }
    }
    
    Ok(())
}

pub fn directory_exists_windows(path: &std::path::Path) -> bool {
    // Convert the path to a UTF-16 encoded string that Windows APIs expect
    let wide_path = path.to_string_lossy().encode_utf16().collect::<Vec<u16>>();
    
    // Terminate with a null character
    let mut wide_path_null = wide_path.clone();
    wide_path_null.push(0);
    
    // Check if the directory exists
    unsafe {
        let attrs = GetFileAttributesW(windows::core::PCWSTR::from_raw(wide_path_null.as_ptr()));
        if attrs != u32::MAX && attrs & (FILE_ATTRIBUTE_DIRECTORY.0 as u32) != 0 {
            return true; // Directory exists
        }
    }
    
    false // Directory does not exist
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

