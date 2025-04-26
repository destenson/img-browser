pub mod config;
pub mod error;
pub mod settings;
pub mod state;


// Every app has a state and a configuration.
pub use config::Config;
pub use state::State;

pub use error::{Error, Result};

use crate::platform::Platform;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct App {
    pub config: Config,
    pub state: State,
}

impl App {
    pub fn new(args: std::env::Args) -> Self {
        // TODO: use clap
        // process arguments
        let config = Config::from_args(args);
        let state = State::default();
        // create the app
        App { config, state }
    }
}

impl App {
    pub fn run<P: Platform<App = Self, Window = crate::platform::Window>>(self, platform: P) -> Result<()> {
        {
        let Self { config, state } = &self;
        
        // log the configuration
        log::info!("Config: {:?}", self.config);
        // log the state
        log::info!("State: {:?}", self.state);
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
