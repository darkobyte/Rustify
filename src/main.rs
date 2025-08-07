mod app;
mod models;
mod player;
mod ui;
mod utils;

use app::RustifyApp;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 700.0])
            .with_title("Rustify Music Player"),
        ..Default::default()
    };

    eframe::run_native(
        "Rustify",
        options,
        Box::new(|_cc| Box::new(RustifyApp::default())),
    )
}
