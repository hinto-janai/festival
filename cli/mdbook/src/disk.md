# Disk
`festival-cli` saves all of its files within various "OS" directories, owned by the user running it.

You can list all the PATHs on your system with:
```bash
./festival-cli --path
```

And delete them with with:
```bash
./festival-cli --delete
```

`festival-cli` saves everything into `cli/` because other Festival frontends also use the same `festival` project directory, e.g:
```
~/.local/share/festival/
  ├─ gui/
  ├─ daemon/
  ├─ cli/
```

## Config
Where `festival-cli`'s configuration files are saved.

| Platform | Value                                            | Example                                                 |
|----------|--------------------------------------------------|---------------------------------------------------------|
| Windows  | `{FOLDERID_RoamingAppData}\Festival\config\cli`  | `C:\Users\Alice\AppData\Roaming\Festival\config\cli`    |
| macOS    | `$HOME/Library/Application Support/Festival/cli` | `/Users/Alice/Library/Application Support/Festival/cli` |
| Linux    | `$HOME/.config/festival/cli`                     | `/home/alice/.config/festival/cli`                      |

The `Config` sub-directories/files, and their purpose:
```plaintext
├─ cli/
   │
   ├─ festival-cli.toml # Config file used by `festival-cli`
```

## Data
Where `festival-cli`'s data is saved.

| Platform | Value                                                                 | Example                                                 |
|----------|-----------------------------------------------------------------------|---------------------------------------------------------|
| Windows  | `{FOLDERID_RoamingAppData}\Festival\data\cli`                         | `C:\Users\Alice\AppData\Roaming\Festival\data\cli`      |
| macOS    | `$HOME/Library/Application Support/Festival/cli`                      | `/Users/Alice/Library/Application Support/Festival/cli` |
| Linux    | `$HOME/.local/share/festival/cli`                                     | `/home/alice/.local/share/festival/cli`                 |

The `Data` sub-directories/files, and their purpose:
```plaintext
├─ cli/
   │
   ├─ docs/ # The static documentations files used by `festival-cli`
```
