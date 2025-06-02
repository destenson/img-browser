#[cfg(target_os = "windows")]
pub mod win32;

use std::sync::Arc;

use super::Result;

#[cfg(target_os = "windows")]
pub type Window = win32::Window;

// pub struct Bitmap {}

pub trait Platform {
    type Window;
    type App;
    fn run(&self, app: Self::App) -> Result<()>;
    fn create_window(&self, width: i32, height: i32) -> Result<Window>;
    fn message_loop(&self, window: Window, app: &mut Self::App) -> Result<()>;
    /// Get a path to a special folder (like Pictures, Documents, etc.)
    fn get_special_folder(&self, folder_type: SpecialFolder) -> Option<std::path::PathBuf>;
    /// Create a directory and all parent directories if needed
    fn create_directory(&self, path: &std::path::Path) -> Result<()> {
        std::fs::create_dir(path).map_err(Into::into)
    }
    // fn load_image_as_bitmap(&self, path: &str) -> Result<(Bitmap, i32, i32)>;
    
    fn directory_exists(&self, path: &std::path::Path) -> bool {
        std::fs::metadata(path).map(|m| m.is_dir()).unwrap_or(false)
    }
}

/// Types of special folders that can be accessed through the platform layer
pub enum SpecialFolder {
    Documents,
    Pictures,
    Videos,
    Music,
    Downloads,
    Desktop,
    AppData,
}

pub fn main(args: std::env::Args) -> super::Result<()> {

    // Create the app
    let app = super::App::new(args);

    // Create the platorm abstraction layer
    let platform = win32::Platform {};

    // Run the app
    let result = app.run(platform);

    // Handle the result
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
    
    // // Load the image as a bitmap
    // let (image_bmp, width, height) = load_image_as_bitmap("assets/image.png");

    // // Create the window
    // let window = create_window(width, height).expect("Failed to create window");

    // // Main message loop
    // let _ = message_loop(window, image_bmp, width, height).expect("Message loop failed");
    
    Ok(())
}

