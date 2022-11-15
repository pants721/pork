use std::fs;
use enigo::*;
use screenshots::Screen;
use crate::Display;
use opencv::{imgproc, prelude::*, imgcodecs, core::{in_range, count_non_zero}};

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
        let primary_screen = screens[0];

        let image = primary_screen.capture_area(self.x, self.y, self.width, self.height).unwrap();
        let buffer = image.buffer();

        self.sc_dir = format!("screenshots/Instance-{}.png", self.number);
        fs::create_dir_all("screenshots")?;
        fs::write(&self.sc_dir, buffer).unwrap();
        println!("Screenshotted Instance {} (x: {}, y: {}, width: {}, height: {})", self.number, self.x, self.y, self.width, self.height);
        Ok(())
    }

    pub fn run(&self) -> Result<(), std::io::Error> {
        let blue_threshold = 15.0;        

        let mut enigo = Enigo::new();

        let lower_blue = Mat::from_slice(&[100.0, 90.0, 0.0]).unwrap();
        let upper_blue = Mat::from_slice(&[170.0, 255.0, 255.0]).unwrap();

        let img = imgcodecs::imread(&self.sc_dir, imgcodecs::IMREAD_COLOR).unwrap();
        let mut hsv_image = Mat::default();
        imgproc::cvt_color(&img, &mut hsv_image, imgproc::COLOR_BGR2HSV, 0).unwrap();
        let mut hsv_mask = Mat::default();

        in_range(&hsv_image, &lower_blue, &upper_blue, &mut hsv_mask).unwrap();

        // in opencv python, img.size returns img.width * img.height * 3????
        let blue_ratio = count_non_zero(&hsv_mask).unwrap() as f64 / (img.size().unwrap().area()) as f64;
        let blue_percent = blue_ratio * 100.0;
        println!("{:?}", blue_percent);
        if blue_percent >= blue_threshold {
            let center_x = self.x + self.width as i32 / 2;
            let center_y = self.y - self.height as i32 / 2;
            enigo.mouse_move_to(center_x, center_y);
            enigo.mouse_click(MouseButton::Left);
        }
        Ok(())
    }
}
