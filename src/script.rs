use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_LBUTTON};

use crate::mouse::{move_to, Vec2};
use std::{thread, time::Duration};

struct Weapon {
    recoil_pattern: Vec<Vec2<f32>>,
    delay: Duration,
}

impl Weapon {
    fn new(recoil_pattern: Vec<Vec2<f32>>, delay: Duration) -> Self {
        Self {
            recoil_pattern,
            delay,
        }
    }
}

pub fn init() {
    thread::spawn(|| loop {
        let weapons = [Weapon::new(
            vec![
                Vec2::new(0.000000, -2.257792),
                Vec2::new(0.323242, -2.300758),
                Vec2::new(0.649593, -2.299759),
                Vec2::new(0.848786, -2.259034),
                Vec2::new(1.075408, -2.323947),
                Vec2::new(1.268491, -2.215956),
                Vec2::new(1.330963, -2.236556),
                Vec2::new(1.336833, -2.218203),
                Vec2::new(1.505516, -2.143454),
                Vec2::new(1.504423, -2.233091),
                Vec2::new(1.442116, -2.270194),
                Vec2::new(1.478543, -2.204318),
                Vec2::new(1.392874, -2.165817),
                Vec2::new(1.480824, -2.177887),
                Vec2::new(1.597069, -2.270915),
                Vec2::new(1.449996, -2.145893),
                Vec2::new(1.369179, -2.270_45),
                Vec2::new(1.582363, -2.298334),
                Vec2::new(1.516872, -2.235066),
                Vec2::new(1.498249, -2.238401),
                Vec2::new(1.465769, -2.331642),
                Vec2::new(1.564812, -2.242621),
                Vec2::new(1.517519, -2.303052),
                Vec2::new(1.422433, -2.211946),
                Vec2::new(1.553195, -2.248043),
                Vec2::new(1.510463, -2.285327),
                Vec2::new(1.553878, -2.240047),
                Vec2::new(1.520_38, -2.221839),
                Vec2::new(1.553878, -2.240047),
                Vec2::new(1.553195, -2.248043),
            ],
            Duration::from_micros(133330),
        )];

        let acceleration = Vec2::new(vec![0.1, 0.3, 1.0], vec![0.1, 0.3, 1.0]);
        let sensitivity_multipler = (0.03f32 * 2f32) * 3f32 * (90f32 / 100f32);
        let current_weapon = &weapons[0];
        'inner: for delta in current_weapon.recoil_pattern.iter() {
            unsafe {
                if GetAsyncKeyState(VK_LBUTTON.into()) == 0 {
                    break 'inner;
                }
            }

            move_to(
                &Vec2::new(
                    -(delta.x / (-0.3 * sensitivity_multipler)),
                    -(delta.y / (-0.3 * sensitivity_multipler)),
                ),
                &acceleration,
                &current_weapon.delay,
            );
        }
    });
}
