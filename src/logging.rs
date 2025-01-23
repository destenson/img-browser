


pub fn initialize_rust_logging() {
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


