# Rustify 🎵

A modern, lightweight music player built with Rust and egui. Rustify provides a clean, intuitive interface for playing your favorite audio files with support for multiple formats.

![Rustify Screenshot](https://via.placeholder.com/800x500/2d2d2d/ffffff?text=Rustify+Music+Player)

## Features

- **Modern GUI**: Clean, dark-themed interface built with egui
- **Multiple Audio Formats**: Support for MP3, WAV, OGG, FLAC, M4A, AAC, WMA, and MP4
- **Music Library Management**: Automatic scanning and organization of your music collection
- **Search Functionality**: Quickly find tracks by title, artist, or album
- **Playback Controls**: Play, pause, stop, previous/next track navigation
- **Volume Control**: Smooth volume adjustment with visual feedback
- **Progress Tracking**: Visual progress bar with seek functionality
- **Cross-Platform**: Works on Windows, macOS, and Linux

## Installation

### Prerequisites

- Rust 1.70 or later
- A system audio output device

### Building from Source

1. Clone the repository:
```bash
git clone <repository-url>
cd Rustify
```

2. Build and run:
```bash
cargo run --release
```

## Usage

### Getting Started

1. **Launch Rustify**: Run the application using `cargo run --release`
2. **Add Music**: Use `File > Add Music Folder` to select your music directory
3. **Browse**: Your tracks will appear in the main panel after scanning
4. **Play**: Double-click any track to start playing, or select and use the play button

### Controls

- **Play/Pause**: Space bar or click the play/pause button
- **Previous/Next**: Arrow keys or navigation buttons
- **Volume**: Use the volume slider in the bottom panel
- **Search**: Type in the search box to filter tracks
- **Seek**: Click anywhere on the progress bar to jump to that position

### Supported Audio Formats

- MP3 (MPEG Audio Layer III)
- WAV (Waveform Audio File Format)
- OGG (Ogg Vorbis)
- FLAC (Free Lossless Audio Codec)
- M4A (MPEG-4 Audio)
- AAC (Advanced Audio Coding)
- WMA (Windows Media Audio)
- MP4 (MPEG-4 Video with audio)

## Configuration

Rustify automatically detects your system's default music directory. You can add additional directories through the File menu or Settings dialog.

### Settings

Access settings via `File > Settings` or `View > Settings`:

- **Audio Settings**: Volume control and output device selection
- **Library Settings**: Manage music directories and scan options
- **UI Settings**: Theme and display preferences

## Architecture

Rustify is built with:

- **egui**: Immediate mode GUI framework for the user interface
- **eframe**: Application framework built on top of egui
- **rodio**: Audio playback engine supporting multiple formats
- **walkdir**: Recursive directory traversal for library scanning
- **rfd**: Native file dialogs for folder selection

## Project Structure

```
Rustify/
├── src/
│   ├── main.rs          # Application entry point and main UI
│   ├── audio.rs         # Audio playback engine
│   ├── library.rs       # Music library management
│   ├── settings.rs      # Application settings
│   └── ui.rs           # UI components and helpers
├── assets/
│   └── icon-256.png    # Application icon
├── Cargo.toml          # Dependencies and project metadata
└── README.md           # This file
```

## Development

### Building for Development

```bash
# Debug build with faster compilation
cargo run

# Release build with optimizations
cargo run --release

# Run tests
cargo test
```

### Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and add tests
4. Run `cargo test` and `cargo clippy`
5. Commit your changes: `git commit -m 'Add feature'`
6. Push to the branch: `git push origin feature-name`
7. Submit a pull request

## Known Issues

- Seeking functionality is limited (rodio limitation)
- Some metadata may not be extracted from all file formats
- Large libraries may take time to scan initially

## Future Enhancements

- [ ] Album artwork display
- [ ] Playlist creation and management
- [ ] Equalizer and audio effects
- [ ] Last.fm scrobbling
- [ ] Keyboard shortcuts customization
- [ ] Advanced metadata editing
- [ ] Internet radio streaming
- [ ] Plugin system

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [egui](https://github.com/emilk/egui) - Immediate mode GUI framework
- [rodio](https://github.com/RustAudio/rodio) - Audio playback library
- [Rust](https://www.rust-lang.org/) - Systems programming language

## Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/your-username/rustify/issues) page
2. Create a new issue with detailed information
3. Include your system information and error messages

---

Made with ❤️ and 🦀 Rust