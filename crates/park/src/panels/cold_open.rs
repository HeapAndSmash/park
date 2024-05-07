use eframe::egui;

use crate::ParkApp;

pub fn view(ctx: &egui::Context, app: &mut ParkApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading("Choose a file");
                ui.add_space(10.0);
                if ui.button("Open file").clicked() {
                    let file = rfd::FileDialog::new()
                        .add_filter("parquet", &["parquet"])
                        .pick_file();

                    if let Some(path) = file {
                        app.set_parquet_file(path.into_os_string().into_string().unwrap());
                    }
                };
            });
            ui.vertical(|ui| {
                ui.heading("Table area");
            });
        });
    });
}
