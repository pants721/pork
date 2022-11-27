use enigo::{Enigo, MouseButton, MouseControllable};
use opencv::{
    core::{count_non_zero, in_range},
    imgcodecs, imgproc,
    prelude::*,
};
use screenshots::Screen;

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

    fn eval(&mut self) {
        let blue_threshold = 7.5;

        let mut enigo = Enigo::new();

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
        // println!("{blue_percent:.2}%");
        if blue_percent <= blue_threshold {
            let center_x = self.x + self.width as i32 / 2;
            let center_y = self.y + self.height as i32 / 2;
            enigo.mouse_move_to(center_x, center_y);
            enigo.mouse_click(MouseButton::Left);
        }
    }

    pub fn run(&mut self) {
        self.screenshot();
        self.eval();
    }
}
