use std::fs;
use std::process::Command;
use screenshots::Screen;
use crate::Display;

pub struct Instance {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub number: u32,
    pub sc_dir: String,
}

impl Default for Instance {
    fn default() -> Self {
        let display_width = Display::default().width;
        let display_height = Display::default().height;
        let display_cols = Display::default().cols;
        let display_rows = Display::default().rows;
        Instance {
            x: 0,
            y: 0,
            width: display_width / display_cols,
            height: display_height / display_rows,
            number: 0,
            sc_dir: String::from(""),
        }
    }
}

impl Instance {
    pub fn screenshot(&mut self) -> Result<(), std::io::Error> {
        let screens = Screen::all().unwrap();
        let primary_screen = screens[1];
        let image = primary_screen.capture_area(self.x, self.y, self.width, self.height).unwrap();
        let buffer = image.buffer();
        self.sc_dir = format!("screenshots/Instance-{}.png", self.number);
        fs::create_dir_all("screenshots")?;
        fs::write(&self.sc_dir, buffer).unwrap();
        self.launch_predict_script();
        println!("Screenshotted Instance {} (x: {}, y: {}, width: {}, height: {})", self.number, self.x, self.y, self.width, self.height);
        Ok(())
    }
    pub fn launch_predict_script(&self) {
        // https://doc.rust-lang.org/std/process/struct.Command.html
        // https://www.tutorialspoint.com/python/python_command_line_arguments.htm (pass in sc_dir, width, height)
        // arg order is thing.py sc_dir, width, height, x, y
        Command::new("python3")
            .args([
                "src/predict.py", 
                &self.sc_dir, 
                &self.width.to_string(), 
                &self.height.to_string(),
                &self.x.to_string(),
                &self.y.to_string()
                ])
            .spawn()
            .expect("Failed");

    }
}
