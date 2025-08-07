use crate::models::song::Song;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Folder {
    pub name: String,
    pub path: PathBuf,
    pub songs: Vec<Song>,
    pub expanded: bool,
}

impl Folder {
    pub fn new(path: PathBuf, name: String) -> Self {
        let songs = Self::load_songs_from_dir(&path);

        Self {
            name,
            path,
            songs,
            expanded: false,
        }
    }

    pub fn load_songs_from_dir(dir: &PathBuf) -> Vec<Song> {
        let mut songs = Vec::new();

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && Song::is_supported_format(&path) {
                    songs.push(Song::new(path));
                }
            }
        }

        songs.sort_by(|a, b| a.name.cmp(&b.name));
        songs
    }

    pub fn add_song(&mut self, song: Song) {
        self.songs.push(song);
        self.songs.sort_by(|a, b| a.name.cmp(&b.name));
    }

    pub fn remove_song(&mut self, index: usize) -> Option<Song> {
        if index < self.songs.len() {
            Some(self.songs.remove(index))
        } else {
            None
        }
    }

    pub fn toggle_expanded(&mut self) {
        self.expanded = !self.expanded;
    }

    pub fn is_empty(&self) -> bool {
        self.songs.is_empty()
    }

    pub fn song_count(&self) -> usize {
        self.songs.len()
    }
}
