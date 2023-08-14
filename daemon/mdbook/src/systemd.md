# `systemd`
This is a relatively hardened [`systemd`](https://en.wikipedia.org/wiki/Systemd) user-service file for `festivald`.

It can be used as is, assuming `festivald` is at `/usr/bin/festivald`.

It should be placed at:
```
/home/$USER/.config/systemd/user/festivald.service
```
and launched with:
```bash
systemctl --user start festivald
```

## `festivald.service`
```toml
{{#include ../../config/festivald.service}}
```
