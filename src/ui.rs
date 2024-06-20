use std::process::exit;
use std::sync::mpsc::Sender;

use eframe::egui::{self};

use crate::config::{BarrelID, Settings, SightID, WeaponID};

pub fn init(tx: Sender<Settings>) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_resizable(false),
        ..Default::default()
    };

    let mut settings = Settings::default();
    eframe::run_simple_native("eXtr8", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("eXtr8 settings");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Sensitivity: ");
                ui.add(egui::Slider::new(&mut settings.sensitivity, 0.05f64..=1f64));
            });

            ui.horizontal(|ui| {
                ui.label("Weapon: ");
                egui::ComboBox::new("weapon_selector", "")
                    .selected_text(format!("{:?}", &settings.weapon))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut settings.weapon, WeaponID::M2, "M2");
                        ui.selectable_value(&mut settings.weapon, WeaponID::HmLmg, "HmLmg");
                        ui.selectable_value(&mut settings.weapon, WeaponID::Ak47, "Ak47");
                        ui.selectable_value(&mut settings.weapon, WeaponID::Lr300, "Lr300");
                        ui.selectable_value(&mut settings.weapon, WeaponID::Mp5, "Mp5");
                        ui.selectable_value(&mut settings.weapon, WeaponID::Thompson, "Thompson");
                        ui.selectable_value(&mut settings.weapon, WeaponID::Custom, "Custom");
                        ui.selectable_value(&mut settings.weapon, WeaponID::Semi, "Semi");
                        ui.selectable_value(&mut settings.weapon, WeaponID::Python, "Python");
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Sight: ");
                egui::ComboBox::new("sight_selector", "")
                    .selected_text(format!("{:?}", &settings.sight))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut settings.sight, SightID::None, "None");
                        ui.selectable_value(&mut settings.sight, SightID::Handmade, "Handmade");
                        ui.selectable_value(&mut settings.sight, SightID::Holo, "Holo");
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Barrel: ");
                egui::ComboBox::new("barrel_selector", "")
                    .selected_text(format!("{:?}", &settings.barrel))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut settings.barrel, BarrelID::None, "None");
                        ui.selectable_value(&mut settings.barrel, BarrelID::Silencer, "Silencer");
                    });
            });

            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("Update settings")).clicked() {
                    tx.send(settings).unwrap();
                }

                if ui.add(egui::Button::new("Exit")).clicked() {
                    exit(0);
                }
            });
        });
    })
    .unwrap();
}
