mod app;
mod logging;
mod platform;
// #[cfg(target_os = "windows")]
// mod winmain;

// this is bad... we'll change it injecting the platform instead of using different functions
use platform::main as platform_main;

use app::App;

pub use app::{Error, Result};

fn main() {
    // Initialize the logger
    logging::initialize_rust_logging();

    // if false {
    //     winmain::main().expect("winmain::main failed");
    // } else {

        platform_main(std::env::args()).expect("win32_main::main failed");//.unwrap();
            // .map_err(|e| {
            //     eprintln!("Error: {}", e);
            //     Box::new(e)
            // }).map_err(|e| *e);
    // }
}



