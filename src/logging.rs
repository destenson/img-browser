


pub fn initialize_rust_logging() {
    // Set the log level. This is just for debugging purposes, we'll use
    // trace logging for now.
    //
    std::env::set_var("RUST_LOG", "trace");

    // Initialize the logger
    //
    env_logger::Builder::new()
        .parse_env("RUST_LOG")
        .format_line_number(true)
        .format_file(true)
        // .filter(Some("img_browser"), log::LevelFilter::Debug)
        .init();
}


