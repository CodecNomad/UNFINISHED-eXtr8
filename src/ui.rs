#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::egui;

pub fn init() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let mut sensitivity: f32 = 0.03f32;

    eframe::run_simple_native("eXtr8", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                ui.heading("Sensitivity: ");
                ui.add(egui::Slider::new(&mut sensitivity, 0f32..=1f32));
            });
        });
    })
    .unwrap();
}
