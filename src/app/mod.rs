pub mod config;
pub mod error;
pub mod settings;
pub mod state;

// Every app has a state and a configuration.
use config::Config;
use state::State;

pub use error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct App {
    config: Config,
    state: State,
}

impl App {
    pub fn new(args: std::env::Args) -> Self {
        // TODO: use clap
        // process arguments
        let config = Config::from_args(args);
        let state = State::default();
        App { config, state }
    }
}

impl App {
    pub fn run(&self) -> Result<()> {
        // log the configuration
        log::info!("Config: {:?}", self.config);
        // log the state
        log::info!("State: {:?}", self.state);
        Ok(())
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
