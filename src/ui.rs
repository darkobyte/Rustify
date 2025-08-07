use eframe::egui;

/// Custom UI components and utilities for the Rustify music player
pub struct UiHelpers;

impl UiHelpers {
    /// Format file size in human readable format
    pub fn format_file_size(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        format!("{:.1} {}", size, UNITS[unit_index])
    }

    /// Format duration in mm:ss format
    pub fn format_duration(seconds: f64) -> String {
        let total_seconds = seconds as u64;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }

    /// Apply dark theme colors
    pub fn apply_dark_theme(ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();

        // Customize colors for music player
        visuals.window_fill = egui::Color32::from_rgb(25, 25, 25);
        visuals.panel_fill = egui::Color32::from_rgb(30, 30, 30);
        visuals.faint_bg_color = egui::Color32::from_rgb(40, 40, 40);
        visuals.extreme_bg_color = egui::Color32::from_rgb(15, 15, 15);

        // Accent colors
        visuals.selection.bg_fill = egui::Color32::from_rgb(70, 70, 100);
        visuals.widgets.active.bg_fill = egui::Color32::from_rgb(80, 80, 120);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(60, 60, 90);

        ctx.set_visuals(visuals);
    }
}
