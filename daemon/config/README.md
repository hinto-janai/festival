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
`festivald.service` is a relatively hardened [`systemd`](https://en.wikipedia.org/wiki/Systemd) service file for `festivald`.

`${USER}` should be replaced by a user that has access to an audio server (like PulseAudio).

It should be placed at:
```
/etc/systemd/system/festivald.service
```
and launched with:
```bash
sudo systemctl start festivald
```
