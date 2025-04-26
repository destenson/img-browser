use std::path::{Path, PathBuf};
use std::collections::HashSet;

use super::db::MediaDatabase;
use super::fs::{DirectoryInfo, list_directory, ListOptions};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct State {
    window_pos: (i32, i32),
    window_size: (u32, u32),
    state_machine: StateMachine,
    current_image: Option<ImageInfo>,
    // File navigation state
    current_directory: Option<PathBuf>,
    directory_contents: Option<DirectoryInfo>,
    selected_entry_index: Option<usize>,
    // View mode
    view_mode: ViewMode,
    // Media database
    media_db: Option<MediaDatabase>,
    // Persistent settings
    last_directories: Vec<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageInfo {
    pub path: String,
    pub dimensions: (u32, u32),
}

/// View modes for the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    /// Single image view mode
    SingleImage,
    /// Browser/grid view mode for directory contents
    Browser,
    /// Thumbnail gallery view
    Gallery,
}

impl Default for ViewMode {
    fn default() -> Self {
        ViewMode::Browser
    }
}

impl State {
    pub fn new() -> Self {
        State {
            window_pos: (0, 0),
            window_size: (800, 600),
            state_machine: StateMachine::default(),
            current_image: None,
            current_directory: None,
            directory_contents: None,
            selected_entry_index: None,
            view_mode: ViewMode::default(),
            media_db: Some(MediaDatabase::new()),
            last_directories: Vec::new(),
        }
    }
    
    pub fn set_current_image(&mut self, path: String, dimensions: (u32, u32)) {
        // Create the image info first
        let image_info = ImageInfo { 
            path: path.clone(), 
            dimensions 
        };
        
        // Set the current image
        self.current_image = Some(image_info);
        
        // Switch to single image view mode
        self.view_mode = ViewMode::SingleImage;
        
        // Mark the image as viewed in the database
        if let Some(db) = &mut self.media_db {
            let _ = db.mark_image_viewed(&path);
        }
    }
    
    pub fn get_current_image(&self) -> Option<&ImageInfo> {
        self.current_image.as_ref()
    }
    
    /// Get a reference to the current directory path, if any
    pub fn current_directory(&self) -> Option<&Path> {
        self.current_directory.as_deref()
    }
    
    /// Set the current directory and load its contents
    pub fn set_current_directory<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        let path = path.as_ref();
        
        // Load directory contents
        let contents = list_directory(path, ListOptions::All)?;
        
        // Update state
        self.current_directory = Some(path.to_path_buf());
        self.directory_contents = Some(contents);
        self.selected_entry_index = None; // Reset selection
        self.view_mode = ViewMode::Browser; // Switch to browser mode
        
        // Add to last visited directories, avoiding duplicates
        if !self.last_directories.iter().any(|p| p == path) {
            self.last_directories.push(path.to_path_buf());
            // Keep only the last 10 directories
            if self.last_directories.len() > 10 {
                self.last_directories.remove(0);
            }
        }
        
        Ok(())
    }
    
    /// Navigate to the parent directory of the current directory
    pub fn navigate_to_parent(&mut self) -> std::io::Result<bool> {
        // Get the current directory, if any
        let parent = if let Some(current_dir) = &self.current_directory {
            current_dir.parent().map(PathBuf::from)
        } else {
            None
        };
        
        // If we found a parent, set it as the current directory
        if let Some(parent_path) = parent {
            self.set_current_directory(&parent_path)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// Get the current directory contents
    pub fn directory_contents(&self) -> Option<&DirectoryInfo> {
        self.directory_contents.as_ref()
    }
    
    /// Get the selected entry index
    pub fn selected_entry_index(&self) -> Option<usize> {
        self.selected_entry_index
    }
    
    /// Set the selected entry index
    pub fn set_selected_entry_index(&mut self, index: usize) -> bool {
        if let Some(contents) = &self.directory_contents {
            if index < contents.entries.len() {
                self.selected_entry_index = Some(index);
                return true;
            }
        }
        false
    }
    
    /// Get the selected entry, if any
    pub fn selected_entry(&self) -> Option<&super::fs::DirEntry> {
        if let (Some(contents), Some(index)) = (&self.directory_contents, self.selected_entry_index) {
            contents.entries.get(index)
        } else {
            None
        }
    }
    
    /// Open the selected entry (navigate to directory or view image)
    pub fn open_selected_entry(&mut self) -> std::io::Result<bool> {
        // First get information about the selected entry
        let entry_info = if let Some(entry) = self.selected_entry() {
            // Create a copy of the relevant information we need
            Some((
                entry.path.clone(),
                entry.entry_type.clone(),
                entry.is_supported_image,
            ))
        } else {
            None
        };
        
        // Now process the entry info without borrowing self
        if let Some((path, entry_type, is_supported_image)) = entry_info {
            match entry_type {
                super::fs::EntryType::Directory => {
                    self.set_current_directory(&path)?;
                    Ok(true)
                },
                super::fs::EntryType::File => {
                    if is_supported_image {
                        // Try to get image dimensions
                        match image::image_dimensions(&path) {
                            Ok((width, height)) => {
                                self.set_current_image(
                                    path.to_string_lossy().to_string(), 
                                    (width, height)
                                );
                                Ok(true)
                            },
                            Err(e) => Err(std::io::Error::new(
                                std::io::ErrorKind::Other, 
                                format!("Failed to load image: {}", e)
                            )),
                        }
                    } else {
                        // Not a supported image file
                        Ok(false)
                    }
                }
            }
        } else {
            Ok(false)
        }
    }
    
    /// Switch to browser view mode
    pub fn switch_to_browser_mode(&mut self) {
        self.view_mode = ViewMode::Browser;
    }
    
    /// Switch to gallery view mode
    pub fn switch_to_gallery_mode(&mut self) {
        self.view_mode = ViewMode::Gallery;
    }
    
    /// Get the current view mode
    pub fn view_mode(&self) -> ViewMode {
        self.view_mode
    }
    
    /// Get a reference to the media database
    pub fn media_db(&self) -> Option<&MediaDatabase> {
        self.media_db.as_ref()
    }
    
    /// Get a mutable reference to the media database
    pub fn media_db_mut(&mut self) -> Option<&mut MediaDatabase> {
        self.media_db.as_mut()
    }
    
    /// Initialize or update the media database for the current directory
    pub fn update_media_db_for_current_directory(&mut self, recursive: bool) -> std::io::Result<usize> {
        if let Some(dir) = &self.current_directory {
            let dir_path = dir.clone();
            if let Some(db) = &mut self.media_db {
                db.scan_directory(dir_path, recursive)
            } else {
                Ok(0)
            }
        } else {
            Ok(0)
        }
    }
    
    /// Get the list of recently visited directories
    pub fn last_directories(&self) -> &[PathBuf] {
        &self.last_directories
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StateMachine {
    Init,
    Running,
    Shutdown,
}

impl Default for StateMachine {
    fn default() -> Self {
        StateMachine::Init
    }
}

impl std::fmt::Display for StateMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateMachine::Init => write!(f, "Init"),
            StateMachine::Running => write!(f, "Running"),
            StateMachine::Shutdown => write!(f, "Shutdown"),
        }
    }
}

impl std::fmt::Debug for StateMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateMachine::Init => write!(f, "Init"),
            StateMachine::Running => write!(f, "Running"),
            StateMachine::Shutdown => write!(f, "Shutdown"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_new() {
        let state = State::new();
        assert_eq!(state.window_pos, (0, 0));
        assert_eq!(state.window_size, (800, 600));
        assert_eq!(state.state_machine, StateMachine::Init);
    }
}


