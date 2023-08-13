# Config
`festivald.toml` is `festivald`'s config file. It is in the [`TOML`](https://en.wikipedia.org/wiki/TOML) format.

Values will be loaded from this file on startup.

If overlapping `--command-flags` are provided, they will be used instead.

It is located at:

| Platform | Example                                                                  |
|----------|--------------------------------------------------------------------------|
| Windows  | `C:\Users\USER\AppData\Roaming\Festival\config\daemon\festivald.toml`    |
| macOS    | `/Users/USER/Library/Application Support/Festival/daemon/festivald.toml` |
| Linux    | `/home/USER/.local/share/festival/daemon/festivald.toml`                 |

# `systemd` service
`festivald.service` is a `systemd` user-service file for `festivald`.

It can be used as is assuming `festivald` is at `/usr/bin/festivald`.

It should be placed at:
```
/home/$USER/.config/systemd/user/festivald.service
```
and launched with:
```bash
systemctl --user start festivald
```

`festivald` will connect to and use the audio server that the user has access to (most likely `PulseAudio`).
