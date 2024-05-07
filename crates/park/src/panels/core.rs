use eframe::egui;

use crate::ParkApp;

pub fn view(ctx: &egui::Context, _app: &mut ParkApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading("Choose a file");
                ui.add_space(10.0);
            });
            ui.vertical(|ui| {
                ui.heading("Table area");
            });
        });
    });
}
