use std::{thread::sleep, time::Duration};

mod mouse;
mod script;
mod ui;

fn main() {
    script::init();
    // ui::init();
    loop {
        sleep(Duration::from_secs(1))
    }
}
