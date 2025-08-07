use crate::models::{Folder, Song};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub struct MusicPlayer {
    pub folders: Vec<Folder>,
    pub current_song: Option<Song>,
    pub current_folder_index: Option<usize>,
    pub current_song_index: Option<usize>,
    pub sink: Sink,
    pub _stream: OutputStream,
    pub _stream_handle: OutputStreamHandle,
    pub is_playing: bool,
    pub song_start_time: Option<Instant>,
    pub current_position: Duration,
    pub dragged_song: Option<(usize, usize)>, // (folder_index, song_index)
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
    pub fn load_folders(music_dir: &PathBuf) -> Vec<Folder> {
        let mut folders = Vec::new();

        if !music_dir.exists() {
            return folders;
        }

        // First, add songs in the root music directory
        let root_songs = Folder::load_songs_from_dir(music_dir);
        if !root_songs.is_empty() {
            let mut root_folder = Folder::new(music_dir.clone(), "Music".to_string());
            root_folder.expanded = true;
            folders.push(root_folder);
        }

        // Then, add subdirectories as folders (playlists)
        if let Ok(entries) = fs::read_dir(music_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let folder_name = path.file_name().unwrap().to_string_lossy().to_string();
                    let folder = Folder::new(path, folder_name);
                    if !folder.is_empty() {
                        folders.push(folder);
                    }
                }
            }
        }

        folders
    }

    pub fn refresh_folders(&mut self) {
        let music_dir = PathBuf::from("/home")
            .join(std::env::var("USER").unwrap_or_default())
            .join("Downloads/music");
        self.folders = Self::load_folders(&music_dir);
    }

    pub fn move_song_to_folder(&mut self, from_folder: usize, song_index: usize, to_folder: usize) {
        if from_folder >= self.folders.len()
            || to_folder >= self.folders.len()
            || from_folder == to_folder
        {
            return;
        }

        let song = {
            if song_index >= self.folders[from_folder].songs.len() {
                return;
            }
            self.folders[from_folder].remove_song(song_index).unwrap()
        };

        let new_path = self.folders[to_folder].path.join(&song.name);

        // Move file in filesystem
        if fs::rename(&song.path, &new_path).is_err() {
            // If move failed, put the song back
            self.folders[from_folder].songs.insert(song_index, song);
            return;
        }

        // Update song path and add to new folder
        let moved_song = Song::new(new_path);
        self.folders[to_folder].add_song(moved_song);

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

    pub fn play_song(&mut self, folder_index: usize, song_index: usize) {
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

    // Fixed playlist functionality: only play next song within the same folder
    pub fn play_next_song(&mut self) {
        if let (Some(folder_idx), Some(song_idx)) =
            (self.current_folder_index, self.current_song_index)
        {
            if folder_idx < self.folders.len() {
                let folder = &self.folders[folder_idx];
                if song_idx + 1 < folder.songs.len() {
                    // Play next song in the same folder/playlist
                    self.play_song(folder_idx, song_idx + 1);
                } else {
                    // Reached end of playlist - stop playing
                    self.stop();
                }
            }
        }
    }

    pub fn play_previous_song(&mut self) {
        if let (Some(folder_idx), Some(song_idx)) =
            (self.current_folder_index, self.current_song_index)
        {
            if folder_idx < self.folders.len() && song_idx > 0 {
                // Play previous song in the same folder/playlist
                self.play_song(folder_idx, song_idx - 1);
            } else {
                // At the beginning of playlist - restart current song or stop
                if let Some(current_folder) = self.current_folder_index {
                    if let Some(current_song) = self.current_song_index {
                        self.play_song(current_folder, current_song);
                    }
                }
            }
        }
    }

    pub fn pause(&mut self) {
        self.sink.pause();
        self.is_playing = false;
    }

    pub fn resume(&mut self) {
        self.sink.play();
        self.is_playing = true;
        if self.song_start_time.is_some() {
            self.song_start_time = Some(Instant::now() - self.current_position);
        }
    }

    pub fn stop(&mut self) {
        self.sink.stop();
        self.is_playing = false;
        self.song_start_time = None;
        self.current_position = Duration::from_secs(0);
        self.current_song = None;
        self.current_folder_index = None;
        self.current_song_index = None;
    }

    pub fn update_position(&mut self) {
        if self.is_playing && self.song_start_time.is_some() {
            let elapsed = self.song_start_time.unwrap().elapsed();
            self.current_position = elapsed;

            // Check if song finished and auto-play next within the same playlist
            if self.sink.empty() && self.current_song.is_some() {
                self.play_next_song();
            }
        }
    }

    pub fn toggle_folder_expanded(&mut self, folder_index: usize) {
        if folder_index < self.folders.len() {
            self.folders[folder_index].toggle_expanded();
        }
    }

    pub fn set_dragged_song(&mut self, folder_index: usize, song_index: usize) {
        self.dragged_song = Some((folder_index, song_index));
    }

    pub fn clear_dragged_song(&mut self) {
        self.dragged_song = None;
    }

    pub fn is_current_song(&self, folder_index: usize, song_index: usize) -> bool {
        self.current_folder_index == Some(folder_index)
            && self.current_song_index == Some(song_index)
            && self.is_playing
    }
}
