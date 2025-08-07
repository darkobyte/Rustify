# Rustify - Production Deployment Guide

This guide covers building, optimizing, and deploying Rustify for production use.

## Prerequisites

- Rust 1.70 or later
- System audio libraries (ALSA on Linux, CoreAudio on macOS, WASAPI on Windows)
- Git (for version control)

## Building for Production

### 1. Release Build

```bash
# Build optimized release version
cargo build --release

# The binary will be located at:
# target/release/rustify (Linux/macOS)
# target/release/rustify.exe (Windows)
```

### 2. Cross-Platform Builds

#### Linux to Windows
```bash
# Install cross-compilation target
rustup target add x86_64-pc-windows-gnu

# Install mingw-w64 (Ubuntu/Debian)
sudo apt-get install mingw-w64

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu
```

#### Linux to macOS
```bash
# Install cross-compilation target
rustup target add x86_64-apple-darwin

# Build for macOS (requires additional setup)
cargo build --release --target x86_64-apple-darwin
```

### 3. Size Optimization

Add to `Cargo.toml` for smaller binaries:

```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Better optimization
panic = "abort"     # Smaller binary
strip = true        # Remove debug symbols
```

### 4. Performance Optimization

For maximum performance, use:

```toml
[profile.release]
opt-level = 3       # Maximum optimization
lto = "fat"         # Full LTO
codegen-units = 1
target-cpu = "native"
```

## Installation Methods

### 1. Direct Binary Distribution

```bash
# Create release directory
mkdir -p release/rustify-v0.1.0

# Copy binary and assets
cp target/release/rustify release/rustify-v0.1.0/
cp -r assets release/rustify-v0.1.0/
cp README.md LICENSE release/rustify-v0.1.0/

# Create tarball
cd release
tar -czf rustify-v0.1.0-linux-x86_64.tar.gz rustify-v0.1.0/
```

### 2. Cargo Installation

```bash
# Install from local source
cargo install --path .

# Install from Git repository
cargo install --git https://github.com/your-username/rustify
```

### 3. Package Managers

#### Arch Linux (AUR)
Create a PKGBUILD file:

```bash
# PKGBUILD
pkgname=rustify
pkgver=0.1.0
pkgrel=1
pkgdesc="Modern music player built with Rust"
arch=('x86_64')
url="https://github.com/your-username/rustify"
license=('MIT')
depends=('alsa-lib')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/your-username/rustify/archive/v$pkgver.tar.gz")

build() {
    cd "$pkgname-$pkgver"
    cargo build --release --locked
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 target/release/rustify "$pkgdir/usr/bin/rustify"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
```

#### Debian/Ubuntu (.deb)
Create a debian package:

```bash
# Install build dependencies
sudo apt-get install dpkg-dev

# Create package structure
mkdir -p rustify-deb/DEBIAN
mkdir -p rustify-deb/usr/bin
mkdir -p rustify-deb/usr/share/applications
mkdir -p rustify-deb/usr/share/doc/rustify

# Copy files
cp target/release/rustify rustify-deb/usr/bin/
cp README.md rustify-deb/usr/share/doc/rustify/

# Create control file
cat > rustify-deb/DEBIAN/control << EOF
Package: rustify
Version: 0.1.0
Section: sound
Priority: optional
Architecture: amd64
Depends: libasound2
Maintainer: Your Name <your.email@example.com>
Description: Modern music player built with Rust
 Rustify is a lightweight, modern music player with support for
 multiple audio formats and a clean user interface.
EOF

# Build package
dpkg-deb --build rustify-deb rustify_0.1.0_amd64.deb
```

### 4. AppImage (Linux)

```bash
# Download AppImage tools
wget https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage
chmod +x appimagetool-x86_64.AppImage

# Create AppDir structure
mkdir -p Rustify.AppDir/usr/bin
mkdir -p Rustify.AppDir/usr/share/applications
mkdir -p Rustify.AppDir/usr/share/icons/hicolor/256x256/apps

# Copy files
cp target/release/rustify Rustify.AppDir/usr/bin/
cp assets/icon-256.png Rustify.AppDir/usr/share/icons/hicolor/256x256/apps/rustify.png
cp assets/icon-256.png Rustify.AppDir/rustify.png

# Create desktop file
cat > Rustify.AppDir/rustify.desktop << EOF
[Desktop Entry]
Type=Application
Name=Rustify
Comment=Modern music player
Exec=rustify
Icon=rustify
Categories=AudioVideo;Audio;Player;
StartupNotify=true
EOF

cp Rustify.AppDir/rustify.desktop Rustify.AppDir/usr/share/applications/

# Create AppRun
cat > Rustify.AppDir/AppRun << 'EOF'
#!/bin/bash
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export PATH="${HERE}/usr/bin/:${PATH}"
exec "${HERE}/usr/bin/rustify" "$@"
EOF
chmod +x Rustify.AppDir/AppRun

# Build AppImage
./appimagetool-x86_64.AppImage Rustify.AppDir Rustify-x86_64.AppImage
```

## System Integration

### 1. Desktop Entry (Linux)

Create `/usr/share/applications/rustify.desktop`:

```ini
[Desktop Entry]
Type=Application
Name=Rustify
GenericName=Music Player
Comment=Modern music player built with Rust
Exec=rustify
Icon=rustify
Categories=AudioVideo;Audio;Player;
StartupNotify=true
MimeType=audio/mpeg;audio/mp4;audio/x-wav;audio/x-flac;audio/ogg;
```

### 2. MIME Types (Linux)

Create `/usr/share/mime/packages/rustify.xml`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<mime-info xmlns="http://www.freedesktop.org/standards/shared-mime-info">
  <mime-type type="application/x-rustify-playlist">
    <comment>Rustify Playlist</comment>
    <glob pattern="*.rustify"/>
  </mime-type>
</mime-info>
```

### 3. macOS Bundle

Create `Rustify.app` bundle structure:

```
Rustify.app/
├── Contents/
│   ├── Info.plist
│   ├── MacOS/
│   │   └── rustify
│   └── Resources/
│       └── rustify.icns
```

`Info.plist`:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>rustify</string>
    <key>CFBundleIdentifier</key>
    <string>com.yourname.rustify</string>
    <key>CFBundleName</key>
    <string>Rustify</string>
    <key>CFBundleVersion</key>
    <string>0.1.0</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
</dict>
</plist>
```

## Performance Tuning

### 1. System Requirements

**Minimum:**
- RAM: 128 MB
- CPU: Any x86_64 processor
- Storage: 50 MB free space
- Audio: Any compatible audio device

**Recommended:**
- RAM: 512 MB
- CPU: Multi-core processor
- Storage: 100 MB free space
- Audio: Dedicated audio interface

### 2. Runtime Optimizations

Set environment variables for better performance:

```bash
# Reduce audio latency
export PULSE_LATENCY_MSEC=30

# Enable hardware acceleration (if available)
export AUDIO_DRIVER=alsa

# Optimize for low-latency audio
export ALSA_PCM_DMIX=0
```

### 3. Configuration Tuning

Create optimized settings in `~/.config/rustify/settings.json`:

```json
{
  "audio": {
    "volume": 0.7,
    "sample_rate": 44100,
    "buffer_size": 1024
  },
  "library": {
    "auto_scan": true,
    "scan_interval_hours": 24
  },
  "ui": {
    "theme": "Dark",
    "window_size": [1200, 800]
  }
}
```

## Monitoring and Logging

### 1. Enable Logging

```bash
# Debug logging
RUST_LOG=debug rustify

# Error logging only
RUST_LOG=error rustify

# Custom log file
RUST_LOG=info rustify 2>&1 | tee rustify.log
```

### 2. Performance Monitoring

Use system tools to monitor performance:

```bash
# Monitor CPU/Memory usage
top -p $(pgrep rustify)

# Monitor audio performance
cat /proc/asound/cards
```

## Troubleshooting

### 1. Audio Issues

**No sound output:**
```bash
# Check audio devices
aplay -l

# Test audio
speaker-test -t wav

# Check PulseAudio
pulseaudio --check
```

**Crackling/distorted audio:**
- Increase buffer size in settings
- Close other audio applications
- Check system audio settings

### 2. Performance Issues

**High CPU usage:**
- Reduce UI update frequency
- Close unnecessary applications
- Check for file corruption

**High memory usage:**
- Clear library cache
- Reduce concurrent file operations
- Check for memory leaks

### 3. Library Scanning Issues

**Slow scanning:**
- Use SSD storage
- Exclude network drives
- Limit concurrent operations

**Missing files:**
- Check file permissions
- Verify supported formats
- Check symlinks

## Security Considerations

### 1. File Permissions

Ensure proper permissions for:
- Configuration directory: `~/.config/rustify/` (700)
- Music directories: Read access only
- Application binary: Execute permissions

### 2. Network Security

If adding network features:
- Use HTTPS for all connections
- Validate all user inputs
- Implement rate limiting

### 3. Sandboxing

For additional security, run in sandbox:

```bash
# Flatpak (Linux)
flatpak run com.yourname.Rustify

# Snap (Linux)
snap run rustify

# macOS Sandbox
sandbox-exec -f rustify.sb rustify
```

## Continuous Integration

### 1. GitHub Actions

Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macOS-latest]

    steps:
    - uses: actions/checkout@v2
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    
    - name: Build release
      run: cargo build --release
    
    - name: Upload artifacts
      uses: actions/upload-artifact@v2
      with:
        name: rustify-${{ matrix.os }}
        path: target/release/rustify*
```

### 2. Automated Testing

```bash
# Run all tests
cargo test --release

# Check code quality
cargo clippy -- -D warnings

# Format check
cargo fmt -- --check
```

## Support and Maintenance

### 1. User Support

- Create issue templates on GitHub
- Set up discussion forums
- Provide comprehensive documentation
- Maintain FAQ section

### 2. Updates

- Implement auto-update mechanism
- Provide migration guides
- Maintain backward compatibility
- Regular security updates

### 3. Metrics Collection

Consider implementing telemetry:
- Crash reporting
- Performance metrics
- Feature usage statistics
- Error tracking

Remember to respect user privacy and provide opt-out mechanisms for any data collection.