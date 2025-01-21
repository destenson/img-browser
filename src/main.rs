
mod platform;

use platform::main as pmain;


fn main() {
    // if windows target, run win32_main::main

    env_logger::init();

    pmain().expect("win32_main::main failed");
}
