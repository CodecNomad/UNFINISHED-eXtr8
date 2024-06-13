use crate::config::{BarrellID, SightID, Weapon, WeaponID};
use crate::{
    config::Settings,
    mouse::{move_to, Vec2},
};
use core::ptr::null;
use windows_sys::s;
use windows_sys::Win32::System::Diagnostics::Debug::Beep;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_C, VK_L, VK_LBUTTON};
use windows_sys::Win32::UI::WindowsAndMessaging::{FindWindowA, ShowWindow};

use std::time::SystemTime;
use std::{sync::mpsc::Receiver, thread, time::Duration};

pub fn init(rx: Receiver<Settings>) {
    thread::spawn(move || {
        let weapons = [
            Weapon::new(
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
                WeaponID::Ak47,
            ),
            Weapon::new(
                vec![
                    Vec2::new(0.000000, -2.052616),
                    Vec2::new(0.055584, -1.897695),
                    Vec2::new(-0.247226, -1.863222),
                    Vec2::new(-0.243871, -1.940_01),
                    Vec2::new(0.095727, -1.966751),
                    Vec2::new(0.107707, -1.885_52),
                    Vec2::new(0.324888, -1.946722),
                    Vec2::new(-0.181137, -1.880342),
                    Vec2::new(0.162399, -1.820107),
                    Vec2::new(-0.292076, -1.994_94),
                    Vec2::new(0.064575, -1.837156),
                    Vec2::new(-0.126699, -1.887_88),
                    Vec2::new(-0.090568, -1.832799),
                    Vec2::new(0.065338, -1.807_48),
                    Vec2::new(-0.197343, -1.705888),
                    Vec2::new(-0.216561, -1.785949),
                    Vec2::new(0.042567, -1.806371),
                    Vec2::new(-0.065534, -1.757623),
                    Vec2::new(0.086380, -1.904_01),
                    Vec2::new(-0.097326, -1.969296),
                    Vec2::new(-0.213034, -1.850288),
                    Vec2::new(-0.017790, -1.730867),
                    Vec2::new(-0.045577, -1.783686),
                    Vec2::new(-0.053309, -1.886_26),
                    Vec2::new(0.055072, -1.793076),
                    Vec2::new(-0.091874, -1.921906),
                    Vec2::new(-0.033719, -1.796_16),
                    Vec2::new(0.266464, -1.993952),
                    Vec2::new(0.079090, -1.921165),
                ],
                Duration::from_micros(120000),
                WeaponID::Lr300,
            ),
            Weapon::new(
                vec![
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.393750, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.720000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                    Vec2::new(0.000000, -1.4900),
                ],
                Duration::from_micros(100000),
                WeaponID::M2,
            ),
            Weapon::new(
                vec![
                    Vec2::new(0.000000, -1.4000),
                    Vec2::new(-0.390000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                    Vec2::new(-0.730000, -1.4000),
                ],
                Duration::from_micros(100000),
                WeaponID::HmLmg,
            ),
            Weapon::new(
                vec![
                    Vec2::new(0.125361, -1.052446),
                    Vec2::new(-0.099548, -0.931548),
                    Vec2::new(0.027825, -0.954094),
                    Vec2::new(-0.013715, -0.851504),
                    Vec2::new(-0.007947, -1.070579),
                    Vec2::new(0.096096, -1.018017),
                    Vec2::new(-0.045937, -0.794216),
                    Vec2::new(0.034316, -1.112618),
                    Vec2::new(-0.003968, -0.930040),
                    Vec2::new(-0.009403, -0.888503),
                    Vec2::new(0.140813, -0.970807),
                    Vec2::new(-0.015052, -1.046551),
                    Vec2::new(0.095699, -0.860475),
                    Vec2::new(-0.269643, -1.038896),
                    Vec2::new(0.000285, -0.840478),
                    Vec2::new(0.018413, -1.038126),
                    Vec2::new(0.099191, -0.851701),
                    Vec2::new(0.199659, -0.893041),
                    Vec2::new(-0.082660, -1.069278),
                    Vec2::new(0.006826, -0.881493),
                    Vec2::new(0.091709, -1.150956),
                    Vec2::new(-0.108677, -0.965513),
                    Vec2::new(0.169612, -1.099499),
                    Vec2::new(-0.038244, -1.120084),
                    Vec2::new(-0.085513, -0.876956),
                    Vec2::new(0.136279, -1.047589),
                    Vec2::new(0.196392, -1.039977),
                    Vec2::new(-0.152513, -1.209291),
                    Vec2::new(-0.214510, -0.956648),
                    Vec2::new(0.034276, -0.095177),
                ],
                Duration::from_micros(89000),
                WeaponID::Mp5,
            ),
            Weapon::new(
                vec![
                    Vec2::new(-0.114413, -0.680635),
                    Vec2::new(0.008686, -0.676598),
                    Vec2::new(0.010312, -0.682837),
                    Vec2::new(0.064825, -0.691345),
                    Vec2::new(0.104075, -0.655618),
                    Vec2::new(-0.088118, -0.660429),
                    Vec2::new(0.089906, -0.675183),
                    Vec2::new(0.037071, -0.632623),
                    Vec2::new(0.178465, -0.634737),
                    Vec2::new(0.034654, -0.669443),
                    Vec2::new(-0.082658, -0.664826),
                    Vec2::new(0.025550, -0.636631),
                    Vec2::new(0.082414, -0.647118),
                    Vec2::new(-0.123305, -0.662104),
                    Vec2::new(0.028164, -0.662354),
                    Vec2::new(-0.117346, -0.693475),
                    Vec2::new(-0.268777, -0.661123),
                    Vec2::new(-0.053086, -0.677493),
                    Vec2::new(0.042380, -0.647038),
                    Vec2::new(0.042380, -0.647038),
                ],
                Duration::from_micros(113000),
                WeaponID::Thompson,
            ),
            Weapon::new(
                vec![
                    Vec2::new(-0.114414, -0.680635),
                    Vec2::new(0.008685, -0.676597),
                    Vec2::new(0.010312, -0.682837),
                    Vec2::new(0.064825, -0.691344),
                    Vec2::new(0.104075, -0.655617),
                    Vec2::new(-0.088118, -0.660429),
                    Vec2::new(0.089906, -0.675183),
                    Vec2::new(0.037071, -0.632623),
                    Vec2::new(0.178466, -0.634737),
                    Vec2::new(0.034653, -0.669444),
                    Vec2::new(-0.082658, -0.664827),
                    Vec2::new(0.025551, -0.636631),
                    Vec2::new(0.082413, -0.647118),
                    Vec2::new(-0.123305, -0.662104),
                    Vec2::new(0.028164, -0.662354),
                    Vec2::new(-0.117345, -0.693474),
                    Vec2::new(-0.268777, -0.661122),
                    Vec2::new(-0.053086, -0.677493),
                    Vec2::new(0.004238, -0.647037),
                    Vec2::new(0.014169, -0.551440),
                    Vec2::new(-0.009907, -0.552079),
                    Vec2::new(0.044076, -0.577694),
                    Vec2::new(-0.043187, -0.549581),
                ],
                Duration::from_micros(90000),
                WeaponID::Custom,
            ),
            Weapon::new(
                vec![Vec2::new(0.0, -5.8)],
                Duration::from_micros(125000),
                WeaponID::Python,
            ),
            Weapon::new(
                vec![Vec2::new(0.0, -1.4)],
                Duration::from_micros(175000),
                WeaponID::Semi,
            ),
        ];

        let mut enabled = false;
        let mut settings: Settings = Settings::new(
            0.3f32,
            crate::config::WeaponID::Ak47,
            SightID::None,
            BarrellID::None,
        );
        let mut current_weapon = &weapons[0];
        let mut last_press = SystemTime::now();
        let mut focused = false;

        loop {
            unsafe {
                if GetAsyncKeyState(VK_L.into()) != 0
                    && SystemTime::now()
                        .duration_since(last_press)
                        .unwrap()
                        .as_millis()
                        > Duration::from_millis(250).as_millis()
                {
                    let hwnd = FindWindowA(null(), s!("eXtr8"));

                    if focused {
                        ShowWindow(hwnd, 1);
                    } else {
                        ShowWindow(hwnd, 0);
                    }

                    Beep(1000, 100);
                    focused = !focused;
                    last_press = SystemTime::now();
                }

                if GetAsyncKeyState(VK_C.into()) != 0
                    && SystemTime::now()
                        .duration_since(last_press)
                        .unwrap()
                        .as_millis()
                        > Duration::from_millis(250).as_millis()
                {
                    Beep(1000, 100);
                    enabled = !enabled;
                    last_press = SystemTime::now();
                }
            }

            if let Ok(value) = rx.try_recv() {
                settings = value;
                weapons.iter().enumerate().for_each(|(i, weapon)| {
                    if weapon.weapon_id == settings.weapon {
                        current_weapon = &weapons[i];
                    }
                });
            }

            let acceleration = Vec2::new(vec![0.1, 0.75, 1.0], vec![0.1, 0.75, 1.0]);
            let sensitivity_multipler =
                (settings.sensitivity / 10f32 * 2f32) * 3f32 * (90f32 / 100f32);
            'inner: for delta in current_weapon.recoil_pattern.iter() {
                unsafe {
                    if GetAsyncKeyState(VK_LBUTTON.into()) == 0 || !enabled {
                        break 'inner;
                    }
                }

                let barrell_multiplier = settings.barrell.get_modifier();
                let sight_multiplier = settings.sight.get_modifier();

                move_to(
                    &Vec2::new(
                        -(delta.x * sight_multiplier * barrell_multiplier
                            / (-0.3 * sensitivity_multipler)),
                        -(delta.y * sight_multiplier * barrell_multiplier
                            / (-0.3 * sensitivity_multipler)),
                    ),
                    &acceleration,
                    &current_weapon.delay,
                );
            }
        }
    });
}
