use ini::Ini;
use owo_colors::{self, OwoColorize};
use std::path;
use terminal_link::Link;

/*
Commands:
    Welcome
    Help
    Run
*/

const WELCOME_TO_PORK: &str = r#"
╭╮╭╮╭╮╱╱╭╮╱╱╱╱╱╱╱╱╱╱╱╱╱╱╭╮╱╱╱╱╭━━━╮╱╱╱╱╭╮
┃┃┃┃┃┃╱╱┃┃╱╱╱╱╱╱╱╱╱╱╱╱╱╭╯╰╮╱╱╱┃╭━╮┃╱╱╱╱┃┃
┃┃┃┃┃┣━━┫┃╭━━┳━━┳╮╭┳━━╮╰╮╭╋━━╮┃╰━╯┣━━┳━┫┃╭╮
┃╰╯╰╯┃┃━┫┃┃╭━┫╭╮┃╰╯┃┃━┫╱┃┃┃╭╮┃┃╭━━┫╭╮┃╭┫╰╯╯
╰╮╭╮╭┫┃━┫╰┫╰━┫╰╯┃┃┃┃┃━┫╱┃╰┫╰╯┃┃┃╱╱┃╰╯┃┃┃╭╮╮
╱╰╯╰╯╰━━┻━┻━━┻━━┻┻┻┻━━╯╱╰━┻━━╯╰╯╱╱╰━━┻╯╰╯╰╯
"#;

fn warn() {
    println!(
        "{}",
        "\nPork is still in development, so expect bugs and crashes.".red()
    );
    let git_link = Link::new("Github", "https://github.com/pants721/pork");
    println!(
        "For bug reports and further information, check out the {}",
        git_link
    );
}

pub fn check_ini() {
    if !path::Path::new("conf.ini").exists() {
        // println!("conf.ini exists!");
        println!("It seems you haven't configured Pork yet. Let's do that now!");
        println!("How many rows of instances do you have?");
        let mut rows = String::new();
        std::io::stdin()
            .read_line(&mut rows)
            .expect("Failed to read line");
        rows.pop();
        println!("How many columns of instances do you have?");
        let mut columns = String::new();
        std::io::stdin()
            .read_line(&mut columns)
            .expect("Failed to read line");
        columns.pop();

        let mut conf = Ini::new();
        conf.with_section(Some("Settings"))
            .set("rows", &rows)
            .set("cols", &columns);
        conf.write_to_file("conf.ini")
            .expect("Error writing to conf.ini");
    }
}

fn welcome_msg() {
    println!("{}", WELCOME_TO_PORK.green());
    println!("Welcome to Pork! 🐷");
    println!("Type 'cargo run help' for a list of commands.");
}

pub fn welcome() {
    check_ini();
    welcome_msg();
    warn();
}

pub fn help() {
    check_ini();
    println!("Pork is a simple and fast tool for Minecraft speedrunning\n");

    println!("Keybinds:");
    println!("{}", "    F1 - Toggle Pork".yellow());
    println!("{}", "    Q  - Exit Pork".yellow());
    println!("{}", "    P  - Run Pork".yellow());
    println!("{}", "    I  - Pause".yellow());

    println!("Commands:");
    println!("{}", "    help, h - This".yellow());
    println!("{}", "    run, r  - Runs Pork".yellow());
    warn();
}
