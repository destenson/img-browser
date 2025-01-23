mod app;
mod logging;
mod platform;
#[cfg(target_os = "windows")]
mod winmain;

// this is bad... we'll change it injecting the platform instead of using different functions
use platform::main as platform_main;

use app::App;


fn main() {

    if true {
        winmain::main().expect("winmain::main failed");
    } else {

        // Initialize the logger
        logging::initialize_rust_logging();
        
        platform_main().expect("win32_main::main failed");//.unwrap();
            // .map_err(|e| {
            //     eprintln!("Error: {}", e);
            //     Box::new(e)
            // }).map_err(|e| *e);
    }
}


fn main_() {
    // Initialize the logger
    logging::initialize_rust_logging();

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

