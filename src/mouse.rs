use enigo::{Coordinate, Enigo, Mouse, Settings};
use num_integer::binomial;
use std::time::{Duration, SystemTime};

pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
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
    let mut previous_location = Vec2::new(start_location.0 as f32, start_location.1 as f32);
    let mut pixel_loss = Vec2::new(0f32, 0f32);

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
                + delta.x * calculate_recursive_bezier(&control_points.x, &(time_percent)),
            start_location.1 as f32
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
