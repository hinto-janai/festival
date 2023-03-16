# Kernel
`Kernel`, the messenger and coordinator.

When initially spawned in `main()`, `Kernel` starts off a sequential 5-step process:
```
bios() ---> boot_loader() ---> kernel() ---> init() ---> userspace()
         |                                          |
         |--- (bios error occured, skip to init) ---|
```
Ignore the fact that the name of this thing is `Kernel` and it kinda makes sense.

What these phases actually do:

| Phase           | Purpose |
|-----------------|---------|
| `bios()`        | Attempt to read `collection.bin`. Skip to `init()` with default data on failure.
| `boot_loader()` | Wait on `CCD` to transform `Collection`, load other data.
| `kernel()`      | Run safety checks on data.
| `init()`        | Spawn all threads and initialize everything else.
| `userspace()`   | Main loop.

Basically, it initializes the data & threads, then loops forever, waiting for messages.

## Files
| File           | Purpose |
|----------------|---------|
| kernel.rs      | Main `Kernel` functions & data
| state.rs       | Thread-safe `State` that only `Kernel` can mutate (playlists, current song, etc)

The messages that `Kernel` can send & receive are all defined in that thread's respective folder, rather than here.

For example, messages passed between `CCD` & `Kernel` are defined in `src/ccd/msg.rs`.
