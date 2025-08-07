use crate::player::MusicPlayer;
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
