# Audio
Miscellaneous parts of `festivald` related to audio.

### Audio Output Device
`festivald` automatically connects to the "default" audio output device on the machine it is running on.

If a connection failure occurs during playback, it will continue to play, frequently attempting to reconnect.

If started without any audio output devices available, it will attempt to reconnect every 5 seconds, forever.

Thus, reconnecting hardware/interfaces mid-playback should be fine.

This behavior is [`🔴 Unstable`](api-stability/marker.md) and may change in the future.

Ideally, an enumeration and customizable selection of all audio devices would be available.

### Audio State
By default, `festivald` will save audio state upon clean [shutdown](json-rpc/daemon/daemon_shutdown.md) (or `CTRL+C/SIGINT`), and recover audio state upon startup.

This means audio will start playing exactly where it left off, and the queue will be intact.

This can be disabled with the [`recover_audio_state`](config.md) config option or the [`--disable-restore-audio-state`](command-line/command-line.md) command-line flag.

### Media Controls
By default, `festivald` will hook into the OS's native media controls to display some metadata and allow for playback via the OS's interface or via keyboard play/pause/stop/etc signals.

This can be disabled with the [`media_controls`](config.md) config option or the [`--disable-media-controls`](command-line/command-line.md) command-line flag.
