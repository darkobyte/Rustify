use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use walkdir::WalkDir;

use crate::audio::is_audio_file;

#[derive(Debug, Clone, PartialEq)]
pub struct Track {
    pub path: PathBuf,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: f64,
    pub file_size: u64,
    pub format: String,
}

impl Track {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        let path = path.as_ref();

        if !is_audio_file(path) {
            return None;
        }

        let file_name = path.file_stem()?.to_string_lossy().to_string();
        let extension = path.extension()?.to_string_lossy().to_string();

        // Get file metadata
        let metadata = fs::metadata(path).ok()?;
        let file_size = metadata.len();

        // Parse basic info from filename
        // Format: "Artist - Title" or just "Title"
        let (artist, title) = if file_name.contains(" - ") {
            let parts: Vec<&str> = file_name.splitn(2, " - ").collect();
            (parts[0].to_string(), parts[1].to_string())
        } else {
            ("Unknown Artist".to_string(), file_name)
        };

        // Try to get album from parent directory name
        let album = path
            .parent()
            .and_then(|p| p.file_name())
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| "Unknown Album".to_string());

        Some(Track {
            path: path.to_path_buf(),
            title,
            artist,
            album,
            duration: 0.0, // Would need audio metadata parsing for actual duration
            file_size,
            format: extension.to_uppercase(),
        })
    }

    pub fn matches_search(&self, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }

        let query = query.to_lowercase();
        self.title.to_lowercase().contains(&query)
            || self.artist.to_lowercase().contains(&query)
            || self.album.to_lowercase().contains(&query)
    }
}

#[derive(Default)]
pub struct MusicLibrary {
    tracks: Arc<Mutex<Vec<Track>>>,
    is_scanning: Arc<Mutex<bool>>,
    library_paths: Vec<PathBuf>,
}

impl MusicLibrary {
    pub fn scan_directory<P: AsRef<Path>>(&mut self, path: P) {
        let path = path.as_ref().to_path_buf();

        if !self.library_paths.contains(&path) {
            self.library_paths.push(path.clone());
        }

        let tracks = Arc::clone(&self.tracks);
        let is_scanning = Arc::clone(&self.is_scanning);

        // Set scanning flag
        *is_scanning.lock().unwrap() = true;

        // Spawn background thread for scanning
        thread::spawn(move || {
            let mut new_tracks = Vec::new();

            for entry in WalkDir::new(&path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                if let Some(track) = Track::from_path(entry.path()) {
                    new_tracks.push(track);
                }
            }

            // Update tracks collection
            {
                let mut tracks_guard = tracks.lock().unwrap();

                // Remove tracks from this path first
                tracks_guard.retain(|track| !track.path.starts_with(&path));

                // Add new tracks
                tracks_guard.extend(new_tracks);

                // Sort by artist, then album, then title
                tracks_guard.sort_by(|a, b| {
                    a.artist
                        .cmp(&b.artist)
                        .then_with(|| a.album.cmp(&b.album))
                        .then_with(|| a.title.cmp(&b.title))
                });
            }

            // Clear scanning flag
            *is_scanning.lock().unwrap() = false;
        });
    }

    pub fn get_filtered_tracks(&self, search_query: &str) -> Vec<Track> {
        self.tracks
            .lock()
            .unwrap()
            .iter()
            .filter(|track| track.matches_search(search_query))
            .cloned()
            .collect()
    }

    pub fn track_count(&self) -> usize {
        self.tracks.lock().unwrap().len()
    }

    pub fn is_scanning(&self) -> bool {
        self.is_scanning.lock().map(|guard| *guard).unwrap_or(false)
    }
}
