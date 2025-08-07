# 🤝 Contributing to Rustify

Thank you for your interest in contributing to Rustify! We welcome contributions from everyone, regardless of experience level. This document provides guidelines for contributing to the project.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [How to Contribute](#how-to-contribute)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Documentation](#documentation)
- [Community](#community)

## 📜 Code of Conduct

This project adheres to a code of conduct that we expect all contributors to follow:

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on constructive feedback
- Respect different viewpoints and experiences
- Show empathy towards other community members

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or higher
- Git
- Basic familiarity with Rust programming
- Audio system dependencies for your platform

### Development Setup

1. **Fork the repository** on GitHub
2. **Clone your fork**:
   ```bash
   git clone https://github.com/darkobyte/Rustify.git
   cd Rustify
   ```

3. **Set up the upstream remote**:
   ```bash
   git remote add upstream https://github.com/darkobyte/Rustify.git
   ```

4. **Install dependencies and test the build**:
   ```bash
   cargo build
   cargo test
   cargo run
   ```

## 🛠️ How to Contribute

### Types of Contributions We Welcome

- 🐛 **Bug fixes**: Fix issues in the codebase
- ✨ **New features**: Add new functionality
- 📚 **Documentation**: Improve or add documentation
- 🎨 **UI/UX improvements**: Enhance the user interface
- 🧪 **Tests**: Add or improve test coverage
- 🔧 **Refactoring**: Improve code quality and structure
- 🚀 **Performance**: Optimize performance
- 🌐 **Platform support**: Add support for new platforms

## 🔄 Pull Request Process

### Before You Start

1. **Check existing issues** to avoid duplicate work
2. **Create an issue** if one doesn't exist for your contribution
3. **Comment on the issue** to let others know you're working on it

### Making Changes

1. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following our coding standards
3. **Write or update tests** for your changes
4. **Update documentation** if needed
5. **Test your changes**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

### Submitting Your Pull Request

1. **Commit your changes** with descriptive messages:
   ```bash
   git commit -m "Add: New playlist shuffle functionality"
   ```

2. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

3. **Create a Pull Request** on GitHub with:
   - Clear title and description
   - Reference to related issues
   - Screenshots for UI changes
   - List of changes made

### Pull Request Guidelines

- Keep PRs focused and small when possible
- Include tests for new functionality
- Update documentation for new features
- Ensure all CI checks pass
- Respond to feedback promptly and respectfully

## 🎯 Coding Standards

### Rust Code Style

- Follow the official [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
- Use `cargo fmt` to format your code
- Run `cargo clippy` and fix any warnings
- Write clear, self-documenting code
- Add comments for complex logic

### Code Organization

- Keep functions small and focused
- Use meaningful variable and function names
- Organize code into logical modules
- Follow existing project structure

### Example Code Style

```rust
// Good: Clear, documented function
/// Plays the next song in the current playlist
pub fn play_next_song(&mut self) -> Result<(), PlayerError> {
    if let Some(next_index) = self.get_next_song_index() {
        self.play_song_at_index(next_index)
    } else {
        self.stop_playback()
    }
}

// Good: Clear error handling
match self.load_audio_file(&song.path) {
    Ok(audio_source) => self.play_audio(audio_source),
    Err(e) => {
        eprintln!("Failed to load audio file: {}", e);
        self.skip_to_next_song()
    }
}
```

## 🧪 Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Writing Tests

- Write unit tests for new functions
- Add integration tests for complex features
- Test both success and error cases
- Mock external dependencies when needed

### Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_next_song() {
        let mut player = MusicPlayer::new();
        player.load_playlist(&["song1.mp3", "song2.mp3"]);

        player.play_song_at_index(0);
        assert_eq!(player.current_song_index(), Some(0));

        player.play_next_song().unwrap();
        assert_eq!(player.current_song_index(), Some(1));
    }
}
```

## 📚 Documentation

### Types of Documentation

- **Code comments**: Explain complex logic
- **Function documentation**: Use `///` for public APIs
- **README updates**: Keep the main README current
- **Architecture docs**: Document major design decisions

### Documentation Style

```rust
/// Represents a music player with playlist management capabilities.
///
/// The `MusicPlayer` handles audio playback, playlist management, and
/// user interface state. It supports multiple audio formats and provides
/// automatic song advancement.
///
/// # Examples
///
/// ```
/// let mut player = MusicPlayer::new();
/// player.load_folder("/path/to/music")?;
/// player.play_song(0, 0)?;
/// ```
pub struct MusicPlayer {
    // ...
}
```

## 🏗️ Architecture Guidelines

### Project Structure

```
src/
├── main.rs              # Application entry point
├── app.rs               # Main application logic
├── models/              # Data structures and business logic
├── player/              # Audio engine and playback
├── ui/                  # User interface components
└── utils/               # Shared utilities
```

### Design Principles

- **Separation of concerns**: Keep UI, business logic, and audio separate
- **Error handling**: Use `Result` types and handle errors gracefully
- **Performance**: Avoid blocking the UI thread
- **Maintainability**: Write code that's easy to understand and modify

## 🌟 Feature Development Guidelines

### Adding New Features

1. **Design first**: Consider the user experience
2. **Start small**: Implement a minimal viable version
3. **Iterate**: Get feedback and improve
4. **Document**: Update relevant documentation

### UI/UX Considerations

- Maintain consistency with existing design
- Consider accessibility
- Test on different screen sizes
- Follow platform conventions

## 🐛 Bug Reports

### What Makes a Good Bug Report

- **Clear title**: Summarize the problem
- **Steps to reproduce**: Detailed instructions
- **Expected behavior**: What should happen
- **Actual behavior**: What actually happens
- **Environment**: OS, Rust version, etc.
- **Screenshots**: For UI issues

### Bug Report Template

```
**Describe the bug**
A clear description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. See error

**Expected behavior**
A clear description of what you expected to happen.

**Screenshots**
If applicable, add screenshots to help explain your problem.

**Environment:**
- OS: [e.g. Ubuntu 22.04]
- Rust version: [e.g. 1.75]
- Rustify version: [e.g. 0.1.0]
```

## 💬 Community

### Getting Help

- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Code Review**: Learn from feedback on your PRs

### Communication Guidelines

- Be patient and respectful
- Ask specific questions
- Provide context and details
- Help others when you can

## 🎉 Recognition

We appreciate all contributions! Contributors will be:

- Listed in our contributors section
- Mentioned in release notes for significant contributions
- Invited to participate in project direction discussions

## 📝 License

By contributing to Rustify, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Rustify! 🎵🦀
