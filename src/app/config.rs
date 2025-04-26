

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Config {}

impl Config {
    pub fn from_args(_args: std::env::Args) -> Self {
        Config {}
    }
}
