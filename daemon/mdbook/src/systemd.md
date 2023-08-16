# `systemd`
This is a relatively hardened [`systemd`](https://en.wikipedia.org/wiki/Systemd) service file for `festivald`.

`${USER}` should be replaced by a user that has access to an audio server (like PulseAudio).

It should be placed at:
```
/etc/systemd/system/festivald.service
```
and launched with:
```bash
sudo systemctl --user start festivald
```

## `festivald.service`
```toml
{{#include ../../config/festivald.service}}
```
