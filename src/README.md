# Festival Internal Documentation
* [Code Structure](#Code-Structure)
	- [Data](#Data)
	- [Threads](#Threads)
	- [Misc](#Misc)
* [Thread Model](#Thread-Model)
* [Bootstrap](#Bootstrap)
* [Scale](#Scale)

## Code Structure
### Data
These folders represent data:
| Folder         | Purpose |
|----------------|---------|
| `collection/`  | Data for `Collection` and everything `Collection`-related

### Threads
These folders represent OS threads with a distinct purpose:
| Folder         | Purpose |
|----------------|---------|
| `audio/`       | Audio demuxing + decoding + playback thread
| `gui/`         | GUI thread (egui)
| `collection/`  | `Collection` struct definitions
| `ccd/`         | CCD thread (Collector Constructor Destructor)
| `kernel/`      | Coordinates all threads and resources
| `search/`      | Search thread

### Misc
These are top-level files for miscellaneous stuff:
| File           | Purpose |
|----------------|---------|
| `cli.rs`       | Command line argument parsing/handling
| `constants.rs` | General constants
| `logger.rs`    | Console logging code
| `macros.rs`    | General macros
| `main.rs`      | Barebones `main()` that starts stuff and gives up control
| `regex.rs`     | General regexes

## Thread Model

## Collection Model
### HashMap vs Pointers (Rc) vs Indicies
### Sorting pointer/indicies vs actual vec
### Invalid pointers and indicies
### Keys and Keychain
### Queues and playlists
### CCD (Collector Constructor Destructor)
#### Fast Image Resizing

### Pluggable Client (GUI, CLI, Web, HTTP)
### Main Threads (GUI, Kernel, etc)
### Some threads can't hang
### Why Kernel?
### Bincode and version headers
### Performance cost of message passing vs direct access (GUI & CCD progress msg)
#### Metadata vs Same dir image (which image among multiple?)


## Bootstrap

## Scale

## Disk
### Collection
### Settings/State

## Lib
### Disk
### Human
### RoLock
