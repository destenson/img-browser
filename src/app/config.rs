

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// The width of the window.
    pub width: u32,
    /// The height of the window.
    pub height: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width: 800,
            height: 600,
        }
    }
}

impl Config {
    pub fn from_args(args: std::env::Args) -> Self {
        // TODO: parse width and height from args
        
        Config {
            width: 800,
            height: 600,    
        }
    }
}
