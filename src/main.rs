use std::sync::mpsc::{self, Receiver, Sender};

use config::Settings;

mod config;
mod mouse;
mod script;
mod ui;

fn main() {
    let (tx, rx): (Sender<Settings>, Receiver<Settings>) = mpsc::channel();
    script::init(rx);
    ui::init(tx);
}
