
mod platform;

use platform::main as pmain;


fn main() {
    // if windows target, run win32_main::main

    std::env::set_var("RUST_LOG", "trace");

    env_logger::init();

    pmain().unwrap();//.expect("win32_main::main failed");
}
