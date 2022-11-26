use crate::Instance;
use ini::Ini;
use screenshots::Screen;
use std::thread;

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

        for row in 1..=self.rows {
            for col in 1..=self.cols {
                let mut inst = Instance {
                    x: ((col - 1) * self.width / self.cols) as i32,
                    y: ((row - 1) * self.height / self.rows) as i32,
                    width: self.width / self.cols,
                    height: self.height / self.rows,
                    number: col + ((row - 1) * self.cols),
                    img: Vec::new(),
                    path: String::new(),
                };
                inst.path = format!("D:/MultiMC/instances/inst{}/.minecraft", inst.number);
                let hold_file = format!("{}{}", inst.path, "/hold.tmp");
                // let preview_file = String::from(format!("{}{}", inst.path, "/preview.tmp"));
                // let preview_file_contents = if std::path::Path::new(&preview_file).exists() {std::fs::read_to_string(preview_file).expect("Error reading preview file")} else {String::from("0")};
                // println!("{:?}", preview_file_contents.parse::<u64>().unwrap());
                // println!("{}: {}ms", inst.number, now.elapsed().as_millis());
                if !std::path::Path::new(&hold_file).exists() {
                    let inst_thread = thread::spawn(move || inst.run());
                    thread_vec.push(inst_thread);
                    // inst_thread.join().unwrap();
                }
            }
        }

        // Joins all threads in vec
        for thread in thread_vec {
            thread.join().unwrap();
        }
    }
}

impl Default for Display {
    fn default() -> Self {
        let screens = Screen::all().expect("Error getting screens");
        let primary_screen = screens[1];

        let conf = Ini::load_from_file("conf.ini").expect("Error loading conf.ini");
        let settings = conf
            .section(Some("Settings"))
            .expect("Error loading Settings section");
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
