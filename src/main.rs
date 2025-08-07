use eframe::egui;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

use std::fs;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct Song {
    path: PathBuf,
    name: String,
}

#[derive(Clone)]
struct Folder {
    name: String,
    path: PathBuf,
    songs: Vec<Song>,
    expanded: bool,
}

struct MusicPlayer {
    folders: Vec<Folder>,
    current_song: Option<Song>,
    current_folder_index: Option<usize>,
    current_song_index: Option<usize>,
    sink: Sink,
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    is_playing: bool,
    song_start_time: Option<Instant>,
    current_position: Duration,
    dragged_song: Option<(usize, usize)>, // (folder_index, song_index)
}

impl Default for MusicPlayer {
    fn default() -> Self {
        let music_dir = PathBuf::from("/home")
            .join(std::env::var("USER").unwrap_or_default())
            .join("Downloads/music");

        let folders = Self::load_folders(&music_dir);

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        Self {
            folders,
            current_song: None,
            current_folder_index: None,
            current_song_index: None,
            sink,
            _stream,
            _stream_handle: stream_handle,
            is_playing: false,
            song_start_time: None,
            current_position: Duration::from_secs(0),
            dragged_song: None,
        }
    }
}

impl MusicPlayer {
    fn load_folders(music_dir: &PathBuf) -> Vec<Folder> {
        let mut folders = Vec::new();

        if !music_dir.exists() {
            return folders;
        }

        // First, add songs in the root music directory
        let root_songs = Self::load_songs_from_dir(music_dir);
        if !root_songs.is_empty() {
            folders.push(Folder {
                name: "Music".to_string(),
                path: music_dir.clone(),
                songs: root_songs,
                expanded: true,
            });
        }

        // Then, add subdirectories as folders
        if let Ok(entries) = fs::read_dir(music_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let songs = Self::load_songs_from_dir(&path);
                    if !songs.is_empty() {
                        folders.push(Folder {
                            name: path.file_name().unwrap().to_string_lossy().to_string(),
                            path,
                            songs,
                            expanded: false,
                        });
                    }
                }
            }
        }

        folders
    }

    fn load_songs_from_dir(dir: &PathBuf) -> Vec<Song> {
        let mut songs = Vec::new();

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if let Some(ext_str) = ext.to_str() {
                            if matches!(
                                ext_str.to_lowercase().as_str(),
                                "mp3" | "wav" | "flac" | "ogg"
                            ) {
                                songs.push(Song {
                                    name: path.file_name().unwrap().to_string_lossy().to_string(),
                                    path: path.clone(),
                                });
                            }
                        }
                    }
                }
            }
        }

        songs.sort_by(|a, b| a.name.cmp(&b.name));
        songs
    }

    fn move_song_to_folder(&mut self, from_folder: usize, song_index: usize, to_folder: usize) {
        if from_folder >= self.folders.len()
            || to_folder >= self.folders.len()
            || from_folder == to_folder
        {
            return;
        }

        let song = {
            let from_folder_ref = &mut self.folders[from_folder];
            if song_index >= from_folder_ref.songs.len() {
                return;
            }
            from_folder_ref.songs.remove(song_index)
        };

        let new_path = self.folders[to_folder].path.join(&song.name);

        // Move file in filesystem
        if let Err(_) = fs::rename(&song.path, &new_path) {
            // If move failed, put the song back
            self.folders[from_folder].songs.insert(song_index, song);
            return;
        }

        // Update song path and add to new folder
        let mut moved_song = song;
        moved_song.path = new_path;
        self.folders[to_folder].songs.push(moved_song);
        self.folders[to_folder]
            .songs
            .sort_by(|a, b| a.name.cmp(&b.name));

        // Update current song tracking if necessary
        if let (Some(current_folder), Some(current_song)) =
            (self.current_folder_index, self.current_song_index)
        {
            if current_folder == from_folder && current_song == song_index {
                // The currently playing song was moved
                self.current_folder_index = Some(to_folder);
                if let Some(new_index) = self.folders[to_folder]
                    .songs
                    .iter()
                    .position(|s| s.name == self.current_song.as_ref().unwrap().name)
                {
                    self.current_song_index = Some(new_index);
                }
            } else if current_folder == from_folder && current_song > song_index {
                // Adjust current song index in the same folder
                self.current_song_index = Some(current_song - 1);
            }
        }
    }

    fn play_song(&mut self, folder_index: usize, song_index: usize) {
        if folder_index >= self.folders.len()
            || song_index >= self.folders[folder_index].songs.len()
        {
            return;
        }

        let song = &self.folders[folder_index].songs[song_index];
        self.current_song = Some(song.clone());
        self.current_folder_index = Some(folder_index);
        self.current_song_index = Some(song_index);

        // Stop current playback
        self.sink.stop();

        // Create new sink
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        self.sink = Sink::try_new(&stream_handle).unwrap();
        self._stream = _stream;
        self._stream_handle = stream_handle;

        if let Ok(file) = std::fs::File::open(&song.path) {
            if let Ok(source) = Decoder::new(BufReader::new(file)) {
                self.sink.append(source);
                self.sink.play();
                self.is_playing = true;
                self.song_start_time = Some(Instant::now());
                self.current_position = Duration::from_secs(0);
            }
        }
    }

    fn play_next_song(&mut self) {
        if let (Some(folder_idx), Some(song_idx)) =
            (self.current_folder_index, self.current_song_index)
        {
            if folder_idx < self.folders.len() {
                let folder = &self.folders[folder_idx];
                if song_idx + 1 < folder.songs.len() {
                    self.play_song(folder_idx, song_idx + 1);
                } else {
                    // Move to next folder
                    for next_folder_idx in (folder_idx + 1)..self.folders.len() {
                        if !self.folders[next_folder_idx].songs.is_empty() {
                            self.play_song(next_folder_idx, 0);
                            return;
                        }
                    }
                    // Wrap around to first folder
                    for next_folder_idx in 0..=folder_idx {
                        if !self.folders[next_folder_idx].songs.is_empty() {
                            self.play_song(next_folder_idx, 0);
                            return;
                        }
                    }
                }
            }
        }
    }

    fn play_previous_song(&mut self) {
        if let (Some(folder_idx), Some(song_idx)) =
            (self.current_folder_index, self.current_song_index)
        {
            if folder_idx < self.folders.len() {
                if song_idx > 0 {
                    self.play_song(folder_idx, song_idx - 1);
                } else {
                    // Move to previous folder
                    for prev_folder_idx in (0..folder_idx).rev() {
                        let folder = &self.folders[prev_folder_idx];
                        if !folder.songs.is_empty() {
                            self.play_song(prev_folder_idx, folder.songs.len() - 1);
                            return;
                        }
                    }
                    // Wrap around to last folder
                    for prev_folder_idx in (folder_idx..self.folders.len()).rev() {
                        let folder = &self.folders[prev_folder_idx];
                        if !folder.songs.is_empty() {
                            self.play_song(prev_folder_idx, folder.songs.len() - 1);
                            return;
                        }
                    }
                }
            }
        }
    }

    fn pause(&mut self) {
        self.sink.pause();
        self.is_playing = false;
    }

    fn resume(&mut self) {
        self.sink.play();
        self.is_playing = true;
        if self.song_start_time.is_some() {
            self.song_start_time = Some(Instant::now() - self.current_position);
        }
    }

    fn stop(&mut self) {
        self.sink.stop();
        self.is_playing = false;
        self.song_start_time = None;
        self.current_position = Duration::from_secs(0);
        self.current_song = None;
        self.current_folder_index = None;
        self.current_song_index = None;
    }
}

impl eframe::App for MusicPlayer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update current position if playing
        if self.is_playing && self.song_start_time.is_some() {
            let elapsed = self.song_start_time.unwrap().elapsed();
            self.current_position = elapsed;

            // Check if song finished and auto-play next
            if self.sink.empty() && self.current_song.is_some() {
                self.play_next_song();
            }
        }

        // Collect interactions separately to avoid borrow checker issues
        let mut folder_to_toggle = None;
        let mut song_to_play = None;
        let mut drag_started = None;
        let mut should_clear_drag = false;
        let mut move_song_action = None;

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎵 Rustify Music Player");
            ui.separator();

            // Song list with folders
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if self.folders.is_empty() {
                        ui.label("No music files found in ~/Downloads/music");
                        ui.label("Supported formats: MP3, WAV, FLAC, OGG");
                        ui.label("Create folders in the music directory to organize your songs");
                    } else {
                        for (folder_idx, folder) in self.folders.iter().enumerate() {
                            // Folder header
                            let folder_response = ui.horizontal(|ui| {
                                let arrow = if folder.expanded { "▼" } else { "▶" };
                                if ui
                                    .button(format!(
                                        "{} 📁 {} ({})",
                                        arrow,
                                        folder.name,
                                        folder.songs.len()
                                    ))
                                    .clicked()
                                {
                                    folder_to_toggle = Some(folder_idx);
                                }
                            });

                            // Drop target for folder
                            let folder_rect = folder_response.response.rect;
                            if let Some((_drag_folder, _drag_song)) = self.dragged_song {
                                if folder_rect.contains(
                                    ctx.input(|i| i.pointer.hover_pos()).unwrap_or_default(),
                                ) {
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
                                        let is_current = self.current_folder_index
                                            == Some(folder_idx)
                                            && self.current_song_index == Some(song_idx)
                                            && self.is_playing;

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
                                        if self.dragged_song == Some((folder_idx, song_idx)) {
                                            ui.painter().rect_stroke(
                                                song_response.rect,
                                                5.0,
                                                egui::Stroke::new(2.0, egui::Color32::YELLOW),
                                            );
                                        }
                                    }

                                    // Handle drop on folder
                                    if let Some((drag_folder, drag_song)) = self.dragged_song {
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
        });

        // Apply collected interactions
        if let Some(folder_idx) = folder_to_toggle {
            self.folders[folder_idx].expanded = !self.folders[folder_idx].expanded;
        }

        if let Some((folder_idx, song_idx)) = song_to_play {
            self.play_song(folder_idx, song_idx);
        }

        if let Some((folder_idx, song_idx)) = drag_started {
            self.dragged_song = Some((folder_idx, song_idx));
        }

        if let Some((from_folder, song_idx, to_folder)) = move_song_action {
            self.move_song_to_folder(from_folder, song_idx, to_folder);
        }

        if should_clear_drag {
            self.dragged_song = None;
        }

        // Bottom panel for controls
        egui::TopBottomPanel::bottom("controls").show(ctx, |ui| {
            ui.separator();

            // Current song info
            if let Some(ref song) = self.current_song {
                ui.horizontal(|ui| {
                    ui.label("Now playing:");
                    ui.label(&song.name);
                });
            }

            // Time display (only current position)
            ui.horizontal(|ui| {
                let current_str = format_duration(self.current_position);
                ui.label(format!("⏱ {}", current_str));
            });

            ui.separator();

            // Player controls
            ui.horizontal(|ui| {
                if ui.button("⏮").clicked() {
                    self.play_previous_song();
                }

                if self.is_playing {
                    if ui.button("⏸").clicked() {
                        self.pause();
                    }
                } else {
                    if ui.button("▶").clicked() {
                        if self.current_song.is_some() {
                            self.resume();
                        }
                    }
                }

                if ui.button("⏹").clicked() {
                    self.stop();
                }

                if ui.button("⏭").clicked() {
                    self.play_next_song();
                }

                ui.separator();

                if ui.button("🔄 Refresh").clicked() {
                    let music_dir = PathBuf::from("/home")
                        .join(std::env::var("USER").unwrap_or_default())
                        .join("Downloads/music");
                    self.folders = Self::load_folders(&music_dir);
                }
            });
        });

        // Request repaint for smooth updates
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{}:{:02}", minutes, seconds)
}

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
        Box::new(|_cc| Box::new(MusicPlayer::default())),
    )
}
