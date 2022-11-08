mod display;
mod instance;

use display::Display;
use instance::Instance;


fn main() {
    let display = Display::default();
    for row in 1..display.rows + 1 {
        for i in 1..display.cols + 1 {
            let mut inst = Instance::default();
            inst.x = ((row - 1) * inst.width) as i32;
            inst.y = ((i - 1) * inst.height) as i32;
            inst.screenshot(format!("Screenshot(row: {}, col: {}", row, i))
                .map_err(|err| println!("{:?}", err)).ok();
        }
    }
}
