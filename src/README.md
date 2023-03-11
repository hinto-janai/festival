# Festival Documentation
This is a high-level overview of `Festival`.

For details on any part of the system, look within any given sub-directory for its `README.md`. It contains more specific documentation. The code itself is also littered with comments.

* [Code Structure](#Code-Structure)
	- [Data](#Data)
	- [Threads](#Threads)
	- [Misc](#Misc)
* [Threads](#Threads)
	- [Kernel](#Kernel)
	- [CCD](#CCD)
	- [Watch](#Watch)
	- [Search](#Search)
	- [Audio](#Audio)
	- [GUI](#GUI)
* [Collection](#Collection)
* [Bootstrap](#Bootstrap)
* [Scale](#Scale)

---

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
| `ccd/`         | CCD thread (Collection Constructor Destructor)
| `kernel/`      | Coordinates all threads and resources
| `search/`      | Search thread (searches the `Collection` with arbitrary input)
| `watch/`       | Watch thread (watching for user CLI commands)

### Misc
These are top-level files for miscellaneous stuff:

| File           | Purpose |
|----------------|---------|
| `cli.rs`       | Command line argument parsing/handling
| `constants.rs` | General constants
| `logger.rs`    | Console logging initialization
| `macros.rs`    | General macros
| `main.rs`      | Barebones `main()` that starts stuff and turns into `GUI`
| `regex.rs`     | General regexes

---

## Threads
<div align="center">

The relationship between the main threads and the data. They communicate via `crossbeam_channels`.

<img src="assets/images/diagram/overview.png" width="66%"/>

</div>

### Kernel
`Kernel` acts as the coordinator for all other threads. If ownership of any long-lived data is required, `Kernel` is most likely the one to have it. Mutable access to shared data is also restricted to `Kernel`. When any thread needs to mutate something (or really do anything), they usually need to go through `Kernel` first.

Each thread in the system only has one `Sender/Receiver` channel pair, all sending/receiving to & from `Kernel`. Although, `Kernel` is the exception, it has channels to send and receive from _all_ threads.

The main reasons `Kernel` exists:

1. To provide a **small** and **single** interface to other threads
2. Ease communication between threads (reduce blocking calls)
3. Manage resources, and in general be "the one in control"

### CCD
This is temporary thread that gets spun up by `Kernel` for either:

a. Creating a new `Collection` from scratch
b. Converting an already existing `Collection`'s images

All the functions related to the actual _construction_ of the `Collection` belong to `CCD` (found in its folder `ccd/`). `CCD` itself spins up even more threads under its own control (`worker` threads) to process data in parallel.

At the end of option `a)`, `CCD` is given its final task in life: to `drop()` the old `Collection`.

Since dropping a potentially large recursive object like `Collection` might take a while, it shouldn't be done in the `GUI` thread. Best case, a couple frames are skipped, worst case, it freezes for a few seconds. `Kernel` could handle dropping it, but then it would take that much more time to return.

Since we have a perfectly good thread (`CCD`) on its way out anyway... why not give it the (slightly cruel) final job of _deconstruction_, since it won't block anyone?

Thus, `CCD` is the **Collection Constructor Destructor.**

### Watch
`Watch` watches the filesystem (local data folder, e.g.: `~/.local/share/festival/signal`) for file-based signals.

Instead of implementing UNIX sockets and Windows named pipes, or [using](https://github.com/servo/ipc-channel) a [dependency](https://github.com/kotauskas/interprocess), regular files are used to indicate the simple boolean signals needed by the `Watch` thread.

For example:

- Play
- Pause
- Next
- etc...

This command:
```bash
./festival --play
```
just creates a file called `play` within `~/.local/share/festival/signal/`, which `Watch` promptly deletes after sending a message to `Kernel` saying that the user requested `Play`.

The command:
```
touch ~/.local/share/festival/signal/play
```
is equivalent.

`Watch` never reads the data of the file itself since the file itself existing represents `true`.

The filesystem watching functionality is provided by [`notify`](https://github.com/notify-rs/notify).

### Search
`Search` sleeps waiting for arbitrary `String` inputs from `Kernel` and searchs the `Collection` with it.

`Search` returns sorted `Artist`'s, `Album`'s, and `Song`'s that go from most similar, to least similar to the input.

A `HashMap` of already inputted `String`'s and sorted results are kept around by `Search` to act as a cache.

The algorithm used is a string-similarity comparison provided by [strsim](https://docs.rs/strsim), specifically the `jaro` variant.

### Audio
`Audio` is responsible for the continuous demuxing and decoding of a given audio file.

It also writes that data to the audio hardware so that it can be played.

This is provided by [`Symphonia`](https://github.com/pdeljanov/Symphonia).

`Audio` also sends/receives messages from `Kernel`, e.g: pause/play, adjust volume, etc.

### GUI
`GUI` is the thread that runs... the GUI.

Just like the other threads, `GUI` doesn't control much, it just sends/receives messages from `Kernel`.

`GUI` really just is a frontend to visualize the `Collection` itself. Since the `Collection` is wrapped in an `Arc` for all its lifetime, the `GUI` thread has (very fast) access to all of its contents, but cannot mutate it.

Instead, if `GUI` (really, the user) clicks a particular song, really what is happening is that `GUI` sends a message to `Kernel` detailing exactly what `Song` should be played, added to the queue, removed from a playlist, etc.

There are `State/Settings` that `GUI` holds ownership over and can mutate for itself, e.g: sorting method, art size, etc, but those will be dependent on the frontend implementation itself rather than everything else in the system.

The GUI library currently used is [`egui`](https://github.com/emilk/egui).

---

## Collection
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
### Kernel order of events
### Bincode and version headers
### Performance cost of message passing vs direct access (GUI & CCD progress msg)
#### Metadata vs Same dir image (which image among multiple?)

### Audio Codec
- AAC
- ALAC
- FLAC
- MP3
- Ogg/OPUS
- PCM (wav, aiff)

## Bootstrap

## Scale

## Disk
### Collection
### Settings/State

## Lib
### Disk
### Human
### RoLock
