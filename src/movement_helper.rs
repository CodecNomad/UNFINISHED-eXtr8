use num_integer::binomial;

pub fn calculate_recursive_bezier(control_points: Vec<f32>, time: f32) -> f32 {
    let size = control_points.len();
    let mut x = 0f32;
    let delta_time = 1f32 - time;

    for i in 0..size {
        let binom = binomial(size - 1, i);
        let term = binom as f32 * delta_time.powi((size - 1 - i) as i32) * time.powi(i as i32);
        x += term * control_points[i];
    }

    x
}
