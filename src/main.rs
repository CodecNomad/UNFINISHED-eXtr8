use crate::movement_helper::{move_to, Vec2};
use std::time::Duration;

mod movement_helper;
mod script;
mod ui;

fn main() {
    script::init();
    ui::init();

    let movement = Vec2 {
        x: 130f32,
        y: 130f32,
    };

    let acceleration = Vec2 {
        x: vec![0.1, 0.3, 0.9, 0.95, 1.55, 1.0],
        y: vec![0.1, 0.3, 0.9, 0.95, 1.55, 1.0],
    };

    move_to(&movement, &acceleration, &Duration::from_millis(500));
}
