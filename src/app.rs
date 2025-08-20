use crate::player::MusicPlayer;
use crate::ui::{Controls, SpotifyLayout, SpotifyTheme};
use eframe::egui;
use std::time::Duration;

pub struct RustifyApp {
    pub player: MusicPlayer,
    pub layout: SpotifyLayout,
}

impl Default for RustifyApp {
    fn default() -> Self {
        Self {
            player: MusicPlayer::default(),
            layout: SpotifyLayout::default(),
        }
    }
}

impl eframe::App for RustifyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply Spotify theme
        SpotifyTheme::apply_to_context(ctx);
        
        // Handle keyboard shortcuts
        self.handle_keyboard_shortcuts(ctx);
        
        // Update player position and handle auto-play
        self.player.update_position();

        // Main layout with three panels
        egui::CentralPanel::default().show(ctx, |ui| {
            let (folder_to_toggle, song_to_play, drag_started, should_clear_drag, move_song_action) =
                self.layout.show(&mut self.player, ctx, ui);

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

impl RustifyApp {
    /// Handle keyboard shortcuts for better usability
    fn handle_keyboard_shortcuts(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            // Spacebar: Play/Pause
            if i.key_pressed(egui::Key::Space) {
                if self.player.is_playing {
                    self.player.pause();
                } else if self.player.current_song.is_some() {
                    self.player.resume();
                }
            }
            
            // Arrow keys: Previous/Next
            if i.key_pressed(egui::Key::ArrowLeft) {
                self.player.play_previous_song();
            }
            
            if i.key_pressed(egui::Key::ArrowRight) {
                self.player.play_next_song();
            }
            
            // Escape: Stop
            if i.key_pressed(egui::Key::Escape) {
                self.player.stop();
            }
            
            // F5: Refresh
            if i.key_pressed(egui::Key::F5) {
                self.player.refresh_folders();
            }
        });
    }
}
