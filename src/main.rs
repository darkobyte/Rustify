use eframe::egui;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::{Duration, Instant};

struct MusicPlayer {
    songs: Vec<PathBuf>,
    current_song_index: usize,
    current_song_name: String,
    sink: Sink,
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    is_playing: bool,
    song_start_time: Option<Instant>,
    song_duration: Option<Duration>,
    current_position: Duration,
}

impl Default for MusicPlayer {
    fn default() -> Self {
        let music_dir = PathBuf::from("/home")
            .join(std::env::var("USER").unwrap_or_default())
            .join("Downloads/music");

        let mut songs = if music_dir.exists() {
            fs::read_dir(&music_dir)
                .unwrap_or_else(|_| fs::read_dir(".").unwrap())
                .filter_map(|entry| {
                    let path = entry.ok()?.path();
                    let ext = path.extension()?.to_str()?;
                    if ext == "mp3" || ext == "wav" || ext == "flac" || ext == "ogg" {
                        Some(path)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        songs.sort();

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        Self {
            songs,
            current_song_index: 0,
            current_song_name: String::new(),
            sink,
            _stream,
            _stream_handle: stream_handle,
            is_playing: false,
            song_start_time: None,
            song_duration: None,
            current_position: Duration::from_secs(0),
        }
    }
}

impl eframe::App for MusicPlayer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update current position if playing
        if self.is_playing && self.song_start_time.is_some() {
            let elapsed = self.song_start_time.unwrap().elapsed();
            self.current_position = elapsed;

            // Check if song finished and auto-play next
            if self.sink.empty() && !self.songs.is_empty() {
                self.play_next_song();
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎵 Rustify Music Player");
            ui.separator();

            // Song list
            egui::ScrollArea::vertical()
                .max_height(300.0)
                .show(ui, |ui| {
                    if self.songs.is_empty() {
                        ui.label("No music files found in ~/Downloads/music");
                        ui.label("Supported formats: MP3, WAV, FLAC, OGG");
                    } else {
                        let mut selected_index = None;
                        for (i, song) in self.songs.iter().enumerate() {
                            let name = song.file_name().unwrap().to_string_lossy();
                            let is_current = i == self.current_song_index && self.is_playing;

                            let button_text = if is_current {
                                format!("▶ {}", name)
                            } else {
                                name.to_string()
                            };

                            if ui.button(&button_text).clicked() {
                                selected_index = Some(i);
                            }
                        }

                        if let Some(index) = selected_index {
                            self.play_song_at_index(index);
                        }
                    }
                });

            ui.separator();

            // Current song info
            if !self.current_song_name.is_empty() {
                ui.horizontal(|ui| {
                    ui.label("Now playing:");
                    ui.label(&self.current_song_name);
                });
            }

            // Time display
            ui.horizontal(|ui| {
                let current_str = format_duration(self.current_position);
                let total_str = if let Some(duration) = self.song_duration {
                    format_duration(duration)
                } else {
                    "--:--".to_string()
                };
                ui.label(format!("{} / {}", current_str, total_str));
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
                        self.resume();
                    }
                }

                if ui.button("⏹").clicked() {
                    self.stop();
                }

                if ui.button("⏭").clicked() {
                    self.play_next_song();
                }
            });
        });

        // Request repaint for smooth time updates
        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

impl MusicPlayer {
    fn play_song_at_index(&mut self, index: usize) {
        if index >= self.songs.len() {
            return;
        }

        self.current_song_index = index;
        let song_path = &self.songs[index];
        self.current_song_name = song_path.file_name().unwrap().to_string_lossy().to_string();

        // Stop current playback
        self.sink.stop();

        // Create new sink
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        self.sink = Sink::try_new(&stream_handle).unwrap();
        self._stream = _stream;
        self._stream_handle = stream_handle;

        if let Ok(file) = std::fs::File::open(song_path) {
            if let Ok(source) = Decoder::new(BufReader::new(file)) {
                // Get duration before consuming the source
                let duration = get_source_duration(song_path);
                self.song_duration = duration;

                self.sink.append(source);
                self.sink.play();
                self.is_playing = true;
                self.song_start_time = Some(Instant::now());
                self.current_position = Duration::from_secs(0);
            }
        }
    }

    fn play_next_song(&mut self) {
        if !self.songs.is_empty() {
            let next_index = (self.current_song_index + 1) % self.songs.len();
            self.play_song_at_index(next_index);
        }
    }

    fn play_previous_song(&mut self) {
        if !self.songs.is_empty() {
            let prev_index = if self.current_song_index == 0 {
                self.songs.len() - 1
            } else {
                self.current_song_index - 1
            };
            self.play_song_at_index(prev_index);
        }
    }

    fn pause(&mut self) {
        self.sink.pause();
        self.is_playing = false;
    }

    fn resume(&mut self) {
        self.sink.play();
        self.is_playing = true;
        // Adjust start time to account for pause duration
        if self.song_start_time.is_some() {
            self.song_start_time = Some(Instant::now() - self.current_position);
        }
    }

    fn stop(&mut self) {
        self.sink.stop();
        self.is_playing = false;
        self.song_start_time = None;
        self.current_position = Duration::from_secs(0);
        self.current_song_name.clear();
    }
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{}:{:02}", minutes, seconds)
}

fn get_source_duration(path: &PathBuf) -> Option<Duration> {
    if let Ok(file) = std::fs::File::open(path) {
        if let Ok(source) = Decoder::new(BufReader::new(file)) {
            source.total_duration()
        } else {
            None
        }
    } else {
        None
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([450.0, 650.0])
            .with_title("Rustify Music Player"),
        ..Default::default()
    };

    eframe::run_native(
        "Rustify",
        options,
        Box::new(|_cc| Box::new(MusicPlayer::default())),
    )
}
