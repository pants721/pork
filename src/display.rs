extern crate ini;
use ini::Ini;
use screenshots::Screen;
use crate::Instance;

pub struct Display {
    pub width: u32,
    pub height: u32,
    pub rows: u32,
    pub cols: u32,
}

impl Display {
    pub fn screenshot_instances(&self) {
        for row in 1..self.rows + 1 {
            for col in 1..self.cols + 1 {
                let mut inst = Instance::default();
                inst.x = ((col - 1) * inst.width) as i32;
                inst.y = ((row - 1) * inst.height) as i32;
                inst.number = col + ((row - 1) * 3);
                inst.screenshot()
                    .map_err(|err| println!("{:?}", err)).ok();
            }
        }
    }
}

impl Default for Display {
    fn default() -> Self {
        let screens = Screen::all().unwrap();
        let primary_screen = screens[1];

        let conf = Ini::load_from_file("conf.ini").unwrap();
        let settings = conf.section(Some("Settings")).unwrap();
        let u_rows: u32 = settings.get("rows")
            .unwrap()
            .parse()
            .unwrap();
        let u_cols: u32 = settings.get("cols")
            .unwrap()
            .parse()
            .unwrap();
        Display {
            width: primary_screen.display_info.width,
            height: primary_screen.display_info.height,
            rows: u_rows,
            cols: u_cols,
        }
    }
}