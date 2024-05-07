#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod panels;
mod pq_file;

use eframe::egui;

static DEFAULT_WIDTH: f32 = 1024.0;
static DEFAULT_HEIGHT: f32 = 840.0;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([DEFAULT_WIDTH, DEFAULT_HEIGHT]),
        ..Default::default()
    };
    eframe::run_native(
        "Park",
        options,
        Box::new(|_cc| {
            // This gives us image support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<ParkApp>::default()
        }),
    )
}

pub enum Panel {
    Files,
    Table,
    Pivot,
}

pub struct ParkApp {
    pub panel: Panel,
    pub file_path: Option<String>,
    pub file_schema: Option<String>,
}

impl Default for ParkApp {
    fn default() -> Self {
        Self {
            panel: Panel::Files,
            file_path: Option::None,
            file_schema: Option::None,
        }
    }
}

impl ParkApp {
    pub fn set_parquet_file(&mut self, pq_file: String) {
        self.file_path = Some(pq_file.clone());

        let schema = pq_file::get_parquet_schema(pq_file);
    }
}

impl eframe::App for ParkApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("file", |ui| {
                    if ui.button("About").clicked() {
                        println!("Clicked about");
                    }

                    if ui.button("Check for updates").clicked() {
                        // TODO: Check for updates from S3 bucket
                        println!("check for updates");
                    }
                });
            });
        });

        panels::cold_open::view(ctx, self);
        if Some(file_schema) = self.file_schema {
            panels::table::view(ctx, self);
        }
        // panels::core::view(ctx, self);

        // match self.page {
        //     Panel::Files => panels::files::view(ctx, self),
        //     Panel::Table => panels::table::view(ctx, self),
        //     Panel::Pivot => panels::pivot::view(ctx, self),
        // }
    }
}

