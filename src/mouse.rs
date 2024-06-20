use std::time::{Duration, SystemTime};

use enigo::{Coordinate, Enigo, Mouse, Settings};
use num_integer::binomial;
use rayon::prelude::*;

pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

fn calculate_recursive_bezier(control_points: &[f64], time: &f64) -> f64 {
    let size = control_points.len();
    let delta_time = 1f64 - time;

    control_points.par_iter().enumerate().map(|(i, val)| {
        let binom = binomial(size - 1, i);
        let term = binom as f64 * delta_time.powi((size - 1 - i) as i32) * time.powi(i as i32);
        term * val
    }).sum()
}

pub fn move_to(delta: &Vec2<f64>, control_points: &Vec2<Vec<f64>>, duration: &Duration) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let start = SystemTime::now();
    let start_location = enigo.location().unwrap();
    let mut previous_location = Vec2::new(start_location.0 as f64, start_location.1 as f64);
    let mut pixel_loss = Vec2::new(0.0, 0.0);

    loop {
        let elapsed_time = start.elapsed().unwrap().as_millis();

        if elapsed_time == 0 {
            continue;
        }

        if elapsed_time > duration.as_millis() {
            break;
        }

        let time_percent = elapsed_time as f64 / duration.as_millis() as f64;

        let absolute_location = Vec2::new(
            start_location.0 as f64
                + delta.x * calculate_recursive_bezier(&control_points.x, &(time_percent)),
            start_location.1 as f64
                + delta.y * calculate_recursive_bezier(&control_points.y, &(time_percent)),
        );

        let mut relative_position = Vec2::new(
            absolute_location.x - previous_location.x,
            absolute_location.y - previous_location.y,
        );

        previous_location = absolute_location;
        relative_position.x += pixel_loss.x;
        relative_position.y += pixel_loss.y;

        pixel_loss.x = relative_position.x.fract();
        pixel_loss.y = relative_position.y.fract();

        enigo
            .move_mouse(
                -relative_position.x as i32,
                -relative_position.y as i32,
                Coordinate::Rel,
            )
            .unwrap();
    }
}
