use std::thread;

pub fn init() {
    thread::spawn(|| loop {});
}
