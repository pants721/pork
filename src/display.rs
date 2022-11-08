extern crate ini;
use ini::Ini;
use screenshots::Screen;

pub struct Display {
    pub width: u32,
    pub height: u32,
    pub rows: u32,
    pub cols: u32,
}

impl Default for Display {
    fn default() -> Self {
        let screens = Screen::all().unwrap();
        let primary_screen = screens[0];

        let conf = Ini::load_from_file("conf.ini").unwrap();
        let settings = conf.section(Some("Settings")).unwrap();
        let u_rows: u32 = settings.get("rows").unwrap().parse().unwrap();
        let u_cols: u32 = settings.get("cols").unwrap().parse().unwrap();
        Display {
            width: primary_screen.display_info.width,
            height: primary_screen.display_info.height,
            rows: u_rows,
            cols: u_cols,
        }
    }
}