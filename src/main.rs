extern crate tensorflow;

mod display;
mod instance;

use display::Display;
use instance::Instance;


fn main() {
    let display = Display::default();

    for row in 1..display.rows + 1 {
        for col in 1..display.cols + 1 {
            let mut inst = Instance::default();
            inst.x = ((col - 1) * inst.width) as i32;
            inst.y = ((row - 1) * inst.height) as i32;
            inst.number = col + ((row - 1) * 3);
            inst.screenshot(format!("Instance {}", inst.number))
                .map_err(|err| println!("{:?}", err)).ok();
        }
    }
}
