use crate::player::MusicPlayer;
use crate::ui::SpotifyTheme;
use eframe::egui;

pub struct SearchBar {
    search_text: String,
}

impl Default for SearchBar {
    fn default() -> Self {
        Self {
            search_text: String::new(),
        }
    }
}

impl SearchBar {
    /// Show search bar and return filtered results
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        player: &MusicPlayer,
    ) -> Vec<(usize, usize)> {  // Returns (folder_idx, song_idx) for matching songs
        let _theme = SpotifyTheme::default();
        
        ui.horizontal(|ui| {
            ui.label("🔍");
            
            let _search_response = ui.add(
                egui::TextEdit::singleline(&mut self.search_text)
                    .hint_text("Search songs, folders...")
                    .desired_width(ui.available_width() - 100.0)
            );
            
            if ui.button("Clear").clicked() {
                self.search_text.clear();
            }
        });
        
        // Filter songs based on search text
        if self.search_text.is_empty() {
            Vec::new() // Return empty if no search
        } else {
            self.filter_songs(player)
        }
    }
    
    /// Filter songs based on search criteria
    fn filter_songs(&self, player: &MusicPlayer) -> Vec<(usize, usize)> {
        let search_lower = self.search_text.to_lowercase();
        let mut results = Vec::new();
        
        for (folder_idx, folder) in player.folders.iter().enumerate() {
            // Search in folder name
            if folder.name.to_lowercase().contains(&search_lower) {
                // If folder matches, add all songs
                for (song_idx, _) in folder.songs.iter().enumerate() {
                    results.push((folder_idx, song_idx));
                }
                continue;
            }
            
            // Search in song names
            for (song_idx, song) in folder.songs.iter().enumerate() {
                if song.name.to_lowercase().contains(&search_lower) {
                    results.push((folder_idx, song_idx));
                }
            }
        }
        
        results
    }
    
    /// Get search text for highlighting
    pub fn get_search_text(&self) -> &str {
        &self.search_text
    }
    
    /// Check if search is active
    pub fn is_searching(&self) -> bool {
        !self.search_text.is_empty()
    }
}