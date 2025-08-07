use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct AudioPlayer {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Arc<Mutex<Option<Sink>>>,
    current_file: Arc<Mutex<Option<String>>>,
    start_time: Arc<Mutex<Option<Instant>>>,
    pause_time: Arc<Mutex<Option<Duration>>>,
    total_duration: Arc<Mutex<Option<Duration>>>,
    volume: Arc<Mutex<f32>>,
    is_paused: Arc<Mutex<bool>>,
}

impl Default for AudioPlayer {
    fn default() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();

        Self {
            _stream: stream,
            stream_handle,
            sink: Arc::new(Mutex::new(None)),
            current_file: Arc::new(Mutex::new(None)),
            start_time: Arc::new(Mutex::new(None)),
            pause_time: Arc::new(Mutex::new(None)),
            total_duration: Arc::new(Mutex::new(None)),
            volume: Arc::new(Mutex::new(0.7)),
            is_paused: Arc::new(Mutex::new(false)),
        }
    }
}

impl AudioPlayer {
    pub fn play_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(format!("File does not exist: {}", path.display()).into());
        }

        let file = File::open(path)
            .map_err(|e| format!("Failed to open file {}: {}", path.display(), e))?;

        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| format!("Failed to decode audio file {}: {}", path.display(), e))?;

        // Get duration from the source
        let duration = source.total_duration();

        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| format!("Failed to create audio sink: {}", e))?;

        let volume = *self.volume.lock().unwrap();
        sink.set_volume(volume);

        sink.append(source);
        sink.play();

        *self.sink.lock().unwrap() = Some(sink);
        *self.current_file.lock().unwrap() = Some(path.to_string_lossy().to_string());
        *self.start_time.lock().unwrap() = Some(Instant::now());
        *self.pause_time.lock().unwrap() = None;
        *self.total_duration.lock().unwrap() = duration;
        *self.is_paused.lock().unwrap() = false;

        println!("Playing file: {}", path.display());
        if let Some(dur) = duration {
            println!("Duration: {:.2}s", dur.as_secs_f64());
        }

        Ok(())
    }

    pub fn toggle_playback(&self) {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                let mut is_paused = self.is_paused.lock().unwrap();

                if *is_paused {
                    sink.play();
                    *is_paused = false;
                    // Resume from where we paused
                    if let Some(pause_duration) = *self.pause_time.lock().unwrap() {
                        *self.start_time.lock().unwrap() = Some(Instant::now() - pause_duration);
                    }
                    *self.pause_time.lock().unwrap() = None;
                } else {
                    sink.pause();
                    *is_paused = true;
                    // Record how much time has elapsed
                    if let Some(start) = *self.start_time.lock().unwrap() {
                        *self.pause_time.lock().unwrap() = Some(start.elapsed());
                    }
                }
            }
        }
    }

    pub fn stop(&self) {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                sink.stop();
            }
        }

        *self.sink.lock().unwrap() = None;
        *self.current_file.lock().unwrap() = None;
        *self.start_time.lock().unwrap() = None;
        *self.pause_time.lock().unwrap() = None;
        *self.total_duration.lock().unwrap() = None;
        *self.is_paused.lock().unwrap() = false;
    }

    pub fn is_playing(&self) -> bool {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                !sink.empty() && !*self.is_paused.lock().unwrap()
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn is_finished(&self) -> bool {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                sink.empty()
            } else {
                true
            }
        } else {
            true
        }
    }

    pub fn set_volume(&self, volume: f32) {
        let clamped_volume = volume.clamp(0.0, 1.0);

        if let Ok(mut volume_guard) = self.volume.lock() {
            *volume_guard = clamped_volume;
        }

        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                sink.set_volume(clamped_volume);
            }
        }
    }

    pub fn current_time(&self) -> f64 {
        let is_paused = *self.is_paused.lock().unwrap();

        if is_paused {
            // Return the paused time
            if let Some(pause_duration) = *self.pause_time.lock().unwrap() {
                pause_duration.as_secs_f64()
            } else {
                0.0
            }
        } else {
            // Return current elapsed time
            if let Some(start_time) = *self.start_time.lock().unwrap() {
                start_time.elapsed().as_secs_f64()
            } else {
                0.0
            }
        }
    }

    pub fn total_time(&self) -> f64 {
        if let Ok(duration_guard) = self.total_duration.lock() {
            if let Some(duration) = *duration_guard {
                duration.as_secs_f64()
            } else {
                // If we don't have duration from metadata, estimate based on file size
                // This is a fallback for formats that don't provide duration info
                180.0 // Default to 3 minutes for unknown duration
            }
        } else {
            0.0
        }
    }

    pub fn get_progress(&self) -> f32 {
        let current = self.current_time();
        let total = self.total_time();

        if total > 0.0 {
            (current / total).min(1.0) as f32
        } else {
            0.0
        }
    }

    pub fn seek(&self, time: f64) {
        // Note: rodio doesn't support seeking directly
        // This would require more complex implementation with symphonia
        println!(
            "Warning: Seeking to {} seconds is not yet implemented",
            time
        );
    }

    pub fn has_file_loaded(&self) -> bool {
        self.current_file.lock().unwrap().is_some()
    }
}

// Additional helper functions for audio format detection
pub fn is_audio_file<P: AsRef<Path>>(path: P) -> bool {
    if let Some(extension) = path.as_ref().extension() {
        let ext = extension.to_string_lossy().to_lowercase();
        matches!(
            ext.as_str(),
            "mp3" | "wav" | "ogg" | "flac" | "m4a" | "aac" | "wma" | "mp4"
        )
    } else {
        false
    }
}
