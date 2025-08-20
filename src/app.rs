use crate::player::MusicPlayer;
use crate::ui::{Controls, SpotifyLayout, SpotifyTheme};
use eframe::egui;
use std::time::Duration;

pub struct RustifyApp {
    pub player: MusicPlayer,
}

impl Default for RustifyApp {
    fn default() -> Self {
        Self {
            player: MusicPlayer::default(),
        }
    }
}

impl eframe::App for RustifyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply Spotify theme
        SpotifyTheme::apply_to_context(ctx);
        
        // Update player position and handle auto-play
        self.player.update_position();

        // Main layout with three panels
        egui::CentralPanel::default().show(ctx, |ui| {
            let (folder_to_toggle, song_to_play, drag_started, should_clear_drag, move_song_action) =
                SpotifyLayout::show(&mut self.player, ctx, ui);

            // Apply collected interactions
            if let Some(folder_idx) = folder_to_toggle {
                self.player.toggle_folder_expanded(folder_idx);
            }

            if let Some((folder_idx, song_idx)) = song_to_play {
                self.player.play_song(folder_idx, song_idx);
            }

            if let Some((folder_idx, song_idx)) = drag_started {
                self.player.set_dragged_song(folder_idx, song_idx);
            }

            if let Some((from_folder, song_idx, to_folder)) = move_song_action {
                self.player.move_song_to_folder(from_folder, song_idx, to_folder);
            }

            if should_clear_drag {
                self.player.clear_dragged_song();
            }
        });

        // Bottom panel for modern player controls
        egui::TopBottomPanel::bottom("player_controls")
            .resizable(false)
            .exact_height(100.0)
            .show(ctx, |ui| {
                Controls::show(&mut self.player, ui);
            });

        // Request repaint for smooth updates
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}
