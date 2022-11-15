mod display;
mod instance;


use display::Display;
use instance::Instance;
#[cfg(target_os = "windows")]
use std::{env, process};
#[cfg(target_os = "windows")]
use inputbot::{KeySequence, KeybdKey::*, MouseButton::*};


fn main() {
    let display = Display::default();
    #[cfg(target_os = "windows")] {
        QKey.bind(|| process::exit(0));
        HKey.bind(move || display.run());
        inputbot::handle_input_events();
    }
    #[cfg(target_os = "macos")] {display.run()}
}

