#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use eframe::egui;

pub fn init() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let mut fps_saver: i8 = 0;

    eframe::run_simple_native("eXtr8", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                ui.heading("FPS saver (%): ");
                ui.add(egui::Slider::new(&mut fps_saver, 0..=100));
            });
        });
    })
    .unwrap();
}
