use crate::player::MusicPlayer;
use crate::ui::SpotifyTheme;
use crate::utils::format_duration;
use eframe::egui;

pub struct SongList;

impl SongList {
    pub fn show(
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
        // Use the enhanced version by default
        Self::show_enhanced(player, ui, ctx)
    }

    /// Enhanced song list with modern card-based design
    pub fn show_enhanced(
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
        let mut folder_to_toggle = None;
        let mut song_to_play = None;
        let mut drag_started = None;
        let mut should_clear_drag = false;
        let mut move_song_action = None;
        let theme = SpotifyTheme::default();

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for (folder_idx, folder) in player.folders.iter().enumerate() {
                    // Folder card with modern styling
                    let folder_frame = egui::Frame::default()
                        .fill(theme.surface_variant)
                        .rounding(egui::Rounding::same(8.0))
                        .stroke(egui::Stroke::new(1.0, theme.border))
                        .inner_margin(egui::style::Margin::same(12.0));

                    folder_frame.show(ui, |ui| {
                        // Folder header with expand/collapse
                        let folder_response = ui.horizontal(|ui| {
                            let arrow = if folder.expanded { "▼" } else { "▶" };
                            let folder_text = format!("{} 📁 {}", arrow, folder.name);
                            
                            if ui.add(
                                egui::Button::new(egui::RichText::new(&folder_text).size(16.0).strong())
                                    .fill(egui::Color32::TRANSPARENT)
                                    .frame(false)
                            ).clicked() {
                                folder_to_toggle = Some(folder_idx);
                            }

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(
                                    egui::RichText::new(&format!("{} songs", folder.song_count()))
                                        .size(12.0)
                                        .color(theme.text_secondary)
                                );
                            });
                        });

                        // Drop target styling for folder
                        let folder_rect = folder_response.response.rect;
                        if let Some((_drag_folder, _drag_song)) = player.dragged_song {
                            if folder_rect.contains(ctx.input(|i| i.pointer.hover_pos()).unwrap_or_default()) {
                                ui.painter().rect_stroke(
                                    folder_rect,
                                    8.0,
                                    egui::Stroke::new(2.0, theme.primary),
                                );
                            }
                        }

                        // Songs list when expanded
                        if folder.expanded {
                            ui.add_space(8.0);
                            ui.separator();
                            ui.add_space(8.0);

                            for (song_idx, song) in folder.songs.iter().enumerate() {
                                let is_current = player.is_current_song(folder_idx, song_idx);
                                
                                Self::show_song_card(
                                    ui, 
                                    song, 
                                    is_current, 
                                    &mut song_to_play, 
                                    &mut drag_started,
                                    folder_idx, 
                                    song_idx,
                                    player.dragged_song
                                );
                            }

                            // Handle drop on folder
                            if let Some((drag_folder, drag_song)) = player.dragged_song {
                                if ctx.input(|i| i.pointer.any_released()) {
                                    if folder_rect.contains(
                                        ctx.input(|i| i.pointer.hover_pos()).unwrap_or_default(),
                                    ) {
                                        move_song_action = Some((drag_folder, drag_song, folder_idx));
                                    }
                                    should_clear_drag = true;
                                }
                            }
                        }
                    });

                    ui.add_space(12.0); // Space between folder cards
                }
            });

        (folder_to_toggle, song_to_play, drag_started, should_clear_drag, move_song_action)
    }

    /// Show individual song as a card
    fn show_song_card(
        ui: &mut egui::Ui,
        song: &crate::models::Song,
        is_current: bool,
        song_to_play: &mut Option<(usize, usize)>,
        drag_started: &mut Option<(usize, usize)>,
        folder_idx: usize,
        song_idx: usize,
        dragged_song: Option<(usize, usize)>,
    ) {
        let theme = SpotifyTheme::default();
        
        // Song card styling
        let song_bg = if is_current { 
            theme.selected 
        } else { 
            theme.surface 
        };

        let song_frame = egui::Frame::default()
            .fill(song_bg)
            .rounding(egui::Rounding::same(6.0))
            .stroke(if is_current { 
                egui::Stroke::new(1.0, theme.primary)
            } else {
                egui::Stroke::new(0.5, theme.border)
            })
            .inner_margin(egui::style::Margin::same(8.0));

        let song_response = song_frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                // Play indicator or album art placeholder
                let icon = if is_current { "▶" } else { "🎵" };
                let icon_color = if is_current { theme.primary } else { theme.text_secondary };
                
                ui.label(egui::RichText::new(icon).size(16.0).color(icon_color));

                ui.add_space(8.0);

                // Song info
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        let song_name = egui::RichText::new(&song.name)
                            .size(14.0)
                            .color(if is_current { theme.accent } else { theme.text_primary });
                        
                        ui.label(song_name);
                        
                        // Duration on the right
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if let Some(duration) = song.duration {
                                ui.label(
                                    egui::RichText::new(format_duration(duration))
                                        .size(12.0)
                                        .color(theme.text_secondary)
                                );
                            }
                        });
                    });
                });
            });
        }).response;

        // Handle interactions
        if song_response.clicked() {
            *song_to_play = Some((folder_idx, song_idx));
        }

        if song_response.drag_started() {
            *drag_started = Some((folder_idx, song_idx));
        }

        // Hover effect
        if song_response.hovered() && !is_current {
            ui.painter().rect_filled(
                song_response.rect,
                egui::Rounding::same(6.0),
                theme.hover,
            );
        }

        // Drag visual feedback
        if dragged_song == Some((folder_idx, song_idx)) {
            ui.painter().rect_stroke(
                song_response.rect,
                6.0,
                egui::Stroke::new(2.0, theme.accent),
            );
        }

        ui.add_space(4.0); // Space between songs
    }

    /// Legacy show method for backward compatibility
    pub fn show_legacy(
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
        let mut folder_to_toggle = None;
        let mut song_to_play = None;
        let mut drag_started = None;
        let mut should_clear_drag = false;
        let mut move_song_action = None;

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                if player.folders.is_empty() {
                    ui.label("No music files found in ~/Downloads/music");
                    ui.label("Supported formats: MP3, WAV, FLAC, OGG");
                    ui.label("Create folders in the music directory to organize your songs");
                } else {
                    for (folder_idx, folder) in player.folders.iter().enumerate() {
                        // Folder header
                        let folder_response = ui.horizontal(|ui| {
                            let arrow = if folder.expanded { "▼" } else { "▶" };
                            if ui
                                .button(format!(
                                    "{} 📁 {} ({})",
                                    arrow,
                                    folder.name,
                                    folder.song_count()
                                ))
                                .clicked()
                            {
                                folder_to_toggle = Some(folder_idx);
                            }
                        });

                        // Drop target for folder
                        let folder_rect = folder_response.response.rect;
                        if let Some((_drag_folder, _drag_song)) = player.dragged_song {
                            if folder_rect
                                .contains(ctx.input(|i| i.pointer.hover_pos()).unwrap_or_default())
                            {
                                ui.painter().rect_stroke(
                                    folder_rect,
                                    5.0,
                                    egui::Stroke::new(2.0, egui::Color32::GREEN),
                                );
                            }
                        }

                        if folder.expanded {
                            ui.indent(folder_idx, |ui| {
                                for (song_idx, song) in folder.songs.iter().enumerate() {
                                    let is_current = player.is_current_song(folder_idx, song_idx);

                                    let button_text = if is_current {
                                        format!("▶ 🎵 {}", song.name)
                                    } else {
                                        format!("🎵 {}", song.name)
                                    };

                                    let song_response = ui.add(
                                        egui::Button::new(&button_text)
                                            .min_size(egui::vec2(300.0, 20.0)),
                                    );

                                    // Handle click to play
                                    if song_response.clicked() {
                                        song_to_play = Some((folder_idx, song_idx));
                                    }

                                    // Handle drag
                                    if song_response.drag_started() {
                                        drag_started = Some((folder_idx, song_idx));
                                    }

                                    // Visual feedback for dragging
                                    if player.dragged_song == Some((folder_idx, song_idx)) {
                                        ui.painter().rect_stroke(
                                            song_response.rect,
                                            5.0,
                                            egui::Stroke::new(2.0, egui::Color32::YELLOW),
                                        );
                                    }
                                }

                                // Handle drop on folder
                                if let Some((drag_folder, drag_song)) = player.dragged_song {
                                    if ctx.input(|i| i.pointer.any_released()) {
                                        if folder_rect.contains(
                                            ctx.input(|i| i.pointer.hover_pos())
                                                .unwrap_or_default(),
                                        ) {
                                            move_song_action =
                                                Some((drag_folder, drag_song, folder_idx));
                                        }
                                        should_clear_drag = true;
                                    }
                                }
                            });
                        }

                        ui.separator();
                    }
                }
            });

        (
            folder_to_toggle,
            song_to_play,
            drag_started,
            should_clear_drag,
            move_song_action,
        )
    }
}
