# Core functions
`CCD` has a lot of different types of tasks, so instead of shoving them all in `ccd.rs`, it acts more as a "frontend" to `Kernel`, exposing short public functions. The actual individual tasks are logically seperated into their own files. Unit tests and `use` statements are also more organized this way.

All these functions are only separted for clarity, in reality they are going to be ran right next to each other. They all have side-effects: sending update to `Kernel` via a channel, this is why the `Sender` is in all the function signatures (it would be in local scope if all this code was in _one big function_).

| File           | Purpose |
|----------------|---------|
| ccd.rs         | High-level convenience wrapper functions for `Kernel` that call the below internal ones
| convert.rs     | Converting art bytes into `egui` images
| metadata.rs    | Audio metadata (MIME) processing
| walk.rs        | `WalkDir`'ing PATH(s) and filtering for audio files

# Misc functions/data
Some important but more miscellaneous files.
| File           | Purpose |
|----------------|---------|
| img.rs         | Image processing functions
| thread.rs      | Thread availability functions
| msg.rs         | Types of messages `CCD` and `Kernel` can send to each other
