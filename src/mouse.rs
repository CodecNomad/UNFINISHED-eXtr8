use enigo::{Coordinate, Enigo, Mouse, Settings};
use num_integer::binomial;
use std::{
    io::Cursor,
    time::{Duration, SystemTime},
};

pub struct Vec2<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

fn calculate_recursive_bezier(control_points: &[f32], time: &f32) -> f32 {
    let size = control_points.len();
    let mut x = 0f32;
    let delta_time = 1f32 - time;

    for i in control_points.iter().enumerate() {
        let binom = binomial(size - 1, i.0);
        let term = binom as f32 * delta_time.powi((size - 1 - i.0) as i32) * time.powi(i.0 as i32);
        x += term * i.1;
    }

    x
}

pub fn move_to(delta: &Vec2<f32>, control_points: &Vec2<Vec<f32>>, duration: &Duration) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let start = SystemTime::now();
    let start_location = enigo.location().unwrap();
    let mut virtual_location = Vec2::new(start_location.0 as f32, start_location.1 as f32);

    loop {
        let elapsed_time = start.elapsed().unwrap().as_millis();

        if elapsed_time == 0 {
            continue;
        }

        if elapsed_time > duration.as_millis() {
            break;
        }

        let time_percent = elapsed_time as f32 / duration.as_millis() as f32;

        let absolute_location = Vec2::new(
            start_location.0 as f32
                - delta.x * calculate_recursive_bezier(&control_points.x, &(time_percent)),
            start_location.1 as f32
                - delta.y * calculate_recursive_bezier(&control_points.y, &(time_percent)),
        );

        virtual_location.x = absolute_location.x;
        virtual_location.y = absolute_location.y;

        let current_location = enigo.location().unwrap();

        enigo
            .move_mouse(
                virtual_location.x.round() as i32 - current_location.0,
                virtual_location.y.round() as i32 - current_location.1,
                Coordinate::Rel,
            )
            .unwrap();
    }
}
