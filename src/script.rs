use crate::config::Weapons;
use crate::{
    config::Settings,
    mouse::{move_to, Vec2},
};
use windows_sys::Win32::System::Diagnostics::Debug::Beep;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VK_C, VK_CONTROL, VK_LBUTTON, VK_RBUTTON,
};

use std::time::SystemTime;
use std::{sync::mpsc::Receiver, thread, time::Duration};

pub fn init(rx: Receiver<Settings>) {
    thread::spawn(move || {
        let mut enabled = false;
        let mut settings: Settings = Settings::default();
        let weapons = Weapons::get();
        let mut current_weapon = &weapons[0];
        let mut last_press = SystemTime::now();

        loop {
            unsafe {
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

                let barrel_multiplier = settings.barrel.get_modifier();
                let mut sight_multiplier = settings.sight.get_modifier();
                let stand_multiplier = if unsafe { GetAsyncKeyState(VK_CONTROL.into()) } != 0 {
                    1f32
                } else {
                    2f32
                };
                let hip_modifier = if unsafe { GetAsyncKeyState(VK_RBUTTON.into()) != 0 } {
                    1f32
                } else {
                    sight_multiplier = 1f32;
                    0.8f32
                };

                move_to(
                    &Vec2::new(
                        -(delta.x
                            * sight_multiplier
                            * barrel_multiplier
                            * stand_multiplier
                            * hip_modifier
                            / (-0.3 * sensitivity_multipler)),
                        -(delta.y
                            * sight_multiplier
                            * barrel_multiplier
                            * stand_multiplier
                            * hip_modifier
                            / (-0.3 * sensitivity_multipler)),
                    ),
                    &acceleration,
                    &current_weapon.delay,
                );
            }
        }
    });
}
