use eframe::egui;

mod audio;
mod error;
mod library;
mod settings;
mod ui;

use audio::AudioPlayer;
use library::MusicLibrary;
use settings::Settings;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Rustify - Modern Music Player"),
        ..Default::default()
    };

    eframe::run_native(
        "Rustify - Music Player",
        options,
        Box::new(|cc| {
            // Apply custom theme
            setup_custom_fonts(cc);
            apply_custom_theme(&cc.egui_ctx);
            Box::new(RustifyApp::new(cc))
        }),
    )
}

fn setup_custom_fonts(_cc: &eframe::CreationContext<'_>) {
    // Custom font loading disabled for now
    // Would require actual font files in assets/fonts/
}

fn apply_custom_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();

    // Background colors
    visuals.window_fill = egui::Color32::from_rgb(20, 20, 25);
    visuals.panel_fill = egui::Color32::from_rgb(25, 25, 30);
    visuals.faint_bg_color = egui::Color32::from_rgb(35, 35, 40);
    visuals.extreme_bg_color = egui::Color32::from_rgb(15, 15, 20);

    // Interactive elements
    visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(40, 40, 45);
    visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(45, 45, 50);
    visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(55, 55, 65);
    visuals.widgets.active.bg_fill = egui::Color32::from_rgb(65, 65, 75);

    // Accent colors
    visuals.selection.bg_fill = egui::Color32::from_rgb(80, 120, 255);
    visuals.hyperlink_color = egui::Color32::from_rgb(100, 150, 255);

    // Text colors are handled automatically by egui theme

    ctx.set_visuals(visuals);
}

struct RustifyApp {
    audio_player: AudioPlayer,
    music_library: MusicLibrary,
    settings: Settings,

    // UI State
    selected_track: Option<usize>,
    show_settings: bool,
    search_query: String,
    volume: f32,

    // Library state
    library_path: String,
    scan_status: ScanStatus,

    // Playback state
    is_playing: bool,
    current_time: f64,
    total_time: f64,
    is_shuffled: bool,
    repeat_mode: RepeatMode,
}

#[derive(Debug, Clone, PartialEq)]
enum ScanStatus {
    Idle,
    Scanning,
    Complete(usize), // number of tracks found
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
enum RepeatMode {
    None,
    Track,
    All,
}

impl RustifyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let settings = if let Some(storage) = cc.storage {
            Settings::load_from_storage(storage).unwrap_or_default()
        } else {
            Settings::default()
        };

        let library_path = settings
            .library
            .music_directories
            .first()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| {
                dirs::audio_dir()
                    .unwrap_or_else(|| dirs::home_dir().unwrap_or_default())
                    .to_string_lossy()
                    .to_string()
            });

        Self {
            audio_player: AudioPlayer::default(),
            music_library: MusicLibrary::default(),
            volume: settings.audio.volume,
            library_path,
            scan_status: ScanStatus::Idle,
            selected_track: None,
            show_settings: false,
            search_query: String::new(),
            is_playing: false,
            current_time: 0.0,
            total_time: 0.0,
            is_shuffled: false,
            repeat_mode: RepeatMode::None,
            settings,
        }
    }

    fn update_playback_state(&mut self) {
        self.is_playing = self.audio_player.is_playing();
        self.current_time = self.audio_player.current_time();
        self.total_time = self.audio_player.total_time();

        // Auto-advance to next track if current track finished
        if !self.is_playing && self.total_time > 0.0 && self.current_time >= self.total_time {
            self.play_next_track();
        }
    }

    fn play_selected_track(&mut self) {
        if let Some(track_idx) = self.selected_track {
            let tracks = self.music_library.get_filtered_tracks(&self.search_query);
            if let Some(track) = tracks.get(track_idx) {
                if let Err(e) = self.audio_player.play_file(&track.path) {
                    log::error!("Error playing file: {}", e);
                    self.scan_status = ScanStatus::Error(format!("Playback error: {}", e));
                }
            }
        }
    }

    fn play_next_track(&mut self) {
        if let Some(current) = self.selected_track {
            let tracks = self.music_library.get_filtered_tracks(&self.search_query);

            let next_idx = match self.repeat_mode {
                RepeatMode::Track => current, // Repeat current track
                RepeatMode::All => {
                    if current + 1 >= tracks.len() {
                        0 // Loop back to first track
                    } else {
                        current + 1
                    }
                }
                RepeatMode::None => {
                    if current + 1 < tracks.len() {
                        current + 1
                    } else {
                        return; // Stop at end
                    }
                }
            };

            self.selected_track = Some(next_idx);
            self.play_selected_track();
        }
    }

    fn play_previous_track(&mut self) {
        if let Some(current) = self.selected_track {
            let tracks = self.music_library.get_filtered_tracks(&self.search_query);

            let prev_idx = if current == 0 {
                if self.repeat_mode == RepeatMode::All {
                    tracks.len().saturating_sub(1)
                } else {
                    0
                }
            } else {
                current - 1
            };

            self.selected_track = Some(prev_idx);
            self.play_selected_track();
        }
    }

    fn format_time(seconds: f64) -> String {
        let total_seconds = seconds as u64;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }
}

impl eframe::App for RustifyApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.settings.audio.volume = self.volume;
        if let Err(e) = self.settings.save_to_storage(storage) {
            log::error!("Failed to save settings: {}", e);
        }
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_playback_state();

        // Request repaint for smooth UI updates
        ctx.request_repaint_after(std::time::Duration::from_millis(100));

        self.render_menu_bar(ctx);
        self.render_control_panel(ctx);
        self.render_main_content(ctx);

        if self.show_settings {
            self.render_settings_window(ctx);
        }
    }
}

impl RustifyApp {
    fn render_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("📁 Add Music Folder").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.library_path = path.to_string_lossy().to_string();
                            self.music_library.scan_directory(&self.library_path);
                            self.scan_status = ScanStatus::Scanning;
                        }
                    }
                    ui.separator();
                    if ui.button("⚙️ Settings").clicked() {
                        self.show_settings = true;
                    }
                    ui.separator();
                    if ui.button("❌ Exit").clicked() {
                        std::process::exit(0);
                    }
                });

                ui.menu_button("Playback", |ui| {
                    if ui.button("⏯️ Play/Pause").clicked() {
                        self.audio_player.toggle_playback();
                    }
                    if ui.button("⏹️ Stop").clicked() {
                        self.audio_player.stop();
                    }
                    if ui.button("⏮️ Previous").clicked() {
                        self.play_previous_track();
                    }
                    if ui.button("⏭️ Next").clicked() {
                        self.play_next_track();
                    }
                    ui.separator();
                    if ui.checkbox(&mut self.is_shuffled, "🔀 Shuffle").changed() {
                        // TODO: Implement shuffle logic
                    }

                    ui.horizontal(|ui| {
                        ui.label("🔁 Repeat:");
                        egui::ComboBox::from_id_source("repeat_mode")
                            .selected_text(format!("{:?}", self.repeat_mode))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.repeat_mode,
                                    RepeatMode::None,
                                    "None",
                                );
                                ui.selectable_value(
                                    &mut self.repeat_mode,
                                    RepeatMode::Track,
                                    "Track",
                                );
                                ui.selectable_value(&mut self.repeat_mode, RepeatMode::All, "All");
                            });
                    });
                });

                ui.menu_button("View", |ui| {
                    ui.checkbox(&mut self.show_settings, "Settings");
                });
            });
        });
    }

    fn render_control_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("control_panel").show(ctx, |ui| {
            ui.vertical(|ui| {
                // Progress bar
                let progress = if self.total_time > 0.0 {
                    (self.current_time / self.total_time) as f32
                } else {
                    0.0
                };

                let response = ui.add(
                    egui::ProgressBar::new(progress)
                        .show_percentage()
                        .animate(self.is_playing),
                );

                if response.clicked() {
                    if let Some(click_pos) = response.interact_pointer_pos() {
                        let bar_rect = response.rect;
                        let relative_pos = (click_pos.x - bar_rect.left()) / bar_rect.width();
                        let seek_time = relative_pos as f64 * self.total_time;
                        self.audio_player.seek(seek_time);
                    }
                }

                // Time display
                ui.horizontal(|ui| {
                    ui.label(Self::format_time(self.current_time));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(Self::format_time(self.total_time));
                    });
                });

                // Control buttons
                ui.horizontal(|ui| {
                    // Previous
                    if ui.button("⏮️").clicked() {
                        self.play_previous_track();
                    }

                    // Play/Pause
                    let play_button_text = if self.is_playing { "⏸️" } else { "▶️" };
                    if ui.button(play_button_text).clicked() {
                        self.audio_player.toggle_playback();
                    }

                    // Stop
                    if ui.button("⏹️").clicked() {
                        self.audio_player.stop();
                    }

                    // Next
                    if ui.button("⏭️").clicked() {
                        self.play_next_track();
                    }

                    ui.separator();

                    // Volume control
                    ui.label("🔊");
                    if ui
                        .add(egui::Slider::new(&mut self.volume, 0.0..=1.0).show_value(false))
                        .changed()
                    {
                        self.audio_player.set_volume(self.volume);
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Currently playing track info
                        if let Some(track_idx) = self.selected_track {
                            let tracks = self.music_library.get_filtered_tracks(&self.search_query);
                            if let Some(track) = tracks.get(track_idx) {
                                ui.label(format!("♪ {} - {}", track.artist, track.title));
                            }
                        }
                    });
                });
            });
        });
    }

    fn render_main_content(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Left sidebar
                ui.vertical(|ui| {
                    ui.set_width(250.0);
                    self.render_sidebar(ui);
                });

                ui.separator();

                // Main content area
                ui.vertical(|ui| {
                    self.render_track_list(ui);
                });
            });
        });
    }

    fn render_sidebar(&mut self, ui: &mut egui::Ui) {
        ui.heading("📚 Library");
        ui.separator();

        ui.label(format!("📁 Path: {}", self.library_path));
        ui.label(format!("🎵 Tracks: {}", self.music_library.track_count()));

        match &self.scan_status {
            ScanStatus::Idle => {
                if ui.button("🔍 Scan Library").clicked() {
                    self.music_library.scan_directory(&self.library_path);
                    self.scan_status = ScanStatus::Scanning;
                }
            }
            ScanStatus::Scanning => {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Scanning...");
                });
            }
            ScanStatus::Complete(count) => {
                ui.colored_label(egui::Color32::GREEN, format!("✅ Found {} tracks", count));
                if ui.button("🔍 Rescan").clicked() {
                    self.music_library.scan_directory(&self.library_path);
                    self.scan_status = ScanStatus::Scanning;
                }
            }
            ScanStatus::Error(err) => {
                ui.colored_label(egui::Color32::RED, format!("❌ Error: {}", err));
                if ui.button("🔄 Retry").clicked() {
                    self.music_library.scan_directory(&self.library_path);
                    self.scan_status = ScanStatus::Scanning;
                }
            }
        }

        // Update scan status
        if matches!(self.scan_status, ScanStatus::Scanning) && !self.music_library.is_scanning() {
            let count = self.music_library.track_count();
            self.scan_status = ScanStatus::Complete(count);
        }

        ui.separator();

        // Playback controls
        ui.heading("🎛️ Controls");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("🔀");
            ui.checkbox(&mut self.is_shuffled, "Shuffle");
        });

        ui.horizontal(|ui| {
            ui.label("🔁");
            egui::ComboBox::from_label("Repeat")
                .selected_text(format!("{:?}", self.repeat_mode))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.repeat_mode, RepeatMode::None, "None");
                    ui.selectable_value(&mut self.repeat_mode, RepeatMode::Track, "Track");
                    ui.selectable_value(&mut self.repeat_mode, RepeatMode::All, "All");
                });
        });
    }

    fn render_track_list(&mut self, ui: &mut egui::Ui) {
        // Search bar
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.add(
                egui::TextEdit::singleline(&mut self.search_query)
                    .hint_text("Search tracks, artists, albums...")
                    .desired_width(ui.available_width()),
            );
        });

        ui.separator();

        // Track list
        egui::ScrollArea::vertical().show(ui, |ui| {
            let tracks = self.music_library.get_filtered_tracks(&self.search_query);

            if tracks.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    ui.heading("🎵 No tracks found");
                    ui.label("Add a music folder to get started");
                });
                return;
            }

            for (idx, track) in tracks.iter().enumerate() {
                let is_selected = self.selected_track == Some(idx);
                let is_current = is_selected && self.is_playing;

                let response = ui
                    .horizontal(|ui| {
                        // Playing indicator
                        if is_current {
                            ui.colored_label(egui::Color32::GREEN, "♪");
                        } else {
                            ui.add_space(15.0);
                        }

                        // Track info
                        let text = format!("{} - {} ({})", track.artist, track.title, track.album);
                        let response = ui.selectable_label(is_selected, text);

                        // Duration (right-aligned)
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(Self::format_time(track.duration));
                        });

                        response
                    })
                    .inner;

                if response.clicked() {
                    self.selected_track = Some(idx);
                }

                if response.double_clicked() {
                    self.selected_track = Some(idx);
                    self.play_selected_track();
                }
            }
        });
    }

    fn render_settings_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("⚙️ Settings")
            .open(&mut self.show_settings)
            .default_width(400.0)
            .show(ctx, |ui| {
                ui.heading("🔊 Audio Settings");

                ui.horizontal(|ui| {
                    ui.label("Volume:");
                    if ui
                        .add(egui::Slider::new(&mut self.volume, 0.0..=1.0))
                        .changed()
                    {
                        self.audio_player.set_volume(self.volume);
                        self.settings.audio.volume = self.volume;
                    }
                });

                ui.separator();

                ui.heading("📁 Library Settings");

                ui.horizontal(|ui| {
                    ui.label("Music Directory:");
                    ui.text_edit_singleline(&mut self.library_path);
                    if ui.button("📁").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.library_path = path.to_string_lossy().to_string();
                        }
                    }
                });

                if ui.button("🔍 Rescan Library").clicked() {
                    self.music_library.scan_directory(&self.library_path);
                    self.scan_status = ScanStatus::Scanning;
                }

                ui.separator();

                if ui.button("💾 Save Settings").clicked() {
                    if let Err(e) = self.settings.save() {
                        log::error!("Failed to save settings: {}", e);
                    }
                }
            });
    }
}
