# Sound Settings

The `[sound]` section controls audio notifications when transitioning between
work/break phases. By default, tomat plays high-quality WAV files built into the
application. On Linux, this requires ALSA (Advanced Linux Sound Architecture);
on macOS, Tomat uses Core Audio without additional dependencies. If the audio
system is unavailable, Tomat falls back to the system beep or disables audio.

```toml
[sound]
mode = "embedded"
volume = 0.5
```

## Options

`mode`
: Sound notification mode. Controls how phase transitions are announced.

  `"embedded"` (default)
  : Use built-in audio files

  `"system-beep"`
  : Use system beep (terminal bell)

  `"none"`
  : No sound notifications

`volume`
: Audio volume level for embedded and custom sounds (0.0-1.0). Default: `0.5`

`work_to_break`
: Path to custom sound file for work→break transitions. Overrides embedded sound. Optional.

`break_to_work`
: Path to custom sound file for break→work transitions. Overrides embedded sound. Optional.

`work_to_long_break`
: Path to custom sound file for work→long break transitions. Overrides embedded sound. Optional.


## Running Sound

The `[sound.running]` table plays a looping sound while the timer counts down,
for example a ticking clock or a noise loop. It stops when the timer is paused,
stopped, or enters a phase that is not listed in `phases`. The running sound
needs `mode = "embedded"`.

```toml
[sound.running]
enabled = true
phases = ["work"]
```

`enabled`
: Play the running sound. Default: `false`

`phases`
: Phases in which the sound plays. Any of `"work"`, `"break"`, `"long-break"`.
  Default: `["work"]`

`file`
: Path to a custom loop file. Overrides the embedded tick. Optional.

`volume`
: Volume level for the running sound (0.0-1.0). Default: the `[sound]` `volume`
  value.

> [!NOTE]
>
> The audio device stays open while the running sound plays. Transition
> sounds open the device only for a few seconds.

`"enabled"`
: Enable sound notifications.

  > [!WARNING]
  > 
  > Deprecated option. Use `mode = "embedded"` instead.

`"system_beep"`
: Use system beep.

  > [!WARNING]
  >
  > Deprecated option. Use `mode = "system-beep"` instead.

`"use_embedded"`
: Use embedded sounds.

  > [!WARNING]
  >
  > Deprecated option. Use `mode = "embedded"` instead.


## Examples

To use your own sound files, keep `mode = "embedded"` and specify paths to your
audio files. Custom sounds override the built-in ones:

```toml
[sound]
mode = "embedded"
work_to_break = "/home/user/sounds/work-done.ogg"
break_to_work = "/home/user/sounds/break-over.ogg"
work_to_long_break = "/home/user/sounds/long-break.ogg"
volume = 0.7
```

To disable all audio:

```toml
[sound]
mode = "none"
```

To play the embedded tick during work sessions:

```toml
[sound.running]
enabled = true
```

To play your own ambience loop during work and breaks at a low volume:

```toml
[sound.running]
enabled = true
phases = ["work", "break", "long-break"]
file = "/home/user/sounds/cafe.ogg"
volume = 0.2
```

To use system beep only:

```toml
[sound]
mode = "system-beep"
```
