# Config
`festivald` reads and loads its configuration file, `festivald.toml`, on startup. It controls various behaviors of `festivald`.

Exactly where this file is depends on the OS, more details in the [`Disk`](disk.md) section.

[`Command Line`](command-line/command-line.md) flags will override any overlapping config values.

## `festivald.toml`
This is the default configuration file `festivald` creates and uses.

If `festivald` is started with no `--flags`, e.g:
```bash
./festivald
```
Then it will be equivalent to this config file.

```toml
{{#include ../../config/festivald.toml}}
```
