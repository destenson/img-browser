use clap::Parser;

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
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
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width: 800,
            height: 600,
            image_path: None,
        }
    }
}

impl Config {
    pub fn from_args(args: std::env::Args) -> Self {
        Self::parse_from(args)
    }
}
