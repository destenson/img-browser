pub mod config;
pub mod db;
pub mod error;
pub mod fs;
pub mod settings;
pub mod state;

// Every app has a state and a configuration.
pub use config::Config;
pub use state::State;

pub use error::{Error, Result};

use crate::platform::Platform;
use std::path::{Path, PathBuf};
use std::env;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct App {
    pub config: Config,
    pub state: State,
}

impl App {
    pub fn new(args: std::env::Args) -> Self {
        // Parse command line arguments
        let config = Config::from_args(args);
        let mut state = State::new();
        
        // Create the app
        let mut app = App { config, state };
        
        // Initialize file system navigation based on config
        app.initialize_navigation();
        
        app
    }
    
    /// Initialize file system navigation based on config
    fn initialize_navigation(&mut self) {
        // If a directory was specified, use it
        if let Some(dir) = &self.config.directory {
            if let Err(e) = self.state.set_current_directory(dir) {
                log::error!("Failed to set directory {}: {}", dir.display(), e);
            } else {
                // If gallery mode is enabled, update the view mode
                if self.config.gallery {
                    self.state.switch_to_gallery_mode();
                }
                
                // Scan directory for images
                if let Err(e) = self.state.update_media_db_for_current_directory(self.config.recursive) {
                    log::error!("Failed to scan directory {}: {}", dir.display(), e);
                }
            }
        } 
        // If no directory specified but an image path was provided, use its parent directory
        else if let Some(img_path) = &self.config.image_path {
            let path = Path::new(img_path);
            if let Some(parent) = path.parent() {
                if let Err(e) = self.state.set_current_directory(parent) {
                    log::error!("Failed to set directory {}: {}", parent.display(), e);
                } else {
                    // Scan directory for images
                    if let Err(e) = self.state.update_media_db_for_current_directory(self.config.recursive) {
                        log::error!("Failed to scan directory {}: {}", parent.display(), e);
                    }
                }
            }
        } 
        // If no directory or image specified, use the current directory
        else {
            if let Ok(current_dir) = env::current_dir() {
                if let Err(e) = self.state.set_current_directory(&current_dir) {
                    log::error!("Failed to set current directory {}: {}", current_dir.display(), e);
                } else {
                    // Scan directory for images
                    if let Err(e) = self.state.update_media_db_for_current_directory(self.config.recursive) {
                        log::error!("Failed to scan directory {}: {}", current_dir.display(), e);
                    }
                    
                    // If gallery mode is enabled, update the view mode
                    if self.config.gallery {
                        self.state.switch_to_gallery_mode();
                    }
                }
            }
        }
    }
}

impl App {
    pub fn run<P: Platform<App = Self, Window = crate::platform::Window>>(self, platform: P) -> Result<()> {
        {
            let Self { config, state } = &self;
            
            // log the configuration
            log::debug!("Config: {}", self.config);
            // log the state
            log::debug!("State: {}", self.state);
        }
        
        platform.run(self)
    }
}

/*
TODO:

- [ ] Add clap for argument parsing
- [ ] Add logging
- [ ] Add error handling
- [ ] Add a window
- [ ] Add a message loop
- [ ] Add a bitmap
- [ ] Add a bitmap blit
- [ ] Add a bitmap from an image
- [ ] Add a bitmap from a file
- [ ] Add a bitmap from a resource
- [ ] Add a bitmap from a URL
- [ ] Add a bitmap from a buffer
- [ ] Add a bitmap from a stream
- [ ] Add multiple bitmaps
- [ ] Add directory scanning
- [ ] Add file scanning
- [ ] Add image comparison
- [ ] Add image diffing
- [ ] Add image processing
- [ ] Add image manipulation
- [ ] Add image transformation
- [ ] Add image scaling
- [ ] Add image tagging
- [ ] Add image metadata
- [ ] Add image generation
- [ ] Add image cataloging
- [ ] Add image searching
- [ ] Add image sorting

*/
