use std::time::SystemTime;
use std::{sync::mpsc::Receiver, thread, time::Duration};

use windows_sys::Win32::System::Diagnostics::Debug::Beep;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, VK_A, VK_C, VK_CONTROL, VK_D, VK_LBUTTON, VK_RBUTTON, VK_S, VK_W,
};

use crate::config::Weapons;
use crate::mouse::calculate_recursive_bezier;
use crate::{
    config::Settings,
    mouse::{move_to, Vec2},
};

pub fn init(rx: Receiver<Settings>) {
    thread::spawn(move || {
        let mut enabled = false;
        let mut settings: Settings = Settings::default();
        let weapons = Weapons::get();
        let mut current_weapon = &weapons[0];
        let mut last_press = SystemTime::now();
        let mut was_moving = false;
        let mut start_of_move = SystemTime::now();

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
            let sensitivity_multiplier =
                (settings.sensitivity / 10f64 * 2f64) * 3f64 * (90f64 / 100f64);
            'inner: for delta in current_weapon.recoil_pattern.iter() {
                unsafe {
                    if GetAsyncKeyState(VK_LBUTTON.into()) == 0 || !enabled {
                        break 'inner;
                    }
                }

                let barrel_multiplier = settings.barrel.get_modifier();
                let sight_multiplier = settings.sight.get_modifier();
                let movement_penalty = current_weapon.move_penalty;

                let is_aiming = unsafe { GetAsyncKeyState(VK_RBUTTON.into()) != 0 };
                let is_crouching = unsafe { GetAsyncKeyState(VK_CONTROL.into()) } != 0;
                let is_moving = unsafe { GetAsyncKeyState(VK_W.into()) != 0 }
                    || unsafe { GetAsyncKeyState(VK_S.into()) != 0 }
                    || unsafe { GetAsyncKeyState(VK_A.into()) != 0 }
                    || unsafe { GetAsyncKeyState(VK_D.into()) != 0 };

                if is_moving && was_moving == false {
                    start_of_move = SystemTime::now()
                }

                was_moving = is_moving;

                let mut multiplier = 1.0;
                let mut max_speed = 1.7;
                if is_aiming {
                    multiplier *= sight_multiplier;
                }

                multiplier *= barrel_multiplier;

                if !is_crouching {
                    multiplier *= 2.0;
                    max_speed = 2.8;
                }

                if is_moving {
                    let vc = calculate_recursive_bezier(
                        &vec![0f64, max_speed],
                        &((start_of_move.elapsed().unwrap().as_millis()) as f64),
                    );
                    let vt = (vc / 5.5f64).clamp(0.0, 1.0);
                    multiplier *= 1.0 + vt * movement_penalty;
                }

                move_to(
                    &Vec2::new(
                        -(delta.x * multiplier / (-0.3 * sensitivity_multiplier)),
                        -(delta.y * multiplier / (-0.3 * sensitivity_multiplier)),
                    ),
                    &acceleration,
                    &current_weapon.delay,
                );
            }
        }
    });
}
