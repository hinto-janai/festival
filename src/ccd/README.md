# Core functions
`CCD` has a lot of different types of tasks, so instead of shoving them all in `ccd.rs`, it acts more as a "frontend" to `Kernel`, exposing short public functions. The actual individual tasks are logically seperated into their own files. Unit tests and `use` statements are also more organized this way.

All these functions are only separted for clarity, in reality they are going to be ran right next to each other. They all have side-effects: sending update to `Kernel` via a channel, this is why the `Sender` is in all the function signatures (it would be in local scope if all this code was in _one big function_).

| File           | Purpose |
|----------------|---------|
| ccd.rs         | High-level convenience wrapper functions for `Kernel` that call the below internal ones
| convert.rs     | Converting art bytes into `egui` images
| date.rs        | Date from `str` parsing functions
| metadata.rs    | Audio metadata (MIME) processing
| sort.rs        | `Collection` sorting functions
| walk.rs        | `WalkDir`'ing PATH(s) and filtering for audio files

# Misc functions/data
Some important but more miscellaneous files.

| File           | Purpose |
|----------------|---------|
| img.rs         | Image processing functions
| thread.rs      | Thread availability functions
| msg.rs         | Types of messages `CCD` and `Kernel` can send to each other

# The Loop™
`metadata.rs` contains the function `audio_paths_to_incomplete_vecs()`, or, "The Loop".

When creating the `Collection`, most of the time is spent here.

"The Loop":

- Takes an input of `Vec<PathBuf>`
- For each PATH, parses the metadata
- Remembers if it has seen this `Artist/Album` before
- Adds relational data by embedding `Artist/Album/Song` structs together with `Key`'s
- Adds new `Artist/Album/Song`'s to the `Vec`'s
- Outputs the 3 (mostly) complete `Vec`'s (`Vec<Artist>`, `Vec<Album>`, `Vec<Sort>`)

A `HashMap` is used to "remember" which `Artist/Album`'s we've seen before, and 3 incrementing `usize`'s is used to determine which `Artist/Album/Song` we're on.

Each `PathBuf` represents a new `Song` with metadata. Each loop, there's 3 logical branches that can be taken:

| If                                     | Then         |
|----------------------------------------|--------------|
| `Artist` exists, `Album` exists        | Add `Song`
| `Artist` exists, `Album` DOESN'T exist | Add `Album + Song`
| `Artist` DOESN'T exist                 | Add `Artist + Album + Song`

These have to be linked, of course. Indicies and the `HashMap` must be updated as well.

"The Loop" is probably the longest function in this codebase, it also has lots of variables in scope. The `Vec<PathBuf>` input is also chunked into smaller pieces and handed off to multiple threads, so there's some thread/sync code mixed in as well.

The actual code has tons of comments annotating what's going on, so please read those. While the function is long and has a few branches, it's actually relatively sequential.

# Threads
The heaviest operations associated with the `Collection` ("The Loop" and image conversion) are both multi-threaded for performance.

The functions that calculate an appropriate amount of threads to use is in `threads.rs`. It's based off 2 factors:

- How many available threads do we have?
- How much work is there to do? (Image count, PATH count)

Since image conversion is 100% CPU-bound, it scales really nicely. It currently uses around `75%` of available threads.

"The Loop" benefits from multiple threads as well, but being mostly IO-bound, it hits diminishing returns pretty quickly. It currently uses `25%` of available threads.

These percentages were selected after doing some external tests and seeing how well they scale with more threads. "The Loop" in particular, does get _marginally_ faster, but not enough to justify hogging the user's cores.
