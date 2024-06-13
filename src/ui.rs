#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::mpsc::Sender;

use eframe::egui;

use crate::config::Settings;

pub fn init(tx: Sender<Settings>) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let mut sensitivity: f32 = 0.3f32;

    eframe::run_simple_native("eXtr8", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                ui.heading("Sensitivity: ");
                ui.add(egui::Slider::new(&mut sensitivity, 0.005f32..=5f32));
            });
            if ui.add(egui::Button::new("Update settings")).clicked() {
                tx.send(Settings::new(sensitivity)).unwrap();
            }
        });
    })
    .unwrap();
}
