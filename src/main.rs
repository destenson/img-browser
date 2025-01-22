
mod app;
mod platform;

// this is bad... we'll change it injecting the platform instead of using different functions
use platform::main as platform_main;

use app::App;

fn main() -> Result<(), Box<impl std::error::Error>> {

    // Initialize the logger
    initialize_rust_logging();

    platform_main() //.unwrap();//.expect("win32_main::main failed");
        .map_err(|e| {
            eprintln!("Error: {}", e);
            Box::new(e)
        })
}


fn main_() {
    // Initialize the logger
    initialize_rust_logging();

    // TODO: use clap for the args

    // Create the app
    let app = App::new(std::env::args().collect());

    // Run the app
    let result = app.run();

    // Handle the result
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
    
    // // Load the image as a bitmap
    // let (image_bmp, width, height) = load_image_as_bitmap("assets/image.png");

    // // Create the window
    // let window = create_window(width, height).expect("Failed to create window");

    // // Main message loop
    // let _ = message_loop(window, image_bmp, width, height).expect("Message loop failed");
}

fn initialize_rust_logging() {
    // Set the log level. This is just for debugging purposes, we'll use
    // trace logging for now.
    //
    std::env::set_var("RUST_LOG", "trace");

    // Initialize the logger
    //
    env_logger::Builder::new()
        .format(|buf, record| {
            use std::io::Write;
            writeln!(
                buf,
                "{} [{}] {}:{} - {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .parse_env("RUST_LOG")
        // .filter(Some("img_browser"), log::LevelFilter::Debug)
        .init();
}

