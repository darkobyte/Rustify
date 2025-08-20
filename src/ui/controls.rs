use crate::player::MusicPlayer;
use crate::utils::format_duration;
use crate::ui::SpotifyTheme;
use eframe::egui;

pub struct Controls;

impl Controls {
    /// Modern Spotify-like bottom player bar
    pub fn show(player: &mut MusicPlayer, ui: &mut egui::Ui) {
        let theme = SpotifyTheme::default();
        
        // Player bar with dark background
        let player_frame = egui::Frame::default()
            .fill(theme.surface_variant)
            .stroke(egui::Stroke::new(1.0, theme.border))
            .inner_margin(egui::style::Margin::same(12.0));

        player_frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                // Left section: Current song info
                ui.allocate_ui_with_layout(
                    egui::vec2(250.0, ui.available_height()),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        Self::show_now_playing_info(player, ui);
                    },
                );

                ui.separator();

                // Center section: Player controls
                ui.allocate_ui_with_layout(
                    egui::vec2(300.0, ui.available_height()),
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        Self::show_player_controls(player, ui);
                    },
                );

                ui.separator();

                // Right section: Volume and additional controls
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    Self::show_additional_controls(player, ui);
                });
            });
        });
    }

    /// Show current song information with album art placeholder
    fn show_now_playing_info(player: &MusicPlayer, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Album art placeholder
            let art_size = egui::vec2(56.0, 56.0);
            let (rect, _) = ui.allocate_exact_size(art_size, egui::Sense::hover());
            
            ui.painter().rect_filled(
                rect,
                egui::Rounding::same(4.0),
                SpotifyTheme::default().surface,
            );
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "🎵",
                egui::FontId::default(),
                SpotifyTheme::default().text_secondary,
            );

            ui.add_space(12.0);

            // Song info
            if let Some(ref song) = player.current_song {
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(&song.name).size(14.0).strong());
                    
                    // Show folder/artist info if available
                    if let Some(folder_idx) = player.current_folder_index {
                        if folder_idx < player.folders.len() {
                            ui.label(
                                egui::RichText::new(&player.folders[folder_idx].name)
                                    .size(12.0)
                                    .color(SpotifyTheme::default().text_secondary)
                            );
                        }
                    }
                });
            } else {
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("No song playing").color(SpotifyTheme::default().text_secondary));
                });
            }
        });
    }

    /// Show main player controls
    fn show_player_controls(player: &mut MusicPlayer, ui: &mut egui::Ui) {
        // Control buttons row
        ui.horizontal(|ui| {
            ui.add_space(20.0); // Center the controls

            // Previous button
            if ui.add(
                egui::Button::new("⏮")
                    .min_size(egui::vec2(40.0, 40.0))
                    .rounding(egui::Rounding::same(20.0))
                    .fill(egui::Color32::TRANSPARENT)
            ).clicked() {
                player.play_previous_song();
            }

            ui.add_space(8.0);

            // Play/Pause button (larger, primary)
            let play_button_text = if player.is_playing { "⏸" } else { "▶" };
            if ui.add(
                SpotifyTheme::styled_button(play_button_text, true)
                    .min_size(egui::vec2(48.0, 48.0))
                    .rounding(egui::Rounding::same(24.0))
            ).clicked() {
                if player.is_playing {
                    player.pause();
                } else if player.current_song.is_some() {
                    player.resume();
                }
            }

            ui.add_space(8.0);

            // Next button
            if ui.add(
                egui::Button::new("⏭")
                    .min_size(egui::vec2(40.0, 40.0))
                    .rounding(egui::Rounding::same(20.0))
                    .fill(egui::Color32::TRANSPARENT)
            ).clicked() {
                player.play_next_song();
            }
        });

        ui.add_space(8.0);

        // Progress bar row
        ui.horizontal(|ui| {
            // Current time
            let current_str = format_duration(player.current_position);
            ui.label(egui::RichText::new(current_str).size(11.0).color(SpotifyTheme::default().text_secondary));

            // Progress bar (simplified for now)
            let _progress_response = ui.add(
                egui::ProgressBar::new(0.5) // Placeholder progress
                    .fill(SpotifyTheme::default().primary)
                    .animate(false)
            );

            // Total time (if available)
            if let Some(ref song) = player.current_song {
                if let Some(duration) = song.duration {
                    let total_str = format_duration(duration);
                    ui.label(egui::RichText::new(total_str).size(11.0).color(SpotifyTheme::default().text_secondary));
                } else {
                    ui.label(egui::RichText::new("--:--").size(11.0).color(SpotifyTheme::default().text_secondary));
                }
            } else {
                ui.label(egui::RichText::new("--:--").size(11.0).color(SpotifyTheme::default().text_secondary));
            }
        });
    }

    /// Show additional controls (volume, etc.)
    fn show_additional_controls(player: &mut MusicPlayer, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Volume control (placeholder)
            ui.label("🔊");
            ui.add(egui::Slider::new(&mut 0.7f32, 0.0..=1.0).show_value(false));

            ui.add_space(16.0);

            // Stop button
            if ui.add(
                egui::Button::new("⏹")
                    .min_size(egui::vec2(32.0, 32.0))
                    .rounding(egui::Rounding::same(16.0))
                    .fill(egui::Color32::TRANSPARENT)
            ).clicked() {
                player.stop();
            }
        });
    }

    /// Legacy controls for backward compatibility
    pub fn show_legacy(player: &mut MusicPlayer, ui: &mut egui::Ui) {
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
