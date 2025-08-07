use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub audio: AudioSettings,
    pub library: LibrarySettings,
    pub ui: UiSettings,
    pub playback: PlaybackSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    pub volume: f32,
    pub output_device: Option<String>,
    pub sample_rate: u32,
    pub buffer_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibrarySettings {
    pub music_directories: Vec<PathBuf>,
    pub auto_scan: bool,
    pub scan_interval_hours: u32,
    pub watch_folders: bool,
    pub supported_formats: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    pub theme: Theme,
    pub show_album_art: bool,
    pub window_size: (f32, f32),
    pub window_position: Option<(f32, f32)>,
    pub sidebar_width: f32,
    pub font_size: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackSettings {
    pub shuffle: bool,
    pub repeat: RepeatMode,
    pub crossfade_duration: f32,
    pub gapless_playback: bool,
    pub replay_gain: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepeatMode {
    None,
    Track,
    Playlist,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            audio: AudioSettings::default(),
            library: LibrarySettings::default(),
            ui: UiSettings::default(),
            playback: PlaybackSettings::default(),
        }
    }
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            volume: 0.7,
            output_device: None,
            sample_rate: 44100,
            buffer_size: 1024,
        }
    }
}

impl Default for LibrarySettings {
    fn default() -> Self {
        let default_music_dir = dirs::audio_dir()
            .or_else(|| dirs::home_dir().map(|p| p.join("Music")))
            .unwrap_or_else(|| PathBuf::from("."));

        Self {
            music_directories: vec![default_music_dir],
            auto_scan: true,
            scan_interval_hours: 24,
            watch_folders: true,
            supported_formats: vec![
                "mp3".to_string(),
                "wav".to_string(),
                "ogg".to_string(),
                "flac".to_string(),
                "m4a".to_string(),
                "aac".to_string(),
                "wma".to_string(),
                "mp4".to_string(),
            ],
        }
    }
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            show_album_art: true,
            window_size: (1200.0, 800.0),
            window_position: None,
            sidebar_width: 250.0,
            font_size: 14.0,
        }
    }
}

impl Default for PlaybackSettings {
    fn default() -> Self {
        Self {
            shuffle: false,
            repeat: RepeatMode::None,
            crossfade_duration: 0.0,
            gapless_playback: false,
            replay_gain: false,
        }
    }
}

impl Settings {
    pub fn load_from_storage(
        storage: &dyn eframe::Storage,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(value) = eframe::get_value(storage, eframe::APP_KEY) {
            Ok(value)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config_dir) = dirs::config_dir() {
            let rustify_dir = config_dir.join("rustify");
            std::fs::create_dir_all(&rustify_dir)?;

            let config_path = rustify_dir.join("settings.json");
            let content = serde_json::to_string_pretty(self)?;
            std::fs::write(config_path, content)?;
        }

        Ok(())
    }

    pub fn save_to_storage(
        &self,
        storage: &mut dyn eframe::Storage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        eframe::set_value(storage, eframe::APP_KEY, self);
        Ok(())
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Light => write!(f, "Light"),
            Theme::Dark => write!(f, "Dark"),
            Theme::Auto => write!(f, "Auto"),
        }
    }
}

impl std::fmt::Display for RepeatMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepeatMode::None => write!(f, "None"),
            RepeatMode::Track => write!(f, "Track"),
            RepeatMode::Playlist => write!(f, "Playlist"),
        }
    }
}
