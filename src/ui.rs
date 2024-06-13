use std::{panic, sync::mpsc::Sender};

use eframe::egui::{self};

use crate::config::{BarrellID, Settings, SightID, WeaponID};

pub fn init(tx: Sender<Settings>) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_decorations(false)
            .with_resizable(false),
        ..Default::default()
    };

    let mut sensitivity: f32 = 0.3f32;
    let mut weapon: WeaponID = WeaponID::Ak47;
    let mut sight: SightID = SightID::None;
    let mut barrell: BarrellID = BarrellID::None;

    eframe::run_simple_native("eXtr8", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                ui.heading("Sensitivity: ");
                ui.add(egui::Slider::new(&mut sensitivity, 0.005f32..=5f32));
            });

            ui.horizontal_top(|ui| {
                ui.heading("Weapon: ");
                egui::ComboBox::new("weapon_selector", "")
                    .selected_text(format!("{:?}", &weapon))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut weapon, WeaponID::M2, "M2");
                        ui.selectable_value(&mut weapon, WeaponID::HmLmg, "HmLmg");
                        ui.selectable_value(&mut weapon, WeaponID::Ak47, "Ak47");
                        ui.selectable_value(&mut weapon, WeaponID::Lr300, "Lr300");
                        ui.selectable_value(&mut weapon, WeaponID::Mp5, "Mp5");
                        ui.selectable_value(&mut weapon, WeaponID::Thompson, "Thompson");
                        ui.selectable_value(&mut weapon, WeaponID::Custom, "Custom");
                        ui.selectable_value(&mut weapon, WeaponID::Semi, "Semi");
                        ui.selectable_value(&mut weapon, WeaponID::Python, "Python");
                    });
            });

            ui.horizontal_top(|ui| {
                ui.heading("Sight: ");
                egui::ComboBox::new("sight_selector", "")
                    .selected_text(format!("{:?}", &sight))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut sight, SightID::None, "None");
                        ui.selectable_value(&mut sight, SightID::Handmade, "Handmade");
                        ui.selectable_value(&mut sight, SightID::Holo, "Holo");
                        ui.selectable_value(&mut sight, SightID::X8, "X8");
                    });
            });

            ui.horizontal_top(|ui| {
                ui.heading("Barrell: ");
                egui::ComboBox::new("barrell_selector", "")
                    .selected_text(format!("{:?}", &barrell))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut barrell, BarrellID::None, "None");
                        ui.selectable_value(&mut barrell, BarrellID::Silencer, "Silencer 🍑💦");
                    });
            });

            ui.horizontal_top(|ui| {
                if ui.add(egui::Button::new("Update settings")).clicked() {
                    tx.send(Settings::new(sensitivity, weapon, sight, barrell))
                        .unwrap();
                }

                if ui.add(egui::Button::new("Exit")).clicked() {
                    panic!("User exit");
                }
            });
        });
    })
    .unwrap();
}
