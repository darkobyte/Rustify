use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Song {
    pub path: PathBuf,
    pub name: String,
}

impl Song {
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        Self { path, name }
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
