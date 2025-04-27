#![allow(unused)]
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[clap(name = "img-browser", about = "Image browser and organizer")]
pub struct Config {
    /// The width of the window.
    #[clap(short, long, default_value = "800")]
    pub width: u32,
    
    /// The height of the window.
    #[clap(short = 'H', long, default_value = "600")]
    pub height: u32,
    
    /// Optional path to an image file to load.
    #[clap(name = "IMAGE_PATH")]
    pub image_path: Option<String>,
    
    /// Optional directory to browse (overrides default)
    #[clap(short, long, name = "DIRECTORY")]
    pub directory: Option<PathBuf>,
    
    /// Recursively scan directories for images
    #[clap(short, long, default_value = "true")]
    pub recursive: bool,
    
    /// Start in gallery mode (showing all images in the directory)
    #[clap(short, long)]
    pub gallery: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width: 800,
            height: 600,
            image_path: None,
            directory: None,
            recursive: true,
            gallery: false,
        }
    }
}

impl Config {
    pub fn from_args(args: std::env::Args) -> Self {
        Self::parse_from(args)
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{:?}", self)
            // write!(f, "Config {{ width: {}, height: {}, image_path: {:?}, directory: {:?}, recursive: {}, gallery: {} }}", self.width, self.height, self.image_path, self.directory, self.recursive, self.gallery)
        } else {
            write!(f, "Config: width={}, height={}, image_path={}, directory={}, recursive={}, gallery={}", self.width, self.height, match self.image_path {
                Some(ref path) => path.as_str(),
                None => "None",
            }, match &self.directory {
                Some(ref path) => path.display().to_string(),
                None => "None".to_string(),
            },
            self.recursive, self.gallery)
        }
    }
}
