# Audio Issues

## No Sound Notifications

### Problem

Timer works but no audio plays during transitions.

### Solution

1. **Check audio configuration**:

   ```toml
   # $XDG_CONFIG_HOME/tomat/config.toml
   [sound]
   mode = "embedded"  # Must not be "none"
   ```

2. **Test system audio**:

   ```bash
   # Test if ALSA works
   aplay /usr/share/sounds/alsa/Front_Left.wav

   # Or try speaker-test
   speaker-test -t sine -f 1000 -l 1
   ```

3. **Check volume levels**:
   - System volume (alsamixer, pavucontrol)
   - Tomat volume in config (0.0-1.0)

4. **Try different audio modes**:
   ```toml
   [sound]
   mode = "system-beep"  # Use system beep instead
   ```

## Wrong Audio Device

### Problem

Audio plays on wrong device or not audible.

### Solution

1. **Check default ALSA device**:

   ```bash
   aplay -l  # List audio devices
   cat ~/.asoundrc  # Check ALSA configuration
   ```

2. **Use system beep as fallback**:
   ```toml
   [sound]
   mode = "system-beep"
   ```

## Custom Sound Files Not Working

### Problem

Custom sound files don't play.

### Solution

1. **Check file paths and existence**:

   ```bash
   ls -la /path/to/your/sound.ogg
   ```

2. **Verify file format** (must be supported audio format):

   ```bash
   file /path/to/your/sound.ogg
   # Common formats: WAV, OGG, FLAC, MP3
   ```

3. **Test file with system player**:

   ```bash
   aplay /path/to/your/sound.wav
   ```

4. **Use absolute paths**:
   ```toml
   [sound]
   mode = "embedded"
   work_to_break = "/home/user/sounds/work-done.ogg"  # Absolute path
   ```

## Running Sound Does Not Play

### Problem

`[sound.running]` is enabled but nothing loops while the timer runs.

### Solution

1. **Check the sound mode**. The running sound needs `mode = "embedded"`:

   ```toml
   [sound]
   mode = "embedded"

   [sound.running]
   enabled = true
   ```

2. **Check the phase**. By default the sound plays only during work sessions
   and stops while the timer is paused. Add breaks if you want them:

   ```toml
   [sound.running]
   phases = ["work", "break", "long-break"]
   ```

3. **Check the custom file**. The daemon logs a failure once per running
   period. Restart the daemon in the foreground to see it:

   ```bash
   tomat daemon stop
   tomat daemon run
   ```

## Running Sound Blocks Other Applications

### Problem

Another application cannot open the audio device while the timer runs.

### Solution

The running sound keeps the audio device open for the whole phase. Transition
sounds release it after a few seconds. Use a sound server such as PipeWire or
PulseAudio that mixes streams from several applications, or disable the running
sound:

```toml
[sound.running]
enabled = false
```
