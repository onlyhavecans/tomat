#[cfg(feature = "audio")]
use rodio::{Decoder, Source};
#[cfg(feature = "audio")]
use std::io::Cursor;

// Embed sound files at compile time (only when audio feature is enabled)
#[cfg(feature = "audio")]
const WORK_TO_BREAK_SOUND: &[u8] = include_bytes!("../assets/sounds/work-to-break.wav");
#[cfg(feature = "audio")]
const BREAK_TO_WORK_SOUND: &[u8] = include_bytes!("../assets/sounds/break-to-work.wav");
#[cfg(feature = "audio")]
const WORK_TO_LONG_BREAK_SOUND: &[u8] = include_bytes!("../assets/sounds/work-to-long-break.wav");
#[cfg(feature = "audio")]
const TICK_SOUND: &[u8] = include_bytes!("../assets/sounds/tick.wav");

#[derive(Debug, Clone, Copy)]
pub enum SoundType {
    WorkToBreak,
    BreakToWork,
    WorkToLongBreak,
}

#[cfg(feature = "audio")]
pub fn play_embedded_sound(
    sound_type: SoundType,
    volume: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let sound_data = match sound_type {
        SoundType::WorkToBreak => WORK_TO_BREAK_SOUND,
        SoundType::BreakToWork => BREAK_TO_WORK_SOUND,
        SoundType::WorkToLongBreak => WORK_TO_LONG_BREAK_SOUND,
    };

    // Check if the sound data is just a placeholder (empty/minimal WAV)
    if sound_data.len() <= 44 {
        // Fallback to system beep for placeholder files
        play_system_beep();
        return Ok(());
    }

    // Use tokio::spawn_blocking for audio playback to avoid blocking the async runtime
    // and prevent holding the audio device open
    let sound_data = sound_data.to_vec();
    if tokio::runtime::Handle::try_current().is_ok() {
        tokio::task::spawn_blocking(move || {
            if let Ok(handle) = rodio::stream::DeviceSinkBuilder::open_default_sink() {
                let mixer = handle.mixer();

                let cursor = Cursor::new(sound_data);
                if let Ok(source) = Decoder::new(cursor) {
                    let source = source.amplify(volume);
                    mixer.add(source);
                    // Wait for playback to complete
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
                // Handle is dropped here, releasing the audio device
            }
        });
    } else {
        // Fallback to std::thread if not in tokio runtime (e.g., tests without runtime)
        std::thread::spawn(move || {
            if let Ok(handle) = rodio::stream::DeviceSinkBuilder::open_default_sink() {
                let mixer = handle.mixer();

                let cursor = Cursor::new(sound_data);
                if let Ok(source) = Decoder::new(cursor) {
                    let source = source.amplify(volume);
                    mixer.add(source);
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
            }
        });
    }

    Ok(())
}

#[cfg(feature = "audio")]
pub fn play_system_beep() {
    // Use tokio::spawn_blocking for beep playback
    if tokio::runtime::Handle::try_current().is_ok() {
        tokio::task::spawn_blocking(|| {
            if let Ok(handle) = rodio::stream::DeviceSinkBuilder::open_default_sink() {
                let mixer = handle.mixer();

                // Generate a simple beep tone
                let source = rodio::source::SineWave::new(800.0)
                    .take_duration(std::time::Duration::from_millis(300))
                    .amplify(0.3);

                mixer.add(source);
                std::thread::sleep(std::time::Duration::from_millis(400));
                // Handle is dropped here, releasing the audio device
            }
        });
    } else {
        // Fallback to std::thread if not in tokio runtime
        std::thread::spawn(|| {
            if let Ok(handle) = rodio::stream::DeviceSinkBuilder::open_default_sink() {
                let mixer = handle.mixer();

                let source = rodio::source::SineWave::new(800.0)
                    .take_duration(std::time::Duration::from_millis(300))
                    .amplify(0.3);

                mixer.add(source);
                std::thread::sleep(std::time::Duration::from_millis(400));
            }
        });
    }
}

#[cfg(feature = "audio")]
pub fn play_custom_file<P: AsRef<std::path::Path>>(
    path: P,
    volume: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Load file data before spawning task
    let file = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    let mut buffer = Vec::new();
    std::io::Read::read_to_end(&mut reader, &mut buffer)?;

    // Use tokio::spawn_blocking for audio playback
    if tokio::runtime::Handle::try_current().is_ok() {
        tokio::task::spawn_blocking(move || {
            if let Ok(handle) = rodio::stream::DeviceSinkBuilder::open_default_sink() {
                let mixer = handle.mixer();

                let cursor = Cursor::new(buffer);
                if let Ok(source) = Decoder::new(cursor) {
                    let source = source.amplify(volume);
                    mixer.add(source);
                    // Wait for playback to complete
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
                // Handle is dropped here, releasing the audio device
            }
        });
    } else {
        // Fallback to std::thread if not in tokio runtime
        std::thread::spawn(move || {
            if let Ok(handle) = rodio::stream::DeviceSinkBuilder::open_default_sink() {
                let mixer = handle.mixer();

                let cursor = Cursor::new(buffer);
                if let Ok(source) = Decoder::new(cursor) {
                    let source = source.amplify(volume);
                    mixer.add(source);
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
            }
        });
    }

    Ok(())
}

/// Looping playback that runs until the handle is dropped.
///
/// A dedicated thread owns the audio device for the lifetime of the loop and
/// releases it when the handle drops.
pub struct LoopPlayer {
    #[cfg(feature = "audio")]
    stop_tx: Option<std::sync::mpsc::Sender<()>>,
}

#[cfg(feature = "audio")]
impl LoopPlayer {
    /// Start looping the embedded tick sound
    pub fn embedded(volume: f32) -> Result<Self, Box<dyn std::error::Error>> {
        if TICK_SOUND.len() <= 44 {
            return Err("embedded tick sound is a placeholder".into());
        }
        Self::from_bytes(TICK_SOUND.to_vec(), volume)
    }

    /// Start looping a user-supplied audio file
    pub fn from_file<P: AsRef<std::path::Path>>(
        path: P,
        volume: f32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let data = std::fs::read(path)?;
        Self::from_bytes(data, volume)
    }

    fn from_bytes(data: Vec<u8>, volume: f32) -> Result<Self, Box<dyn std::error::Error>> {
        let (stop_tx, stop_rx) = std::sync::mpsc::channel::<()>();
        let (ready_tx, ready_rx) = std::sync::mpsc::channel::<Result<(), String>>();

        std::thread::Builder::new()
            .name("tomat-running-sound".into())
            .spawn(move || {
                let mut handle = match rodio::stream::DeviceSinkBuilder::open_default_sink() {
                    Ok(handle) => handle,
                    Err(e) => {
                        let _ = ready_tx.send(Err(format!("cannot open audio device: {e}")));
                        return;
                    }
                };
                handle.log_on_drop(false);

                let source = match Decoder::new_looped(Cursor::new(data)) {
                    Ok(source) => source,
                    Err(e) => {
                        let _ = ready_tx.send(Err(format!("cannot decode audio: {e}")));
                        return;
                    }
                };

                let player = rodio::Player::connect_new(handle.mixer());
                player.append(source.amplify(volume));
                let _ = ready_tx.send(Ok(()));

                // Block until the LoopPlayer handle drops
                let _ = stop_rx.recv();
                player.stop();
                // `player` and `handle` drop here, releasing the audio device
            })?;

        ready_rx.recv()??;

        Ok(Self {
            stop_tx: Some(stop_tx),
        })
    }
}

#[cfg(feature = "audio")]
impl Drop for LoopPlayer {
    fn drop(&mut self) {
        if let Some(tx) = self.stop_tx.take() {
            let _ = tx.send(());
        }
    }
}

#[cfg(not(feature = "audio"))]
impl LoopPlayer {
    pub fn embedded(_volume: f32) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }

    pub fn from_file<P: AsRef<std::path::Path>>(
        _path: P,
        _volume: f32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {})
    }
}

#[cfg(not(feature = "audio"))]
pub fn play_embedded_sound(
    _sound_type: SoundType,
    _volume: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Audio feature not enabled, do nothing
    Ok(())
}

#[cfg(not(feature = "audio"))]
pub fn play_system_beep() {
    // Audio feature not enabled, do nothing
}

#[cfg(not(feature = "audio"))]
pub fn play_custom_file<P: AsRef<std::path::Path>>(
    _path: P,
    _volume: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    // Audio feature not enabled, do nothing
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sound_types() {
        // Test that SoundType enum variants exist
        let _work_to_break = SoundType::WorkToBreak;
        let _break_to_work = SoundType::BreakToWork;
        let _work_to_long_break = SoundType::WorkToLongBreak;
    }

    #[test]
    #[allow(clippy::const_is_empty)]
    fn test_embedded_sounds_exist() {
        // Only test when audio feature is enabled
        #[cfg(feature = "audio")]
        {
            // Test that embedded sound data exists (even if placeholder)
            assert!(!WORK_TO_BREAK_SOUND.is_empty());
            assert!(!BREAK_TO_WORK_SOUND.is_empty());
            assert!(!WORK_TO_LONG_BREAK_SOUND.is_empty());
            assert!(TICK_SOUND.len() > 44);
        }
    }
}
