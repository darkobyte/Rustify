use rodio::{Decoder, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Song {
    pub path: PathBuf,
    pub name: String,
    pub duration: Option<Duration>,
}

impl Song {
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let duration = Self::get_duration(&path);

        Self {
            path,
            name,
            duration,
        }
    }

    fn get_duration(path: &PathBuf) -> Option<Duration> {
        match File::open(path) {
            Ok(file) => {
                match Decoder::new(BufReader::new(file)) {
                    Ok(source) => {
                        let duration = source.total_duration();
                        if duration.is_some() {
                            duration
                        } else {
                            // Try alternative method for formats that don't provide duration
                            Self::estimate_duration_from_file(path)
                        }
                    }
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    fn estimate_duration_from_file(path: &PathBuf) -> Option<Duration> {
        // For MP3 files, we can try to estimate based on file size and bitrate
        // This is a rough estimation and won't be accurate for all files
        if let Some(ext) = path.extension() {
            if ext.to_string_lossy().to_lowercase() == "mp3" {
                if let Ok(metadata) = std::fs::metadata(path) {
                    let file_size = metadata.len();
                    // Assume average bitrate of 128 kbps for estimation
                    let estimated_seconds = (file_size * 8) / (128 * 1000);
                    return Some(Duration::from_secs(estimated_seconds));
                }
            }
        }
        None
    }

    pub fn is_supported_format(path: &PathBuf) -> bool {
        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                return matches!(
                    ext_str.to_lowercase().as_str(),
                    "mp3" | "wav" | "flac" | "ogg"
                );
            }
        }
        false
    }
}
