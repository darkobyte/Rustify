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
            .with_inner_size([1000.0, 700.0])  // Larger window for better layout
            .with_min_inner_size([800.0, 600.0])  // Minimum size for usability
            .with_title("Rustify Music Player - Spotify Style"),
        ..Default::default()
    };

    eframe::run_native(
        "Rustify",
        options,
        Box::new(|_cc| Box::new(RustifyApp::default())),
    )
}
