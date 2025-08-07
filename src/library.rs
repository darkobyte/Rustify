use std::collections::HashMap;
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

    #[allow(dead_code)]
    pub fn get_tracks(&self) -> Vec<Track> {
        self.tracks.lock().unwrap().clone()
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

    #[allow(dead_code)]
    pub fn get_artists(&self) -> Vec<String> {
        let tracks = self.tracks.lock().unwrap();
        let mut artists: Vec<String> = tracks
            .iter()
            .map(|track| track.artist.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        artists.sort();
        artists
    }

    #[allow(dead_code)]
    pub fn get_albums(&self) -> Vec<String> {
        let tracks = self.tracks.lock().unwrap();
        let mut albums: Vec<String> = tracks
            .iter()
            .map(|track| track.album.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        albums.sort();
        albums
    }

    #[allow(dead_code)]
    pub fn get_tracks_by_artist(&self, artist: &str) -> Vec<Track> {
        self.tracks
            .lock()
            .unwrap()
            .iter()
            .filter(|track| track.artist == artist)
            .cloned()
            .collect()
    }

    #[allow(dead_code)]
    pub fn get_tracks_by_album(&self, album: &str) -> Vec<Track> {
        self.tracks
            .lock()
            .unwrap()
            .iter()
            .filter(|track| track.album == album)
            .cloned()
            .collect()
    }

    #[allow(dead_code)]
    pub fn get_library_stats(&self) -> LibraryStats {
        let tracks = self.tracks.lock().unwrap();
        let total_tracks = tracks.len();
        let total_size: u64 = tracks.iter().map(|t| t.file_size).sum();
        let total_duration: f64 = tracks.iter().map(|t| t.duration).sum();

        let mut format_counts: HashMap<String, usize> = HashMap::new();
        for track in tracks.iter() {
            *format_counts.entry(track.format.clone()).or_insert(0) += 1;
        }

        let artists: std::collections::HashSet<String> =
            tracks.iter().map(|track| track.artist.clone()).collect();
        let albums: std::collections::HashSet<String> =
            tracks.iter().map(|track| track.album.clone()).collect();

        LibraryStats {
            total_tracks,
            total_size,
            total_duration,
            format_counts,
            unique_artists: artists.len(),
            unique_albums: albums.len(),
        }
    }

    #[allow(dead_code)]
    pub fn remove_track(&mut self, path: &Path) -> bool {
        let mut tracks = self.tracks.lock().unwrap();
        let initial_len = tracks.len();
        tracks.retain(|track| track.path != path);
        tracks.len() != initial_len
    }

    #[allow(dead_code)]
    pub fn clear_library(&mut self) {
        self.tracks.lock().unwrap().clear();
        self.library_paths.clear();
    }
}

#[derive(Debug)]
pub struct LibraryStats {
    pub total_tracks: usize,
    pub total_size: u64,
    pub total_duration: f64,
    pub format_counts: HashMap<String, usize>,
    pub unique_artists: usize,
    pub unique_albums: usize,
}

impl LibraryStats {
    #[allow(dead_code)]
    pub fn format_size(&self) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = self.total_size as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        format!("{:.2} {}", size, UNITS[unit_index])
    }

    #[allow(dead_code)]
    pub fn format_duration(&self) -> String {
        let total_seconds = self.total_duration as u64;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        }
    }
}
