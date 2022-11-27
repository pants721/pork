use enigo::{Enigo, MouseButton, MouseControllable};
use std::fs;
use opencv::{
    core::{count_non_zero, in_range},
    imgcodecs, imgproc,
    prelude::*,
};
use screenshots::Screen;
use std::io::Read;

pub struct Instance {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub number: u32,
    pub img: Vec<u8>,
    pub path: String,
}

impl Instance {
    fn screenshot(&mut self) {
        let screens = Screen::all().expect("Error getting screens");
        // get the highest index of screens
        let primary_screen = screens[1];

        let image = primary_screen
            .capture_area(self.x, self.y, self.width, self.height)
            .expect("Error capturing screen");
        let buffer = image.buffer();

        self.img = buffer.to_vec();
        // println!(
        //     "Screenshotted Instance {} (x: {}, y: {}, width: {}, height: {})",
        //     self.number, self.x, self.y, self.width, self.height
        // );
    }
    pub fn eval(&mut self) {
        let use_open_cv = false;
        let blue_threshold = 5.0;
        let biomes = vec!["beach", "ocean", "plains", "forest"];
        let fringe_biomes = vec!["plains", "forest"];

        let log = format!("{}{}", self.path, "/logs/latest.log");
        // search log for line containing "Biome: "
        let mut log_file = fs::File::open(log).expect("Error opening log file");
        let mut log_contents = String::new();
        log_file.read_to_string(&mut log_contents).unwrap();
        let mut biome = String::new();
        for line in log_contents.lines() {
            if line.contains("biome") {
                biome = line.split('.').last().unwrap().to_string();
            }
        }
        println!("{}", biome);
        if !biomes.contains(&biome.as_str()) {
            self.reset();
        }
        if fringe_biomes.contains(&biome.as_str()) {
            self.screenshot();
            let lower_blue = Mat::from_slice(&[100.0, 90.0, 0.0]).unwrap();

            let upper_blue = Mat::from_slice(&[170.0, 255.0, 255.0]).unwrap();

            let buffer_img = Mat::from_slice(&self.img).unwrap();

            let img =
                imgcodecs::imdecode(&buffer_img, imgcodecs::IMREAD_COLOR).expect("Error reading image");

            let mut hsv_image = Mat::default();
            imgproc::cvt_color(&img, &mut hsv_image, imgproc::COLOR_BGR2HSV, 0)
                .expect("Error converting image to HSV");
            let mut hsv_mask = Mat::default();

            in_range(&hsv_image, &lower_blue, &upper_blue, &mut hsv_mask).unwrap();

            // in opencv python, img.size returns img.width * img.height * 3????
            let blue_ratio =
                f64::from(count_non_zero(&hsv_mask).unwrap()) / f64::from(img.size().unwrap().area());
            let blue_percent = blue_ratio * 100.0;
            println!("{}: {}%", self.number, blue_percent);
            if blue_percent < blue_threshold {
                self.reset();
            }
        }
    }
    fn reset(&self) {
        let mut mouse = Enigo::new();
        let center_x = self.x + (self.width / 2) as i32;
        let center_y = self.y + (self.height / 2) as i32;
        mouse.mouse_move_to(center_x, center_y);
        mouse.mouse_click(MouseButton::Left);
    }
}
