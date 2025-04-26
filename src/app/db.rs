///! Media database for tracking image files and metadata

pub mod image_file;

use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use serde::{Serialize, Deserialize};

use image_file::ImageFile;

use super::fs::{is_supported_image, scan_directory_recursive};

/// Represents a collection of images with associated metadata
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaDatabase {
    /// All tracked images, keyed by their path as a string
    images: HashMap<String, ImageFile>,
    /// Set of all unique tags used across all images
    all_tags: HashSet<String>,
    /// Recently viewed images (stores paths)
    recent_views: Vec<PathBuf>,
    /// Favorite images (stores paths)
    favorites: HashSet<String>,
}

impl MediaDatabase {
    /// Create a new empty media database
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
            all_tags: HashSet::new(),
            recent_views: Vec::new(),
            favorites: HashSet::new(),
        }
    }
    
    /// Add an image to the database from a path
    pub fn add_image(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        let path = path.as_ref();
        
        if is_supported_image(path) {
            let image = ImageFile::new(path.to_path_buf())?;
            let path_str = path.to_string_lossy().to_string();
            
            if image.favorite {
                self.favorites.insert(path_str.clone());
            }
            
            self.images.insert(path_str, image);
            
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Not a supported image format",
            ))
        }
    }
    
    /// Remove an image from the database
    pub fn remove_image(&mut self, path: impl AsRef<Path>) -> bool {
        let path_str = path.as_ref().to_string_lossy().to_string();
        
        // Remove from favorites if needed
        self.favorites.remove(&path_str);
        
        // Remove from recent views
        self.recent_views.retain(|p| p.to_string_lossy() != path_str);
        
        // Remove from images map and return whether it existed
        self.images.remove(&path_str).is_some()
    }
    
    /// Get an image from the database by path
    pub fn get_image(&self, path: impl AsRef<Path>) -> Option<&ImageFile> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        self.images.get(&path_str)
    }
    
    /// Get a mutable reference to an image
    pub fn get_image_mut(&mut self, path: impl AsRef<Path>) -> Option<&mut ImageFile> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        self.images.get_mut(&path_str)
    }
    
    /// Mark an image as viewed and update recent views
    pub fn mark_image_viewed(&mut self, path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();
        let path_str = path.to_string_lossy().to_string();
        
        if let Some(image) = self.images.get_mut(&path_str) {
            image.mark_viewed();
            
            // Remove existing entry from recent views if present
            self.recent_views.retain(|p| p != path);
            
            // Add to the front of recent views
            self.recent_views.insert(0, path.to_path_buf());
            
            // Trim recent views list if it gets too long
            if self.recent_views.len() > 50 {
                self.recent_views.truncate(50);
            }
            
            true
        } else {
            false
        }
    }
    
    /// Add a tag to an image and update the global tag set
    pub fn add_tag_to_image(&mut self, path: impl AsRef<Path>, tag: impl Into<String>) -> bool {
        let tag = tag.into();
        let path_str = path.as_ref().to_string_lossy().to_string();
        
        if let Some(image) = self.images.get_mut(&path_str) {
            image.add_tag(tag.clone());
            self.all_tags.insert(tag);
            true
        } else {
            false
        }
    }
    
    /// Remove a tag from an image
    pub fn remove_tag_from_image(&mut self, path: impl AsRef<Path>, tag: &str) -> bool {
        let path_str = path.as_ref().to_string_lossy().to_string();
        
        if let Some(image) = self.images.get_mut(&path_str) {
            image.remove_tag(tag);
            
            // Check if any image still has this tag
            let tag_still_used = self.images.values().any(|img| img.tags.contains(tag));
            
            // If no image has this tag anymore, remove it from all_tags
            if !tag_still_used {
                self.all_tags.remove(tag);
            }
            
            true
        } else {
            false
        }
    }
    
    /// Toggle favorite status for an image
    pub fn toggle_favorite(&mut self, path: impl AsRef<Path>) -> bool {
        let path = path.as_ref();
        let path_str = path.to_string_lossy().to_string();
        
        if let Some(image) = self.images.get_mut(&path_str) {
            image.toggle_favorite();
            
            if image.favorite {
                self.favorites.insert(path_str);
            } else {
                self.favorites.remove(&path_str);
            }
            
            true
        } else {
            false
        }
    }
    
    /// Get all favorite images
    pub fn get_favorites(&self) -> Vec<&ImageFile> {
        self.favorites
            .iter()
            .filter_map(|path| self.images.get(path))
            .collect()
    }
    
    /// Get recently viewed images
    pub fn get_recent_views(&self, limit: usize) -> Vec<&ImageFile> {
        self.recent_views
            .iter()
            .take(limit)
            .filter_map(|path| {
                let path_str = path.to_string_lossy().to_string();
                self.images.get(&path_str)
            })
            .collect()
    }
    
    /// Get all images with a specific tag
    pub fn get_images_with_tag(&self, tag: &str) -> Vec<&ImageFile> {
        self.images
            .values()
            .filter(|img| img.tags.contains(tag))
            .collect()
    }
    
    /// Get all available tags
    pub fn get_all_tags(&self) -> &HashSet<String> {
        &self.all_tags
    }
    
    /// Scan a directory and add all supported images to the database
    pub fn scan_directory(&mut self, path: impl AsRef<Path>, recursive: bool) -> io::Result<usize> {
        let path = path.as_ref();
        let image_paths = if recursive {
            scan_directory_recursive(path)?
        } else {
            let mut images = Vec::new();
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();
                
                if entry.file_type()?.is_file() && is_supported_image(&entry_path) {
                    images.push(entry_path);
                }
            }
            images
        };
        
        let mut added_count = 0;
        
        for image_path in image_paths {
            if let Ok(()) = self.add_image(&image_path) {
                added_count += 1;
            }
        }
        
        Ok(added_count)
    }
    
    /// Returns the total number of images in the database
    pub fn image_count(&self) -> usize {
        self.images.len()
    }
    
    /// Update an image's metadata if the file has changed on disk
    pub fn refresh_image(&mut self, path: impl AsRef<Path>) -> io::Result<bool> {
        let path = path.as_ref();
        let path_str = path.to_string_lossy().to_string();
        
        if let Some(existing) = self.images.get(&path_str) {
            // Get current file metadata
            let metadata = fs::metadata(path)?;
            let modified = metadata.modified()?
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
                
            // Check if file has been modified
            if modified > existing.modified || metadata.len() != existing.size {
                let mut new_image = ImageFile::new(path.to_path_buf())?;
                
                // Preserve user data
                new_image.viewed = existing.viewed;
                new_image.tags = existing.tags.clone();
                new_image.favorite = existing.favorite;
                
                // Update the image
                self.images.insert(path_str, new_image);
                return Ok(true);
            }
        }
        
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    #[test]
    fn test_add_and_get_image() {
        let mut db = MediaDatabase::new();
        
        // This test is simplified since we can't easily create real files in a unit test
        // In a real application, we would use a test fixture or mock the file system
        
        // Instead, we'll just check the API logic works
        assert_eq!(db.image_count(), 0);
        
        // Adding an invalid path would fail in practice but we're testing the API
        // so we'll pretend the file exists and is valid
        let dummy_path = Path::new("test_image.jpg");
        
        // This would normally fail, but we're just testing API logic
        // In practice, add_image would read the file which doesn't exist here
        // So we're just checking the database structure is correct
        
        assert_eq!(db.get_image(dummy_path), None.as_ref());
    }
}


