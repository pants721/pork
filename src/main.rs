mod cmds;
mod display;
mod instance;

use cmds::{check_ini, help, welcome};
use display::Display;
use instance::Instance;
use std::env;
#[cfg(target_os = "windows")]
use std::thread;

#[cfg(target_os = "windows")]
use inputbot::KeybdKey::*;
#[cfg(target_os = "windows")]
use std::process;

fn main() {
    let arg = env::args().nth(1).unwrap_or_default();

    match arg.to_lowercase().as_ref() {
        "help" | "h" => help(),
        "run" | "r" => run(),
        "" => welcome(),
        _ => println!("Invalid command!"),
    }
}

#[cfg(target_os = "macos")]
fn run() {
    check_ini();
    let display = Display::default();
    for _ in 0..20 {
        display.run();
    }
}

#[cfg(target_os = "windows")]
fn run() {
    check_ini();
    OKey.bind(|| process::exit(0));
    MKey.bind(|| {
        let mut display = Display::default();
        display.run();
    }); 
    inputbot::handle_input_events();
}
