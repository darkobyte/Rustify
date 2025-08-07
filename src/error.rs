use std::fmt;

/// Main error type for the Rustify application
#[derive(Debug)]
pub enum RustifyError {
    /// Audio playback related errors
    Audio(AudioError),
    /// Library management errors
    Library(LibraryError),
    /// Settings and configuration errors
    Settings(SettingsError),
    /// UI related errors
    Ui(UiError),
    /// IO errors
    Io(std::io::Error),
    /// Generic error with message
    Generic(String),
}

#[derive(Debug)]
pub enum AudioError {
    /// Failed to initialize audio device
    DeviceInitialization(String),
    /// Failed to load audio file
    FileLoad(String),
    /// Unsupported audio format
    UnsupportedFormat(String),
    /// Playback error
    Playback(String),
    /// Volume control error
    VolumeControl(String),
}

#[derive(Debug)]
pub enum LibraryError {
    /// Failed to scan directory
    ScanFailed(String),
    /// Failed to read metadata
    MetadataRead(String),
    /// Invalid path
    InvalidPath(String),
    /// Permission denied
    PermissionDenied(String),
}

#[derive(Debug)]
pub enum SettingsError {
    /// Failed to load settings
    LoadFailed(String),
    /// Failed to save settings
    SaveFailed(String),
    /// Invalid configuration value
    InvalidConfig(String),
    /// Serialization error
    Serialization(String),
}

#[derive(Debug)]
pub enum UiError {
    /// Window creation failed
    WindowCreation(String),
    /// Rendering error
    Rendering(String),
    /// Font loading error
    FontLoading(String),
}

impl fmt::Display for RustifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RustifyError::Audio(err) => write!(f, "Audio error: {}", err),
            RustifyError::Library(err) => write!(f, "Library error: {}", err),
            RustifyError::Settings(err) => write!(f, "Settings error: {}", err),
            RustifyError::Ui(err) => write!(f, "UI error: {}", err),
            RustifyError::Io(err) => write!(f, "IO error: {}", err),
            RustifyError::Generic(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AudioError::DeviceInitialization(msg) => {
                write!(f, "Device initialization failed: {}", msg)
            }
            AudioError::FileLoad(msg) => write!(f, "Failed to load audio file: {}", msg),
            AudioError::UnsupportedFormat(msg) => write!(f, "Unsupported audio format: {}", msg),
            AudioError::Playback(msg) => write!(f, "Playback error: {}", msg),
            AudioError::VolumeControl(msg) => write!(f, "Volume control error: {}", msg),
        }
    }
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LibraryError::ScanFailed(msg) => write!(f, "Library scan failed: {}", msg),
            LibraryError::MetadataRead(msg) => write!(f, "Failed to read metadata: {}", msg),
            LibraryError::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),
            LibraryError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
        }
    }
}

impl fmt::Display for SettingsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SettingsError::LoadFailed(msg) => write!(f, "Failed to load settings: {}", msg),
            SettingsError::SaveFailed(msg) => write!(f, "Failed to save settings: {}", msg),
            SettingsError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            SettingsError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl fmt::Display for UiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UiError::WindowCreation(msg) => write!(f, "Window creation failed: {}", msg),
            UiError::Rendering(msg) => write!(f, "Rendering error: {}", msg),
            UiError::FontLoading(msg) => write!(f, "Font loading error: {}", msg),
        }
    }
}

impl std::error::Error for RustifyError {}
impl std::error::Error for AudioError {}
impl std::error::Error for LibraryError {}
impl std::error::Error for SettingsError {}
impl std::error::Error for UiError {}

// Conversions from standard library errors
impl From<std::io::Error> for RustifyError {
    fn from(err: std::io::Error) -> Self {
        RustifyError::Io(err)
    }
}

impl From<serde_json::Error> for RustifyError {
    fn from(err: serde_json::Error) -> Self {
        RustifyError::Settings(SettingsError::Serialization(err.to_string()))
    }
}

impl From<walkdir::Error> for RustifyError {
    fn from(err: walkdir::Error) -> Self {
        RustifyError::Library(LibraryError::ScanFailed(err.to_string()))
    }
}

impl From<rodio::decoder::DecoderError> for RustifyError {
    fn from(err: rodio::decoder::DecoderError) -> Self {
        RustifyError::Audio(AudioError::FileLoad(err.to_string()))
    }
}

impl From<rodio::PlayError> for RustifyError {
    fn from(err: rodio::PlayError) -> Self {
        RustifyError::Audio(AudioError::Playback(err.to_string()))
    }
}

impl From<rodio::StreamError> for RustifyError {
    fn from(err: rodio::StreamError) -> Self {
        RustifyError::Audio(AudioError::DeviceInitialization(err.to_string()))
    }
}

// Helper type for Results
pub type Result<T> = std::result::Result<T, RustifyError>;

// Utility functions for error handling
impl RustifyError {
    /// Create a generic error with a message
    pub fn generic(msg: impl Into<String>) -> Self {
        RustifyError::Generic(msg.into())
    }

    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            RustifyError::Audio(AudioError::VolumeControl(_)) => true,
            RustifyError::Library(LibraryError::MetadataRead(_)) => true,
            RustifyError::Settings(SettingsError::LoadFailed(_)) => true,
            RustifyError::Io(err) => match err.kind() {
                std::io::ErrorKind::NotFound => true,
                std::io::ErrorKind::PermissionDenied => false,
                _ => true,
            },
            _ => false,
        }
    }

    /// Get user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            RustifyError::Audio(AudioError::DeviceInitialization(_)) => {
                "Could not initialize audio device. Check your audio settings.".to_string()
            }
            RustifyError::Audio(AudioError::UnsupportedFormat(_)) => {
                "This audio format is not supported.".to_string()
            }
            RustifyError::Library(LibraryError::PermissionDenied(_)) => {
                "Permission denied accessing music files.".to_string()
            }
            RustifyError::Settings(SettingsError::SaveFailed(_)) => {
                "Could not save settings. Check file permissions.".to_string()
            }
            _ => self.to_string(),
        }
    }

    /// Log the error with appropriate level
    pub fn log(&self) {
        match self {
            RustifyError::Audio(AudioError::VolumeControl(_)) => {
                log::warn!("{}", self);
            }
            RustifyError::Library(LibraryError::MetadataRead(_)) => {
                log::warn!("{}", self);
            }
            RustifyError::Settings(_) => {
                log::error!("{}", self);
            }
            _ => {
                log::error!("{}", self);
            }
        }
    }
}

// Macro for easy error creation
#[macro_export]
macro_rules! rustify_error {
    ($msg:expr) => {
        $crate::error::RustifyError::generic($msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::RustifyError::generic(format!($fmt, $($arg)*))
    };
}

// Macro for easy audio error creation
#[macro_export]
macro_rules! audio_error {
    (device, $msg:expr) => {
        $crate::error::RustifyError::Audio($crate::error::AudioError::DeviceInitialization(
            $msg.into(),
        ))
    };
    (file, $msg:expr) => {
        $crate::error::RustifyError::Audio($crate::error::AudioError::FileLoad($msg.into()))
    };
    (format, $msg:expr) => {
        $crate::error::RustifyError::Audio($crate::error::AudioError::UnsupportedFormat(
            $msg.into(),
        ))
    };
    (playback, $msg:expr) => {
        $crate::error::RustifyError::Audio($crate::error::AudioError::Playback($msg.into()))
    };
    (volume, $msg:expr) => {
        $crate::error::RustifyError::Audio($crate::error::AudioError::VolumeControl($msg.into()))
    };
}

// Macro for easy library error creation
#[macro_export]
macro_rules! library_error {
    (scan, $msg:expr) => {
        $crate::error::RustifyError::Library($crate::error::LibraryError::ScanFailed($msg.into()))
    };
    (metadata, $msg:expr) => {
        $crate::error::RustifyError::Library($crate::error::LibraryError::MetadataRead($msg.into()))
    };
    (path, $msg:expr) => {
        $crate::error::RustifyError::Library($crate::error::LibraryError::InvalidPath($msg.into()))
    };
    (permission, $msg:expr) => {
        $crate::error::RustifyError::Library($crate::error::LibraryError::PermissionDenied(
            $msg.into(),
        ))
    };
}
