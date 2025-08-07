use crate::player::MusicPlayer;
use crate::utils::format_duration;
use eframe::egui;

pub struct Controls;

impl Controls {
    pub fn show(player: &mut MusicPlayer, ui: &mut egui::Ui) {
        ui.separator();

        // Current song info
        if let Some(ref song) = player.current_song {
            ui.horizontal(|ui| {
                ui.label("Now playing:");
                ui.label(&song.name);
            });
        }

        // Time display (only current position)
        ui.horizontal(|ui| {
            let current_str = format_duration(player.current_position);
            ui.label(format!("⏱ {}", current_str));
        });

        ui.separator();

        // Player controls
        ui.horizontal(|ui| {
            if ui.button("⏮").clicked() {
                player.play_previous_song();
            }

            if player.is_playing {
                if ui.button("⏸").clicked() {
                    player.pause();
                }
            } else {
                if ui.button("▶").clicked() {
                    if player.current_song.is_some() {
                        player.resume();
                    }
                }
            }

            if ui.button("⏹").clicked() {
                player.stop();
            }

            if ui.button("⏭").clicked() {
                player.play_next_song();
            }

            ui.separator();

            if ui.button("🔄 Refresh").clicked() {
                player.refresh_folders();
            }
        });
    }
}
