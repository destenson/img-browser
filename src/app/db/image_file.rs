#![allow(unused)]

use std::{collections::HashSet, path::PathBuf, time::UNIX_EPOCH};

use std::{io, fs};

use serde::{Deserialize, Serialize};

use crate::{Result, Error};

/// Represents a single image file in the database
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageFile {
    /// Path to the image file
    pub path: PathBuf,
    /// File size in bytes
    pub size: u64,
    /// Last modified timestamp (seconds since epoch)
    pub modified: u64,
    /// Whether the file has been viewed before
    pub viewed: bool,
    /// Custom tags applied to the image
    pub tags: HashSet<String>,
    /// Favorite status
    pub favorite: bool,
    /// File hash for duplicate detection and file integrity/change detection
    pub file_hash: Vec<u8>,
}

impl ImageFile {
    /// Create a new ImageFile from a path
    pub fn new(path: PathBuf) -> Result<Self> {
        let metadata = fs::metadata(&path)?;
        
        let modified = metadata.modified()?
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
            
        Ok(Self {
            path: path.clone(),
            size: metadata.len(),
            modified,
            viewed: false,
            tags: HashSet::new(),
            favorite: false,
            file_hash: hash_file(&path)?,
        })
    }
    
    pub fn name(&self) -> String {
        self.path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default()
    }
    
    pub fn extension(&self) -> String {
        self.path.extension()
            .map(|ext| ext.to_string_lossy().to_lowercase())
            .unwrap_or_default()
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

fn hash_file(path: &PathBuf) -> Result<Vec<u8>> {
    use std::hash::{Hash, Hasher};
    use std::io::Read;
    use std::fs::File;
    use std::collections::hash_map::DefaultHasher;
    
    let mut file = File::open(path)?;
    let mut hasher = DefaultHasher::new();
    
    let mut buffer = [0; 1024];
    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        hasher.write(&buffer[..n]);
    }
    
    let hash = hasher.finish();
    let hash_bytes = hash.to_be_bytes().to_vec();
    
    Ok(hash_bytes)
}
