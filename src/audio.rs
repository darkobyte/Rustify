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
    duration: Arc<Mutex<Option<Duration>>>,
    volume: Arc<Mutex<f32>>,
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
            duration: Arc::new(Mutex::new(None)),
            volume: Arc::new(Mutex::new(0.7)),
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

        // Get duration from the source if possible
        let duration = source.total_duration();

        let sink = Sink::try_new(&self.stream_handle)
            .map_err(|e| format!("Failed to create audio sink: {}", e))?;

        let volume = *self.volume.lock().map_err(|_| "Volume lock poisoned")?;
        sink.set_volume(volume);

        sink.append(source);
        sink.play();

        *self.sink.lock().map_err(|_| "Sink lock poisoned")? = Some(sink);
        *self
            .current_file
            .lock()
            .map_err(|_| "Current file lock poisoned")? = Some(path.to_string_lossy().to_string());
        *self
            .start_time
            .lock()
            .map_err(|_| "Start time lock poisoned")? = Some(Instant::now());
        *self.duration.lock().map_err(|_| "Duration lock poisoned")? = duration;

        Ok(())
    }

    pub fn toggle_playback(&self) {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                if sink.is_paused() {
                    sink.play();
                } else {
                    sink.pause();
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

        if let Ok(mut sink_guard) = self.sink.lock() {
            *sink_guard = None;
        }
        if let Ok(mut file_guard) = self.current_file.lock() {
            *file_guard = None;
        }
        if let Ok(mut time_guard) = self.start_time.lock() {
            *time_guard = None;
        }
        if let Ok(mut duration_guard) = self.duration.lock() {
            *duration_guard = None;
        }
    }

    pub fn is_playing(&self) -> bool {
        if let Ok(sink_guard) = self.sink.lock() {
            if let Some(sink) = sink_guard.as_ref() {
                !sink.is_paused() && !sink.empty()
            } else {
                false
            }
        } else {
            false
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
        if let (Ok(start_time_guard), Ok(sink_guard)) = (self.start_time.lock(), self.sink.lock()) {
            if let (Some(start_time), Some(_)) = (*start_time_guard, sink_guard.as_ref()) {
                start_time.elapsed().as_secs_f64()
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    pub fn total_time(&self) -> f64 {
        if let Ok(duration_guard) = self.duration.lock() {
            if let Some(duration) = *duration_guard {
                duration.as_secs_f64()
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    pub fn seek(&self, time: f64) {
        // Note: rodio doesn't support seeking directly
        // This would require more complex implementation with symphonia
        log::warn!("Seeking to {} seconds is not yet implemented", time);
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
