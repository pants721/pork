mod display;
mod instance;

use std::process;

use display::Display;
use instance::Instance;
use inputbot::KeybdKey::*;


fn main() {
    let display = Display::default();
    HKey.bind(move || display.screenshot_instances());
    QKey.bind(|| process::exit(0));
    inputbot::handle_input_events();
}

