use std::fs;
use screenshots::Screen;
use crate::Display;

pub struct Instance {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    // pub number: u32,
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
            // number: 0,
        }
    }
}

impl Instance {
    pub fn screenshot(&self, file_name: String) {
        let screens = Screen::all().unwrap();
        let primary_screen = screens[0];

        let image = primary_screen.capture_area(self.x, self.y, self.width, self.height).unwrap();
        let buffer = image.buffer();
        fs::write(format!("screenshots/{}.png", file_name), buffer).unwrap();
        println!("Screenshotted (x: {}, y: {}, width: {}, height: {}", self.x, self.y, self.width, self.height);
    }

    // pub fn get_instance_number(&self) -> u32 {
    //     (self.x as u32 / self.width * Display::default().cols) + (self.y as u32 / self.height) + 1_u32
    // }

}
