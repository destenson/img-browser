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
    #[clap(short, long)]
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
            recursive: false,
            gallery: false,
        }
    }
}

impl Config {
    pub fn from_args(args: std::env::Args) -> Self {
        Self::parse_from(args)
    }
}
