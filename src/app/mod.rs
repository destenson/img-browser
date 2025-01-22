mod config;
mod state;

// Every app has a state and a configuration.
use config::Config;
use state::State;



#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct App {
    config: Config,
    state: State,
}

impl App {
    pub fn new(args: Vec<String>) -> Self {
        // TODO: use clap
        // process arguments
        let config = Config::from_args(args);
        let state = State::default();
        App { config, state }
    }
}

impl App {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // log the configuration
        log::info!("Config: {:?}", self.config);
        // log the state
        log::info!("State: {:?}", self.state);
        Ok(())
    }
}

