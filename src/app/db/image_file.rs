use std::{collections::HashSet, path::PathBuf, time::UNIX_EPOCH};

use std::{io, fs};


/// Represents a single image file in the database
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageFile {
    /// Path to the image file
    pub path: PathBuf,
    #[deprecated]
    /// File name with extension (deprecated because path already contains this information)
    pub name: String,
    #[deprecated]
    /// File extension (jpg, png, etc.) (deprecated because path already contains this information)
    pub extension: String,
    #[deprecated]
    /// File size in bytes (deprecated because path already contains this information)
    pub size: u64,
    #[deprecated]
    /// Last modified timestamp (seconds since epoch) (deprecated because path already contains this information)
    pub modified: u64,
    /// Whether the file has been viewed before
    pub viewed: bool,
    /// Custom tags applied to the image
    pub tags: HashSet<String>,
    /// Favorite status
    pub favorite: bool,
}

impl ImageFile {
    /// Create a new ImageFile from a path
    pub fn new(path: PathBuf) -> io::Result<Self> {
        let metadata = fs::metadata(&path)?;
        
        let name = path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
            
        let extension = path.extension()
            .map(|ext| ext.to_string_lossy().to_lowercase())
            .unwrap_or_default();
            
        let modified = metadata.modified()?
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
            
        Ok(Self {
            path,
            name,
            extension,
            size: metadata.len(),
            modified,
            viewed: false,
            tags: HashSet::new(),
            favorite: false,
        })
    }
    
    /// Mark the image as viewed
    pub fn mark_viewed(&mut self) {
        self.viewed = true;
    }
    
    /// Add a tag to the image
    pub fn add_tag(&mut self, tag: String) {
        self.tags.insert(tag);
    }
    
    /// Remove a tag from the image
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.remove(tag);
    }
    
    /// Toggle favorite status
    pub fn toggle_favorite(&mut self) {
        self.favorite = !self.favorite;
    }
}

