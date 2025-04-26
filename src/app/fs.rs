///! filesystem operations

use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::collections::HashSet;

/// Supported image file extensions
pub const SUPPORTED_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "webp", "bmp", "gif", "tiff", "tif"
];

/// Entry type in a directory (file or directory)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryType {
    File,
    Directory,
}

/// Represents an entry in a directory (file or subdirectory)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirEntry {
    /// The path to the entry
    pub path: PathBuf,
    /// The entry name (file or directory name)
    pub name: String,
    /// The type of entry (file or directory)
    pub entry_type: EntryType,
    /// For files, indicates if it's a supported image format
    pub is_supported_image: bool,
}

impl DirEntry {
    /// Create a new directory entry from a path and type
    fn new(path: PathBuf, entry_type: EntryType) -> Self {
        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| String::from(""));
        
        let is_supported_image = if entry_type == EntryType::File {
            path.extension()
                .map(|ext| {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    SUPPORTED_EXTENSIONS.contains(&ext_str.as_str())
                })
                .unwrap_or(false)
        } else {
            false
        };
        
        Self {
            path,
            name,
            entry_type,
            is_supported_image,
        }
    }
}

/// Metadata about a directory
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryInfo {
    /// The path to the directory
    pub path: PathBuf,
    /// The entries in the directory (files and subdirectories)
    pub entries: Vec<DirEntry>,
    /// Parent directory path, if any
    pub parent: Option<PathBuf>,
    /// Number of image files in the directory
    pub image_count: usize,
    /// Number of subdirectories in the directory
    pub subdir_count: usize,
}

/// The type of entries to list in a directory
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListOptions {
    /// List all entries (files and directories)
    All,
    /// List only directories
    DirectoriesOnly,
    /// List only supported image files
    ImagesOnly,
}

/// List the contents of a directory
pub fn list_directory(path: impl AsRef<Path>, options: ListOptions) -> io::Result<DirectoryInfo> {
    let path = path.as_ref();
    let mut entries = Vec::new();
    let mut image_count = 0;
    let mut subdir_count = 0;
    
    // Get the parent directory, if any
    let parent = path.parent().map(PathBuf::from);
    
    // Read the directory contents
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        
        if file_type.is_dir() {
            subdir_count += 1;
            if options != ListOptions::ImagesOnly {
                entries.push(DirEntry::new(path, EntryType::Directory));
            }
        } else if file_type.is_file() {
            // Check if the file is a supported image format
            let is_image = path.extension()
                .map(|ext| {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    SUPPORTED_EXTENSIONS.contains(&ext_str.as_str())
                })
                .unwrap_or(false);
            
            if is_image {
                image_count += 1;
                if options != ListOptions::DirectoriesOnly {
                    entries.push(DirEntry::new(path, EntryType::File));
                }
            } else if options == ListOptions::All {
                entries.push(DirEntry::new(path, EntryType::File));
            }
        }
    }
    
    // Sort entries: directories first, then files, both alphabetically
    entries.sort_by(|a, b| {
        match (&a.entry_type, &b.entry_type) {
            (EntryType::Directory, EntryType::File) => std::cmp::Ordering::Less,
            (EntryType::File, EntryType::Directory) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });
    
    Ok(DirectoryInfo {
        path: path.to_path_buf(),
        entries,
        parent,
        image_count,
        subdir_count,
    })
}

/// Check if a directory contains any supported image files
pub fn contains_images(path: impl AsRef<Path>) -> io::Result<bool> {
    let path = path.as_ref();
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            let file_path = entry.path();
            if let Some(ext) = file_path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if SUPPORTED_EXTENSIONS.contains(&ext_str.as_str()) {
                    return Ok(true);
                }
            }
        }
    }
    
    Ok(false)
}

/// Scan a directory recursively for all image files
pub fn scan_directory_recursive(path: impl AsRef<Path>) -> io::Result<Vec<PathBuf>> {
    let path = path.as_ref();
    let mut image_files = Vec::new();
    
    // Create a set of supported extensions for fast lookup
    let extensions: HashSet<_> = SUPPORTED_EXTENSIONS.iter().map(|&s| s.to_string()).collect();
    
    // Helper function to recursively scan directories
    fn scan_dir_recursive(
        dir: &Path, 
        image_files: &mut Vec<PathBuf>,
        extensions: &HashSet<String>,
    ) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if entry.file_type()?.is_dir() {
                scan_dir_recursive(&path, image_files, extensions)?;
            } else if entry.file_type()?.is_file() {
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    if extensions.contains(&ext_str) {
                        image_files.push(path);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    scan_dir_recursive(path, &mut image_files, &extensions)?;
    
    // Sort images alphabetically by full path
    image_files.sort_by(|a, b| {
        a.to_string_lossy().to_lowercase().cmp(&b.to_string_lossy().to_lowercase())
    });
    
    Ok(image_files)
}

/// Navigate to a parent directory if it exists
pub fn navigate_up(current_path: impl AsRef<Path>) -> Option<PathBuf> {
    current_path.as_ref().parent().map(PathBuf::from)
}

/// Check if a file is a supported image format
pub fn is_supported_image(path: impl AsRef<Path>) -> bool {
    path.as_ref()
        .extension()
        .map(|ext| {
            let ext_str = ext.to_string_lossy().to_lowercase();
            SUPPORTED_EXTENSIONS.contains(&ext_str.as_str())
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_supported_image() {
        assert!(is_supported_image("test.jpg"));
        assert!(is_supported_image("test.jpeg"));
        assert!(is_supported_image("test.png"));
        assert!(is_supported_image("test.webp"));
        assert!(is_supported_image("test.bmp"));
        assert!(is_supported_image("test.gif"));
        assert!(!is_supported_image("test.txt"));
        assert!(!is_supported_image("test"));
    }
}


