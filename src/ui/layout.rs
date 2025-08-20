use crate::player::MusicPlayer;
use crate::ui::{SongList, SpotifyTheme};
use eframe::egui;

pub struct SpotifyLayout;

impl SpotifyLayout {
    /// Show the main Spotify-like layout with three panels
    pub fn show(
        player: &mut MusicPlayer,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
    ) -> (
        Option<usize>,                 // folder_to_toggle
        Option<(usize, usize)>,        // song_to_play
        Option<(usize, usize)>,        // drag_started
        bool,                          // should_clear_drag
        Option<(usize, usize, usize)>, // move_song_action
    ) {
        let mut folder_to_toggle = None;
        let mut song_to_play = None;
        let mut drag_started = None;
        let mut should_clear_drag = false;
        let mut move_song_action = None;

        // Main horizontal layout
        ui.horizontal(|ui| {
            // Left sidebar (Library/Playlists) - 250px fixed width
            ui.allocate_ui_with_layout(
                egui::vec2(250.0, ui.available_height()),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    SpotifyTheme::sidebar_frame().show(ui, |ui| {
                        Self::show_sidebar(player, ui);
                    });
                },
            );

            ui.separator();

            // Main content area (takes remaining width)
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), ui.available_height()),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    SpotifyTheme::card_frame().show(ui, |ui| {
                        // Main content header
                        ui.horizontal(|ui| {
                            ui.heading("🎵 Your Music");
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.add(SpotifyTheme::styled_button("🔄 Refresh", false)).clicked() {
                                    player.refresh_folders();
                                }
                            });
                        });
                        
                        ui.separator();

                        // Music content
                        let (ft, stp, ds, scd, msa) = Self::show_main_content(player, ui, ctx);
                        folder_to_toggle = ft;
                        song_to_play = stp;
                        drag_started = ds;
                        should_clear_drag = scd;
                        move_song_action = msa;
                    });
                },
            );
        });

        (folder_to_toggle, song_to_play, drag_started, should_clear_drag, move_song_action)
    }

    /// Show the left sidebar with library navigation
    fn show_sidebar(player: &MusicPlayer, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // Logo/Title
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("🎵 Rustify").size(20.0).strong());
            });
            
            ui.add_space(16.0);

            // Navigation sections
            ui.label(egui::RichText::new("YOUR LIBRARY").size(12.0).color(SpotifyTheme::default().text_secondary));
            ui.add_space(8.0);

            // Library stats
            let total_songs: usize = player.folders.iter().map(|f| f.songs.len()).sum();
            let total_folders = player.folders.len();

            Self::sidebar_item(ui, &format!("📁 {} Folders", total_folders), false);
            Self::sidebar_item(ui, &format!("🎵 {} Songs", total_songs), false);
            
            ui.add_space(16.0);

            // Folder list
            if !player.folders.is_empty() {
                ui.label(egui::RichText::new("FOLDERS").size(12.0).color(SpotifyTheme::default().text_secondary));
                ui.add_space(8.0);

                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .max_height(300.0)
                    .show(ui, |ui| {
                        for folder in &player.folders {
                            let is_active = player.current_folder_index
                                .map_or(false, |idx| idx < player.folders.len() && 
                                        &player.folders[idx].name == &folder.name);
                            
                            Self::sidebar_item(
                                ui, 
                                &format!("📁 {} ({})", folder.name, folder.songs.len()), 
                                is_active
                            );
                        }
                    });
            }

            // Push now playing info to bottom
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                if let Some(ref song) = player.current_song {
                    ui.separator();
                    ui.add_space(8.0);
                    
                    ui.label(egui::RichText::new("NOW PLAYING").size(10.0).color(SpotifyTheme::default().text_secondary));
                    ui.label(egui::RichText::new(&song.name).size(12.0).strong());
                    
                    // Simple play/pause for sidebar
                    ui.horizontal(|ui| {
                        let play_button = if player.is_playing { "⏸" } else { "▶" };
                        if ui.small_button(play_button).clicked() {
                            if player.is_playing {
                                // This will be handled by the main controls
                            }
                        }
                    });
                }
            });
        });
    }

    /// Helper for sidebar navigation items
    fn sidebar_item(ui: &mut egui::Ui, text: &str, active: bool) {
        let theme = SpotifyTheme::default();
        let text_color = if active { theme.accent } else { theme.text_secondary };
        
        let response = ui.add(
            egui::Button::new(egui::RichText::new(text).color(text_color))
                .fill(if active { theme.selected } else { egui::Color32::TRANSPARENT })
                .frame(false)
                .min_size(egui::vec2(ui.available_width(), 32.0))
        );

        if !active && response.hovered() {
            ui.painter().rect_filled(
                response.rect, 
                egui::Rounding::same(6.0), 
                theme.hover
            );
        }
    }

    /// Show the main content area
    fn show_main_content(
        player: &mut MusicPlayer,
        ui: &mut egui::Ui,
        ctx: &egui::Context,
    ) -> (
        Option<usize>,                 // folder_to_toggle
        Option<(usize, usize)>,        // song_to_play
        Option<(usize, usize)>,        // drag_started
        bool,                          // should_clear_drag
        Option<(usize, usize, usize)>, // move_song_action
    ) {
        if player.folders.is_empty() {
            // Empty state with better styling
            ui.vertical_centered(|ui| {
                ui.add_space(50.0);
                ui.label(egui::RichText::new("🎵").size(48.0));
                ui.add_space(16.0);
                ui.label(egui::RichText::new("No music found").size(18.0).strong());
                ui.label(egui::RichText::new("Add music files to ~/Downloads/music").color(SpotifyTheme::default().text_secondary));
                ui.add_space(8.0);
                ui.label(egui::RichText::new("Supported: MP3, WAV, FLAC, OGG").color(SpotifyTheme::default().text_secondary));
            });
            
            (None, None, None, false, None)
        } else {
            // Use enhanced song list
            SongList::show_enhanced(player, ui, ctx)
        }
    }
}