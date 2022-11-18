extern crate ini;

use crate::Instance;
use ini::Ini;
use screenshots::Screen;
use std::{thread, time::Instant};

pub struct Display {
    // Screen width
    pub width: u32,
    // Screen height
    pub height: u32,
    // Number of rows of instances, determined by user in conf.ini
    pub rows: u32,
    // Number of columns of instances, determined by user in conf.ini
    pub cols: u32,
}

impl Display {
    pub fn run(&self) {
        // Stores thread JoinHandles in thread_vec
        let mut thread_vec = Vec::<std::thread::JoinHandle<()>>::new();
        // Timer
        let start = Instant::now();
        for row in 1..=self.rows {
            for col in 1..=self.cols {
                let mut inst = Instance::default();
                inst.x = ((col - 1) * inst.width) as i32;
                inst.y = ((row - 1) * inst.height) as i32;
                inst.number = col + ((row - 1) * self.cols);
                let inst_thread = thread::spawn(move || inst.run());
                thread_vec.push(inst_thread);
            }
        }

        // Joins all threads in array
        for thread in thread_vec {
            thread.join().unwrap();
        }
        let dur = start.elapsed();
        println!("Time elapsed in Display.run(): {dur:?}");
    }
}

impl Default for Display {
    fn default() -> Self {
        let screens = Screen::all().unwrap();
        let primary_screen = screens[1];

        let conf = Ini::load_from_file("conf.ini").unwrap();
        let settings = conf.section(Some("Settings")).unwrap();
        let u_rows: u32 = settings.get("rows").unwrap().parse().unwrap();
        let u_cols: u32 = settings.get("cols").unwrap().parse().unwrap();
        Self {
            width: primary_screen.display_info.width,
            height: primary_screen.display_info.height,
            rows: u_rows,
            cols: u_cols,
        }
    }
}
