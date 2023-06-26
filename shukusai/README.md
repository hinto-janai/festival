# `shukusai` documentation
This is an overview of Festival's internals: `shukusai`.

The code itself is also littered with comments. Some `grep`-able keywords:

| Word        | Meaning |
|-------------|---------|
| `INVARIANT` | This code makes an _assumption_ that must be upheld for correctness
| `SAFETY`    | This `unsafe` code is okay, for `x,y,z` reasons
| `FIXME`     | This code works but isn't ideal
| `HACK`      | This code is a brittle workaround
| `PERF`      | This code is weird for performance reasons
| `TODO`      | This has to be implemented
| `SOMEDAY`   | This should be implemented... someday

---

The crate [`festival`](https://crates.io/crates/festival) is being squatted, so instead, Festival's
original name, [`shukusai`](https://crates.io/crates/shukusai), is the name used to represent the internals.

`祝祭/shukusai` translated means: `Festival`.

In documentation:

- `shukusai` _specifically_ means the internals
- `Festival` means a frontend OR the project as a whole

---

* [Code Structure](#Code-Structure)
	- [Data](#Data)
	- [Threads](#Threads)
	- [Misc](#Misc)
* [Overview](#Overview)
	- [Collection](#Collection)
	- [Kernel](#Kernel)
	- [Global State](#Global-State)
	- [CCD](#CCD)
	- [Watch](#Watch)
	- [Search](#Search)
	- [Audio](#Audio)
	- [Frontend](#Frontend)
* [Collection](#Collection-1)
	- [The 3 Slices](#The-3-Slices)
	- [Keys](#Keys)
	- [Construction and Destruction](#Construction-and-Destruction)
	- [Lifetime](#Lifetime)
	- [Mutation](#Mutation)
	- [File Format](#File-Format)
* [External Libraries](#External-Libraries)

---

# Code Structure
The structure of the folders & files located in `src/`.

## Data
These folders represent data:

| Folder         | Purpose |
|----------------|---------|
| `collection/`  | Data for `Collection` and everything `Collection`-related
| `frontend/`    | Some frontend-specific compatibility variables/functions
| `sort/`        | The sorting methods for the `Collection`
| `state/`       | Global state

## Threads
These folders represent OS threads with a distinct purpose:

| Folder           | Purpose |
|------------------|---------|
| `audio/`         | Audio demuxing + decoding + playback thread
| `ccd/`           | CCD (Collection Constructor Destructor)
| `kernel/`        | Coordinates all threads and resources
| `search/`        | Searches the `Collection` with arbitrary inputs
| `watch/`         | Watchs the filesystem for signals

## Misc
These are top-level `src/` files for miscellaneous stuff:

| File           | Purpose |
|----------------|---------|
| `constants.rs` | General `shukusai` and `Festival` constants
| `lib.rs`       | `pub` re-exposing, `doc` comments
| `logger.rs`    | Console logging initialization
| `panic.rs`     | Custom panic handling
| `signal.rs`    | Data definitions for filesystem-based signals
| `thread.rs`    | Thread count helpers
| `validate.rs`  | Data validation for the `Collection`

# Overview
The relationship between the main threads and data. Threads (mostly) communicate via `crossbeam::channel`'s.

<img src="../assets/images/diagram/overview.png" width="80%"/>

A simplified explanation of what happens at `main()`:

1. User opens `Festival`
2. CLI arguments are handled
3. `Kernel` <-> `Frontend` channels created
4. `Kernel` is spawned, given channel
5. `Kernel` spawns everyone else, starts initialization
6. `main()` directly turns into `Frontend`, taking the other end of the channel

## Collection
This is the core data structure that holds the (meta)data of the user's music collection.

It is mostly made up of:
- `Artist`'s
- `Album`'s
- `Song`'s
- Art
- Relational data
- Sorted indices

The actual audio data is not stored, rather a `PATH` to where that particular `Song` can be found is saved, album art bytes on the other hand _is_ saved (after being resized).

`Collection`'s lifetime is as long as `Festival` is open, or until a reset is requested.

After initial creation, `Collection` is permanently wrapped in an `Arc` such that all threads can have fast access to it while disallowing any mutation.

## Kernel
`Kernel` is the messenger and coordinator for all threads.

When any thread needs to do anything, they usually just go through `Kernel` via a channel message.

The main reasons `Kernel` exists:

1. To provide a small and single interface to other threads
2. Ease communication between threads (reduce blocking calls)
3. Manage resources, and in general be "the one in control"

## Global State
The big exceptions to the notion of `Kernel` and everything going through it is the presence of global state such as `AUDIO_STATE`, and technically the `Collection` itself, since almost all thread have an `Arc<Collection>`.

There are also many smaller one-off global atomics and such that are essentially globally mutable state.

These are used where the `Kernel` interface abstraction doesn't really do much other than add "cleanliness". Sometimes, the state truly is global (1 copy, it lasts forever, all parts of the system use it) like `AUDIO_STATE`, so it actually being a `pub static` makes sense.

## CCD
This is temporary thread that gets spun up by `Kernel` for either:

a. Creating a new `Collection` from scratch
b. Converting an already existing `Collection`'s images

 All the functions related to the actual _construction_ of the `Collection` belong to `CCD` (found in its folder `ccd/`). `CCD` itself spins up even more threads under its own control (`worker` threads) to process data in parallel.

`CCD` is also given the task of `drop()`'ing the old `Collection`.

Since dropping a potentially large recursive object like `Collection` might take a while, it shouldn't be done in the `Frontend` thread. Best case, a couple frames are skipped, worst case, it freezes for a few seconds.

Thus, `CCD` is the **Collection Constructor Destructor.**

## Watch
`Watch` watches the filesystem (local data folder, e.g.: `~/.local/share/festival/gui/signal`) for file-based signals.

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
just creates a file called `play` within `~/.local/share/festival/gui/signal/`, which `Watch` promptly deletes after sending a message to `Kernel` saying that the user requested `Play`.

The command:
```bash
touch ~/.local/share/festival/gui/signal/play
```
is equivalent.

## Search
`Search` sleeps waiting for arbitrary `String` inputs from `Kernel` and searches the `Collection` with it.

`Search` returns sorted `Artist`, `Album`, and `Song` keys that go from most similar, to least similar to the input.

A `HashMap` of already inputted `String`'s and sorted results are kept around by `Search` to act as a cache.

The algorithm used is a string-similarity comparison provided by [strsim](https://docs.rs/strsim), specifically the `jaro` variant.

## Audio
`Audio` is responsible for many things:
- Continuous decoding/demuxing/resampling of the current audio file
- Handling logic for the messages (clear queue, add stuff to queue, etc)
- Handling audio output devices, writing data to them
- Updating the global `AUDIO_STATE`
- Updating OS media controls metadata

## Frontend
The `Frontend` really just is a `Collection` visualizer. Since the `Collection` is wrapped in an `Arc` for all its lifetime, the `Frontend` thread has (very fast) access to all of its contents, but cannot mutate it.

Instead, if `Frontend` (really, the user) clicks a particular song, really what is happening is that `Frontend` sends a message to `Kernel` detailing exactly what `Song` should be played, added to the queue, removed from a playlist, etc.

There can be `State/Settings` that `Frontend` holds ownership over and can mutate for itself, e.g: sorting method, art size, etc, but those will be dependent on the frontend implementation itself rather than everything else in the system.

# Collection
The core "database" that holds all the (meta)data about the user's music.

This would normally be an _actual_ database like `SQLite`, `Postgres`, `LMDB`, etc, but `shukusai` just uses a regular old `struct` made up of `Vec`'s, `Box<[T]>`'s and a few other `std` types. The entire `Collection` is in-memory at all times.

The main reasons why this is done:

- Performance
- `std` types instead of `C` bindings
- Access to all `slice` methods (`albums.iter()`) and indexing (`albums[88]`)
- Metadata is static (there's _always_ an artist, track length, song title, etc). This makes a "hand-crafted" database less insane than it sounds since the _type_ of data is known at compile time (song title lengths will vary, but there _will_ be a title).
- I'm stupid and this was the first idea I thought of when considering how `egui` would (cheaply) draw all the album art

Here's an abbreviated version of what `Collection` actually looks like:
```rust
struct Collection {
	artists: Box<[Artist]>,
	albums: Box<[Album]>,
	songs: Box<[Song]>,

	sorted_artists: Box<[ArtistKey]>,
	sorted_albums: Box<[AlbumKey]>,
	sorted_songs: Box<[SongKey]>,

	[...]
}
```

## The 3 Slices
The three main `Box<[T]>`'s that the `Collection` is made up of are:

1. `Box<[Artist]>`
2. `Box<[Album]>`
3. `Box<[Song]>`

These 3 are completely separate.

The actual `struct`'s in the `Collection` aren't raw `Box<[T]>`, but a typed wrapper around it that re-implements indexing, but with `Key`'s instead of `usize`:
```rust
pub struct Artists(Box<[Artist]>);
```

## Keys
`Key`'s are just typed wrappers around a `usize`, literally defined like so:
```rust
struct ArtistKey(usize);
struct AlbumKey(usize);
struct SongKey(usize);
```
This is 100% just for compile-time type safety.

When indexing `Box<[Artist]>`, you really want to make sure you have a `usize` corresponding to `Box<[Artist]>`, so instead of raw indexing like so:
```rust
let index: usize = 100;
//  ^
//  |_ not necessarily clear which slice this is for.

collection.songs[index];   // Compiles, works
collection.albums[index];  // Compiles, maybe will `panic!()`
collection.artists[index]; // Compiles, maybe will `panic!()`
```
Having this `Key` type makes what thing we're looking for explicit:
```rust
let key = SongKey::from(100);

collection.songs[key];   // We get songs[100].
collection.artists[key]; // Compile error: type `Artists` cannot be index by `SongKey`!
```

The problem of relational data being lost between the 3 `slice`'s is solved by embedding the related `Key`'s within the `Artist/Album/Song` structs:

<img src="../assets/images/diagram/doubly.png" width="80%"/>

This is essentially a doubly-linked-list, but across slices. Since the `Collection` is immutable, we get to rely on indices always being stable, meaning `artist[0]` will always `== artist[0]` and that saving to disk is possible (it's just a `usize`).

This relational-link only needs to be done once, when the `Collection` is initially created, where-after, when given _any_ `Song`, you can traverse to the `Album`, and then to the `Artist` or vice-versa.

## Construction and Destruction

The linking and creation of the `Collection` itself is done by the `Collection Constructor Destructor`, or, `CCD`.

The "create a new `Collection`" process, simplified:

1. User requests a new `Collection`
2. `Kernel` spawns `CCD`
3. `CCD` deconstructs the old `Collection`
4. `CCD` creates the new `Collection`
5. `CCD` sends the new `Collection` to `Kernel`
6. `Kernel` sends a pointer to the new `Collection` to all threads
7. `CCD` saves the new `Collection` to disk in the background

The request for a new `Collection` always comes from the user (GUI, in this diagram).

After sending the `PATH`'s it wants scanned to `Kernel`, `GUI` drops its pointer to the old `Collection`.

`Kernel` then tells everyone to drop their old pointers, and goes into "CCD" mode, spawning `CCD`, giving it the last old `Arc` and only listening to it.

`CCD`, being the last one holding onto an `Arc` to the _old_ `Collection`, will be the one actually _deconstructing_ the underlying memory.

## Lifetime
`Collection` gets created _once_, (its core data) never gets mutated, and lives in memory as long as `Festival` is open or until the user requests a new one.

To prevent invalidating all the indices to the 3 `slice`'s, `Collection` _must_ be immutable.

This unfortunately means the lifetime of user-created playlists are tied with the `Collection`.

## Mutation
After `CCD` hands off the finished `Collection` to `Kernel`, `Kernel` wraps it in an `Arc` and it becomes immutable from that point.

**Although, mutation does occur at a single point:** when `Festival` starts up, `Collection` is read from disk, and the `Album` art is converted from `Vec<u8>` into `egui` images. Other than this, the core data is _never_ touched. The reason why this isn't handled by `serde` and immediately wrapped into an `Arc` is because that image conversion has been customized to use multiple threads.

## File Format
[`Bincode`](https://github.com/bincode-org/bincode) is used to save the `Collection` and its related data structures (`Queue`, `Playlist`) to disk.

[`TOML`](https://github.com/ordian/toml_edit) is used for miscellaneous state, like the `GUI` settings.

`shukusai` uses the library `disk` that adds some extra features to `Bincode` files. In particular, it adds a versioning system.

This is for when I eventually realize the `Collection` has a mistake in it's structure, or that something should be changed.

The versioning system is quite simple, `25` bytes of data are just added to the front of the file before saving.

The first `24` bytes are a unique header. This is to make sure the following byte representing the version _is_ the version byte:
```rust
// The 24 unique bytes our Bincode files will start with.
// It is the UTF-8 encoded string "-----BEGIN FESTIVAL-----" as bytes.
// The next byte _should_ be our version, then our actual data.
const FESTIVAL_HEADER: [u8; 24] = [
    45, 45, 45, 45, 45,             // -----
    66, 69, 71, 73, 78,             // BEGIN
    32,                             //
    70, 69, 83, 84, 73, 86, 65, 76, // FESTIVAL
    45, 45, 45, 45, 45              // -----
];
```
The next byte is the version, represented by a `u8`, meaning it has `0-255` different possible values:
```rust
// Current major version of the `Collection`.
const COLLECTION_VERSION: u8 = 0;
```
After this, the rest of the bytes should be the `Collection` in binary form.

Currently, album art is represented in the `Collection` as `Option<Vec<u8>>` (RBG format), which gets transformed into images the frontend can actually display at runtime. The current GUI is `egui`, so it gets turned into `egui_extras::RetainedImage`. This will obviously change as different frontends are made.

# External Libraries
There are forks of external libraries located in `external/` that contain some custom patches.

More details can be found at `external/README.md`.
