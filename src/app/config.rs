#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// The width of the window.
    pub width: u32,
    /// The height of the window.
    pub height: u32,
    /// Optional path to an image file to load.
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
    pub fn from_args(mut args: std::env::Args) -> Self {
        // Skip the program name
        let _ = args.next();
        
        // The first argument after the program name could be an image path
        let image_path = args.next().map(String::from);
        
        // TODO: parse width and height from args
        
        Config {
            width: 800,
            height: 600,
            image_path,
        }
    }
}
